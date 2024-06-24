use std::error::Error;
use srp::client::{SrpClient, SrpClientVerifier};
use srp::groups::G_3072;
use sha2::Sha512;
use rand::rngs::OsRng;
use log::{info, debug, error};

use crate::hap::discovery::HapAccessory;
use crate::hap::tlv8::{Tlv8Writer, Tlv8Reader, TlvType};

#[derive(Debug)]
pub enum PairingMethod {
    PairSetup = 0x00,
    PairVerify = 0x01,
    AddPairing = 0x03,
    RemovePairing = 0x04,
    ListPairings = 0x05,
}

#[derive(Debug)]
pub enum PairingState {
    M1 = 1,
    M2 = 2,
    M3 = 3,
    M4 = 4,
    M5 = 5,
    M6 = 6,
}

pub struct PairSetup<'a> {
    controller_id: &'a str,
    state: PairingState,
    http_client: reqwest::Client,
    srp_client: SrpClient<'a, Sha512>,
}

impl<'a> PairSetup<'a> {
    pub fn new() -> Self {
        info!("Initializing PairSetup");
        PairSetup {
            controller_id: "Domus",
            state: PairingState::M1,
            http_client: reqwest::ClientBuilder::new().build().unwrap(),
            srp_client: SrpClient::<'a, Sha512>::new(&G_3072),
        }
    }

    pub async fn pair(&self, accessory: &HapAccessory, setup_code: &str) -> Result<(), Box<dyn Error>> {
        info!("Starting pairing process with accessory: {:?}", accessory);
        let url = format!("http://{}:{}/pair-setup", accessory.address, accessory.port);
        
        // M1: Send pair setup request
        info!("Sending M1: Pair Setup Request");
        let m1_response = self.send_m1(&url).await?;
        debug!("Received M2 response: {:?}", m1_response);
        self.handle_m2(m1_response)?;

        // M3: Send SRP verify request
        info!("Sending M3: SRP Verify Request");
        let m3_response = self.send_m3(&url, setup_code).await?;
        debug!("Received M4 response: {:?}", m3_response);
        self.handle_m4(m3_response)?;

        // M5: Send exchange request
        info!("Sending M5: Exchange Request");
        let m5_response = self.send_m5(&url).await?;
        debug!("Received M6 response: {:?}", m5_response);
        self.handle_m6(m5_response)?;

        info!("Pairing process completed successfully");
        Ok(())
    }

    async fn send_m1(&self, url: &str) -> Result<Vec<(TlvType, Vec<u8>)>, Box<dyn Error>> {
        let mut payload = Tlv8Writer::new();
        payload.add(TlvType::Method, &[PairingMethod::PairSetup as u8]);
        payload.add(TlvType::State, &[PairingState::M1 as u8]);

        debug!("M1 payload: {:?}", payload);
        let response = self.http_client.post(url).body(payload.to_vec()).send().await?;
        let response_bytes = response.bytes().await?;
        let reader = Tlv8Reader::new(&response_bytes);
        Ok(reader.read()?)
    }

    fn handle_m2(&self, response: Vec<(TlvType, Vec<u8>)>) -> Result<(), Box<dyn Error>> {
        debug!("Handling M2 response");
        // Handle M2 logic
        Ok(())
    }

    async fn send_m3(&self, url: &str, setup_code: &str) -> Result<Vec<(TlvType, Vec<u8>)>, Box<dyn Error>> {
        debug!("Preparing M3 request with setup code: {}", setup_code);
        // Implement M3 sending logic, using setup_code
        todo!()
    }

    fn handle_m4(&self, response: Vec<(TlvType, Vec<u8>)>) -> Result<(), Box<dyn Error>> {
        debug!("Handling M4 response");
        // Handle M4 logic
        Ok(())
    }

    async fn send_m5(&self, url: &str) -> Result<Vec<(TlvType, Vec<u8>)>, Box<dyn Error>> {
        debug!("Preparing M5 request");
        // Implement M5 sending logic
        todo!()
    }

    fn handle_m6(&self, response: Vec<(TlvType, Vec<u8>)>) -> Result<(), Box<dyn Error>> {
        // Handle M6 logic
        Ok(())
    }
}
