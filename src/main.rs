#![allow(dead_code)]

use anyhow::{bail, Result};
use clap::Parser;
use log::{error, info, warn};
use serde::Deserialize;

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
}

#[derive(Deserialize, Debug)]
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

fn generate_random_ip() -> String {
    loop {
        let ip_parts: Vec<u8> = (0..4).map(|_| fastrand::u8(1..255)).collect();

        let is_special_use = match ip_parts.as_slice() {
            [0, _, _, _] => true,                               // 0.0.0.0/8
            [10, _, _, _] => true,                              // 10.0.0.0/8
            [14, _, _, _] => true,                              // 14.0.0.0/8
            [24, _, _, _] => true,                              // 24.0.0.0/8
            [39, _, _, _] => true,                              // 39.0.0.0/8
            [127, _, _, _] => true,                             // 127.0.0.0/8
            [128, _, _, _] => true,                             // 128.0.0.0/16
            [169, 254, _, _] => true,                           // 169.254.0.0/16
            [172, x, _, _] if (&16..=&31).contains(&x) => true, // 172.16.0.0/12
            [191, 255, _, _] => true,                           // 191.255.0.0/16
            [192, 0, 0, _] => true,                             // 192.0.0.0/24
            [192, 0, 2, _] => true,                             // 192.0.2.0/24
            [192, 88, 99, _] => true,                           // 192.88.99.0/24
            [192, 168, _, _] => true,                           // 192.168.0.0/16
            [198, 18, _, _] => true,                            // 198.18.0.0/15
            [223, 255, 255, _] => true,                         // 223.255.255.0/24
            [224..=239, _, _, _] => true,                       // 224.0.0.0/4 (multicast)
            [240..=255, _, _, _] => true,                       // 240.0.0.0/4 (reserved)
            _ => false,
        };

        if !is_special_use {
            return format!(
                "{}.{}.{}.{}",
                ip_parts[0], ip_parts[1], ip_parts[2], ip_parts[3]
            );
        }
    }
}

fn response_to_string(resp: Result<ureq::Response, ureq::Error>, fallback: &str) -> Result<String> {
    let body: String = match &resp {
        Ok(_) => resp?.into_string().unwrap_or_else(|_| fallback.to_string()),
        Err(error) => {
            error!("There was an error with processing the request: {}", error);
            error!("Response status: {}", resp?.status());
            warn!("Falling back to: `{}`. Errors may follow this.", fallback);
            fallback.to_string()
        }
    };
    Ok(body)
}

fn get_public_ip() -> Result<String> {
    let response = ureq::get("https://ident.me").call();
    let resp = response_to_string(response, FALLBACK_IP)?;
    Ok(resp)
}

fn get_ip_info(ip: &str) -> Result<IpInfo> {
    info!("Getting information about IP '{}'.", ip);
    let url = format!("https://api.iplocation.net/?ip={}", ip);
    let response = ureq::get(&url).call();
    let body = response_to_string(response, "{}")?;
    let ip_info: IpInfo = serde_json::from_str(&body)?;
    Ok(ip_info)
}

const FALLBACK_IP: &str = "0.0.0.0";

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.logging {
        simple_logger::init_with_env().unwrap();
    }

    let mut ip_to_lookup = match args.ip_address {
        Some(ip) => ip,
        None => {
            info!("Getting IP address of host machine.");
            get_public_ip()?
        }
    };

    if ip_to_lookup == FALLBACK_IP {
        bail!("The response is wrong or the IP address is special-use.");
    }

    ip_to_lookup = match args.random {
        true => {
            info!("Generating a random IP address.");
            generate_random_ip()
        }
        false => ip_to_lookup,
    };

    let ip_info = get_ip_info(&ip_to_lookup)?;

    if args.verbose {
        println!("IP Information:");
        println!("{:#?}", ip_info);
    } else {
        println!("IP Address: {}", ip_to_lookup);
        println!("Country: {}", ip_info.country_name);
        println!("ISP: {}", ip_info.isp);
    }

    Ok(())
}
