#![feature(test)]
extern crate test;

extern crate sleep_parser;

use sleep_parser::Header;
use test::Bencher;

const HEADER: &[u8; 32] = b"\x05\x02W\x01\x00\x00\x28\x07BLAKE2b\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

#[bench]
fn hand_rolled(b: &mut Bencher) {
  b.iter(|| Header::from_vec(HEADER));
}

#[bench]
fn nom(b: &mut Bencher) {
  b.iter(|| Header::from_bytes(HEADER));
}
