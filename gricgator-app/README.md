# GricGator

## Agriculture Market Intelligence Platform

---

### What is GricGator?

GricGator is a market aggregation platform designed to empower smallholder farmers and agricultural stakeholders with real-time pricing data, weather forecasts, and market intelligence.

### Quick Start

**Usage:** CLI application

Create a .env in the root folder and paste for the weather api service to work 
**API_KEY=00868b6d053d4ebaa41132701261501** (I'm aware it's not a good practice :)

---

### Available Commands

#### Weather
Get weather forecasts and farming advisories.

**Examples:**
- `cli weather get-evening-weather-forecast Accra`
- `cli weather get-current-weather Accra`

#### Pricing
Access commodity pricing across markets.

**Examples:**
- `cli pricing get-best-regional-market-price --commodity "cassava" --region "western"`
- `cli pricing get-best-market-price --commodity "cassava"`

#### Commodity
Explore available commodities and markets.

**Examples:**
- `cli commodity list-commodities`
- `cli commodity list-categories`

---

### Examples

**Find best prices for maize in your region:**
```bash
$ cli pricing get-best-regional-market-price --commodity "maize" --region "western"