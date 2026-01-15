mod services;


use std::path::Path;
use std::sync::OnceLock;



use services::commodities_service::*;
use services::weather_service::*;
use dotenv::dotenv;

static APP_ID: OnceLock<String> = OnceLock::new();

pub fn init(){
    dotenv().ok();

    APP_ID.set(
        std::env::var("API_KEY")
            .unwrap_or("".to_string())
    );

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    init();

    APP_ID.get().map(|e| {
       println!("{}", e);
    });

    let locations_data = load_locations_data().await;

    locations_data.map(|rec| {
        rec.iter().take(2).for_each(|city| {
            println!("{:#?}", city);
        })
    })?;

    let market_data = load_market_price_data(Path::new(
        "./src/wfp_food_prices_gha.csv",
    ));

    if let Ok(res) = market_data {
        res.iter()
            .take(2)
            .for_each(|rec: &Record| println!("{:?}", rec));
    }

    let res = services::api::get_weather_forecast_of_a_place("Yendi", PinPointLocation {
        latitude: 9.44272,
        longitude: -0.00991,
    }).await?;


    Ok(())
}
