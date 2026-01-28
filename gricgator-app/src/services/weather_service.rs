use crate::services::types::{City, ForecastDay, Hour, WeatherApiResponse};
use crate::services::{
    get_avail_locations_data,
    get_current_weather_of_a_place,
    get_one_day_weather_forecast_of_a_place
};
use chrono::{NaiveTime, Timelike};
use std::collections::HashMap;
use std::error::Error;

pub async fn get_current_weather(city_name: &str) -> Result<(), Box<dyn Error>> {

    let locations: &HashMap<String, City> = get_avail_locations_data();

    let result = locations
        .keys()
        .find(|name| { city_name.to_lowercase() == **name});

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

pub async fn get_weather_forecast(city_name: &str) -> Result<ForecastDay, Box<dyn Error>> {

    let locations: &HashMap<String, City> = get_avail_locations_data();

    let result = locations
        .keys()
        .find(|name| { city_name.to_lowercase() == **name});

    match result {
        Some(location) => {
            let current_weather_forecast = get_one_day_weather_forecast_of_a_place(location).await;

            match current_weather_forecast {
                Ok(weather_forecast) => {
                    let forecast_day = weather_forecast.forecast.forecast_day.first().unwrap().clone();
                    Ok(forecast_day)
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

pub async fn get_time_period_forecast_of_day(
    city_name: &str,
    time_period: &str
) -> Result<Vec<Hour>, Box<dyn Error>> {

    let forecast_day = get_weather_forecast(city_name).await?;
    let hours_in_forecast_day = forecast_day.hour;
    let hours_bounds = (
        NaiveTime::from_hms_opt(6, 0, 0),
        NaiveTime::from_hms_opt(12, 0, 0),
        NaiveTime::from_hms_opt(12, 0, 0),
        NaiveTime::from_hms_opt(16, 0, 0),
        NaiveTime::from_hms_opt(16, 0, 0),
        NaiveTime::from_hms_opt(20, 0, 0),
        NaiveTime::from_hms_opt(20, 0, 0),
        NaiveTime::from_hms_opt(0, 0, 0)
    );

    let mut results: Vec<Hour> = Vec::new();

    for hourly_forecast in hours_in_forecast_day {
            let parsed_time = NaiveTime::parse_from_str(hourly_forecast.time.as_str(), "%Y-%m-%d %H:%M")?;
            match time_period.to_lowercase().as_str() {
                "afternoon" => {

                    if (parsed_time.hour() >= hours_bounds.2.unwrap().hour() &&
                        parsed_time.hour() <= hours_bounds.3.unwrap().hour()) {
                        results.push(hourly_forecast)
                    }
                },
                "morning" => {

                    if (parsed_time.hour() >= hours_bounds.0.unwrap().hour() &&
                        parsed_time.hour() <= hours_bounds.1.unwrap().hour()) {
                        results.push(hourly_forecast)
                    }
                },
                "evening" => {

                    if (parsed_time.hour() >= hours_bounds.4.unwrap().hour() &&
                        parsed_time.hour() <= hours_bounds.5.unwrap().hour()) {
                        results.push(hourly_forecast)
                    }
                },
                "night" => {

                    if (parsed_time.hour() >= hours_bounds.6.unwrap().hour() ||
                        parsed_time.hour() == hours_bounds.7.unwrap().hour()) {
                        results.push(hourly_forecast)
                    }
                },
                _ => {
                    Err("Invalid time period".to_string())?
                }
            }
    }

    Ok(results)
}

pub fn is_city_or_location_available(city_name: &str) -> bool {
    let available_locations = get_avail_locations_data();
    available_locations.contains_key(&city_name.to_string().to_lowercase())
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