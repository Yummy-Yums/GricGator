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
• cli weather get-forecast Accra
• cli weather current Kumasi

💰 pricing       Access commodity pricing across markets
Examples:
• cli pricing get-price maize --region Central
• cli pricing best-markets cassava
• cli pricing trends rice --days 7

🌽 commodity     Explore available commodities and markets
Examples:
• cli commodity list
• cli commodity info maize
• cli commodity markets

❓ help          Print this message or command-specific help
Examples:
• cli help
• cli weather --help
• cli pricing --help

💡 Examples
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Find best prices for maize in your region:
$ cli pricing best-markets maize --region Central

Check if it's a good day to harvest:
$ cli weather get-forecast Tamale

See 7-day price trends:
$ cli pricing trends cassava --days 7


🤝 Support
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Built with ❤️ for African farmers
Powered by Rust • Data from FAO GIEWS, WFP, WeatherAPI

For help with any command, use:
cli <COMMAND> --help