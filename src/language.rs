pub fn score_string(input: &String) -> usize {
  input.chars().map(|ch| score_char(ch)).sum()
}

fn score_char(ch: char) -> usize {
  match ch {
    'e' | 't' | 'a' | 'o' | 'i' | 'n' | 's' | 'h' | 'r' | 'd' | 'l' | 'u' => 2,
    'E' | 'T' | 'A' | 'O' | 'I' | 'N' | 'S' | 'H' | 'R' | 'D' | 'L' | 'U' => 1,
    ' ' => 1,
    _ => 0
  }
}
