pub const LLVMErrorSuccess: ::libc::c_int = 0;

#[derive(Debug)]
pub enum LLVMOpaqueError {}

pub type LLVMErrorRef = *mut LLVMOpaqueError;

pub type LLVMErrorTypeId = *const ::libc::c_void;

extern "C" {
    pub fn LLVMGetErrorTypeId(Err: LLVMErrorRef) -> LLVMErrorTypeId;
    pub fn LLVMConsumeError(Err: LLVMErrorRef);
    ///  Report a fatal error if Err is a failure value.
    ///
    /// This function can be used to wrap calls to fallible functions ONLY when it is
    /// known that the Error will always be a success value.
    pub fn LLVMCantFail(Err: LLVMErrorRef);
    pub fn LLVMGetErrorMessage(Err: LLVMErrorRef) -> *mut ::libc::c_char;
    pub fn LLVMDisposeErrorMessage(ErrMsg: *mut ::libc::c_char);
    pub fn LLVMGetStringErrorTypeId() -> LLVMErrorTypeId;
    /// Create a StringError.
    pub fn LLVMCreateStringError(ErrMst: *const ::libc::c_char) -> LLVMErrorRef;
}
