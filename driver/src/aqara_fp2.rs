use core::{DiscoveryInfo, DeviceProperties, Device, Driver, LifeCycle};
use std::{net::IpAddr, time::Duration};
use mdns_sd::{ServiceDaemon, ServiceEvent};
use sha2::{digest::Digest, Sha256, Sha512};

/* 
enum Category {
	Other = 1,
	Bridge = 2,
	Fan = 3,
	GarageDoorOpener = 4,
	Lightbulb = 5,
	DoorLock = 6,
	Outlet = 7,
	Switch = 8,
	Thermostat = 9,
	Sensor = 10,
	SecuritySystem = 11,
	Door = 12,
	Window = 13,
	WindowCovering = 14,
	ProgrammableSwitch = 15,
	RangeExtender = 16,
	IpCamera = 17,
	VideoDoorbell = 18,
	AirPurifier = 19,
	AirHeater = 20,
	AirConditioner = 21,
	AirHumidifier = 22,
	AirDehumidifier = 23,
	AppleTv = 24,
	Speaker = 26,
	Airport = 27,
	Sprinkler = 28,
	Faucet = 29,
	ShowerHead = 30,
	Television = 31,
	TargetController = 32,
	WiFiRouter = 33,
	AudioReceiver = 34,
	TelevisionSetTopBox = 35,
	TelevisionStreamingStick = 36,
}
// hap related things
struct TxtRecord {
        pub name: String,
        pub device_id: MacAddress, // id
        pub configuration_number: u64, // c#
        pub state_number: u8, // s#
        pub category: Category, // ci
        pub protocol_version: String, // pv
        pub status_flag: BonjourStatusFlag, // sf
        pub feature_flag: BonjourFeatureFlag, // ff
}

format!("c#={}", self.configuration_number),
format!("ff={}", self.feature_flag as u8),
format!("id={}", self.device_id.to_string()),
format!("md={}", self.name),
format!("pv={}", self.protocol_version),
format!("s#={}", self.state_number),
format!("sf={}", self.status_flag as u8),
format!("ci={}", self.category as u8),
*/

/*
discovery() -> impl DeviceDiscovery
pair(DeviceDiscovery) -> impl DeviceProperties
attach(DeviceProperties) -> impl Device
*/

pub struct AqaraFP2Discovery {
    id: String,
    name: String,
    ip: IpAddr,
}

impl DiscoveryInfo for AqaraFP2Discovery {
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> &str {
        &self.id
    }
}


// device properties

#[derive(Debug)]
pub struct AqaraFP2 {
    pub name: &'static str,
    pub ip: &'static str
}

impl DeviceProperties for AqaraFP2 {
}

impl std::fmt::Display for AqaraFP2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"
AqaraFP2 {{ 
    name: \"{}\",
    ip: \"{}\",
}}
"#, 
self.name, 
self.ip)
    }
}


impl LifeCycle for AqaraFP2 {
    async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Implement initialization logic for AqaraFP2 device
        log::info!("Initializing AqaraFP2 device: {}", self.name);
        // Add actual initialization code here
        Ok(())
    }

    async fn dispose(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Implement disposal logic for AqaraFP2 device
        println!("Disposing AqaraFP2 device: {}", self.name);
        // Add actual disposal code here
        Ok(())
    }
}



impl Device for AqaraFP2 {

}


pub struct AqaraFP2Driver {}

impl AqaraFP2Driver {
    pub fn new() -> Self {
        AqaraFP2Driver {}
    }
}

const SERVICE_NAME: &'static str = "_hap._tcp.local.";

impl Driver<AqaraFP2Discovery, AqaraFP2, AqaraFP2> for AqaraFP2Driver {
    async fn discover(&self) -> Vec<AqaraFP2Discovery> {
        let mdns = ServiceDaemon::new().expect("Failed to create daemon");
        let receiver = mdns.browse(SERVICE_NAME).expect("Failed to browse");
        let mut discoveries = Vec::new();

        while let Ok(event) = receiver.recv_timeout(Duration::from_secs(5)) {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    if let Some(txt) = info.get_property("md") {
                        if txt.val_str() == "PS-S02D" {
                            if let Some(addr) = info.get_addresses().iter().next() {
                                log::debug!("found cast device at {}, properties: {:?}", addr, info.get_properties());
                                discoveries.push(AqaraFP2Discovery { 
                                    name: info.get_property_val_str("md").unwrap().to_string(),
                                    ip: addr.to_owned(),
                                    id: info.get_property_val_str("id").unwrap().to_string(),
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        discoveries
    }
    
    async fn pair(&self, _discovery: &AqaraFP2Discovery) -> Result<AqaraFP2, Box<dyn std::error::Error>> {
        
        let srp_client = srp::client::SrpClient::<Sha512>::new(&srp::groups::G_3072);
        
        todo!()
    }
}


