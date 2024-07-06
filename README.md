# llvm_quick

[中文](./README_CN.md)

Rust wrapper around LLVM, based on [llvm-sys](https://crates.io/crates/llvm-sys)。

## 示例

```
let context = Context::create();
let module = context.create_module(c"sum");
let builder = context.create_builder();

let i64_type = context.i64_type();
let function_type = i64_type.fun((i64_type, i64_type, i64_type));

let function = module.add_function(c"sum", function_type);

let bb = context.append_basic_block(function, c"entry");

builder.position_at_end(bb);

let (x, y, z) = function.get_params();

let sum = builder.add(x, y, c"sum.1");
let sum = builder.add(sum, z, c"sum.2");

builder.return_value(sum);
```

## 特性

The wrapper aims to be as thin and fast as possible, mostly just wrapping the C-style
API as associated methods.

Where APIs cannot safely be made safe, read-only access is made safe whenever possible,
and write functions are marked as unsafe in those cases.

## 反馈和建议

If you find any potential unsafe in the API or have suggestions for abstraction, please open an issue.

Note: This library is merely a wrapper around llvm-sys and does not concern itself with linking LLVM. If you have
difficulties linking llvm, please head over to gitlab llvm-sys for assistance.
Note: This library is merely a wrapper around [llvm-sys](https://crates.io/crates/llvm-sys)
and does not concern itself with linking LLVM. If you have difficulties linking, please head over to
[gitlab llvm-sys](https://gitlab.com/taricorp/llvm-sys.rs) for assistance.
