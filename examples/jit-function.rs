use llvm_quick::execution_engine::{link_in_mc_jit, ExecutionEngine};
use llvm_quick::target::{initialize_native_asm_printer, initialize_native_target};
use llvm_quick::Context;

fn main() {
    // Set up a context, module and builder in that context.
    let context = Context::create();
    let module = context.create_module(c"sum");
    let builder = context.create_builder();

    // get a type for sum function
    let i64_type = context.i64_type();
    let function_type = i64_type.fun((i64_type, i64_type, i64_type));

    // add it to our module
    let function = module.add_function(c"sum", function_type);

    // Create a basic block in the function and set our builder to generate
    // code in it.
    let bb = context.append_basic_block(function, c"entry");

    builder.position_at_end(bb);

    // get the function's arguments
    let (x, y, z) = function.get_params();

    let sum = builder.add(x, y, c"sum.1");
    let sum = builder.add(sum, z, c"sum.2");

    // Emit a `ret i64` into the function to return the computed sum.
    builder.return_value(sum);

    // done building
    drop(builder);

    // Dump the module as IR to stdout.
    module.dump();

    // Robust code should check that these calls complete successfully.
    // Each of calls is necessary to setup an execution engine which
    // compiles to native code.
    link_in_mc_jit();
    initialize_native_target();
    initialize_native_asm_printer();

    // Build an execution engine.
    let ee = ExecutionEngine::create_execution_engine_for_module(module)
        .expect("Failed to create execution engine:");

    let addr = ee.get_function_address(c"sum");

    let f =
        unsafe { std::mem::transmute::<u64, Option<extern "C" fn(u64, u64, u64) -> u64>>(addr) }
            .unwrap();

    let x: u64 = 1;
    let y: u64 = 1;
    let z: u64 = 1;
    let res = f(x, y, z);

    println!("{} + {} + {} = {}", x, y, z, res);

    // Clean up the rest.
    drop(ee);
    drop(context);
}
