use crate::bytes;

pub fn repeating_key_xor(
  input: &bytes::Bytes,
  key: &bytes::Bytes,
) -> Result<bytes::Bytes, &'static str> {
  let repeated_key = (0..input.len()).fold(Vec::new(), |mut acc, idx| {
    let key_idx = idx % key.len();
    acc.push(key[key_idx]);
    acc
  });

  let xored = bytes::fixed_xor_bytes(input, &repeated_key);

  Ok(xored)
}
