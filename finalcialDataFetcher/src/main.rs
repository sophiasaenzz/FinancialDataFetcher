//build rust application that periodically fetches and records the pricing data of bitcoin, ethereum and the SP500 index
//use ureq::AgentBuilder; //ureq for making http requests
use std::time::Duration;
use serde::{Deserialize, Serialize}; //serde for parsing json data into rust structs
use std::fs::File; //file for writing to files
use std::error::Error;
use serde_json::Value;


//structs 
//bitcoin, ethereum, sp500
#[derive(Serialize, Deserialize, Debug)]
pub struct Bitcoin {
    price: f64,
    //time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ethereum {
    price: f64,
    //time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SP500 {
    price: f64,
    //time: String,
}

//define a pricing trait with the following methods
//fetch_price() fetches the latest price from a API
//save_to_file() saves the fetched price to a file

/* 
#[derive(Debug)]
enum ApiResult {
    Success(f64),
    ApiError(String),
    NetworkError(String),    
} */

trait Pricing {
    fn fetch_price(&self) -> Result<f64, Box<dyn Error>>;
    fn save_to_file(&self);
}

////implement the Pricing trait for the Bitcoin struct
impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64, Box<dyn Error>> {
        let url = "https://api.coindesk.com/v1/bpi/currentprice.json";
        let response = ureq::get(url).call();

        match response {
            Ok(response)  => { 
                if response.status() == 200 {
                    let body = response.into_string()?;
                    
                    // Parse the response string as JSON
                    let data: Value = serde_json::from_str(&body)?;
                    
                    // Extract the price from the JSON
                    if let Some(price) = data["current_price"].as_f64() {
                        Ok(price)
                    } else {
                        Err("Failed to extract Bitcoin price from response".into())
                    }
                } else {
                    Err(format!("Unexpected status: {}", response.status()).into())
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }
    

    fn save_to_file(&self) {
        match self.fetch_price() {
            Ok(price) => {
                let bitcoin = Bitcoin { price };
                let file = File::create("bitcoin.txt").unwrap();
                serde_json::to_writer(file, &bitcoin).unwrap();
            }
            Err(e) => eprintln!("Failed to fetch price: {}", e),
        }
        
    }
}


//implement the Pricing trait for the Ethereum struct
impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64, Box<dyn Error>> {
        let url = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd";
        let response = ureq::get(url).call();  //change some of the fields

        match response {
            Ok(response)  => { 
                if response.status() == 200 {
                    let body = response.into_string()?;
                    
                    // Parse the response string as JSON
                    let data: Value = serde_json::from_str(&body)?;
                    
                    // Extract the price from the JSON
                    if let Some(price) = data["Ethereum"]["USD"]["rate_float"].as_f64() {
                        Ok(price)
                    } else {
                        Err("Failed to extract Bitcoin price from response".into())
                    }
                } else {
                    Err(format!("Unexpected status: {}", response.status()).into())
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }
    

    fn save_to_file(&self) {
        match self.fetch_price() {
            Ok(price) => {
                let ethereum = Ethereum { price };
                let file = File::create("ethereum.txt").unwrap();
                serde_json::to_writer(file, &ethereum).unwrap();
            }
            Err(e) => eprintln!("Failed to fetch price: {}", e),
        }
    }
}

//implement the Pricing trait for the SP500 struct
impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64, Box<dyn Error>> {
        let url = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd";
        let response = ureq::get(url).call();  //change some of the fields

        match response {
            Ok(response)  => { 
                if response.status() == 200 {
                    let body = response.into_string()?;
                    
                    // Parse the response string as JSON
                    let data: Value = serde_json::from_str(&body)?;
                    
                    // Extract the price from the JSON
                    if let Some(price) = data["Ethereum"]["USD"]["rate_float"].as_f64() {
                        Ok(price)
                    } else {
                        Err("Failed to extract Bitcoin price from response".into())
                    }
                } else {
                    Err(format!("Unexpected status: {}", response.status()).into())
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    fn save_to_file(&self) {
        match self.fetch_price() {
            Ok(price) => {
                let ethereum = Ethereum { price };
                let file = File::create("SP500.txt").unwrap();
                serde_json::to_writer(file, &ethereum).unwrap();
            }
            Err(e) => eprintln!("Failed to fetch price: {}", e),
        }
    }
}




/*enum Common {
    Bitcoin(Bitcoin),
    Ethereum(Ethereum),
    SP500(SP500),
  } */
  

fn main() {
    //println!("Hello, world!");

    //create a vector containing instances of the Bitcoin, Ethereum and SP500 structs
    let pricing: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin { price: 0.0 }),
        //Box::new(Ethereum { price: 0.0 }),
        //Box::new(SP500 { price: 0.0 }),
    ];



    // Periodically fetch and save prices
    loop {
        for p in &pricing {
            p.save_to_file();
        }
        std::thread::sleep(Duration::from_secs(10));
    }
}