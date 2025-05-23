use crate::IpInfo;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct OutputJson {
    pub ip: String,
    pub ip_number: String,
    pub ip_version: i32,
    pub isp: String,
    pub location: Location,
}

#[derive(Serialize, Debug)]
pub struct Location {
    pub country_name: String,
    pub country_code: String,
}

impl From<IpInfo> for OutputJson {
    fn from(ip_info: IpInfo) -> Self {
        OutputJson {
            ip: ip_info.ip,
            ip_number: ip_info.ip_number,
            ip_version: ip_info.ip_version,
            location: Location {
                country_name: ip_info.country_name,
                country_code: ip_info.country_code2,
            },
            isp: ip_info.isp,
        }
    }
}
