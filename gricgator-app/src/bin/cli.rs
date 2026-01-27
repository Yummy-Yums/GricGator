use std::fmt::format;
use clap::{Parser, Subcommand};
use gricgator_app::*;

const HEADER: &str = "\
______________________
|  GRICGATOR   APP   |
______________________
";

#[derive(Parser)]
#[clap(name = "Gricgator CLI")]
#[clap(about = "Markets Aggregator for Agriculture StakeHolders")]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Weather Subcommand to weather functionality
    Weather {
        #[clap(subcommand)]
        weather_cmd: WeatherCommands
    },
    /// Pricing Subcommand to Pricing functionality
    Pricing {
        #[clap(subcommand)]
        pricing_cmd: CommodityPricingCommands
    },
    /// Pricing Subcommand to Commodities functionality
    Commodity {
        #[clap(subcommand)]
        commodity_cmd: CommodityCommands
    },

}

#[derive(Subcommand)]
enum WeatherCommands {
    /// List Available Locations
    ListAvailableLocations,
    /// Get Current Weather of a place
    GetCurrentWeather(CommonWeatherArgs),
    /// Get Weather Forecast for the day of a place
    GetWeatherForecast(CommonWeatherArgs),
    /// Morning (6am - 12pm) Weather Forecast of a place
    GetMorningWeatherForecast(CommonWeatherArgs),
    /// Afternoon (12pm - 4pm) Weather Forecast of a place
    GetAfternoonWeatherForecast(CommonWeatherArgs),
    /// Evening (4pm - 8pm) Weather Forecast of a place
    GetEveningWeatherForecast(CommonWeatherArgs),
    /// Night (8pm - 12pm) Weather Forecast of a place
    GetNightWeatherForecast(CommonWeatherArgs),
}

#[derive(clap::Args)]
#[derive(Debug)]
struct CommonWeatherArgs {
    location: Option<String>,
}


#[derive(Subcommand)]
enum CommodityCommands {
    /// list all commodities available
    ListCommodities,
    /// list the categories of commodities available
    ListCategories,
    /// list commodites available in a category
    ListCommoditiesInCategory{
        #[clap(long)]
        category: String
    },
}

#[derive(Subcommand)]
enum CommodityPricingCommands {
    /// get best market price of commodity
    GetBestMarketPrice {
        #[clap(long)]
        commodity: String,
    },
    /// Get the best regional market price of commodity
    GetBestRegionalMarketPrice {
        #[clap(long)]
        commodity: String,
        #[clap(long)]
        region: String,
    },
    /// Get price of commodity across all markets
    GetPriceAcrossAllMarkets {
        #[clap(long)]
        commodity: String,
    }
}

fn formatted_header(title: &str) -> String {
    format!("\
_______________________
   {:>6}
|---------------------|
", title)
}

#[tokio::main]
async fn main() {

    println!("{}", HEADER);

    init_api_key();

    let cli = Cli::parse();

    match cli.commands {
        Commands::Weather { weather_cmd} =>
            match weather_cmd {
                WeatherCommands::ListAvailableLocations => {
                    println!("Getting Available locations\n");

                    let available_locations = get_avail_locations_data();
                    let max_len = available_locations.iter().map(|s| s.0.len()).max().unwrap();

                    available_locations
                        .keys()
                        .for_each(|key| {
                            println!("{:width$}", key, width=max_len);
                            println!("{}", "=".repeat(max_len));
                        });
                },
                WeatherCommands::GetCurrentWeather(location) => {

                    let loc = &location.location.expect("No location specified");
                    let is_city_available= is_city_or_location_available(loc);
                    println!("Current Weather Of {:?}", loc);

                    if is_city_available {
                        println!("{}", "=".repeat(30));

                        // add retry block here
                        let _ = retry(|| async {
                            get_current_weather(loc).await
                        }, 3).await;

                    } else {
                        let error_msg = format!(
                            "City'{}' is not available \n\
                                \n\
                                Please:\n\
                                \n\
                                • Run 'list-available-locations' to see available options\n\
                                • Check your spelling\n\
                            ",
                            loc
                        );
                        println!("{error_msg}");
                    }
                },
                WeatherCommands::GetWeatherForecast(location) => {
                    println!("Get Weather Forecast {:?}", location.location);
                    println!("{}", "=".repeat(30));

                    let loc = &location.location.unwrap().to_lowercase();

                     let _ = retry( || async {
                          get_weather_forecast(&loc).await
                     }, 3).await;
                },
                WeatherCommands::GetMorningWeatherForecast(location) => {
                    let loc = location.location.expect("No location specified");
                    println!("Get Morning Weather Forecast {:?}", loc);

                    let response = get_time_period_forecast_of_day(&*loc, "Morning").await.unwrap();
                    response.iter().for_each(|er| { println!("{:?}", er)})
                },
                WeatherCommands::GetAfternoonWeatherForecast(location) => {
                    let loc = location.location.expect("No location specified");
                    println!("Get Afternoon Weather Forecast {:?}", loc);

                    let response = get_time_period_forecast_of_day(&*loc, "Afternoon").await.unwrap();
                    response.iter().for_each(|er| { println!("{:?}", er)})
                },
                WeatherCommands::GetEveningWeatherForecast(location) => {
                    let loc = location.location.expect("No location specified");
                    println!("Get Evening Weather Forecast {:?}", loc);

                    let response = get_time_period_forecast_of_day(&*loc, "Evening").await.unwrap();
                    response.iter().for_each(|er| { println!("{:?}", er)})
                },
                WeatherCommands::GetNightWeatherForecast(location) => {
                    let loc = location.location.expect("No location specified");
                    println!("Get Night Weather Forecast {:?}", loc);

                    let response = get_time_period_forecast_of_day(&*loc, "Night").await.expect("Invalid Time period");
                    response.iter().for_each(|er| { println!("{:?}", er)})
                }
            },
        Commands::Pricing {pricing_cmd} =>
            match pricing_cmd {
                CommodityPricingCommands::GetBestMarketPrice { commodity } => {
                    println!("Get best market price of commodities");

                    let result = get_best_market_prices_of_commodity(commodity.as_str());
                    match result {
                        Ok(result) => {
                            if result.is_empty() {
                                println!("No commodity pricing found for {}", commodity);

                                let error_msg = format!(
                                    "No such commodity as '{}'\n\
                                    \n\
                                    Please:\n\
                                    \n\
                                    • Run 'list-commodities' to see available options\n\
                                    • Check your spelling\n\
                               ",
                                    commodity
                                );
                                println!("{error_msg}");
                            } else {
                                println!("Top Best market price for {} {:?}", commodity, result);

                                result
                                    .iter()
                                    .for_each(|commodity_pricing| {
                                        println!("{}", commodity_pricing);
                                        println!("=================\n")
                                    })
                            }
                        },
                        Err(error) => {
                            println!("Error getting best market price: {:?}", error);
                        }
                    }
                }
                CommodityPricingCommands::GetBestRegionalMarketPrice {
                    commodity,
                    region
                } => {
                    println!("Get best  market price of commodities in {} region...\n", region);

                    let result = get_best_market_prices_of_commodity_in_a_region(
                        commodity.as_str(),
                        region.as_str()
                    );

                    match result {
                        Ok(result) => {
                            if result.is_empty() {

                                let error_msg = format!(
                                    "No such commodity as '{}' or region '{}'\n\
                                \n\
                                Please:\n\
                                \n\
                                • Run 'list-commodities' to see available options\n\
                                • Check your spelling whether regions are correct\n\
                               ",
                                    commodity,
                                    region
                                );
                                println!("{error_msg}");
                            } else {
                                println!("Top Best market price for {} in {} region\n", commodity, region);

                                result
                                    .iter()
                                    .for_each(|commodity_pricing| {
                                        println!("{}", commodity_pricing);
                                        println!("=================\n")
                                    })
                            }
                        },
                        Err(error) => {
                            println!("Error getting best market price: {:?}", error);
                        }
                    }
                }
                CommodityPricingCommands::GetPriceAcrossAllMarkets { commodity } => {
                    println!("Get best price of commodity across all best markets");
                }
            },
        Commands::Commodity { commodity_cmd } =>
            match commodity_cmd {
                CommodityCommands::ListCommodities => {
                    let all_commodities = list_all_commodities();
                    let max_len= all_commodities.iter().map(|s| s.len()).max().unwrap_or(0);

                    println!("{}", formatted_header("Commodities"));

                    all_commodities
                        .iter()
                        .for_each(|commodity| {
                            println!("|{:width$}|", commodity, width = max_len);
                            println!("|{:width$}|", "_".repeat(max_len), width = max_len);
                        });
                },
                CommodityCommands::ListCategories => {
                    println!("Listing categories of commodities");

                    let categories = list_categories_of_commodities();
                    let max_len= categories.iter().map(|s| s.len()).max().unwrap_or(0);

                    println!("{}", formatted_header("Categories"));

                    categories
                        .iter()
                        .for_each(|commodity| {
                            println!("|{:width$}|", commodity, width = max_len);
                            println!("|{:width$}|", "_".repeat(max_len), width = max_len);
                        });
                },
                CommodityCommands::ListCommoditiesInCategory {
                    category,
                } => {
                    println!("Listing commodities in categories.....\n");

                    let commodities_in_category = get_commodities_in_a_category(&category);

                    match commodities_in_category {
                        Some(commodities) => {
                            let max_len = commodities.iter().map(|c| c.name.len()).max().unwrap_or(0);
                            commodities
                                .iter()
                                .for_each(|commodity| {
                                    println!("|{:width$}|", commodity.name, width = max_len);
                                    println!("|{:width$}|", "_".repeat(max_len), width = max_len);
                                });
                        },
                        None => {
                            let error_msg = format!(
                                "No such category as '{}'\n\
                                \n\
                                Please:\n\
                                \n\
                                • Run 'list-categories' to see available options\n\
                                • Check your spelling\n\
                                ",
                                category
                            );
                            println!("{error_msg}");
                        }
                    }
                },
            }

    }
}