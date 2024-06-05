use llvm_sys::core::*;

use crate::core::type_tag::pointers::{ptr, ptr_any, PtrTypeTag};
use crate::Context;
use crate::Opaque;
use crate::Type;

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
        T::type_get_address_space(self)
    }
}
