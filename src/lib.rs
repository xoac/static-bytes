//! The init state of this crate is to improve user experience when working with static bytes.
//! ```compile_fail
//! let len_before = slice.len();
//! frame.encode(&mut slice);
//! let len = len_before - slice.len();
//! ```
//! where: `slice` is `&mut [u8]` and definintion of encode `fn encode(&self, buf: &mut dyn BufMut)`
//!

#![deny(missing_docs)]
#![no_std]

mod bytes_slice;
pub use bytes_slice::SafeBytesSlice;
