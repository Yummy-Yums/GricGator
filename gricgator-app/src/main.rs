mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   services::weather_service::fetch().await
    
}