mod services;

use std::error::Error;
use std::path::Path;
use std::sync::OnceLock;
use services::types::*;



use services::commodities_service::*;
use services::weather_service::*;
use dotenv::dotenv;

static API_KEY: OnceLock<String> = OnceLock::new();
static MARKET_DATA: OnceLock<Vec<Record>> = OnceLock::new();

pub fn init(){
    dotenv().ok();

    API_KEY.set(
        std::env::var("API_KEY")
            .unwrap_or("".to_string())
    ).expect("Could not set API_KEY");

}

pub fn get_market_data() -> &'static Vec<Record> {
    MARKET_DATA.get_or_init(|| {
        load_market_price_data(Path::new(
            "./src/wfp_food_prices_gha.csv",
        )).expect("Could not load market price data")
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    init();

    // let locations_data = load_locations_data().await;
    //
    // locations_data.map(|rec| {
    //     rec.iter().take(2).for_each(|city| {
    //         println!("{:#?}", city);
    //     })
    // })?;
    //


    // if let Ok(res) = market_data {
    //     res.iter()
    //         .take(2)
    //         .for_each(|rec: &Record| println!("{:?}", rec));
    // }

    // let res = get_best_market_prices_of_commodity("Cassava")?;
    let res2 = get_best_market_prices_of_commodity_in_a_region("Cassava", "central")?;

    // res.iter().take(10).for_each(|e| {println!("{:?}", e)});

    println!("{:#?}", res2);

    // let res = services::api::get_weather_forecast_of_a_place("Yendi", PinPointLocation {
    //     latitude: 9.44272,
    //     longitude: -0.00991,
    // }).await?;

    Ok(())
}