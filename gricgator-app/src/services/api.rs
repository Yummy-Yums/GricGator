use serde::{Deserialize, Serialize};
use crate::APP_ID;
use crate::services::weather_service::PinPointLocation;

static BASE_URL: &'static str = "https://api.weatherapi.com/v1/current.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct GetCurrentWeatherDataRequest {
    #[serde(rename = "appid")]
    app_id: String,
    #[serde(flatten)]
    pinpoint_location: PinPointLocation,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetCurrentWeatherDataRequest2 {
    key: String,
    #[serde(rename = "q")]
    query: String,
    #[serde(default = "default_aqi")]
    aqi: String,
}

fn default_aqi() -> String {
    "no".to_string()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetCurrentWeatherDataResponse {
    dt: u32,
    weather: Vec<WeatherData>,
    main: Main,
    wind: WindData,
    clouds: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Main {
    temperature: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: f32,
    humidity: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherData {
    id: i16,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WindData {
    speed: i16,
    direction: String,
    gust: String
}

pub async fn get_weather_forecast_of_a_place(
    place: &str,
    pinpoint_location: PinPointLocation
) -> Result<(), reqwest::Error> {

    let key = APP_ID.get().unwrap().to_string();
    println!("{}", key);

    println!("getting weather forecast of {}", place);

    let request_data = GetCurrentWeatherDataRequest2 {
        key,
        query: "Accra".to_string(),
        aqi: default_aqi()
    };

    let response = reqwest::Client::new()
        .post(BASE_URL)
        .json(&request_data)
        .send()
        .await?;

    let response_text = response.text().await?;
    println!("Raw API response: {}", response_text);

        // .json::<GetCurrentWeatherDataResponse>()
        // .await?;

    Ok(())
}





