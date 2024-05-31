use llvm_sys::LLVMTypeKind;

use crate::type_tag::{any, type_check_kind, InstanceTypeTag, MathTypeTag, TypeTag};
use crate::types::Type;

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct half {}

impl TypeTag for half {
    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMHalfTypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMHalfTypeKind) }
    }
}

impl InstanceTypeTag for half {}
impl MathTypeTag for half {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct float {}

impl TypeTag for float {
    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMFloatTypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMFloatTypeKind) }
    }
}

impl InstanceTypeTag for float {}
impl MathTypeTag for float {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct double {}

impl TypeTag for double {
    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMDoubleTypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMDoubleTypeKind) }
    }
}

impl InstanceTypeTag for double {}
impl MathTypeTag for double {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct x86_fp80 {}

impl TypeTag for x86_fp80 {
    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMX86_FP80TypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMX86_FP80TypeKind) }
    }
}

impl InstanceTypeTag for x86_fp80 {}
impl MathTypeTag for x86_fp80 {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct fp128 {}

impl TypeTag for fp128 {
    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMFP128TypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMFP128TypeKind) }
    }
}

impl InstanceTypeTag for fp128 {}
impl MathTypeTag for fp128 {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct ppc_fp128 {}

impl TypeTag for ppc_fp128 {
    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMPPC_FP128TypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMPPC_FP128TypeKind) }
    }
}

impl InstanceTypeTag for ppc_fp128 {}
impl MathTypeTag for ppc_fp128 {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct bfloat {}

impl TypeTag for bfloat {
    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMBFloatTypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMBFloatTypeKind) }
    }
}

impl InstanceTypeTag for bfloat {}
impl MathTypeTag for bfloat {}
