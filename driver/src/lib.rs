pub mod aqara_fp2;

pub use aqara_fp2::{AqaraFP2Discovery, AqaraFP2, AqaraFP2Driver};

use core::{Device, LifeCycle};


#[derive(Debug)]
pub struct DummyDevice {
    pub device_type: &'static str,
    pub name: &'static str,
}

impl Device for DummyDevice {}

impl LifeCycle for DummyDevice {
    async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Initializing {} device: {}", self.device_type, self.name);
        Ok(())
    }

    async fn dispose(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Disposing {} device: {}", self.device_type, self.name);
        Ok(())
    }
}
