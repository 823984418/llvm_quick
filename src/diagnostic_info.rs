use llvm_sys::core::{LLVMGetDiagInfoDescription, LLVMGetDiagInfoSeverity};
use llvm_sys::{LLVMDiagnosticInfo, LLVMDiagnosticSeverity};

use crate::message::Message;
use crate::opaque::{Opaque, PhantomOpaque};

pub struct DiagnosticInfo {
    opaque: PhantomOpaque,
}

unsafe impl Opaque for DiagnosticInfo {
    type Inner = LLVMDiagnosticInfo;
}

impl DiagnosticInfo {
    pub fn get_description(&self) -> Message {
        unsafe { Message::from_raw(LLVMGetDiagInfoDescription(self.as_ptr())) }
    }

    pub fn get_severity(&self) -> LLVMDiagnosticSeverity {
        unsafe { LLVMGetDiagInfoSeverity(self.as_ptr()) }
    }
}
