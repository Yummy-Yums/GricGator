use std::cmp::Ordering;

use std::fmt::{Formatter, Display, Result as FmtResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherCondition {
    pub text: String,
    icon: String,
    code: i16
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Hour {
    pub time: String,
    pub temp_c: f32,
    pub condition: WeatherCondition,
    pub will_it_rain: i8,
    pub chance_of_rain: i16,
    pub humidity: i16,
    #[serde(rename = "dewpoint_c")]
    pub dew_point: f32,
}

// have an impl block for hour to properly format the code here
impl Display for Hour {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let time = self.time.split(' ').collect::<Vec<&str>>()[1];
        let msg = format!(
            "Hourly Forecast at {}: the temperature is {}, the condition is {}. Humidity is {}",
            time,
            self.temp_c,
            self.condition.text.trim(),
            self.humidity
        );
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AstrologicalData {
    pub sunrise: String,
    pub sunset: String,
    pub moonrise: String,
    pub moonset: String,
    pub moon_phase: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Day {
    #[serde(rename = "maxtemp_c")]
    pub max_temp_in_celsius: f32,
    #[serde(rename = "mintemp_c")]
    pub min_temp_in_celsius: f32,
    #[serde(rename = "avgtemp_c")]
    pub avg_temp_c: f32,
    #[serde(rename = "maxwind_mph")]
    pub max_wind_mph: f32,
    #[serde(rename = "totalprecip_mm")]
    pub total_precipitation: f32,
    #[serde(rename = "avghumidity")]
    pub average_humidity: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ForecastDay {
    pub date: String,
    pub day: Day,
    pub astro: AstrologicalData,
    pub hour: Vec<Hour>
}

#[derive(Debug, Deserialize)]
pub struct Forecast {
    #[serde(rename = "forecastday")]
    pub forecast_day: Vec<ForecastDay>,
}

#[derive(Debug, Deserialize)]
pub struct WeatherForecastResponse {
    pub forecast: Forecast,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherData {
    pub temp_c: f32,
    pub is_day: i8,
    #[serde(rename = "feelslike_c")]
    pub feels_like: f32,
    #[serde(rename = "windchill_c")]
    pub wind_chill: f32,
    pub condition: WeatherCondition,
    pub wind_mph: f32,
    pub humidity: f32,
    #[serde(rename = "precip_mm")]
    pub precipitation: f32,
}

#[derive(Debug, Deserialize)]
pub struct WeatherApiResponse {
    pub current: WeatherData
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WindData {
    speed: i16,
    direction: String,
    gust: String
}

#[derive(Debug, Clone)]
pub struct Commodity {
    pub id: i8,
    pub name: String,
    pub price: String,
    pub category: String,
    pub unit: String,
    pub price_type: String,
    pub currency: String,
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

impl Display for CommodityPricing {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let msg = format!("\
        The price per {} is {}{} in the {} market
        ", self.unit, self.currency, self.price, self.market);
        write!(f, "{}", msg)
    }
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
    pub market_id: i16,
    latitude: f32,
    longitude: f32,
    pub category: String,
    pub commodity: String,
    commodity_id: String,
    pub unit: String,
    #[serde(rename = "priceflag")]
    price_flag: String,
    #[serde(rename = "pricetype")]
    pub price_type: String,
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