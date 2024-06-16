use std::ffi::CStr;

use llvm_sys::core::*;
use llvm_sys::LLVMOpcode;

use crate::opaque::Opaque;
use crate::type_tag::*;
use crate::{Constant, Context, Module, Type, Value, ValueMetadataEntries};

impl<T: TypeTag> Type<T> {
    pub fn const_null(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstNull(self.as_raw())) }
    }

    pub fn const_all_ones(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstAllOnes(self.as_raw())) }
    }

    pub fn get_undef(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMGetUndef(self.as_raw())) }
    }

    pub fn get_poison(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMGetPoison(self.as_raw())) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn is_null(&self) -> bool {
        unsafe { LLVMIsNull(self.as_raw()) != 0 }
    }
}

impl<T: PtrTypeTag> Type<T> {
    pub fn const_pointer_null(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstPointerNull(self.as_raw())) }
    }
}

impl<T: IntTypeTag> Type<T> {
    pub fn const_int(&self, n: u64, sign_extend: bool) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstInt(self.as_raw(), n, sign_extend as _)) }
    }

    pub fn const_int_of_arbitrary_precision(&self, words: &[u64]) -> &Constant<T> {
        unsafe {
            Constant::from_raw(LLVMConstIntOfArbitraryPrecision(
                self.as_raw(),
                words.len() as _,
                words.as_ptr(),
            ))
        }
    }

    pub fn const_int_of_string(&self, text: &CStr, radix: u8) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstIntOfString(self.as_raw(), text.as_ptr(), radix)) }
    }

    pub fn const_int_of_string_and_size(&self, text: &[u8], radix: u8) -> &Constant<T> {
        unsafe {
            Constant::from_raw(LLVMConstIntOfStringAndSize(
                self.as_raw(),
                text.as_ptr() as _,
                text.len() as _,
                radix,
            ))
        }
    }
}

impl<T: FloatTypeTag> Type<T> {
    pub fn const_real(&self, n: f64) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstReal(self.as_raw(), n)) }
    }

    pub fn const_real_of_string(&self, text: &CStr) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstRealOfString(self.as_raw(), text.as_ptr())) }
    }

    pub fn const_real_of_string_and_size(&self, text: &[u8]) -> &Constant<T> {
        unsafe {
            Constant::from_raw(LLVMConstRealOfStringAndSize(
                self.as_raw(),
                text.as_ptr() as _,
                text.len() as _,
            ))
        }
    }
}

impl<T: IntTypeTag> Constant<T> {
    pub fn get_z_ext_value(&self) -> u64 {
        unsafe { LLVMConstIntGetZExtValue(self.as_raw()) }
    }

    pub fn get_s_ext_value(&self) -> i64 {
        unsafe { LLVMConstIntGetSExtValue(self.as_raw()) }
    }
}

impl<T: FloatTypeTag> Constant<T> {
    pub fn get_double(&self) -> (f64, bool) {
        unsafe {
            let mut loses_info = 0;
            let v = LLVMConstRealGetDouble(self.as_raw(), &mut loses_info);
            (v, loses_info != 0)
        }
    }
}

impl Context {
    pub fn const_string(
        &self,
        s: &[u8],
        dont_null_terminate: bool,
    ) -> &Constant<array_any_len<int8>> {
        unsafe {
            Constant::from_raw(LLVMConstStringInContext(
                self.as_raw(),
                s.as_ptr() as _,
                s.len() as _,
                dont_null_terminate as _,
            ))
        }
    }
}

impl<T: TypeTag> Constant<T> {
    pub fn is_constant_string(&self) -> bool {
        unsafe { LLVMIsConstantString(self.as_raw()) != 0 }
    }

    pub fn get_as_string(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetAsString(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr as _, len)
        }
    }
}

impl Context {
    pub fn const_struct(
        &self,
        constant_vals: &[&Value<any>],
        packed: bool,
    ) -> &Constant<struct_any> {
        unsafe {
            Constant::from_raw(LLVMConstStructInContext(
                self.as_raw(),
                constant_vals.as_ptr() as _,
                constant_vals.len() as _,
                packed as _,
            ))
        }
    }
}

impl<T: TypeTag> Type<T> {
    pub fn const_array(&self, constant_vals: &[&Value<T>]) -> &Constant<array_any_len<T>> {
        unsafe {
            Constant::from_raw(LLVMConstArray2(
                self.as_raw(),
                constant_vals.as_ptr() as _,
                constant_vals.len() as _,
            ))
        }
    }
}

impl Type<struct_any> {
    pub fn const_named_struct(&self, constant_vals: &[&Value<any>]) -> &Constant<struct_any> {
        unsafe {
            Constant::from_raw(LLVMConstNamedStruct(
                self.as_raw(),
                constant_vals.as_ptr() as _,
                constant_vals.len() as _,
            ))
        }
    }
}

impl<T: TypeTag> Constant<T> {
    pub fn get_aggregate_element(&self, idx: u32) -> &Value<any> {
        unsafe { Value::from_raw(LLVMGetAggregateElement(self.as_raw(), idx)) }
    }
}

pub fn const_vector<'s, T: TypeTag>(
    scalar_constant_vals: &[&'s Value<T>],
) -> &'s Constant<vector_any_len<T>> {
    unsafe {
        Constant::from_raw(LLVMConstVector(
            scalar_constant_vals.as_ptr() as _,
            scalar_constant_vals.len() as _,
        ))
    }
}

impl<T: TypeTag> Constant<T> {
    pub fn get_const_opcode(&self) -> LLVMOpcode {
        unsafe { LLVMGetConstOpcode(self.as_raw()) }
    }
}

impl<T: TypeTag> Type<T> {
    pub fn align_of(&self) -> &Constant<int64> {
        unsafe { Constant::from_raw(LLVMAlignOf(self.as_raw())) }
    }

    pub fn size_of(&self) -> &Constant<int64> {
        unsafe { Constant::from_raw(LLVMSizeOf(self.as_raw())) }
    }
}

impl<T: TypeTag> Constant<T> {
    pub fn const_neg(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstNeg(self.as_raw())) }
    }
}

// TODO

impl<'s> Drop for ValueMetadataEntries<'s> {
    fn drop(&mut self) {
        unsafe { LLVMDisposeValueMetadataEntries(self.as_raw()) }
    }
}

// TODO

impl<'s> Module<'s> {
    pub fn add_alias<T: TypeTag>(
        &self,
        value_type: &Type<T>,
        addr_space: u32,
        aliasee: &Value<T>,
        name: &CStr,
    ) -> &Value<T> {
        unsafe {
            Value::from_raw(LLVMAddAlias2(
                self.as_raw(),
                value_type.as_raw(),
                addr_space,
                aliasee.as_raw(),
                name.as_ptr(),
            ))
        }
    }
}
