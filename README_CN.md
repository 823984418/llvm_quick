# llvm_quick
Rust的llvm封装，基于[llvm-sys](https://crates.io/crates/llvm-sys)。

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
封装应尽可能薄且快，大多数情况下仅仅将C风格API包装为关联方法。

对于不能同时保证安全性的API，尽可能保证读取访问是安全的，在这种情况下，写入函数将被标记为不安全的。

## 反馈和建议
如果您发现API的任何潜在不安全性或有任何抽象方面的建议，请打开一个问题。

请注意：本库仅仅是对[llvm-sys](https://crates.io/crates/llvm-sys)的封装，
本身并不关心如何链接LLVM，因此，如果您在链接llvm时遇到了困难，请前往
[gitlab llvm-sys](https://gitlab.com/taricorp/llvm-sys.rs)
请求帮助。
