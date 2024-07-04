# llvm_quick
Rust's wrapper for llvm.

The wrapping should be as thin and fast as possible, 
and for APIs that cannot guarantee safety at the same time,
provide safety for reading as much as possible.

If you find any potential unsafe aspects of APIs or have any abstract suggestions, please open an Issues.
