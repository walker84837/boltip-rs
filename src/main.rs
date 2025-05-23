use crate::output::OutputJson;
use clap::Parser;
use log::{error, info, warn};
use serde::Deserialize;
use std::net::Ipv4Addr;
use thiserror::Error;

mod output;

#[derive(Error, Debug)]
enum RequestError {
    #[error("Network request failed: {0}")]
    UreqError(#[from] ureq::Error),
    #[error("Invalid response body: {0}")]
    ResponseBodyError(#[from] std::io::Error),
    #[error("Invalid JSON format: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("Special-use IP address")]
    SpecialUse,
    #[error("Invalid IP format")]
    InvalidIpFormat,
    #[error("No information found for IP")]
    NoIpInformation,
}

#[derive(Parser)]
struct Cli {
    /// Get more info on the provided IP address
    #[clap(short, long)]
    verbose: bool,

    /// Enable logging
    #[clap(short, long)]
    logging: bool,

    /// Get a random IP and show how the program works
    #[clap(short, long)]
    random: bool,

    /// This is the IP address to look up. The default is the host IP
    #[clap(short = 'i', long = "addr")]
    ip_address: Option<String>,

    /// Show the IP information in JSON format
    json: bool,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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

fn is_special_use(ip_parts: &[u8]) -> bool {
    match ip_parts {
        [0, _, _, _] => true,         // 0.0.0.0/8
        [10, _, _, _] => true,        // 10.0.0.0/8
        [14, _, _, _] => true,        // 14.0.0.0/8
        [24, _, _, _] => true,        // 24.0.0.0/8
        [39, _, _, _] => true,        // 39.0.0.0/8
        [127, _, _, _] => true,       // 127.0.0.0/8
        [128, _, _, _] => true,       // 128.0.0.0/16
        [169, 254, _, _] => true,     // 169.254.0.0/16
        [172, 16..=31, _, _] => true, // 172.16.0.0/12
        [191, 255, _, _] => true,     // 191.255.0.0/16
        [192, 0, 0, _] => true,       // 192.0.0.0/24
        [192, 0, 2, _] => true,       // 192.0.2.0/24
        [192, 88, 99, _] => true,     // 192.88.99.0/24
        [192, 168, _, _] => true,     // 192.168.0.0/16
        [198, 18, _, _] => true,      // 198.18.0.0/15
        [223, 255, 255, _] => true,   // 223.255.255.0/24
        [224..=239, _, _, _] => true, // 224.0.0.0/4 (multicast)
        [240..=255, _, _, _] => true, // 240.0.0.0/4 (reserved)
        _ => false,
    }
}

fn generate_random_ip() -> String {
    loop {
        let ip_parts: Vec<u8> = (0..4).map(|_| fastrand::u8(1..255)).collect();

        if !is_special_use(&ip_parts) {
            return format!(
                "{}.{}.{}.{}",
                ip_parts[0], ip_parts[1], ip_parts[2], ip_parts[3]
            );
        }
    }
}

fn parse_ip(ip: &str) -> Result<Vec<u8>, RequestError> {
    ip.parse()
        .map_err(|_| RequestError::InvalidIpFormat)
        .map(|ip: Ipv4Addr| ip.octets().to_vec())
}

fn get_public_ip() -> Result<String, RequestError> {
    ureq::get("https://4.ident.me")
        .call()
        .map_err(|e| RequestError::UreqError(e))?
        .into_string()
        .map_err(|e| RequestError::ResponseBodyError(e))
}

fn get_ip_info(ip: &str) -> Result<IpInfo, RequestError> {
    info!("Fetching info for IP: {ip}");
    let url = format!("https://api.iplocation.net/?ip={ip}");

    let response = ureq::get(&url).call().map_err(|e| {
        error!("IP info request failed: {e}");
        e
    })?;

    let body = response
        .into_string()
        .map_err(|e| RequestError::ResponseBodyError(e))?;

    let ip_info: IpInfo = serde_json::from_str(&body).map_err(|e| RequestError::InvalidJson(e))?;

    if ip_info.response_code != "200" {
        warn!("API returned non-success status");
        Err(RequestError::NoIpInformation)
    } else {
        Ok(ip_info)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    if args.logging {
        simple_logger::init_with_env().unwrap();
    }

    let ip_to_lookup = if args.random {
        generate_random_ip()
    } else {
        args.ip_address.unwrap_or_else(|| {
            get_public_ip().unwrap_or_else(|e| {
                warn!("Using fallback IP due to error: {e}");
                "0.0.0.0".to_string()
            })
        })
    };

    if !args.random {
        parse_ip(&ip_to_lookup)
            .and_then(|parts| {
                if is_special_use(&parts) {
                    Err(RequestError::SpecialUse)
                } else {
                    Ok(())
                }
            })
            .map_err(|e| {
                error!("IP validation failed: {e}");
                e
            })?;
    }

    let ip_info = get_ip_info(&ip_to_lookup).map_err(|e| {
        error!("IP lookup failed: {e}");
        e
    })?;

    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&OutputJson::from(ip_info))?
        );
    } else if args.verbose {
        println!("IP Details:\n{:#?}", ip_info);
    } else {
        println!(
            "IP: {}\nCountry: {}\nISP: {}",
            ip_info.ip, ip_info.country_name, ip_info.isp
        );
    }

    Ok(())
}
