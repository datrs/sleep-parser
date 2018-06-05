#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate sleep_parser;

fuzz_target!(|data: &[u8]| {
  if let Ok(header) = sleep_parser::Header::from_vec(data) {
    sleep_parser::Header::from_vec(&header.to_vec()).unwrap();
  }
});
