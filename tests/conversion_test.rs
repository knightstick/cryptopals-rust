mod tests {
  #[test]
  fn test_hex_to_bytes() {
    let valid_input = "49af".to_string();
    let output = cryptopals::conversion::hex_to_bytes(valid_input).unwrap();
    // 0b0100 0b1001 0b1010 0b1111 squashed
    let expected: Vec<u8> = vec![0b01001001, 0b10101111];

    assert_eq!(output, expected);

    let invalid_input = "invalid".to_string();

    assert!(
      cryptopals::conversion::hex_to_bytes(invalid_input).is_err(),
      "should not succeed"
    )
  }

  #[test]
  fn test_bytes_to_base64() {
    let valid_input: Vec<u8> = vec![0b11110000, 0b11110000, 0b11110000];
    let output = cryptopals::conversion::bytes_to_base64(valid_input).unwrap();
    let expected = "8PDw"; // 0b111100, 0b001111, 0b000011, 0b110000 -> 60, 15, 3, 48 -> 8, P, D, w

    assert_eq!(output, expected);
  }

  #[test]
  fn test_hex_to_base64() {
    let valid_input = "f0f0f0".to_string();
    let output = cryptopals::hex_to_base64(valid_input).unwrap();
    let expected = "8PDw".to_string();

    assert_eq!(output, expected);
  }

  #[test]
  fn test_bytes_to_hex_simplest() {
    let valid_input: Vec<u8> = vec![0b11111111];
    let output = cryptopals::conversion::bytes_to_hex(valid_input).unwrap();
    let expected = "ff";

    assert_eq!(output, expected);
  }

  #[test]
  fn test_bytes_to_hex() {
    let valid_input: Vec<u8> = vec![0b11111111, 0b10000000, 0b00000001];
    let output = cryptopals::conversion::bytes_to_hex(valid_input).unwrap();
    let expected = "ff8001";

    assert_eq!(output, expected);
  }

  #[test]
  fn test_bytes_to_hex_again() {
    let valid_input: Vec<u8> = vec![0b11111111, 0b11111111];
    let output = cryptopals::conversion::bytes_to_hex(valid_input).unwrap();
    let expected = "ffff";

    assert_eq!(output, expected);
  }
}
