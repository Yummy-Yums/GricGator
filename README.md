# GricGator
Agriculture Market Aggregator

--

## 🎯 Problem

Smallholder farmers struggle to:

- Know **current market prices**
- Predict **best selling times**
- Access **weather + demand signals** in one place  
    This leads to middlemen exploitation and poor planning.
    
---

## 💡 Solution

A **Rust-powered backend service** that aggregates:

- 🏷️ Crop prices from multiple markets
    
- 🌦️ Weather forecasts
    
- 📈 Simple demand trends
    
And exposes this via:

- REST API
- CLI (for low-bandwidth users)
- Optional web dashboard / mobile frontend
    
---

## 🧱 Core Features (Hackathon Scope)

### 1️⃣ Market Price Aggregation

- Input sources:
    - CSV uploads (mock market data)
    - Public APIs (optional)
        
- Data stored per:
    - Crop (maize, rice, cassava, etc.)
    - Location
    - Date
        
```json
{
  "crop": "maize",
  "market": "Accra",
  "price_per_kg": 4.20,
  "date": "2026-01-05"
}
```

---

### 2️⃣ Weather Intelligence

- Pull:
    - Temperature
    - Rainfall forecast
        
- Show **actionable advice**, not raw data:
    - “Good week to harvest”
    - “Delay planting – heavy rainfall expected”
        
---

### 3️⃣ Price Trend Insights (Simple but Smart)

- Rust logic to compute:
    - 7-day average
    - Price increase/decrease %
        
- Output:
    - 🔺 Rising
    - 🔻 Falling
    - ➖ Stable
        
---

### 4️⃣ Farmer-Friendly Access

#### Option A: CLI Tool (Very Rusty 💙)

```bash
gricgator price maize --location accra
gricgator weather --location kumasi
```

Perfect for:

- Low bandwidth
- Judges who love terminals
    

#### Option B: Web API

- `/prices`
- `/weather`
- `/trends`
    

Frontend can be:
- WASM + Yew
- Simple React consuming Rust API
    
---

## 🦀 Rust Tech Stack (Judges Will Love This)

### Backend

- **Axum** or **Actix-web** – API
- **Serde** – JSON
- **SQLx** – Database (Postgres / SQLite)
- **Tokio** – async fetching
- **Reqwest** – external APIs
    
### Data Storage

- SQLite (simple, demo-friendly)
- Later: Postgres
    
---

## 🏗️ Architecture (Simple & Clean)

```
┌────────────┐
│  CLI / UI  │
└─────┬──────┘
      │ REST
┌─────▼──────┐
│ Rust API   │
│ (Axum)     │
├────────────┤
│ Aggregator │◄── Market APIs / CSV
│ Engine     │◄── Weather APIs
├────────────┤
│ Database   │
└────────────┘
```

---

## 🌍 Africa-Relevant Enhancements (Big Points)

Choose **1–2**, not all:

- 🌐 Multi-language support (EN + local language)
    
- 📡 Offline-first caching
    
- 📲 SMS-ready API (future scope)
    
- 🧑🏽‍🌾 Focus on 3–5 staple crops only
    
---

### ✅ Impact

- Directly helps farmers earn more
- Addresses real market inefficiencies
    

### ✅ Technical Quality

- Clean Rust async architecture
- Proper error handling
- Strong type safety

### ✅ Usability

- CLI + simple dashboard
- Clear insights, not raw data

### ✅ Innovation

- Aggregation + insight layer (not just data dump)
    
---
