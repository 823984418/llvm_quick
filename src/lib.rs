use std::marker::PhantomData;

pub use llvm_sys;
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
pub struct Module<'s> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'s Context>,
}

unsafe impl<'s> Opaque for Module<'s> {
    type Inner = LLVMModule;
}

#[repr(transparent)]
pub struct Type<T: TypeTag> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<fn(T) -> T>,
}

unsafe impl<T: TypeTag> Opaque for Type<T> {
    type Inner = LLVMType;
}

#[repr(transparent)]
pub struct Value<T: TypeTag> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<fn(T) -> T>,
}

unsafe impl<T: TypeTag> Opaque for Value<T> {
    type Inner = LLVMValue;
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
pub struct Builder<'s> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'s Context>,
}

unsafe impl<'s> Opaque for Builder<'s> {
    type Inner = LLVMBuilder;
}

#[repr(transparent)]
pub struct DIBuilder {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for DIBuilder {
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
pub struct OperandBundle<'s> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'s Context>,
}

unsafe impl<'s> Opaque for OperandBundle<'s> {
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
