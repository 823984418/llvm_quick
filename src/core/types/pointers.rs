use llvm_sys::core::*;

use crate::type_tag::*;
use crate::{Context, Opaque, Type};

impl Context {
    /// Create an opaque pointer type in a context.
    pub fn pointer_type_in(&self, address_space: u32) -> &Type<ptr_any> {
        unsafe { Type::from_ref(LLVMPointerTypeInContext(self.as_raw(), address_space)) }
    }

    /// Create an opaque pointer type in a context.
    pub fn pointer_type<const ADDRESS_SPACE: u32>(&self) -> &Type<ptr<ADDRESS_SPACE>> {
        unsafe { self.pointer_type_in(ADDRESS_SPACE).cast_unchecked() }
    }
}

impl<T: PtrTypeTag> Type<T> {
    pub fn get_address_space(&self) -> u32 {
        unsafe { LLVMGetPointerAddressSpace(self.as_raw()) }
    }
}
