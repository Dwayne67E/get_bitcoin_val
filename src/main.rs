use error_chain::error_chain;
use serde::Deserialize;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Json(serde_json::Error);
    }
}

#[derive(Deserialize)]
struct TickerResponse {
    weightedAvgPrice: String,
    // Add other fields you might need here
}

#[tokio::main]
async fn main() -> Result<()> {
    // Première requête
    let symbol_btcusdt = "BTCUSDT";
    let url_btcusdt = format!("https://api.binance.com/api/v3/ticker/24hr?symbol={}", symbol_btcusdt);

    let res_btcusdt = reqwest::get(&url_btcusdt).await?;
    let body_btcusdt = res_btcusdt.text().await?;
    let ticker_response_btcusdt: TickerResponse = serde_json::from_str(&body_btcusdt)?;
    let weighted_avg_price_btcusdt = ticker_response_btcusdt.weightedAvgPrice;

    println!("Weighted Avg Price for BTC/USDT: {}", weighted_avg_price_btcusdt);

    // Deuxième requête
    let res = reqwest::get("https://api.kraken.com/0/public/Ticker?pair=XBTUSD").await?;
    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}
