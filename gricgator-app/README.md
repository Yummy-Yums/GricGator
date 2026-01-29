# GricGator

## Agriculture Market Intelligence Platform

---

### What is GricGator?

GricGator is a market aggregation platform designed to empower smallholder farmers and agricultural stakeholders with real-time pricing data, weather forecasts, and market intelligence.

### Quick Start

**Usage:** CLI application

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