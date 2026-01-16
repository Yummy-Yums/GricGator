use serde::{Deserialize, Serialize};
use crate::API_KEY;
use crate::services::types::*;
use crate::services::weather_service::PinPointLocation;

static BASE_URL: &'static str = "https://api.weatherapi.com/v1/current.json";

fn default_aqi() -> String {
    "no".to_string()
}

pub async fn get_weather_forecast_of_a_place(
    place: &str,
    pinpoint_location: PinPointLocation
) -> Result<(), reqwest::Error> {

    let key = API_KEY.get().unwrap().to_string();
    println!("key {}", key);

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





