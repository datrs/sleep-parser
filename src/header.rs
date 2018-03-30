use failure::Error;

/// Algorithm used for hashing the data.
#[derive(Debug)]
pub enum HashAlgorithm {
  /// [BLAKE2b](https://blake2.net/) hashing algorithm.
  BLAKE2b,
  /// [Ed25519](https://ed25519.cr.yp.to/) hashing algorithm.
  Ed25519,
}

/// Type of file.
///
/// `signatures`, `bitfield` and `tree` are the three SLEEP files. There are two
///additional files, `key`, and `data`, which do not contain SLEEP file headers
///and store plain serialized data for easy access. `key` stores the public key
///that is described by the `signatures` file, and `data` stores the raw chunk
///data that the `tree` file contains the hashes and metadata.
#[derive(Debug)]
pub enum FileType {
  /// The bitfield describes which pieces of data you have, and which nodes in
  /// the tree file have been written.  This file exists as an index of the tree
  /// and data to quickly figure out which pieces of data you have or are
  /// missing. This file can be regenerated if you delete it, so it is
  /// considered a materialized index.
  Bitfield,
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
  pub hash_algorithm: HashAlgorithm,
}

impl Header {
  /// Create a new `Header`.
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
      0 => FileType::Bitfield,
      1 => FileType::Signatures,
      2 => FileType::Tree,
      num => bail!(format!(
        "The byte '{}' does not belong to any known SLEEP file type",
        num
      )),
    };

    let protocol_version = match buffer[4] {
      0 => ProtocolVersion::V0,
      num => bail!(format!(
        "The byte '{}' does not belong to any known SLEEP protocol protocol_version",
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
      protocol_version: protocol_version,
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
