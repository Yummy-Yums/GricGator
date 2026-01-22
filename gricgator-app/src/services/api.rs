use std::sync::OnceLock;
use dotenv::dotenv;
use crate::services::types::*;

static BASE_URL: &'static str = "https://api.weatherapi.com/v1";
static API_KEY: OnceLock<String> = OnceLock::new();

pub fn init_api_key(){
    dotenv().ok();

    let key = std::env::var("API_KEY")
        .expect("Missing API_KEY env var");

    API_KEY.set(key).expect("Could not set API_KEY");
}

pub fn get_api_key() -> &'static str {
    API_KEY.get().expect("Could not get API_KEY, Ensure the API_KEY is set ")
}

pub async fn get_current_weather_of_a_place(
    place: &str,
) -> Result<WeatherApiResponse, reqwest::Error> {


    let key = get_api_key();
    let query_params = [
        ("key", key),
        ("query", place),
        ("aqi", "no")
    ];

    println!("getting current weather of {}", place);

    let response = reqwest::Client::new()
        .get(format!("{}/current.json", BASE_URL))
        .query(&query_params)
        .send()
        .await?;

    let response_text = response.text().await?;
    let weather_data = serde_json::from_str(&response_text)
        .expect(format!("Failed to parse current Weather API response of {}", place).as_str());

    Ok(weather_data)
}

pub async fn get_one_day_weather_forecast_of_a_place(
    place: &str,
) -> Result<WeatherForecastResponse, reqwest::Error> {
    let key = get_api_key();

    let query_params = [
        ("key", key.to_string()),
        ("query", place.to_string()),
        ("days", 1.to_string()),
        ("aqi", "no".to_string()),
        ("alerts", "no".to_string())
    ];

    println!("getting one day weather forecast of {}", place);

    let response = reqwest::Client::new()
        .get(format!("{}/forecast.json", BASE_URL))
        .query(&query_params)
        .send()
        .await?;

    let response_text = response.text().await?;

    let weather_data: WeatherForecastResponse = serde_json::from_str(&response_text)
        .expect(format!("Failed to parse weather forecast of {}", place).as_str());

    Ok(weather_data)
}






