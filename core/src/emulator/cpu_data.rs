use crate::emulator::cpu::CPU;

pub(in crate::emulator) const DEFAULT_STATUS: u8 = 0x34;

pub(in crate::emulator) const DEFAULT_SP: u8 = 0xFD; 

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub(in crate::emulator) enum AddressingMode {
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

pub(in crate::emulator) enum CpuATTR {
    RegisterA,
    RegisterX,
    RegisterY,
    CpuSP,
    CpuStatus,
    CpuPc,
}

pub(in crate::emulator) enum CpuVal {
    U8(u8),
    U16(u16)
}

pub (in crate::emulator) type OpCodeFn = fn(&mut CPU, &AddressingMode);

#[macro_export]
#[doc(hidden)]
macro_rules! __internal_opcodes {
    ($($code:expr => ($mode:expr, $func:path)),* $(,)?) => {{
        let mut map: HashMap<u8, (AddressingMode, OpCodeFn)> = HashMap::new();
        $(
            map.insert($code, ($mode, $func as OpCodeFn));
        )*
        map
    }};
}

pub(in crate::emulator) use __internal_opcodes as opcodes;

