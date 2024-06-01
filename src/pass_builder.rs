use std::ffi::CStr;

use llvm_sys::transforms::pass_builder::*;

use crate::error::Error;
use crate::module::Module;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::{Dispose, Owning};
use crate::target_machine::TargetMachine;

pub struct PassBuilderOptions {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for PassBuilderOptions {
    type Inner = LLVMOpaquePassBuilderOptions;
}

impl Dispose for PassBuilderOptions {
    unsafe fn dispose(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposePassBuilderOptions(ptr) };
    }
}

impl PassBuilderOptions {
    pub fn create() -> Owning<Self> {
        unsafe { Owning::from_raw(LLVMCreatePassBuilderOptions()) }
    }

    pub fn set_verify_each(&self, v: bool) {
        unsafe { LLVMPassBuilderOptionsSetVerifyEach(self.as_ptr(), v as _) };
    }

    pub fn set_debug_logging(&self, v: bool) {
        unsafe { LLVMPassBuilderOptionsSetDebugLogging(self.as_ptr(), v as _) };
    }

    pub fn set_loop_interleaving(&self, v: bool) {
        unsafe { LLVMPassBuilderOptionsSetLoopInterleaving(self.as_ptr(), v as _) };
    }

    pub fn set_loop_vectorization(&self, v: bool) {
        unsafe { LLVMPassBuilderOptionsSetLoopVectorization(self.as_ptr(), v as _) };
    }

    pub fn set_slp_vectorization(&self, v: bool) {
        unsafe { LLVMPassBuilderOptionsSetSLPVectorization(self.as_ptr(), v as _) };
    }

    pub fn set_loop_unrolling(&self, v: bool) {
        unsafe { LLVMPassBuilderOptionsSetLoopUnrolling(self.as_ptr(), v as _) };
    }

    pub fn set_forget_all_scevin_loop_unroll(&self, v: bool) {
        unsafe { LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll(self.as_ptr(), v as _) };
    }

    pub fn set_licm_mssa_opt_cap(&self, v: u32) {
        unsafe { LLVMPassBuilderOptionsSetLicmMssaOptCap(self.as_ptr(), v) };
    }

    pub fn set_licm_mssa_no_acc_for_promotion_cap(&self, v: u32) {
        unsafe { LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap(self.as_ptr(), v) };
    }

    pub fn set_call_graph_profile(&self, v: bool) {
        unsafe { LLVMPassBuilderOptionsSetCallGraphProfile(self.as_ptr(), v as _) };
    }

    pub fn set_merge_functions(&self, v: bool) {
        unsafe { LLVMPassBuilderOptionsSetMergeFunctions(self.as_ptr(), v as _) };
    }

    pub fn set_inliner_threshold(&self, v: i32) {
        unsafe { LLVMPassBuilderOptionsSetInlinerThreshold(self.as_ptr(), v) };
    }
}

impl<'s> Module<'s> {
    pub fn run_pass(
        &self,
        passes: &CStr,
        target_machine: &TargetMachine,
        options: &PassBuilderOptions,
    ) -> Result<(), Owning<Error>> {
        unsafe {
            Error::check(LLVMRunPasses(
                self.as_ptr(),
                passes.as_ptr(),
                target_machine.as_ptr(),
                options.as_ptr(),
            ))
        }
    }
}
