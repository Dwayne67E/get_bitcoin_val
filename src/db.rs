use serde::Deserialize;
use error_chain::error_chain;
use std::collections::HashMap;
use tokio::time::sleep;
use std::time::Duration;
use sqlx::mysql::MySqlPoolOptions;


//use mysql_async::{prelude::Queryable, Pool, params}; -----advise from teacher use SQLX crate 

mod orphan_impls {
    use super::*;

    pub trait IntoError {
        fn into_error(self) -> Error;
    }

    impl IntoError for sqlx::Error {
        fn into_error(self) -> Error {
            ErrorKind::Sqlx(self).into()
        }
    }
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Json(serde_json::Error);
        Sqlx(sqlx::Error);
    }
}

#[derive(Deserialize, Debug)] //krakennn
struct KrakenTickerResponse {
    result: HashMap<String, PairData>,
}

#[derive(Deserialize, Debug)] //kraken e ignorer
struct PairData {
    c: Vec<String>,
    // c: Vec<String>,
}



async fn get_last_traded_closed_price_kraken(api_url: &str, trading_pair: &str) -> Result<f64> {
    let response = reqwest::get(api_url).await?;
    let body = response.text().await?;
    let ticker_response: KrakenTickerResponse = serde_json::from_str(&body)?;

    if let Some(pair_data) = ticker_response.result.get(trading_pair) {
        if let Some(price_str) = pair_data.c.get(0) {
            if let Ok(price) = price_str.parse::<f64>() {
                return Ok(price);
            }
        }
    }
    Err(format!("Failed to extract current price from Kraken for trading pair: {}",trading_pair).into())
}

async fn get_last_traded_closed_lot_volume_kraken(api_url: &str, trading_pair: &str) -> Result<f64> {
    let response = reqwest::get(api_url).await?;
    let body = response.text().await?;
    let ticker_response: KrakenTickerResponse = serde_json::from_str(&body)?;

    if let Some(pair_data) = ticker_response.result.get(trading_pair) {
        if let Some(volume_str) = pair_data.c.get(1) {
            if let Ok(volume) = volume_str.parse::<f64>() {
                return Ok(volume);
            }
        }
    }
    Err(format!("Failed to extract current price from Kraken for trading pair: {}",trading_pair).into())
}


async fn insert_price_volume_to_database(pool: &sqlx::MySqlPool, price: f64, volume: f64) -> Result<()> {
    match sqlx::query(
        "INSERT IGNORE INTO tab_25_01_24 (price, volume, insertion_date) VALUES (?, ?, CURRENT_TIMESTAMP)"
    )
    .bind(price)
    .bind(volume)
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            // Vérifier si l'erreur est due à une entrée en double
            if let Some(db_error) = err.as_database_error() {
                if db_error.code().is_some() && db_error.code().unwrap() == "1062" {
                    println!("Duplicate entry: price {}", price);
                    Ok(())
                } else {
                    Err(err.into())
                }
            } else {
                Err(err.into())
            }
        }
    }
}

async fn run() -> Result<()> {
    let kraken_pair = "XLTCZUSD";

    let kraken_api_url = format!("https://api.kraken.com/0/public/Ticker?pair={}", kraken_pair);

    // Créer une connexion à la base de données
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://kyllian2:cash7823@localhost/get_bitcoin_val")
        .await?;

    loop {
        match get_last_traded_closed_price_kraken(&kraken_api_url, kraken_pair).await {
            Ok(price) => {
                println!("Last Trade Closed Price from Kraken for {}: {}", kraken_pair, price);
                // Insérer le prix et le volume dans la table
                match get_last_traded_closed_lot_volume_kraken(&kraken_api_url, kraken_pair).await {
                    Ok(volume) => {
                        println!("Last Trade Closed Lot Volume from Kraken for {}: {}", kraken_pair, volume);
                        insert_price_volume_to_database(&pool, price, volume).await?;
                    }
                    Err(err) => {
                        eprintln!("Error fetching volume from Kraken:");
                        print_error_details(&err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Error fetching price from Kraken:");
                print_error_details(&err);
            }
        }

        sleep(Duration::from_secs(10)).await; //(enlever les commentaires si volonté d'utilisation )
        

    }
}
#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        print_error_details(&err);
    }
}

fn print_error_details(err: &Error) {
    for cause in err.iter().skip(1) {
        eprintln!("Caused by: {}", cause);
    }
}
