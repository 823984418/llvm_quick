use std::ptr::null_mut;

use llvm_sys::analysis::*;

use crate::core::Message;
use crate::type_tag::*;
use crate::*;

impl<'c> Module<'c> {
    pub fn verify(&self, action: LLVMVerifierFailureAction) -> Result<(), Message> {
        unsafe {
            let mut err = null_mut();
            if LLVMVerifyModule(self.as_raw(), action, &mut err) != 0 {
                return Err(Message::from_raw(err));
            }
            Ok(())
        }
    }
}

impl<T: FunTypeTag> Value<T> {
    pub fn verify(&self, action: LLVMVerifierFailureAction) -> bool {
        unsafe { LLVMVerifyFunction(self.as_raw(), action) != 0 }
    }

    pub fn view_cfg(&self) {
        unsafe { LLVMViewFunctionCFG(self.as_raw()) }
    }

    pub fn view_cfg_only(&self) {
        unsafe { LLVMViewFunctionCFGOnly(self.as_raw()) }
    }
}
