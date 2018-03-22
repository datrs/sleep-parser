#![deny(warnings, missing_docs)]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

//! Parse [Dat protocol SLEEP
//! files](https://github.com/datproject/docs/blob/master/papers/sleep.md).

extern crate nom;
