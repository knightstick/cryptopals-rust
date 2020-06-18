pub fn hex_to_bytes(hex: String) -> Result<Vec<u8>, &'static str> {
  let mut nibbles = Vec::new();

  for hex_char in hex.chars() {
    let byte_code = match hex_char {
      '0' => 0,
      '1' => 1,
      '2' => 2,
      '3' => 3,
      '4' => 4,
      '5' => 5,
      '6' => 6,
      '7' => 7,
      '8' => 8,
      '9' => 9,
      'a' => 10,
      'b' => 11,
      'c' => 12,
      'd' => 13,
      'e' => 14,
      'f' => 15,
      _ => return Err("Invalid hex character"),
    };

    nibbles.push(byte_code)
  }

  let mut compacted: Vec<u8> = Vec::new();

  for i in (0..nibbles.len()).step_by(2) {
    let first_nibble = nibbles[i];

    let second_nibble = if i + 1 < nibbles.len() {
      nibbles[i + 1]
    } else {
      0
    };

    let byte = (first_nibble << 4) + second_nibble; // joins two nibbles into one byte, 1111, 0101 -> 11110101;

    compacted.push(byte);
  }

  Ok(compacted)
}

pub fn bytes_to_base64(bytes: Vec<u8>) -> Result<String, &'static str> {
  let mut result = String::new();

  for byte in each_six(bytes).iter() {
    let symbol = match byte {
      0 => 'A',
      1 => 'B',
      2 => 'C',
      3 => 'D',
      4 => 'E',
      5 => 'F',
      6 => 'G',
      7 => 'H',
      8 => 'I',
      9 => 'J',
      10 => 'K',
      11 => 'L',
      12 => 'M',
      13 => 'N',
      14 => 'O',
      15 => 'P',
      16 => 'Q',
      17 => 'R',
      18 => 'S',
      19 => 'T',
      20 => 'U',
      21 => 'V',
      22 => 'W',
      23 => 'X',
      24 => 'Y',
      25 => 'Z',
      26 => 'a',
      27 => 'b',
      28 => 'c',
      29 => 'd',
      30 => 'e',
      31 => 'f',
      32 => 'g',
      33 => 'h',
      34 => 'i',
      35 => 'j',
      36 => 'k',
      37 => 'l',
      38 => 'm',
      39 => 'n',
      40 => 'o',
      41 => 'p',
      42 => 'q',
      43 => 'r',
      44 => 's',
      45 => 't',
      46 => 'u',
      47 => 'v',
      48 => 'w',
      49 => 'x',
      50 => 'y',
      51 => 'z',
      52 => '0',
      53 => '1',
      54 => '2',
      55 => '3',
      56 => '4',
      57 => '5',
      58 => '6',
      59 => '7',
      60 => '8',
      61 => '9',
      62 => '+',
      63 => '/',

      _ => return Err("Invalid hex digit"),
    };

    result.push(symbol)
  }

  Ok(result)
}

fn each_six(bytes: Vec<u8>) -> Vec<u8> {
  let total_digits = (bytes.len() * 8) / 6;

  let mut values: Vec<u8> = Vec::new();

  for i in 0..total_digits {
    values.push(read_six(&bytes, i * 6));
  }

  values
}

fn read_six(bytes: &Vec<u8>, offset: usize) -> u8 {
  let mut value: u8 = 0;

  for index in offset..offset + 6 {
    let byte = bytes[index / 8];
    let bit = index % 8;
    value = (value << 1) + ((byte >> (7 - bit)) & 1);
  }

  value
}
