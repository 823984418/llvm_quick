use llvm_sys::execution_engine::*;
use llvm_sys::orc2::ee::*;

use crate::execution_engine::JitEventListener;
use crate::opaque::Opaque;
use crate::orc2::{OrcExecutionSession, OrcObjectLayer};
use crate::owning::Owning;

impl OrcObjectLayer {
    pub fn orc_create_rt_dyld_object_layer_with_section_memory_manager(
        es: &OrcExecutionSession,
    ) -> Owning<OrcObjectLayer> {
        unsafe {
            Owning::from_raw(
                LLVMOrcCreateRTDyldObjectLinkingLayerWithSectionMemoryManager(es.as_ptr()),
            )
        }
    }

    // FIXME: error api
    pub fn orc_create_rt_dyld_object_linking_layer_with_mc_jit_memory_manager_like_callbacks(
        es: &OrcExecutionSession,
        create_context: LLVMMemoryManagerCreateContextCallback,
        notify_terminating: LLVMMemoryManagerNotifyTerminatingCallback,
        allocate_code_section: LLVMMemoryManagerAllocateCodeSectionCallback,
        allocate_data_section: LLVMMemoryManagerAllocateDataSectionCallback,
        finalize_memory: LLVMMemoryManagerFinalizeMemoryCallback,
        destroy: LLVMMemoryManagerDestroyCallback,
    ) -> Owning<OrcObjectLayer> {
        unsafe {
            Owning::from_raw(
                LLVMOrcCreateRTDyldObjectLinkingLayerWithMCJITMemoryManagerLikeCallbacks(
                    es.as_ptr(),
                    create_context,
                    notify_terminating,
                    allocate_code_section,
                    allocate_data_section,
                    finalize_memory,
                    destroy,
                ),
            )
        }
    }

    // FIXME: owner?
    pub fn orc_rt_dyld_object_linking_layer_register_jit_event_listener(
        &self,
        listener: &JitEventListener,
    ) {
        unsafe {
            LLVMOrcRTDyldObjectLinkingLayerRegisterJITEventListener(
                self.as_ptr(),
                listener.as_ptr(),
            )
        }
    }
}
