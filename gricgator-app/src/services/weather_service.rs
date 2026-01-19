use std::collections::HashMap;
use super::types::*;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;


pub fn load_location_store() -> Result<HashMap<String, City>, Box<dyn Error>> {
    let mut store = HashMap::new();
    let cities_data = load_locations_data_from_file()?;

    cities_data.iter().for_each(|city| {
        store.insert(city.name.clone(), city.clone());
    });

    Ok(store)
}

pub async fn get_weather_data_of_city() -> Result<(), Box<dyn Error>> {
    // look in the location stor, get the data and pass it in the api function, format it properly
    Ok(())
}

fn load_locations_data_from_file() -> Result<Vec<City>, Box<dyn Error>> {

    let path = Path::new( "./src/cities.json");

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let read_data: Location = serde_json::from_reader(reader)?;

    let cities_data = read_data.cities;

    Ok(cities_data)
}