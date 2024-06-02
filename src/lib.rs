pub use llvm_sys;
use llvm_sys::core::*;

pub mod analysis;
pub mod core;
pub mod error;
pub mod error_handling;
pub mod execution_engine;
pub mod opaque;
pub mod owning;
pub mod support;
pub mod target;
pub mod target_machine;
pub mod transforms;
pub mod type_tag;

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
