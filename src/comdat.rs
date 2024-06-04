use std::ffi::CStr;

use llvm_sys::comdat::*;
use llvm_sys::LLVMComdat;

use crate::core::module::Module;
use crate::core::type_tag::TypeTag;
use crate::core::values::Value;
use crate::opaque::{Opaque, PhantomOpaque};

#[repr(transparent)]
pub struct Comdat {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Comdat {
    type Inner = LLVMComdat;
}

impl<'s> Module<'s> {
    pub fn get_or_insert_comdat(&self, name: &CStr) -> &Comdat {
        unsafe { Comdat::from_ref(LLVMGetOrInsertComdat(self.as_raw(), name.as_ptr())) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn get_comdat(&self) -> &Comdat {
        unsafe { Comdat::from_ref(LLVMGetComdat(self.as_raw())) }
    }

    pub fn set_comdat(&self, c: &Comdat) {
        unsafe { LLVMSetComdat(self.as_raw(), c.as_raw()) };
    }
}

impl Comdat {
    pub fn get_selection_kind(&self) -> LLVMComdatSelectionKind {
        unsafe { LLVMGetComdatSelectionKind(self.as_raw()) }
    }
    pub fn set_selection_kind(&self, kind: LLVMComdatSelectionKind) {
        unsafe { LLVMSetComdatSelectionKind(self.as_raw(), kind) };
    }
}
