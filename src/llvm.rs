use llvm_sys::core::{LLVMGetVersion, LLVMShutdown};
pub use llvm_sys::LLVMCallConv;

/// Deallocate and destroy all ManagedStatic variables.
pub unsafe fn shutdown() {
    unsafe { LLVMShutdown() }
}

/// Return the major, minor, and patch version of LLVM.
pub fn get_version() -> (u32, u32, u32) {
    let mut r = (0, 0, 0);
    unsafe { LLVMGetVersion(&mut r.0, &mut r.1, &mut r.2) };
    r
}
