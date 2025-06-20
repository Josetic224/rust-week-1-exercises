// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // Check if the input is at least 8 hex chars (4 bytes)
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }
    // Take the first 8 hex chars (4 bytes)
    let version_hex = &raw_tx_hex[0..8];
    // Convert hex to bytes
    let bytes = match hex::decode(version_hex) {
        Ok(b) if b.len() == 4 => b,
        Ok(_) => return Err("Version hex does not decode to 4 bytes".to_string()),
        Err(e) => return Err(format!("Hex decode error: {}", e)), // error message should contain 'Hex decode error'
    };
    // Interpret as little-endian u32
    let version = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    Ok(version)
}
