#![forbid(unsafe_code, missing_debug_implementations, missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! ## Example
//! ```rust
//! extern crate sleep_parser as sleep_parser;
//!
//! use sleep_parser::{FileType, HashType, Header};
//! use std::fs::File;
//! use std::io::{BufRead, BufReader};
//!
//! let file = File::open("tests/fixtures/content.bitfield").unwrap();
//! let mut reader = BufReader::with_capacity(32, file);
//! let buffer = reader.fill_buf().unwrap();
//! let header = Header::from_vec(&buffer).unwrap();
//! assert!(header.is_bitfield());
//! ```

mod file_type;
mod hash_type;
mod protocol_version;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use failure::{bail, ensure, format_err, Error};
use std::io::Cursor;

pub use crate::file_type::FileType;
pub use crate::hash_type::HashType;
pub use crate::protocol_version::ProtocolVersion;

const HEADER_LENGTH: usize = 32;
const MAX_ALGORITHM_NAME_LENGTH: usize = HEADER_LENGTH - 8;

/// Structural representation of 32 byte SLEEP headers.
#[derive(Debug)]
pub struct Header {
  /// Type of file.
  pub file_type: FileType,
  /// Version of the SLEEP protocol.
  pub protocol_version: ProtocolVersion,
  /// Size of each piece of data in the file body.
  pub entry_size: u16,
  /// Algorithm used for hashing the content.
  pub hash_type: HashType,
}

impl Header {
  /// Create a new `Header`.
  #[inline]
  pub fn new(
    file_type: FileType,
    entry_size: u16,
    hash_type: HashType,
  ) -> Self {
    Header {
      file_type,
      entry_size,
      hash_type,
      protocol_version: ProtocolVersion::V0,
    }
  }

  /// Parse a 32 byte buffer slice into a valid Header.
  #[inline]
  pub fn from_vec(buffer: &[u8]) -> Result<Header, Error> {
    ensure!(buffer.len() == 32, "buffer should be 32 bytes");

    let mut rdr = Cursor::new(buffer);
    let byte = rdr.read_u8().unwrap();
    ensure!(
      byte == 5,
      "The first byte of a SLEEP header should be '5', found {}",
      byte
    );

    let byte = rdr.read_u8().unwrap();
    ensure!(
      byte == 2,
      "The second byte of a SLEEP header should be '2', found {}",
      byte
    );

    let byte = rdr.read_u8().unwrap();
    ensure!(
      byte == 87,
      "The third byte of a SLEEP header should be '87', found {}",
      byte
    );

    let file_type = match rdr.read_u8().unwrap() {
      0 => FileType::BitField,
      1 => FileType::Signatures,
      2 => FileType::Tree,
      num => bail!(
        "The fourth byte '{}' does not belong to any known SLEEP file type",
        num
      ),
    };

    let protocol_version = match rdr.read_u8().unwrap() {
      0 => ProtocolVersion::V0,
      num => bail!(
        "The fifth byte '{}' does not belong to any known SLEEP protocol protocol_version",
        num
      ),
    };

    // Read entry size which will inform how many bytes to read next.
    let entry_size = rdr.read_u16::<BigEndian>().unwrap();

    // Read out the "entry_size" bytes into a string.
    // NOTE(yw): there should be a more concise way of doing this.
    let hash_name_len = rdr.read_u8().unwrap() as usize;
    let current = rdr.position() as usize;

    ensure!(
      hash_name_len <= MAX_ALGORITHM_NAME_LENGTH,
      "Algorithm name is too long: {} (max: {})",
      hash_name_len,
      MAX_ALGORITHM_NAME_LENGTH
    );

    let hash_name_upper = current + hash_name_len;
    ensure!(
      buffer.len() >= hash_name_upper,
      "Broken parser: algorithm name is out of bounds: {} {}",
      hash_name_upper,
      buffer.len()
    );

    let buf_slice = &buffer[current..hash_name_upper];
    rdr.set_position(hash_name_upper as u64 + 1);
    let algo = ::std::str::from_utf8(buf_slice).map_err(|e| {
      format_err!("The algorithm string was invalid utf8 encoded: {:?}", e)
    })?;

    let hash_type = match algo {
      "BLAKE2b" => HashType::BLAKE2b,
      "Ed25519" => HashType::Ed25519,
      "" => HashType::None,
      name => bail!("Unexpected algorithm name: {}", name),
    };

    Ok(Header {
      protocol_version,
      entry_size,
      file_type,
      hash_type,
    })
  }

  /// Convert a `Header` into a `Vec<u8>`. Use this to persist a header back to
  /// disk.
  #[inline]
  pub fn to_vec(&self) -> Vec<u8> {
    let mut wtr = Vec::with_capacity(32);

    wtr.extend_from_slice(&[5u8, 2, 87]);

    let file_type = match self.file_type {
      FileType::BitField => 0,
      FileType::Signatures => 1,
      FileType::Tree => 2,
    };
    wtr.write_u8(file_type).unwrap();

    let protocol_version = match self.protocol_version {
      ProtocolVersion::V0 => 0,
    };
    wtr.write_u8(protocol_version).unwrap();

    wtr.write_u16::<BigEndian>(self.entry_size).unwrap();

    let hash_type = match self.hash_type {
      HashType::BLAKE2b => "BLAKE2b",
      HashType::Ed25519 => "Ed25519",
      HashType::None => "",
    };
    let hash_type = hash_type.as_bytes();
    wtr.write_u8(hash_type.len() as u8).unwrap();
    wtr.extend_from_slice(hash_type);

    for _ in wtr.len()..wtr.capacity() {
      wtr.write_u8(0).unwrap();
    }
    wtr
  }

  /// Check whether the header is formatted as a `.bitfield`.
  #[inline]
  pub fn is_bitfield(&self) -> bool {
    self.entry_size == 3328
      && self.file_type.is_bitfield()
      && self.hash_type.is_none()
  }

  /// Check whether the header is formatted as a `.signatures`.
  #[inline]
  pub fn is_signatures(&self) -> bool {
    self.entry_size == 64
      && self.file_type.is_signatures()
      && self.hash_type.is_ed25519()
  }

  /// Check whether the header is formatted as a `.tree`.
  #[inline]
  pub fn is_tree(&self) -> bool {
    self.entry_size == 40
      && self.file_type.is_tree()
      && self.hash_type.is_blake2b()
  }
}

/// Create a new `Header` in the `Bitfield` configuration.
#[inline]
pub fn create_bitfield() -> Header {
  Header::new(FileType::BitField, 3328, HashType::None)
}

/// Create a new `Header` in the `Signatures` configuration.
#[inline]
pub fn create_signatures() -> Header {
  Header::new(FileType::Signatures, 64, HashType::Ed25519)
}

/// Create a new `Header` in the `Tree` configuration.
#[inline]
pub fn create_tree() -> Header {
  Header::new(FileType::Tree, 40, HashType::BLAKE2b)
}
