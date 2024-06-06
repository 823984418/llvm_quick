use llvm_sys::core::*;

use crate::type_tag::*;
use crate::{Metadata, Opaque, Value};

// TODO: more

impl<T: TypeTag> Value<T> {
    pub fn as_metadata(&self) -> &Metadata {
        unsafe { Metadata::from_ref(LLVMValueAsMetadata(self.as_raw())) }
    }
}
