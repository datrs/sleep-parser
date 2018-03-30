extern crate sleep_parser;

use sleep_parser::{FileType, HashType, Header};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[test]
fn new_header() {
  Header::new(FileType::Tree, 40, HashType::BLAKE2b);
}

#[test]
fn from_vec_content_bitfield() {
  let file = File::open("tests/fixtures/content.bitfield").unwrap();
  let mut reader = BufReader::with_capacity(32, file);
  let buffer = reader.fill_buf().unwrap();
  let header = Header::from_vec(&buffer).unwrap();
  assert!(header.is_bitfield());
}

#[test]
fn from_vec_content_signatures() {
  let file = File::open("tests/fixtures/content.signatures").unwrap();
  let mut reader = BufReader::with_capacity(32, file);
  let buffer = reader.fill_buf().unwrap();
  let header = Header::from_vec(&buffer).unwrap();
  assert!(header.is_signatures());
}

#[test]
fn from_vec_content_tree() {
  let file = File::open("tests/fixtures/content.tree").unwrap();
  let mut reader = BufReader::with_capacity(32, file);
  let buffer = reader.fill_buf().unwrap();
  let header = Header::from_vec(&buffer).unwrap();
  assert!(header.is_tree());
}

#[test]
fn from_vec_metadata_bitfield() {
  let file = File::open("tests/fixtures/metadata.bitfield").unwrap();
  let mut reader = BufReader::with_capacity(32, file);
  let buffer = reader.fill_buf().unwrap();
  let header = Header::from_vec(&buffer).unwrap();
  assert!(header.is_bitfield());
}

#[test]
fn from_vec_metadata_signatures() {
  let file = File::open("tests/fixtures/metadata.signatures").unwrap();
  let mut reader = BufReader::with_capacity(32, file);
  let buffer = reader.fill_buf().unwrap();
  let header = Header::from_vec(&buffer).unwrap();
  assert!(header.is_signatures());
}

#[test]
fn from_vec_metadata_tree() {
  let file = File::open("tests/fixtures/metadata.tree").unwrap();
  let mut reader = BufReader::with_capacity(32, file);
  let buffer = reader.fill_buf().unwrap();
  let header = Header::from_vec(&buffer).unwrap();
  assert!(header.is_tree());
}
