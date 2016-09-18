//! The module/file/archive linker

use super::prelude::*;

#[repr(C)]
pub enum LLVMLinkerMode {
    LLVMLinkerDestroySource = 0,
    #[deprecated(since="3.7.0", note="LLVMLinkerPreserveSource has no effect")]
    LLVMLinkerPreserveSource_Removed = 1,
}

extern "C" {
    /// Link the source module into the destination.
    ///
    /// Takes ownership of the source module, returning false on success.
    /// Optionally returns a human-readable error message in `OutMessage`.
    pub fn LLVMLinkModules(Dest: LLVMModuleRef,
                           Src: LLVMModuleRef,
                           _Unused: LLVMLinkerMode,
                           OutMessage: *mut *mut ::libc::c_char)
                           -> LLVMBool;
}
