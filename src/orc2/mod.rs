use std::ffi::CStr;

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

#[repr(transparent)]
pub struct OrcJitTargetMachineBuilder {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcJitTargetMachineBuilder {
    type Inner = LLVMOrcOpaqueJITTargetMachineBuilder;
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

#[repr(transparent)]
pub struct OrcLazyCallThroughManager {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcLazyCallThroughManager {
    type Inner = LLVMOrcOpaqueLazyCallThroughManager;
}

#[repr(transparent)]
pub struct OrcDumpObjects {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcDumpObjects {
    type Inner = LLVMOrcOpaqueDumpObjects;
}

impl OrcExecutionSession {
    pub fn set_error_reporter_raw(&self, report_error: LLVMOrcErrorReporterFunction, ctx: *mut ()) {
        unsafe { LLVMOrcExecutionSessionSetErrorReporter(self.as_raw(), report_error, ctx as _) }
    }

    pub fn get_symbol_string_pool(&self) -> &OrcSymbolStringPool {
        unsafe {
            OrcSymbolStringPool::from_raw(LLVMOrcExecutionSessionGetSymbolStringPool(self.as_raw()))
        }
    }
}

impl OrcSymbolStringPool {
    pub unsafe fn clear_dead_entries(&self) {
        unsafe { LLVMOrcSymbolStringPoolClearDeadEntries(self.as_raw()) }
    }
}

impl OrcExecutionSession {
    pub fn intern(&self, name: &CStr) -> &OrcSymbolStringPoolEntry {
        unsafe {
            OrcSymbolStringPoolEntry::from_raw(LLVMOrcExecutionSessionIntern(
                self.as_raw(),
                name.as_ptr(),
            ))
        }
    }
}

impl OrcExecutionSession {
    pub fn lookup_raw(
        &self,
        k: LLVMOrcLookupKind,
        search_order: &LLVMOrcCJITDylibSearchOrderElement,
        symbols: &LLVMOrcCLookupSetElement,
        handle_result: LLVMOrcExecutionSessionLookupHandleResultFunction,
        ctx: *mut (),
    ) {
        unsafe {
            LLVMOrcExecutionSessionLookup(
                self.as_raw(),
                k,
                search_order as *const _ as *mut _,
                size_of::<LLVMOrcCJITDylibSearchOrderElement>(),
                symbols as *const _ as *mut _,
                size_of::<LLVMOrcCLookupSetElement>(),
                handle_result,
                ctx as _,
            )
        }
    }
}

// TODO

impl OpaqueDrop for LLVMOrcOpaqueSymbolStringPoolEntry {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcReleaseSymbolStringPoolEntry(ptr) }
    }
}

// TODO

impl OpaqueClone for LLVMOrcOpaqueSymbolStringPoolEntry {
    unsafe fn clone_raw(ptr: *mut Self) -> *mut Self {
        unsafe { LLVMOrcRetainSymbolStringPoolEntry(ptr) }
        ptr
    }
}

// TODO

impl OpaqueDrop for LLVMOrcOpaqueThreadSafeModule {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeThreadSafeModule(ptr) }
    }
}

// TODO

impl OpaqueDrop for LLVMOrcOpaqueJITTargetMachineBuilder {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeJITTargetMachineBuilder(ptr) }
    }
}

// TODO

impl OpaqueDrop for LLVMOrcOpaqueIndirectStubsManager {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeIndirectStubsManager(ptr) }
    }
}

// TODO

impl OpaqueDrop for LLVMOrcOpaqueLazyCallThroughManager {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeLazyCallThroughManager(ptr) }
    }
}

// TODO

impl OpaqueDrop for LLVMOrcOpaqueDumpObjects {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeDumpObjects(ptr) }
    }
}

// TODO
