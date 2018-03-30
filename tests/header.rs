extern crate sleep_parser as sp;

use sp::{FileType, HashType, Header};

#[test]
fn new_header() {
  sp::Header::new(FileType::Tree, 40, HashType::BLAKE2b);
}

#[test]
fn from_vec_as_bitfield() {
  use std::fs::File;
  use std::io::{BufRead, BufReader};

  let file = File::open("tests/fixtures/content.bitfield").unwrap();
  let mut reader = BufReader::with_capacity(32, file);
  let buffer = reader.fill_buf().unwrap();
  let header = sp::Header::from_vec(&buffer).unwrap();
  assert!(header.is_bitfield());
}
