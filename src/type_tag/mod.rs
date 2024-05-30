use llvm_sys::core::LLVMGetTypeKind;
use llvm_sys::LLVMTypeKind;
use crate::opaque::Opaque;

use crate::types::Type;
use crate::values::Value;

pub mod array;
pub mod float;
pub mod function;
pub mod integer;
pub mod pointer;

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
    fn type_kind(ty: &Type<Self>) -> LLVMTypeKind;

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>>;
}

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
    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMVoidTypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMVoidTypeKind) }
    }
}

pub trait TagTuple: Copy + 'static {
    type Types<'s>: TypeTuple<'s, Tags = Self>;
    type Values<'s>: ValueTuple<'s, Tags = Self>;
}

pub trait TypeTuple<'s>: Sized {
    type Tags: TagTuple<Types<'s> = Self>;

    fn from_slice_any(slice: &[&'s Type<any>]) -> Option<Self>;
    fn vec_any(tuple: Self) -> Vec<&'s Type<any>>;
}

pub trait ValueTuple<'s>: Sized {
    type Tags: TagTuple<Values<'s> = Self>;

    fn from_slice_any(slice: &[&'s Value<any>]) -> Option<Self>;
    fn vec_any(tuple: Self) -> Vec<&'s Value<any>>;
}

macro_rules! impl_tuple {
    ($($arg:ident),*) => {
        impl<$($arg: TypeTag),*> TagTuple for ($($arg,)*) {
            type Types<'s> = ($(&'s Type<$arg>,)*);
            type Values<'s> = ($(&'s Value<$arg>,)*);
        }

        impl<'s, $($arg: TypeTag),*> TypeTuple<'s> for ($(&'s Type<$arg>,)*) {
            type Tags = ($($arg,)*);

            #[allow(non_snake_case)]
            fn from_slice_any(slice: &[&'s Type<any>]) -> Option<Self> {
                if let &[$($arg,)*] = slice {
                    Some(($($arg.cast(),)*))
                } else {
                    None
                }
            }

            #[allow(non_snake_case)]
            fn vec_any(tuple: Self) -> Vec<&'s Type<any>> {
                let ($($arg,)*) = tuple;
                vec![$($arg.to_any(),)*]
            }
        }

        impl<'s, $($arg: TypeTag),*> ValueTuple<'s> for ($(&'s Value<$arg>,)*) {
            type Tags = ($($arg,)*);

            #[allow(non_snake_case)]
            fn from_slice_any(slice: &[&'s Value<any>]) -> Option<Self> {
                if let &[$($arg,)*] = slice {
                    Some(($($arg.cast(),)*))
                } else {
                    None
                }
            }

            #[allow(non_snake_case)]
            fn vec_any(tuple: Self) -> Vec<&'s Value<any>> {
                let ($($arg,)*) = tuple;
                vec![$($arg.to_any(),)*]
            }
        }
    };
}

impl_tuple!();
impl_tuple!(A);
impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
impl_tuple!(A, B, C, D, E, F, G, H);
impl_tuple!(A, B, C, D, E, F, G, H, J);
impl_tuple!(A, B, C, D, E, F, G, H, J, K);
impl_tuple!(A, B, C, D, E, F, G, H, J, K, L);
impl_tuple!(A, B, C, D, E, F, G, H, J, K, L, M);
impl_tuple!(A, B, C, D, E, F, G, H, J, K, L, M, N);
impl_tuple!(A, B, C, D, E, F, G, H, J, K, L, M, N, O);
impl_tuple!(A, B, C, D, E, F, G, H, J, K, L, M, N, O, P);
impl_tuple!(A, B, C, D, E, F, G, H, J, K, L, M, N, O, P, Q);
impl_tuple!(A, B, C, D, E, F, G, H, J, K, L, M, N, O, P, Q, R);
impl_tuple!(A, B, C, D, E, F, G, H, J, K, L, M, N, O, P, Q, R, S);
impl_tuple!(A, B, C, D, E, F, G, H, J, K, L, M, N, O, P, Q, R, S, T);
