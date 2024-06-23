use std::error::Error;
use srp::client::{SrpClient, SrpClientVerifier};
use srp::groups::G_3072;
use sha2::Sha512;
use rand::rngs::OsRng;

use crate::hap::discovery::HapAccessory;
use crate::hap::tlv8::{Tlv8Writer, TlvType};

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
    state: PairingState,
    http_client: reqwest::Client,
    srp_client: SrpClient<'a, Sha512>
}

impl<'a> PairSetup<'a> {
    pub fn new() -> Self {
        PairSetup {
            state: PairingState::M1,
            http_client: reqwest::ClientBuilder::new().build().unwrap(),
            srp_client: SrpClient::<'a, Sha512>::new(&G_3072),
        }
    }

    pub fn handle_request(&mut self, request: Vec<TLV>) -> Result<Vec<TLV>, String> {
        match self.state {
            PairingState::M1 => {
                // Handle M1: Controller -> Accessory
                self.state = PairingState::M2;
                self.handle_m1(request)
            }
            PairingState::M2 => {
                // Handle M2: Accessory -> Controller
                self.state = PairingState::M3;
                self.handle_m2(request)
            }
            PairingState::M3 => {
                // Handle M3: Controller -> Accessory
                self.state = PairingState::M4;
                self.handle_m3(request)
            }
            PairingState::M4 => {
                // Handle M4: Accessory -> Controller
                self.state = PairingState::M5;
                self.handle_m4(request)
            }
            PairingState::M5 => {
                // Handle M5: Controller -> Accessory
                self.state = PairingState::M6;
                self.handle_m5(request)
            }
            PairingState::M6 => {
                // Handle M6: Accessory -> Controller
                self.handle_m6(request)
            }
        }
    }


    fn handle_m1(&self, request: Vec<TLV>) -> Result<Vec<TLV>, String> {
        //pub async fn handle_m1(&mut self, identifier: String, url: &str) -> Result<(), String> {
            // Create Pair Setup Request (M1)
            let writer = Tlv8Writer::new();
            

            const PAIR_SETUP_METHOD: [u8; 1] = [PairingMethod::PairSetup as u8];
            writer.add(TlvType::Method, &PAIR_SETUP_METHOD);
            writer.add(TlvType::Identifier, identifier.as_bytes().to_vec());

            let request_bytes = writer.to_vec();
    
            // Send the request to the accessory
            let response = self
                .client
                .post(url)
                .body(request_bytes)
                .send()
                .await
                .map_err(|e| e.to_string())?;
    
            let response_bytes = response.bytes().await.map_err(|e| e.to_string())?;
            let response_tlv = PairSetup::parse_tlv(&response_bytes);
    
            // Handle the response (M2)
            self.handle_m2(response_tlv)?;
    
            Ok(())
        //}
    }

    fn handle_m2(&self, request: Vec<TLV>) -> Result<Vec<TLV>, String> {
        // Handle M2 logic
        Ok(vec![])
    }

    fn handle_m3(&self, request: Vec<TLV>) -> Result<Vec<TLV>, String> {
        // Handle M3 logic
        Ok(vec![])
    }

    fn handle_m4(&self, request: Vec<TLV>) -> Result<Vec<TLV>, String> {
        // Handle M4 logic
        Ok(vec![])
    }

    fn handle_m5(&self, request: Vec<TLV>) -> Result<Vec<TLV>, String> {
        // Handle M5 logic
        Ok(vec![])
    }

    fn handle_m6(&self, request: Vec<TLV>) -> Result<Vec<TLV>, String> {
        // Handle M6 logic
        Ok(vec![])
    }
}