use bytes::BufMut;
use core::mem;

/// Error that indicates that capacity was exceeded
#[derive(Debug, Clone, Copy, Hash)]
pub struct CapacityExceeded;

/// Non panic wrapper for `&mut [u8]` that implement BufMut.
pub struct SafeBytesSlice<'a> {
    slice: &'a mut [u8],
    len: usize,
    // capacity exceeded. User tried put more bytes than available in slice
    cap_exceeded: bool,
}

impl<'a> SafeBytesSlice<'a> {
    /// Create new instance of [`SafeBytesSlice`](SafeBytesSlice)
    pub fn new(slice: &'a mut [u8]) -> Self {
        Self {
            slice,
            len: 0,
            cap_exceeded: false,
        }
    }

    /// Returns the number of bytes wrote into inner slice.
    ///
    /// Use [`BufMut::remaining_mut()`](bytes::buf::BufMut::remaining_mut) to check the number of bytes that
    /// can be written from the current position until the end of the buffer is reached.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns filled bytes (`&[u8]`) or error if capacity exceeded.
    pub fn try_into_bytes(self) -> Result<&'a [u8], CapacityExceeded> {
        if self.is_exceed() {
            Err(CapacityExceeded {})
        } else {
            Ok(&self.slice[..self.len])
        }
    }

    /// Returns true if capacity was exceeded.
    ///
    /// `SafeBytesSlice` is not usable anymore - there is no access to inner bytes because they are
    /// in improper state.
    pub fn is_exceed(&self) -> bool {
        self.cap_exceeded
    }
}

// Implement required methods
impl<'a> BufMut for SafeBytesSlice<'a> {
    fn remaining_mut(&self) -> usize {
        debug_assert!(self.len <= self.slice.len());
        self.slice.len() - self.len
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        let new_len = self.len + cnt;
        if new_len > self.slice.len() {
            self.len = self.slice.len(); // make `remaining_mut()` return 0
            self.cap_exceeded = true;
        } else {
            self.len = new_len;
        }
    }

    fn bytes_mut(&mut self) -> &mut [mem::MaybeUninit<u8>] {
        // taken from: https://github.com/tokio-rs/bytes/blob/6fdb7391ce83dc71ccaeda6a54211ce723e5d9a5/src/buf/buf_mut.rs#L981

        // MaybeUninit is repr(transparent), so safe to transmute
        let part_slice = &mut self.slice[self.len..];
        unsafe { mem::transmute(&mut *part_slice) }
    }

    fn put_slice(&mut self, src: &[u8]) {
        use core::ptr;
        // check if we have enough data to put slice. If no set flag instead of panic!
        if self.remaining_mut() < src.len() {
            self.cap_exceeded = true;
            return;
        }

        unsafe {
            let dst = self.bytes_mut();
            ptr::copy_nonoverlapping(src[..].as_ptr(), dst.as_mut_ptr() as *mut u8, src.len());
            self.advance_mut(src.len());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fill_with_random(buf: &mut dyn BufMut, amount: usize) {
        for _ in 0..amount {
            buf.put_u8(0xFF);
        }
    }

    #[test]
    fn usefullness() {
        // standard way
        let mut data = [0u8; 32];
        let mut slice = &mut data[..];
        let slice_len = slice.len();
        fill_with_random(&mut slice, 27);
        // how many data was wrote (?)
        let n = slice_len - slice.len();
        assert_eq!(n, 27);
        let _wrote_data = &data[..n];

        // new way
        let mut raw = [0u8; 32];
        let mut slice = SafeBytesSlice::new(&mut raw);
        fill_with_random(&mut slice, 27);
        // slice already contains len();
        let _wrote_data = match slice.try_into_bytes() {
            Ok(bytes) => bytes,
            Err(_err) => unimplemented!(),
        };
    }

    #[test]
    fn naive_test() {
        let mut static_data = [0u8; 32];
        let mut safe_slice = SafeBytesSlice::new(&mut static_data);

        fill_with_random(&mut safe_slice, 32);
        assert_eq!(safe_slice.is_exceed(), false);

        for v in safe_slice
            .try_into_bytes()
            .expect("not expected capacity")
            .iter()
        {
            assert_eq!(*v, 0xFF);
        }

        // reuse data
        let mut safe_slice = SafeBytesSlice::new(&mut static_data);
        fill_with_random(&mut safe_slice, 33);
        assert_eq!(safe_slice.is_exceed(), true);
    }
}
