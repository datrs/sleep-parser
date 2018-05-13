extern crate sleep_parser;

use sleep_parser::*;
use std::fs::File;
use std::io::Read;

#[test]
fn new_header() {
  Header::new(FileType::Tree, 40, HashType::BLAKE2b);
}

fn read_header_bytes(file_name: &str) -> Result<[u8; 32], std::io::Error> {
  let mut file = File::open(file_name)?;
  let mut buffer = [0u8; 32];
  file.read_exact(&mut buffer).map(|_| buffer)
}

#[test]
fn from_vec_content_bitfield() {
  let buffer = read_header_bytes("tests/fixtures/content.bitfield").unwrap();
  let header = Header::from_vec(&buffer).unwrap();
  assert!(header.is_bitfield());
  assert_eq!(header.to_vec(), buffer);
}

#[test]
fn from_vec_content_signatures() {
  let buffer = read_header_bytes("tests/fixtures/content.signatures").unwrap();
  let header = Header::from_vec(&buffer).unwrap();
  assert!(header.is_signatures());
  assert_eq!(header.to_vec(), buffer);
}

#[test]
fn from_vec_content_tree() {
  let buffer = read_header_bytes("tests/fixtures/content.tree").unwrap();
  let header = Header::from_vec(&buffer).unwrap();
  assert!(header.is_tree());
  assert_eq!(header.to_vec(), buffer);
}

#[test]
fn from_vec_metadata_bitfield() {
  let buffer = read_header_bytes("tests/fixtures/metadata.bitfield").unwrap();
  let header = Header::from_vec(&buffer).unwrap();
  assert!(header.is_bitfield());
  assert_eq!(header.to_vec(), buffer);
}

#[test]
fn from_vec_metadata_signatures() {
  let buffer = read_header_bytes("tests/fixtures/metadata.signatures").unwrap();
  let header = Header::from_vec(&buffer).unwrap();
  assert!(header.is_signatures());
  assert_eq!(header.to_vec(), buffer);
}

#[test]
fn from_vec_metadata_tree() {
  let buffer = read_header_bytes("tests/fixtures/metadata.tree").unwrap();
  let header = Header::from_vec(&buffer).unwrap();
  assert!(header.is_tree());
  assert_eq!(header.to_vec(), buffer);
}

#[test]
fn to_vec() {
  let header = Header::new(FileType::Tree, 40, HashType::BLAKE2b);
  assert_eq!(
    header.to_vec(),
    vec![
      5, 2, 87, 2, 0, 0, 40, 7, 66, 76, 65, 75, 69, 50, 98, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]
  );
}
