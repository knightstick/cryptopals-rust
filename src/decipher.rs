use crate::bytes;
use crate::language;

pub fn single_byte_xor(input: crate::bytes::Bytes) -> Result<String, &'static str> {
  let highest_with_score = (0..0b11111111).fold(("".to_string(), 0), |acc, ch| {
    let (xored, score) = score_char(&input, ch);
    if score > acc.1 {
      (xored, score)
    } else {
      acc
    }
  });

  Ok(highest_with_score.0)
}

fn score_char(input_bytes: &bytes::Bytes, ch: u8) -> (String, usize) {
  let length = input_bytes.len();

  // XOR against the char repeated the right number of times
  let repeated = vec![ch; length];
  let xored_bytes = bytes::fixed_xor_bytes(&input_bytes, &repeated);
  let xored = String::from_utf8_lossy(&xored_bytes).into_owned();

  // Score the letter freqeuncies of the top 12 letters
  let score = language::score_string(&xored);

  (xored, score)
}
