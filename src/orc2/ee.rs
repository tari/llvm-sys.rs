use super::*;
use crate::execution_engine::*;

pub type LLVMMemoryManagerCreateContextCallback = extern "C" fn(CtxCtx: *mut ::libc::c_void);
pub type LLVMMemoryManagerNotifyTerminatingCallback = extern "C" fn(CtxCtx: *mut ::libc::c_void);

extern "C" {
    /// Create a ObjectLinkingLayer instance using the standard JITLink
    /// InProcessMemoryManager for memory management.
    pub fn LLVMOrcCreateObjectLinkingLayerWithInProcessMemoryManager(
        Result: *mut LLVMOrcObjectLayerRef,
        ES: LLVMOrcExecutionSessionRef,
    ) -> LLVMErrorRef;
    pub fn LLVMOrcCreateRTDyldObjectLinkingLayerWithSectionMemoryManager(
        ES: LLVMOrcExecutionSessionRef,
    ) -> LLVMOrcObjectLayerRef;
    /// Create a RTDyldObjectLinkingLayer instance using the standard
    /// SectionMemoryManager for memory management. If ReserveAlloc is true then
    /// a contiguous range of memory will be reserved for each object file.
    pub fn LLVMOrcCreateRTDyldObjectLinkingLayerWithSectionMemoryManagerReserveAlloc(
        ES: LLVMOrcExecutionSessionRef,
        ReserveAlloc: LLVMBool,
    ) -> LLVMOrcObjectLayerRef;
    pub fn LLVMOrcCreateRTDyldObjectLinkingLayerWithMCJITMemoryManagerLikeCallbacks(
        ES: LLVMOrcExecutionSessionRef,
        CreateContext: LLVMMemoryManagerCreateContextCallback,
        NotifyTerminating: LLVMMemoryManagerNotifyTerminatingCallback,
        AllocateCodeSection: LLVMMemoryManagerAllocateCodeSectionCallback,
        AllocateDataSection: LLVMMemoryManagerAllocateDataSectionCallback,
        FinalizeMemory: LLVMMemoryManagerFinalizeMemoryCallback,
        Destroy: LLVMMemoryManagerDestroyCallback,
    ) -> LLVMOrcObjectLayerRef;
    pub fn LLVMOrcRTDyldObjectLinkingLayerRegisterJITEventListener(
        RTDyldObjLinkingLayer: LLVMOrcObjectLayerRef,
        Listener: LLVMJITEventListenerRef,
    );
}
