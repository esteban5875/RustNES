use core::panic;
use std::process;

use crate::modules::cpu;

const DEFAULT_STATUS: u8 = 0x34;
const DEFAULT_SP: u8 = 0xFD; 
pub enum CpuATTR {
    RegisterA,
    RegisterX,
    RegisterY,
    CpuSP,
    CpuStatus,
    CpuPc,
}

pub enum CpuVal {
    U8(u8),
    U16(u16)
}

pub struct CPU {
    register_a: u8,
    register_x: u8,
    register_y: u8,
    cpu_sp    : u8,
    cpu_status: u8,
    cpu_pc    : u16,
    memory: [u8; 0xFFFF]
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            cpu_sp    : DEFAULT_SP, 
            cpu_status: DEFAULT_STATUS,
            cpu_pc    : 0, 
            memory    : [0; 0xFFFF]
        }
    }

    pub fn setCPU(&mut self, attr:CpuATTR, val: CpuVal) -> () { // In case need to actualize CPU state outside impl
        match attr {
            CpuATTR::RegisterA => {
                if let CpuVal::U8(v) = val {
                    self.register_a = v;
                } else {
                    panic!("Tried to assign a 16 bit integer to a 8 bit register -> Register A.")
                }
            }

            CpuATTR::RegisterX => {
                if let CpuVal::U8(v) = val {
                    self.register_x = v;
                } else {
                    panic!("Tried to assign a 16 bit integer to a 8 bit register -> Register X.")
                }
            }

            CpuATTR::RegisterY => {
                if let CpuVal::U8(v) = val {
                    self.register_y = v;
                } else {
                    panic!("Tried to assign a 16 bit integer to a 8 bit register -> Register Y.")
                }
            }

            CpuATTR::CpuStatus => {
                if let CpuVal::U8(v) = val {
                    self.cpu_status = v;
                } else {
                    panic!("Tried to assign a 16 bit integer to a 8 bit register -> CPU Status.")
                }
            }

            CpuATTR::CpuSP => {
                if let CpuVal::U8(v) = val {
                    self.cpu_sp = v;
                } else {
                    panic!("Tried to assign a 16 bit integer to a 8 bit register -> CPU Stack Pointer.")
                }
            }

            CpuATTR::CpuPc => {
                self.cpu_pc = match val {
                    CpuVal::U8(v) => v as u16,
                    CpuVal::U16(v) => v,
                    _ => self.cpu_pc,
                };
            }
        }
    }

    pub fn getCPU(&self, attr:CpuATTR) -> CpuVal { // In case need to get CPU state outside impl
        match attr {
            CpuATTR::RegisterA => CpuVal::U8(self.register_a),
            CpuATTR::RegisterX => CpuVal::U8(self.register_x),
            CpuATTR::RegisterY => CpuVal::U8(self.register_y),
            CpuATTR::CpuStatus => CpuVal::U8(self.cpu_status),
            CpuATTR::CpuSP     => CpuVal::U8(self.cpu_sp),
            CpuATTR::CpuPc     => CpuVal::U16(self.cpu_pc)
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }

    fn mem_write(&mut self, addr: u16, data: u8) -> bool{
        self.memory[addr as usize] = data;
        if self.mem_read(addr) == data {
            return true;
        }

        return false;
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        loop {
            let opcode = program[self.cpu_pc as usize];
            self.cpu_pc += 1;

            match opcode {
                0xA9 => {
                    let param = program[self.cpu_pc as usize];
                    self.cpu_pc += 1;
                    self.register_a = param;

                    if self.register_a == 0 {
                        self.cpu_status = self.cpu_status | 0b0000_0010;
                    } else {
                        self.cpu_status = self.cpu_status & 0b1111_1101;
                    }

                    if self.register_a & 0b1000_0000 != 0 {
                        self.cpu_status = self.cpu_status | 0b1000_0000;
                    } else {
                        self.cpu_status = self.cpu_status & 0b0111_1111;
                    }
                }

                0x00 => {
                    return;
                }
                
                _ => todo!()
            }
        }
    }
}
