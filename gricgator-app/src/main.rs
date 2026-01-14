use std::path::Path;

mod services;
use services::commodities_service::Record;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let e = services::weather_service::load_locations_data().await;
    e.map(|rec| {
        rec.iter().for_each(|city| {
            println!("{:#?}", city);
        })
    })?;
    // let res = services::commodities_service::load_market_price_data(Path::new(
    //     "./src/wfp_food_prices_gha.csv",
    // ));
    //
    // if let Ok(res) = res {
    //     res.iter()
    //         .take(20)
    //         .for_each(|rec: &Record| println!("{:?}", rec.region));
    // }

    // let hello = std::env::var("API_KEY")
    //     .unwrap_or("could not get key".to_string());
    // println!("Hello {hello}");

    Ok(())
}
