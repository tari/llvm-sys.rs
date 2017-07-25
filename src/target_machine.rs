//! Target machine information, to generate assembly or object files.

use super::prelude::*;
use super::target::LLVMTargetDataRef;

pub enum LLVMOpaqueTargetMachine {}
pub type LLVMTargetMachineRef = *mut LLVMOpaqueTargetMachine;

pub enum LLVMTarget {}
pub type LLVMTargetRef = *mut LLVMTarget;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMCodeGenOptLevel {
    LLVMCodeGenLevelNone = 0,
    LLVMCodeGenLevelLess = 1,
    LLVMCodeGenLevelDefault = 2,
    LLVMCodeGenLevelAggressive = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMRelocMode {
    LLVMRelocDefault = 0,
    LLVMRelocStatic = 1,
    LLVMRelocPIC = 2,
    LLVMRelocDynamicNoPic = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMCodeModel {
    LLVMCodeModelDefault = 0,
    LLVMCodeModelJITDefault = 1,
    LLVMCodeModelSmall = 2,
    LLVMCodeModelKernel = 3,
    LLVMCodeModelMedium = 4,
    LLVMCodeModelLarge = 5,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMCodeGenFileType {
    LLVMAssemblyFile = 0,
    LLVMObjectFile = 1,
}

extern "C" {
    pub fn LLVMGetFirstTarget() -> LLVMTargetRef;
    pub fn LLVMGetNextTarget(T: LLVMTargetRef) -> LLVMTargetRef;
    pub fn LLVMGetTargetFromName(Name: *const ::libc::c_char) -> LLVMTargetRef;
    pub fn LLVMGetTargetFromTriple(Triple: *const ::libc::c_char,
                                   T: *mut LLVMTargetRef,
                                   ErrorMessage: *mut *mut ::libc::c_char)
                                   -> LLVMBool;
    pub fn LLVMGetTargetName(T: LLVMTargetRef) -> *const ::libc::c_char;
    pub fn LLVMGetTargetDescription(T: LLVMTargetRef) -> *const ::libc::c_char;
    pub fn LLVMTargetHasJIT(T: LLVMTargetRef) -> LLVMBool;
    pub fn LLVMTargetHasTargetMachine(T: LLVMTargetRef) -> LLVMBool;
    pub fn LLVMTargetHasAsmBackend(T: LLVMTargetRef) -> LLVMBool;
    pub fn LLVMCreateTargetMachine(T: LLVMTargetRef,
                                   Triple: *const ::libc::c_char,
                                   CPU: *const ::libc::c_char,
                                   Features: *const ::libc::c_char,
                                   Level: LLVMCodeGenOptLevel,
                                   Reloc: LLVMRelocMode,
                                   CodeModel: LLVMCodeModel)
                                   -> LLVMTargetMachineRef;
    pub fn LLVMDisposeTargetMachine(T: LLVMTargetMachineRef);
    pub fn LLVMGetTargetMachineTarget(T: LLVMTargetMachineRef) -> LLVMTargetRef;
    pub fn LLVMGetTargetMachineTriple(T: LLVMTargetMachineRef) -> *mut ::libc::c_char;
    pub fn LLVMGetTargetMachineCPU(T: LLVMTargetMachineRef) -> *mut ::libc::c_char;
    pub fn LLVMGetTargetMachineFeatureString(T: LLVMTargetMachineRef) -> *mut ::libc::c_char;
    /// Create a DataLayout based on the target machine.
    pub fn LLVMCreateTargetDataLayout(T: LLVMTargetMachineRef) -> LLVMTargetDataRef;
    pub fn LLVMSetTargetMachineAsmVerbosity(T: LLVMTargetMachineRef, VerboseAsm: LLVMBool);
    pub fn LLVMTargetMachineEmitToFile(T: LLVMTargetMachineRef,
                                       M: LLVMModuleRef,
                                       Filename: *mut ::libc::c_char,
                                       codegen: LLVMCodeGenFileType,
                                       ErrorMessage: *mut *mut ::libc::c_char)
                                       -> LLVMBool;
    pub fn LLVMTargetMachineEmitToMemoryBuffer(T: LLVMTargetMachineRef,
                                               M: LLVMModuleRef,
                                               codegen: LLVMCodeGenFileType,
                                               ErrorMessage: *mut *mut ::libc::c_char,
                                               OutMemBuf: *mut LLVMMemoryBufferRef)
                                               -> LLVMBool;
    pub fn LLVMGetDefaultTargetTriple() -> *mut ::libc::c_char;
    pub fn LLVMAddAnalysisPasses(T: LLVMTargetMachineRef, PM: LLVMPassManagerRef);
}
