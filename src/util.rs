use crate::{OCR_PROTOCOL_ID};

pub fn vec_to_array<T, const N: usize>(v: Vec<T>) -> Result<[T; N], String> {
    v.try_into().map_err(|v: Vec<T>|
        format!("Expected Vec of length {}. Received Vec of length {}.", N, v.len())
    )
}

/// Returns true if value is a valid hex byte string that is prefixed with '0x'.
pub fn is_hex_string(value: &str, length: i32) -> bool {
    if !value.starts_with("0x") {
        return false;
    }
    for c in value[2..].chars() {
        if !c.is_ascii_hexdigit() {
            return false;
        }
    }
    if value.len() != (2 + 2 * length) as usize {
        return false;
    }
    return true;
}

/// Returns true if value is a valid hex byte string that is prefixed with '0x' and contains a valid OCR protocol ID.
pub fn is_ocr_contenthash_format(value: &str, ocr_protocol_id: Option<u8>) -> bool {
    if !is_hex_string(value, 55) {
        return false;
    }
    let valid_protocol_id = ocr_protocol_id.unwrap_or(OCR_PROTOCOL_ID);
    let hash_protocol_id: u8 = hex::decode(&value[2..4]).unwrap().first().unwrap().clone();
    return hash_protocol_id == valid_protocol_id
}

/// Encode an array of buffers into a single, flattened buffer.
/// The buffers are front-padded with zeros to ensure they meet their corresponding lengths.
/// If a buffer's length exceeds its corresponding target length, excess bytes are ignored.
pub fn encode_fixed_bytes(array_of_bytes: &Vec<Vec<u8>>, lengths: &Vec<usize>) -> Vec<u8> {
    let mut all_encoded_bytes: Vec<u8> = Vec::with_capacity(lengths.iter().sum());

    for i in 0..lengths.len() {
        let bytes = &array_of_bytes[i];
        let length = lengths[i];
        for _j in 0..(length - bytes.len()) {
            all_encoded_bytes.push(0);
        }
        for j in 0..bytes.len() {
            all_encoded_bytes.push(bytes[j]);
        }
    }

    return all_encoded_bytes;
}

/// Decode buffer into an array of buffers with lengths corresponding to the provided target lengths.
/// An offset can be used to ignore the first N bytes of the input buffer.
/// The sum of lengths is expected to equal the adjusted length of the byte array.
pub fn decode_fixed_bytes(bytes: &Vec<u8>, lengths: &Vec<usize>, offset: usize) -> Result<Vec<Vec<u8>>, String> {
    let length_sum: usize = lengths.iter().sum();
    if bytes.len() - offset != length_sum {
        return Err(format!("Sum of lengths {} does not match the number of bytes {}", length_sum, bytes.len()))
    }

    let mut array_of_bytes: Vec<Vec<u8>> = Vec::new();
    let mut offset = offset;
    for length in lengths {
        let buffer = &bytes[offset..offset + length];
        array_of_bytes.push(buffer.to_vec());
        offset += length;
    }

    return Ok(array_of_bytes);
}