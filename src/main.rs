use reqwest;
use serde::Deserialize;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long, help = "Get additional information about the IP address")]
    verbose: bool,

    #[structopt(short, long, help = "IP address to look up")]
    ip_address: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();

    let ip_to_lookup = match args.ip_address {
        Some(ip) => ip,
        None => get_public_ip().await?,
    };

    let ip_info = get_ip_info(&ip_to_lookup).await?;

    if args.verbose {
        println!("IP Address: {}", ip_to_lookup);
        println!("IP Information:\n{:#?}", ip_info);
    } else {
        println!("IP Address: {}", ip_to_lookup);
        println!("Country: {}", ip_info.country_name);
        println!("ISP: {}", ip_info.isp);
    }

    Ok(())
}

async fn get_public_ip() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://ident.me").await?;
    let body = response.text().await?;
    Ok(body)
}

async fn get_ip_info(ip: &str) -> Result<IpInfo, Box<dyn Error>> {
    let url = format!("https://api.iplocation.net/?ip={}", ip);
    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    let ip_info: IpInfo = serde_json::from_str(&body)?;
    Ok(ip_info)
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct IpInfo {
    ip: String,
    ip_number: String,
    ip_version: i32,
    country_name: String,
    country_code2: String,
    isp: String,
    response_code: String,
    response_message: String,
}
