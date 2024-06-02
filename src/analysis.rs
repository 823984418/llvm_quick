use std::ptr::null_mut;

use llvm_sys::analysis::*;

use crate::core::module::Module;
use crate::core::values::Value;
use crate::message::Message;
use crate::opaque::Opaque;
use crate::type_tag::function_tag::FunTypeTag;

impl<'s> Module<'s> {
    pub fn verify(&self, action: LLVMVerifierFailureAction) -> Result<(), Message> {
        unsafe {
            let mut err = null_mut();
            if LLVMVerifyModule(self.as_ptr(), action, &mut err) != 0 {
                return Err(Message::from_raw(err));
            }
            Ok(())
        }
    }
}

impl<T: FunTypeTag> Value<T> {
    pub fn verify(&self, action: LLVMVerifierFailureAction) -> bool {
        unsafe { LLVMVerifyFunction(self.as_ptr(), action) != 0 }
    }

    pub fn view_cfg(&self) {
        unsafe { LLVMViewFunctionCFG(self.as_ptr()) }
    }

    pub fn view_cfg_only(&self) {
        unsafe { LLVMViewFunctionCFGOnly(self.as_ptr()) }
    }
}

// Mission completed
