use llvm_sys::core::*;
use llvm_sys::*;

use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::Dispose;

#[repr(transparent)]
pub struct PassManager {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for PassManager {
    type Inner = LLVMPassManager;
}

impl Dispose for PassManager {
    unsafe fn dispose(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposePassManager(ptr) };
    }
}
