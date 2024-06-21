use std::{net::IpAddr, time::Duration};
use hal::{Device, DiscoveryInfo};
use mdns::{Record, RecordKind};
use futures_util::{pin_mut, stream::StreamExt};
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

impl DiscoveryInfo for AqaraFP2Discovery {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_id(&self) -> &str {
        &self.id
    }
    // Implement the required methods here
}

pub struct AqaraFP2Driver {}

pub struct AqaraFP2Device {}

impl AqaraFP2Driver {
    pub fn new() -> Self {
        AqaraFP2Driver {}
    }
}


impl Device for AqaraFP2Device {
    async fn init(&self) -> Result<(), String> {
        // Implement the required methods for the Device trait here
        todo!()
    }
}

const SERVICE_NAME: &'static str = "_hap._tcp.local";

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}

fn get_txt(record: &Record) -> Option<&Vec<String>> {
    match &record.kind {
        RecordKind::TXT(txt) => Some(txt),
        _ => None
    }
}

impl hal::Driver<AqaraFP2Discovery, AqaraFP2Device> for AqaraFP2Driver {
    async fn discover(&self) -> Vec<AqaraFP2Discovery> {
        let stream_result = mdns::discover::all(SERVICE_NAME, Duration::from_secs(5));
        let mut discoveries = Vec::new();
        println!("EF");
        match stream_result {
            Ok(stream) => {
                let stream = stream.listen();
                println!("QA");
                pin_mut!(stream);
                while let Some(Ok(response)) = stream.next().await {
                    println!("YERRP");
                    let txt = response.records().filter_map(self::get_txt).next();
                    if let Some(txt) = txt {
                        if txt.iter().any(|t| t == "md=PS-S02D") {
                            println!("Found FP2");
                        }
                    } 
                    let addr = response.records().filter_map(self::to_ip_addr).next();

                    if let Some(addr) = addr {
                        if let Some(txt) = txt {
                            println!("found cast device at {}, txt: {:?}", addr, txt);
                            discoveries.push(AqaraFP2Discovery { 
                                name: addr.to_string() ,
                                ip: addr,
                                id: "abc".to_string()
                            });
                        }
                    } else {
                        println!("cast device does not advertise address");
                    }
                }
            }
            Err(_) => {
                // Handle the error appropriately
                println!("Failed to start mDNS discovery");
            }
        }

        discoveries
    }

    async fn create(_info: AqaraFP2Discovery) -> AqaraFP2Device {
        todo!()
    }
}
