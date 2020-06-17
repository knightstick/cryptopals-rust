mod tests {
  #[test]
  fn test_hex_to_bytes() {
    let valid_input = "49af".to_string();
    let output = cryptopals::hex_to_bytes(valid_input).unwrap();
    let expected: Vec<u8> = vec![4, 9, 10, 15];

    assert_eq!(output, expected);

    let invalid_input = "invalid".to_string();

    assert!(
      cryptopals::hex_to_bytes(invalid_input).is_err(),
      "should not succeed"
    )
  }

  #[test]
  fn test_bytes_to_base64() {
    let valid_input: Vec<u8> = vec![0b1111, 0b0000, 0b1111];
    let output = cryptopals::bytes_to_base64(valid_input).unwrap();
    let expected = "8P"; // 0b111100, 0b001111 -> 60, 15 -> 8, P

    assert_eq!(output, expected);
  }

  #[test]
  fn test_hex_to_base64() {
    let valid_input = "f0f".to_string();
    let output = cryptopals::hex_to_base64(valid_input);
    let expected = "8P".to_string();

    assert_eq!(output, expected);
  }
}
