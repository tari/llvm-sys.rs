//! Object file reading and writing

use super::prelude::*;

pub enum LLVMOpaqueObjectFile {}
pub type LLVMObjectFileRef = *mut LLVMOpaqueObjectFile;

pub enum LLVMOpaqueSectionIterator {}
pub type LLVMSectionIteratorRef = *mut LLVMOpaqueSectionIterator;

pub enum LLVMOpaqueSymbolIterator {}
pub type LLVMSymbolIteratorRef = *mut LLVMOpaqueSymbolIterator;

pub enum LLVMOpaqueRelocationIterator {}
pub type LLVMRelocationIteratorRef = *mut LLVMOpaqueRelocationIterator;

extern "C" {
    pub fn LLVMCreateObjectFile(MemBuf: LLVMMemoryBufferRef) -> LLVMObjectFileRef;
    pub fn LLVMDisposeObjectFile(ObjectFile: LLVMObjectFileRef);
    pub fn LLVMGetSections(ObjectFile: LLVMObjectFileRef) -> LLVMSectionIteratorRef;
    pub fn LLVMDisposeSectionIterator(SI: LLVMSectionIteratorRef);
    pub fn LLVMIsSectionIteratorAtEnd(ObjectFile: LLVMObjectFileRef,
                                      SI: LLVMSectionIteratorRef)
                                      -> LLVMBool;
    pub fn LLVMMoveToNextSection(SI: LLVMSectionIteratorRef);
    pub fn LLVMMoveToContainingSection(Sect: LLVMSectionIteratorRef, Sym: LLVMSymbolIteratorRef);
    pub fn LLVMGetSymbols(ObjectFile: LLVMObjectFileRef) -> LLVMSymbolIteratorRef;
    pub fn LLVMDisposeSymbolIterator(SI: LLVMSymbolIteratorRef);
    pub fn LLVMIsSymbolIteratorAtEnd(ObjectFile: LLVMObjectFileRef,
                                     SI: LLVMSymbolIteratorRef)
                                     -> LLVMBool;
    pub fn LLVMMoveToNextSymbol(SI: LLVMSymbolIteratorRef);
    pub fn LLVMGetSectionName(SI: LLVMSectionIteratorRef) -> *const ::libc::c_char;
    pub fn LLVMGetSectionSize(SI: LLVMSectionIteratorRef) -> u64;
    pub fn LLVMGetSectionContents(SI: LLVMSectionIteratorRef) -> *const ::libc::c_char;
    pub fn LLVMGetSectionAddress(SI: LLVMSectionIteratorRef) -> u64;
    pub fn LLVMGetSectionContainsSymbol(SI: LLVMSectionIteratorRef,
                                        Sym: LLVMSymbolIteratorRef)
                                        -> LLVMBool;
    pub fn LLVMGetRelocations(Section: LLVMSectionIteratorRef) -> LLVMRelocationIteratorRef;
    pub fn LLVMDisposeRelocationIterator(RI: LLVMRelocationIteratorRef);
    pub fn LLVMIsRelocationIteratorAtEnd(Section: LLVMSectionIteratorRef,
                                         RI: LLVMRelocationIteratorRef)
                                         -> LLVMBool;
    pub fn LLVMMoveToNextRelocation(RI: LLVMRelocationIteratorRef);
    pub fn LLVMGetSymbolName(SI: LLVMSymbolIteratorRef) -> *const ::libc::c_char;
    pub fn LLVMGetSymbolAddress(SI: LLVMSymbolIteratorRef) -> u64;
    pub fn LLVMGetSymbolSize(SI: LLVMSymbolIteratorRef) -> u64;
    pub fn LLVMGetRelocationOffset(RI: LLVMRelocationIteratorRef) -> u64;
    pub fn LLVMGetRelocationSymbol(RI: LLVMRelocationIteratorRef) -> LLVMSymbolIteratorRef;
    pub fn LLVMGetRelocationType(RI: LLVMRelocationIteratorRef) -> u64;
    pub fn LLVMGetRelocationTypeName(RI: LLVMRelocationIteratorRef) -> *const ::libc::c_char;
    pub fn LLVMGetRelocationValueString(RI: LLVMRelocationIteratorRef) -> *const ::libc::c_char;
}
