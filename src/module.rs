use std::ffi::CStr;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ptr::null_mut;

use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyModule};
use llvm_sys::core::*;
use llvm_sys::LLVMModule;

use crate::context::Context;
use crate::message::Message;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::Dispose;
use crate::type_tag::any;
use crate::type_tag::function_tag::FunTypeTag;
use crate::types::Type;
use crate::values::Value;

#[repr(transparent)]
pub struct Module<'s> {
    opaque: PhantomOpaque,
    marker: PhantomData<&'s Context>,
}

unsafe impl<'s> Opaque for Module<'s> {
    type Inner = LLVMModule;
}

impl<'s> Dispose for Module<'s> {
    unsafe fn dispose(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeModule(ptr) };
    }
}

impl<'s> Debug for Module<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.print_to_string().to_str().unwrap())
    }
}

impl<'s> Module<'s> {
    /// Obtain the context to which this module is associated.
    pub fn context(&self) -> &'s Context {
        unsafe { Context::from_ref(LLVMGetModuleContext(self.as_ptr())) }
    }

    /// Add a function to a module under a specified name.
    pub fn add_function<T: FunTypeTag>(&self, name: &CStr, ty: &'s Type<T>) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMAddFunction(self.as_ptr(), name.as_ptr(), ty.as_ptr())) }
    }

    /// Obtain a Function value from a Module by its name.
    pub fn get_function<T: FunTypeTag>(&self, name: &CStr) -> &'s Value<T> {
        unsafe { Value::<any>::from_ref(LLVMGetNamedFunction(self.as_ptr(), name.as_ptr())).cast() }
    }

    pub fn print_to_string(&self) -> Message {
        unsafe { Message::from_raw(LLVMPrintModuleToString(self.as_ptr())) }
    }

    pub fn verify(&self, action: LLVMVerifierFailureAction) -> Result<(), Message> {
        unsafe {
            let mut err = null_mut();
            if LLVMVerifyModule(self.as_ptr(), action, &mut err) != 0 {
                return Err(Message::from_raw(err));
            }
            Ok(())
        }
    }
}
