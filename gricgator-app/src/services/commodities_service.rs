use std::path::Path;
use std::{error::Error, fs::File};
use std::cmp::Ordering;
use chrono::{Datelike, NaiveDate};
use crate::get_market_data;
use crate::services::types::*;

const PATH: &str = "./src/wfp_food_prices_gha.csv";

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

pub fn get_price_of_commodity_across_all_markets(
    commodity: &str
) -> Result<Vec<CommodityPricing>, Box<dyn Error>> {
    let market_data = get_market_data();
    let mut results: Vec<CommodityPricing> = Vec::new();

    market_data
        .iter()
        .filter(|record| record.commodity == commodity)
        .filter(|record| { record.date.parse::<NaiveDate>().unwrap().year() == 2023 })
        .for_each(|record|
            {
            results.push(
                CommodityPricing {
                        price: record.price.clone().parse().unwrap(),
                        currency: record.currency.clone(),
                        market: record.market.clone(),
                        region: record.region.clone(),
                        unit: record.unit.clone(),
                    }
                );
            }
        );

    results.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
    results.dedup_by(|a, b| a.price == b.price);

    Ok(results)
}

pub fn get_best_market_prices_of_commodity(
    commodity: &str
) -> Result<Vec<CommodityPricing>, Box<dyn Error>> {

    let prices = get_price_of_commodity_across_all_markets(commodity)?;
    let response = prices.into_iter().take(10).collect();

    Ok(response)
}

pub fn get_best_market_prices_of_commodity_in_a_region(
    commodity: &str,
    region: &str
) -> Result<Vec<CommodityPricing>, Box<dyn Error>> {

    let prices = get_price_of_commodity_across_all_markets(commodity)?;

    let mut response: Vec<CommodityPricing>= prices
        .into_iter()
        .filter(|prices| prices.region.eq_ignore_ascii_case(region))
        .rev()
        .take(10)
        .collect();

    response.sort_by(|price_a, price_b| price_a.partial_cmp(&price_b)
        .unwrap_or(Ordering::Equal));

    Ok(response)
}

pub fn compare_best_market_prices_of_commodity_btn_markets(
    commodity: &str,
    first_region: &str,
    second_region: &str,
) -> Result<Vec<CommodityPricing>, Box<dyn Error>> {

    let first_region_best_prices = get_best_market_prices_of_commodity_in_a_region(commodity, first_region)?
        .into_iter()
        .rev()
        .take(5)
        .collect::<Vec<CommodityPricing>>();
    let second_region_best_prices = get_best_market_prices_of_commodity_in_a_region(commodity, second_region)?
        .into_iter()
        .rev()
        .take(5)
        .collect::<Vec<CommodityPricing>>();;

    let response = [first_region_best_prices, second_region_best_prices]
        .into_iter().flatten().collect::<Vec<CommodityPricing>>();

    Ok(response)
}

pub fn get_commodities_in_a_category(category: &str) -> Vec<CommodityPricing> {
    // use market data
    // let mut set: HashSet<&String> = HashSet::new();
    // let _ = market_price_data
    //     .iter()
    //     .for_each(|record| {
    //         set.insert(&record.category);
    //     });
    todo!()

}