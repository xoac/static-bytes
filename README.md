[![crates.io](https://img.shields.io/crates/v/static-bytes.svg)](https://crates.io/crates/static-bytes)
[![Documentation](https://docs.rs/static-bytes/badge.svg)](https://docs.rs/static-bytes/)
[![CI master](https://github.com/xoac/static-bytes/workflows/Continuous%20integration/badge.svg?branch=master)](https://github.com/xoac/static-bytes/actions?query=workflow%3A%22Continuous+integration%22)
![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)

# static-bytes

The aim of this crate is to improve user experience when working with static bytes.
Look at this pseudo code example to understand problem with `&mut [u8]` and `bytes::buf::BufMut`
```compile_fail
let mut fixed_storage = [u8; 16];
let mut slice = fixed_storage[..];
let len_before = slice.len();
// declaration fn encode(&self, buf: &mut dyn BufMut);
frame.encode(&mut slice);
let len = len_before - slice.len();
let filled_bytes = fixed_storage[..len];
```
There are two problems with code above:
- it will panic if encode want to use more than 16 bytes!
- it is boilerplate

You can resolve both with `SafeBytesSlice`. For example usage see
[docs](https://docs.rs/static-bytes/0.2/static_bytes/struct.SafeBytesSlice.html).

### Compatibility with bytes
- v0.1.x is compatible with bytes >=0.5.0, <0.6.0
- v0.2.x is compatible with bytes >=0.6.0, <0.7.0
- v0.3.x is compatible with bytes >=0.1.0, <2.0.0


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

This project try follow rules:
* [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
* [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

_This README was generated with [cargo-readme](https://github.com/livioribeiro/cargo-readme) from [template](https://github.com/xoac/crates-io-lib-template)_
