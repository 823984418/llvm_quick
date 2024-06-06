use std::ffi::{c_void, CStr};

use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::type_tag::any;
use crate::core::Message;
use crate::owning::{OpaqueDrop, Owning};
use crate::{Context, DiagnosticInfo, Opaque, Type};

impl Context {
    /// Create a new context.
    pub fn create() -> Owning<Self> {
        unsafe { Owning::from_raw(LLVMContextCreate() as _) }
    }

    /// Obtain the global context instance.
    pub fn get_global() -> &'static Self {
        unsafe { Self::from_ref(LLVMGetGlobalContext()) }
    }

    pub fn set_diagnostic_handler(&self, handle: LLVMDiagnosticHandler, handle_ctx: *mut ()) {
        unsafe { LLVMContextSetDiagnosticHandler(self.as_raw(), handle, handle_ctx as _) };
    }

    /// Leak
    pub fn set_diagnostic_handler_leak<T: Fn(&DiagnosticInfo) + 'static>(&self, handle: T) {
        extern "C" fn handler_raw<T: Fn(&DiagnosticInfo) + 'static>(
            info: *mut LLVMDiagnosticInfo,
            handle: *mut c_void,
        ) {
            let handle = handle as *mut T;
            unsafe { (*handle)(DiagnosticInfo::from_ref(info)) }
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
        unsafe { LLVMContextSetYieldCallback(self.as_raw(), callback, opaque_handle as _) };
    }

    pub fn should_discard_value_names(&self) -> bool {
        unsafe { LLVMContextShouldDiscardValueNames(self.as_raw()) != 0 }
    }

    pub fn set_discard_value_name(&self, discard: bool) {
        unsafe { LLVMContextSetDiscardValueNames(self.as_raw(), discard as _) };
    }
}

impl OpaqueDrop for Context {
    fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMContextDispose(ptr) };
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

impl Context {
    pub fn get_type_by_name(&self, name: &CStr) -> &Type<any> {
        unsafe { Type::from_ref(LLVMGetTypeByName2(self.as_raw(), name.as_ptr())) }
    }
}
