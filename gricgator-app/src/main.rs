use gricgator_app::{get_avail_locations_data, list_categories_of_commodities};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let locations_data = get_avail_locations_data();

    // locations_data.into_iter().take(10).for_each(|(name, city)| {
    //         println!("Name of {} with details {:#?}", name, city);
    // });

    // let res2 = get_best_market_prices_of_commodity_in_a_region("Cassava", "central")?;
    // let _ = get_weather_forecast("Accra").await?;
    // let _ = get_current_weather("Accra").await?;
    let _ = list_categories_of_commodities();


    Ok(())
}