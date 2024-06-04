use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::Message;
use crate::opaque::{Opaque, PhantomOpaque};

#[repr(transparent)]
pub struct DiagnosticInfo {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for DiagnosticInfo {
    type Inner = LLVMDiagnosticInfo;
}

impl DiagnosticInfo {
    pub fn get_description(&self) -> Message {
        unsafe { Message::from_raw(LLVMGetDiagInfoDescription(self.as_raw())) }
    }

    pub fn get_severity(&self) -> LLVMDiagnosticSeverity {
        unsafe { LLVMGetDiagInfoSeverity(self.as_raw()) }
    }
}
