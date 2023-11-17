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
    weighted_avg_price: String,
    // Add other fields you might need here
}

#[tokio::main]
async fn main() -> Result<()> {
    let symbol = "BTCUSDT";
    let url = format!("https://api.binance.com/api/v3/ticker/24hr?symbol={}", symbol);

    let res = reqwest::get(&url).await?;
    //println!("Status: {}", res.status());
    //println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    //println!("Body:\n{}", body);

    let ticker_response: TickerResponse = serde_json::from_str(&body)?;
    let weighted_avg_price = ticker_response.weightedAvgPrice;

    println!("Weighted Avg Price for BTC/USDT: {}", weighted_avg_price);
    Ok(())
}
