mod services;
mod cli;
use std::sync::OnceLock;

use dotenv::dotenv;
use services::{
    get_avail_locations_data,
    get_current_weather_of_a_place,
    get_current_weather,
    get_best_market_prices_of_commodity_in_a_region
};
use crate::services::get_weather_forecast;

static API_KEY: OnceLock<String> = OnceLock::new();


pub fn init(){
    dotenv().ok();

    API_KEY.set(
        std::env::var("API_KEY")
            .unwrap_or("".to_string())
    ).expect("Could not set API_KEY");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    init();

    let locations_data = get_avail_locations_data();

    // locations_data.into_iter().take(10).for_each(|(name, city)| {
    //         println!("Name of {} with details {:#?}", name, city);
    // });

    let res2 = get_best_market_prices_of_commodity_in_a_region("Cassava", "central")?;
    let _ = get_weather_forecast("Accra").await?;
    // let _ = get_current_weather("Accra").await?;


    Ok(())
}