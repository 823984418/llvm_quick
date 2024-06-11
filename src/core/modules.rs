use std::ffi::CStr;
use std::fmt::{Debug, Formatter};

use llvm_sys::core::*;

use crate::core::Message;
use crate::owning::{OpaqueClone, OpaqueDrop, Owning};
use crate::type_tag::*;
use crate::{Context, Module, Opaque, Type, Value};

impl<'s> OpaqueDrop for Module<'s> {
    unsafe fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeModule(ptr) }
    }
}

impl<'s> Debug for Module<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.print_to_string().to_str().unwrap())
    }
}

impl Context {
    /// Create a new, empty module in a specific context.
    pub fn create_module(&self, name: &CStr) -> Owning<Module> {
        unsafe {
            let ptr = LLVMModuleCreateWithNameInContext(name.as_ptr(), self.as_raw());
            Owning::from_raw(ptr)
        }
    }
}

impl<'s> OpaqueClone for Module<'s> {
    unsafe fn clone_raw(ptr: *mut Self::Inner) -> *mut Self::Inner {
        unsafe { LLVMCloneModule(ptr) }
    }
}

impl<'s> Module<'s> {
    /// Obtain the context to which this module is associated.
    pub fn context(&self) -> &'s Context {
        unsafe { Context::from_ref(LLVMGetModuleContext(self.as_raw())) }
    }

    /// Add a function to a module under a specified name.
    pub fn add_function<T: FunTypeTag>(&self, name: &CStr, ty: &'s Type<T>) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMAddFunction(self.as_raw(), name.as_ptr(), ty.as_raw())) }
    }

    /// Obtain a Function value from a Module by its name.
    pub fn get_function<T: FunTypeTag>(&self, name: &CStr) -> &'s Value<T> {
        unsafe { Value::<any>::from_ref(LLVMGetNamedFunction(self.as_raw(), name.as_ptr())).cast() }
    }

    pub fn print_to_string(&self) -> Message {
        unsafe { Message::from_raw(LLVMPrintModuleToString(self.as_raw())) }
    }
}
