use super::Object;
use std::{marker::PhantomData, ptr};

pub struct IntoIter {
    pub(super) buf: *mut Object,
    pub(super) phantom: PhantomData<Object>,
    pub(super) cap: usize,
    pub(super) alloc: Object,
    pub(super) ptr: *const Object,
    pub(super) end: *const Object,
}

impl IntoIter {
    fn as_raw_mut_slice(&mut self) -> *mut [Object] {
        ptr::slice_from_raw_parts_mut(self.ptr as *mut Object, self.len())
    }

    fn len(&self) -> usize {
        let (lower, upper) = self.size_hint();
        // Note: This assertion is overly defensive, but it checks the invariant
        // guaranteed by the trait. If this trait were rust-internal,
        // we could use debug_assert!; assert_eq! will check all Rust user
        // implementations too.
        assert_eq!(upper, Some(lower));
        lower
    }
}

impl Drop for IntoIter {
    fn drop(&mut self) {
        struct DropGuard<'a>(&'a mut IntoIter);

        impl Drop for DropGuard<'_> {
            fn drop(&mut self) {
                unsafe {
                    let _alloc = ptr::read(&self.0.alloc);
                    let _v = Vec::from_raw_parts(self.0.buf, self.0.cap, self.0.cap);
                }
            }
        }

        let guard = DropGuard(self);
        // destroy the remaining elements
        unsafe {
            ptr::drop_in_place(guard.0.as_raw_mut_slice());
        }
        // now `guard` will be dropped and do the rest
    }
}

impl Iterator for IntoIter {
    type Item = Object;

    #[inline]
    fn next(&mut self) -> Option<Object> {
        if self.ptr == self.end {
            None
        } else {
            let old = self.ptr;
            self.ptr = unsafe { self.ptr.offset(1) };

            Some(unsafe { ptr::read(old) })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = unsafe { self.end.offset_from(self.ptr) as usize };
        (exact, Some(exact))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
}
