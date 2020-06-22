pub type Bytes = Vec<u8>;

pub fn fixed_xor_bytes(first: &Bytes, second: &Bytes) -> Bytes {
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
