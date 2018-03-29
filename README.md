# sleep-parser
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Parse [Dat protocol SLEEP
files](https://github.com/datproject/docs/blob/master/papers/sleep.md).

- [Documentation][8]
- [Crates.io][2]

## Format

```txt,ignore
<32 byte header>
  <4 byte magic string: 0x05025702>
  <1 byte version number: 0>
  <2 byte entry size: 40>
  <1 byte algorithm name length prefix: 7>
  <7 byte algorithm name: BLAKE2b>
  <17 zeroes>
<40 byte entries>
  <32 byte BLAKE2b hash>
  <8 byte Uint64BE children leaf byte length>
```

## Installation
```sh
$ cargo add sleep-parser
```

## License
[Apache-2.0](./LICENSE)

[1]: https://img.shields.io/crates/v/sleep-parser.svg?style=flat-square
[2]: https://crates.io/crates/sleep-parser
[3]: https://img.shields.io/travis/datrs/sleep-parser.svg?style=flat-square
[4]: https://travis-ci.org/datrs/sleep-parser
[5]: https://img.shields.io/crates/d/sleep-parser.svg?style=flat-square
[6]: https://crates.io/crates/sleep-parser
[7]: https://docs.rs/sleep-parser/badge.svg
[8]: https://docs.rs/sleep-parser
