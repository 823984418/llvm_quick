use std::fmt::{Debug, Formatter};

use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::Message;
use crate::type_tag::*;
use crate::{Context, Opaque, Type};

pub mod float;
pub mod function;
pub mod integer;
pub mod other;
pub mod sequential;
pub mod structs;

impl<T: TypeTag> Debug for Type<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.print_to_string().to_str().unwrap())
    }
}

impl<T: TypeTag> Type<T> {
    pub fn to_any(&self) -> &Type<any> {
        unsafe { self.cast_unchecked() }
    }
}

impl<T: TypeTag> Type<T> {
    /// Obtain the enumerated type of a Type instance.
    pub fn get_kind(&self) -> LLVMTypeKind {
        unsafe { LLVMGetTypeKind(self.as_raw()) }
    }

    pub fn is_sized(&self) -> bool {
        unsafe { LLVMTypeIsSized(self.as_raw()) != 0 }
    }

    /// Obtain the context to which this type instance is associated.
    pub fn get_context(&self) -> &Context {
        unsafe { Context::from_raw(LLVMGetTypeContext(self.as_raw())) }
    }

    pub fn dump(&self) {
        unsafe { LLVMDumpType(self.as_raw()) }
    }

    /// Return a string representation of the type.
    pub fn print_to_string(&self) -> Message {
        unsafe { Message::from_raw(LLVMPrintTypeToString(self.as_raw())) }
    }
}
