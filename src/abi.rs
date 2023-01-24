pub const OCR_CONTRACT_CORE_ABI: [&str; 1] = [
    "function protocolVersion() external view returns (uint256)",
];

pub const OCR_CONTRACT_ABI_V1: [&str; 7] = [
    "event StartPublish(uint256 indexed packageIndex, address indexed author)",
    "event EndPublish(uint256 indexed packageIndex, uint64 partCount)",
    "event PackagePart(uint256 indexed packageIndex, uint64 partIndex, bytes data)",
    "function protocolVersion() external view returns (uint256)",
    "function startPublish(bytes memory data, bool end) external returns(uint256)",
    "function publishPart(uint256 packageIndex, bytes memory data, bool end) external",
    "function package(uint256 packageIndex) public view returns(tuple(uint256 startBlock, uint256 endBlock, address author, uint64 partCount))"
];

pub fn build_ocr_contract_abi(protocol_version: i32) -> Option<Vec<&'static str>> {
    match protocol_version {
        1 => Some(OCR_CONTRACT_ABI_V1.to_vec()),
        _ => None
    }
}