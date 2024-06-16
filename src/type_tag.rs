#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::borrow::{Borrow, BorrowMut};
use std::marker::PhantomData;
use std::mem::MaybeUninit;

use llvm_sys::*;

use crate::opaque::Opaque;
use crate::{Type, Value};

pub trait TypeTag: Sized {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>>;
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
pub struct struct_any {}

impl TypeTag for struct_any {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMStructTypeKind) }
    }
}

#[derive(Copy, Clone)]
pub struct label {}

impl TypeTag for label {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMLabelTypeKind) }
    }
}

#[derive(Copy, Clone)]
pub struct metadata {}

impl TypeTag for metadata {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMMetadataTypeKind) }
    }
}

#[derive(Copy, Clone)]
pub struct x86_mmx {}

impl TypeTag for x86_mmx {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMX86_MMXTypeKind) }
    }
}

#[derive(Copy, Clone)]
pub struct x86_amx {}

impl TypeTag for x86_amx {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMX86_AMXTypeKind) }
    }
}

#[derive(Copy, Clone)]
pub struct token {}

impl TypeTag for token {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMTokenTypeKind) }
    }
}

#[derive(Copy, Clone)]
pub struct target_ext_any {}

impl TypeTag for target_ext_any {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMTargetExtTypeKind) }
    }
}

pub trait SequentialTypeTag: TypeTag {
    type ElementType: TypeTag;
}

pub trait ArrayTypeTag: SequentialTypeTag {}

pub type array_any = array_any_len<any>;

#[derive(Copy, Clone)]
pub struct array_any_len<T: TypeTag> {
    marker: PhantomData<fn(T) -> T>,
}

impl<T: TypeTag> TypeTag for array_any_len<T> {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe {
            let ty = type_check_kind::<array_any>(ty, LLVMTypeKind::LLVMArrayTypeKind)?;
            if ty.element_type().try_cast::<Type<T>>().is_some() {
                Some(ty.cast_unchecked())
            } else {
                None
            }
        }
    }
}

impl<T: TypeTag> SequentialTypeTag for array_any_len<T> {
    type ElementType = T;
}
impl<T: TypeTag> ArrayTypeTag for array_any_len<T> {}

#[derive(Copy, Clone)]
pub struct array<T: TypeTag, const N: u64> {
    marker: PhantomData<fn(T) -> T>,
}

impl<T: TypeTag, const N: u64> TypeTag for array<T, N> {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        let ty = array_any_len::<T>::type_cast(ty)?;
        if ty.get_length() == N {
            Some(unsafe { ty.cast_unchecked() })
        } else {
            None
        }
    }
}

impl<T: TypeTag, const N: u64> SequentialTypeTag for array<T, N> {
    type ElementType = T;
}
impl<T: TypeTag, const N: u64> ArrayTypeTag for array<T, N> {}

pub trait VectorTypeTag: SequentialTypeTag {}

pub type vector_any = vector_any_len<any>;

#[derive(Copy, Clone)]
pub struct vector_any_len<T: TypeTag> {
    marker: PhantomData<fn(T) -> T>,
}

impl<T: TypeTag> TypeTag for vector_any_len<T> {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe {
            let ty = type_check_kind::<vector_any>(ty, LLVMTypeKind::LLVMVectorTypeKind)?;
            if ty.element_type().try_cast::<Type<T>>().is_some() {
                Some(ty.cast_unchecked())
            } else {
                None
            }
        }
    }
}

impl<T: TypeTag> SequentialTypeTag for vector_any_len<T> {
    type ElementType = T;
}
impl<T: TypeTag> VectorTypeTag for vector_any_len<T> {}

#[derive(Copy, Clone)]
pub struct vector<T: TypeTag, const N: u32> {
    marker: PhantomData<fn(T) -> T>,
}

impl<T: TypeTag, const N: u32> TypeTag for vector<T, N> {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        let ty = vector_any_len::<T>::type_cast(ty)?;
        if ty.get_size() == N {
            Some(unsafe { ty.cast_unchecked() })
        } else {
            None
        }
    }
}

impl<T: TypeTag, const N: u32> SequentialTypeTag for vector<T, N> {
    type ElementType = T;
}
impl<T: TypeTag, const N: u32> VectorTypeTag for vector<T, N> {}

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

pub trait FunTypeTag: TypeTag {}

#[derive(Copy, Clone)]
pub struct fun_any {}

impl TypeTag for fun_any {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMFunctionTypeKind) }
    }
}

impl FunTypeTag for fun_any {}

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
        ty.get_return_any().try_cast::<Type<Output>>()?;
        unsafe {
            let mut array =
                MaybeUninit::<<Args::Types<'_> as Tuple>::Array<Option<&Type<any>>>>::zeroed()
                    .assume_init();
            ty.get_param_into_slice(array.as_mut());
            Args::Types::try_from_array_any(std::mem::transmute(array.as_ref()))?;
            Some(ty.cast_unchecked())
        }
    }
}

impl<Args: TagTuple, Output: TypeTag, const VAR: bool> FunTypeTag for fun<Args, Output, VAR> {}

pub trait IntTypeTag: TypeTag {}

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

pub trait PtrTypeTag: TypeTag {}

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

impl<const ADDRESS_SPACE: u32> PtrTypeTag for ptr<ADDRESS_SPACE> {}

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

pub trait ElementTypeTag: TypeTag {}
impl ElementTypeTag for struct_any {}
impl<T: TypeTag, const N: u64> ElementTypeTag for array<T, N> {}
impl<T: TypeTag> ElementTypeTag for array_any_len<T> {}

pub trait InstanceTagTuple: TagTuple {}

pub trait Tuple {
    const COUNT: usize;
    /// `[Type; Self::COUNT]`
    type Array<Type>: AsRef<[Type]> + AsMut<[Type]> + Borrow<[Type]> + BorrowMut<[Type]>;
}

pub trait TagTuple: Tuple {
    type Types<'s>: TypeTuple<'s, Tags = Self>
    where
        Self: 's;
    type Values<'s>: ValueTuple<'s, Tags = Self>
    where
        Self: 's;
}

pub trait TypeTuple<'s>: Tuple + Sized + 's {
    type Tags: TagTuple<Types<'s> = Self>;
    fn try_from_array_any(array: &[&'s Type<any>]) -> Option<Self>;
    unsafe fn from_array_any_unchecked(array: &[&'s Type<any>]) -> Self;
    fn to_array_any(&self) -> Self::Array<&'s Type<any>>;
}

pub trait ValueTuple<'s>: Tuple + Sized + 's {
    type Tags: TagTuple<Values<'s> = Self>;
    fn try_from_array_any(array: &[&'s Value<any>]) -> Option<Self>;
    unsafe fn from_array_any_unchecked(array: &[&'s Value<any>]) -> Self;
    fn to_array_any(&self) -> Self::Array<&'s Value<any>>;
}

macro_rules! impl_tuple {
    (impl Tuple for ($($arg:ident),*)[$count:literal]) => {
        impl<$($arg),*> Tuple for ($($arg,)*) {
            const COUNT: usize = $count;
            type Array<Type> = [Type; $count];
        }
    };
    (impl TagTuple for ($($arg:ident),*)) => {
        impl<$($arg: TypeTag),*> TagTuple for ($($arg,)*) {
            type Types<'s> = ($(&'s Type<$arg>,)*)
            where
                Self:'s;
            type Values<'s> = ($(&'s Value<$arg>,)*)
            where
                Self:'s;
        }
    };
    (impl TypeTuple for ($($arg:ident),*)) => {
        impl<'s, $($arg: TypeTag + 's),*> TypeTuple<'s> for ($(&'s Type<$arg>,)*) {
            type Tags = ($($arg,)*);
            fn try_from_array_any(array: &[&'s Type<any>]) -> Option<Self> {
                let &[$($arg,)*] = array else { panic!() };
                Some(($(Type::try_cast($arg)?,)*))
            }
            #[allow(unused_unsafe)]
            unsafe fn from_array_any_unchecked(array: &[&'s Type<any>]) -> Self {
                let &[$($arg,)*] = array else { panic!() };
                unsafe { ($(Type::cast_unchecked($arg),)*) }
            }
            fn to_array_any(&self) -> Self::Array<&'s Type<any>> {
                let ($($arg,)*) = self;
                [$(Type::to_any($arg),)*]
            }
        }
    };
    (impl ValueTuple for ($($arg:ident),*)) => {
        impl<'s, $($arg: TypeTag + 's),*> ValueTuple<'s> for ($(&'s Value<$arg>,)*) {
            type Tags = ($($arg,)*);
            fn try_from_array_any(array: &[&'s Value<any>]) -> Option<Self> {
                let &[$($arg,)*] = array else { panic!() };
                Some(($(Value::try_cast($arg)?,)*))
            }
            #[allow(unused_unsafe)]
            unsafe fn from_array_any_unchecked(array: &[&'s Value<any>]) -> Self {
                let &[$($arg,)*] = array else { panic!() };
                unsafe { ($(Value::cast_unchecked($arg),)*) }
            }
            fn to_array_any(&self) -> Self::Array<&'s Value<any>> {
                let ($($arg,)*) = self;
                [$(Value::to_any($arg),)*]
            }
        }
    };
    (impl InstanceTagTuple for ($($arg:ident),*)) => {
        impl<$($arg: InstanceTypeTag),*> InstanceTagTuple for ($($arg,)*) {}
    };
    ($count:literal $(,$arg:ident)*) => {
        impl_tuple!(impl Tuple for ($($arg),*)[$count]);
        impl_tuple!(impl TagTuple for ($($arg),*));
        impl_tuple!(impl TypeTuple for ($($arg),*));
        impl_tuple!(impl ValueTuple for ($($arg),*));
        impl_tuple!(impl InstanceTagTuple for ($($arg),*));
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
