//! Generation of DWARF debug info.
use super::*;

/// Debug info flags.
#[repr(C)]
pub enum LLVMDIFlags {
    LLVMDIFlagZero = 0,
    LLVMDIFlagPrivate = 1,
    LLVMDIFlagProtected = 2,
    LLVMDIFlagPublic = 3,
    LLVMDIFlagFwdDecl = 1 << 2,
    LLVMDIFlagAppleBlock = 1 << 3,
    LLVMDIFlagBlockByrefStruct = 1 << 4,
    LLVMDIFlagVirtual = 1 << 5,
    LLVMDIFlagArtificial = 1 << 6,
    LLVMDIFlagExplicit = 1 << 7,
    LLVMDIFlagPrototyped = 1 << 8,
    LLVMDIFlagObjcClassComplete = 1 << 9,
    LLVMDIFlagObjectPointer = 1 << 10,
    LLVMDIFlagVector = 1 << 11,
    LLVMDIFlagStaticMember = 1 << 12,
    LLVMDIFlagLValueReference = 1 << 13,
    LLVMDIFlagRValueReference = 1 << 14,
    LLVMDIFlagReserved = 1 << 15,
    LLVMDIFlagSingleInheritance = 1 << 16,
    LLVMDIFlagMultipleInheritance = 2 << 16,
    LLVMDIFlagVirtualInheritance = 3 << 16,
    LLVMDIFlagIntroducedVirtual = 1 << 18,
    LLVMDIFlagBitField = 1 << 19,
    LLVMDIFlagNoReturn = 1 << 20,
    LLVMDIFlagMainSubprogram = 1 << 21,
    LLVMDIFlagIndirectVirtualBase = (1 << 2) | (1 << 5),
}

pub const LLVMDIFlagAccessibility: LLVMDIFlags = LLVMDIFlags::LLVMDIFlagPublic;
pub const LLVMDIFlagPtrToMemberRep: LLVMDIFlags = LLVMDIFlags::LLVMDIFlagVirtualInheritance;

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
