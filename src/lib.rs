#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![feature(external_doc)]
#![doc(include = "../README.md")]

#[macro_use]
extern crate failure;

mod header;

pub use header::*;

/// Create a new `Header` in the `Bitfield` configuration.
pub fn create_bitfield() -> Header {
  Header::new(FileType::BitField, 3328, HashType::None)
}

/// Create a new `Header` in the `Signatures` configuration.
pub fn create_signatures() -> Header {
  Header::new(FileType::Signatures, 64, HashType::Ed25519)
}

/// Create a new `Header` in the `Tree` configuration.
pub fn create_tree() -> Header {
  Header::new(FileType::Tree, 40, HashType::BLAKE2b)
}
