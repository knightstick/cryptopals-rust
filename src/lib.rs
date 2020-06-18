pub mod conversion;

pub fn hex_to_base64(hex: String) -> Result<String, &'static str> {
  let bytes = conversion::hex_to_bytes(hex)?;
  conversion::bytes_to_base64(bytes)
}

pub fn print_bytes_in_binary(bytes: &Vec<u8>) {
  print!("[");

  for byte in bytes.iter() {
    print!("{:08b} ", byte);
  }

  println!("]");
}

pub fn print_bytes(bytes: &Vec<u8>) {
  print!("[");

  for byte in bytes.iter() {
    print!("{} ", byte);
  }

  println!("]");
}
