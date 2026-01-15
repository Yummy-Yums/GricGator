use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Deserialize)]
pub struct Location {
    cities: Vec<City>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct City {
    pub id: u32,
    pub name: String,
    pub state: String,
    pub country: String,
    #[serde(rename = "coord")]
    pub coordinates: PinPointLocation,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct PinPointLocation {
    #[serde(rename = "lat")]
    pub latitude: f32,
    #[serde(rename = "lon")]
    pub longitude: f32,
}


pub async fn fetch() -> Result<(), Box<dyn Error>> {

    let response = reqwest::get("https://httpbin.org/status/418").await?;
    println!("{}", response.status());
    for header in response.headers().iter() {
        println!("{}: {}", header.0, header.1.to_str()?);
    }
    let mut buf = String::new();
    buf.push_str(response.text().await?.as_str());
    println!("{}", buf);

    Ok(())
}

// fetch weather data from the agromonitor site
pub async fn fetch_data() -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub async fn get_weather_data_of_city() -> Result<(), Box<dyn Error>> { Ok(()) }

pub async fn load_locations_data() -> Result<Vec<City>, Box<dyn Error>> {

    let path = Path::new( "./src/cities.json");

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let read_data: Location = serde_json::from_reader(reader)?;

    let cities_data = read_data.cities;

    Ok(cities_data)
}