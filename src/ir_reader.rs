//! The IR reader

use super::prelude::*;

extern "C" {
    /// Read LLVM IR from a memory buffer and convert it to an in-memory Module.
    ///
    /// Returns 0 on success, and an optional human-readable description of any
    /// errors that occurred.
    #[deprecated(since = "221.0.0", note = "Use LLVMParseIRInContext2 instead.")]
    pub fn LLVMParseIRInContext(
        ContextRef: LLVMContextRef,
        MemBuf: LLVMMemoryBufferRef,
        OutM: *mut LLVMModuleRef,
        OutMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    /// Read LLVM IR from a memory buffer and convert it into an in-memory Module
    /// object. Returns 0 on success.
    /// Optionally returns a human-readable description of any errors that
    /// occurred during parsing IR. OutMessage must be disposed with
    /// LLVMDisposeMessage.
    /// The memory buffer is not consumed by this function. It is the responsibility
    /// of the caller to free it with `LLVMDisposeMemoryBuffer`.
    ///
    /// See `llvm::ParseIR()`.
    pub fn LLVMParseIRInContext2(
        ContextRef: LLVMContextRef,
        MemBuf: LLVMMemoryBufferRef,
        OutM: *mut LLVMModuleRef,
        OutMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
}
