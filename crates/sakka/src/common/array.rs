use core::{mem::MaybeUninit, ptr};

/// Creates an array of length `N` by calling the given function for each index.
///
/// If the function returns an error for any index, the function will return that error and drop
/// any already-initialized elements.
pub fn try_array_from_fn<T, E, const N: usize>(
    mut f: impl FnMut(usize) -> Result<T, E>,
) -> Result<[T; N], E> {
    let mut array: [MaybeUninit<T>; N] = [const { MaybeUninit::uninit() }; N];

    for i in 0..N {
        match f(i) {
            Ok(value) => {
                array[i].write(value);
            }
            Err(err) => {
                for slot in array[..i].iter_mut().rev() {
                    unsafe {
                        slot.assume_init_drop();
                    }
                }

                return Err(err);
            }
        }
    }

    // SAFETY: All elements were initialized above. On failure paths,
    // every initialized element was dropped before returning.
    Ok(unsafe { ptr::read(array.as_ptr() as *const [T; N]) })
}

#[cfg(test)]
mod tests {
    use core::sync::atomic::{AtomicUsize, Ordering};

    use super::try_array_from_fn;

    #[test]
    fn builds_array_on_success() {
        let arr = try_array_from_fn::<_, (), 4>(|i| Ok((i as u32) * 2)).unwrap();
        assert_eq!(arr, [0, 2, 4, 6]);
    }

    #[test]
    fn returns_error_and_drops_initialized_elements() {
        struct DropCounter<'a>(&'a AtomicUsize);

        impl Drop for DropCounter<'_> {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }

        let drops = AtomicUsize::new(0);

        let result = try_array_from_fn::<_, &'static str, 5>(|i| {
            if i == 2 {
                return Err("boom");
            }

            Ok(DropCounter(&drops))
        });

        match result {
            Err(err) => assert_eq!(err, "boom"),
            Ok(_) => panic!("expected error"),
        }
        assert_eq!(drops.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn supports_zero_length_arrays() {
        let called = AtomicUsize::new(0);

        let arr = try_array_from_fn::<_, (), 0>(|_| {
            called.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .unwrap();

        assert_eq!(arr, []);
        assert_eq!(called.load(Ordering::SeqCst), 0);
    }
}
