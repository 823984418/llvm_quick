use std::marker::PhantomData;

use llvm_sys::core::LLVMTypeOf;
use llvm_sys::LLVMValue;

use crate::opaque::{PhantomOpaque, Opaque};
use crate::type_tag::{any, TypeTag};
use crate::types::Type;

#[repr(transparent)]
pub struct Value<T: TypeTag> {
    opaque: PhantomOpaque,
    marker: PhantomData<fn(T) -> T>,
}

unsafe impl<T: TypeTag> Opaque for Value<T> {
    type Inner = LLVMValue;
}

impl<T: TypeTag> Value<T> {
    pub unsafe fn cast_unchecked<N: TypeTag>(&self) -> &Value<N> {
        unsafe { Value::from_ref(self.as_ptr()) }
    }

    pub fn try_cast<N: TypeTag>(&self) -> Option<&Value<N>> {
        if self.get_type().try_cast::<N>().is_some() {
            Some(unsafe { self.cast_unchecked() })
        } else {
            None
        }
    }

    pub fn cast<N: TypeTag>(&self) -> &Value<N> {
        self.try_cast().unwrap()
    }

    pub fn to_any(&self) -> &Value<any> {
        unsafe { self.cast_unchecked() }
    }

    /// Obtain the type of a value.
    pub fn get_type(&self) -> &Type<T> {
        unsafe { Type::from_ref(LLVMTypeOf(self.as_ptr())) }
    }
}
