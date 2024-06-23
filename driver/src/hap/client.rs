use std::error::Error;
use crate::hap::discovery::HapAccessory;
use crate::hap::pairing::PairingSession;

pub struct HapClient {
    pairing_session: Option<PairingSession>,
}

impl HapClient {
    pub fn new() -> Self {
        HapClient {
            pairing_session: None,
        }
    }

    pub fn pair(&mut self, accessory: &HapAccessory, setup_code: &str) -> Result<(), Box<dyn Error>> {
        println!("Initiating pairing with accessory: {:?}", accessory);

        self.pairing_session = Some(PairingSession::new(accessory.clone(), setup_code));
        let pairing_session = self.pairing_session.as_mut().unwrap();

        // Start the pairing process
        let start_request = pairing_session.start_pairing()?;
        // TODO: Send the start_request and handle the response

        // Implement the rest of the pairing process using pairing_session methods

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
