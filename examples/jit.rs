use llvm_sys::target_machine::LLVMCodeModel;

use llvm_quick::builder::Builder;
use llvm_quick::core::context::Context;
use llvm_quick::execution_engine::{link_in_mc_jit, ExecutionEngine, MCJITCompilerOptions};
use llvm_quick::owning::Owning;
use llvm_quick::target::{
    initialize_all_target_infos, initialize_native_asm_parser, initialize_native_asm_printer,
    initialize_native_disassembler, initialize_native_target,
};
use llvm_quick::target_machine::Target;

type SumFunc = unsafe extern "C" fn(i64, i64, i64) -> i64;

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

        let sum = self.builder.add(x, y, c"sum");
        let sum = self.builder.add(sum, z, c"sum");

        self.builder.return_value(sum);

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
    initialize_native_target();
    initialize_native_asm_printer();
    initialize_native_asm_parser();
    initialize_native_disassembler();

    initialize_all_target_infos();
    for i in Target::iter_all() {
        println!("{:?}", i.get_name());
    }

    let context = Context::create();

    context.set_diagnostic_handler(|info| {
        println!("{:?}", info.get_description());
    });

    let codegen = CodeGen {
        context: &context,
        builder: context.create_builder(),
    };

    let execution_engine = codegen.jit_compile_sum();

    let f = execution_engine.get_function_address(c"sum");
    let sum: Option<SumFunc> = unsafe { std::mem::transmute(f as usize) };
    let sum = sum.unwrap();

    let x = 1i64;
    let y = 2i64;
    let z = 3i64;

    unsafe {
        println!("{} + {} + {} = {}", x, y, z, sum(x, y, z));
        assert_eq!(sum(x, y, z), x + y + z);
    }
}

// fn main() {}
