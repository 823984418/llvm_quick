use std::ffi::CStr;
use std::ops::Deref;
use std::ptr::NonNull;

use llvm_sys::orc2::*;

use crate::error::Error;
use crate::owning::{OpaqueClone, OpaqueDrop, Owning};
use crate::{MemoryBuffer, Opaque, PhantomOpaque};

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

impl OpaqueClone for LLVMOrcOpaqueSymbolStringPoolEntry {
    unsafe fn clone_raw(ptr: *mut Self) -> *mut Self {
        unsafe { LLVMOrcRetainSymbolStringPoolEntry(ptr) }
        ptr
    }
}

impl OpaqueDrop for LLVMOrcOpaqueSymbolStringPoolEntry {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcReleaseSymbolStringPoolEntry(ptr) }
    }
}

impl OrcSymbolStringPoolEntry {
    pub fn get_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMOrcSymbolStringPoolEntryStr(self.as_raw())) }
    }
}

impl OpaqueDrop for LLVMOrcOpaqueResourceTracker {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcReleaseResourceTracker(ptr) }
    }
}

impl OrcResourceTracker {
    pub unsafe fn transfer_to(&self, dst: &OrcResourceTracker) {
        unsafe { LLVMOrcResourceTrackerTransferTo(self.as_raw(), dst.as_raw()) }
    }

    pub unsafe fn remove(&self) -> Result<(), Owning<Error>> {
        unsafe { Error::check(LLVMOrcResourceTrackerRemove(self.as_raw())) }
    }
}

impl OpaqueDrop for LLVMOrcOpaqueDefinitionGenerator {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeDefinitionGenerator(ptr) }
    }
}

impl OpaqueDrop for LLVMOrcOpaqueMaterializationUnit {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeMaterializationUnit(ptr) }
    }
}
impl OrcMaterializationUnit {
    pub fn create_custom_raw(
        name: &CStr,
        ctx: &mut (),
        syms: &[&LLVMOrcCSymbolFlagsMapPair],
        init_sym: OrcSymbolStringPoolEntry,
        materialize: LLVMOrcMaterializationUnitMaterializeFunction,
        discard: LLVMOrcMaterializationUnitDiscardFunction,
        destroy: LLVMOrcMaterializationUnitDestroyFunction,
    ) -> Owning<OrcMaterializationUnit> {
        unsafe {
            Owning::from_raw(LLVMOrcCreateCustomMaterializationUnit(
                name.as_ptr(),
                ctx as *mut _ as *mut _,
                syms.as_ptr() as _,
                syms.len(),
                init_sym.as_raw(),
                materialize,
                discard,
                destroy,
            ))
        }
    }
}

pub fn absolute_symbols<'m>(
    syms: &[(&'m OrcSymbolStringPoolEntry, LLVMJITEvaluatedSymbol)],
) -> &'m OrcMaterializationUnit {
    unsafe {
        OrcMaterializationUnit::from_raw(LLVMOrcAbsoluteSymbols(syms.as_ptr() as _, syms.len()))
    }
}

impl OrcLazyCallThroughManager {
    pub fn lazy_reexports(
        &self,
        ism: &OrcIndirectStubsManager,
        source_ref: &OrcJitDylib,
        callable_aliases: &[&LLVMOrcCSymbolAliasMapPair],
    ) -> &OrcMaterializationUnit {
        unsafe {
            OrcMaterializationUnit::from_raw(LLVMOrcLazyReexports(
                self.as_raw(),
                ism.as_raw(),
                source_ref.as_raw(),
                callable_aliases.as_ptr() as _,
                callable_aliases.len(),
            ))
        }
    }
}

impl OpaqueDrop for LLVMOrcOpaqueMaterializationResponsibility {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeMaterializationResponsibility(ptr) }
    }
}

impl OrcMaterializationResponsibility {
    pub fn get_target_dylib(&self) -> &OrcJitDylib {
        unsafe {
            OrcJitDylib::from_raw(LLVMOrcMaterializationResponsibilityGetTargetDylib(
                self.as_raw(),
            ))
        }
    }

    pub fn get_execution_session(&self) -> &OrcExecutionSession {
        unsafe {
            OrcExecutionSession::from_raw(LLVMOrcMaterializationResponsibilityGetExecutionSession(
                self.as_raw(),
            ))
        }
    }

    pub fn get_symbols(&self) -> CSymbolFlagsMap<'_> {
        unsafe {
            let mut len = 0;
            let ptr = LLVMOrcMaterializationResponsibilityGetSymbols(self.as_raw(), &mut len);
            CSymbolFlagsMap {
                ptr: NonNull::from(std::slice::from_raw_parts(ptr as _, len)),
            }
        }
    }
}

pub struct CSymbolFlagsMap<'m> {
    ptr: NonNull<[&'m LLVMOrcCSymbolFlagsMapPair]>,
}

impl<'m> Deref for CSymbolFlagsMap<'m> {
    type Target = [&'m LLVMOrcCSymbolFlagsMapPair];

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<'m> Drop for CSymbolFlagsMap<'m> {
    fn drop(&mut self) {
        unsafe { LLVMOrcDisposeCSymbolFlagsMap(self.ptr.as_ptr() as _) }
    }
}

impl OrcMaterializationResponsibility {
    pub fn get_initializer_symbol(&self) -> &OrcSymbolStringPoolEntry {
        unsafe {
            OrcSymbolStringPoolEntry::from_raw(
                LLVMOrcMaterializationResponsibilityGetInitializerSymbol(self.as_raw()),
            )
        }
    }

    pub fn get_requested_symbols(&self) -> Symbols<'_> {
        unsafe {
            let mut len = 0;
            let ptr =
                LLVMOrcMaterializationResponsibilityGetRequestedSymbols(self.as_raw(), &mut len);
            Symbols {
                ptr: NonNull::from(std::slice::from_raw_parts(ptr as _, len)),
            }
        }
    }
}

pub struct Symbols<'m> {
    ptr: NonNull<[&'m OrcSymbolStringPoolEntry]>,
}

impl<'m> Deref for Symbols<'m> {
    type Target = [&'m OrcSymbolStringPoolEntry];

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<'m> Drop for Symbols<'m> {
    fn drop(&mut self) {
        unsafe { LLVMOrcDisposeSymbols(self.ptr.as_ptr() as _) }
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

impl OpaqueDrop for LLVMOrcOpaqueLazyCallThroughManager {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeLazyCallThroughManager(ptr) }
    }
}

impl OrcDumpObjects {
    pub fn create(dump_dir: &CStr, identifier_override: &CStr) -> Owning<OrcDumpObjects> {
        unsafe {
            Owning::from_raw(LLVMOrcCreateDumpObjects(
                dump_dir.as_ptr(),
                identifier_override.as_ptr(),
            ))
        }
    }
}

impl OpaqueDrop for LLVMOrcOpaqueDumpObjects {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeDumpObjects(ptr) }
    }
}

impl OrcDumpObjects {
    pub fn call_operator(
        &self,
        obj_buffers: Owning<MemoryBuffer>,
    ) -> Result<Owning<MemoryBuffer>, Owning<Error>> {
        unsafe {
            let mut obj_buffers = obj_buffers.into_raw();
            Error::check(LLVMOrcDumpObjects_CallOperator(
                self.as_raw(),
                &mut obj_buffers,
            ))?;
            Ok(Owning::from_raw(obj_buffers))
        }
    }
}
