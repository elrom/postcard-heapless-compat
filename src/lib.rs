#![no_std]

use heapless::Vec;
use heapless_vec::HVec;
use postcard::serialize_with_flavor;
use postcard::{Error, Result};
use serde::Serialize;

/// Serialize a `T` to a `heapless::Vec<u8>`, with the `Vec` containing
/// data in a serialized format.
///
pub fn to_vec<T, const B: usize>(value: &T) -> Result<Vec<u8, B>>
where
    T: Serialize + ?Sized,
{
    serialize_with_flavor::<T, HVec<B>, Vec<u8, B>>(value, HVec::default())
}

mod heapless_vec {
    use crate::{Error, Result};
    use core::ops::Index;
    use core::ops::IndexMut;
    use postcard::ser_flavors::Flavor;

    use heapless::Vec;

    ////////////////////////////////////////
    // HVec
    ////////////////////////////////////////

    /// The `HVec` flavor is a wrapper type around a `heapless::Vec`. This is a stack
    /// allocated data structure, with a fixed maximum size and variable amount of contents.
    #[derive(Default)]
    pub struct HVec<const B: usize> {
        /// the contained data buffer
        vec: Vec<u8, B>,
    }

    impl<const B: usize> Flavor for HVec<B> {
        type Output = Vec<u8, B>;

        #[inline(always)]
        fn try_extend(&mut self, data: &[u8]) -> Result<()> {
            self.vec
                .extend_from_slice(data)
                .map_err(|_| Error::SerializeBufferFull)
        }

        #[inline(always)]
        fn try_push(&mut self, data: u8) -> Result<()> {
            self.vec.push(data).map_err(|_| Error::SerializeBufferFull)
        }

        fn finalize(self) -> Result<Vec<u8, B>> {
            Ok(self.vec)
        }
    }

    impl<const B: usize> Index<usize> for HVec<B> {
        type Output = u8;

        fn index(&self, idx: usize) -> &u8 {
            &self.vec[idx]
        }
    }

    impl<const B: usize> IndexMut<usize> for HVec<B> {
        fn index_mut(&mut self, idx: usize) -> &mut u8 {
            &mut self.vec[idx]
        }
    }
}
