use std::ffi::CStr;

use llvm_sys::disassembler::*;

use crate::owning::{OpaqueDrop, Owning};
use crate::{Opaque, PhantomOpaque};

#[repr(transparent)]
pub struct DisasmContext {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for DisasmContext {
    type Inner = LLVMOpaqueDisasmContext;
}

impl DisasmContext {
    /// Create a disassembler for the TripleName.
    pub fn create(
        triple_name: &CStr,
        disinfo: *mut (),
        tag: i32,
        info: LLVMOpInfoCallback,
        lookup: LLVMSymbolLookupCallback,
    ) -> Owning<Self> {
        unsafe {
            Owning::from_raw(LLVMCreateDisasm(
                triple_name.as_ptr(),
                disinfo as _,
                tag,
                info,
                lookup,
            ))
        }
    }

    pub fn create_with_cpu(
        triple_name: &CStr,
        cpu: &CStr,
        disinfo: *mut (),
        tag: i32,
        info: LLVMOpInfoCallback,
        lookup: LLVMSymbolLookupCallback,
    ) -> Owning<Self> {
        unsafe {
            Owning::from_raw(LLVMCreateDisasmCPU(
                triple_name.as_ptr(),
                cpu.as_ptr(),
                disinfo as _,
                tag,
                info,
                lookup,
            ))
        }
    }

    pub fn create_with_cpu_features(
        triple_name: &CStr,
        cpu: &CStr,
        features: &CStr,
        disinfo: *mut (),
        tag: i32,
        info: LLVMOpInfoCallback,
        lookup: LLVMSymbolLookupCallback,
    ) -> Owning<Self> {
        unsafe {
            Owning::from_raw(LLVMCreateDisasmCPUFeatures(
                triple_name.as_ptr(),
                cpu.as_ptr(),
                features.as_ptr(),
                disinfo as _,
                tag,
                info,
                lookup,
            ))
        }
    }

    pub fn set_options(&self, options: u64) -> Result<(), ()> {
        unsafe {
            if LLVMSetDisasmOptions(self.as_raw(), options) != 0 {
                Err(())
            } else {
                Ok(())
            }
        }
    }
}

impl OpaqueDrop for DisasmContext {
    fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisasmDispose(ptr) }
    }
}

impl DisasmContext {
    pub fn instruction(&self, bytes: &[u8], pc: u64, output: &mut [u8]) -> usize {
        unsafe {
            LLVMDisasmInstruction(
                self.as_raw(),
                bytes.as_ptr() as _,
                bytes.len() as _,
                pc,
                output.as_mut_ptr() as _,
                output.len(),
            )
        }
    }
}
