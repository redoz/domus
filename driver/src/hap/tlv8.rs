use std::convert::TryFrom;
use std::fmt;

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

    pub fn add(&mut self, tag: TlvType, value: &[u8]) {
        let mut offset = 0;
        while offset < value.len() {
            let chunk_size = std::cmp::min(255, value.len() - offset);
            self.buffer.push(tag.into());
            self.buffer.push(chunk_size as u8);
            self.buffer.extend_from_slice(&value[offset..offset + chunk_size]);
            offset += chunk_size;
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
        let mut offset = 0;
        while offset < self.buffer.len() {
            if offset + 2 > self.buffer.len() {
                writeln!(f, "    Invalid TLV data")?;
                break;
            }
            let tag = self.buffer[offset];
            let length = self.buffer[offset + 1] as usize;
            offset += 2;
            
            if offset + length > self.buffer.len() {
                writeln!(f, "    Invalid TLV length")?;
                break;
            }
            
            let value = &self.buffer[offset..offset + length];
            offset += length;

            let tlv_type = TlvType::try_from(tag).unwrap_or(TlvType::Separator);
            write!(f, "    {:?} (0x{:02X}): ", tlv_type, tag)?;
            
            if length <= 16 {
                // For short values, print as hex
                for byte in value {
                    write!(f, "{:02X} ", byte)?;
                }
            } else {
                // For longer values, print length and first few bytes
                write!(f, "[{} bytes] {:02X} {:02X} {:02X} ...", length, value[0], value[1], value[2])?;
            }
            writeln!(f)?;
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

    pub fn read(&self) -> Result<Vec<(TlvType, Vec<u8>)>, String> {
        let mut decoded = Vec::new();
        let mut i = 0;
        while i < self.input.len() {
            if i + 2 > self.input.len() {
                return Err("Incomplete TLV".to_string());
            }
            let tag = TlvType::try_from(self.input[i])
                .map_err(|_| format!("Invalid TLV type: {}", self.input[i]))?;
            let length = self.input[i + 1] as usize;
            i += 2;
            if i + length > self.input.len() {
                return Err("Invalid length in TLV".to_string());
            }
            decoded.push((tag, self.input[i..i + length].to_vec()));
            i += length;
        }
        Ok(decoded)
    }
}
