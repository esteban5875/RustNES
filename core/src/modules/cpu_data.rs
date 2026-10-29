pub(in crate::modules) const DEFAULT_STATUS: u8 = 0x34;

pub(in crate::modules) const DEFAULT_SP: u8 = 0xFD; 

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(in crate::modules) enum AddressingMode {
   Immediate,
   ZeroPage,
   ZeroPage_X,
   ZeroPage_Y,
   Absolute,
   Absolute_X,
   Absolute_Y,
   Indirect_X,
   Indirect_Y,
   NoneAddressing,
}

pub(in crate::modules) enum CpuATTR {
    RegisterA,
    RegisterX,
    RegisterY,
    CpuSP,
    CpuStatus,
    CpuPc,
}

pub(in crate::modules) enum CpuVal {
    U8(u8),
    U16(u16)
}

