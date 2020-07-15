//! Generation of DWARF debug info.
use super::*;

// Debug info flags.
pub type LLVMDIFlags = ::libc::c_int;
pub const LLVMDIFlagZero: LLVMDIFlags = 0;
pub const LLVMDIFlagPrivate: LLVMDIFlags = 1;
pub const LLVMDIFlagProtected: LLVMDIFlags = 2;
pub const LLVMDIFlagPublic: LLVMDIFlags = 3;
pub const LLVMDIFlagFwdDecl: LLVMDIFlags = 1 << 2;
pub const LLVMDIFlagAppleBlock: LLVMDIFlags = 1 << 3;
pub const LLVMDIFlagBlockByrefStruct: LLVMDIFlags = 1 << 4;
pub const LLVMDIFlagVirtual: LLVMDIFlags = 1 << 5;
pub const LLVMDIFlagArtificial: LLVMDIFlags = 1 << 6;
pub const LLVMDIFlagExplicit: LLVMDIFlags = 1 << 7;
pub const LLVMDIFlagPrototyped: LLVMDIFlags = 1 << 8;
pub const LLVMDIFlagObjcClassComplete: LLVMDIFlags = 1 << 9;
pub const LLVMDIFlagObjectPointer: LLVMDIFlags = 1 << 10;
pub const LLVMDIFlagVector: LLVMDIFlags = 1 << 11;
pub const LLVMDIFlagStaticMember: LLVMDIFlags = 1 << 12;
pub const LLVMDIFlagLValueReference: LLVMDIFlags = 1 << 13;
pub const LLVMDIFlagRValueReference: LLVMDIFlags = 1 << 14;
pub const LLVMDIFlagReserved: LLVMDIFlags = 1 << 15;
pub const LLVMDIFlagSingleInheritance: LLVMDIFlags = 1 << 16;
pub const LLVMDIFlagMultipleInheritance: LLVMDIFlags = 2 << 16;
pub const LLVMDIFlagVirtualInheritance: LLVMDIFlags = 3 << 16;
pub const LLVMDIFlagIntroducedVirtual: LLVMDIFlags = 1 << 18;
pub const LLVMDIFlagBitField: LLVMDIFlags = 1 << 19;
pub const LLVMDIFlagNoReturn: LLVMDIFlags = 1 << 20;
pub const LLVMDIFlagMainSubprogram: LLVMDIFlags = 1 << 21;
pub const LLVMDIFlagIndirectVirtualBase: LLVMDIFlags = (1 << 2) | (1 << 5);
pub const LLVMDIFlagAccessibility: LLVMDIFlags =
    LLVMDIFlagProtected | LLVMDIFlagPrivate | LLVMDIFlagPublic;
pub const LLVMDIFlagPtrToMemberRep: LLVMDIFlags =
    LLVMDIFlagSingleInheritance | LLVMDIFlagMultipleInheritance | LLVMDIFlagVirtualInheritance;

/// Source languages known by DWARF.
#[repr(C)]
pub enum LLVMDWARFSourceLanguage {
    LLVMDWARFSourceLanguageC89,
    LLVMDWARFSourceLanguageC,
    LLVMDWARFSourceLanguageAda83,
    LLVMDWARFSourceLanguageC_plus_plus,
    LLVMDWARFSourceLanguageCobol74,
    LLVMDWARFSourceLanguageCobol85,
    LLVMDWARFSourceLanguageFortran77,
    LLVMDWARFSourceLanguageFortran90,
    LLVMDWARFSourceLanguagePascal83,
    LLVMDWARFSourceLanguageModula2,
    // New in DWARF v3:
    LLVMDWARFSourceLanguageJava,
    LLVMDWARFSourceLanguageC99,
    LLVMDWARFSourceLanguageAda95,
    LLVMDWARFSourceLanguageFortran95,
    LLVMDWARFSourceLanguagePLI,
    LLVMDWARFSourceLanguageObjC,
    LLVMDWARFSourceLanguageObjC_plus_plus,
    LLVMDWARFSourceLanguageUPC,
    LLVMDWARFSourceLanguageD,
    // New in DWARF v4:
    LLVMDWARFSourceLanguagePython,
    // New in DWARF v5:
    LLVMDWARFSourceLanguageOpenCL,
    LLVMDWARFSourceLanguageGo,
    LLVMDWARFSourceLanguageModula3,
    LLVMDWARFSourceLanguageHaskell,
    LLVMDWARFSourceLanguageC_plus_plus_03,
    LLVMDWARFSourceLanguageC_plus_plus_11,
    LLVMDWARFSourceLanguageOCaml,
    LLVMDWARFSourceLanguageRust,
    LLVMDWARFSourceLanguageC11,
    LLVMDWARFSourceLanguageSwift,
    LLVMDWARFSourceLanguageJulia,
    LLVMDWARFSourceLanguageDylan,
    LLVMDWARFSourceLanguageC_plus_plus_14,
    LLVMDWARFSourceLanguageFortran03,
    LLVMDWARFSourceLanguageFortran08,
    LLVMDWARFSourceLanguageRenderScript,
    LLVMDWARFSourceLanguageBLISS,
    // Vendor extensions:
    LLVMDWARFSourceLanguageMips_Assembler,
    LLVMDWARFSourceLanguageGOOGLE_RenderScript,
    LLVMDWARFSourceLanguageBORLAND_Delphi
}

/// The amount of debug information to emit.
#[repr(C)]
pub enum LLVMDWARFEmissionKind {
    LLVMDWARFEmissionKindNone = 0,
    LLVMDWARFEmissionKindFull,
    LLVMDWARFEmissionKindLineTablesOnly,
}

extern "C" {
    /// The current debug metadata version number.
    pub fn LLVMDebugMetadataVersion() -> ::libc::c_uint;
    /// The version of debug metadata that's present in the provided Module.
    pub fn LLVMGetModuleDebugMetadataVersion(Module: LLVMModuleRef) -> ::libc::c_uint;
    /// Strip debug info in the module if it exists.
    pub fn LLVMStripModuleDebugInfo(Module: LLVMModuleRef) -> LLVMBool;
    /// Construct a builder for a module, do not allow unresolved nodes.
    pub fn LLVMCreateDIBuilderDisallowUnresolved(M: LLVMModuleRef) -> LLVMDIBuilderRef;
    /// Construct a builder for a module and collect unresolved nodes.
    pub fn LLVMCreateDIBuilder(M: LLVMModuleRef) -> LLVMDIBuilderRef;
    /// Deallocate a builder and everything it owns.
    /// 
    /// The builder must be finalized before this.
    pub fn LLVMDisposeDIBuilder(Builder: LLVMDIBuilderRef);
    /// Construct any deferred debug info descriptors.
    pub fn LLVMDIBuilderFinalize(Builder: LLVMDIBuilderRef);
    pub fn LLVMDIBuilderCreateCompileUnit(Builder: LLVMDIBuilderRef,
                                       Lang: LLVMDWARFSourceLanguage,
                                       FileRef: LLVMMetadataRef,
                                       Producer: *const ::libc::c_char,
                                       ProducerLen: ::libc::size_t,
                                       isOptimized: LLVMBool,
                                       Flags: *const ::libc::c_char,
                                       FlagsLen: ::libc::size_t,
                                       RuntimeVer: ::libc::c_uint,
                                       SplitName: *const ::libc::c_char,
                                       SplitNameLen: ::libc::size_t,
                                       Kind: LLVMDWARFEmissionKind,
                                       DWOId: ::libc::c_uint,
                                       SplitDebugInlining: LLVMBool,
                                       DebugInfoForProfiling: LLVMBool) -> LLVMMetadataRef;
    /// Create a file descriptor to hold debugging information for a file.
    pub fn LLVMDIBuilderCreateFile(Builder: LLVMDIBuilderRef,
                                Filename: *const ::libc::c_char,
                                FilenameLen: ::libc::size_t,
                                Directory: *const ::libc::c_char,
                                DirectoryLen: ::libc::size_t) -> LLVMMetadataRef;
    /// Creates a new DebugLocation that describes a source location.
    pub fn LLVMDIBuilderCreateDebugLocation(Ctx: LLVMContextRef,
                                         Line: ::libc::c_uint,
                                         Column: ::libc::c_uint,
                                         Scope: LLVMMetadataRef,
                                         InlinedAt: LLVMMetadataRef) -> LLVMMetadataRef;

}
