use std::{net::IpAddr, time::Duration};
use hal::{Device, DeviceInfo};
use mdns::{Record, RecordKind};
use futures_util::{pin_mut, stream::StreamExt};

pub struct AqaraFP2Discovery {
    name: String,
}

impl DeviceInfo for AqaraFP2Discovery {
    fn get_name(&self) -> &str {
        &self.name
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
        let stream_result = mdns::discover::all(SERVICE_NAME, Duration::from_secs(15));
        let mut discoveries = Vec::new();

        match stream_result {
            Ok(stream) => {
                let stream = stream.listen();
                pin_mut!(stream);
                while let Some(Ok(response)) = stream.next().await {
                    let txt = response.records().filter_map(self::get_txt).next();
                    let addr = response.records().filter_map(self::to_ip_addr).next();

                    if let Some(addr) = addr {
                        if let Some(txt) = txt {
                            println!("found cast device at {}, txt: {:?}", addr, txt);
                            discoveries.push(AqaraFP2Discovery { name: addr.to_string() });
                        }
                    } else {
                        println!("cast device does not advertise address");
                    }
                    break;
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
