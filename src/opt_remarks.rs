#![allow(non_snake_case)]

use super::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct LLVMOptRemarkStringRef {
    Str: *const libc::c_char,
    Len: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct LLVMOptRemarkDebugLoc {
    SourceFile: LLVMOptRemarkStringRef,
    SourceLineNumber: u32,
    SourceColumnNumber: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct LLVMOptRemarkArg {
    Key: LLVMOptRemarkStringRef,
    Value: LLVMOptRemarkStringRef,
    DebugLoc: LLVMOptRemarkDebugLoc,
}

#[repr(C)]
#[derive(Debug)]
pub struct LLVMOptRemarkEntry {
    RemarkType: LLVMOptRemarkStringRef,
    PassName: LLVMOptRemarkStringRef,
    RemarkName: LLVMOptRemarkStringRef,
    FunctionName: LLVMOptRemarkStringRef,
    DebugLoc: LLVMOptRemarkDebugLoc,
    Hotness: u32,
    NumArgs: u32,
    Args: *mut LLVMOptRemarkArg,
}

#[derive(Debug)]
pub enum LLVMOptRemarkOpaqueParser {}

pub type LLVMOptRemarkParserRef = *mut LLVMOptRemarkOpaqueParser;

extern "C" {
    pub fn LLVMOptRemarkParserCreate(Buf: *const libc::c_void, Size: u64)
        -> LLVMOptRemarkParserRef;
    pub fn LLVMOptRemarkParserGetNext(Parser: LLVMOptRemarkParserRef) -> *mut LLVMOptRemarkEntry;
    pub fn LLVMOptRemarkParserHasError(Parser: LLVMOptRemarkParserRef) -> LLVMBool;
    pub fn LLVMOptRemarkParserGetErrorMessage(
        Parser: LLVMOptRemarkParserRef,
    ) -> *const libc::c_char;
    pub fn LLVMOptRemarkParserDispose(Parser: LLVMOptRemarkParserRef);
    pub fn LLVMOptRemarkVersion() -> u32;
}
