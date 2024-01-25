use mysql::{Opts, prelude::Queryable};
use plotters::{prelude::*, style::full_palette::BLUE};
use mysql::Conn;

fn main() {

    
    // Connexion à la base de données MySQL
    let mut conn = Conn::new(
        Opts::from_url("mysql://kyllian2:cash7823@localhost:3306/get_bitcoin_val").unwrap(),
    )
    .unwrap();

    // Exemple de requête pour récupérer les données (ajustez selon votre schéma de base de données)
    let query = "SELECT id, price FROM tab_25_01_24 ORDER BY id";
    let rows: Vec<(f64, f64)> = conn
        .query_map(query, |(price, id)| (price, id))
        .unwrap();

    // Extraction des données
    let (prices, ids): (Vec<f64>, Vec<f64>) = rows.into_iter().unzip();

    // Création du graphique
    let root = BitMapBackend::new("Price(id).png", (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();  // Fond blanc

    // Plage fixe des ordonnées entre 65 et 65.5
    let y_range = 65.0..65.3;

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .caption("Prix en fonction de l'id du trade", ("Arial", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..prices.iter().cloned().fold(f64::NAN, f64::max), y_range.clone())

        .unwrap();

    chart.configure_mesh().draw().unwrap();

    // Création de la courbe
    chart.draw_series(LineSeries::new(ids.iter().zip(prices.iter()).map(|(&x, &y)| (y,x)), &BLUE))
        .unwrap()
        .label("157 valeurs enregistrées en 25 minutes");


    chart.configure_series_labels()
        .border_style(&BLACK)
        .draw().unwrap();
}
