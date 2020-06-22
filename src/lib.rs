use std::collections::HashMap;

pub mod conversion;
pub mod language;

type Bytes = Vec<u8>;

pub fn hex_to_base64(hex: String) -> Result<String, &'static str> {
  let bytes = conversion::hex_to_bytes(&hex)?;
  conversion::bytes_to_base64(bytes)
}

pub fn fixed_xor(first: &String, second: &String) -> Result<String, &'static str> {
  let first_bytes = conversion::hex_to_bytes(first)?;
  let second_bytes = conversion::hex_to_bytes(second)?;

  let xored = fixed_xor_bytes(&first_bytes, &second_bytes);
  let output = conversion::bytes_to_hex(xored)?;

  Ok(output)
}

pub fn fixed_xor_bytes(first: &Bytes, second: &Bytes) -> Bytes {
  let mut result: Bytes = Vec::new();

  for idx in 0..first.len() {
    result.push(first[idx] ^ second[idx]);
  }

  result
}

pub fn decipher_single_byte_xor(input: String) -> Result<String, &'static str> {
  let length = input.len();
  let input_bytes = conversion::hex_to_bytes(&input)?;

  let mut scores: HashMap<String, usize> = HashMap::new();

  // For each single byte char
  for ch_int in 0..0b11111111 {
    let ch_u8 = ch_int as u8;

    // XOR against the char repeated the right number of times
    let repeated = vec![ch_u8; length];
    let xored_bytes = fixed_xor_bytes(&input_bytes, &repeated);
    let xored = String::from_utf8_lossy(&xored_bytes).into_owned();

    // Score the letter freqeuncies of the top 12 letters
    let score = language::score_string(&xored);
    scores.insert(xored, score);
  }

  // Return the XORed value with the highest score
  let mut score_vec: Vec<(&String, &usize)> = scores.iter().collect();
  score_vec.sort_by(|a, b| b.1.cmp(a.1));

  Ok(score_vec[0].0.to_string())
}

pub fn print_bytes_in_binary(bytes: Bytes) {
  print!("[");

  for byte in bytes.iter() {
    print!("{:08b} ", byte);
  }

  println!("]");
}

pub fn print_bytes(bytes: &Bytes) {
  print!("[");

  for byte in bytes.iter() {
    print!("{} ", byte);
  }

  println!("]");
}
