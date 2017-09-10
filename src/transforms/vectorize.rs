//! Vectorization transformations of LLVM IR.

use super::super::prelude::*;

extern "C" {
    #[deprecated(since="50.0.0", note="Use LLVMAddSPLVectorizePass")]
    pub fn LLVMAddBBVectorizePass(PM: LLVMPassManagerRef);
    pub fn LLVMAddLoopVectorizePass(PM: LLVMPassManagerRef);
    pub fn LLVMAddSLPVectorizePass(PM: LLVMPassManagerRef);
}
