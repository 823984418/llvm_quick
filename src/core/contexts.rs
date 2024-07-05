use std::ffi::{c_void, CStr};

use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::Message;
use crate::owning::{OpaqueDrop, Owning};
use crate::type_tag::*;
use crate::*;

impl Context {
    /// Create a new context.
    pub fn create() -> Owning<Self> {
        unsafe { Owning::from_raw(LLVMContextCreate() as _) }
    }

    /// Obtain the global context instance.
    pub fn get_global() -> &'static Self {
        unsafe { Self::from_raw(LLVMGetGlobalContext()) }
    }

    pub fn set_diagnostic_handler(&self, handle: LLVMDiagnosticHandler, handle_ctx: *mut ()) {
        unsafe { LLVMContextSetDiagnosticHandler(self.as_raw(), handle, handle_ctx as _) }
    }

    /// Leak
    pub fn set_diagnostic_handler_leak<T: Fn(&DiagnosticInfo) + 'static>(&self, handle: T) {
        extern "C" fn handler_raw<T: Fn(&DiagnosticInfo) + 'static>(
            info: *mut LLVMDiagnosticInfo,
            handle: *mut c_void,
        ) {
            let handle = handle as *mut T;
            unsafe { (*handle)(DiagnosticInfo::from_raw(info)) }
        }
        self.set_diagnostic_handler(Some(handler_raw::<T>), Box::into_raw(Box::new(handle)) as _);
    }

    pub fn get_diagnostic_handler(&self) -> LLVMDiagnosticHandler {
        unsafe { LLVMContextGetDiagnosticHandler(self.as_raw()) }
    }

    pub fn get_diagnostic_context(&self) -> *mut () {
        unsafe { LLVMContextGetDiagnosticContext(self.as_raw()) as _ }
    }

    pub fn set_yield_callback(&self, callback: LLVMYieldCallback, opaque_handle: *mut ()) {
        unsafe { LLVMContextSetYieldCallback(self.as_raw(), callback, opaque_handle as _) }
    }

    pub fn should_discard_value_names(&self) -> bool {
        unsafe { LLVMContextShouldDiscardValueNames(self.as_raw()) != 0 }
    }

    pub fn set_discard_value_name(&self, discard: bool) {
        unsafe { LLVMContextSetDiscardValueNames(self.as_raw(), discard as _) }
    }
}

impl OpaqueDrop for LLVMContext {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMContextDispose(ptr) }
    }
}

impl DiagnosticInfo {
    pub fn get_description(&self) -> Message {
        unsafe { Message::from_raw(LLVMGetDiagInfoDescription(self.as_raw())) }
    }

    pub fn get_severity(&self) -> LLVMDiagnosticSeverity {
        unsafe { LLVMGetDiagInfoSeverity(self.as_raw()) }
    }
}

impl Context {
    pub fn get_md_kind_id(&self, name: &[u8]) -> u32 {
        unsafe { LLVMGetMDKindIDInContext(self.as_raw(), name.as_ptr() as _, name.len() as _) }
    }
}

pub fn get_enum_attribute_for_name(name: &[u8]) -> u32 {
    unsafe { LLVMGetEnumAttributeKindForName(name.as_ptr() as _, name.len()) }
}

pub fn get_last_enum_attribute_kind() -> u32 {
    unsafe { LLVMGetLastEnumAttributeKind() }
}

impl Context {
    pub fn create_enum_attribute(&self, kind_id: u32, val: u64) -> &EnumAttribute {
        unsafe { EnumAttribute::from_raw(LLVMCreateEnumAttribute(self.as_raw(), kind_id, val)) }
    }
}

impl EnumAttribute {
    pub fn get_kind(&self) -> u32 {
        unsafe { LLVMGetEnumAttributeKind(self.as_raw()) }
    }

    pub fn get_value(&self) -> u64 {
        unsafe { LLVMGetEnumAttributeValue(self.as_raw()) }
    }
}

impl Context {
    pub fn create_type_attribute<T: TypeTag>(
        &self,
        kind_id: u32,
        type_ref: &Type<T>,
    ) -> &TypeAttribute {
        unsafe {
            TypeAttribute::from_raw(LLVMCreateTypeAttribute(
                self.as_raw(),
                kind_id,
                type_ref.as_raw(),
            ))
        }
    }
}

impl TypeAttribute {
    pub fn get_value(&self) -> &Type<any> {
        unsafe { Type::from_raw(LLVMGetTypeAttributeValue(self.as_raw())) }
    }
}

impl Context {
    pub fn create_string_attribute(&self, k: &[u8], v: &[u8]) -> &StringAttribute {
        unsafe {
            StringAttribute::from_raw(LLVMCreateStringAttribute(
                self.as_raw(),
                k.as_ptr() as _,
                k.len() as _,
                v.as_ptr() as _,
                v.len() as _,
            ))
        }
    }
}

impl StringAttribute {
    pub fn get_kind(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetStringAttributeKind(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr as _, len as _)
        }
    }

    pub fn get_value(&self) -> &[u8] {
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

impl Context {
    pub fn get_type_by_name(&self, name: &CStr) -> &Type<any> {
        unsafe { Type::from_raw(LLVMGetTypeByName2(self.as_raw(), name.as_ptr())) }
    }
}
