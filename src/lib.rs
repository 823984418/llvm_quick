pub mod basic_block;
pub mod builder;
pub mod context;
pub mod execution_engine;
pub mod llvm;
pub mod module;
pub mod opaque;
pub mod owning;
pub mod type_tag;
pub mod types;
pub mod util;
pub mod values;

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum CallConv {
    CCallConv = 0,
    FastCallConv = 8,
    ColdCallConv = 9,
    GHCCallConv = 10,
    HiPECallConv = 11,
    AnyRegCallConv = 13,
    PreserveMostCallConv = 14,
    PreserveAllCallConv = 15,
    SwiftCallConv = 16,
    CXXFASTTLSCallConv = 17,
    X86StdcallCallConv = 64,
    X86FastcallCallConv = 65,
    ARMAPCSCallConv = 66,
    ARMAAPCSCallConv = 67,
    ARMAAPCSVFPCallConv = 68,
    MSP430INTRCallConv = 69,
    X86ThisCallCallConv = 70,
    PTXKernelCallConv = 71,
    PTXDeviceCallConv = 72,
    SPIRFUNCCallConv = 75,
    SPIRKERNELCallConv = 76,
    IntelOCLBICallConv = 77,
    X8664SysVCallConv = 78,
    Win64CallConv = 79,
    X86VectorCallCallConv = 80,
    HHVMCallConv = 81,
    HHVMCCallConv = 82,
    X86INTRCallConv = 83,
    AVRINTRCallConv = 84,
    AVRSIGNALCallConv = 85,
    AVRBUILTINCallConv = 86,
    AMDGPUVSCallConv = 87,
    AMDGPUGSCallConv = 88,
    AMDGPUPSCallConv = 89,
    AMDGPUCSCallConv = 90,
    AMDGPUKERNELCallConv = 91,
    X86RegCallCallConv = 92,
    AMDGPUHSCallConv = 93,
    MSP430BUILTINCallConv = 94,
    AMDGPULSCallConv = 95,
    AMDGPUESCallConv = 96,
}


