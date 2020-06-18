pub mod conversion;

type Bytes = Vec<u8>;

pub fn hex_to_base64(hex: String) -> Result<String, &'static str> {
  let bytes = conversion::hex_to_bytes(hex)?;
  conversion::bytes_to_base64(bytes)
}

pub fn fixed_xor(first: String, second: String) -> Result<String, &'static str> {
  let first_bytes = conversion::hex_to_bytes(first)?;
  let second_bytes = conversion::hex_to_bytes(second)?;

  let xored = fixed_xor_bytes(first_bytes, second_bytes);
  let output = conversion::bytes_to_hex(xored)?;

  Ok(output)
}

pub fn fixed_xor_bytes(first: Bytes, second: Bytes) -> Bytes {
  let mut result: Bytes = Vec::new();

  for idx in 0..first.len() {
    result.push(first[idx] ^ second[idx]);
  }

  result
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
