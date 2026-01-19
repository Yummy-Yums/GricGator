mod services;

use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;
use services::types::*;

use services::commodities_service::*;
use services::weather_service::*;
use services::api::*;
use dotenv::dotenv;

static API_KEY: OnceLock<String> = OnceLock::new();
static MARKET_DATA: OnceLock<Vec<Record>> = OnceLock::new();
static AVAILABLE_LOCATIONS: OnceLock<HashMap<String, City>> = OnceLock::new();

pub fn init(){
    dotenv().ok();

    API_KEY.set(
        std::env::var("API_KEY")
            .unwrap_or("".to_string())
    ).expect("Could not set API_KEY");
}

pub fn get_market_data() -> &'static Vec<Record> {
    MARKET_DATA.get_or_init(|| {
        load_market_price_data().expect("Could not load market price data")
    })
}

pub fn get_avail_locations_data() -> &'static HashMap<String, City> {
    AVAILABLE_LOCATIONS.get_or_init(|| {
        load_location_store().unwrap()
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    init();

    let locations_data = load_location_store();

    // locations_data.map(|rec| {
    //     rec.iter().take(2).for_each(|city| {
    //         println!("{:#?}", city);
    //     })
    // })?;

    let res2 = get_best_market_prices_of_commodity_in_a_region("Cassava", "central")?;

    println!("{:#?}", res2);

    let res = get_current_weather_of_a_place("Accra", PinPointLocation {
        latitude: 9.44272,
        longitude: -0.00991,
    }).await?;

    Ok(())
}