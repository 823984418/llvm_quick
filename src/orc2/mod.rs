use llvm_sys::orc2::*;

use crate::owning::{OpaqueClone, OpaqueDrop};
use crate::{Opaque, PhantomOpaque};

pub mod ee;
pub mod lljit;

#[repr(transparent)]
pub struct OrcExecutionSession {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcExecutionSession {
    type Inner = LLVMOrcOpaqueExecutionSession;
}

#[repr(transparent)]
pub struct OrcSymbolStringPool {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcSymbolStringPool {
    type Inner = LLVMOrcOpaqueSymbolStringPool;
}

#[repr(transparent)]
pub struct OrcSymbolStringPoolEntry {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcSymbolStringPoolEntry {
    type Inner = LLVMOrcOpaqueSymbolStringPoolEntry;
}

impl OpaqueDrop for LLVMOrcOpaqueSymbolStringPoolEntry {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcReleaseSymbolStringPoolEntry(ptr) }
    }
}

impl OpaqueClone for LLVMOrcOpaqueSymbolStringPoolEntry {
    unsafe fn clone_raw(ptr: *mut Self) -> *mut Self {
        unsafe { LLVMOrcRetainSymbolStringPoolEntry(ptr) }
        ptr
    }
}

#[repr(transparent)]
pub struct OrcJitDylib {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcJitDylib {
    type Inner = LLVMOrcOpaqueJITDylib;
}

#[repr(transparent)]
pub struct OrcMaterializationUnit {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcMaterializationUnit {
    type Inner = LLVMOrcOpaqueMaterializationUnit;
}

#[repr(transparent)]
pub struct OrcMaterializationResponsibility {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcMaterializationResponsibility {
    type Inner = LLVMOrcOpaqueMaterializationResponsibility;
}

#[repr(transparent)]
pub struct OrcResourceTracker {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcResourceTracker {
    type Inner = LLVMOrcOpaqueResourceTracker;
}

#[repr(transparent)]
pub struct OrcDefinitionGenerator {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcDefinitionGenerator {
    type Inner = LLVMOrcOpaqueDefinitionGenerator;
}

#[repr(transparent)]
pub struct OrcLookupState {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcLookupState {
    type Inner = LLVMOrcOpaqueLookupState;
}

#[repr(transparent)]
pub struct OrcThreadSafeContext {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcThreadSafeContext {
    type Inner = LLVMOrcOpaqueThreadSafeContext;
}

#[repr(transparent)]
pub struct OrcThreadSafeModule {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcThreadSafeModule {
    type Inner = LLVMOrcOpaqueThreadSafeModule;
}

impl OpaqueDrop for LLVMOrcOpaqueThreadSafeModule {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeThreadSafeModule(ptr) }
    }
}

#[repr(transparent)]
pub struct OrcJitTargetMachineBuilder {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcJitTargetMachineBuilder {
    type Inner = LLVMOrcOpaqueJITTargetMachineBuilder;
}

impl OpaqueDrop for LLVMOrcOpaqueJITTargetMachineBuilder {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeJITTargetMachineBuilder(ptr) }
    }
}

#[repr(transparent)]
pub struct OrcObjectLayer {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcObjectLayer {
    type Inner = LLVMOrcOpaqueObjectLayer;
}

impl OpaqueDrop for LLVMOrcOpaqueObjectLayer {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeObjectLayer(ptr) }
    }
}

#[repr(transparent)]
pub struct OrcObjectLinkingLayer {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcObjectLinkingLayer {
    type Inner = LLVMOrcOpaqueObjectLinkingLayer;
}

#[repr(transparent)]
pub struct OrcIrTransformLayer {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcIrTransformLayer {
    type Inner = LLVMOrcOpaqueIRTransformLayer;
}

#[repr(transparent)]
pub struct OrcObjectTransformLayer {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcObjectTransformLayer {
    type Inner = LLVMOrcOpaqueObjectTransformLayer;
}

#[repr(transparent)]
pub struct OrcIndirectStubsManager {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcIndirectStubsManager {
    type Inner = LLVMOrcOpaqueIndirectStubsManager;
}

impl OpaqueDrop for LLVMOrcOpaqueIndirectStubsManager {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeIndirectStubsManager(ptr) }
    }
}

#[repr(transparent)]
pub struct OrcLazyCallThroughManager {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcLazyCallThroughManager {
    type Inner = LLVMOrcOpaqueLazyCallThroughManager;
}

impl OpaqueDrop for LLVMOrcOpaqueLazyCallThroughManager {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeLazyCallThroughManager(ptr) }
    }
}

#[repr(transparent)]
pub struct OrcDumpObjects {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcDumpObjects {
    type Inner = LLVMOrcOpaqueDumpObjects;
}

impl OpaqueDrop for LLVMOrcOpaqueDumpObjects {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeDumpObjects(ptr) }
    }
}

// TODO
