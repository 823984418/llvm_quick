use std::ffi::CStr;

use llvm_sys::core::*;

use crate::opaque::Opaque;
use crate::type_tag::*;
use crate::*;

impl Context {
    pub fn struct_type(&self, element_types: &[Type<any>], packed: bool) -> &Type<struct_any> {
        unsafe {
            Type::from_raw(LLVMStructTypeInContext(
                self.as_raw(),
                element_types.as_ptr() as _,
                element_types.len() as _,
                packed as _,
            ))
        }
    }

    pub fn struct_create_named(&self, name: &CStr) -> &Type<struct_any> {
        unsafe { Type::from_raw(LLVMStructCreateNamed(self.as_raw(), name.as_ptr())) }
    }
}

impl Type<struct_any> {
    pub fn get_struct_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMGetStructName(self.as_raw())) }
    }

    pub fn set_body<T: TypeTag>(&self, element_types: &[Type<T>], packed: bool) {
        unsafe {
            LLVMStructSetBody(
                self.as_raw(),
                element_types.as_ptr() as _,
                element_types.len() as _,
                packed as _,
            )
        }
    }

    pub fn count_struct_element_types(&self) -> u32 {
        unsafe { LLVMCountStructElementTypes(self.as_raw()) }
    }

    pub fn get_struct_element_types<'s, 'c>(
        &'c self,
        slice: &'s mut [Option<&'c Type<any>>],
    ) -> &'s mut [&'c Type<any>] {
        unsafe {
            LLVMGetStructElementTypes(self.as_raw(), slice.as_mut_ptr() as _);
            std::mem::transmute(slice)
        }
    }

    pub fn get_type_at_index(&self, i: u32) -> Option<&Type<any>> {
        unsafe { Type::from_ptr(LLVMStructGetTypeAtIndex(self.as_raw(), i)) }
    }

    pub fn is_packed_struct(&self) -> bool {
        unsafe { LLVMIsPackedStruct(self.as_raw()) != 0 }
    }

    pub fn is_opaque_struct(&self) -> bool {
        unsafe { LLVMIsOpaqueStruct(self.as_raw()) != 0 }
    }

    pub fn is_literal_struct(&self) -> bool {
        unsafe { LLVMIsLiteralStruct(self.as_raw()) != 0 }
    }
}
