#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![feature(external_doc)]
#![doc(include = "../README.md")]

#[macro_use]
extern crate failure;

mod header;

pub use header::*;
