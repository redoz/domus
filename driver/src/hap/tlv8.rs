use std::collections::BTreeMap;

pub struct TLV8Encoder;

impl TLV8Encoder {
    pub fn new() -> Self {
        TLV8Encoder
    }

    pub fn encode(&self, data: &BTreeMap<u8, Vec<u8>>) -> Vec<u8> {
        let mut encoded = Vec::new();
        for (&tag, value) in data {
            let mut offset = 0;
            while offset < value.len() {
                let chunk_size = std::cmp::min(255, value.len() - offset);
                encoded.push(tag);
                encoded.push(chunk_size as u8);
                encoded.extend_from_slice(&value[offset..offset + chunk_size]);
                offset += chunk_size;
            }
        }
        encoded
    }

    pub fn decode(&self, data: &[u8]) -> Result<BTreeMap<u8, Vec<u8>>, String> {
        let mut decoded = BTreeMap::new();
        let mut i = 0;
        while i < data.len() {
            if i + 2 > data.len() {
                return Err("Incomplete TLV".to_string());
            }
            let tag = data[i];
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_single_item() {
        let codec = TLV8Encoder::new();
        let mut data = BTreeMap::new();
        data.insert(1, vec![0x12, 0x34]); // Insert a single item with tag 1 and value [0x12, 0x34]
        let encoded = codec.encode(&data);
        // Expected output: [tag, length, value1, value2]
        assert_eq!(encoded, vec![1, 2, 0x12, 0x34]);
    }

    #[test]
    fn test_encode_multiple_items() {
        let codec = TLV8Encoder::new();
        let mut data = BTreeMap::new();
        data.insert(1, vec![0x12, 0x34]); // First item: tag 1, value [0x12, 0x34]
        data.insert(2, vec![0x56]); // Second item: tag 2, value [0x56]
        let encoded = codec.encode(&data);
        // Expected output: [tag1, length1, value1, value2, tag2, length2, value3]
        assert_eq!(encoded, vec![1, 2, 0x12, 0x34, 2, 1, 0x56]);
    }

    #[test]
    fn test_encode_long_value() {
        let codec = TLV8Encoder::new();
        let mut data = BTreeMap::new();
        data.insert(1, vec![0; 300]); // Insert a single item with tag 1 and 300 zero bytes
        let encoded = codec.encode(&data);
        assert_eq!(encoded.len(), 304); // Total length: 257 (first chunk) + 47 (second chunk) = 304 bytes
        assert_eq!(encoded[0], 1); // First chunk: tag
        assert_eq!(encoded[1], 255); // First chunk: length (max 255)
        assert_eq!(encoded[2..257], vec![0; 255]); // First chunk: 255 zero bytes
        assert_eq!(encoded[257], 1); // Second chunk: tag
        assert_eq!(encoded[258], 45); // Second chunk: length (remaining 45 bytes)
        assert_eq!(encoded[259..304], vec![0; 45]); // Second chunk: 45 zero bytes
    }

    #[test]
    fn test_decode_single_item() {
        let codec = TLV8Encoder::new();
        let encoded = vec![1, 2, 0x12, 0x34]; // Single item: tag 1, length 2, value [0x12, 0x34]
        let decoded = codec.decode(&encoded).unwrap();
        assert_eq!(decoded.len(), 1); // Should contain one item
        assert_eq!(decoded[&1], vec![0x12, 0x34]); // Item with tag 1 should have value [0x12, 0x34]
    }

    #[test]
    fn test_decode_multiple_items() {
        let codec = TLV8Encoder::new();
        let encoded = vec![1, 2, 0x12, 0x34, 2, 1, 0x56]; // Two items: (1, [0x12, 0x34]) and (2, [0x56])
        let decoded = codec.decode(&encoded).unwrap();
        assert_eq!(decoded.len(), 2); // Should contain two items
        assert_eq!(decoded[&1], vec![0x12, 0x34]); // Item with tag 1
        assert_eq!(decoded[&2], vec![0x56]); // Item with tag 2
    }

    #[test]
    fn test_decode_long_value() {
        let codec = TLV8Encoder::new();
        let mut encoded = vec![1, 255]; // First chunk: tag 1, length 255
        encoded.extend_from_slice(&vec![0; 255]); // First chunk: 255 zero bytes
        encoded.extend_from_slice(&[1, 45]); // Second chunk: tag 1, length 45
        encoded.extend_from_slice(&vec![0; 45]); // Second chunk: 45 zero bytes
        let decoded = codec.decode(&encoded).unwrap();
        assert_eq!(decoded.len(), 1); // Should contain one item
        assert_eq!(decoded[&1].len(), 300); // Item with tag 1 should have 300 bytes
    }

    #[test]
    fn test_decode_invalid_data() {
        let codec = TLV8Encoder::new();
        let encoded = vec![1, 2, 0x12]; // Incomplete TLV
        assert!(codec.decode(&encoded).is_err());
    }

    #[test]
    fn test_roundtrip() {
        let codec = TLV8Encoder::new();
        let mut original = BTreeMap::new();
        original.insert(1, vec![0x12, 0x34]);
        original.insert(2, vec![0x56]);
        original.insert(3, vec![0; 300]);
        let encoded = codec.encode(&original);
        let decoded = codec.decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }
}