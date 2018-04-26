extern crate sleep_parser;

use sleep_parser::*;

#[test]
fn bitfield() {
  assert!(create_bitfield().is_bitfield());
}

#[test]
fn signatures() {
  assert!(create_signatures().is_signatures());
}

#[test]
fn tree() {
  assert!(create_tree().is_tree());
}
