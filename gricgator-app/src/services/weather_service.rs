use std::collections::HashMap;
use std::error::Error;
use crate::services::{
    get_avail_locations_data,
    get_current_weather_of_a_place,
    get_one_day_weather_forecast_of_a_place
};
use crate::services::types::{City, WeatherApiResponse, WeatherForecastResponse};

pub async fn get_current_weather(city_name: &str) -> Result<(), Box<dyn Error>> {

    let locations: &HashMap<String, City> = get_avail_locations_data();

    let result = locations
        .keys()
        .find(|name| { city_name == *name});

    match result {
        Some(location) => {
            let current_weather: WeatherApiResponse = get_current_weather_of_a_place(location).await?;

            let response = format!("\
                The weather condition for {} today is {} and the temperature is {}°C. It will feel like {}°C, \
                 The wind chill is {} and the wind will move at {}mph, humidity is {} and precipitation is {}",
                    city_name,
                    current_weather.current.condition.text,
                    current_weather.current.temp_c,
                    current_weather.current.feels_like,
                    current_weather.current.wind_chill,
                    current_weather.current.wind_mph,
                    current_weather.current.humidity,
                    current_weather.current.precipitation,
            );
            println!("{}", response);
            Ok(())
        },
        None => Err("Could not find city location for city".to_string())?,
    }


}

pub async fn get_weather_forecast(city_name: &str) -> Result<(), Box<dyn Error>> {

    let locations: &HashMap<String, City> = get_avail_locations_data();

    let result = locations
        .keys()
        .find(|name| { city_name.to_lowercase() == **name});

    match result {
        Some(location) => {
            let current_weather_forecast = get_one_day_weather_forecast_of_a_place(location).await;

            match current_weather_forecast {
                Ok(weather_forecast) => {
                    println!("{:?}", weather_forecast.forecast.forecast_day.first().unwrap().day);
                    Ok(())
                },
                Err(error) => {
                    println!("{}", error.without_url());
                    Err("error getting weather forecast".to_string())?
                }
            }
        },
        None => {
            Err("Could not find city location for city".to_string())?
        },
    }


}

pub fn is_city_or_location_available(city_name: &str) -> bool {
    let available_locations = get_avail_locations_data();
    available_locations.contains_key(city_name)
}

pub async fn retry<F, Fut, T, E>(
    mut f: F,
    max_retries: u32,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    for attempt in 0..max_retries {
        let result = f().await;
        match result {
            Ok(result) => return Ok(result),
            Err(error) if attempt == max_retries - 1 =>   {
                println!("Retry Later");
                return Err(error)
            }
            Err(_) => {
                println!("{} retry , Retrying Again", attempt + 1);
                tokio::time::sleep(std::time::Duration::from_secs(2_u64.pow(attempt))).await;
            }
        }
    }

    unreachable!("The second arm will always be matched")
}