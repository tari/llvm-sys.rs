use super::prelude::*;

extern "C" {
    pub fn LLVMLoadLibraryPermanently(Filename: *const ::libc::c_char) -> LLVMBool;
    pub fn LLVMParseCommandLineOptions(argc: ::libc::c_int,
                                       argv: *const *const ::libc::c_char,
                                       Overview: *const ::libc::c_char) -> ();
}
