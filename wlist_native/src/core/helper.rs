pub mod hasher {
    use std::sync::Arc;

    use bytes::Bytes;
    use faster_hex::{hex_encode, Error};
    use md5::Md5;
    use sha2::Sha256;
    use tokio::sync::Mutex;
    use tokio::task::spawn_blocking;

    fn hex32(src: &[u8]) -> Result<String, Error> {
        let mut buffer = vec![0; 32];
        hex_encode(src, &mut buffer)?;
        Ok(unsafe { String::from_utf8_unchecked(buffer) })
    }

    fn hex64(src: &[u8]) -> Result<String, Error> {
        let mut buffer = vec![0; 64];
        hex_encode(src, &mut buffer)?;
        Ok(unsafe { String::from_utf8_unchecked(buffer) })
    }

    macro_rules! define_hasher {
        ($vis: vis $name: ident($hasher: ty) $digest: ident -> $hex: ident ? $h: literal) => {
            $vis struct $name {
                inner: Arc<Mutex<$hasher>>,
            }

            impl $name {
                $vis fn new() -> Self {
                    Self { inner: Arc::new(Mutex::new($digest::Digest::new())), }
                }

                $vis async fn update(&self, data: Bytes) {
                    let mut inner = Arc::clone(&self.inner).lock_owned().await;
                    spawn_blocking(move || {
                        let inner: &mut $hasher = &mut *inner;
                        $digest::Digest::update(inner, data.as_ref());
                    }).await.expect(concat!("update ", $h, " should not fail"))
                }

                $vis async fn finalize(self) -> String {
                    let hasher = Arc::try_unwrap(self.inner).expect(concat!("waiting ", $h, " update finish")).into_inner();
                    spawn_blocking(move || $hex(&$digest::Digest::finalize(hasher)).expect(concat!($h, " should be valid hex"))).await
                        .expect(concat!("finalize ", $h, " should not fail"))
                }
            }
        };
    }
    define_hasher!(pub Md5Hasher(Md5) md5 -> hex32 ? "md5");
    define_hasher!(pub Sha256Hasher(Sha256) sha2 -> hex64 ? "sha256");
}

pub mod buffer {
    use bytes::buf::UninitSlice;
    use bytes::{BufMut, Bytes};

    #[inline]
    pub unsafe fn new_read_buffer(ptr: *const u8, cap: usize) -> Bytes {
        debug_assert!(!ptr.is_null(), "ptr is null");
        Vec::from_raw_parts(ptr as _, cap, cap).into()
    }

    #[inline]
    pub unsafe fn new_write_buffer(ptr: *mut u8, cap: usize) -> WriteBuffer {
        debug_assert!(!ptr.is_null(), "ptr is null");
        WriteBuffer { ptr, len: 0, cap, }
    }

    pub struct WriteBuffer {
        ptr: *mut u8,
        len: usize,
        cap: usize,
    }

    unsafe impl BufMut for WriteBuffer {
        #[inline]
        fn remaining_mut(&self) -> usize {
            self.cap - self.len
        }

        #[inline]
        unsafe fn advance_mut(&mut self, cnt: usize) {
            let remaining = self.remaining_mut();
            if remaining < cnt {
                panic!("advance_mut: cnt({cnt}) > remaining_mut({remaining}).");
            }
            self.len += cnt;
        }

        #[inline]
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unsafe { UninitSlice::from_raw_parts_mut(self.ptr.add(self.len), self.remaining_mut()) }
        }
    }

    unsafe impl Send for WriteBuffer { }
}
