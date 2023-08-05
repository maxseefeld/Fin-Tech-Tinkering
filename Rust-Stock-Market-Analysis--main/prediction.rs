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

    // Extract the closing prices from the time series data
    let mut closing_prices: Vec<f64> = response.time_series_daily.values()
        .map(|entry| entry.close.parse::<f64>().unwrap())
        .collect();

    // Reverse the prices so that they are in chronological order
    closing_prices.reverse();

    // Calculate the Simple Moving Average (SMA) of the closing prices
    let period = 10; // Use a 10-day SMA
    let sma: Vec<f64> = closing_prices.windows(period)
        .map(|window| window.iter().sum::<f64>() / period as f64)
        .collect();

    // Use the last SMA value to predict the next week's stock price
    let last_sma = sma.last().unwrap();
    let predicted_price = last_sma * 1.05; // Assume a 5% increase in price

    // Print the predicted price
    println!("The predicted stock price of TSLA next week is: {}", predicted_price);
}
