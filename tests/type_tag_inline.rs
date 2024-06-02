use std::fmt::Write;

use llvm_quick::core::context::Context;
use llvm_quick::core::types::Type;
use llvm_quick::type_tag::TypeTag;

fn check_type<T: TypeTag>(ty: &Type<T>) {
    let mut debug = String::new();
    let f = &mut debug;
    write!(f, "{:?}", ty).unwrap();
    let p = ty.print_to_string();
    println!("{:?}", p);
    assert_eq!(p.to_string_lossy(), debug);
}

#[test]
fn check_type_tag_debug_fmt_inline() {
    let context = Context::create();

    let i64 = context.i64_type();
    let void = context.void_type();
    let ptr = context.pointer_type_in(0);
    let ptr_0 = context.pointer_type::<0>();
    let ptr_1 = context.pointer_type::<1>();
    let fun_i64_void = void.fun((i64,));

    check_type(i64);
    check_type(i64.as_int_any());
    check_type(void);
    check_type(ptr);
    check_type(ptr_0);
    check_type(ptr_1);
    check_type(fun_i64_void);
}
