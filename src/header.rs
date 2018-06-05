extern crate byteorder;

use self::byteorder::{BigEndian, WriteBytesExt};
use failure::Error;
use nom;
use parsers;

/// Algorithm used for hashing the data.
#[derive(Debug, PartialEq)]
pub enum HashType {
  /// [BLAKE2b](https://blake2.net/) hashing algorithm.
  BLAKE2b,
  /// [Ed25519](https://ed25519.cr.yp.to/) hashing algorithm.
  Ed25519,
  /// No hashing used.
  None,
}

/// Type of file.
///
/// `signatures`, `bitfield` and `tree` are the three SLEEP files. There are two
///additional files, `key`, and `data`, which do not contain SLEEP file headers
///and store plain serialized data for easy access. `key` stores the public key
///that is described by the `signatures` file, and `data` stores the raw chunk
///data that the `tree` file contains the hashes and metadata.
#[derive(Debug, PartialEq)]
pub enum FileType {
  /// The bitfield describes which pieces of data you have, and which nodes in
  /// the tree file have been written.  This file exists as an index of the tree
  /// and data to quickly figure out which pieces of data you have or are
  /// missing. This file can be regenerated if you delete it, so it is
  /// considered a materialized index.
  BitField,
  /// A SLEEP formatted 32 byte header with data entries being 64 byte
  /// signatures.
  Signatures,
  /// A SLEEP formatted 32 byte header with data entries representing a
  /// serialized Merkle tree based on the data in the data storage layer. All
  /// the fixed size nodes written in in-order tree notation. The header
  /// algorithm string for `tree` files is `BLAKE2b`. The entry size is 40
  /// bytes.
  Tree,
}

/// SLEEP Protocol version.
#[derive(Debug, PartialEq)]
pub enum ProtocolVersion {
  /// The version specified as per the paper released in 2017-09.
  V0,
}

/// Structural representation of 32 byte SLEEP headers.
#[derive(Debug, PartialEq)]
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

  /// Parses a 32 byte buffer slice into a valid Header.
  pub fn from_bytes(buf: &[u8]) -> Result<Header, Error> {
    convert_nom_result(buf, parsers::header(buf))
  }

  /// Parse a 32 byte buffer slice into a valid Header.
  #[deprecated(note = "Use from_bytes")]
  pub fn from_vec(buffer: &[u8]) -> Result<Header, Error> {
    Header::from_bytes(buffer)
  }

  /// Convert a `Header` into a `Vec<u8>`. Use this to persist a header back to
  /// disk.
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

    wtr
      .write_u16::<BigEndian>(self.entry_size)
      .unwrap();

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
  pub fn is_bitfield(&self) -> bool {
    self.entry_size == 3328 && self.file_type == FileType::BitField
      && self.hash_type == HashType::None
  }

  /// Check whether the header is formatted as a `.signatures`.
  pub fn is_signatures(&self) -> bool {
    self.entry_size == 64 && self.file_type == FileType::Signatures
      && self.hash_type == HashType::Ed25519
  }

  /// Check whether the header is formatted as a `.tree`.
  pub fn is_tree(&self) -> bool {
    self.entry_size == 40 && self.file_type == FileType::Tree
      && self.hash_type == HashType::BLAKE2b
  }
}

fn convert_nom_result(
  buf: &[u8],
  result: Result<(&[u8], Header), nom::Err<&[u8]>>,
) -> Result<Header, Error> {
  match result {
    Ok((&[], h)) => Ok(h),
    Ok((remaining, _)) => {
      assert!(
        buf.len() > parsers::HEADER_LENGTH,
        "broken parser: input length is {}, but got unparsed input of length {}",
        buf.len(),
        remaining.len()
      );
      Err(format_err!("input must be {} bytes", parsers::HEADER_LENGTH))
    }
    Err(e @ nom::Err::Incomplete(_)) => {
      assert!(
        buf.len() < parsers::HEADER_LENGTH,
        "broken parser: input length is {}, but got error: {:?}",
        buf.len(),
        e
      );
      Err(format_err!("input must be {} bytes", parsers::HEADER_LENGTH))
    }
    Err(nom::Err::Error(context)) => {
      Err(format_err!("nom error: {:?}", context.into_error_kind()))
    }
    Err(nom::Err::Failure(context)) => {
      Err(format_err!("nom failure: {:?}", context.into_error_kind()))
    }
  }
}
