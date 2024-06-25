use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::error::Error;
use std::time::Duration;
use std::collections::HashMap;
use std::io;
use enumflags2::{bitflags, BitFlags};
use std::net::IpAddr;
use std::convert::TryFrom;

const HAP_SERVICE_TYPE: &str = "_hap._tcp.local.";

fn parse_bitflags<T: enumflags2::BitFlag>(txt_value: Option<&str>) -> Result<BitFlags<T>, Box<dyn Error>>
where
    T: enumflags2::BitFlag,
    T::Numeric: TryFrom<u32>,
    <T::Numeric as TryFrom<u32>>::Error: Error + 'static,
{
    let Some(value) = txt_value else {
        return Ok(BitFlags::empty());
    };

    let bits = u32::from_str_radix(value, 16)?;

    let numeric = T::Numeric::try_from(bits)?;
    
    let flags = BitFlags::from_bits(numeric);

    flags.map_err(|_| Box::new(io::Error::new(io::ErrorKind::InvalidData, "Failed to parse bit flags")) as Box<dyn Error>)
}

pub struct HapDiscovery {
    mdns: ServiceDaemon,
}

impl HapDiscovery {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mdns = ServiceDaemon::new()?;
        Ok(HapDiscovery { mdns })
    }

    pub fn start_discovery(&self, ipv4_only: bool, timeout: Duration) -> Result<Vec<HapAccessory>, Box<dyn Error>> {

        let receiver = self.mdns.browse(HAP_SERVICE_TYPE)?;
        let mut accessories = HashMap::new();

        let start_time = std::time::Instant::now();
        while start_time.elapsed() < timeout {
            if let Ok(event) = receiver.recv_timeout(Duration::from_millis(500)) {
                if let ServiceEvent::ServiceResolved(info) = event {
                    log::info!("Found device: {:#?}", info);
                    if ipv4_only && info.get_addresses_v4().len() == 0 {
                        continue;
                    }
                    match HapAccessory::try_from(&info) {
                        Ok(accessory) => {
                            log::debug!("Found accessory with ID: {}", accessory.id);
                            accessories.insert(accessory.id.clone(), accessory);
                        },
                        Err(error) => {
                            log::debug!("Failed to parse HapAccessory: {:?}. Error: {:?}", info, error);
                        }
                    }
                }
            }
        }

        Ok(accessories.into_values().collect())
    }
}


#[bitflags]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum PairingFeatureFlag {
    AppleAuthenticationCoprocessor = 0x01,
    SoftwareAuthentication = 0x02,
}

#[bitflags]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum StatusFlag {
    NotPaired = 0x01,
    NotConfiguredForWiFi = 0x02,
    ProblemDetected = 0x04,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AccessoryCategory {
    Other = 1,
    Bridges = 2,
    Fans = 3,
    GarageDoorOpeners = 4,
    Lighting = 5,
    Locks = 6,
    Outlets = 7,
    Switches = 8,
    Thermostats = 9,
    Sensors = 10,
    SecuritySystems = 11,
    Doors = 12,
    Windows = 13,
    WindowCoverings = 14,
    ProgrammableSwitches = 15,
    Reserved16 = 16,
    IPCameras = 17,
    VideoDoorBells = 18,
    AirPurifiers = 19,
    Heaters = 20,
    AirConditioners = 21,
    Humidifiers = 22,
    Dehumidifiers = 23,
    Reserved24 = 24,
    Reserved25 = 25,
    Reserved26 = 26,
    Reserved27 = 27,
    Sprinklers = 28,
    Faucets = 29,
    ShowerSystems = 30,
    Reserved31 = 31,
    Remotes = 32,
}

impl TryFrom<u8> for AccessoryCategory {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1..=32 => Ok(unsafe { std::mem::transmute(value) }),
            _ => Err("Invalid AccessoryCategory value"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HapAccessory {
    pub address: IpAddr,
    pub port: u16,
    pub id: String,
    pub model: String,
    pub configuration_number: u32,
    pub current_state_number: u32,
    pub pairing_feature_flags: BitFlags<PairingFeatureFlag>,
    pub status_flags: BitFlags<StatusFlag>,
    pub setup_hash: Option<String>,
    pub category: AccessoryCategory,
    pub protocol_version: String,
}


impl TryFrom<&mdns_sd::ServiceInfo> for HapAccessory {
    type Error = Box<dyn Error>;

    fn try_from(info: &mdns_sd::ServiceInfo) -> Result<Self, Self::Error> {
        Ok(HapAccessory {
            address: info.get_addresses().iter()
                .find(|addr| addr.is_ipv4())
                .or_else(|| info.get_addresses().iter().find(|addr| addr.is_ipv6()))
                .ok_or("No IP address found")?
                .clone(),
            port: info.get_port(),
            id: info.get_property_val_str("id").ok_or("Missing id")?.to_string(),
            model: info.get_property_val_str("md").ok_or("Missing model")?.to_string(),
            configuration_number: info.get_property_val_str("c#").ok_or("Missing c#")?.parse()?,
            current_state_number: info.get_property_val_str("s#").ok_or("Missing s#")?.parse()?,
            pairing_feature_flags: parse_bitflags(info.get_property_val_str("pf"))?,
            status_flags: parse_bitflags(info.get_property_val_str("sf"))?,
            setup_hash: info.get_property_val_str("sh").map(|s| s.to_string()),
            category: AccessoryCategory::try_from(
                info.get_property_val_str("ci")
                    .ok_or("Missing ci")?
                    .parse::<u8>()?
            )?,
            protocol_version: info.get_property_val_str("pv")
                .ok_or("Missing pv")?
                .to_string(),
        })
    }
}

impl HapAccessory {
    pub fn is_paired(&self) -> bool {
        !self.status_flags.contains(StatusFlag::NotPaired)
    }

    pub fn is_configured_for_wifi(&self) -> bool {
        !self.status_flags.contains(StatusFlag::NotConfiguredForWiFi)
    }

    pub fn has_problem_detected(&self) -> bool {
        self.status_flags.contains(StatusFlag::ProblemDetected)
    }

    // Add more methods as needed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hap_discovery() {
        let discovery = HapDiscovery::new().unwrap();
        let accessories = discovery.start_discovery(true,Duration::from_secs(5)).unwrap();
        
        for accessory in accessories {
            println!("Found accessory: {:?}", accessory);
            println!("Is paired: {}", accessory.is_paired());
            println!("Is configured for WiFi: {}", accessory.is_configured_for_wifi());
            println!("Has problem detected: {}", accessory.has_problem_detected());
            println!("Supports Apple Authentication Coprocessor: {}", 
                     accessory.pairing_feature_flags.contains(PairingFeatureFlag::AppleAuthenticationCoprocessor));
        }
    }
}

