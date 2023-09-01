// ----------------------------------------------------------------------------------
// Lagertha Consulting - Cryptocurrency Analysis Tool
// Copyright (C) 2023 Lagertha Consulting, All rights reserved.
// Senior Developer: Sid Berraf
// Purpose: Fetch cryptocurrency data from CoinGecko and compute MACD for analysis.
// ----------------------------------------------------------------------------------

use reqwest;
use serde_json::Value;  

const BASE_URL: &str = "https://api.coingecko.com/api/v3/coins";


/// Fetches historical data for a given cryptocurrency from CoinGecko.
///
/// # Arguments
/// * `id` - The ID of the cryptocurrency (e.g., "bitcoin").
/// * `days` - The number of past days to retrieve data for.
///
/// # Returns
/// A `Result` containing a vector of `(timestamp, price)` or an error.
async fn fetch_data(id: &str, days: u32) -> Result<Vec<(f64, f64)>, reqwest::Error> {
    let endpoint = format!("{}/{}/market_chart?vs_currency=usd&days={}&interval=daily", BASE_URL, id, days);
    let response: Value = reqwest::get(&endpoint).await?.json().await?;

    let prices = response["prices"].as_array().unwrap().iter().map(|entry| {
        let timestamp = entry[0].as_f64().expect("Expected a float timestamp");
        let price = entry[1].as_f64().expect("Expected a float price");
        (timestamp, price)
    }).collect::<Vec<_>>();

    Ok(prices)
}

/// Computes the Exponential Moving Average (EMA) for given price data and periods.
///
/// # Arguments
/// * `prices` - A reference to a vector of `(timestamp, price)`.
/// * `periods` - The number of periods for the EMA.
///
/// # Returns
/// A vector containing the EMA values.
fn compute_ema(prices: &[(f64, f64)], periods: usize) -> Vec<f64> {
    // This is a placeholder for the EMA computation logic.
    // Best to rely on a well-tested library for actual implementations.
    vec![] 
}

/// Computes the Moving Average Convergence Divergence (MACD) and its signal line.
///
/// # Arguments
/// * `prices` - A reference to a vector of `(timestamp, price)`.
///
/// # Returns
/// A tuple containing two vectors: MACD values and Signal Line values.
fn compute_macd(prices: &[(f64, f64)]) -> (Vec<f64>, Vec<f64>) {
    let ema12 = compute_ema(prices, 12);
    let ema26 = compute_ema(prices, 26);
    let macd: Vec<_> = ema12.iter().zip(ema26.iter()).map(|(&a, &b)| a - b).collect();
    let signal = compute_ema(&macd.iter().enumerate().map(|(i, &val)| (prices[i].0, val)).collect::<Vec<_>>(), 9);
    (macd, signal)
}

#[tokio::main]
async fn main() {
    let data = fetch_data("bitcoin", 90).await.expect("Failed to fetch data for Bitcoin");
    let (macd, signal) = compute_macd(&data);
    // Further analysis or visualization using MACD and Signal Line can be implemented here.
}
