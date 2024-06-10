#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::marker::PhantomData;

use llvm_sys::core::*;
use llvm_sys::*;

use crate::opaque::Opaque;
use crate::{Type, Value};

pub trait TypeTag: Copy + 'static {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>>;
}

#[derive(Copy, Clone)]
pub struct any {}

impl TypeTag for any {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        Some(ty)
    }
}

#[derive(Copy, Clone)]
pub struct void {}

impl TypeTag for void {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMVoidTypeKind) }
    }
}

#[derive(Copy, Clone)]
pub struct label {}

impl TypeTag for label {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMLabelTypeKind) }
    }
}

pub trait ArrayTypeTag: TypeTag {
    type ElementType: TypeTag;

    fn type_length(ty: &Type<Self>) -> u64 {
        unsafe { LLVMGetArrayLength2(ty.as_raw()) }
    }
}

pub type array_any = array_any_len<any>;

#[derive(Copy, Clone)]
pub struct array_any_len<T: TypeTag> {
    marker: PhantomData<fn(T) -> T>,
}

impl<T: TypeTag> TypeTag for array_any_len<T> {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe {
            let ty = type_check_kind::<array_any>(ty, LLVMTypeKind::LLVMArrayTypeKind)?;
            if ty.element_type().try_cast::<T>().is_some() {
                Some(ty.cast_unchecked())
            } else {
                None
            }
        }
    }
}

impl<T: TypeTag> ArrayTypeTag for array_any_len<T> {
    type ElementType = T;
}

#[derive(Copy, Clone)]
pub struct array<T: TypeTag, const N: u64> {
    marker: PhantomData<fn(T) -> T>,
}

impl<T: TypeTag, const N: u64> TypeTag for array<T, N> {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        let ty = array_any_len::<T>::type_cast(ty)?;
        if ty.length() == N {
            Some(unsafe { ty.cast_unchecked() })
        } else {
            None
        }
    }
}

impl<T: TypeTag, const N: u64> ArrayTypeTag for array<T, N> {
    type ElementType = T;

    fn type_length(_ty: &Type<Self>) -> u64 {
        N
    }
}

pub trait FloatTypeTag: TypeTag {}

#[derive(Copy, Clone)]
pub struct float_any {}

impl TypeTag for float_any {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        match ty.get_kind() {
            LLVMTypeKind::LLVMHalfTypeKind
            | LLVMTypeKind::LLVMFloatTypeKind
            | LLVMTypeKind::LLVMDoubleTypeKind
            | LLVMTypeKind::LLVMX86_FP80TypeKind
            | LLVMTypeKind::LLVMFP128TypeKind
            | LLVMTypeKind::LLVMPPC_FP128TypeKind
            | LLVMTypeKind::LLVMBFloatTypeKind => Some(unsafe { ty.cast_unchecked() }),
            _ => None,
        }
    }
}

impl FloatTypeTag for float_any {}

#[derive(Copy, Clone)]
pub struct half {}

impl TypeTag for half {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMHalfTypeKind) }
    }
}

impl FloatTypeTag for half {}

#[derive(Copy, Clone)]
pub struct float {}

impl TypeTag for float {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMFloatTypeKind) }
    }
}

impl FloatTypeTag for float {}

#[derive(Copy, Clone)]
pub struct double {}

impl TypeTag for double {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMDoubleTypeKind) }
    }
}

impl FloatTypeTag for double {}

#[derive(Copy, Clone)]
pub struct x86_fp80 {}

impl TypeTag for x86_fp80 {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMX86_FP80TypeKind) }
    }
}

impl FloatTypeTag for x86_fp80 {}

#[derive(Copy, Clone)]
pub struct fp128 {}

impl TypeTag for fp128 {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMFP128TypeKind) }
    }
}

impl FloatTypeTag for fp128 {}

#[derive(Copy, Clone)]
pub struct ppc_fp128 {}

impl TypeTag for ppc_fp128 {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMPPC_FP128TypeKind) }
    }
}

impl FloatTypeTag for ppc_fp128 {}

#[derive(Copy, Clone)]
pub struct bfloat {}

impl TypeTag for bfloat {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMBFloatTypeKind) }
    }
}

impl FloatTypeTag for bfloat {}

pub trait FunTypeTag: TypeTag {
    fn type_get_param_count(ty: &Type<Self>) -> u32;

    fn type_is_var(ty: &Type<Self>) -> bool;

    #[allow(clippy::needless_lifetimes)]
    fn type_get_param_with_slice<'s, F: FnOnce(&[&'s Type<any>]) -> R, R>(
        ty: &'s Type<Self>,
        f: F,
    ) -> R;

    fn value_get_param_count(val: &Value<Self>) -> u32;

    #[allow(clippy::needless_lifetimes)]
    fn value_get_param_with_slice<'s, F: FnOnce(&[&'s Value<any>]) -> R, R>(
        ty: &'s Value<Self>,
        f: F,
    ) -> R;
}

#[derive(Copy, Clone)]
pub struct fun_any {}

impl TypeTag for fun_any {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMFunctionTypeKind) }
    }
}

impl FunTypeTag for fun_any {
    fn type_get_param_count(ty: &Type<Self>) -> u32 {
        unsafe { LLVMCountParamTypes(ty.as_raw()) }
    }

    fn type_is_var(ty: &Type<Self>) -> bool {
        unsafe { LLVMIsFunctionVarArg(ty.as_raw()) != 0 }
    }

    #[allow(clippy::needless_lifetimes)]
    fn type_get_param_with_slice<'s, F: FnOnce(&[&'s Type<any>]) -> R, R>(
        ty: &'s Type<Self>,
        f: F,
    ) -> R {
        let count = ty.get_param_count() as usize;
        let mut buffer = vec![None; count];
        f(ty.get_param_into_slice(&mut buffer))
    }

    fn value_get_param_count(val: &Value<Self>) -> u32 {
        unsafe { LLVMCountParams(val.as_raw()) }
    }

    #[allow(clippy::needless_lifetimes)]
    fn value_get_param_with_slice<'s, F: FnOnce(&[&'s Value<any>]) -> R, R>(
        val: &'s Value<Self>,
        f: F,
    ) -> R {
        let count = val.get_param_count() as usize;
        let mut buffer = vec![None; count];
        f(val.get_param_into_slice(&mut buffer))
    }
}

#[derive(Copy, Clone)]
pub struct fun<Args: TagTuple, Output: TypeTag, const VAR: bool = false> {
    marker: PhantomData<fn(Args) -> Output>,
}

impl<Args: TagTuple, Output: TypeTag, const VAR: bool> TypeTag for fun<Args, Output, VAR> {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        let ty = fun_any::type_cast(ty)?;
        if ty.is_var() != VAR {
            return None;
        }
        if ty.get_param_count() as usize != Args::COUNT {
            return None;
        }
        ty.get_return_any().try_cast::<Output>()?;
        ty.get_param_with_slice(|slice| Args::type_from_slice(slice))?;
        unsafe { Some(ty.cast_unchecked()) }
    }
}

impl<Args: TagTuple, Output: TypeTag, const VAR: bool> FunTypeTag for fun<Args, Output, VAR> {
    fn type_get_param_count(_ty: &Type<Self>) -> u32 {
        Args::COUNT as u32
    }

    fn type_is_var(_ty: &Type<Self>) -> bool {
        VAR
    }

    #[allow(clippy::needless_lifetimes)]
    fn type_get_param_with_slice<'s, F: FnOnce(&[&'s Type<any>]) -> R, R>(
        ty: &'s Type<Self>,
        f: F,
    ) -> R {
        Args::stack_array(|array| f(ty.get_param_into_slice(array)))
    }

    fn value_get_param_count(_val: &Value<Self>) -> u32 {
        Args::COUNT as u32
    }

    #[allow(clippy::needless_lifetimes)]
    fn value_get_param_with_slice<'s, F: FnOnce(&[&'s Value<any>]) -> R, R>(
        val: &'s Value<Self>,
        f: F,
    ) -> R {
        Args::stack_array(|array| f(val.get_param_into_slice(array)))
    }
}

impl<Args: TagTuple, Output: TypeTag, const VAR: bool> Type<fun<Args, Output, VAR>> {
    /// Obtain the types of a function's parameters.
    #[allow(clippy::needless_lifetimes)]
    pub fn get_params<'s>(&'s self) -> Args::Types<'s> {
        self.get_param_with_slice(|slice| Args::type_from_slice(slice))
            .unwrap()
    }
}

impl<Args: TagTuple, Output: TypeTag, const VAR: bool> Value<fun<Args, Output, VAR>> {
    /// Obtain the parameters in a function.
    #[allow(clippy::needless_lifetimes)]
    pub fn get_params<'s>(&'s self) -> Args::Values<'s> {
        self.get_param_with_slice(|slice| Args::value_from_slice(slice))
            .unwrap()
    }
}

pub trait IntTypeTag: TypeTag {
    fn type_get_int_width(ty: &Type<Self>) -> u32 {
        unsafe { LLVMGetIntTypeWidth(ty.as_raw()) }
    }
}

#[derive(Copy, Clone)]
pub struct int_any {}

impl TypeTag for int_any {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMIntegerTypeKind) }
    }
}

impl IntTypeTag for int_any {}

#[derive(Copy, Clone)]
pub struct int<const N: u32> {}

impl<const N: u32> TypeTag for int<N> {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        let ty = int_any::type_cast(ty)?;
        if ty.get_int_width() == N {
            Some(unsafe { ty.cast_unchecked() })
        } else {
            None
        }
    }
}

impl<const N: u32> IntTypeTag for int<N> {}

impl<T: IntTypeTag> Type<T> {
    pub fn as_int_any(&self) -> &Type<int_any> {
        unsafe { self.cast_unchecked() }
    }
}

pub type int1 = int<1>;
pub type int8 = int<8>;
pub type int16 = int<16>;
pub type int32 = int<32>;
pub type int64 = int<64>;
pub type int128 = int<128>;

pub trait PtrTypeTag: TypeTag {
    fn type_get_address_space(ty: &Type<Self>) -> u32 {
        unsafe { LLVMGetPointerAddressSpace(ty.as_raw()) }
    }
}

#[derive(Copy, Clone)]
pub struct ptr_any {}

impl TypeTag for ptr_any {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMPointerTypeKind) }
    }
}

impl PtrTypeTag for ptr_any {}

#[derive(Copy, Clone)]
pub struct ptr<const ADDRESS_SPACE: u32 = 0> {}

impl<const ADDRESS_SPACE: u32> TypeTag for ptr<ADDRESS_SPACE> {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        let ty = ptr_any::type_cast(ty)?;
        if ty.get_address_space() == ADDRESS_SPACE {
            Some(unsafe { ty.cast_unchecked() })
        } else {
            None
        }
    }
}

impl<const ADDRESS_SPACE: u32> PtrTypeTag for ptr<ADDRESS_SPACE> {
    fn type_get_address_space(_ty: &Type<Self>) -> u32 {
        ADDRESS_SPACE
    }
}

pub(crate) unsafe fn type_check_kind<T: TypeTag>(
    ty: &Type<any>,
    kind: LLVMTypeKind,
) -> Option<&Type<T>> {
    if ty.get_kind() == kind {
        Some(unsafe { ty.cast_unchecked() })
    } else {
        None
    }
}

pub trait InstanceTypeTag: TypeTag {}
impl InstanceTypeTag for void {}
impl InstanceTypeTag for label {}
impl<T: InstanceTypeTag, const N: u64> InstanceTypeTag for array<T, N> {}
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
