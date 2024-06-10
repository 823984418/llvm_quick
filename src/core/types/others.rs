use llvm_sys::core::*;

use crate::type_tag::*;
use crate::{Context, Opaque, Type};

impl Context {
    /// Create a void type in a context.
    pub fn void_type(&self) -> &Type<void> {
        unsafe { Type::from_ref(LLVMVoidTypeInContext(self.as_raw())) }
    }

    pub fn label_type(&self) -> &Type<label> {
        unsafe { Type::from_ref(LLVMLabelTypeInContext(self.as_raw())) }
    }
}
