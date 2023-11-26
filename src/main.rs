use serde::Deserialize;
use structopt::StructOpt;
use ureq::Error as request_error;
use std::error::Error as std_error;

#[derive(StructOpt)]
struct Cli {
    /// Get additional information about the IP address
    #[structopt(short, long)]
    verbose: bool,

    /// Generate a random IP
    #[structopt(short, long)]
    random: Option<bool>,

    /// IP address to look up. If not provided, it will lookup the host's IP address
    #[structopt(short = "i", long = "address")]
    ip_address: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IpInfo {
    // ip: String,
    // ip_number: String,
    // ip_version: i32,
    country_name: String,
    // country_code2: String,
    isp: String,
    // response_code: String,
    // response_message: String,
}

fn generate_random_ip() -> String {
    loop {
        let ip_parts: Vec<u8> = (0..4).map(|_| fastrand::u8(1..255)).collect();

        let is_special_use = match ip_parts.as_slice() {
            [0, _, _, _] => true, // 0.0.0.0/8
            [10, _, _, _] => true, // 10.0.0.0/8
            [14, _, _, _] => true, // 14.0.0.0/8
            [24, _, _, _] => true, // 24.0.0.0/8
            [39, _, _, _] => true, // 39.0.0.0/8
            [127, _, _, _] => true, // 127.0.0.0/8
            [128, _, _, _] => true, // 128.0.0.0/16
            [169, 254, _, _] => true, // 169.254.0.0/16
            [172, x, _, _] if x >= &16 && x <= &31 => true, // 172.16.0.0/12
            [191, 255, _, _] => true, // 191.255.0.0/16
            [192, 0, 0, _] => true, // 192.0.0.0/24
            [192, 0, 2, _] => true, // 192.0.2.0/24
            [192, 88, 99, _] => true, // 192.88.99.0/24
            [192, 168, _, _] => true, // 192.168.0.0/16
            [198, 18, _, _] => true, // 198.18.0.0/15
            [223, 255, 255, _] => true, // 223.255.255.0/24
            [224..=239, _, _, _] => true, // 224.0.0.0/4 (multicast)
            [240..=255, _, _, _] => true, // 240.0.0.0/4 (reserved)
            _ => false,
        };

        if !is_special_use {
            return format!("{}.{}.{}.{}", ip_parts[0], ip_parts[1], ip_parts[2], ip_parts[3]);
        }
    }
}

fn is_okay(resp: &Result<ureq::Response, request_error>) -> bool {
    match resp {
        Ok(_) => true,
        Err(_) => false
    }
}

fn response_to_string(resp: Result<ureq::Response, request_error>, fallback: &str) -> String {
    let body: String = match is_okay(&resp) {
        true => resp.unwrap().into_string().unwrap_or_else(|_| fallback.to_string()),
        false => {
            eprintln!("Bad HTTP request: {}", resp.unwrap().status());
            fallback.to_string()
        }
    };
    body
}

fn get_public_ip() -> String {
    let response = ureq::get("https://ident.me").call();
    response_to_string(response, "0.0.0.0")
}

fn get_ip_info(ip: &str) -> Result<IpInfo, Box<dyn std_error>> {
    let url = format!("https://api.iplocation.net/?ip={}", ip);
    let response = ureq::get(&url).call();
    let body = response_to_string(response, "{}");
    let ip_info: IpInfo = serde_json::from_str(&body)?;
    Ok(ip_info)
}

fn main() -> Result<(), Box<dyn std_error>> {
    let args = Cli::from_args();

    let mut ip_to_lookup = match args.ip_address {
        Some(ip) => ip,
        None => get_public_ip(),
    };

    if ip_to_lookup == "0.0.0.0" {
        return Err("Either a local IP address may have been provided or the program has sent a faulty HTTP response.".into());
    }

    ip_to_lookup = match args.random {
        Some(true) => generate_random_ip(),
        Some(false) => ip_to_lookup,
        None => ip_to_lookup
    };

    let ip_info = get_ip_info(&ip_to_lookup)?;

    if args.verbose {
        println!("IP Address: {}", ip_to_lookup);
        println!("IP Information:");
        println!("{:#?}", ip_info);
    } else {
        println!("IP Address: {}", ip_to_lookup);
        println!("Country: {}", ip_info.country_name);
        println!("ISP: {}", ip_info.isp);
    }

    Ok(())
}
