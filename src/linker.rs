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
    #[deprecated(since="3.8", reason="Use LLVMLinkModules2 instead")]
    pub fn LLVMLinkModules(Dest: LLVMModuleRef,
                           Src: LLVMModuleRef,
                           _Unused: LLVMLinkerMode,
                           OutMessage: *mut *mut ::libc::c_char)
                           -> LLVMBool;

    /// Link the source module into the destination module.
    ///
    /// Destroys the source module, returns true on error. Use the diagnostic
    /// handler to get any diagnostic message.
    pub fn LLVMLinkModules2(Dest: LLVMModuleRef, Src: LLVMModuleRef) -> LLVMBool;
}
