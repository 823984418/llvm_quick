use std::ffi::{c_void, CStr};

use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::basic_block::BasicBlock;
use crate::core::builder::Builder;
use crate::core::diagnostic::DiagnosticInfo;
use crate::core::module::Module;
use crate::core::types::Type;
use crate::core::values::Value;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::{OpaqueDrop, Owning};
use crate::type_tag::float_tag::float;
use crate::type_tag::function_tag::FunTypeTag;
use crate::type_tag::integer_tag::{int, int1, int128, int16, int32, int64, int8};
use crate::type_tag::pointer_tag::{ptr, ptr_any};
use crate::type_tag::void;

#[repr(transparent)]
pub struct Context {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Context {
    type Inner = LLVMContext;
}

impl OpaqueDrop for Context {
    unsafe fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMContextDispose(ptr) };
    }
}

impl Context {
    /// Obtain the global context instance.
    pub fn global() -> &'static Self {
        unsafe { Self::from_ref(LLVMGetGlobalContext()) }
    }

    /// Create a new context.
    pub fn create() -> Owning<Self> {
        unsafe { Owning::from_raw(LLVMContextCreate() as _) }
    }

    pub fn set_diagnostic_handler_raw(&self, handle: LLVMDiagnosticHandler, handle_ctx: *mut ()) {
        unsafe { LLVMContextSetDiagnosticHandler(self.as_ptr(), handle, handle_ctx as _) };
    }

    pub fn set_diagnostic_handler<T: Fn(&DiagnosticInfo) + 'static>(&self, handle: T) {
        extern "C" fn handler_raw<T: Fn(&DiagnosticInfo) + 'static>(
            info: *mut LLVMDiagnosticInfo,
            handle: *mut c_void,
        ) {
            let handle = handle as *mut T;
            unsafe { (*handle)(DiagnosticInfo::from_ref(info)) }
        }
        self.set_diagnostic_handler_raw(
            Some(handler_raw::<T>),
            Box::into_raw(Box::new(handle)) as _,
        );
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn i1_type(&self) -> &Type<int1> {
        unsafe { Type::from_ref(LLVMInt1TypeInContext(self.as_ptr())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn i8_type(&self) -> &Type<int8> {
        unsafe { Type::from_ref(LLVMInt8TypeInContext(self.as_ptr())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn i16_type(&self) -> &Type<int16> {
        unsafe { Type::from_ref(LLVMInt16TypeInContext(self.as_ptr())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn i32_type(&self) -> &Type<int32> {
        unsafe { Type::from_ref(LLVMInt32TypeInContext(self.as_ptr())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn i64_type(&self) -> &Type<int64> {
        unsafe { Type::from_ref(LLVMInt64TypeInContext(self.as_ptr())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn i128_type(&self) -> &Type<int128> {
        unsafe { Type::from_ref(LLVMInt128TypeInContext(self.as_ptr())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn int_type<const N: u32>(&self) -> &Type<int<N>> {
        unsafe { Type::from_ref(LLVMIntTypeInContext(self.as_ptr(), N)) }
    }

    /// Create an opaque pointer type in a context.
    pub fn pointer_type_in(&self, address_space: u32) -> &Type<ptr_any> {
        unsafe { Type::from_ref(LLVMPointerTypeInContext(self.as_ptr(), address_space)) }
    }

    /// Create an opaque pointer type in a context.
    pub fn pointer_type<const ADDRESS_SPACE: u32>(&self) -> &Type<ptr<ADDRESS_SPACE>> {
        unsafe { self.pointer_type_in(ADDRESS_SPACE).cast_unchecked() }
    }

    /// Create a void type in a context.
    pub fn void_type(&self) -> &Type<void> {
        unsafe { Type::from_ref(LLVMVoidTypeInContext(self.as_ptr())) }
    }

    /// Create a float type in a context.
    pub fn float_type(&self) -> &Type<float> {
        unsafe { Type::from_ref(LLVMFloatTypeInContext(self.as_ptr())) }
    }

    /// Create a new, empty module in a specific context.
    pub fn create_module(&self, name: &CStr) -> Owning<Module> {
        unsafe {
            let ptr = LLVMModuleCreateWithNameInContext(name.as_ptr(), self.as_ptr());
            Owning::from_raw(ptr)
        }
    }

    pub fn create_builder(&self) -> Owning<Builder> {
        unsafe { Owning::from_raw(LLVMCreateBuilder()) }
    }

    pub fn append_basic_block<'s, T: FunTypeTag>(
        &'s self,
        f: &'s Value<T>,
        name: &'s CStr,
    ) -> &'s BasicBlock {
        unsafe {
            let ptr = LLVMAppendBasicBlockInContext(self.as_ptr(), f.as_ptr(), name.as_ptr());
            BasicBlock::from_ref(ptr)
        }
    }
}
