use llvm_sys::core::*;

use crate::core::context::Context;
use crate::core::type_tag::void;
use crate::core::types::Type;
use crate::opaque::Opaque;

impl Context {
    /// Create a void type in a context.
    pub fn void_type(&self) -> &Type<void> {
        unsafe { Type::from_ref(LLVMVoidTypeInContext(self.as_raw())) }
    }
}
