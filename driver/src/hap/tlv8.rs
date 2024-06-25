use std::convert::TryFrom;
use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlvType {
    Method = 0x00,
    Identifier = 0x01,
    Salt = 0x02,
    PublicKey = 0x03,
    Proof = 0x04,
    EncryptedData = 0x05,
    State = 0x06,
    Error = 0x07,
    RetryDelay = 0x08,
    Certificate = 0x09,
    Signature = 0x0A,
    Permissions = 0x0B,
    FragmentData = 0x0C,
    FragmentLast = 0x0D,
    Flags = 0x13,
    Separator = 0xFF,
}

impl Into<u8> for TlvType {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for TlvType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, <TlvType as TryFrom<u8>>::Error> {
        match value {
            0x00 => Ok(TlvType::Method),
            0x01 => Ok(TlvType::Identifier),
            0x02 => Ok(TlvType::Salt),
            0x03 => Ok(TlvType::PublicKey),
            0x04 => Ok(TlvType::Proof),
            0x05 => Ok(TlvType::EncryptedData),
            0x06 => Ok(TlvType::State),
            0x07 => Ok(TlvType::Error),
            0x08 => Ok(TlvType::RetryDelay),
            0x09 => Ok(TlvType::Certificate),
            0x0A => Ok(TlvType::Signature),
            0x0B => Ok(TlvType::Permissions),
            0x0C => Ok(TlvType::FragmentData),
            0x0D => Ok(TlvType::FragmentLast),
            0xFF => Ok(TlvType::Separator),
            _ => Err(()),
        }
    }
}

pub struct Tlv8Writer {
    buffer: Vec<u8>,
}

impl Tlv8Writer {
    pub fn new() -> Self {
        Tlv8Writer { buffer: Vec::new() }
    }

    pub fn add(&mut self, tlv_type: TlvType, value: &[u8]) {
        let t = tlv_type as u8;
        let mut remaining = value.len();
        let mut offset = 0;

        loop {
            let chunk_size = remaining.min(255);
            self.buffer.push(t);
            self.buffer.push(chunk_size as u8);
            if chunk_size > 0 {
                self.buffer.extend_from_slice(&value[offset..offset + chunk_size]);
            }
            
            remaining -= chunk_size;
            offset += chunk_size;

            if remaining == 0 {
                break;
            }
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.buffer
    }
}

impl fmt::Debug for Tlv8Writer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tlv8Writer {{")?;
        
        let reader = Tlv8Reader::new(&self.buffer);
        match reader.read() {
            Ok(tlv_items) => {
                for (tlv_type, value) in tlv_items {
                    write!(f, "    {:?} (0x{:02X}): ", tlv_type, tlv_type as u8)?;
                    
                    if value.len() <= 16 {
                        // For short values, print as hex
                        for byte in &value {
                            write!(f, "{:02X} ", byte)?;
                        }
                    } else {
                        // For longer values, print length and first few bytes
                        write!(f, "[{} bytes] {:02X} {:02X} {:02X} ...", 
                               value.len(), value[0], value[1], value[2])?;
                    }
                    writeln!(f)?;
                }
            },
            Err(e) => {
                writeln!(f, "    Error parsing TLV data: {}", e)?;
            }
        }
        
        write!(f, "}}")
    }
}

pub struct Tlv8Reader<'a> {
    input: &'a [u8],
}

impl<'a> Tlv8Reader<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Tlv8Reader { input }
    }

    pub fn read(&self) -> Result<Vec<(TlvType, Vec<u8>)>, Box<dyn Error>> {
        let mut decoded = Vec::new();
        let mut p = 0;
        let mut current_type: Option<TlvType> = None;
        let mut current_value = Vec::new();

        while p < self.input.len() {
            if p + 2 > self.input.len() {
                return Err("Incomplete TLV".into());
            }

            let t = self.input[p];
            let l = self.input[p + 1] as usize;
            p += 2;

            if p + l > self.input.len() {
                return Err("Invalid length in TLV".into());
            }

            let tlv_type = TlvType::try_from(t)
                .map_err(|_| format!("Invalid TLV type: {}", t))?;

            if Some(tlv_type) != current_type {
                if let Some(typ) = current_type {
                    decoded.push((typ, current_value));
                    current_value = Vec::new();
                }
                current_type = Some(tlv_type);
            }

            current_value.extend_from_slice(&self.input[p..p + l]);
            p += l;
        }

        if let Some(typ) = current_type {
            decoded.push((typ, current_value));
        }

        Ok(decoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_and_read_simple_tlv() {
        let mut writer = Tlv8Writer::new();
        writer.add(TlvType::State, &[1]);
        writer.add(TlvType::Method, &[2]);

        let encoded = writer.to_vec();
        let reader = Tlv8Reader::new(&encoded);
        let result = reader.read().unwrap();

        assert_eq!(result, vec![
            (TlvType::State, vec![1]),
            (TlvType::Method, vec![2]),
        ]);
    }

    #[test]
    fn test_write_and_read_empty_value() {
        let mut writer = Tlv8Writer::new();
        writer.add(TlvType::Error, &[]);

        let encoded = writer.to_vec();
        let reader = Tlv8Reader::new(&encoded);
        let result = reader.read().unwrap();

        assert_eq!(result, vec![(TlvType::Error, vec![])]);
    }

    #[test]
    fn test_write_and_read_255_byte_value() {
        let mut writer = Tlv8Writer::new();
        let data = vec![0; 255];
        writer.add(TlvType::PublicKey, &data);

        let encoded = writer.to_vec();
        let reader = Tlv8Reader::new(&encoded);
        let result = reader.read().unwrap();

        assert_eq!(result, vec![(TlvType::PublicKey, data)]);
    }

    #[test]
    fn test_write_and_read_256_byte_value() {
        let mut writer = Tlv8Writer::new();
        let data = vec![0; 256];
        writer.add(TlvType::PublicKey, &data);

        let encoded = writer.to_vec();
        let reader = Tlv8Reader::new(&encoded);
        let result = reader.read().unwrap();

        assert_eq!(result, vec![(TlvType::PublicKey, data)]);
    }

    #[test]
    fn test_write_and_read_large_value() {
        let mut writer = Tlv8Writer::new();
        let data = vec![0; 1000];
        writer.add(TlvType::EncryptedData, &data);

        let encoded = writer.to_vec();
        let reader = Tlv8Reader::new(&encoded);
        let result = reader.read().unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], (TlvType::EncryptedData, data));
    }

    #[test]
    fn test_write_and_read_multiple_types() {
        let mut writer = Tlv8Writer::new();
        writer.add(TlvType::State, &[1]);
        writer.add(TlvType::Error, &[2, 3]);
        writer.add(TlvType::Certificate, &vec![4; 300]);

        let encoded = writer.to_vec();
        let reader = Tlv8Reader::new(&encoded);
        let result = reader.read().unwrap();

        assert_eq!(result, vec![
            (TlvType::State, vec![1]),
            (TlvType::Error, vec![2, 3]),
            (TlvType::Certificate, vec![4; 300]),
        ]);
    }

    #[test]
    fn test_read_incomplete_tlv() {
        let data = vec![0x06, 0x01]; // Missing value byte
        let reader = Tlv8Reader::new(&data);
        assert!(reader.read().is_err());
    }

    #[test]
    fn test_read_invalid_length() {
        let data = vec![0x06, 0x02, 0x01]; // Length 2 but only 1 byte of value
        let reader = Tlv8Reader::new(&data);
        assert!(reader.read().is_err());
    }


    #[test]
    fn test_debug_output() {
        let mut writer = Tlv8Writer::new();
        writer.add(TlvType::State, &[1]);
        writer.add(TlvType::PublicKey, &vec![2; 300]);

        let debug_output = format!("{:?}", writer);
        assert!(debug_output.contains("State (0x06): 01"));
        assert!(debug_output.contains("PublicKey (0x03): [300 bytes] 02 02 02 ..."));
    }

    #[test]
    fn test_write_and_read_all_types() {
        let mut writer = Tlv8Writer::new();
        writer.add(TlvType::Method, &[1]);
        writer.add(TlvType::Identifier, &[2, 3]);
        writer.add(TlvType::Salt, &[4, 5, 6]);
        writer.add(TlvType::PublicKey, &vec![7; 300]);
        writer.add(TlvType::Proof, &[8, 9, 10, 11]);
        writer.add(TlvType::EncryptedData, &vec![12; 1000]);
        writer.add(TlvType::State, &[13]);
        writer.add(TlvType::Error, &[14]);
        writer.add(TlvType::RetryDelay, &[15, 16]);
        writer.add(TlvType::Certificate, &vec![17; 500]);
        writer.add(TlvType::Signature, &vec![18; 64]);
        writer.add(TlvType::Permissions, &[19]);
        writer.add(TlvType::FragmentData, &vec![20; 256]);
        writer.add(TlvType::FragmentLast, &vec![21; 100]);
        writer.add(TlvType::Separator, &[]);

        let encoded = writer.to_vec();
        let reader = Tlv8Reader::new(&encoded);
        let result = reader.read().unwrap();

        assert_eq!(result[0], (TlvType::Method, vec![1]));
        assert_eq!(result[1], (TlvType::Identifier, vec![2, 3]));
        assert_eq!(result[2], (TlvType::Salt, vec![4, 5, 6]));
        assert_eq!(result[3], (TlvType::PublicKey, vec![7; 300]));
        assert_eq!(result[4], (TlvType::Proof, vec![8, 9, 10, 11]));
        assert_eq!(result[5], (TlvType::EncryptedData, vec![12; 1000]));
        assert_eq!(result[6], (TlvType::State, vec![13]));
        assert_eq!(result[7], (TlvType::Error, vec![14]));
        assert_eq!(result[8], (TlvType::RetryDelay, vec![15, 16]));
        assert_eq!(result[9], (TlvType::Certificate, vec![17; 500]));
        assert_eq!(result[10], (TlvType::Signature, vec![18; 64]));
        assert_eq!(result[11], (TlvType::Permissions, vec![19]));
        assert_eq!(result[12], (TlvType::FragmentData, vec![20; 256]));
        assert_eq!(result[13], (TlvType::FragmentLast, vec![21; 100]));
        assert_eq!(result[14], (TlvType::Separator, vec![]));

        assert_eq!(15, result.len());
    }

    #[test]
    fn test_encode_empty_value() {
        let mut writer = Tlv8Writer::new();
        writer.add(TlvType::Separator, &[]);

        let encoded = writer.to_vec();
        let expected = [0xFF, 0x00];

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_write_and_read_boundary_values() {
        let mut writer = Tlv8Writer::new();
        writer.add(TlvType::State, &[0]);
        writer.add(TlvType::Error, &[255]);
        writer.add(TlvType::PublicKey, &vec![0; 254]);
        writer.add(TlvType::EncryptedData, &vec![255; 257]);

        let encoded = writer.to_vec();
        let reader = Tlv8Reader::new(&encoded);
        let result = reader.read().unwrap();

        assert_eq!(result[0], (TlvType::State, vec![0]));
        assert_eq!(result[1], (TlvType::Error, vec![255]));
        assert_eq!(result[2], (TlvType::PublicKey, vec![0; 254]));
        assert_eq!(result[3], (TlvType::EncryptedData, vec![255; 257]));
    }
}