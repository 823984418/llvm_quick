use llvm_sys::core::{LLVMArrayType2, LLVMGetElementType};

use crate::type_tag::*;
use crate::{Opaque, Type};

impl<T: ArrayTypeTag> Type<T> {
    pub fn element_type(&self) -> &Type<T::ElementType> {
        unsafe { Type::from_ref(LLVMGetElementType(self.as_raw())) }
    }
}

impl<T: ArrayTypeTag> Type<T> {
    pub fn to_array_any(&self) -> &Type<array_any_len<any>> {
        unsafe { self.cast_unchecked() }
    }

    pub fn length(&self) -> u64 {
        T::type_length(self)
    }
}

impl<T: TypeTag> Type<T> {
    /// Create a fixed size array type that refers to a specific type.
    ///
    /// The created type will exist in the context that its element type exists in.
    pub fn array_in(&self, count: u64) -> &Type<array_any_len<T>> {
        unsafe { Type::from_ref(LLVMArrayType2(self.as_raw(), count)) }
    }

    pub fn array<const N: u64>(&self) -> &Type<array<T, N>> {
        unsafe { self.array_in(N).cast_unchecked() }
    }
}
