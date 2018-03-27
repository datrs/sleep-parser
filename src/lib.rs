// #![deny(warnings, missing_docs)]
// #![cfg_attr(test, feature(plugin))]
// #![cfg_attr(test, plugin(clippy))]

//! Parse [Dat protocol SLEEP
//! files](https://github.com/datproject/docs/blob/master/papers/sleep.md).
//!
//! ## Format
//!
//! ```txt,ignore
//! <32 byte header>
//!   <4 byte magic string: 0x05025702>
//!   <1 byte version number: 0>
//!   <2 byte entry size: 40>
//!   <1 byte algorithm name length prefix: 7>
//!   <7 byte algorithm name: BLAKE2b>
//!   <17 zeroes>
//! <40 byte entries>
//!   <32 byte BLAKE2b hash>
//!   <8 byte Uint64BE children leaf byte length>
//! ```

#[macro_use]
extern crate failure;

use failure::Error;

/// Algorithm used for hashing the data.
pub enum HashAlgorithm {
  /// [BLAKE2b](https://blake2.net/) hashing algorithm.
  BLAKE2b,
  /// [Ed25519](https://ed25519.cr.yp.to/) hashing algorithm.
  Ed25519,
}

/// Type of file.
pub enum FileType {
  BitField,
  Signatures,
  Tree,
}

/// SLEEP Protocol version.
pub enum Version {
  V0,
}

/// Struct representation of 32 byte SLEEP headers.
pub struct Header {
  pub file_type: FileType,
  pub version: Version,
  pub entry_size: u16,
  pub hash_algorithm: HashAlgorithm,
}

impl Header {
  pub fn new(
    _tree_type: FileType,
    _entry_size: u16,
    _hash_algorithm: HashAlgorithm,
  ) {
  }

  /// Parse a 32 bit buffer slice into a valid Header.
  pub fn from_vec(buffer: &[u8]) -> Result<Header, Error> {
    ensure!(
      buffer.len() == 32,
      "buffer should be at least 32 bytes"
    );
    ensure!(
      buffer[0] == 5,
      "The first byte of a SLEEP header should be '5' (hex '0x05')"
    );
    ensure!(
      buffer[1] == 2,
      "The second byte of a SLEEP header should be '2' (hex '0x02')"
    );
    ensure!(
      buffer[2] == 87,
      "The third byte of a SLEEP header should be '87' (hex '0x57')"
    );

    let file_type = match buffer[3] {
      0 => FileType::BitField,
      1 => FileType::Signatures,
      2 => FileType::Tree,
      num => bail!(format!(
        "The byte '{}' does not belong to any known SLEEP file type",
        num
      )),
    };

    let version = match buffer[4] {
      0 => Version::V0,
      num => bail!(format!(
        "The byte '{}' does not belong to any known SLEEP protocol version",
        num
      )),
    };

    let entry_size = buffer[5] as u16 + (buffer[6] << 16) as u16;

    let hash_name_len = buffer[7] as usize;
    let hash_name_upper = 8 + hash_name_len;
    let buf_slice = &buffer[8..hash_name_upper];
    let algo = std::str::from_utf8(&buf_slice)
      .expect("The algorithm string was invalid utf8 encoded");

    let hash_algorithm = match algo {
      "BLAKE2b" => HashAlgorithm::BLAKE2b,
      "Ed25519" => HashAlgorithm::Ed25519,
      _ => bail!(format!("The byte sequence '{:?}' does not belong to any known SLEEP hashing algorithm.", &buf_slice)),
    };

    for (index, byte) in (hash_name_upper..32).enumerate() {
      ensure!(byte == 0, format!("The remainder of the header should be zero-filled. Found byte {} at position {}.", byte, index));
    }

    Ok(Header {
      version: version,
      entry_size: entry_size,
      file_type: file_type,
      hash_algorithm: hash_algorithm,
    })
  }

  /// Convert a `Header` into a `Vec<u8>`. Use this to persist a header back to
  /// disk.
  pub fn to_vec(&self) {}
}

#[test]
fn test() {
  use std::fs::File;
  use std::io::{BufRead, BufReader};

  let file = File::open("README.md").unwrap();
  let mut reader = BufReader::with_capacity(40, file);
  let buffer = reader.fill_buf().unwrap();
  println!("{:?}", buffer.len());
}
