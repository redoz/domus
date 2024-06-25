use std::error::Error;
// use srp::{client::SrpClient, groups::G_3072};
use sha2::Sha512;
use rand::{rngs::OsRng, RngCore};
use log::{info, debug, error};
use reqwest::Client;

use crate::hap::discovery::HapAccessory;
use crate::hap::tlv8::{Tlv8Writer, Tlv8Reader, TlvType};

#[derive(Debug)]
pub enum PairingMethod {
    PairSetup = 0x00,
    PairSetupWithAuth = 0x01,
    PairVerify = 0x02,
    AddPairing = 0x03,
    RemovePairing = 0x04,
    ListPairings = 0x05,
}

#[derive(Debug)]
#[repr(u8)]
pub enum PairingState {
    M1 = 1,
    M2 = 2,
    M3 = 3,
    M4 = 4,
    M5 = 5,
    M6 = 6,
}

impl Into<u8> for PairingState {
    fn into(self) -> u8 {
        self as u8
    }
}

pub struct PairSetup<'a> {
    controller_id: &'a str,
    http_client: Client,
    srp_client: SrpClient<'a, Sha512>,
}



impl<'a> PairSetup<'a> {
    pub fn new() -> Self {
        info!("Initializing PairSetup");
        PairSetup {
            controller_id: "Domus",
            http_client: Client::new(),
            srp_client: SrpClient::<'a, Sha512>::new(&G_3072),
        }
    }

    pub async fn pair(&self, accessory: &HapAccessory, setup_code: &str) -> Result<(), Box<dyn Error>> {
        info!("Starting pairing process with accessory: {:#?}", accessory);
        let url = format!("http://{}:{}/pair-setup", accessory.address, accessory.port);
        debug!("Pairing URL: {}", url);
        
        // M1: Send pair setup request
        info!("Sending M1: Pair Setup Request");
        let m1_response = self.send_m1(&url).await;
        
        if let Err(ref e) = m1_response {
            debug!("Error in M1 response: {:?}", e);
            return Err("Failed to send M1 message".into());
        }
        let m1_response = m1_response?;
        debug!("Received M2 response: {:?}", m1_response);
        let (salt, public_key) = self.handle_m2(m1_response)?;


        // M3: Send SRP verify request
        info!("Sending M3: SRP Verify Request");
        let m3_response = self.send_m3(&url, setup_code, &salt, &public_key).await?;
        debug!("Received M4 response: {:?}", m3_response);
        let (server_proof, encrypted_data) = self.handle_m4(m3_response)?;

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
        payload.add(TlvType::Method, &[PairingMethod::PairSetupWithAuth as u8]);
        payload.add(TlvType::State, &[PairingState::M1.into()]);

        debug!("M1 payload: {:?}", payload);
        let response = self.http_client.post(url)
            .body(payload.to_vec())
            .send()
            .await?;

        assert_eq!(response.status(), 200, "Expected 200 OK response, got {}", response.status());

        let response_bytes = response.bytes().await?;
        let reader = Tlv8Reader::new(&response_bytes);
        Ok(reader.read()?)
    }

    fn handle_m2(&self, response: Vec<(TlvType, Vec<u8>)>) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
        debug!("Handling M2 response");

        let mut state: Option<u8> = None;
        let mut salt: Option<Vec<u8>> = None;
        let mut public_key: Option<Vec<u8>> = None;
        let mut error: Option<u8> = None;

        for (tlv_type, value) in response {
            match tlv_type {
                TlvType::State => {
                    state = value.first().copied();
                    debug!("M2 State: {:?}", state);
                },
                TlvType::Salt => {
                    salt = Some(value.clone());
                    debug!("M2 Salt length: {} bytes", value.len());
                },
                TlvType::PublicKey => {
                    public_key = Some(value.clone());
                    debug!("M2 Public Key length: {} bytes", value.len());
                },
                TlvType::Error => {
                    error = value.first().copied();
                    debug!("M2 Error: {:?}", error);
                },
                _ => debug!("Unexpected TLV type in M2 response: {:?}", tlv_type),
            }
        }

        if let Some(error) = error {
            PairSetup::handle_error(error)?;
        }

        // Verify that we received all required fields
        let state = state.ok_or("M2 response missing state")?;
        let salt = salt.ok_or("M2 response missing salt")?;
        let public_key = public_key.ok_or("M2 response missing public key")?;

        // Verify the state
        if state != PairingState::M2 as u8 {
            return Err(format!("Unexpected state in M2 response: {}", state).into());
        }

        Ok((salt, public_key))
    }

    async fn send_m3(&self, url: &str, setup_code: &str, salt: &[u8], public_key: &[u8]) -> Result<Vec<(TlvType, Vec<u8>)>, Box<dyn Error>> {
        debug!("Preparing M3 request with setup code: {}", setup_code);

        // Format the setup code
        let formatted_setup_code = format!("{}-{}-{}", 
            &setup_code[0..3], 
            &setup_code[3..5], 
            &setup_code[5..8]
        );
        debug!("Formatted setup code: {}", formatted_setup_code);

        
        let mut rng = OsRng;
        
        let mut a: [u8; 64] = [0u8; 64];
        rng.fill_bytes(&mut a);
        let a_pub = self.srp_client.compute_public_ephemeral(&a);

        // Use the formatted setup code in the SRP calculations
        let verifier = self.srp_client.process_reply_rfc5054(
            &a,
            "Pair-Setup".as_bytes(),
            formatted_setup_code.as_bytes(),
            salt,
            public_key
        );


        // Return an error if the verifier wasn't created properly
        let verifier = match verifier {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to process SRP reply: {}", e);
                return Err(format!("Failed to process SRP reply: {}", e).into());
            }
        };

        

        debug!("Successfully processed SRP reply");

        // 3. Generate the client proof
        let client_proof = verifier.proof();

        // 4. Construct the M3 TLV payload
        let mut payload = Tlv8Writer::new();
        payload.add(TlvType::State, &[PairingState::M3.into()]);
        payload.add(TlvType::PublicKey, &a_pub);
        payload.add(TlvType::Proof, client_proof);

        debug!("M3 payload: {:?}", payload);

        // Send the M3 request
        let response = self.http_client.post(url)
            .body(payload.to_vec())
            .send()
            .await?;

        assert_eq!(response.status(), 200, "Expected 200 OK response, got {}", response.status());

        let response_bytes = response.bytes().await?;
        let reader = Tlv8Reader::new(&response_bytes);
        Ok(reader.read()?)
    }

    fn handle_m4(&self, response: Vec<(TlvType, Vec<u8>)>) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
        debug!("Handling M4 response");
        
        let mut state: Option<u8> = None;
        let mut server_proof: Option<Vec<u8>> = None;
        let mut encrypted_data: Option<Vec<u8>> = None;
        let mut error: Option<u8> = None;

        for (tlv_type, value) in response {
            match tlv_type {
                TlvType::State => {
                    debug!("M4 State: {}", value[0]);
                    state = Some(value[0]);
                },
                TlvType::Proof => {
                    debug!("M4 Server Proof length: {} bytes", value.len());
                    server_proof = Some(value);
                },
                TlvType::EncryptedData => {
                    debug!("M4 Encrypted Data length: {} bytes", value.len());
                    encrypted_data = Some(value);
                },
                TlvType::Error => {
                    debug!("M4 Error: {}", value[0]);
                    error = Some(value[0]);
                },
                _ => {
                    debug!("Unexpected TLV type in M4 response: {:?}", tlv_type);
                },
            }
        }

        if let Some(error) = error {
            PairSetup::handle_error(error)?;
        }

        let state = state.ok_or("M4 response missing state")?;
        if state != PairingState::M4 as u8 {
            return Err(format!("Unexpected state in M4 response: {}", state).into());
        }

        let server_proof = server_proof.ok_or("M4 response missing server_proof")?;
        let encrypted_data = encrypted_data.ok_or("M4 response missing encrypted_data")?;

        Ok((server_proof, encrypted_data))
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

    fn handle_error(error_code: u8) -> Result<(), Box<dyn Error>> {
        match error_code {
            0x01 => Err("Unknown error".into()),
            0x02 => Err("Authentication failed: setup code incorrect".into()),
            0x03 => Err("Backoff: too many attempts, please try again later".into()),
            0x04 => Err("Max peers reached: too many paired controllers".into()),
            0x05 => Err("Max tries reached: too many attempts".into()),
            0x06 => Err("Unavailable: accessory is not ready to accept a new pairing".into()),
            0x07 => Err("Busy: accessory is busy with another operation".into()),
            _ => Err(format!("Unknown error code: {}", error_code).into()),
        }
    }

}
