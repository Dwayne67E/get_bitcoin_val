use serde::Deserialize;
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Json(serde_json::Error);
    }
}

#[derive(Deserialize, Debug)]
struct KrakenTickerResponse {
    // Adjust this struct based on the actual Kraken API response
    result: ResultData,
}

#[derive(Deserialize, Debug)]
struct ResultData {
    #[serde(rename = "XETHZUSD")]
    xethzusd: XETHZUSDData,
}

#[derive(Deserialize, Debug)]
struct XETHZUSDData {
    c: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct BinanceTickerResponse {
    lastPrice: String,
    // Add other fields you might need here
}

async fn get_current_price_kraken(api_url: &str) -> Result<f64> {
    let response = reqwest::get(api_url).await?;
    let body = response.text().await?;

    let ticker_response: KrakenTickerResponse = serde_json::from_str(&body)?;

    // Extract the correct field from the Kraken API response
    // For example, if the price is in the first element of the `c` vector:
    if let Some(price_str) = ticker_response.result.xethzusd.c.get(0) {
        if let Ok(price) = price_str.parse::<f64>() {
            return Ok(price);
        }
    }

    // Return an error if the price extraction fails
    Err("Failed to extract current price from Kraken".into())
}

async fn get_current_price_binance(symbol: &str) -> Result<f64> {
    let url = format!("https://api.binance.com/api/v3/ticker/24hr?symbol={}", symbol);

    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    //println!("{}",body);

    let ticker_response: BinanceTickerResponse = serde_json::from_str(&body)?;

    // Extract the price from the "weightedAvgPrice" field
    if let Ok(price) = ticker_response.lastPrice.parse::<f64>() {
        return Ok(price);
    }

    // Return an error if the price extraction fails
    Err("Failed to extract current price from Binance".into())
}

#[tokio::main]
async fn main() -> Result<()> {
    let kraken_api_url = "https://api.kraken.com/0/public/Ticker?pair=ETHUSD";

    match get_current_price_kraken(kraken_api_url).await {
        Ok(price) => println!("LastPrice for ETH/USDT Kraken: {}", price),
        Err(err) => eprintln!("Error from Kraken: {:?}", err),
    }

    let binance_symbol_ethusdt = "ETHUSDT";

    match get_current_price_binance(binance_symbol_ethusdt).await {
        Ok(price) => println!("LastPrice for Binance {}: {}", binance_symbol_ethusdt, price),
        Err(err) => eprintln!("Error from Binance: {:?}", err),
    }

    Ok(())
}
