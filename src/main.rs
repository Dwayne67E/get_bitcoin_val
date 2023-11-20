use serde::Deserialize;
use error_chain::error_chain;
use std::collections::HashMap;
use tokio::time::sleep;
use std::time::Duration;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Json(serde_json::Error);
    }
}

#[derive(Deserialize, Debug)] //kraken
struct KrakenTickerResponse {
    result: HashMap<String, PairData>,
}

#[derive(Deserialize, Debug)] //kraken
struct PairData {
    c: Vec<String>,
    v: Vec<String>,
}

#[derive(Deserialize, Debug)] //binance
struct BinanceTickerResponse {
    lastPrice: String,
    volume: String,
}

async fn get_current_price_kraken(api_url: &str, trading_pair: &str) -> Result<f64> {
    let response = reqwest::get(api_url).await?;
    let body = response.text().await?;
    let ticker_response: KrakenTickerResponse = serde_json::from_str(&body)?;
    //println!("{}", body);

    if let Some(pair_data) = ticker_response.result.get(trading_pair) {
        if let Some(price_str) = pair_data.c.get(0) {
            if let Ok(price) = price_str.parse::<f64>() {
                return Ok(price);
            }
        }
    }

    Err(format!(
        "Failed to extract current price from Kraken for trading pair: {}",
        trading_pair
    )
    .into())
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

fn print_error_details(err: &Error) {
    for cause in err.iter().skip(1) {
        eprintln!("Caused by: {}", cause);
    }
}

async fn get_current_volume_kraken(api_url: &str, trading_pair: &str) -> Result<f64> {
    let response = reqwest::get(api_url).await?;
    let body = response.text().await?;
    let ticker_response: KrakenTickerResponse = serde_json::from_str(&body)?;
    //println!("{}", body);

    if let Some(pair_data) = ticker_response.result.get(trading_pair) {
        if let Some(volume_str) = pair_data.v.get(1) {
            if let Ok(volume) = volume_str.parse::<f64>() {
                return Ok(volume);
            }
        }
    }

    Err(format!(
        "Failed to extract current price from Kraken for trading pair: {}",
        trading_pair
    )
    .into())
}

async fn run() -> Result<()> {
    let kraken_pair = "XLTCZEUR";
    let binance_pair = "LTCEUR";

    let kraken_api_url = format!("https://api.kraken.com/0/public/Ticker?pair={}", kraken_pair);
    let binance_api_url = format!("https://api.binance.com/api/v3/ticker/24hr?symbol={}", binance_pair);

    loop {
        // Fetch and display Kraken price
        match get_current_price_kraken(&kraken_api_url, kraken_pair).await {
            Ok(price) => println!("LastPrice from Kraken for {}: {}", kraken_pair, price),
            Err(err) => {
                eprintln!("Error from Kraken:");
                print_error_details(&err);
            }
        }

        // Fetch and display Binance price
        match get_current_price_binance(&binance_api_url).await {
            Ok(price) => println!("LastPrice from Binance for {}: {}", binance_pair, price),
            Err(err) => {
                eprintln!("Error from Binance:");
                print_error_details(&err);
            }
        }

        match get_current_volume_kraken(&kraken_api_url, kraken_pair).await {
            Ok(volume) => println!("Last 24h Volume from Kraken for {}: {}", kraken_pair, volume),
            Err(err) => {
                eprintln!("Error from Binance:");
                print_error_details(&err);
            }
        }

        // fait une pause de 20secondes
        //sleep(Duration::from_secs(20)).await; (enlever les commentaires si volonté d'utilisation)
    }
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        print_error_details(&err);
    }
}
