use llvm_sys::core::LLVMDisposeModuleProvider;
use llvm_sys::*;

use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::OpaqueDrop;

#[repr(transparent)]
pub struct ModuleProvider {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for ModuleProvider {
    type Inner = LLVMModuleProvider;
}

impl OpaqueDrop for ModuleProvider {
    unsafe fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeModuleProvider(ptr) };
    }
}
