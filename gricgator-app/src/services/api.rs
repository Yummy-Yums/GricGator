use crate::services::types::*;
use crate::API_KEY;

static BASE_URL: &'static str = "https://api.weatherapi.com/v1";

pub async fn get_current_weather_of_a_place(
    place: &str,
    pinpoint_location: PinPointLocation
) -> Result<(), reqwest::Error> {

    let key = API_KEY.get().unwrap().to_string();

    let query_params = [("key", key), ("query", place.to_string()), ("aqi", "no".to_string())];

    println!("getting weather of {}", place);

    let response = reqwest::Client::new()
        .get(format!("{}/current.json", BASE_URL))
        .query(&query_params)
        .send()
        .await?;

    let response_text = response.text().await?;
    let weather_data: WeatherApiResponse = serde_json::from_str(&response_text).unwrap();
    println!("Raw API response: {:?}", weather_data.current);


    Ok(())
}


pub async fn get_one_day_weather_forecast_of_a_place(
    place: &str,
    pinpoint_location: PinPointLocation
) -> Result<(), reqwest::Error> {
    let key = API_KEY.get().unwrap().to_string();

    let query_params = 
        [("key", key),("query", place.to_string()), ("days", 1.to_string()),("aqi", "no".to_string()), ("alerts", "no".to_string())];

    println!("getting weather forecast of {}", place);

    let response = reqwest::Client::new()
        .get(format!("{}/forecast.json", BASE_URL))
        .query(&query_params)
        .send()
        .await?;

    let response_text = response.text().await?;
    let weather_data: WeatherApiResponse = serde_json::from_str(&response_text).unwrap();
    println!("Raw API response: {:?}", weather_data.current);


    Ok(())
}






