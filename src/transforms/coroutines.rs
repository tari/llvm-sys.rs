use prelude::*;
use transforms::pass_manager_builder::LLVMPassManagerBuilderRef;

extern "C" {
    pub fn LLVMAddCoroEarlyPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddCoroSplitPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddCoroElidePass(PM: LLVMPassManagerRef);
    pub fn LLVMAddCoroCleanupPass(PM: LLVMPassManagerRef);
    pub fn LLVMPassManagerBuilderAddCoroutinePassesToExtensionPoints(
        PMB: LLVMPassManagerBuilderRef,
    );
}
