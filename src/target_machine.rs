use llvm_sys::target_machine::*;

use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::Dispose;

pub struct TargetMachine {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for TargetMachine {
    type Inner = LLVMOpaqueTargetMachine;
}

impl Dispose for TargetMachine {
    unsafe fn dispose(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeTargetMachine(ptr) };
    }
}
