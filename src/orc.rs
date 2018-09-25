//! The ORC JIT.

use super::prelude::*;
use super::target_machine::LLVMTargetMachineRef;

pub enum LLVMOrcOpaqueJITStack {}
pub type LLVMOrcJITStackRef = *mut LLVMOrcOpaqueJITStack;
pub type LLVMOrcModuleHandle = u64;
pub type LLVMOrcTargetAddress = u64;

pub type LLVMOrcSymbolResolverFn = Option<extern "C" fn(*const ::libc::c_char,
                                                        *mut ::libc::c_void)
                                                        -> u64>;
pub type LLVMOrcLazyCompileCallbackFn = Option<extern "C" fn(LLVMOrcJITStackRef,
                                                             *mut ::libc::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMOrcErrorCode {
    LLVMOrcErrSuccess = 0,
    LLVMOrcErrGeneric,
}

extern "C" {
    /// Create an ORC JIT stack.
    ///
    /// The client owns the returned stack and must call OrcDisposeInstance
    /// when done with it. The JIT stack takes ownership of the provided
    /// TargetMachine.
    pub fn LLVMOrcCreateInstance(TM: LLVMTargetMachineRef) -> LLVMOrcJITStackRef;

    /// Get the error message for the most recent error (if any).
    ///
    /// The returned message is owned by the ORC JIT stack.
    pub fn LLVMOrcGetErrorMsg(JITStack: LLVMOrcJITStackRef) -> *const ::libc::c_char;

    /// Mangle the given symbol.
    ///
    /// Memory is allocated for the mangled symbol, which will be owned by
    /// the client.
    pub fn LLVMOrcGetMangledSymbol(JITStack: LLVMOrcJITStackRef,
                                   MangledSymbol: *mut *mut ::libc::c_char,
                                   Symbol: *const ::libc::c_char);

    /// Dispose of a mangled symbol.
    pub fn LLVMOrcDisposeMangledSymbol(MangledSymbol: *mut ::libc::c_char);

    /// Create a lazy compile callback.
    pub fn LLVMOrcCreateLazyCompileCallback(JITStack: LLVMOrcJITStackRef,
                                            RetAddr: *mut LLVMOrcTargetAddress,
                                            Callback: LLVMOrcLazyCompileCallbackFn,
                                            CallbackCtx: *mut ::libc::c_void)
                                            -> LLVMOrcErrorCode;

    /// Create a named indirect call stub.
    pub fn LLVMOrcCreateIndirectStub(JITStack: LLVMOrcJITStackRef,
                                     StubName: *const ::libc::c_char,
                                     InitAddr: LLVMOrcTargetAddress)
                                     -> LLVMOrcErrorCode;

    /// Set the pointer for the given indirect stub.
    pub fn LLVMOrcSetIndirectStubPointer(JITStack: LLVMOrcJITStackRef,
                                         StubName: *const ::libc::c_char,
                                         NewAddr: LLVMOrcTargetAddress)
                                         -> LLVMOrcErrorCode;

    /// Add a module to be eagerly compiled.
    pub fn LLVMOrcAddEagerlyCompiledIR(JITStack: LLVMOrcJITStackRef,
                                       RetHandle: *mut LLVMOrcModuleHandle,
                                       Mod: LLVMModuleRef,
                                       SymbolResolver: LLVMOrcSymbolResolverFn,
                                       SymbolResolverCtx: *mut ::libc::c_void)
                                       -> LLVMOrcErrorCode;

    /// Add a module to be lazily compiled one function at a time.
    pub fn LLVMOrcAddLazilyCompiledIR(JITStack: LLVMOrcJITStackRef,
                                      RetHandle: *mut LLVMOrcModuleHandle,
                                      Mod: LLVMModuleRef,
                                      SymbolResolver: LLVMOrcSymbolResolverFn,
                                      SymbolResolverCtx: *mut ::libc::c_void)
                                      -> LLVMOrcErrorCode;

    /// Add an object file.
    pub fn LLVMOrcAddObjectFile(JITStack: LLVMOrcJITStackRef,
                                RetHandle: *mut LLVMOrcModuleHandle,
                                Obj: LLVMMemoryBufferRef,
                                SymbolResolver: LLVMOrcSymbolResolverFn,
                                SymbolResolverCtx: *mut ::libc::c_void)
                                -> LLVMOrcErrorCode;

    /// Remove a module set from the JIT.
    pub fn LLVMOrcRemoveModule(JITStack: LLVMOrcJITStackRef,
                               H: LLVMOrcModuleHandle)
                               -> LLVMOrcErrorCode;

    /// Get symbol address from JIT instance.
    pub fn LLVMOrcGetSymbolAddress(JITStack: LLVMOrcJITStackRef,
                                   RetAddr: *mut LLVMOrcTargetAddress,
                                   SymbolName: *const ::libc::c_char)
                                   -> LLVMOrcErrorCode;

    /// Get symbol address from JIT instance, searching only the specified handle.
    pub fn LLVMOrcGetSymbolAddressIn(JITStack: LLVMOrcJITStackRef,
                                     RetAddr: *mut LLVMOrcTargetAddress,
                                     H: LLVMOrcModuleHandle,
                                     SymbolName: *const ::libc::c_char)
                                     -> LLVMOrcErrorCode;

    /// Dispose of an ORC JIT stack.
    pub fn LLVMOrcDisposeInstance(JITStack: LLVMOrcJITStackRef) -> LLVMOrcErrorCode;

    /// Register a JIT Event Listener.
    ///
    /// A NULL listener is ignored.
    pub fn LLVMOrcRegisterJITEventListener(JITStack: LLVMOrcJITStackRef,
                                           L: LLVMJITEventListenerRef);

    /// Unegister a JIT Event Listener.
    ///
    /// A NULL listener is ignored.
    pub fn LLVMOrcUnregisterJITEventListener(JITStack: LLVMOrcJITStackRef,
                                             L: LLVMJITEventListenerRef);
}
