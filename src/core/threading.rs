use llvm_sys::core::*;

/// Check whether LLVM is executing in thread-safe mode or not.
pub fn is_multithreaded() -> bool {
    unsafe { LLVMIsMultithreaded() != 0 }
}
