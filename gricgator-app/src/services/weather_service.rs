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
                The weather condition for today is {} and the temperature is {}°C. It will feel like {}°C, \
                 The wind chill is {} and the wind will move at {}mph, humidity is {} and precipitation is {}",
                    current_weather.current.condition.text,
                    current_weather.current.temp_c,
                    current_weather.current.feels_like,
                    current_weather.current.wind_chill,
                    current_weather.current.wind_mph,
                    current_weather.current.humidity,
                    current_weather.current.precipitation,
            );

            println!("{}", response);
        },
        None => Err("Could not find city location for city".to_string())?,
    }

    Ok(())
}

pub async fn get_weather_forecast(city_name: &str) -> Result<(), Box<dyn Error>> {

    let locations: &HashMap<String, City> = get_avail_locations_data();

    let result = locations
        .keys()
        .find(|name| { city_name == *name});

    match result {
        Some(location) => {
            let current_weather_forecast: WeatherForecastResponse = get_one_day_weather_forecast_of_a_place(location).await?;
            println!("{:?}", current_weather_forecast.forecast.forecast_day.first().unwrap().day);

            let response = format!(""

            );

            println!("{}", response);
        },
        None => Err("Could not find city location for city".to_string())?,
    }

    Ok(())
}