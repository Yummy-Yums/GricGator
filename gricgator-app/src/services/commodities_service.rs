use crate::services::get_market_data;
use crate::services::types::*;
use chrono::{Datelike, NaiveDate};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;

pub fn get_price_of_commodity_across_all_markets(
    commodity: &str
) -> Result<Vec<CommodityPricing>, Box<dyn Error>> {
    let market_data = get_market_data();
    let mut results: Vec<CommodityPricing> = Vec::new();

    market_data
        .iter()
        .filter(|record| record.commodity.to_lowercase() == commodity.to_lowercase())
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
    let response = prices
        .into_iter()
        .take(10)
        .collect();

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

pub fn get_commodities_in_a_category(category: &str) -> Option<Vec<Commodity>> {
    let does_category_exists = list_categories_of_commodities().contains(&category.to_string());
    let market_data = get_market_data();

    if !does_category_exists {
        return None
    }

    let mut result = market_data
        .iter()
        .filter(|record| record.category == category)
        .map(|record| {
            Commodity {
                id: record.market_id as i8,
                name: record.commodity.clone(),
                category: record.category.clone(),
                unit: record.unit.clone(),
                price: record.price.clone(),
                price_type: record.price_type.clone(),
                currency: record.currency.clone(),
            }
        })
        .collect::<Vec<Commodity>>();

    result.sort_by(|a, b| b.name.cmp(&a.name));
    result.dedup_by(|a, b| a.name == b.name);

    Some(result)
}

pub fn list_all_commodities() -> HashSet<String> {
    let market_data = get_market_data();
    let mut commodities = HashSet::new();

    market_data
        .iter()
        .filter(|record| record.date.contains("2023"))
        .for_each(|record| {
            commodities.insert(record.commodity.clone());
        });

    commodities
}

pub fn list_categories_of_commodities() -> HashSet<String> {
    let market_data = get_market_data();
    let mut categories = HashSet::new();

    market_data
        .iter()
        .filter(|record| record.date.contains("2023"))
        .for_each(|record| {
            categories.insert(record.category.clone());
        });
    categories
}

