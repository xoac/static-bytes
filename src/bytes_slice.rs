use super::error::CapacityExceeded;
use bytes::buf::UninitSlice;
use bytes::BufMut;
use core::mem;
use core::mem::MaybeUninit;

/// Non panic wrapper for `&mut [u8]` that implement [`BufMut`](bytes::buf::BufMut) trait.
///
/// # Example
/// ### work with uninitialized memory `&mut [MaybeUninit<u8>]`
/// ```rust
/// use bytes::buf::BufMut;
/// use static_bytes::SafeBytesSlice;
/// use core::mem::MaybeUninit;
/// //are you sure that's random?
/// fn fill_with_random(buf: &mut dyn BufMut, amount: usize) {
///     for _ in 0..amount {
///         buf.put_u8(9);
///     }
/// }
/// // This is optimized way of working with slices
/// // This is safe. See way in rust doc:
/// // https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
/// let mut fixed_storage: [MaybeUninit<u8>; 128] = unsafe {
///        MaybeUninit::uninit().assume_init()
///    };
/// let mut slice = SafeBytesSlice::from(&mut fixed_storage[..]);
/// // your function that accept `&mut dyn BufMut`
/// fill_with_random(&mut slice, 32);
/// let output = slice.try_into_bytes().unwrap();
/// assert_eq!(output.len(), 32);
/// assert_eq!(output[31], 9);
/// ```
///
/// ### work with standard slice `&mut [u8]`
/// ```rust
/// use bytes::buf::BufMut;
/// use static_bytes::SafeBytesSlice;
/// # //are you sure that's random?
/// # fn fill_with_random(buf: &mut dyn BufMut, amount: usize) {
/// #    for _ in 0..amount {
/// #        buf.put_u8(9);
/// #    }
/// # }
///
/// let mut fixed_storage = [0u8; 64];
/// let mut slice = SafeBytesSlice::from(&mut fixed_storage[..]);
/// // your function that accept `&mut dyn BufMut`
/// // see function impl in `&mut [MaybeUninit<u8>]` example
/// fill_with_random(&mut slice, 32);
/// let output = slice.try_into_bytes().unwrap();
/// assert_eq!(output.len(), 32);
/// assert_eq!(output[31], 9);
/// ```
///
/// Is `fill_with_random()` random?
/// ![](https://starecat.com/content/wp-content/uploads/tour-of-accounting-over-here-we-have-our-random-number-generator-nine-nine-are-you-sure-thats-random-thats-the-problem-with-randomness-you-can-never-be-sure-gilbert-comic.jpg)
pub struct SafeBytesSlice<'a> {
    slice: &'a mut [MaybeUninit<u8>],
    bytes_written: usize,
    // capacity exceeded. User tried put more bytes than available in slice
    cap_exceeded: bool,
}

impl<'a> From<&'a mut [u8]> for SafeBytesSlice<'a> {
    fn from(slice: &'a mut [u8]) -> Self {
        let maybe_uninit_slice =
            unsafe { &mut *(&mut *slice as *mut [u8] as *mut [mem::MaybeUninit<u8>]) };

        Self::from(maybe_uninit_slice)
    }
}

impl<'a> From<&'a mut [MaybeUninit<u8>]> for SafeBytesSlice<'a> {
    fn from(slice: &'a mut [MaybeUninit<u8>]) -> Self {
        Self {
            slice,
            bytes_written: 0,
            cap_exceeded: false,
        }
    }
}

impl<'a> SafeBytesSlice<'a> {
    /// Returns the number of bytes wrote into inner slice.
    ///
    /// Use [`BufMut::remaining_mut()`](bytes::buf::BufMut::remaining_mut) to check the number of bytes that
    /// can be written from the current position until the end of the buffer is reached.
    ///
    /// # Example
    ///
    /// This is example of BAD usage. Showing why this method is private and probably should never
    /// be public.
    ///
    /// [Tracking Issue 1](https://github.com/xoac/static-bytes/issues/1)
    ///
    /// ```compile_fail
    /// use static_bytes::SafeBytesSlice;
    /// use bytes::buf::BufMut;
    /// # fn fill_with_random(buf: &mut dyn BufMut, amount: usize) {
    /// #    for _ in 0..amount {
    /// #        buf.put_u8(0xFF);
    /// #    }
    /// # }
    /// let mut static_data = [0u8; 32];
    /// let mut safe_slice = SafeBytesSlice::from(&mut static_data[..]);
    ///
    /// fill_with_random(&mut safe_slice, 33);
    /// assert_eq!(safe_slice.is_exceed(), true);
    /// let output_len = safe_slice.bytes_written();
    /// // ⚠️ This  allow use  output_len in improper way!
    /// drop(safe_slice);
    ///
    /// // output is incorrect contains 32 bytes but should 33.
    /// let _output = &static_data[..output_len];
    /// ```
    #[allow(dead_code)]
    fn bytes_written(&self) -> usize {
        debug_assert_eq!(self.cap_exceeded, false);
        self.bytes_written
    }

    /// Returns true if the inner slice contains 0 bytes.
    #[inline]
    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.bytes_written() == 0
    }

    /// Returns filled bytes (`&[u8]`) or error if capacity exceeded.
    pub fn try_into_bytes(self) -> Result<&'a [u8], CapacityExceeded> {
        if self.is_exceed() {
            Err(CapacityExceeded {})
        } else {
            // TODO in the future there will be function for this
            // https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#method.slice_get_ref
            //
            // Safty this is safe because memory is properly initialized up to self.bytes_written
            // MaybeUninit<T> is #[repr(transparent)]
            Ok(unsafe {
                &*(&self.slice[..self.bytes_written] as *const [core::mem::MaybeUninit<u8>]
                    as *const [u8])
            })
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
unsafe impl<'a> BufMut for SafeBytesSlice<'a> {
    fn remaining_mut(&self) -> usize {
        debug_assert!(self.bytes_written <= self.slice.len());
        self.slice.len() - self.bytes_written
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        let new_bytes_written = self.bytes_written + cnt;
        if new_bytes_written > self.slice.len() {
            self.bytes_written = self.slice.len(); // make `remaining_mut()` return 0
            self.cap_exceeded = true;
        } else {
            self.bytes_written = new_bytes_written;
        }
    }

    fn bytes_mut(&mut self) -> &mut UninitSlice {
        let bytes = &mut self.slice[self.bytes_written..];
        let len = bytes.len();
        let ptr = bytes.as_mut_ptr() as *mut _;
        unsafe { return UninitSlice::from_raw_parts_mut(ptr, len) }
    }

    fn put_slice(&mut self, src: &[u8]) {
        use core::ptr;
        // check if we have enough data to put slice. If no set flag instead of panic!
        let src_len = src.len();
        if self.remaining_mut() < src_len {
            self.bytes_written = self.slice.len(); // make `remaining_mut()` return 0
            self.cap_exceeded = true;
            return;
        }

        // enough inner capacity to execute safe copy
        unsafe {
            let dst = self.bytes_mut();
            ptr::copy_nonoverlapping(src[..].as_ptr(), dst.as_mut_ptr() as *mut u8, src_len);
            self.advance_mut(src_len);
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
        let mut slice = SafeBytesSlice::from(&mut raw[..]);
        fill_with_random(&mut slice, 27);
        // slice already contains bytes_written();
        let _wrote_data = match slice.try_into_bytes() {
            Ok(bytes) => bytes,
            Err(_err) => unimplemented!(),
        };
    }

    #[test]
    fn naive_test() {
        let mut static_data = [0u8; 32];
        let mut safe_slice = SafeBytesSlice::from(&mut static_data[..]);

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
        let mut safe_slice = SafeBytesSlice::from(&mut static_data[..]);
        fill_with_random(&mut safe_slice, 33);
        assert_eq!(safe_slice.is_exceed(), true);
    }
}
