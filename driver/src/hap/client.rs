use std::error::Error;
use crate::hap::discovery::HapAccessory;
use crate::hap::pairing::PairSetup;

pub struct HapClient {
}

impl HapClient {
    pub fn new() -> Self {
        HapClient {
        }
    }

    pub async fn pair(&mut self, accessory: &HapAccessory, setup_code: &str) -> Result<(), Box<dyn Error>> {
        println!("Initiating pairing with accessory: {:?}", accessory);

        let pair_setup = PairSetup::new();

        let result = pair_setup.pair(accessory, setup_code).await?;



        println!("Pairing completed successfully");
        Ok(())
    }
}

// You might want to create a custom error type later
#[derive(Debug)]
pub enum HapError {
    PairingFailed,
    // Add more error types as needed
}

impl std::fmt::Display for HapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HapError::PairingFailed => write!(f, "Failed to pair with the accessory"),
        }
    }
}

impl Error for HapError {}
