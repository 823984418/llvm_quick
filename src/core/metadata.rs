use llvm_sys::core::*;

use crate::core::type_tag::TypeTag;
use crate::Opaque;
use crate::{Metadata, Value};

// TODO: more

impl<T: TypeTag> Value<T> {
    pub fn as_metadata(&self) -> &Metadata {
        unsafe { Metadata::from_ref(LLVMValueAsMetadata(self.as_raw())) }
    }
}
