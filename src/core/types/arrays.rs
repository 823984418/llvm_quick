use llvm_sys::core::{LLVMArrayType2, LLVMGetElementType};

use crate::type_tag::*;
use crate::{Opaque, Type};

impl<T: ArrayTypeTag> Type<T> {
    pub fn element_type(&self) -> &Type<T::ElementType> {
        unsafe { Type::from_ref(LLVMGetElementType(self.as_raw())) }
    }
}

impl<T: TypeTag> Type<T> {
    /// Create a fixed size array type that refers to a specific type.
    ///
    /// The created type will exist in the context that its element type exists in.
    pub fn array_in(&self, count: u64) -> &Type<array_unsized<T>> {
        unsafe { Type::from_ref(LLVMArrayType2(self.as_raw(), count)) }
    }

    pub fn array<const N: u64>(&self) -> &Type<array<T, N>> {
        unsafe { self.array_in(N).cast_unchecked() }
    }
}
