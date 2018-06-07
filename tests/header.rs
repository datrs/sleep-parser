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

#[test]
fn issue_3() {
  // https://github.com/datrs/sleep-parser/issues/3

  let data = b"\x05\x02W\x01\x00\xb0\xb0\xb0\xb0\xb0\xb0\xb0\xb0\xb0\xb0\xb0\xfb\x03p\xb0\xb0\xb0\xb0\xb0\xb0\xb0\xb0\xbb9\xb0\xf5\xf5";
  assert!(Header::from_vec(data).is_err());

  let data = b"\x05\x02W\x01\x00\x00\x00\x12\x12\x12\x00\x00S\xc3\xcf\x8a2\xcc\xd1\xce9\xc4K\x9343\x00602\xb5\x07";
  assert!(Header::from_vec(data).is_err());
}

#[test]
fn invalid_algorithm() {
  fn mk_header(prefix: &[u8]) -> [u8; 32] {
    let mut h = [0u8; 32];
    h[0..prefix.len()].clone_from_slice(prefix);
    h
  }

  assert!(
    Header::from_vec(&mk_header(b"\x05\x02W\x01\x00\x00\x28\x01B")).is_err()
  );
  assert!(
    Header::from_vec(&mk_header(b"\x05\x02W\x01\x00\x00\x28\x01B")).is_err()
  );
  assert!(
    Header::from_vec(b"\x05\x02W\x01\x00\x00\x28\x19BLAKE2bXXXXXXXXXXXXXXXXXX")
      .is_err()
  );
}
