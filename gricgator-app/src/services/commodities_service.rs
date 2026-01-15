use std::path::Path;
use std::{error::Error, fs::File};
use std::collections::HashSet;

struct Commodity {}

// World Food Program Data, add pub to the fields you actually need
#[derive(Debug, serde::Deserialize)]
pub struct Record {
    pub date: String,
    pub region: String,
    assembly: String,
    market_id: i16,
    latitude: f32,
    longitude: f32,
    category: String,
    commodity: String,
    commodity_id: String,
    unit: String,
    #[serde(rename = "priceflag")]
    price_flag: String,
    #[serde(rename = "pricetype")]
    price_type: String,
    currency: String,
    price: String,
    #[serde(rename = "usdprice")]
    usd_price: String,
}

pub fn load_market_price_data(path: &Path) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut market_price_data: Vec<Record> = Vec::new();

    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let record: Record = result?;
        market_price_data.push(record);
    }

    let mut set: HashSet<&String> = HashSet::new();
    let _ = market_price_data
        .iter()
        .for_each(|record| {
            set.insert(&record.category);
        });

    for a in set {
        println!("== {}", a);
    }

    Ok(market_price_data)
}
