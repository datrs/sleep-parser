/// SLEEP Protocol version.
#[derive(Debug, PartialEq)]
pub enum ProtocolVersion {
  /// The version specified as per the paper released in 2017-09.
  V0,
}

impl ProtocolVersion {
  /// Returns true if the version is `V0`.
  #[inline]
  pub fn is_v0(&self) -> bool {
    *self == ProtocolVersion::V0
  }
}
