use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct TimeSeriesDaily {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. volume")]
    volume: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AlphaVantageResponse {
    #[serde(rename = "Time Series (Daily)")]
    time_series_daily: std::collections::HashMap<String, TimeSeriesDaily>,
}

fn main() {
    let api_key = "YOUR_API_KEY_HERE";
    let symbol = "TSLA";

    // Make a request to the Alpha Vantage API to retrieve the daily time series for TSLA
    let url = format!("https://www.alphavantage.co/query?function=TIME_SERIES_DAILY_ADJUSTED&symbol={}&apikey={}", symbol, api_key);
    let response_text = reqwest::blocking::get(&url).unwrap().text().unwrap();

    // Deserialize the response into a struct
    let response: AlphaVantageResponse = serde_json::from_str(&response_text).unwrap();

    // Get the latest time series entry
    let latest_date = response.time_series_daily.keys().next().unwrap();
    let latest_entry = &response.time_series_daily[latest_date];

    // Print the latest close price
    println!("The current stock price of TSLA is: {}", latest_entry.close);
}
