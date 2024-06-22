use std::{net::IpAddr, time::Duration};
use hal::DeviceDiscovery;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use core::LifeCycle;
use core::Device;
use hal::DiscoveryInfo;

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

pub struct AqaraFP2Discovery {
    id: String,
    name: String,
    ip: IpAddr,
}



pub struct AqaraFP2Driver {}

#[derive(Debug)]
pub struct AqaraFP2 {
    pub name: &'static str
}

impl AqaraFP2Driver {
    pub fn new() -> Self {
        AqaraFP2Driver {}
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

const SERVICE_NAME: &'static str = "_hap._tcp.local.";

impl DeviceDiscovery for AqaraFP2Driver {
    async fn discover() -> Vec<DiscoveryInfo> {
        let mdns = ServiceDaemon::new().expect("Failed to create daemon");
        let receiver = mdns.browse(SERVICE_NAME).expect("Failed to browse");
        let mut discoveries = Vec::new();

        while let Ok(event) = receiver.recv_timeout(Duration::from_secs(5)) {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    println!("YERRP");
                    if let Some(txt) = info.get_property("md") {
                        if txt.val_str() == "PS-S02D" {
                            println!("Found FP2");
                            if let Some(addr) = info.get_addresses().iter().next() {
                                println!("found cast device at {}, txt: {:?}", addr, info.get_properties());
                                discoveries.push(DiscoveryInfo { 
                                    name: info.get_hostname().to_string(),
                                    definition: "".to_string()
                                    //ip: addr.to_owned(),
                                    //id: info.get_fullname().to_string(),
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

}

