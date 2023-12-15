#![allow(dead_code)]

use anyhow::{Error, Result};
use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// Get additional information about the IP address
    #[structopt(short, long)]
    verbose: bool,

    /// Generate a random IP
    #[structopt(short, long)]
    random: bool,

    /// IP address to look up. Defualt: lookup the host's IP address
    #[structopt(short = "i", long = "address")]
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
            return format!("{}.{}.{}.{}", ip_parts[0], ip_parts[1], ip_parts[2], ip_parts[3]);
        }
    }
}

fn response_to_string(resp: Result<ureq::Response, ureq::Error>, fallback: &str) -> Result<String> {
    let body: String = match &resp {
        Ok(_) => resp?
            .into_string()
            .unwrap_or_else(|_| fallback.to_string()),
        Err(error) => {
            eprintln!("Error: {}", error);
            eprintln!("Bad HTTP request: {}", resp?.status());
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
    let url = format!("https://api.iplocation.net/?ip={}", ip);
    let response = ureq::get(&url).call();
    let body = response_to_string(response, "{}")?;
    let ip_info: IpInfo = serde_json::from_str(&body)?;
    Ok(ip_info)
}

const FALLBACK_IP: &str = "0.0.0.0";

fn main() -> Result<()> {
    let args = Cli::from_args();

    let mut ip_to_lookup = match args.ip_address {
        Some(ip) => ip,
        None => get_public_ip()?,
    };

    if ip_to_lookup == FALLBACK_IP {
        return Err(Error::msg("The HTTP response may be incorrect, or a special-use IP address may have been provided."));
    }

    ip_to_lookup = match args.random {
        true => generate_random_ip(),
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
