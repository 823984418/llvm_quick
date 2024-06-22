use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

pub use llvm_sys;
use llvm_sys::prelude::{LLVMModuleFlagEntry, LLVMValueMetadataEntry};
use llvm_sys::*;

use crate::opaque::{Opaque, PhantomOpaque};
use crate::type_tag::*;

pub mod analysis;
pub mod bit_reader;
pub mod bit_writer;
pub mod blake3;
pub mod comdat;
pub mod core;
pub mod debuginfo;
pub mod disassembler;
pub mod error;
pub mod error_handling;
pub mod execution_engine;
pub mod ir_reader;
pub mod linker;
pub mod lto;
pub mod object;
pub mod opaque;
pub mod orc2;
pub mod owning;
pub mod remark;
pub mod support;
pub mod target;
pub mod target_machine;
pub mod transforms;
pub mod type_tag;

#[repr(transparent)]
pub struct MemoryBuffer {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for MemoryBuffer {
    type Inner = LLVMMemoryBuffer;
}

#[repr(transparent)]
pub struct Context {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Context {
    type Inner = LLVMContext;
}

#[repr(transparent)]
pub struct Module<'c> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'c Context>,
}

unsafe impl<'c> Opaque for Module<'c> {
    type Inner = LLVMModule;
}

#[repr(transparent)]
pub struct Type<T: TypeTag> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<fn(T) -> T>,
}

unsafe impl<T: TypeTag> Opaque for Type<T> {
    type Inner = LLVMType;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe { T::type_cast(Type::from_raw(ptr)) }
    }
}

#[repr(transparent)]
pub struct Value<T: TypeTag> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<fn(T) -> T>,
}

unsafe impl<T: TypeTag> Opaque for Value<T> {
    type Inner = LLVMValue;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe {
            let x = Value::<any>::from_raw(ptr);
            x.get_type().try_cast::<Type<T>>()?;
            Some(x.cast_unchecked())
        }
    }
}

#[repr(transparent)]
pub struct Argument<T: TypeTag> {
    parent: Value<T>,
}

unsafe impl<T: TypeTag> Opaque for Argument<T> {
    type Inner = LLVMValue;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe { Value::<T>::try_from_raw(ptr)?.is_a_argument() }
    }
}

impl<T: TypeTag> Deref for Argument<T> {
    type Target = Value<T>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[repr(transparent)]
pub struct Constant<T: TypeTag> {
    parent: Value<T>,
}

unsafe impl<T: TypeTag> Opaque for Constant<T> {
    type Inner = LLVMValue;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe {
            let x = Value::<any>::try_from_raw(ptr)?.is_a_constant()?;
            x.get_type().try_cast::<Type<T>>()?;
            Some(x.cast_unchecked())
        }
    }
}

impl<T: TypeTag> Deref for Constant<T> {
    type Target = Value<T>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[repr(transparent)]
pub struct GlobalValue<T: TypeTag> {
    parent: Constant<ptr_any>,
    _marker: PhantomData<fn(T) -> T>,
}

unsafe impl<T: TypeTag> Opaque for GlobalValue<T> {
    type Inner = LLVMValue;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe {
            let x = Value::<any>::try_from_raw(ptr)?.is_a_global_value()?;
            x.get_value_type().try_cast::<Type<T>>()?;
            Some(x.cast_unchecked())
        }
    }
}

impl<T: TypeTag> Deref for GlobalValue<T> {
    type Target = Constant<ptr_any>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[repr(transparent)]
pub struct GlobalAlias<T: TypeTag> {
    parent: GlobalValue<T>,
}

unsafe impl<T: TypeTag> Opaque for GlobalAlias<T> {
    type Inner = LLVMValue;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        let x = Value::<any>::try_from_raw(ptr)?.is_a_global_alias()?;
        x.get_value_type().try_cast::<Type<T>>()?;
        Some(x.cast_unchecked())
    }
}

impl<T: TypeTag> Deref for GlobalAlias<T> {
    type Target = GlobalValue<T>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[repr(transparent)]
pub struct Instruction<T: TypeTag> {
    parent: Value<T>,
}

unsafe impl<T: TypeTag> Opaque for Instruction<T> {
    type Inner = LLVMValue;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe { Value::<T>::try_from_raw(ptr)?.is_a_instruction() }
    }
}

impl<T: TypeTag> Deref for Instruction<T> {
    type Target = Value<T>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[repr(transparent)]
pub struct BasicBlock {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for BasicBlock {
    type Inner = LLVMBasicBlock;
}

#[repr(transparent)]
pub struct Metadata {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Metadata {
    type Inner = LLVMOpaqueMetadata;
}

#[repr(transparent)]
pub struct NamedMDNode {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for NamedMDNode {
    type Inner = LLVMOpaqueNamedMDNode;
}

#[repr(transparent)]
pub struct ValueMetadataEntry {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for ValueMetadataEntry {
    type Inner = LLVMOpaqueValueMetadataEntry;
}

#[repr(transparent)]
pub struct Builder<'c> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'c Context>,
}

unsafe impl<'c> Opaque for Builder<'c> {
    type Inner = LLVMBuilder;
}

#[repr(transparent)]
pub struct DIBuilder<'m, 'c> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'m Module<'c>>,
}

unsafe impl<'m, 'c> Opaque for DIBuilder<'m, 'c> {
    type Inner = LLVMOpaqueDIBuilder;
}

#[repr(transparent)]
pub struct ModuleProvider {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for ModuleProvider {
    type Inner = LLVMModuleProvider;
}

#[repr(transparent)]
pub struct PassManager {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for PassManager {
    type Inner = LLVMPassManager;
}

#[repr(transparent)]
pub struct Use {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Use {
    type Inner = LLVMUse;
}

#[repr(transparent)]
pub struct OperandBundle<'c> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'c Context>,
}

unsafe impl<'c> Opaque for OperandBundle<'c> {
    type Inner = LLVMOpaqueOperandBundle;
}

#[repr(transparent)]
pub struct DiagnosticInfo {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for DiagnosticInfo {
    type Inner = LLVMDiagnosticInfo;
}

#[repr(transparent)]
pub struct Comdat {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Comdat {
    type Inner = LLVMComdat;
}

#[repr(transparent)]
pub struct ModuleFlagEntry {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for ModuleFlagEntry {
    type Inner = LLVMOpaqueModuleFlagEntry;
}

#[repr(transparent)]
pub struct JITEventListener {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for JITEventListener {
    type Inner = LLVMOpaqueJITEventListener;
}

#[repr(transparent)]
pub struct Attribute {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Attribute {
    type Inner = LLVMOpaqueAttributeRef;
}

#[repr(transparent)]
pub struct EnumAttribute {
    parent: Attribute,
}

unsafe impl Opaque for EnumAttribute {
    type Inner = LLVMOpaqueAttributeRef;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe {
            let x = Attribute::try_from_raw(ptr)?;
            if x.is_enum_attribute() {
                Some(x.cast_unchecked())
            } else {
                None
            }
        }
    }
}

impl Deref for EnumAttribute {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[repr(transparent)]
pub struct TypeAttribute {
    parent: Attribute,
}

unsafe impl Opaque for TypeAttribute {
    type Inner = LLVMOpaqueAttributeRef;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe {
            let x = Attribute::try_from_raw(ptr)?;
            if x.is_type_attribute() {
                Some(x.cast_unchecked())
            } else {
                None
            }
        }
    }
}

impl Deref for TypeAttribute {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[repr(transparent)]
pub struct StringAttribute {
    parent: Attribute,
}

unsafe impl Opaque for StringAttribute {
    type Inner = LLVMOpaqueAttributeRef;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe {
            let x = Attribute::try_from_raw(ptr)?;
            if x.is_string_attribute() {
                Some(x.cast_unchecked())
            } else {
                None
            }
        }
    }
}

impl Deref for StringAttribute {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

pub struct ValueMetadataEntries<'m> {
    ptr: NonNull<LLVMValueMetadataEntry>,
    len: usize,
    marker: PhantomData<&'m Metadata>,
}

impl<'m> ValueMetadataEntries<'m> {
    pub unsafe fn from_raw(ptr: *mut LLVMValueMetadataEntry, len: usize) -> Self {
        unsafe {
            Self {
                ptr: NonNull::new(ptr).unwrap_unchecked(),
                len,
                marker: PhantomData,
            }
        }
    }

    pub fn as_raw(&self) -> *mut LLVMValueMetadataEntry {
        self.ptr.as_ptr()
    }
}

pub struct ModuleFlagsMetadata<'m> {
    ptr: NonNull<LLVMModuleFlagEntry>,
    len: usize,
    marker: PhantomData<&'m Metadata>,
}

impl<'m> ModuleFlagsMetadata<'m> {
    pub unsafe fn from_raw(ptr: *mut LLVMModuleFlagEntry, len: usize) -> Self {
        unsafe {
            Self {
                ptr: NonNull::new(ptr).unwrap_unchecked(),
                len,
                marker: PhantomData,
            }
        }
    }

    pub fn as_raw(&self) -> *mut LLVMModuleFlagEntry {
        self.ptr.as_ptr()
    }
}
