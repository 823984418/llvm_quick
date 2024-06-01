pub use llvm_sys;
use llvm_sys::core::*;

pub mod analysis;
pub mod basic_block;
pub mod builder;
pub mod context;
pub mod diagnostic;
pub mod error;
pub mod execution_engine;
pub mod memory_buffer;
pub mod message;
pub mod metadata;
pub mod module;
pub mod opaque;
pub mod owning;
pub mod pass_builder;
pub mod support;
pub mod target;
pub mod target_machine;
pub mod type_tag;
pub mod types;
pub mod values;

/// Return the major, minor, and patch version of LLVM.
pub fn get_version() -> (u32, u32, u32) {
    let mut r = (0, 0, 0);
    unsafe { LLVMGetVersion(&mut r.0, &mut r.1, &mut r.2) };
    r
}

/// Deallocate and destroy all ManagedStatic variables.
pub unsafe fn shutdown() {
    unsafe { LLVMShutdown() }
}

/// Check whether LLVM is executing in thread-safe mode or not.
pub fn is_multithreaded() -> bool {
    unsafe { LLVMIsMultithreaded() != 0 }
}
