use std::ffi::{c_void, CStr};
use std::ptr::null_mut;

use llvm_sys::core::{LLVMContextSetDiagnosticHandler, LLVMGetDiagInfoDescription};
use llvm_sys::execution_engine::LLVMLinkInInterpreter;
use llvm_sys::prelude::LLVMDiagnosticInfoRef;
use llvm_sys::target::{
    LLVM_InitializeNativeAsmParser, LLVM_InitializeNativeAsmPrinter,
    LLVM_InitializeNativeDisassembler, LLVM_InitializeNativeTarget,
};
use llvm_sys::target_machine::LLVMCodeModel;

use llvm_quick::builder::Builder;
use llvm_quick::context::Context;
use llvm_quick::execution_engine::{link_in_mc_jit, ExecutionEngine, MCJITCompilerOptions};
use llvm_quick::opaque::Opaque;
use llvm_quick::owning::Owning;

type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    builder: Owning<Builder<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    fn jit_compile_sum(&self) -> Owning<ExecutionEngine> {
        let module = self.context.create_module(c"sum");
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fun((i64_type, i64_type, i64_type));
        let function = module.add_function(c"sum", fn_type);
        let basic_block = self.context.append_basic_block(function, c"entry");

        self.builder.position_at_end(basic_block);

        let (x, y, z) = function.get_params();

        let sum = self.builder.build_int_add(x, y, c"sum");
        let sum = self.builder.build_int_add(sum, z, c"sum");

        let r = self.builder.build_return(sum);

        println!("{:?}", module);
        println!("{:?}", function.get_name());

        ExecutionEngine::create_mc_jit_compiler_for_module(
            module,
            MCJITCompilerOptions {
                opt_level: 3,
                code_model: LLVMCodeModel::LLVMCodeModelJITDefault,
                no_frame_pointer_elim: false,
                enable_fast_instruction_select: false,
                mc_jit_memory_manager: None,
            },
        )
        .unwrap()
    }
}

fn main() {
    link_in_mc_jit();
    unsafe {
        LLVM_InitializeNativeTarget();
        LLVM_InitializeNativeAsmPrinter();
        LLVM_InitializeNativeAsmParser();
        LLVM_InitializeNativeDisassembler();
    }

    let context = Context::create();

    unsafe {
        extern "C" fn handle(info: LLVMDiagnosticInfoRef, _this: *mut c_void) {
            unsafe {
                let des = LLVMGetDiagInfoDescription(info);
                println!("{}", CStr::from_ptr(des).to_str().unwrap());
            }
        }

        LLVMContextSetDiagnosticHandler(context.as_ptr(), Some(handle), null_mut());
    }
    let codegen = CodeGen {
        context: &context,
        builder: context.create_builder(),
    };

    let execution_engine = codegen.jit_compile_sum();

    let f = execution_engine.get_function_address(c"sum");
    let sum: Option<SumFunc> = unsafe { std::mem::transmute(f as usize) };
    let sum = sum.unwrap();

    let x = 1u64;
    let y = 2u64;
    let z = 3u64;

    unsafe {
        println!("{} + {} + {} = {}", x, y, z, sum(x, y, z));
        assert_eq!(sum(x, y, z), x + y + z);
    }
}

// fn main() {}
