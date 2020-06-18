mod tests {
  #[test]
  fn test_fixed_xor() {
    let first = "f0f0".to_string();
    let second = "0f0f".to_string();
    let output = cryptopals::fixed_xor(first, second).unwrap();
    let expected = "ffff";

    assert_eq!(output, expected);
  }

  #[test]
  fn test_fixed_xor_again() {
    let first = "3333".to_string();
    let second = "1248".to_string();
    let output = cryptopals::fixed_xor(first, second).unwrap();
    let expected = "217b";

    assert_eq!(output, expected);
  }
}
