#![cfg_attr(feature = "cargo-clippy", allow(clippy))]

use header::*;
use nom::{be_u16, be_u8, rest};
use std::str;

pub(crate) const HEADER_LENGTH: usize = 32;
const VERIFY_TRAILING_ZEROS: bool = true;

named!(
  file_type<FileType>,
  switch!(be_u8,
    0 => value!(FileType::BitField) |
    1 => value!(FileType::Signatures) |
    2 => value!(FileType::Tree)
  )
);

named!(
  protocol_version<ProtocolVersion>,
  switch!(be_u8,
    0 => value!(ProtocolVersion::V0)
  )
);

named_args!(
  algorithm(len: u8)<HashType>,
  switch!(map_res!(take!(len), str::from_utf8),
    "BLAKE2b" => value!(HashType::BLAKE2b) |
    "Ed25519" => value!(HashType::Ed25519) |
    "" => value!(HashType::None)
  )
);

named!(
  pub header<Header>,
  flat_map!(
    take!(HEADER_LENGTH),
    do_parse!(
      tag!(b"\x05\x02\x57") >>
      file_type: file_type >>
      protocol_version: protocol_version >>
      entry_size: be_u16 >>

      algorithm_len: verify!(be_u8, |len: u8| len <= HEADER_LENGTH as u8 - 8) >>
      algorithm: apply!(algorithm, algorithm_len) >>

      verify!(rest, |bytes: &[u8]| {
        let header_consumed = bytes.len() + algorithm_len as usize + 8 == HEADER_LENGTH;
        let trailing_zeros = !VERIFY_TRAILING_ZEROS || bytes.iter().all(|&b| b == 0u8);
        header_consumed && trailing_zeros
      }) >>

      (Header {
        file_type,
        protocol_version,
        entry_size,
        hash_type: algorithm,
      })
    )
  )
);

#[cfg(test)]
mod test {
  use super::*;

  use nom;

  #[test]
  fn parse_file_type() {
    assert_eq!(
      file_type(b"\x00"),
      Ok((&[][..], FileType::BitField))
    );
    assert_eq!(
      file_type(b"\x01"),
      Ok((&[][..], FileType::Signatures))
    );
    assert_eq!(
      file_type(b"\x02"),
      Ok((&[][..], FileType::Tree))
    );
    assert!(file_type(b"\xff").is_err());
  }

  #[test]
  fn parse_header() {
    fn mk_header(prefix: &[u8]) -> [u8; 32] {
      let mut h = [0u8; 32];
      h[0..prefix.len()].clone_from_slice(prefix);
      h
    }

    assert_eq!(
      header(&mk_header(
        b"\x05\x02W\x01\x00\x00\x28\x07BLAKE2b"
      )),
      Ok((
        &[][..],
        Header {
          file_type: FileType::Signatures,
          protocol_version: ProtocolVersion::V0,
          entry_size: 40,
          hash_type: HashType::BLAKE2b
        }
      ))
    );
    assert_eq!(
      header(&mk_header(
        b"\x05\x02W\x01\x00\x00\x28\x07BLAKE2b"
      )).unwrap()
        .1
        .hash_type,
      HashType::BLAKE2b
    );
    assert_eq!(
      header(&mk_header(
        b"\x05\x02W\x01\x00\x00\x28\x07Ed25519"
      )).unwrap()
        .1
        .hash_type,
      HashType::Ed25519
    );
    assert_eq!(
      header(&mk_header(b"\x05\x02W\x01\x00\x00\x28\x00"))
        .unwrap()
        .1
        .hash_type,
      HashType::None
    );
    assert!(header(&mk_header(b"\x05\x02W\x01\x00\x00\x28\x01B")).is_err());
    assert!(header(&mk_header(b"\x05\x02W\x01\x00\x00\x28\x01B")).is_err());

    let h = b"\x05\x02W\x01\x00\x00\x28\x19BLAKE2bXXXXXXXXXXXXXXXXXX";
    assert!(header(h).is_err());
  }

  #[test]
  fn invalid_algorithm_len() {
    match header(b"\x05\x02W\x00\x00\x00\x00\xFF\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00") {
      Err(nom::Err::Error(nom::Context::Code(_, nom::ErrorKind::Verify))) => (),
      x => panic!("{:?}", x),
    }
  }
}
