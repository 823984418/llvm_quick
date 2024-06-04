use llvm_sys::core::*;
use llvm_sys::LLVMOpaqueAttributeRef;

use crate::core::context::Context;
use crate::core::type_tag::TypeTag;
use crate::core::types::Type;
use crate::opaque::{Opaque, PhantomOpaque};

pub fn get_enum_attribute_for_name(name: &[u8]) -> u32 {
    unsafe { LLVMGetEnumAttributeKindForName(name.as_ptr() as _, name.len()) }
}

pub fn get_last_enum_attribute_kind() -> u32 {
    unsafe { LLVMGetLastEnumAttributeKind() }
}

#[repr(transparent)]
pub struct Attribute {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Attribute {
    type Inner = LLVMOpaqueAttributeRef;
}

impl Context {
    pub fn create_enum_attribute(&self, kind_id: u32, val: u64) -> &Attribute {
        unsafe { Attribute::from_ref(LLVMCreateEnumAttribute(self.as_raw(), kind_id, val)) }
    }
}

impl Attribute {
    pub fn get_enum_attribute_kind(&self) -> u32 {
        unsafe { LLVMGetEnumAttributeKind(self.as_raw()) }
    }

    pub fn get_enum_attribute_value(&self) -> u64 {
        unsafe { LLVMGetEnumAttributeValue(self.as_raw()) }
    }
}

impl Context {
    pub fn create_type_attribute<T: TypeTag>(
        &self,
        kind_id: u32,
        type_ref: &Type<T>,
    ) -> &Attribute {
        unsafe {
            Attribute::from_ref(LLVMCreateTypeAttribute(
                self.as_raw(),
                kind_id,
                type_ref.as_raw(),
            ))
        }
    }
}

impl Attribute {
    pub fn get_type_attribute_value(&self) -> u64 {
        unsafe { LLVMGetEnumAttributeValue(self.as_raw()) }
    }
}

impl Context {
    pub fn create_string_attribute(&self, k: &[u8], v: &[u8]) -> &Attribute {
        unsafe {
            Attribute::from_ref(LLVMCreateStringAttribute(
                self.as_raw(),
                k.as_ptr() as _,
                k.len() as _,
                v.as_ptr() as _,
                v.len() as _,
            ))
        }
    }
}

impl Attribute {
    pub fn get_string_attribute_kind(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetStringAttributeKind(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr as _, len as _)
        }
    }

    pub fn get_string_attribute_value(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetStringAttributeValue(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr as _, len as _)
        }
    }
}

impl Attribute {
    pub fn is_enum_attribute(&self) -> bool {
        unsafe { LLVMIsEnumAttribute(self.as_raw()) != 0 }
    }

    pub fn is_string_attribute(&self) -> bool {
        unsafe { LLVMIsStringAttribute(self.as_raw()) != 0 }
    }

    pub fn is_type_attribute(&self) -> bool {
        unsafe { LLVMIsTypeAttribute(self.as_raw()) != 0 }
    }
}
