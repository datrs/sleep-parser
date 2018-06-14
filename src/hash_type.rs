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
