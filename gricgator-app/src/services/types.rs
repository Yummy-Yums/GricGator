use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::services::weather_service::PinPointLocation;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetCurrentWeatherDataRequest {
    #[serde(rename = "appid")]
    app_id: String,
    #[serde(flatten)]
    pinpoint_location: PinPointLocation,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetCurrentWeatherDataRequest2 {
    pub(crate) key: String,
    #[serde(rename = "q")]
    pub(crate) query: String,
    pub(crate) aqi: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct GetCurrentWeatherDataResponse {
    dt: u32,
    weather: Vec<WeatherData>,
    main: Main,
    wind: WindData,
    clouds: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Main {
    temperature: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: f32,
    humidity: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherData {
    id: i16,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WindData {
    speed: i16,
    direction: String,
    gust: String
}

#[derive(Debug, Clone)]
struct Commodity {
    name: String,
    price: String,
    category: String,
    unit: String,
    price_type: String,
    currency: String,
}

#[derive(Debug, Clone)]
#[derive(PartialOrd)]
#[derive(PartialEq)]
pub struct CommodityPricing {
    pub price: f64,
    pub currency: String,
    pub market: String,
    pub region: String,
    pub unit: String,
}

impl Eq for CommodityPricing {}

impl Ord for CommodityPricing {

    fn cmp(&self, other: &Self) -> Ordering {
        self.price.partial_cmp(&other.price).unwrap()
    }
}

// World Food Program Data, add pub to the fields you actually need
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Record {
    pub date: String,
    pub region: String,
    assembly: String,
    pub(crate) market: String,
    market_id: i16,
    latitude: f32,
    longitude: f32,
    category: String,
    pub(crate) commodity: String,
    commodity_id: String,
    pub(crate) unit: String,
    #[serde(rename = "priceflag")]
    price_flag: String,
    #[serde(rename = "pricetype")]
    price_type: String,
    pub(crate) currency: String,
    pub(crate) price: String,
    #[serde(rename = "usdprice")]
    usd_price: String,
}