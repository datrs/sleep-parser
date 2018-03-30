extern crate sleep_parser as sp;

use sp::{FileType, HashType, Header};

#[test]
fn new_header() {
  sp::Header::new(FileType::Tree, 40, HashType::BLAKE2b);
}
