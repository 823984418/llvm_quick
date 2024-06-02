use std::ffi::CStr;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

use llvm_sys::core::*;
use llvm_sys::target::*;
use llvm_sys::*;

use crate::core::context::Context;
use crate::core::types::Type;
use crate::core::values::Value;
use crate::message::Message;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::Dispose;
use crate::target::TargetData;
use crate::type_tag::any;
use crate::type_tag::function_tag::FunTypeTag;

#[repr(transparent)]
pub struct Module<'s> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'s Context>,
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

    pub fn get_data_layout(&self) -> &TargetData {
        unsafe { TargetData::from_ref(LLVMGetModuleDataLayout(self.as_ptr())) }
    }

    pub fn set_data_layout(&self, v: &TargetData) {
        unsafe { LLVMSetModuleDataLayout(self.as_ptr(), v.as_ptr()) };
    }
}
