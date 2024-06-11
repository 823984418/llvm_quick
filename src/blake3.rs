use std::mem::MaybeUninit;

use llvm_sys::blake3::*;

#[repr(transparent)]
pub struct Blake3Hasher {
    inner: llvm_blake3_hasher,
}

impl Blake3Hasher {
    pub fn init() -> Self {
        unsafe {
            let mut inner = MaybeUninit::uninit();
            llvm_blake3_hasher_init(inner.as_mut_ptr() as _);
            let inner = inner.assume_init();
            Self { inner }
        }
    }

    pub fn init_keyed(&mut self, key: [u8; LLVM_BLAKE3_KEY_LEN]) {
        unsafe { llvm_blake3_hasher_init_keyed(&mut self.inner, key.as_ptr()) }
    }

    pub fn init_derive_keyed(&mut self, key: &[u8]) {
        unsafe {
            llvm_blake3_hasher_init_derive_key_raw(&mut self.inner, key.as_ptr() as _, key.len());
        }
    }

    pub fn update(&mut self, input: &[u8]) {
        unsafe { llvm_blake3_hasher_update(&mut self.inner, input.as_ptr() as _, input.len()) }
    }

    pub fn finalize(&mut self, out: &[u8]) {
        unsafe { llvm_blake3_hasher_finalize(&mut self.inner, out.as_ptr() as _, out.len()) }
    }

    pub fn finalize_seek(&mut self, seek: u64, out: &[u8]) {
        unsafe {
            llvm_blake3_hasher_finalize_seek(&mut self.inner, seek, out.as_ptr() as _, out.len());
        }
    }

    pub fn reset(&mut self) {
        unsafe { llvm_blake3_hasher_reset(&mut self.inner) }
    }
}
