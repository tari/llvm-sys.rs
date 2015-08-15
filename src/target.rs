//! Target information

use super::prelude::*;

#[repr(C)]
pub enum LLVMByteOrdering {
    LLVMBigEndian = 0,
    LLVMLittleEndian = 1
}

pub enum LLVMOpaqueTargetData {}
pub type LLVMTargetDataRef = *mut LLVMOpaqueTargetData;

pub enum LLVMOpaqueTargetLibraryInfotData {}
pub type LLVMTargetLibraryInfoRef = *mut LLVMOpaqueTargetLibraryInfotData;

extern "C" {
    pub fn LLVMInitializeR600TargetInfo() -> ();
    pub fn LLVMInitializeSystemZTargetInfo() -> ();
    pub fn LLVMInitializeHexagonTargetInfo() -> ();
    pub fn LLVMInitializeNVPTXTargetInfo() -> ();
    pub fn LLVMInitializeCppBackendTargetInfo() -> ();
    pub fn LLVMInitializeMSP430TargetInfo() -> ();
    pub fn LLVMInitializeXCoreTargetInfo() -> ();
    pub fn LLVMInitializeMipsTargetInfo() -> ();
    pub fn LLVMInitializeAArch64TargetInfo() -> ();
    pub fn LLVMInitializeARMTargetInfo() -> ();
    pub fn LLVMInitializePowerPCTargetInfo() -> ();
    pub fn LLVMInitializeSparcTargetInfo() -> ();
    pub fn LLVMInitializeX86TargetInfo() -> ();
    pub fn LLVMInitializeR600Target() -> ();
    pub fn LLVMInitializeSystemZTarget() -> ();
    pub fn LLVMInitializeHexagonTarget() -> ();
    pub fn LLVMInitializeNVPTXTarget() -> ();
    pub fn LLVMInitializeCppBackendTarget() -> ();
    pub fn LLVMInitializeMSP430Target() -> ();
    pub fn LLVMInitializeXCoreTarget() -> ();
    pub fn LLVMInitializeMipsTarget() -> ();
    pub fn LLVMInitializeAArch64Target() -> ();
    pub fn LLVMInitializeARMTarget() -> ();
    pub fn LLVMInitializePowerPCTarget() -> ();
    pub fn LLVMInitializeSparcTarget() -> ();
    pub fn LLVMInitializeX86Target() -> ();
    pub fn LLVMInitializeR600TargetMC() -> ();
    pub fn LLVMInitializeSystemZTargetMC() -> ();
    pub fn LLVMInitializeHexagonTargetMC() -> ();
    pub fn LLVMInitializeNVPTXTargetMC() -> ();
    pub fn LLVMInitializeCppBackendTargetMC() -> ();
    pub fn LLVMInitializeMSP430TargetMC() -> ();
    pub fn LLVMInitializeXCoreTargetMC() -> ();
    pub fn LLVMInitializeMipsTargetMC() -> ();
    pub fn LLVMInitializeAArch64TargetMC() -> ();
    pub fn LLVMInitializeARMTargetMC() -> ();
    pub fn LLVMInitializePowerPCTargetMC() -> ();
    pub fn LLVMInitializeSparcTargetMC() -> ();
    pub fn LLVMInitializeX86TargetMC() -> ();
    pub fn LLVMInitializeR600AsmPrinter() -> ();
    pub fn LLVMInitializeSystemZAsmPrinter() -> ();
    pub fn LLVMInitializeHexagonAsmPrinter() -> ();
    pub fn LLVMInitializeNVPTXAsmPrinter() -> ();
    pub fn LLVMInitializeMSP430AsmPrinter() -> ();
    pub fn LLVMInitializeXCoreAsmPrinter() -> ();
    pub fn LLVMInitializeMipsAsmPrinter() -> ();
    pub fn LLVMInitializeAArch64AsmPrinter() -> ();
    pub fn LLVMInitializeARMAsmPrinter() -> ();
    pub fn LLVMInitializePowerPCAsmPrinter() -> ();
    pub fn LLVMInitializeSparcAsmPrinter() -> ();
    pub fn LLVMInitializeX86AsmPrinter() -> ();
    pub fn LLVMInitializeR600AsmParser() -> ();
    pub fn LLVMInitializeSystemZAsmParser() -> ();
    pub fn LLVMInitializeMipsAsmParser() -> ();
    pub fn LLVMInitializeAArch64AsmParser() -> ();
    pub fn LLVMInitializeARMAsmParser() -> ();
    pub fn LLVMInitializePowerPCAsmParser() -> ();
    pub fn LLVMInitializeSparcAsmParser() -> ();
    pub fn LLVMInitializeX86AsmParser() -> ();
    pub fn LLVMInitializeSystemZDisassembler() -> ();
    pub fn LLVMInitializeHexagonDisassembler() -> ();
    pub fn LLVMInitializeXCoreDisassembler() -> ();
    pub fn LLVMInitializeMipsDisassembler() -> ();
    pub fn LLVMInitializeAArch64Disassembler() -> ();
    pub fn LLVMInitializeARMDisassembler() -> ();
    pub fn LLVMInitializePowerPCDisassembler() -> ();
    pub fn LLVMInitializeSparcDisassembler() -> ();
    pub fn LLVMInitializeX86Disassembler() -> ();
}

extern "C" {
    pub fn LLVMCreateTargetData(StringRep: *const ::libc::c_char)
     -> LLVMTargetDataRef;
    pub fn LLVMAddTargetData(TD: LLVMTargetDataRef, PM: LLVMPassManagerRef)
     -> ();
    pub fn LLVMAddTargetLibraryInfo(TLI: LLVMTargetLibraryInfoRef,
                                    PM: LLVMPassManagerRef) -> ();
    pub fn LLVMCopyStringRepOfTargetData(TD: LLVMTargetDataRef)
     -> *mut ::libc::c_char;
    pub fn LLVMByteOrder(TD: LLVMTargetDataRef) -> LLVMByteOrdering;
    pub fn LLVMPointerSize(TD: LLVMTargetDataRef) -> ::libc::c_uint;
    pub fn LLVMPointerSizeForAS(TD: LLVMTargetDataRef, AS: ::libc::c_uint)
     -> ::libc::c_uint;
    pub fn LLVMIntPtrType(TD: LLVMTargetDataRef) -> LLVMTypeRef;
    pub fn LLVMIntPtrTypeForAS(TD: LLVMTargetDataRef, AS: ::libc::c_uint)
     -> LLVMTypeRef;
    pub fn LLVMIntPtrTypeInContext(C: LLVMContextRef, TD: LLVMTargetDataRef)
     -> LLVMTypeRef;
    pub fn LLVMIntPtrTypeForASInContext(C: LLVMContextRef,
                                        TD: LLVMTargetDataRef,
                                        AS: ::libc::c_uint) -> LLVMTypeRef;
    pub fn LLVMSizeOfTypeInBits(TD: LLVMTargetDataRef, Ty: LLVMTypeRef)
     -> ::libc::c_ulonglong;
    pub fn LLVMStoreSizeOfType(TD: LLVMTargetDataRef, Ty: LLVMTypeRef)
     -> ::libc::c_ulonglong;
    pub fn LLVMABISizeOfType(TD: LLVMTargetDataRef, Ty: LLVMTypeRef)
     -> ::libc::c_ulonglong;
    pub fn LLVMABIAlignmentOfType(TD: LLVMTargetDataRef, Ty: LLVMTypeRef)
     -> ::libc::c_uint;
    pub fn LLVMCallFrameAlignmentOfType(TD: LLVMTargetDataRef,
                                        Ty: LLVMTypeRef) -> ::libc::c_uint;
    pub fn LLVMPreferredAlignmentOfType(TD: LLVMTargetDataRef,
                                        Ty: LLVMTypeRef) -> ::libc::c_uint;
    pub fn LLVMPreferredAlignmentOfGlobal(TD: LLVMTargetDataRef,
                                          GlobalVar: LLVMValueRef)
     -> ::libc::c_uint;
    pub fn LLVMElementAtOffset(TD: LLVMTargetDataRef, StructTy: LLVMTypeRef,
                               Offset: ::libc::c_ulonglong) -> ::libc::c_uint;
    pub fn LLVMOffsetOfElement(TD: LLVMTargetDataRef, StructTy: LLVMTypeRef,
                               Element: ::libc::c_uint)
     -> ::libc::c_ulonglong;
    pub fn LLVMDisposeTargetData(TD: LLVMTargetDataRef) -> ();
}

// Functions from our target wrappers, since the C interface defines them with
// macros (wrappers/target.c).
extern "C" {
    pub fn LLVM_InitializeAllTargetInfos();
    pub fn LLVM_InitializeAllTargets();
    pub fn LLVM_InitializeAllTargetMCs();
    pub fn LLVM_InitializeAllAsmPrinters();
    pub fn LLVM_InitializeAllAsmParsers();
    pub fn LLVM_InitializeAllDisassemblers();

    /// Returns 1 on failure.
    pub fn LLVM_InitializeNativeTarget() -> LLVMBool;
    /// Returns 1 on failure.
    pub fn LLVM_InitializeNativeAsmParser() -> LLVMBool;
    /// Returns 1 on failure.
    pub fn LLVM_InitializeNativeAsmPrinter() -> LLVMBool;
    /// Returns 1 on failure.
    pub fn LLVM_InitializeNativeDisassembler() -> LLVMBool;
}
