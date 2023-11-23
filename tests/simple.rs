extern crate llvm_sys;

use llvm_sys::core::LLVMContextCreate;

#[test]
fn test_simple() {
    unsafe {
        let llvmcontext = LLVMContextCreate();
        assert!(!llvmcontext.is_null())
    }
}
