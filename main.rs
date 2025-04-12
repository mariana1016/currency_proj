// src/main.rs
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::thread;
use std::time::Duration;
use chrono::Local;
use serde::Deserialize;
use std::error::Error;
use std::fmt;

// Define custom error type for better error handling
#[derive(Debug)]
enum PriceError {
    NetworkError(String),
    ParseError(String),
    FileError(String),
}

impl fmt::Display for PriceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PriceError::NetworkError(msg) => write!(f, "Network Error: {}", msg),
            PriceError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            PriceError::FileError(msg) => write!(f, "File Error: {}", msg),
        }
    }
}

impl Error for PriceError {}

// Define the Pricing trait
trait Pricing {
    fn fetch_price(&self) -> Result<f64, PriceError>;
    fn save_to_file(&self, price: f64) -> Result<(), PriceError>;
    fn name(&self) -> &str;
}

// Bitcoin struct
struct Bitcoin {
    filename: String,
}

// Ethereum struct
struct Ethereum {
    filename: String,
}

// SP500 struct
struct SP500 {
    filename: String,
}

// Response structs for API data
#[derive(Deserialize)]
struct CoinGeckoResponse {
    #[serde(rename = "usd")]
    price: f64,
}

#[derive(Deserialize)]
struct AlphaVantageResponse {
    #[serde(rename = "Global Quote")]
    global_quote: GlobalQuote,
}

#[derive(Deserialize)]
struct GlobalQuote {
    #[serde(rename = "05. price")]
    price: String,
}

// Implementation of Pricing trait for Bitcoin
impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64, PriceError> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        let response = ureq::get(url)
            .call()
            .map_err(|e| PriceError::NetworkError(e.to_string()))?;
        
        let response_str = response.into_string()
            .map_err(|e| PriceError::ParseError(e.to_string()))?;
        
        let json: serde_json::Value = serde_json::from_str(&response_str)
            .map_err(|e| PriceError::ParseError(e.to_string()))?;
        
        // Extract the price from the nested structure
        json.get("bitcoin")
            .and_then(|btc| btc.get("usd"))
            .and_then(|price| price.as_f64())
            .ok_or_else(|| PriceError::ParseError("Failed to extract Bitcoin price".to_string()))
    }

    fn save_to_file(&self, price: f64) -> Result<(), PriceError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.filename)
            .map_err(|e| PriceError::FileError(e.to_string()))?;
        
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let data = format!("{},{:.2}\n", timestamp, price);
        
        file.write_all(data.as_bytes())
            .map_err(|e| PriceError::FileError(e.to_string()))?;
        
        println!("[{}] Bitcoin: ${:.2}", timestamp, price);
        Ok(())
    }

    fn name(&self) -> &str {
        "Bitcoin"
    }
}

// Implementation of Pricing trait for Ethereum
impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64, PriceError> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        let response = ureq::get(url)
            .call()
            .map_err(|e| PriceError::NetworkError(e.to_string()))?;
        
        let response_str = response.into_string()
            .map_err(|e| PriceError::ParseError(e.to_string()))?;
        
        let json: serde_json::Value = serde_json::from_str(&response_str)
            .map_err(|e| PriceError::ParseError(e.to_string()))?;
        
        // Extract the price from the nested structure
        json.get("ethereum")
            .and_then(|eth| eth.get("usd"))
            .and_then(|price| price.as_f64())
            .ok_or_else(|| PriceError::ParseError("Failed to extract Ethereum price".to_string()))
    }

    fn save_to_file(&self, price: f64) -> Result<(), PriceError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.filename)
            .map_err(|e| PriceError::FileError(e.to_string()))?;
        
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let data = format!("{},{:.2}\n", timestamp, price);
        
        file.write_all(data.as_bytes())
            .map_err(|e| PriceError::FileError(e.to_string()))?;
        
        println!("[{}] Ethereum: ${:.2}", timestamp, price);
        
        Ok(())
    }

    fn name(&self) -> &str {
        "Ethereum"
    }
}

// Implementation of Pricing trait for SP500
impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64, PriceError> {
        // Note: Alpha Vantage requires an API key, you'll need to get one
        let api_key = "demo"; // Replace with your actual API key
        let url = format!("https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m={}", api_key);
        
        let response = ureq::get(&url)
            .call()
            .map_err(|e| PriceError::NetworkError(e.to_string()))?;
        
        let response_str = response.into_string()
            .map_err(|e| PriceError::ParseError(e.to_string()))?;
        
        let response_data: AlphaVantageResponse = serde_json::from_str(&response_str)
            .map_err(|e| PriceError::ParseError(e.to_string()))?;
        
        // Parse the price string to a float
        response_data.global_quote.price.parse::<f64>()
            .map_err(|e| PriceError::ParseError(e.to_string()))
    }

    fn save_to_file(&self, price: f64) -> Result<(), PriceError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.filename)
            .map_err(|e| PriceError::FileError(e.to_string()))?;
        
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let data = format!("{},{:.2}\n", timestamp, price);
        
        file.write_all(data.as_bytes())
            .map_err(|e| PriceError::FileError(e.to_string()))?;
        
        println!("[{}] S&P 500: ${:.2}", timestamp, price);
    
        Ok(())
    }

    fn name(&self) -> &str {
        "S&P 500"
    }
}

fn main() {
    // Create instances of each asset
    let assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin { filename: "bitcoin_prices.csv".to_string() }),
        Box::new(Ethereum { filename: "ethereum_prices.csv".to_string() }),
        Box::new(SP500 { filename: "sp500_prices.csv".to_string() }),
    ];

    // Create header for each CSV file if it doesn't exist
    for asset in &assets {
        if let Err(_) = File::open(match asset.as_ref() {
            asset if asset.name() == "Bitcoin" => "bitcoin_prices.csv",
            asset if asset.name() == "Ethereum" => "ethereum_prices.csv",
            _ => "sp500_prices.csv",
        }) {
            let mut file = File::create(match asset.as_ref() {
                asset if asset.name() == "Bitcoin" => "bitcoin_prices.csv",
                asset if asset.name() == "Ethereum" => "ethereum_prices.csv",
                _ => "sp500_prices.csv",
            }).unwrap();

            file.write_all(b"timestamp,price\n").unwrap();
        }
    }

    println!("Starting price tracker...");
    println!("Press Ctrl+C to stop the program");

    // Main loop to fetch and save prices
    loop {
        for asset in &assets {
            match asset.fetch_price() {
                Ok(price) => {
                    if let Err(e) = asset.save_to_file(price) {
                        eprintln!("Error saving price for {}: {}", asset.name(), e);
                    }
                },
                Err(e) => {
                    eprintln!("Error fetching price for {}: {}", asset.name(), e);
                }
            }
        }

        // Wait for 10 seconds before the next fetch
        thread::sleep(Duration::from_secs(10));
    }
}