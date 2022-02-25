#![allow(non_snake_case)]
//! OrcV2

pub mod ee;
pub mod lljit;

use error::LLVMErrorRef;
use prelude::*;
use target_machine::LLVMTargetMachineRef;

pub type LLVMOrcJITTargetAddress = u64;

/// Generic linkage flags for a symbol definition.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMJITSymbolGenericFlags {
    LLVMJITSymbolGenericFlagsExported = 1,
    LLVMJITSymbolGenericFlagsWeak = 2,
}

/// Target specific flags for a symbol definition.
pub type LLVMJITTargetSymbolFlags = u8;

/// Linkage flags for a symbol definition.
#[repr(C)]
#[derive(Debug)]
pub struct LLVMJITSymbolFlags {
    pub GenericFlags: u8,
    pub TargetFlags: u8,
}

/// An evaluated symbol address and flags.
#[repr(C)]
#[derive(Debug)]
pub struct LLVMJITEvaluatedSymbol {
    pub Address: LLVMOrcJITTargetAddress,
    pub Flags: LLVMJITSymbolFlags,
}

#[derive(Debug)]
pub enum LLVMOrcOpaqueExecutionSession {}
pub type LLVMOrcExecutionSessionRef = *mut LLVMOrcOpaqueExecutionSession;

/// Error reporter function.
pub type LLVMOrcErrorReporterFunction = extern "C" fn(Ctx: *mut ::libc::c_void, Err: LLVMErrorRef);

#[derive(Debug)]
pub enum LLVMOrcOpaqueSymbolStringPool {}
/// A reference to an orc::SymbolStringPool.
pub type LLVMOrcSymbolStringPoolRef = *mut LLVMOrcOpaqueSymbolStringPool;

#[derive(Debug)]
pub enum LLVMOrcQuaqueSymbolStringPoolEntry {}
pub type LLVMOrcSymbolStringPoolEntryRef = *mut LLVMOrcQuaqueSymbolStringPoolEntry;

/// A pair of a symbol name and an evaluated symbol.
#[repr(C)]
#[derive(Debug)]
pub struct LLVMJITCSymbolMapPair {
    pub Name: LLVMOrcSymbolStringPoolEntryRef,
    pub Sym: LLVMJITEvaluatedSymbol,
}

/// A list of (SymbolStringPtr, JITEvaluatedSymbol) pairs that can be
/// used to construct a SymbolMap.
pub type LLVMOrcCSymbolMapPairs = *mut LLVMJITCSymbolMapPair;

/// Lookup kind. This can be used by definition generators when deciding whether
/// to produce a definition for a requested symbol.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMOrcLookupKind {
    LLVMOrcLookupKindStatic,
    LLVMOrcLookupKindDLSym,
}

/// JITDylib lookup flags. This can be used by definition generators when
/// deciding whether to produce a definition for a requested symbol.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMOrcJITDylibLookupFlags {
    LLVMOrcJITDylibLookupFlagsMatchExportedSymbolsOnly,
    LLVMOrcJITDylibLookupFlagsMatchAllSymbols,
}

/// Symbol lookup flags for lookup sets.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMOrcSymbolLookupFlags {
    LLVMOrcSymbolLookupFlagsRequiredSymbol,
    LLVMOrcSymbolLookupFlagsWeaklyReferencedSymbol,
}

/// An element type for a symbol lookup set.
#[repr(C)]
#[derive(Debug)]
pub struct LLVMOrcCLookupSetElement {
    pub Name: LLVMOrcSymbolStringPoolEntryRef,
    pub LookupFlags: LLVMOrcSymbolLookupFlags,
}

/// A set of symbols to look up / generate.
///
/// The list is terminated with an element containing a null pointer for the
/// Name field.
///
/// The creator is responsible for freeing the set and ensuring all strings
/// are retained for the set's lifetime.
pub type LLVMOrcCLookupSet = *mut LLVMOrcCLookupSetElement;

#[derive(Debug)]
pub enum LLVMOrcOpaqueMaterializationUnit {}
pub type LLVMOrcMaterializationUnitRef = *mut LLVMOrcOpaqueMaterializationUnit;

#[derive(Debug)]
pub enum LLVMOrcOpaqueJITDylib {}
pub type LLVMOrcJITDylibRef = *mut LLVMOrcOpaqueJITDylib;

#[derive(Debug)]
pub enum LLVMOrcOpaqueResourceTracker {}
pub type LLVMOrcResourceTrackerRef = *mut LLVMOrcOpaqueResourceTracker;

#[derive(Debug)]
pub enum LLVMOrcOpaqueDefinitionGenerator {}
pub type LLVMOrcDefinitionGeneratorRef = *mut LLVMOrcOpaqueDefinitionGenerator;

#[derive(Debug)]
pub enum LLVMOrcOpaqueLookupState {}
pub type LLVMOrcLookupStateRef = *mut LLVMOrcOpaqueLookupState;

pub type LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction = extern "C" fn(
    GeneratorObj: LLVMOrcDefinitionGeneratorRef,
    Ctx: *mut ::libc::c_void,
    LookupState: *mut LLVMOrcLookupStateRef,
    Kind: LLVMOrcLookupKind,
    JD: LLVMOrcJITDylibRef,
    JDLookupFlags: LLVMOrcJITDylibLookupFlags,
    LookupSet: LLVMOrcCLookupSet,
    LookupSetSize: usize,
) -> LLVMErrorRef;

pub type LLVMOrcSymbolPredicate = Option<
    extern "C" fn(Ctx: *mut ::libc::c_void, Sym: LLVMOrcSymbolStringPoolEntryRef) -> ::libc::c_int,
>;

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
pub enum LLVMOrcOpaqueObjectLayer {}
pub type LLVMOrcObjectLayerRef = *mut LLVMOrcOpaqueObjectLayer;

extern "C" {
    pub fn LLVMOrcExecutionSessionSetErrorReporter(
        ES: LLVMOrcExecutionSessionRef,
        ReportError: LLVMOrcErrorReporterFunction,
        Ctx: *mut ::libc::c_void,
    );
    pub fn LLVMOrcExecutionSessionGetSymbolStringPool(
        ES: LLVMOrcExecutionSessionRef,
    ) -> LLVMOrcSymbolStringPoolRef;
    pub fn LLVMOrcSymbolStringPoolClearDeadEntries(SSP: LLVMOrcSymbolStringPoolRef);
    pub fn LLVMOrcExecutionSessionIntern(
        ES: LLVMOrcExecutionSessionRef,
        Name: *const ::libc::c_char,
    ) -> LLVMOrcSymbolStringPoolEntryRef;
    pub fn LLVMOrcRetainSymbolStringPoolEntry(S: LLVMOrcSymbolStringPoolEntryRef);
    pub fn LLVMOrcReleaseSymbolStringPoolEntry(S: LLVMOrcSymbolStringPoolEntryRef);
    pub fn LLVMOrcSymbolStringPoolEntryStr(
        S: LLVMOrcSymbolStringPoolEntryRef,
    ) -> *const ::libc::c_char;
    pub fn LLVMOrcReleaseResourceTracker(RT: LLVMOrcResourceTrackerRef);
    pub fn LLVMOrcResourceTrackerTransferTo(
        SrcRT: LLVMOrcResourceTrackerRef,
        DstRT: LLVMOrcResourceTrackerRef,
    );
    pub fn LLVMOrcResourceTrackerRemove(RT: LLVMOrcResourceTrackerRef) -> LLVMErrorRef;
    pub fn LLVMOrcDisposeDefinitionGenerator(DG: LLVMOrcDefinitionGeneratorRef);
    pub fn LLVMOrcDisposeMaterializationUnit(MU: LLVMOrcMaterializationUnitRef);
    pub fn LLVMOrcAbsoluteSymbols(
        Syms: LLVMOrcCSymbolMapPairs,
        NumPairs: usize,
    ) -> LLVMOrcMaterializationUnitRef;
    pub fn LLVMOrcExecutionSessionCreateBareJITDylib(
        ES: LLVMOrcExecutionSessionRef,
        Name: *const ::libc::c_char,
    ) -> LLVMOrcJITDylibRef;
    pub fn LLVMOrcExecutionSessionCreateJITDylib(
        ES: LLVMOrcExecutionSessionRef,
        Result_: *mut LLVMOrcJITDylibRef,
        Name: *const ::libc::c_char,
    ) -> LLVMErrorRef;
    pub fn LLVMOrcExecutionSessionGetJITDylibByName(
        ES: LLVMOrcExecutionSessionRef,
        Name: *const ::libc::c_char,
    ) -> LLVMOrcJITDylibRef;
    pub fn LLVMOrcJITDylibCreateResourceTracker(
        JD: LLVMOrcJITDylibRef,
    ) -> LLVMOrcResourceTrackerRef;
    pub fn LLVMOrcJITDylibGetDefaultResourceTracker(
        JD: LLVMOrcJITDylibRef,
    ) -> LLVMOrcResourceTrackerRef;
    pub fn LLVMOrcJITDylibDefine(
        JD: LLVMOrcJITDylibRef,
        MU: LLVMOrcMaterializationUnitRef,
    ) -> LLVMErrorRef;
    pub fn LLVMOrcJITDylibClear(JD: LLVMOrcJITDylibRef) -> LLVMErrorRef;
    pub fn LLVMOrcJITDylibAddGenerator(JD: LLVMOrcJITDylibRef, DG: LLVMOrcDefinitionGeneratorRef);
    pub fn LLVMOrcCreateCustomCAPIDefinitionGenerator(
        F: LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction,
        Ctx: *mut ::libc::c_void,
    ) -> LLVMOrcDefinitionGeneratorRef;
    pub fn LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess(
        Result: *mut LLVMOrcDefinitionGeneratorRef,
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
    pub fn LLVMOrcDisposeObjectLayer(ObjLayer: LLVMOrcObjectLayerRef);
}
