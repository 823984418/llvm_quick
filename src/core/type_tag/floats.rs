use llvm_sys::*;

use crate::core::type_tag::{any, type_check_kind, TypeTag};
use crate::Type;

pub trait FloatTypeTag: TypeTag {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct float_any {}

impl TypeTag for float_any {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        match ty.get_kind() {
            LLVMTypeKind::LLVMHalfTypeKind
            | LLVMTypeKind::LLVMFloatTypeKind
            | LLVMTypeKind::LLVMDoubleTypeKind
            | LLVMTypeKind::LLVMX86_FP80TypeKind
            | LLVMTypeKind::LLVMFP128TypeKind
            | LLVMTypeKind::LLVMPPC_FP128TypeKind
            | LLVMTypeKind::LLVMBFloatTypeKind => Some(unsafe { ty.cast_unchecked() }),
            _ => None,
        }
    }
}

impl FloatTypeTag for float_any {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct half {}

impl TypeTag for half {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMHalfTypeKind) }
    }
}

impl FloatTypeTag for half {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct float {}

impl TypeTag for float {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMFloatTypeKind) }
    }
}

impl FloatTypeTag for float {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct double {}

impl TypeTag for double {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMDoubleTypeKind) }
    }
}

impl FloatTypeTag for double {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct x86_fp80 {}

impl TypeTag for x86_fp80 {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMX86_FP80TypeKind) }
    }
}

impl FloatTypeTag for x86_fp80 {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct fp128 {}

impl TypeTag for fp128 {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMFP128TypeKind) }
    }
}

impl FloatTypeTag for fp128 {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct ppc_fp128 {}

impl TypeTag for ppc_fp128 {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMPPC_FP128TypeKind) }
    }
}

impl FloatTypeTag for ppc_fp128 {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct bfloat {}

impl TypeTag for bfloat {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMBFloatTypeKind) }
    }
}

impl FloatTypeTag for bfloat {}
