use std::cmp::Ordering;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct GetCurrentWeatherDataRequest {
    pub  key: String,
    #[serde(rename = "q")]
    pub  query: String,
    pub  aqi: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherCondition {
    text: String,
    icon: String,
    code: i16
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherData {
    temp_c: f32,
    is_day: i8,
    #[serde(rename = "feelslike_c")]
    feels_like: f32,
    #[serde(rename = "windchill_c")]
    wind_chill: f32,
    condition: WeatherCondition,
    wind_mph: f32,
    pub(crate) humidity: f32,
    #[serde(rename = "precip_mm")]
    precipitation: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Day {
    #[serde(rename = "maxtemp_c")]
    max_temp_in_celsius: f32,
    #[serde(rename = "mintemp_c")]
    min_temp_in_celsius: f32,
    #[serde(rename = "avgtemp_c")]
    avg_temp_c: f32,
    #[serde(rename = "maxwind_mph")]
    max_wind_mph: f32,
    #[serde(rename = "totalprecip_mm")]
    total_precipitation: f32,
    #[serde(rename = "avghumidity")]
    average_humidity: f32,

}

#[derive(Debug, Deserialize)]
pub struct ForecastDay {
    date: String,
    date_epoch: String,
    day: Day,
}

#[derive(Debug, Deserialize)]
pub struct Forecast {
    forecastday: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct WeatherApiResponse {
    location: serde_json::Value,
    pub current: WeatherData,
    forecast: Forecast,
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
    pub market: String,
    market_id: i16,
    latitude: f32,
    longitude: f32,
    category: String,
    pub commodity: String,
    commodity_id: String,
    pub unit: String,
    #[serde(rename = "priceflag")]
    price_flag: String,
    #[serde(rename = "pricetype")]
    price_type: String,
    pub currency: String,
    pub price: String,
    #[serde(rename = "usdprice")]
    usd_price: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Location {
    pub cities: Vec<City>,
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