use llvm_sys::core::*;

use crate::core::type_tag::floats::float;
use crate::Context;
use crate::Opaque;
use crate::Type;

impl Context {
    /// Create a float type in a context.
    pub fn float_type(&self) -> &Type<float> {
        unsafe { Type::from_ref(LLVMFloatTypeInContext(self.as_raw())) }
    }
}
