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

impl HashType {
  /// Returns true if the hash is `BLAKE2b`
  #[inline]
  pub fn is_blake2b(&self) -> bool {
    *self == HashType::BLAKE2b
  }

  /// Returns true if the hash is `Ed25519`
  #[inline]
  pub fn is_ed25519(&self) -> bool {
    *self == HashType::Ed25519
  }

  /// Returns true if no hash function was used.
  #[inline]
  pub fn is_none(&self) -> bool {
    *self == HashType::None
  }
}
