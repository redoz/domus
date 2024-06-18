
use hal;


struct AqaraFP2Info {
    name: String,
    address: String,
}

struct AqaraFP2Device {
    info: AqaraFP2Info,
    initialized: bool,
}

impl DeviceInfo for AqaraFP2Info {
    fn new(name: String, address: String) -> Self {
        AqaraFP2Info { name, address }
    }
}

impl Device for AqaraFP2Device {
    async fn init(&self) -> Result<(), String> {
        if self.info.model == "Aqara FP2" {
            Ok(())
        } else {
            Err("Unsupported device model".to_string())
        }
    }
}

pub(crate) struct AqaraFP2Driver;

impl AqaraFP2Driver {
    fn new() -> Self {
        AqaraFP2Driver
    }
}

impl Driver<AqaraFP2Info, AqaraFP2Device> for AqaraFP2Driver {
    async fn discover(&self) -> Vec<AqaraFP2Info> {
        vec![AqaraFP2Info::new("Aqara FP2 - Living Room", "192.168.1.10", "Aqara FP2")]
    }

    async fn create(&self, info: AqaraFP2Info) -> AqaraFP2Device {
        AqaraFP2Device {
            info,
            initialized: false,
        }
    }
}
