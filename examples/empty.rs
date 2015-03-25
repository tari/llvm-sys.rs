extern crate "llvm-sys" as llvm;

fn main() {
    unsafe {
        let module = llvm::core::LLVMModuleCreateWithName(b"empty\0".as_ptr() as *const _);
        llvm::core::LLVMDumpModule(module);
        llvm::core::LLVMDisposeModule(module);
    }
}
