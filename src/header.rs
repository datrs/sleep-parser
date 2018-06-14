extern crate byteorder;

use self::byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use failure::Error;
use std::io::Cursor;

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
#[derive(Debug)]
pub enum ProtocolVersion {
  /// The version specified as per the paper released in 2017-09.
  V0,
}

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

const HEADER_LENGTH: usize = 32;
const MAX_ALGORITHM_NAME_LENGTH: usize = HEADER_LENGTH - 8;

/// According to https://github.com/datproject/docs/blob/master/papers/sleep.md trailing bytes
/// should be zeros, so garbage is probably fine too.
const VERIFY_TRAILING_ZEROS: bool = false;

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

  /// Parse a 32 byte buffer slice into a valid Header.
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

    if VERIFY_TRAILING_ZEROS {
      for index in rdr.position()..32 {
        let byte = rdr.read_u8().unwrap();
        ensure!(
          byte == 0,
          "The remainder of the header should be zero-filled. Found byte '{}' at position '{}'.",
          byte, index
        );
      }
    }

    Ok(Header {
      protocol_version,
      entry_size,
      file_type,
      hash_type,
    })
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
  pub fn is_bitfield(&self) -> bool {
    self.entry_size == 3328
      && self.file_type == FileType::BitField
      && self.hash_type == HashType::None
  }

  /// Check whether the header is formatted as a `.signatures`.
  pub fn is_signatures(&self) -> bool {
    self.entry_size == 64
      && self.file_type == FileType::Signatures
      && self.hash_type == HashType::Ed25519
  }

  /// Check whether the header is formatted as a `.tree`.
  pub fn is_tree(&self) -> bool {
    self.entry_size == 40
      && self.file_type == FileType::Tree
      && self.hash_type == HashType::BLAKE2b
  }
}
