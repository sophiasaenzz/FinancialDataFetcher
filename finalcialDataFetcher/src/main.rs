use std::io::Write;
//build rust application that periodically fetches and records the pricing data of bitcoin, ethereum and the SP500 index
use std::time::Duration;
use serde::{Deserialize, Serialize}; //serde for parsing json data into rust structs
use std::fs::OpenOptions; //file for writing to files
use std::error::Error;
use serde_json::Value;


//structs 
//bitcoin, ethereum, sp500
#[derive(Serialize, Deserialize, Debug)]
pub struct Bitcoin {
    price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ethereum {
    price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SP500 {
    price: f64,
}

//define a pricing trait with the following methods
//fetch_price() fetches the latest price from a API
//save_to_file() saves the fetched price to a file

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
                    if let Some(price) = data["bpi"]["USD"]["rate_float"].as_f64() {
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

                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open("bitcoin.txt")
                    .unwrap();

                file.write_all(bitcoin.price.to_string().as_bytes()).unwrap();

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
                    if let Some(ethereum_data) = data[1]["current_price"].as_f64() {
                        Ok(ethereum_data)
                    } else {
                        Err("Failed to extract ethereum price from response".into())
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
                //let file = File::create("ethereum.txt").unwrap();
                
                let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("ethereum.txt")
                .unwrap();

                file.write_all(ethereum.price.to_string().as_bytes()).unwrap();
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
                    if let Some(price) = data[3]["current_price"].as_f64() {
                        Ok(price)
                    } else {
                        Err("Failed to extract sp500 price from response".into())
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
            //create file            
            Ok(price) => {

                //let mut file = std::fs::File::create("sp500.txt").unwrap();
                let sp500 = Ethereum { price };
                let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("sp500.txt")
                .unwrap();

            file.write_all(sp500.price.to_string().as_bytes()).unwrap();
            }
            Err(e) => eprintln!("Failed to fetch price: {}", e),
        }
    }
}


fn main() {
    //create a vector containing instances of the Bitcoin, Ethereum and SP500 structs
    let pricing: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin { price: 0.0 }),
        Box::new(Ethereum { price: 0.0 }),
        Box::new(SP500 { price: 0.0 }),
    ];

    // Periodically fetch and save prices
    loop {
        for p in &pricing {
            p.save_to_file();
        }
        std::thread::sleep(Duration::from_secs(10));
    }
}