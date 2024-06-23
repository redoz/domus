use std::convert::TryFrom;

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

pub struct Tlv8Reader {
    //data: BTreeMap<TlvType, Vec<u8>>,
}

/* 
impl Tlv8Reader {
    pub fn new(input: &[u8]) -> Result<Self, String> {
        Ok(Tlv8Reader {
            data: Self::decode(input)?,
        })
    }

    fn decode(data: &[u8]) -> Result<BTreeMap<TlvType, Vec<u8>>, String> {
        let mut decoded = BTreeMap::new();
        let mut i = 0;
        while i < data.len() {
            if i + 2 > data.len() {
                return Err("Incomplete TLV".to_string());
            }
            let tag = match TlvType::try_from(data[i]) {
                Ok(t) => t,
                Err(_) => return Err(format!("Invalid TLV type: {}", data[i])),
            };
            let length = data[i + 1] as usize;
            i += 2;
            if i + length > data.len() {
                return Err("Invalid length in TLV".to_string());
            }
            let value = data[i..i + length].to_vec();
            i += length;
            decoded.entry(tag).or_insert_with(Vec::new).extend_from_slice(&value);
        }
        Ok(decoded)
    }

    pub fn get(&self, tag: TlvType) -> Option<&Vec<u8>> {
        self.data.get(&tag)
    }
}
*/