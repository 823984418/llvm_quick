use std::fmt::Formatter;

use llvm_sys::*;
use llvm_sys::core::*;

use crate::core::types::Type;
use crate::core::values::Value;
use crate::opaque::Opaque;
use crate::type_tag::array_tag::array_sized;
use crate::type_tag::float_tag::{bfloat, double, float, fp128, half, ppc_fp128, x86_fp80};
use crate::type_tag::function_tag::fun;
use crate::type_tag::integer_tag::int;
use crate::type_tag::pointer_tag::ptr;

pub mod array_tag;
pub mod float_tag;
pub mod function_tag;
pub mod integer_tag;
pub mod pointer_tag;

pub(crate) unsafe fn type_check_kind<T: TypeTag>(
    ty: &Type<any>,
    kind: LLVMTypeKind,
) -> Option<&Type<T>> {
    if ty.kind() == kind {
        Some(unsafe { ty.cast_unchecked() })
    } else {
        None
    }
}

pub trait TypeTag: Copy + 'static {
    fn type_debug_fmt(ty: &Type<Self>, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(ty.print_to_string().to_str().unwrap())
    }

    fn type_kind(ty: &Type<Self>) -> LLVMTypeKind;

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>>;

    fn value_debug_fmt(val: &Value<Self>, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(val.print_to_string().to_str().unwrap())
    }
}

pub trait InstanceTypeTag: TypeTag {}
impl InstanceTypeTag for void {}
impl InstanceTypeTag for label {}
impl<T: InstanceTypeTag, const N: u64> InstanceTypeTag for array_sized<T, N> {}
impl InstanceTypeTag for half {}
impl InstanceTypeTag for float {}
impl InstanceTypeTag for double {}
impl InstanceTypeTag for x86_fp80 {}
impl InstanceTypeTag for fp128 {}
impl InstanceTypeTag for ppc_fp128 {}
impl InstanceTypeTag for bfloat {}
impl<const N: u32> InstanceTypeTag for int<N> {}
impl<const ADDRESS_SPACE: u32> InstanceTypeTag for ptr<ADDRESS_SPACE> {}
impl<Args: InstanceTagTuple, Output: InstanceTypeTag, const VAR: bool> InstanceTypeTag
    for fun<Args, Output, VAR>
{
}

pub trait IntMathTypeTag: InstanceTypeTag {}
impl<const N: u32> IntMathTypeTag for int<N> {}

pub trait FloatMathTypeTag: InstanceTypeTag {}
impl FloatMathTypeTag for half {}
impl FloatMathTypeTag for float {}
impl FloatMathTypeTag for double {}
impl FloatMathTypeTag for x86_fp80 {}
impl FloatMathTypeTag for fp128 {}
impl FloatMathTypeTag for ppc_fp128 {}
impl FloatMathTypeTag for bfloat {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct any {}

impl TypeTag for any {
    fn type_kind(ty: &Type<Self>) -> LLVMTypeKind {
        unsafe { LLVMGetTypeKind(ty.as_ptr()) }
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        Some(ty)
    }
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct void {}

impl TypeTag for void {
    fn type_debug_fmt(_ty: &Type<Self>, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("void")
    }

    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMVoidTypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMVoidTypeKind) }
    }
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct label {}

impl TypeTag for label {
    fn type_debug_fmt(_ty: &Type<Self>, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("label")
    }

    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMLabelTypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMLabelTypeKind) }
    }
}

pub trait TagTuple: Copy + 'static {
    const COUNT: usize;

    type Types<'s>: TypeTuple<'s, Tags = Self>;

    type Values<'s>: ValueTuple<'s, Tags = Self>;

    fn stack_array<Type: Default, Fun: FnOnce(&mut [Type]) -> Ret, Ret>(f: Fun) -> Ret;

    fn type_into_slice<'a, 's>(
        tuple: Self::Types<'s>,
        slice: &'a mut [Option<&'s Type<any>>],
    ) -> &'a mut [&'s Type<any>];

    fn type_from_slice<'s>(slice: &[&'s Type<any>]) -> Option<Self::Types<'s>>;

    fn value_into_slice<'a, 's>(
        tuple: Self::Values<'s>,
        slice: &'a mut [Option<&'s Value<any>>],
    ) -> &'a mut [&'s Value<any>];

    fn value_from_slice<'s>(slice: &[&'s Value<any>]) -> Option<Self::Values<'s>>;
}

pub trait InstanceTagTuple: TagTuple {}

pub trait TypeTuple<'s>: Sized {
    type Tags: TagTuple<Types<'s> = Self>;
}

pub trait ValueTuple<'s>: Sized {
    type Tags: TagTuple<Values<'s> = Self>;
}

macro_rules! impl_tuple {
    ($count:literal $(,$arg:ident)*) => {
        impl<$($arg: TypeTag),*> TagTuple for ($($arg,)*) {
            const COUNT: usize = $count;

            type Types<'s> = ($(&'s Type<$arg>,)*);

            type Values<'s> = ($(&'s Value<$arg>,)*);

            fn stack_array<Type: Default, Fun: FnOnce(&mut [Type]) -> Ret, Ret>(f: Fun) -> Ret {
                let mut alloc: [Type; $count] = std::array::from_fn(|_| Type::default());
                f(&mut alloc)
            }

            #[allow(non_snake_case)]
            fn type_into_slice<'a, 's>(
                tuple: Self::Types<'s>,
                slice: &'a mut [Option<&'s Type<any>>],
            ) -> &'a mut [&'s Type<any>] {
                let ($($arg,)*) = tuple;
                slice.copy_from_slice(&[$(Some($arg.to_any()),)*]);
                unsafe { std::mem::transmute(slice) }
            }

            #[allow(non_snake_case)]
            fn type_from_slice<'s>(slice: &[&'s Type<any>]) -> Option<Self::Types<'s>> {
                if let &[$($arg,)*] = slice {
                    Some(($($arg.try_cast()?,)*))
                } else {
                    None
                }
            }

            #[allow(non_snake_case)]
            fn value_into_slice<'a, 's>(
                tuple: Self::Values<'s>,
                slice: &'a mut [Option<&'s Value<any>>],
            ) -> &'a mut [&'s Value<any>] {
                let ($($arg,)*) = tuple;
                slice.copy_from_slice(&[$(Some($arg.to_any()),)*]);
                unsafe { std::mem::transmute(slice) }
            }

            #[allow(non_snake_case)]
            fn value_from_slice<'s>(slice: &[&'s Value<any>]) -> Option<Self::Values<'s>> {
                if let &[$($arg,)*] = slice {
                    Some(($($arg.try_cast()?,)*))
                } else {
                    None
                }
            }
        }

        impl<$($arg: InstanceTypeTag),*> InstanceTagTuple for ($($arg,)*) {}

        impl<'s, $($arg: TypeTag),*> TypeTuple<'s> for ($(&'s Type<$arg>,)*) {
            type Tags = ($($arg,)*);
        }

        impl<'s, $($arg: TypeTag),*> ValueTuple<'s> for ($(&'s Value<$arg>,)*) {
            type Tags = ($($arg,)*);
        }
    };
}

impl_tuple!(0x00);
impl_tuple!(0x01, A);
impl_tuple!(0x02, A, B);
impl_tuple!(0x03, A, B, C);
impl_tuple!(0x04, A, B, C, D);
impl_tuple!(0x05, A, B, C, D, E);
impl_tuple!(0x06, A, B, C, D, E, F);
impl_tuple!(0x07, A, B, C, D, E, F, G);
impl_tuple!(0x08, A, B, C, D, E, F, G, H);
impl_tuple!(0x09, A, B, C, D, E, F, G, H, I);
impl_tuple!(0x0A, A, B, C, D, E, F, G, H, I, J);
impl_tuple!(0x0B, A, B, C, D, E, F, G, H, I, J, K);
impl_tuple!(0x0C, A, B, C, D, E, F, G, H, I, J, K, L);
impl_tuple!(0x0D, A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_tuple!(0x0E, A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_tuple!(0x0F, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_tuple!(0x10, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_tuple!(0x11, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_tuple!(0x12, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_tuple!(0x13, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_tuple!(0x14, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
impl_tuple!(0x15, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
impl_tuple!(0x16, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
impl_tuple!(0x17, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
impl_tuple!(0x18, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
impl_tuple!(0x19, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
impl_tuple!(0x1A, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
