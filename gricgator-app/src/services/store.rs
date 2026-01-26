use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::OnceLock;
use crate::services::types::{City, Location, Record};

const PATH: &str = "./src/resources/wfp_food_prices_gha.csv";
static AVAILABLE_LOCATIONS: OnceLock<HashMap<String, City>> = OnceLock::new();
static MARKET_DATA: OnceLock<Vec<Record>> = OnceLock::new();
static COMMODITY_DATA: OnceLock<Vec<Record>> = OnceLock::new();

pub fn get_market_data() -> &'static Vec<Record> {
    MARKET_DATA.get_or_init(|| {
        load_market_price_data().expect("Could not load market price data")
    })
}

// hold on for now...cos if it's used once, it might not be needed
pub fn get_commodity_data() -> &'static Vec<Record> {
    COMMODITY_DATA.get_or_init(|| {
        load_market_price_data().expect("Could not load market price data")
    })
}

pub fn get_avail_locations_data() -> &'static HashMap<String, City> {
    AVAILABLE_LOCATIONS.get_or_init(|| {
        load_location_store().expect("Could not load market price data")
    })
}

fn load_market_price_data() -> Result<Vec<Record>, Box<dyn Error>> {
    let mut market_price_data: Vec<Record> = Vec::new();

    let file = File::open(PATH)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let record: Record = result?;
        market_price_data.push(record);
    }

    let _ = market_price_data.iter().filter(|record| record.date.contains("2023"));

    Ok(market_price_data)
}

fn load_location_store() -> Result<HashMap<String, City>, Box<dyn Error>> {

    let path = Path::new("./src/resources/cities.json");

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let read_data: Location = serde_json::from_reader(reader)?;

    let cities_data: Vec<City> = read_data
        .cities
        .into_iter()
        .filter(|er| {
            !er.name.to_lowercase().contains("region")
        })
        .collect();

    let mut store = HashMap::new();

    cities_data
        .iter()
        .for_each(|city| {
        store.insert(city.name.clone(), city.clone());
        });

    Ok(store)
}



