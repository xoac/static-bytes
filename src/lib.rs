//! The aim of this crate is to improve user experience when working with static bytes.
//! Look at this pseudo code example to understand problem with `&mut [u8]` and `bytes::buf::BufMut`
//! ```compile_fail
//! let mut fixed_storage = [u8; 16];
//! let mut slice = fixed_storage[..];
//! let len_before = slice.len();
//! // declaration fn encode(&self, buf: &mut dyn BufMut);
//! frame.encode(&mut slice);
//! let len = len_before - slice.len();
//! let filled_bytes = fixed_storage[..len];
//! ```
//! There are two problems with code above:
//! - it will panic if encode want to use more than 16 bytes!
//! - it is boilerplate
//!
//! You can resolve both with `SafeBytesSlice`. For example usage see
//! [docs](https://docs.rs/static-bytes/0.2/static_bytes/struct.SafeBytesSlice.html).
//!
//! ## Compatibility with bytes
//! - v0.1.x is compatible with bytes >=0.5.0, <0.6.0
//! - v0.2.x is compatible with bytes >=0.6.0, <0.7.0
//!

#![deny(missing_docs)]
#![no_std]

mod bytes_slice;
pub mod error;
pub use bytes_slice::SafeBytesSlice;
