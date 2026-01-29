┌──────────────────────────────────────────────────────────────────┐
│                      🌾 GRICGATOR APP 🌾                         │
│          Agriculture Market Intelligence Platform               │
└──────────────────────────────────────────────────────────────────┘

📊 What is GricGator?
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
GricGator is a market aggregation platform designed to empower
smallholder farmers and agricultural stakeholders with real-time
pricing data, weather forecasts, and market intelligence.

🚀 Quick Start
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Usage: cli <COMMAND>

📋 Available Commands
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🌦️  weather      Get weather forecasts and farming advisories
Examples:
• cli weather get-evening-weather-forecast Accra
• cli weather get-current-weather Accra

💰 pricing       Access commodity pricing across markets
Examples:
• cli pricing get-best-regional-market-price --commodity "cassava" --region "western"
• cli pricing get-best-market-price --commodity "cassava"

🌽 commodity     Explore available commodities and markets
Examples:
• cli commodity list-commodities
• cli commodity list-categories

💡 Examples
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Find best prices for maize in your region:
$ cli pricing get-best-regional-market-price --commodity "maize" --region "western"

Check if it's a good day to harvest:
$ cli weather get-weather-forecast Tamale


🤝 Support
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Built with ❤️ for African farmers
Powered by Rust • Data from FAO GIEWS, WFP, WeatherAPI