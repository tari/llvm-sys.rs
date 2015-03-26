//! The module/file/archive linker

use super::prelude::*;

#[repr(C)]
pub enum LLVMLinkerMode {
    LLVMLinkerDestroySource = 0,
    LLVMLinkerPreserveSource = 1
}

extern "C" {
    /// Link the source module into the destination.
    ///
    /// Takes ownership of the source module, returning false on success.
    /// Optionally returns a human-readable error message in `OutMessage`.
    pub fn LLVMLinkModules(Dest: LLVMModuleRef, Src: LLVMModuleRef,
                           Mode: LLVMLinkerMode,
                           OutMessage: *mut *mut ::libc::c_char) -> LLVMBool;
}
