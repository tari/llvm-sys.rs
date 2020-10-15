//! OrcV2

use error::LLVMErrorRef;
use prelude::*;
use target_machine::LLVMTargetMachineRef;

pub type LLVMOrcJITTargetAddress = u64;

#[derive(Debug)]
pub enum LLVMOrcOpaqueExecutionSession {}
pub type LLVMOrcExecutionSessionRef = *mut LLVMOrcOpaqueExecutionSession;

#[derive(Debug)]
pub enum LLVMOrcQuaqueSymbolStringPoolEntryPtr {}
pub type LLVMOrcSymbolStringPoolEntryRef = *mut LLVMOrcQuaqueSymbolStringPoolEntryPtr;

#[derive(Debug)]
pub enum LLVMOrcOpaqueJITDylib {}
pub type LLVMOrcJITDylibRef = *mut LLVMOrcOpaqueJITDylib;

#[derive(Debug)]
pub enum LLVMOrcOpaqueJITDylibDefinitionGenerator {}
pub type LLVMOrcJITDylibDefinitionGeneratorRef = *mut LLVMOrcOpaqueJITDylibDefinitionGenerator;

pub type LLVMOrcSymbolPredicate =
    extern "C" fn(Sym: LLVMOrcSymbolStringPoolEntryRef, Ctx: *mut ::libc::c_void) -> ::libc::c_int;

#[derive(Debug)]
pub enum LLVMOrcOpaqueThreadSafeContext {}
pub type LLVMOrcThreadSafeContextRef = *mut LLVMOrcOpaqueThreadSafeContext;

#[derive(Debug)]
pub enum LLVMOrcOpaqueThreadSafeModule {}
pub type LLVMOrcThreadSafeModuleRef = *mut LLVMOrcOpaqueThreadSafeModule;

#[derive(Debug)]
pub enum LLVMOrcOpaqueJITTargetMachineBuilder {}
pub type LLVMOrcJITTargetMachineBuilderRef = *mut LLVMOrcOpaqueJITTargetMachineBuilder;

#[derive(Debug)]
pub enum LLVMOrcOpaqueLLJITBuilder {}
pub type LLVMOrcLLJITBuilderRef = *mut LLVMOrcOpaqueLLJITBuilder;

#[derive(Debug)]
pub enum LLVMOrcOpaqueLLJIT {}
pub type LLVMOrcLLJITRef = *mut LLVMOrcOpaqueLLJIT;

extern "C" {
    pub fn LLVMOrcExecutionSessionIntern(
        ES: LLVMOrcExecutionSessionRef,
        Name: *const ::libc::c_char,
    ) -> LLVMOrcSymbolStringPoolEntryRef;
    pub fn LLVMOrcReleaseSymbolStringPoolEntry(S: LLVMOrcSymbolStringPoolEntryRef);
    pub fn LLVMOrcDisposeJITDylibDefinitionGenerator(DG: LLVMOrcJITDylibDefinitionGeneratorRef);
    pub fn LLVMOrcJITDylibAddGenerator(
        JD: LLVMOrcJITDylibRef,
        DG: LLVMOrcJITDylibDefinitionGeneratorRef,
    );
    pub fn LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess(
        Result: *mut LLVMOrcJITDylibDefinitionGeneratorRef,
        GlobalPrefix: ::libc::c_char,
        Filter: LLVMOrcSymbolPredicate,
        FilterCtx: *mut ::libc::c_void,
    ) -> LLVMErrorRef;
    pub fn LLVMOrcCreateNewThreadSafeContext() -> LLVMOrcThreadSafeContextRef;
    pub fn LLVMOrcThreadSafeContextGetContext(TSCtx: LLVMOrcThreadSafeContextRef)
        -> LLVMContextRef;
    pub fn LLVMOrcDisposeThreadSafeContext(TSCtx: LLVMOrcThreadSafeContextRef);
    pub fn LLVMOrcCreateNewThreadSafeModule(
        M: LLVMModuleRef,
        TSCtx: LLVMOrcThreadSafeContextRef,
    ) -> LLVMOrcThreadSafeModuleRef;
    pub fn LLVMOrcDisposeThreadSafeModule(TSM: LLVMOrcThreadSafeModuleRef);
    pub fn LLVMOrcJITTargetMachineBuilderDetectHost(
        Result: *mut LLVMOrcJITTargetMachineBuilderRef,
    ) -> LLVMErrorRef;
    pub fn LLVMOrcJITTargetMachineBuilderCreateFromTargetMachine(
        TM: LLVMTargetMachineRef,
    ) -> LLVMOrcJITTargetMachineBuilderRef;
    pub fn LLVMOrcDisposeJITTargetMachineBuilder(JTMB: LLVMOrcJITTargetMachineBuilderRef);
    pub fn LLVMOrcCreateLLJITBuilder() -> LLVMOrcLLJITBuilderRef;
    pub fn LLVMOrcDisposeLLJITBuilder(Builder: LLVMOrcLLJITBuilderRef);
    pub fn LLVMOrcLLJITBuilderSetJITTargetMachineBuilder(
        Builder: LLVMOrcLLJITBuilderRef,
        JTMB: LLVMOrcJITTargetMachineBuilderRef,
    );
    pub fn LLVMOrcCreateLLJIT(
        Result: *mut LLVMOrcLLJITRef,
        Builder: LLVMOrcLLJITBuilderRef,
    ) -> LLVMErrorRef;
    pub fn LLVMOrcDisposeLLJIT(J: LLVMOrcLLJITRef) -> LLVMErrorRef;
    pub fn LLVMOrcLLJITGetExecutionSession(J: LLVMOrcLLJITRef) -> LLVMOrcExecutionSessionRef;
    pub fn LLVMOrcLLJITGetMainJITDylib(J: LLVMOrcLLJITRef) -> LLVMOrcJITDylibRef;
    pub fn LLVMOrcLLJITGetTripleString(J: LLVMOrcLLJITRef) -> *const ::libc::c_char;
    pub fn LLVMOrcLLJITGetGlobalPrefix(J: LLVMOrcLLJITRef) -> ::libc::c_char;
    pub fn LLVMOrcLLJITMangleAndIntern(
        J: LLVMOrcLLJITRef,
        UnmangledName: *const ::libc::c_char,
    ) -> LLVMOrcSymbolStringPoolEntryRef;
    pub fn LLVMOrcLLJITAddObjectFile(
        J: LLVMOrcLLJITRef,
        JD: LLVMOrcJITDylibRef,
        ObjBuffer: LLVMMemoryBufferRef,
    ) -> LLVMErrorRef;
    pub fn LLVMOrcLLJITAddLLVMIRModule(
        J: LLVMOrcLLJITRef,
        JD: LLVMOrcJITDylibRef,
        TSM: LLVMOrcThreadSafeModuleRef,
    ) -> LLVMErrorRef;
    pub fn LLVMOrcLLJITLookup(
        J: LLVMOrcLLJITRef,
        Result: *mut LLVMOrcJITTargetAddress,
        Name: *const ::libc::c_char,
    ) -> LLVMErrorRef;
}
