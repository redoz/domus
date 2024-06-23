use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::error::Error;
use std::time::Duration;
use std::collections::HashMap;
use std::io;
use enumflags2::{bitflags, BitFlags};
use std::net::IpAddr;

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

    pub fn start_discovery(&self, timeout: Duration) -> Result<Vec<HapAccessory>, Box<dyn Error>> {
        let receiver = self.mdns.browse(HAP_SERVICE_TYPE)?;
        let mut accessories = HashMap::new();

        let start_time = std::time::Instant::now();
        while start_time.elapsed() < timeout {
            if let Ok(event) = receiver.recv_timeout(Duration::from_millis(100)) {
                if let ServiceEvent::ServiceResolved(info) = event {
                    println!("Found device: {:?}", info);
                    match HapAccessory::from_service_info(&info) {
                        Ok(accessory) => {
                            accessories.insert(accessory.id.clone(), accessory);
                        },
                        Err(error) => {
                            println!("Failed to parse HapAccessory: {:?}. Error: {:?}", info, error);
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
#[repr(u32)]
pub enum FeatureFlag {
    HapPairing = 0x01,
    HomeKitSecureVideo = 0x02,
    HomeKitAudio = 0x04,
    HomeKitBridge = 0x08,
}

#[bitflags]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
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

#[derive(Debug, Clone)]
pub struct HapAccessory {
    pub address: IpAddr,
    pub port: u16,
    pub id: String,
    pub model: String,
    pub configuration_number: u32,
    pub current_state_number: u32,
    pub feature_flags: BitFlags<FeatureFlag>,
    pub pairing_feature_flags: BitFlags<PairingFeatureFlag>,
    pub status_flags: BitFlags<StatusFlag>,
    pub setup_hash: Option<String>,
}

impl HapAccessory {
    fn from_service_info(info: &mdns_sd::ServiceInfo) -> Result<Self, Box<dyn Error>> {
        Ok(HapAccessory {
            address: info.get_addresses().iter().next().ok_or("No IP address found")?.clone(),
            port: info.get_port(),
            id: info.get_property_val_str("id").ok_or("Missing id")?.to_string(),
            model: info.get_property_val_str("md").ok_or("Missing model")?.to_string(),
            configuration_number: info.get_property_val_str("c#").ok_or("Missing c#")?.parse()?,
            current_state_number: info.get_property_val_str("s#").ok_or("Missing s#")?.parse()?,
            feature_flags: parse_bitflags(info.get_property_val_str("ff"))?,
            pairing_feature_flags: parse_bitflags(info.get_property_val_str("pf"))?,
            status_flags: parse_bitflags(info.get_property_val_str("sf"))?,
            setup_hash: info.get_property_val_str("sh").map(|s| s.to_string()),
        })
    }

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
        let accessories = discovery.start_discovery(Duration::from_secs(5)).unwrap();
        
        for accessory in accessories {
            println!("Found accessory: {:?}", accessory);
            println!("Is paired: {}", accessory.is_paired());
            println!("Is configured for WiFi: {}", accessory.is_configured_for_wifi());
            println!("Has problem detected: {}", accessory.has_problem_detected());
            println!("Supports HAP Pair Setup: {}", accessory.feature_flags.contains(FeatureFlag::HapPairing));
            println!("Supports Apple Authentication Coprocessor: {}", 
                     accessory.pairing_feature_flags.contains(PairingFeatureFlag::AppleAuthenticationCoprocessor));
        }
    }
}
