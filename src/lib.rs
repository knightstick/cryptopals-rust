pub mod bytes;
pub mod conversion;
pub mod decipher;
pub mod language;

pub fn hex_to_base64(hex: String) -> Result<String, &'static str> {
  let bytes = conversion::hex_to_bytes(&hex)?;
  conversion::bytes_to_base64(bytes)
}

pub fn fixed_xor(first: &String, second: &String) -> Result<String, &'static str> {
  let first_bytes = conversion::hex_to_bytes(first)?;
  let second_bytes = conversion::hex_to_bytes(second)?;

  let xored = bytes::fixed_xor_bytes(&first_bytes, &second_bytes);
  let output = conversion::bytes_to_hex(xored)?;

  Ok(output)
}

pub fn decipher_single_byte_xor(input: String) -> Result<String, &'static str> {
  let input_bytes = conversion::hex_to_bytes(&input)?;
  decipher::single_byte_xor(input_bytes)
}
