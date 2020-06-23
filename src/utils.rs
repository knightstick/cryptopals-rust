use crate::bytes;
use crate::conversion;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::path::PathBuf;

pub fn bytes_from_file(filename: PathBuf) -> Result<Vec<bytes::Bytes>, &'static str> {
  let mut result: Vec<bytes::Bytes> = Vec::new();

  match read_lines(filename) {
    Ok(lines) => {
      for line in lines {
        if let Ok(hex) = line {
          if let Ok(bytes) = conversion::hex_to_bytes(&hex) {
            result.push(bytes);
          }
        }
      }
    }

    Err(_err) => return Err("Could not read the file"),
  }

  Ok(result)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
