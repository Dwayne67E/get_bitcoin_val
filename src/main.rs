use serde::Deserialize;
use error_chain::error_chain;
use std::collections::HashMap;
use tokio::time::sleep;
use std::time::Duration;
//use mysql_async::{prelude::Queryable, Pool, params};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Json(serde_json::Error);
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

async fn run() -> Result<()> {
    let kraken_pair = "XLTCZUSD";
    //let binance_pair = "LTCUSD";kd

    let kraken_api_url = format!("https://api.kraken.com/0/public/Ticker?pair={}", kraken_pair);
    //let binance_api_url = format!("https://api.binance.com/api/v3/ticker/24hr?symbol={}", binance_pair);

    loop {
        match get_last_traded_closed_price_kraken(&kraken_api_url, kraken_pair).await {
            Ok(price) => println!("Last Trade Closed Price from Kraken for {}: {}\n", kraken_pair, price),
            Err(err) => {
                eprintln!("Error from Kraken:");
                print_error_details(&err);
            }
        }
        match get_last_traded_closed_lot_volume_kraken(&kraken_api_url, kraken_pair).await {
            Ok(volume) => println!("Last Trade Closed Lot Volume from Kraken for {}: {} \n", kraken_pair, volume),
            Err(err) => {
                eprintln!("Error from Kraken:");
                print_error_details(&err);
            }
        }

        sleep(Duration::from_secs(10)).await; //(enlever les commentaires si volonté d'utilisation )
        
        
        /* 
        match get_current_price_binance(&binance_api_url).await {
            Ok(price) => println!("LastPrice from Binance for {}: {}\n", binance_pair, price),
            Err(err) => {
                eprintln!("Error from Binance:");
                print_error_details(&err);
            }
        }
        */
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


/* 

#[derive(Deserialize, Debug)] //binance
struct BinanceTickerResponse {
    lastPrice: String,
    volume: String,
}

async fn get_current_price_binance(api_url_binance: &str) -> Result<f64> {
    let response = reqwest::get(api_url_binance).await?;
    let body = response.text().await?;
    let ticker_response: BinanceTickerResponse = serde_json::from_str(&body)?;
    //println!("{}", body);

    if let Ok(price) = ticker_response.lastPrice.parse::<f64>() {
        return Ok(price);
    }

    Err("Failed to extract current price from Binance".into())
}
*/