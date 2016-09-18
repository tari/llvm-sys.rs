//! Input of the LLVM bitcode format.

use super::prelude::*;

extern "C" {
    /// Build a module from the bitcode in the specified memory buffer.
    ///
    /// Returns 0 on success and the generated module in `OutModule`.
    /// Optionally returns a human-readable error message in `OutMessage`.
    pub fn LLVMParseBitcode(MemBuf: LLVMMemoryBufferRef,
                            OutModule: *mut LLVMModuleRef,
                            OutMessage: *mut *mut ::libc::c_char)
                            -> LLVMBool;
    pub fn LLVMParseBitcodeInContext(ContextRef: LLVMContextRef,
                                     MemBuf: LLVMMemoryBufferRef,
                                     OutModule: *mut LLVMModuleRef,
                                     OutMessage: *mut *mut ::libc::c_char)
                                     -> LLVMBool;

    /// Read a module from the specified path, returning a module provider
    /// performing lazy deserialization.
    ///
    /// Returns 0 on success and an optional error message.
    pub fn LLVMGetBitcodeModuleInContext(ContextRef: LLVMContextRef,
                                         MemBuf: LLVMMemoryBufferRef,
                                         OutM: *mut LLVMModuleRef,
                                         OutMessage: *mut *mut ::libc::c_char)
                                         -> LLVMBool;
    pub fn LLVMGetBitcodeModule(MemBuf: LLVMMemoryBufferRef,
                                OutM: *mut LLVMModuleRef,
                                OutMessage: *mut *mut ::libc::c_char)
                                -> LLVMBool;
    /// Deprecated: use LLVMGetBitcodeModuleInContext instead
    pub fn LLVMGetBitcodeModuleProviderInContext(ContextRef: LLVMContextRef,
                                                 MemBuf: LLVMMemoryBufferRef,
                                                 OutMP: *mut LLVMModuleProviderRef,
                                                 OutMessage: *mut *mut ::libc::c_char)
                                                 -> LLVMBool;
    /// Deprecated: Use LLVMGetBitcodeModule instead
    pub fn LLVMGetBitcodeModuleProvider(MemBuf: LLVMMemoryBufferRef,
                                        OutMP: *mut LLVMModuleProviderRef,
                                        OutMessage: *mut *mut ::libc::c_char)
                                        -> LLVMBool;
}
