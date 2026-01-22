use std::fmt::format;
use clap::{Parser, Subcommand};
use gricgator_app::{get_commodities_in_a_category, init_api_key, list_all_commodities, list_categories_of_commodities};

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
    CommodityPricing {
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
    ListAvailableLocations(CommonWeatherArgs),
    /// Get Current Weather of a place
    GetCurrentWeather(CommonWeatherArgs),
    /// Get Weather Forecast for the day of a place
    GetWeatherForecast(CommonWeatherArgs),
    /// Morning (6am - 12pm) Weather Forecast of a place
    GetMorningWeatherForecast(CommonWeatherArgs),
    /// Afternoon (12pm - 5pm) Weather Forecast of a place
    GetAfternoonWeatherForecast(CommonWeatherArgs),
    /// Evening (5pm - 8pm) Weather Forecast of a place
    GetEveningWeatherForecast(CommonWeatherArgs),
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
        regional: String,
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

fn main() {

    println!("{}", HEADER);

    init_api_key();

    let cli = Cli::parse();

    match cli.commands {
        Commands::Weather {weather_cmd} =>
            match weather_cmd {
                WeatherCommands::ListAvailableLocations(location) => {
                    println!("Available locations: {:?}", location);
                },
                WeatherCommands::GetCurrentWeather(location) => {
                    println!("Current Weather {:?}", location);
                },
                WeatherCommands::GetWeatherForecast(location) => {
                    println!("Get Weather Forecast {:?}", location);
                },
                WeatherCommands::GetMorningWeatherForecast(location) => {
                    println!("Get Morning Weather Forecast {:?}", location);
                },
                WeatherCommands::GetAfternoonWeatherForecast(location) => {
                    println!("Get Afternoon Weather Forecast {:?}", location);
                },
                WeatherCommands::GetEveningWeatherForecast(location) => {
                    println!("Get Evening Weather Forecast {:?}", location);
                }
            },
        Commands::CommodityPricing {pricing_cmd} =>
            match pricing_cmd {

                CommodityPricingCommands::GetBestMarketPrice { commodity } => {
                    println!("Get best market price of commodities");
                }
                CommodityPricingCommands::GetBestRegionalMarketPrice {
                    commodity,
                    regional
                } => {
                    println!("Get best regional market price of commodities");
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
                                • Use tab completion if available",
                                                            category
                            );
                            println!("{error_msg}");
                        }
                    }
                },
            }

    }
}