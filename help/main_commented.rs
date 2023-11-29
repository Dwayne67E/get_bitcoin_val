// Importation des crates externes nécessaires
use serde::Deserialize;
use error_chain::error_chain;
use std::collections::HashMap;
use tokio::time::sleep;
use std::time::Duration;

// Définition des erreurs personnalisées avec error_chain
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Json(serde_json::Error);
    }
}

// Structure décrivant la réponse JSON attendue de l'API Kraken
#[derive(Deserialize, Debug)]
struct KrakenTickerResponse {
    result: HashMap<String, PairData>,
}

// Structure décrivant les données associées à une paire de trading
#[derive(Deserialize, Debug)]
struct PairData {
    c: Vec<String>,
}

// Fonction asynchrone pour récupérer le dernier prix de négociation fermée de Kraken pour une paire donnée
async fn get_last_traded_closed_price_kraken(api_url: &str, trading_pair: &str) -> Result<f64> {
    // Effectue une requête HTTP GET vers l'API Kraken
    let response = reqwest::get(api_url).await?;
    // Récupère le corps de la réponse
    let body = response.text().await?;
    // Désérialise la réponse JSON en une structure KrakenTickerResponse
    let ticker_response: KrakenTickerResponse = serde_json::from_str(&body)?;

    // Vérifie si la paire de trading spécifiée est présente dans la réponse
    if let Some(pair_data) = ticker_response.result.get(trading_pair) {
        // Vérifie si le prix est présent dans les données de la paire
        if let Some(price_str) = pair_data.c.get(0) {
            // Tente de convertir la chaîne de prix en un nombre flottant
            if let Ok(price) = price_str.parse::<f64>() {
                // Retourne le prix en cas de succès
                return Ok(price);
            }
        }
    }
    // Retourne une erreur si quelque chose ne fonctionne pas comme prévu
    Err(format!("Failed to extract current price from Kraken for trading pair: {}", trading_pair).into())
}

// Fonction asynchrone pour récupérer le dernier volume de lot de négociation fermé de Kraken pour une paire donnée
async fn get_last_traded_closed_lot_volume_kraken(api_url: &str, trading_pair: &str) -> Result<f64> {
    // (Le code est similaire à la fonction précédente avec des modifications appropriées)

    // ...
}

// Fonction principale asynchrone
async fn run() -> Result<()> {
    // Spécifie la paire de trading pour Kraken
    let kraken_pair = "XLTCZUSD";

    // Construit l'URL de l'API Kraken pour la paire spécifiée
    let kraken_api_url = format!("https://api.kraken.com/0/public/Ticker?pair={}", kraken_pair);

    // Boucle infinie pour récupérer périodiquement les données de Kraken
    loop {
        // Récupère et affiche le dernier prix de négociation fermée de Kraken
        match get_last_traded_closed_price_kraken(&kraken_api_url, kraken_pair).await {
            Ok(price) => println!("Last Trade Closed Price from Kraken for {}: {}\n", kraken_pair, price),
            Err(err) => {
                eprintln!("Error from Kraken:");
                print_error_details(&err);
            }
        }

        // Récupère et affiche le dernier volume de lot de négociation fermé de Kraken
        match get_last_traded_closed_lot_volume_kraken(&kraken_api_url, kraken_pair).await {
            Ok(volume) => println!("Last Trade Closed Lot Volume from Kraken for {}: {} \n", kraken_pair, volume),
            Err(err) => {
                eprintln!("Error from Kraken:");
                print_error_details(&err);
            }
        }

        // Pause de 10 secondes entre chaque itération (décommenter si nécessaire)
        sleep(Duration::from_secs(10)).await;
    }
}

// Fonction principale pour exécuter la boucle principale de l'application
#[tokio::main]
async fn main() {
    // Exécute la boucle principale et gère les erreurs le cas échéant
    if let Err(err) = run().await {
        print_error_details(&err);
    }
}

// Fonction utilitaire pour afficher les détails d'une erreur en chaîne
fn print_error_details(err: &Error) {
    for cause in err.iter().skip(1) {
        eprintln!("Caused by: {}", cause);
    }
}
