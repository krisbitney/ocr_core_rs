use hex;
use crate::util::{encode_fixed_bytes, decode_fixed_bytes, vec_to_array};
use std::convert::TryInto;
use crate::{is_hex_string, is_ocr_contenthash_format};

/// OCR Protocol ID
pub const OCR_PROTOCOL_ID: u8 = 77;

/// OCR package ID in struct data representation
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct OcrId {
    pub protocol_version: u16,
    pub chain_id: u64,
    pub package_index: u64,
    pub contract_address: String,
    pub start_block: u64,
    pub end_block: u64
}

impl TryFrom<&str> for OcrId {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        decode_ocr_id_from_contenthash(value, None)
    }
}

impl TryInto<String> for OcrId {
    type Error = String;

    fn try_into(self) -> Result<String, Self::Error> {
        encode_ocr_id_as_contenthash_string(&self, None)
    }
}

impl TryInto<Vec<u8>> for OcrId {
    type Error = String;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        encode_ocr_id_as_contenthash(&self, None)
    }
}

/// Encode an OCR ID to its contenthash byte representation
pub fn encode_ocr_id_as_contenthash(ocr_id: &OcrId, ocr_protocol_id: Option<u8>) -> Result<Vec<u8>, String> {
    let protocol_id = ocr_protocol_id.unwrap_or(OCR_PROTOCOL_ID);

    if !is_hex_string(&ocr_id.contract_address, 20) {
        return Err(format!("Contract address is not a valid hex string"));
    }
    let contract_address: Vec<u8> = hex::decode(&ocr_id.contract_address[2..]).map_err(|e| format!("{}", e))?;

    let bytes: Vec<Vec<u8>> = Vec::from([
        protocol_id.to_be_bytes().to_vec(),
        ocr_id.protocol_version.to_be_bytes().to_vec(),
        ocr_id.chain_id.to_be_bytes().to_vec(),
        contract_address,
        ocr_id.package_index.to_be_bytes().to_vec(),
        ocr_id.start_block.to_be_bytes().to_vec(),
        ocr_id.end_block.to_be_bytes().to_vec(),
    ]);
    let lengths = Vec::from([1usize, 2, 8, 20, 8, 8, 8]);

    let encoded_bytes = encode_fixed_bytes(&bytes, &lengths);

    return Ok(encoded_bytes);
}

/// Encode an OCR ID to its contenthash hex string representation
pub fn encode_ocr_id_as_contenthash_string(ocr_id: &OcrId, ocr_protocol_id: Option<u8>) -> Result<String, String> {
    let encoded_bytes = encode_ocr_id_as_contenthash(ocr_id, ocr_protocol_id)?;
    Ok("0x".to_string() + &hex::encode(encoded_bytes))
}

/// Decode an OCR ID from its contenthash hex string representation
pub fn decode_ocr_id_from_contenthash(contenthash: &str, ocr_protocol_id: Option<u8>) -> Result<OcrId, String> {
    if !is_ocr_contenthash_format(contenthash, ocr_protocol_id) {
        return Err(format!("Contenthash is an invalid hex string or has an invalid OCR protocol ID"));
    }
    let contenthash_bytes: Vec<u8> = hex::decode(&contenthash[2..]).map_err(|e| format!("{}", e))?;

    let lengths: Vec<usize> = Vec::from([2, 8, 20, 8, 8, 8]);
    let ocr_id_data = decode_fixed_bytes(&contenthash_bytes, &lengths, 1)?;
    let protocol_version_bytes = ocr_id_data[0].clone();
    let chain_id_bytes = ocr_id_data[1].clone();
    let contract_address_bytes = ocr_id_data[2].clone();
    let package_index_bytes = ocr_id_data[3].clone();
    let start_block_bytes = ocr_id_data[4].clone();
    let end_block_bytes = ocr_id_data[5].clone();

    let protocol_version: u16 = u16::from_be_bytes(vec_to_array(protocol_version_bytes)?);
    let chain_id: u64 = u64::from_be_bytes(vec_to_array(chain_id_bytes)?);
    let contract_address: String = "0x".to_string() + &hex::encode(contract_address_bytes);
    let package_index: u64 = u64::from_be_bytes(vec_to_array(package_index_bytes)?);
    let start_block: u64 = u64::from_be_bytes(vec_to_array(start_block_bytes)?);
    let end_block: u64 = u64::from_be_bytes(vec_to_array(end_block_bytes)?);

    Ok(OcrId {
        protocol_version,
        chain_id,
        contract_address,
        package_index,
        start_block,
        end_block,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_encode_decode_contenthash() {
        let ocr_id: OcrId = OcrId {
            protocol_version: 231,
            chain_id: 5_354,
            package_index: 1_123_323,
            contract_address: "0x000000000000000000000000000000000f0a5b56".to_string(),
            start_block: 15_000_324,
            end_block: 15_000_420,
        };
        let encoded = encode_ocr_id_as_contenthash_string(&ocr_id, None).unwrap();
        assert_eq!(encoded, "0x4d00e700000000000014ea000000000000000000000000000000000f0a5b5600000000001123fb0000000000e4e3040000000000e4e364");
        let decoded = decode_ocr_id_from_contenthash(&encoded, None).unwrap();
        assert_eq!(decoded, ocr_id);
    }
}
