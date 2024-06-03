use llvm_sys::core::*;
use llvm_sys::*;

use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::OpaqueDrop;

#[repr(transparent)]
pub struct PassManager {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for PassManager {
    type Inner = LLVMPassManager;
}

impl OpaqueDrop for PassManager {
    unsafe fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposePassManager(ptr) };
    }
}
