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
