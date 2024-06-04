use llvm_sys::core::*;

use crate::core::context::Context;
use crate::core::type_tag::floats::float;
use crate::core::types::Type;
use crate::opaque::Opaque;

impl Context {
    /// Create a float type in a context.
    pub fn float_type(&self) -> &Type<float> {
        unsafe { Type::from_ref(LLVMFloatTypeInContext(self.as_raw())) }
    }
}
