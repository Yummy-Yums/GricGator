use std::collections::HashMap;
use std::path::Path;
use std::{env, error::Error, ffi::OsStr, fs::File, io, process};

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
    priceflag: String,
    pricetype: String,
    currency: String,
    price: String,
    usdprice: String,
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record)
    }

    Ok(())
}

pub fn load_market_price_data(path: &Path) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut market_price_data: Vec<Record> = Vec::new();

    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let record: Record = result?;
        market_price_data.push(record);
    }

    Ok(market_price_data)
}
