use std::ffi::CStr;

use llvm_sys::core::{LLVMGetVersion, LLVMShutdown};
use llvm_sys::support::*;

/// This functions permanently adds the symbol symbolName with the value symbolValue.
/// These symbols are searched before any libraries.
pub fn add_symbol(name: &CStr, value: *mut ()) {
    unsafe { LLVMAddSymbol(name.as_ptr(), value.cast()) }
}

/// Return the major, minor, and patch version of LLVM.
pub fn get_version() -> (u32, u32, u32) {
    let mut r = (0, 0, 0);
    unsafe { LLVMGetVersion(&mut r.0, &mut r.1, &mut r.2) };
    r
}

/// This function permanently loads the dynamic library at the given path.
///
/// It is safe to call this function multiple times for the same library.
pub fn load_library_permanently(filename: &CStr) -> bool {
    unsafe { LLVMLoadLibraryPermanently(filename.as_ptr()) != 0 }
}

/// This function parses the given arguments using the LLVM command line parser.
///
/// Note that the only stable thing about this function is its signature;
/// you cannot rely on any particular set of command line arguments being
/// interpreted the same way across LLVM versions.
pub fn parse_command_line_options(args: &[*const u8], over: &CStr) {
    unsafe { LLVMParseCommandLineOptions(args.len() as _, args.as_ptr().cast(), over.as_ptr()) }
}

/// This function will search through all previously loaded dynamic libraries for the symbol symbolName.
/// If it is found, the address of that symbol is returned. If not, null is returned.
pub fn search_for_address_of_symbol(name: &CStr) -> *mut () {
    unsafe { LLVMSearchForAddressOfSymbol(name.as_ptr()).cast() }
}

/// Deallocate and destroy all ManagedStatic variables.
pub unsafe fn shutdown() {
    unsafe { LLVMShutdown() }
}
