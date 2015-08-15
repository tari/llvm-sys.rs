//! Abstract link time optimization.
//!
//! This module provides definitions for LTO API version 11.

#![allow(non_camel_case_types)]

pub type lto_bool_t = u8;

// This looks kind of like bitflags but I'm not sure.
#[repr(C)]
pub enum lto_symbol_attributes {
    LTO_SYMBOL_ALIGNMENT_MASK = 31,
    LTO_SYMBOL_PERMISSIONS_MASK = 224,
    LTO_SYMBOL_PERMISSIONS_CODE = 160,
    LTO_SYMBOL_PERMISSIONS_DATA = 192,
    LTO_SYMBOL_PERMISSIONS_RODATA = 128,
    LTO_SYMBOL_DEFINITION_MASK = 1792,
    LTO_SYMBOL_DEFINITION_REGULAR = 256,
    LTO_SYMBOL_DEFINITION_TENTATIVE = 512,
    LTO_SYMBOL_DEFINITION_WEAK = 768,
    LTO_SYMBOL_DEFINITION_UNDEFINED = 1024,
    LTO_SYMBOL_DEFINITION_WEAKUNDEF = 1280,
    LTO_SYMBOL_SCOPE_MASK = 14336,
    LTO_SYMBOL_SCOPE_INTERNAL = 2048,
    LTO_SYMBOL_SCOPE_HIDDEN = 4096,
    LTO_SYMBOL_SCOPE_PROTECTED = 8192,
    LTO_SYMBOL_SCOPE_DEFAULT = 6144,
    LTO_SYMBOL_SCOPE_DEFAULT_CAN_BE_HIDDEN = 10240,
}

#[repr(C)]
pub enum lto_debug_model {
    LTO_DEBUG_MODEL_NONE = 0,
    LTO_DEBUG_MODEL_DWARF = 1
}

#[repr(C)]
pub enum lto_codegen_model {
    LTO_CODEGEN_PIC_MODEL_STATIC = 0,
    LTO_CODEGEN_PIC_MODEL_DYNAMIC = 1,
    LTO_CODEGEN_PIC_MODEL_DYNAMIC_NO_PIC = 2,
    LTO_CODEGEN_PIC_MODEL_DEFAULT = 3,
}

pub enum LLVMOpaqueLTOModule {}
pub type lto_module_t = *mut LLVMOpaqueLTOModule;

pub enum LLVMOpaqueLTOCodeGenerator {}
pub type lto_code_gen_t = *mut LLVMOpaqueLTOCodeGenerator;

#[repr(C)]
pub enum lto_codegen_diagnostic_severity_t {
    LTO_DS_ERROR = 0,
    LTO_DS_WARNING = 1,
    LTO_DS_REMARK = 3,
    LTO_DS_NOTE = 2,
}

pub type lto_diagnostic_handler_t =
    extern "C" fn(severity:
                      lto_codegen_diagnostic_severity_t,
                  diag: *const ::libc::c_char,
                  ctxt: *mut ::libc::c_void) -> ();

extern "C" {
    pub fn lto_get_version() -> *const ::libc::c_char;
    pub fn lto_get_error_message() -> *const ::libc::c_char;
    pub fn lto_module_is_object_file(path: *const ::libc::c_char)
     -> lto_bool_t;
    pub fn lto_module_is_object_file_for_target(path: *const ::libc::c_char,
                                                target_triple_prefix:
                                                    *const ::libc::c_char)
     -> lto_bool_t;
    pub fn lto_module_is_object_file_in_memory(mem: *const ::libc::c_void,
                                               length: ::libc::size_t) -> lto_bool_t;
    pub fn lto_module_is_object_file_in_memory_for_target(mem:
                                                              *const ::libc::c_void,
                                                          length: ::libc::size_t,
                                                          target_triple_prefix:
                                                              *const ::libc::c_char)
     -> lto_bool_t;
    pub fn lto_module_create(path: *const ::libc::c_char) -> lto_module_t;
    pub fn lto_module_create_from_memory(mem: *const ::libc::c_void,
                                         length: ::libc::size_t) -> lto_module_t;
    pub fn lto_module_create_from_memory_with_path(mem: *const ::libc::c_void,
                                                   length: ::libc::size_t,
                                                   path:
                                                       *const ::libc::c_char)
     -> lto_module_t;
    pub fn lto_module_create_in_local_context(mem: *const ::libc::c_void,
                                              length: ::libc::size_t,
                                              path: *const ::libc::c_char)
     -> lto_module_t;
    pub fn lto_module_create_in_codegen_context(mem: *const ::libc::c_void,
                                                length: ::libc::size_t,
                                                path: *const ::libc::c_char,
                                                cg: lto_code_gen_t)
     -> lto_module_t;
    pub fn lto_module_create_from_fd(fd: ::libc::c_int,
                                     path: *const ::libc::c_char,
                                     file_size: ::libc::size_t) -> lto_module_t;
    pub fn lto_module_create_from_fd_at_offset(fd: ::libc::c_int,
                                               path: *const ::libc::c_char,
                                               file_size: ::libc::size_t,
                                               map_size: ::libc::size_t,
                                               offset: ::libc::off_t) -> lto_module_t;
    pub fn lto_module_dispose(_mod: lto_module_t) -> ();
    pub fn lto_module_get_target_triple(_mod: lto_module_t)
     -> *const ::libc::c_char;
    pub fn lto_module_set_target_triple(_mod: lto_module_t,
                                        triple: *const ::libc::c_char) -> ();
    pub fn lto_module_get_num_symbols(_mod: lto_module_t) -> ::libc::c_uint;
    pub fn lto_module_get_symbol_name(_mod: lto_module_t,
                                      index: ::libc::c_uint)
     -> *const ::libc::c_char;
    pub fn lto_module_get_symbol_attribute(_mod: lto_module_t,
                                           index: ::libc::c_uint)
     -> lto_symbol_attributes;
    pub fn lto_module_get_num_deplibs(_mod: lto_module_t) -> ::libc::c_uint;
    pub fn lto_module_get_deplib(_mod: lto_module_t, index: ::libc::c_uint)
     -> *const ::libc::c_char;
    pub fn lto_module_get_num_linkeropts(_mod: lto_module_t)
     -> ::libc::c_uint;
    pub fn lto_module_get_linkeropt(_mod: lto_module_t, index: ::libc::c_uint)
     -> *const ::libc::c_char;
    pub fn lto_codegen_set_diagnostic_handler(arg1: lto_code_gen_t,
                                              arg2: lto_diagnostic_handler_t,
                                              arg3: *mut ::libc::c_void)
     -> ();
    pub fn lto_codegen_create() -> lto_code_gen_t;
    pub fn lto_codegen_create_in_local_context() -> lto_code_gen_t;
    pub fn lto_codegen_dispose(arg1: lto_code_gen_t) -> ();
    pub fn lto_codegen_add_module(cg: lto_code_gen_t, _mod: lto_module_t)
     -> lto_bool_t;
    pub fn lto_codegen_set_debug_model(cg: lto_code_gen_t,
                                       arg1: lto_debug_model) -> lto_bool_t;
    pub fn lto_codegen_set_pic_model(cg: lto_code_gen_t,
                                     arg1: lto_codegen_model) -> lto_bool_t;
    pub fn lto_codegen_set_cpu(cg: lto_code_gen_t, cpu: *const ::libc::c_char)
     -> ();
    pub fn lto_codegen_set_assembler_path(cg: lto_code_gen_t,
                                          path: *const ::libc::c_char) -> ();
    pub fn lto_codegen_set_assembler_args(cg: lto_code_gen_t,
                                          args: *mut *const ::libc::c_char,
                                          nargs: ::libc::c_int) -> ();
    pub fn lto_codegen_add_must_preserve_symbol(cg: lto_code_gen_t,
                                                symbol: *const ::libc::c_char)
     -> ();
    pub fn lto_codegen_write_merged_modules(cg: lto_code_gen_t,
                                            path: *const ::libc::c_char)
     -> lto_bool_t;
    pub fn lto_codegen_compile(cg: lto_code_gen_t, length: *mut ::libc::size_t)
     -> *const ::libc::c_void;
    pub fn lto_codegen_compile_to_file(cg: lto_code_gen_t,
                                       name: *mut *const ::libc::c_char)
     -> lto_bool_t;
    pub fn lto_codegen_debug_options(cg: lto_code_gen_t,
                                     arg1: *const ::libc::c_char) -> ();
    pub fn lto_initialize_disassembler() -> ();
}
