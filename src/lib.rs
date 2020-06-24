use std::path::PathBuf;

pub mod bytes;
pub mod conversion;
pub mod decipher;
pub mod encrypt;
pub mod language;
pub mod utils;

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

pub fn detect_single_character_xor(filename: PathBuf) -> Result<String, &'static str> {
  let candidate_bytes = utils::bytes_from_file(filename)?;
  decipher::detect_single_character_xor(candidate_bytes)
}

pub fn repeating_key_xor(input: &String, key: &String) -> Result<String, &'static str> {
  let input_bytes = input.clone().into_bytes();
  let key_bytes: bytes::Bytes = key.clone().into_bytes();

  let xored_bytes = encrypt::repeating_key_xor(&input_bytes, &key_bytes)?;
  let hex = conversion::bytes_to_hex(xored_bytes)?;

  Ok(hex)
}
