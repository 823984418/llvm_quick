use llvm_quick::Context;

fn main() {
    // Set up a context, module and builder in that context.
    let context = Context::create();
    let module = context.create_module(c"nop");
    let builder = context.create_builder();

    // Get the type signature for void nop(void);
    // Then create it in our module.
    let void = context.void_type();
    let function_type = void.fun(());
    let function = module.add_function(c"nop", function_type);

    // Create a basic block in the function and set our builder to generate
    // code in it.
    let bb = context.append_basic_block(function, c"entry");
    builder.position_at_end(bb);

    // Emit a `ret void` into the function
    builder.return_void();

    // Dump the module as IR to stdout.
    module.dump();
}
