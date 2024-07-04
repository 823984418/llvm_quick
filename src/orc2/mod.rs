use std::ffi::{c_void, CStr};
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::{null_mut, NonNull};

use llvm_sys::error::LLVMOpaqueError;
use llvm_sys::orc2::*;
use llvm_sys::LLVMModule;

use crate::error::Error;
use crate::owning::{OpaqueClone, OpaqueDrop, Owning};
use crate::target_machine::TargetMachine;
use crate::{Context, MemoryBuffer, Module, Opaque, PhantomOpaque};

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

#[allow(dead_code)]
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

unsafe impl Send for OrcThreadSafeContext {}
unsafe impl Sync for OrcThreadSafeContext {}

unsafe impl Opaque for OrcThreadSafeContext {
    type Inner = LLVMOrcOpaqueThreadSafeContext;
}

#[repr(transparent)]
pub struct OrcThreadSafeModule<'c> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'c OrcThreadSafeContext>,
}

unsafe impl<'c> Opaque for OrcThreadSafeModule<'c> {
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[repr(C)]
pub struct OrcCSymbolFlagsMapPair<'e> {
    pub name: &'e OrcSymbolStringPoolEntry,
    pub flags: LLVMJITSymbolFlags,
}

impl OrcMaterializationUnit {
    pub fn create_custom_raw(
        name: &CStr,
        ctx: &mut (),
        syms: &[&OrcCSymbolFlagsMapPair],
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

pub struct CSymbolFlagsMap<'e> {
    ptr: NonNull<[OrcCSymbolFlagsMapPair<'e>]>,
}

impl<'e> Deref for CSymbolFlagsMap<'e> {
    type Target = [OrcCSymbolFlagsMapPair<'e>];

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<'e> Drop for CSymbolFlagsMap<'e> {
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

impl OrcMaterializationResponsibility {
    pub fn notify_resolved(
        &self,
        symbols: &[&OrcSymbolStringPoolEntry],
    ) -> Result<(), Owning<Error>> {
        unsafe {
            Error::check(LLVMOrcMaterializationResponsibilityNotifyResolved(
                self.as_raw(),
                symbols.as_ptr() as _,
                symbols.len(),
            ))
        }
    }

    pub fn notify_emitted(&self) -> Result<(), Owning<Error>> {
        unsafe {
            Error::check(LLVMOrcMaterializationResponsibilityNotifyEmitted(
                self.as_raw(),
            ))
        }
    }

    pub fn define_materializing(
        &self,
        pairs: &[&OrcCSymbolFlagsMapPair],
    ) -> Result<(), Owning<Error>> {
        unsafe {
            Error::check(LLVMOrcMaterializationResponsibilityDefineMaterializing(
                self.as_raw(),
                pairs.as_ptr() as _,
                pairs.len(),
            ))
        }
    }

    pub unsafe fn fail_materialization(&self) {
        unsafe { LLVMOrcMaterializationResponsibilityFailMaterialization(self.as_raw()) }
    }

    pub unsafe fn replace(&self, mu: &OrcMaterializationUnit) -> Result<(), Owning<Error>> {
        unsafe {
            Error::check(LLVMOrcMaterializationResponsibilityReplace(
                self.as_raw(),
                mu.as_raw(),
            ))
        }
    }

    pub fn delegate(
        &self,
        symbols: &[&OrcSymbolStringPoolEntry],
    ) -> Result<&OrcMaterializationResponsibility, Owning<Error>> {
        unsafe {
            let mut result = null_mut();
            Error::check(LLVMOrcMaterializationResponsibilityDelegate(
                self.as_raw(),
                symbols.as_ptr() as _,
                symbols.len(),
                &mut result,
            ))?;
            Ok(OrcMaterializationResponsibility::from_raw(result))
        }
    }
}

#[repr(C)]
pub struct OrcCDependenceMapPair<'e> {
    pub jit_dylib: &'e OrcJitDylib,
    pub names: &'e [&'e OrcSymbolStringPoolEntry],
}

impl OrcMaterializationResponsibility {
    pub fn add_dependencies(
        &self,
        name: &OrcSymbolStringPoolEntry,
        dependencies: &[OrcCDependenceMapPair],
    ) {
        unsafe {
            LLVMOrcMaterializationResponsibilityAddDependencies(
                self.as_raw(),
                name.as_raw(),
                dependencies.as_ptr() as _,
                dependencies.len(),
            )
        }
    }

    pub fn add_dependencies_for_all(&self, dependencies: &[OrcCDependenceMapPair]) {
        unsafe {
            LLVMOrcMaterializationResponsibilityAddDependenciesForAll(
                self.as_raw(),
                dependencies.as_ptr() as _,
                dependencies.len(),
            )
        }
    }
}

impl OrcExecutionSession {
    pub fn create_bare_jit_dylib(&self, name: &CStr) -> &OrcJitDylib {
        unsafe {
            OrcJitDylib::from_raw(LLVMOrcExecutionSessionCreateBareJITDylib(
                self.as_raw(),
                name.as_ptr(),
            ))
        }
    }

    pub fn create_jit_dylib(&self, name: &CStr) -> Result<&OrcJitDylib, Owning<Error>> {
        unsafe {
            let mut result = null_mut();
            Error::check(LLVMOrcExecutionSessionCreateJITDylib(
                self.as_raw(),
                &mut result,
                name.as_ptr(),
            ))?;
            Ok(OrcJitDylib::from_raw(result))
        }
    }

    pub fn get_jit_dylib_by_name(&self, name: &CStr) -> Option<&OrcJitDylib> {
        unsafe {
            OrcJitDylib::try_from_raw(LLVMOrcExecutionSessionGetJITDylibByName(
                self.as_raw(),
                name.as_ptr(),
            ))
        }
    }
}

impl OrcJitDylib {
    pub fn create_resource_tracker(&self) -> &OrcResourceTracker {
        unsafe { OrcResourceTracker::from_raw(LLVMOrcJITDylibCreateResourceTracker(self.as_raw())) }
    }

    pub fn get_default_resource_tracker(&self) -> &OrcResourceTracker {
        unsafe {
            OrcResourceTracker::from_raw(LLVMOrcJITDylibGetDefaultResourceTracker(self.as_raw()))
        }
    }

    pub fn define(&self, mu: &OrcMaterializationUnit) -> Result<(), Owning<Error>> {
        unsafe { Error::check(LLVMOrcJITDylibDefine(self.as_raw(), mu.as_raw())) }
    }

    pub unsafe fn clear(&self) -> Result<(), Owning<Error>> {
        unsafe { Error::check(LLVMOrcJITDylibClear(self.as_raw())) }
    }

    pub fn add_generator(&self, dg: &OrcDefinitionGenerator) {
        unsafe { LLVMOrcJITDylibAddGenerator(self.as_raw(), dg.as_raw()) }
    }
}

impl OrcDefinitionGenerator {
    pub fn create_custom_c_api_definition_generator_raw(
        f: LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction,
        ctx: *mut (),
        dispose: LLVMOrcDisposeCAPIDefinitionGeneratorFunction,
    ) -> Owning<OrcDefinitionGenerator> {
        unsafe {
            Owning::from_raw(LLVMOrcCreateCustomCAPIDefinitionGenerator(
                f, ctx as _, dispose,
            ))
        }
    }
}

impl OrcLookupState {
    pub fn continue_lookup(&self, error: Option<Owning<Error>>) {
        unsafe {
            LLVMOrcLookupStateContinueLookup(
                self.as_raw(),
                error.map(Owning::into_raw).unwrap_or(null_mut()),
            )
        }
    }
}

impl OrcDefinitionGenerator {
    pub fn create_dynamic_library_search_generator_for_process_raw(
        global_prefix: u8,
        filter: LLVMOrcSymbolPredicate,
        ctx: *mut (),
    ) -> Result<Owning<OrcDefinitionGenerator>, Owning<Error>> {
        unsafe {
            let mut result = null_mut();
            Error::check(LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess(
                &mut result,
                global_prefix as _,
                filter,
                ctx as _,
            ))?;
            Ok(Owning::from_raw(result))
        }
    }

    pub fn create_dynamic_library_search_generator_for_path_raw(
        file_name: &CStr,
        global_prefix: u8,
        filter: LLVMOrcSymbolPredicate,
        ctx: *mut (),
    ) -> Result<Owning<OrcDefinitionGenerator>, Owning<Error>> {
        unsafe {
            let mut result = null_mut();
            Error::check(LLVMOrcCreateDynamicLibrarySearchGeneratorForPath(
                &mut result,
                file_name.as_ptr(),
                global_prefix as _,
                filter,
                ctx as _,
            ))?;
            Ok(Owning::from_raw(result))
        }
    }

    #[inline(always)]
    pub fn create_static_library_search_generator_for_path_raw(
        obj_layer: &OrcObjectLayer,
        file_name: &CStr,
        target_triple: &CStr,
    ) -> Result<Owning<OrcDefinitionGenerator>, Owning<Error>> {
        unsafe {
            let mut result = null_mut();
            Error::check(LLVMOrcCreateStaticLibrarySearchGeneratorForPath(
                &mut result,
                obj_layer.as_raw(),
                file_name.as_ptr(),
                target_triple.as_ptr(),
            ))?;
            Ok(Owning::from_raw(result))
        }
    }
}

impl OrcThreadSafeContext {
    pub fn create() -> Owning<OrcThreadSafeContext> {
        unsafe { Owning::from_raw(LLVMOrcCreateNewThreadSafeContext()) }
    }

    pub fn get_context(&self) -> &Context {
        unsafe { Context::from_raw(LLVMOrcThreadSafeContextGetContext(self.as_raw())) }
    }
}

impl OpaqueDrop for LLVMOrcOpaqueThreadSafeContext {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeThreadSafeContext(ptr) }
    }
}
impl OrcThreadSafeContext {
    pub fn create_module(&self, m: Owning<Module>) -> Owning<OrcThreadSafeModule<'_>> {
        unsafe {
            Owning::from_raw(LLVMOrcCreateNewThreadSafeModule(
                m.into_raw(),
                self.as_raw(),
            ))
        }
    }
}

impl OpaqueDrop for LLVMOrcOpaqueThreadSafeModule {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeThreadSafeModule(ptr) }
    }
}

impl<'c> OrcThreadSafeModule<'c> {
    pub fn with_module_do_raw(
        &self,
        f: LLVMOrcGenericIRModuleOperationFunction,
        ctx: *mut (),
    ) -> Result<(), Owning<Error>> {
        unsafe {
            Error::check(LLVMOrcThreadSafeModuleWithModuleDo(
                self.as_raw(),
                f,
                ctx as _,
            ))
        }
    }

    pub fn with_module_do<F: FnOnce(&Module) -> Result<R, Owning<Error>>, R>(
        &self,
        f: F,
    ) -> Result<R, Owning<Error>> {
        struct Ctx<F: FnOnce(&Module) -> Result<R, Owning<Error>>, R> {
            f: Option<F>,
            r: Option<R>,
        }
        let mut ctx = Ctx::<F, R> {
            f: Some(f),
            r: None,
        };

        extern "C" fn operation<F: FnOnce(&Module) -> Result<R, Owning<Error>>, R>(
            ctx: *mut c_void,
            m: *mut LLVMModule,
        ) -> *mut LLVMOpaqueError {
            unsafe {
                let ctx = &mut *(ctx as *mut Ctx<F, R>);
                match ctx.f.take().unwrap()(Module::from_raw(m)) {
                    Ok(r) => {
                        ctx.r = Some(r);
                        null_mut()
                    }
                    Err(e) => e.into_raw(),
                }
            }
        }

        self.with_module_do_raw(operation::<F, R>, &mut ctx as *mut _ as *mut _)?;
        Ok(ctx.r.unwrap())
    }
}

impl OrcJitTargetMachineBuilder {
    pub fn detect_host() -> Result<Owning<OrcJitTargetMachineBuilder>, Owning<Error>> {
        unsafe {
            let mut result = null_mut();
            Error::check(LLVMOrcJITTargetMachineBuilderDetectHost(&mut result))?;
            Ok(Owning::from_raw(result))
        }
    }

    pub fn create_from_target_machine(tm: &TargetMachine) -> Owning<OrcJitTargetMachineBuilder> {
        unsafe {
            Owning::from_raw(LLVMOrcJITTargetMachineBuilderCreateFromTargetMachine(
                tm.as_raw(),
            ))
        }
    }
}

impl OpaqueDrop for LLVMOrcOpaqueJITTargetMachineBuilder {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeJITTargetMachineBuilder(ptr) }
    }
}

impl OrcJitTargetMachineBuilder {
    pub fn get_target_triple(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMOrcJITTargetMachineBuilderGetTargetTriple(self.as_raw())) }
    }

    pub unsafe fn set_target_triple(&self, target_triple: &CStr) {
        unsafe {
            LLVMOrcJITTargetMachineBuilderSetTargetTriple(self.as_raw(), target_triple.as_ptr())
        }
    }
}

impl OrcObjectLayer {
    pub fn add_object_file(
        &self,
        jd: &OrcJitDylib,
        obj_buffer: &MemoryBuffer,
    ) -> Result<(), Owning<Error>> {
        unsafe {
            Error::check(LLVMOrcObjectLayerAddObjectFile(
                self.as_raw(),
                jd.as_raw(),
                obj_buffer.as_raw(),
            ))
        }
    }

    pub fn add_object_file_with_rt(
        &self,
        rt: &OrcResourceTracker,
        obj_buffer: &MemoryBuffer,
    ) -> Result<(), Owning<Error>> {
        unsafe {
            Error::check(LLVMOrcObjectLayerAddObjectFileWithRT(
                self.as_raw(),
                rt.as_raw(),
                obj_buffer.as_raw(),
            ))
        }
    }

    pub fn emit(&self, r: &OrcMaterializationResponsibility, obj_buffer: &MemoryBuffer) {
        unsafe { LLVMOrcObjectLayerEmit(self.as_raw(), r.as_raw(), obj_buffer.as_raw()) }
    }
}

impl OpaqueDrop for LLVMOrcOpaqueObjectLayer {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeObjectLayer(ptr) }
    }
}

impl OrcIrTransformLayer {
    pub fn emit(&self, mr: &OrcMaterializationResponsibility, tsm: &OrcThreadSafeModule) {
        unsafe { LLVMOrcIRTransformLayerEmit(self.as_raw(), mr.as_raw(), tsm.as_raw()) }
    }

    pub fn set_transform_raw(
        &self,
        transform: LLVMOrcIRTransformLayerTransformFunction,
        ctx: *mut (),
    ) {
        unsafe { LLVMOrcIRTransformLayerSetTransform(self.as_raw(), transform, ctx as _) }
    }
}

impl OrcObjectTransformLayer {
    pub fn set_transform_raw(
        &self,
        transform: LLVMOrcObjectTransformLayerTransformFunction,
        ctx: *mut (),
    ) {
        unsafe { LLVMOrcObjectTransformLayerSetTransform(self.as_raw(), transform, ctx as _) }
    }
}

impl OrcIndirectStubsManager {
    pub fn create_local_indirect_stubs_manager(
        target_triple: &CStr,
    ) -> Owning<OrcIndirectStubsManager> {
        unsafe {
            Owning::from_raw(LLVMOrcCreateLocalIndirectStubsManager(
                target_triple.as_ptr(),
            ))
        }
    }
}

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
