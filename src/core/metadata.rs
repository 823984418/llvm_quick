use std::ptr::null_mut;

use llvm_sys::core::*;

use crate::owning::{OpaqueDrop, Owning};
use crate::type_tag::*;
use crate::{Context, Metadata, Module, Opaque, OperandBundle, Value};

impl<'s> OperandBundle<'s> {
    pub fn create<T: TypeTag>(tag: &[u8], args: &[&'s Value<T>]) -> Owning<Self> {
        unsafe {
            Owning::from_raw(LLVMCreateOperandBundle(
                tag.as_ptr() as _,
                tag.len(),
                args.as_ptr() as _,
                args.len() as _,
            ))
        }
    }
}

impl<'s> OpaqueDrop for OperandBundle<'s> {
    unsafe fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeOperandBundle(ptr) }
    }
}

impl<'s> OperandBundle<'s> {
    pub fn get_tag(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetOperandBundleTag(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr as _, len)
        }
    }

    pub fn get_num_args(&self) -> u32 {
        unsafe { LLVMGetNumOperandBundleArgs(self.as_raw()) }
    }

    pub fn get_arg_at_index(&self, index: u32) -> &'s Value<any> {
        unsafe { Value::from_ref(LLVMGetOperandBundleArgAtIndex(self.as_raw(), index)) }
    }
}

impl<'s> Module<'s> {
    pub fn get_named_global_i_func(&self, name: &[u8]) -> &'s Value<any> {
        unsafe {
            Value::from_ref(LLVMGetNamedGlobalIFunc(
                self.as_raw(),
                name.as_ptr() as _,
                name.len(),
            ))
        }
    }

    pub fn get_first_global_i_func(&self) -> &'s Value<any> {
        unsafe { Value::from_ref(LLVMGetFirstGlobalIFunc(self.as_raw())) }
    }

    pub fn get_last_global_i_func(&self) -> &'s Value<any> {
        unsafe { Value::from_ref(LLVMGetLastGlobalIFunc(self.as_raw())) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn get_next_global_i_func(&self) -> Option<&Value<any>> {
        unsafe { Value::try_from_ref(LLVMGetNextGlobalIFunc(self.as_raw())) }
    }

    pub fn get_previous_global_i_func(&self) -> Option<&Value<any>> {
        unsafe { Value::try_from_ref(LLVMGetPreviousGlobalIFunc(self.as_raw())) }
    }

    pub fn get_global_i_func_resolver(&self) -> Option<&Value<any>> {
        unsafe { Value::try_from_ref(LLVMGetGlobalIFuncResolver(self.as_raw())) }
    }

    pub fn set_global_i_func_resolver<S: TypeTag>(&self, resolver: Option<&Value<S>>) {
        unsafe {
            LLVMSetGlobalIFuncResolver(
                self.as_raw(),
                resolver.map(Value::as_raw).unwrap_or(null_mut()),
            )
        }
    }

    pub fn erase_global_i_func(&self) {
        unsafe { LLVMEraseGlobalIFunc(self.as_raw()) }
    }

    pub fn remove_global_i_func(&self) {
        unsafe { LLVMRemoveGlobalIFunc(self.as_raw()) }
    }
}

impl Context {
    pub fn md_string(&self, str: &[u8]) -> &Metadata {
        unsafe {
            Metadata::from_ref(LLVMMDStringInContext2(
                self.as_raw(),
                str.as_ptr() as _,
                str.len(),
            ))
        }
    }

    pub fn md_node(&self, mds: &[&Metadata]) -> &Metadata {
        unsafe {
            Metadata::from_ref(LLVMMDNodeInContext2(
                self.as_raw(),
                mds.as_ptr() as _,
                mds.len(),
            ))
        }
    }
}

impl Metadata {
    pub fn as_value<'s>(&self, context: &'s Context) -> &'s Value<metadata> {
        unsafe { Value::from_ref(LLVMMetadataAsValue(context.as_raw(), self.as_raw())) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn as_metadata(&self) -> &Metadata {
        unsafe { Metadata::from_ref(LLVMValueAsMetadata(self.as_raw())) }
    }
}

impl Value<metadata> {
    pub fn get_md_string(&self) -> Option<&[u8]> {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetMDString(self.as_raw(), &mut len);
            if ptr.is_null() {
                None
            } else {
                Some(std::slice::from_raw_parts(ptr as _, len as _))
            }
        }
    }

    pub fn get_md_node_num_operands(&self) -> u32 {
        unsafe { LLVMGetMDNodeNumOperands(self.as_raw()) }
    }

    pub fn get_md_node_operands<'a, 's>(
        &self,
        dest: &'a mut [Option<&'s Value<metadata>>],
    ) -> &'a mut [&'s Value<metadata>] {
        unsafe {
            LLVMGetMDNodeOperands(self.as_raw(), dest.as_mut_ptr() as _);
            &mut *(dest as *mut [Option<&Value<metadata>>] as *mut [&Value<metadata>])
        }
    }

    pub fn replace_md_node_operand_with(&self, index: u32, replacement: &Metadata) {
        unsafe { LLVMReplaceMDNodeOperandWith(self.as_raw(), index, replacement.as_raw()) }
    }
}
