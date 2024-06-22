use llvm_sys::core::*;

use crate::type_tag::*;
use crate::{Context, Opaque, Type};

impl<T: ArrayTypeTag> Type<T> {
    pub fn to_array_any(&self) -> &Type<array_any> {
        unsafe { self.cast_unchecked() }
    }
}

impl<T: VectorTypeTag> Type<T> {
    pub fn to_vector_any(&self) -> &Type<vector_any> {
        unsafe { self.cast_unchecked() }
    }
}

impl<T: SequentialTypeTag> Type<T> {
    pub fn element_type(&self) -> &Type<T::ElementType> {
        unsafe { Type::from_raw(LLVMGetElementType(self.as_raw())) }
    }
}

impl<T: TypeTag> Type<T> {
    pub fn get_subtypes<'s, 'c>(
        &'c self,
        slice: &'s mut [Option<&'c Type<any>>],
    ) -> &'s mut [&'c Type<any>] {
        assert_eq!(slice.len(), self.get_num_contained_types() as usize);
        unsafe {
            LLVMGetSubtypes(self.as_raw(), slice.as_mut_ptr() as _);
            std::mem::transmute(slice)
        }
    }

    pub fn get_num_contained_types(&self) -> u32 {
        unsafe { LLVMGetNumContainedTypes(self.as_raw()) }
    }

    /// Create a fixed size array type that refers to a specific type.
    ///
    /// The created type will exist in the context that its element type exists in.
    pub fn array_type_any_len(&self, count: u64) -> &Type<array_any_len<T>> {
        unsafe { Type::from_raw(LLVMArrayType2(self.as_raw(), count)) }
    }

    pub fn array_type<const N: u64>(&self) -> &Type<array<T, N>> {
        unsafe { self.array_type_any_len(N).cast_unchecked() }
    }
}

impl<T: ArrayTypeTag> Type<T> {
    pub fn get_length(&self) -> u64 {
        unsafe { LLVMGetArrayLength2(self.as_raw()) }
    }
}

impl Context {
    /// Create an opaque pointer type in a context.
    pub fn pointer_type_any(&self, address_space: u32) -> &Type<ptr_any> {
        unsafe { Type::from_raw(LLVMPointerTypeInContext(self.as_raw(), address_space)) }
    }

    /// Create an opaque pointer type in a context.
    pub fn pointer_type<const ADDRESS_SPACE: u32>(&self) -> &Type<ptr<ADDRESS_SPACE>> {
        unsafe { self.pointer_type_any(ADDRESS_SPACE).cast_unchecked() }
    }
}

impl<T: PtrTypeTag> Type<T> {
    pub fn get_address_space(&self) -> u32 {
        unsafe { LLVMGetPointerAddressSpace(self.as_raw()) }
    }
}

impl<T: TypeTag> Type<T> {
    pub fn vector_type_any_count(&self, element_count: u32) -> &Type<vector_any_len<T>> {
        unsafe { Type::from_raw(LLVMVectorType(self.as_raw(), element_count)) }
    }

    /// Create a vector type that contains a defined type and has a scalable number of elements.
    pub fn scalable_vector_type_any_count(&self, element_count: u32) -> &Type<vector_any_len<T>> {
        unsafe { Type::from_raw(LLVMScalableVectorType(self.as_raw(), element_count)) }
    }
}
impl<T: VectorTypeTag> Type<T> {
    pub fn get_size(&self) -> u32 {
        unsafe { LLVMGetVectorSize(self.as_raw()) }
    }
}
