use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use anyhow::{Context, Result};
use indicatif::ProgressBar;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    api_key: String,
    default_format: String,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    headers: Headers,
    origin: String,
}

#[derive(Deserialize, Debug)]
struct Headers {
    #[serde(rename = "authorization")]
    auth_header : Option<String>,
}

#[derive(Parser, Debug)]
#[command(name = "client-tool", version = "1.0", about = "Enterprise API Tool")]

struct CLI {
    #[arg(short, long, env = "API_KEY")]
    api_key: Option<String>,

    

    


#[arg(short,long,default_value = "config.json", value_parser = clap::value_parser!(PathBuf))]
config: PathBuf,

#[command(subcommand)]
command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Ping,
}

#[tokio::main]
async fn main() {
    let cli = CLI::parse();

    let mut final_key = cli.api_key.clone();
    if final_key.is_none() {
        println!("No API key provided via CLI or Env. Checking config file: {:?}", cli.config);

        if let Ok(mut file) =File::open(&cli.config)  {
           let mut contents = String::new();

           if file.read_to_string(&mut contents).is_ok() {
            if let Ok(parsed_config) = serde_json::from_str::<Config>(&contents) {
                final_key = Some(parsed_config.api_key);
                println!("Successfully loaded API key from config file!");
            }
           } 
        }
    }

    let api_key = match final_key {
        Some(key) => key,
        None => {
           println!("Error: Missing API Key. Provide it via --api-key, env variable API_KEY, or a config.json file.");
            return; 
        }
    };

    match &cli.command {
        Some(Commands::Ping) => {
            println!("📡 Connecting to secure remote server...");

            let pb = ProgressBar::new_spinner();
            pb.set_message("📡 Connecting to secure remote server...");
            pb.enable_steady_tick(Duration::from_millis(100));


            let client = reqwest::Client::new();

            let response_result = client
            .get("https://httpbin.org/get")
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await;

            match response_result {
                Ok(response) => {
                    pb.finish_and_clear();
                    if let Ok(api_data) = response.json::<ApiResponse>().await {
                        println!("✅ Connection Successful!");
                        println!("🌍 Server verified your IP origin as: {}", api_data.origin);
                        if let Some(auth) = api_data.headers.auth_header {
                           println!("🔑 Server securely received token: {}", auth); 
                        }
                    } else {
                      println!("❌ Server responded, but the data format was unexpected.");  
                    }
                }
                Err(_) => {
                    pb.finish_and_clear();
                    println!("❌ Network Error: Could not reach the server.");
                }
            }
        }
        None => {
           println!("Initialization successful! Pass the 'ping' subcommand to test the network layer.");
            println!("Example: cargo run -- ping"); 
        }
    }
    
}