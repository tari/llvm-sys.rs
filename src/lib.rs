//! Bindings to LLVM's C API, version 3.9.
//!
//! Refer to the [LLVM documentation](http://llvm.org/docs/) for more
//! information.

#![allow(non_upper_case_globals)]

extern crate libc;
#[macro_use]
extern crate bitflags;

use self::prelude::*;

pub enum LLVMMemoryBuffer {}
pub enum LLVMContext {}
pub enum LLVMModule {}
pub enum LLVMType {}
pub enum LLVMValue {}
pub enum LLVMBasicBlock {}
pub enum LLVMBuilder {}
pub enum LLVMModuleProvider {}
pub enum LLVMPassManager {}
pub enum LLVMPassRegistry {}
pub enum LLVMUse {}
pub enum LLVMDiagnosticInfo {}
pub enum LLVMOpaqueAttributeRef {}

/// Core types used throughout LLVM.
///
/// In most cases you will want to `use llvm::prelude::*`.
pub mod prelude {
    pub type LLVMBool = ::libc::c_int;
    pub type LLVMMemoryBufferRef = *mut super::LLVMMemoryBuffer;
    pub type LLVMContextRef = *mut super::LLVMContext;
    pub type LLVMModuleRef = *mut super::LLVMModule;
    pub type LLVMTypeRef = *mut super::LLVMType;
    pub type LLVMValueRef = *mut super::LLVMValue;
    pub type LLVMBasicBlockRef = *mut super::LLVMBasicBlock;
    pub type LLVMBuilderRef = *mut super::LLVMBuilder;
    pub type LLVMModuleProviderRef = *mut super::LLVMModuleProvider;
    pub type LLVMPassManagerRef = *mut super::LLVMPassManager;
    pub type LLVMPassRegistryRef = *mut super::LLVMPassRegistry;
    pub type LLVMUseRef = *mut super::LLVMUse;
    pub type LLVMDiagnosticInfoRef = *mut super::LLVMDiagnosticInfo;
    pub type LLVMAttributeRef = *mut super::LLVMOpaqueAttributeRef;
}

pub mod analysis;
pub mod bit_reader;
pub mod bit_writer;
pub mod core;
pub mod disassembler;
pub mod error_handling;
pub mod execution_engine;
pub mod initialization;
pub mod ir_reader;
pub mod link_time_optimizer;
pub mod linker;
pub mod lto;
pub mod object;
pub mod orc;
pub mod target;
pub mod support;
pub mod target_machine;

pub mod transforms {
    pub mod ipo;
    pub mod pass_manager_builder;
    pub mod scalar;
    pub mod vectorize;
}

bitflags! {
    #[repr(C)]
    pub flags LLVMAttribute: ::libc::c_int {
        #[allow(non_upper_case_globals)] 
        const LLVMZExtAttribute = 1 << 0,
        #[allow(non_upper_case_globals)] 
        const LLVMSExtAttribute = 1 << 1,
        #[allow(non_upper_case_globals)] 
        const LLVMNoReturnAttribute= 4,
        #[allow(non_upper_case_globals)] 
        const LLVMInRegAttribute= 8,
        #[allow(non_upper_case_globals)] 
        const LLVMStructRetAttribute= 16,
        #[allow(non_upper_case_globals)] 
        const LLVMNoUnwindAttribute= 32,
        #[allow(non_upper_case_globals)] 
        const LLVMNoAliasAttribute= 64,
        #[allow(non_upper_case_globals)] 
        const LLVMByValAttribute= 128,
        #[allow(non_upper_case_globals)] 
        const LLVMNestAttribute= 256,
        #[allow(non_upper_case_globals)] 
        const LLVMReadNoneAttribute= 512,
        #[allow(non_upper_case_globals)] 
        const LLVMReadOnlyAttribute= 1024,
        #[allow(non_upper_case_globals)] 
        const LLVMNoInlineAttribute= 2048,
        #[allow(non_upper_case_globals)] 
        const LLVMAlwaysInlineAttribute= 4096,
        #[allow(non_upper_case_globals)] 
        const LLVMOptimizeForSizeAttribute= 8192,
        #[allow(non_upper_case_globals)] 
        const LLVMStackProtectAttribute= 16384,
        #[allow(non_upper_case_globals)] 
        const LLVMStackProtectReqAttribute= 32768,
        #[allow(non_upper_case_globals)] 
        const LLVMAlignment= 2031616,
        #[allow(non_upper_case_globals)] 
        const LLVMNoCaptureAttribute= 2097152,
        #[allow(non_upper_case_globals)] 
        const LLVMNoRedZoneAttribute= 4194304,
        #[allow(non_upper_case_globals)] 
        const LLVMNoImplicitFloatAttribute= 8388608,
        #[allow(non_upper_case_globals)] 
        const LLVMNakedAttribute= 16777216,
        #[allow(non_upper_case_globals)] 
        const LLVMInlineHintAttribute= 33554432,
        #[allow(non_upper_case_globals)] 
        const LLVMStackAlignment= 469762048,
        #[allow(non_upper_case_globals)] 
        const LLVMReturnsTwice= 536870912,
        #[allow(non_upper_case_globals)] 
        const LLVMUWTable= 1073741824,
        #[allow(non_upper_case_globals)] 
        const LLVMNonLazyBind = 1 << 31
    }
}

#[repr(C)]
pub enum LLVMOpcode {
    LLVMRet = 1,
    LLVMBr = 2,
    LLVMSwitch = 3,
    LLVMIndirectBr = 4,
    LLVMInvoke = 5,
    LLVMUnreachable = 7,
    LLVMAdd = 8,
    LLVMFAdd = 9,
    LLVMSub = 10,
    LLVMFSub = 11,
    LLVMMul = 12,
    LLVMFMul = 13,
    LLVMUDiv = 14,
    LLVMSDiv = 15,
    LLVMFDiv = 16,
    LLVMURem = 17,
    LLVMSRem = 18,
    LLVMFRem = 19,
    LLVMShl = 20,
    LLVMLShr = 21,
    LLVMAShr = 22,
    LLVMAnd = 23,
    LLVMOr = 24,
    LLVMXor = 25,
    LLVMAlloca = 26,
    LLVMLoad = 27,
    LLVMStore = 28,
    LLVMGetElementPtr = 29,
    LLVMTrunc = 30,
    LLVMZExt = 31,
    LLVMSExt = 32,
    LLVMFPToUI = 33,
    LLVMFPToSI = 34,
    LLVMUIToFP = 35,
    LLVMSIToFP = 36,
    LLVMFPTrunc = 37,
    LLVMFPExt = 38,
    LLVMPtrToInt = 39,
    LLVMIntToPtr = 40,
    LLVMBitCast = 41,
    LLVMAddrSpaceCast = 60,
    LLVMICmp = 42,
    LLVMFCmp = 43,
    LLVMPHI = 44,
    LLVMCall = 45,
    LLVMSelect = 46,
    LLVMUserOp1 = 47,
    LLVMUserOp2 = 48,
    LLVMVAArg = 49,
    LLVMExtractElement = 50,
    LLVMInsertElement = 51,
    LLVMShuffleVector = 52,
    LLVMExtractValue = 53,
    LLVMInsertValue = 54,
    LLVMFence = 55,
    LLVMAtomicCmpXchg = 56,
    LLVMAtomicRMW = 57,
    LLVMResume = 58,
    LLVMLandingPad = 59,
    LLVMCleanupRet = 61,
    LLVMCatchRet = 62,
    LLVMCatchPad = 63,
    LLVMCleanupPad = 64,
    LLVMCatchSwitch = 65,
}

#[repr(C)]
pub enum LLVMTypeKind {
    LLVMVoidTypeKind = 0,
    LLVMHalfTypeKind = 1,
    LLVMFloatTypeKind = 2,
    LLVMDoubleTypeKind = 3,
    LLVMX86_FP80TypeKind = 4,
    LLVMFP128TypeKind = 5,
    LLVMPPC_FP128TypeKind = 6,
    LLVMLabelTypeKind = 7,
    LLVMIntegerTypeKind = 8,
    LLVMFunctionTypeKind = 9,
    LLVMStructTypeKind = 10,
    LLVMArrayTypeKind = 11,
    LLVMPointerTypeKind = 12,
    LLVMVectorTypeKind = 13,
    LLVMMetadataTypeKind = 14,
    LLVMX86_MMXTypeKind = 15,
    LLVMTokenTypeKind = 16,
}

#[repr(C)]
pub enum LLVMLinkage {
    LLVMExternalLinkage = 0,
    LLVMAvailableExternallyLinkage = 1,
    LLVMLinkOnceAnyLinkage = 2,
    LLVMLinkOnceODRLinkage = 3,
    LLVMLinkOnceODRAutoHideLinkage = 4,
    LLVMWeakAnyLinkage = 5,
    LLVMWeakODRLinkage = 6,
    LLVMAppendingLinkage = 7,
    LLVMInternalLinkage = 8,
    LLVMPrivateLinkage = 9,
    LLVMDLLImportLinkage = 10,
    LLVMDLLExportLinkage = 11,
    LLVMExternalWeakLinkage = 12,
    LLVMGhostLinkage = 13,
    LLVMCommonLinkage = 14,
    LLVMLinkerPrivateLinkage = 15,
    LLVMLinkerPrivateWeakLinkage = 16,
}

#[repr(C)]
pub enum LLVMVisibility {
    LLVMDefaultVisibility = 0,
    LLVMHiddenVisibility = 1,
    LLVMProtectedVisibility = 2,
}

#[repr(C)]
pub enum LLVMDLLStorageClass {
    LLVMDefaultStorageClass = 0,
    LLVMDLLImportStorageClass = 1,
    LLVMDLLExportStorageClass = 2,
}

#[repr(C)]
pub enum LLVMCallConv {
    LLVMCCallConv = 0,
    LLVMFastCallConv = 8,
    LLVMColdCallConv = 9,
    LLVMWebKitJSCallConv = 12,
    LLVMAnyRegCallConv = 13,
    LLVMX86StdcallCallConv = 64,
    LLVMX86FastcallCallConv = 65,
}

#[repr(C)]
pub enum LLVMValueKind {
    LLVMArgumentValueKind,
    LLVMBasicBlockValueKind,
    LLVMMemoryUseValueKind,
    LLVMMemoryDefValueKind,
    LLVMMemoryPhiValueKind,

    LLVMFunctionValueKind,
    LLVMGlobalAliasValueKind,
    LLVMGlobalIFuncValueKind,
    LLVMGlobalVariableValueKind,
    LLVMBlockAddressValueKind,
    LLVMConstantExprValueKind,
    LLVMConstantArrayValueKind,
    LLVMConstantStructValueKind,
    LLVMConstantVectorValueKind,
    LLVMUndefValueValueKind,
    LLVMConstantAggregateZeroValueKind,
    LLVMConstantDataArrayValueKind,
    LLVMConstantDataVectorValueKind,
    LLVMConstantIntValueKind,
    LLVMConstantFPValueKind,
    LLVMConstantPointerNullValueKind,
    LLVMConstantTokenNoneValueKind,

    LLVMMetadataAsValueValueKind,
    LLVMInlineAsmValueKind,

    LLVMInstructionValueKind,
}

#[repr(C)]
pub enum LLVMIntPredicate {
    LLVMIntEQ = 32,
    LLVMIntNE = 33,
    LLVMIntUGT = 34,
    LLVMIntUGE = 35,
    LLVMIntULT = 36,
    LLVMIntULE = 37,
    LLVMIntSGT = 38,
    LLVMIntSGE = 39,
    LLVMIntSLT = 40,
    LLVMIntSLE = 41,
}

#[repr(C)]
pub enum LLVMRealPredicate {
    LLVMRealPredicateFalse = 0,
    LLVMRealOEQ = 1,
    LLVMRealOGT = 2,
    LLVMRealOGE = 3,
    LLVMRealOLT = 4,
    LLVMRealOLE = 5,
    LLVMRealONE = 6,
    LLVMRealORD = 7,
    LLVMRealUNO = 8,
    LLVMRealUEQ = 9,
    LLVMRealUGT = 10,
    LLVMRealUGE = 11,
    LLVMRealULT = 12,
    LLVMRealULE = 13,
    LLVMRealUNE = 14,
    LLVMRealPredicateTrue = 15,
}

#[repr(C)]
pub enum LLVMLandingPadClauseTy {
    LLVMLandingPadCatch = 0,
    LLVMLandingPadFilter = 1,
}

#[repr(C)]
pub enum LLVMThreadLocalMode {
    LLVMNotThreadLocal = 0,
    LLVMGeneralDynamicTLSModel = 1,
    LLVMLocalDynamicTLSModel = 2,
    LLVMInitialExecTLSModel = 3,
    LLVMLocalExecTLSModel = 4,
}

#[repr(C)]
pub enum LLVMAtomicOrdering {
    LLVMAtomicOrderingNotAtomic = 0,
    LLVMAtomicOrderingUnordered = 1,
    LLVMAtomicOrderingMonotonic = 2,
    LLVMAtomicOrderingAcquire = 4,
    LLVMAtomicOrderingRelease = 5,
    LLVMAtomicOrderingAcquireRelease = 6,
    LLVMAtomicOrderingSequentiallyConsistent = 7,
}

#[repr(C)]
pub enum LLVMAtomicRMWBinOp {
    LLVMAtomicRMWBinOpXchg = 0,
    LLVMAtomicRMWBinOpAdd = 1,
    LLVMAtomicRMWBinOpSub = 2,
    LLVMAtomicRMWBinOpAnd = 3,
    LLVMAtomicRMWBinOpNand = 4,
    LLVMAtomicRMWBinOpOr = 5,
    LLVMAtomicRMWBinOpXor = 6,
    LLVMAtomicRMWBinOpMax = 7,
    LLVMAtomicRMWBinOpMin = 8,
    LLVMAtomicRMWBinOpUMax = 9,
    LLVMAtomicRMWBinOpUMin = 10,
}

#[repr(C)]
pub enum LLVMDiagnosticSeverity {
    LLVMDSError = 0,
    LLVMDSWarning = 1,
    LLVMDSRemark = 2,
    LLVMDSNote = 3,
}

pub const LLVMAttributeReturnIndex: ::libc::c_uint = 0;
pub const LLVMAttributeFunctionIndex: ::libc::c_uint = !0; // -1
/// Either LLVMAttributeReturnIndex, LLVMAttributeFunctionIndex, or a parameter
/// number from 1 to N.
pub type LLVMAttributeIndex = ::libc::c_uint;

pub type LLVMDiagnosticHandler = Option<extern "C" fn(arg1: LLVMDiagnosticInfoRef,
                                                      arg2: *mut ::libc::c_void)>;
pub type LLVMYieldCallback = Option<extern "C" fn(arg1: LLVMContextRef, arg2: *mut ::libc::c_void)>;
