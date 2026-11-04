use crate::modules::cpu_data::*;
use std::collections::HashMap;
use core::panic;

pub struct CPU {
    register_a: u8,
    register_x: u8,
    register_y: u8,
    cpu_sp    : u8,
    cpu_status: u8,
    cpu_pc    : u16,
    memory: [u8; 0x10000],
    opcode_table: HashMap<u8, (AddressingMode, OpCodeFn)>
}

impl CPU {
    pub fn new() -> CPU {
        let cpu_map = opcodes! {
            //LDA
            0xA9 => (AddressingMode::Immediate, CPU::lda),
            0xA5 => (AddressingMode::ZeroPage,  CPU::lda),
            0xB5 => (AddressingMode::ZeroPage_X, CPU::lda),
            0xAD => (AddressingMode::Absolute, CPU::lda),
            0xBD => (AddressingMode::Absolute_X, CPU::lda),
            0xB9 => (AddressingMode::Absolute_Y, CPU::lda),
            0xA1 => (AddressingMode::Indirect_X, CPU::lda),
            0xB1 => (AddressingMode::Indirect_Y, CPU::lda),

            //LDX
            0xA2 => (AddressingMode::Immediate, CPU::ldx),
            0xA6 => (AddressingMode::ZeroPage,  CPU::ldx),
            0xB6 => (AddressingMode::ZeroPage_Y, CPU::ldx),
            0xAE => (AddressingMode::Absolute,   CPU::ldx),
            0xBE => (AddressingMode::Absolute_Y, CPU::ldx),

            //LDY
            0xA0 => (AddressingMode::Immediate, CPU::ldy),
            0xA4 => (AddressingMode::ZeroPage,  CPU::ldy),
            0xB4 => (AddressingMode::ZeroPage_X, CPU::ldy),
            0xAC => (AddressingMode::Absolute, CPU::ldy),
            0xBC => (AddressingMode::Absolute_X, CPU::ldy),

            //TAX
            0xAA => (AddressingMode::NoneAddressing, CPU::tax),

            //TAY
            0xA8 => (AddressingMode::NoneAddressing, CPU::tay),

            //STA
            0x85 => (AddressingMode::ZeroPage, CPU::sta),
            0x95 => (AddressingMode::ZeroPage_X, CPU::sta),
            0x8D => (AddressingMode::Absolute, CPU::sta),
            0x9D => (AddressingMode::Absolute_X, CPU::sta),
            0x99 => (AddressingMode::Absolute_Y, CPU::sta),
            0x81 => (AddressingMode::Indirect_X, CPU::sta),
            0x91 => (AddressingMode::Indirect_Y, CPU::sta),

            //STX
            0x86 => (AddressingMode::ZeroPage, CPU::stx),
            0x96 => (AddressingMode::ZeroPage_Y, CPU::stx),
            0x8E => (AddressingMode::Absolute, CPU::stx),

            //STY
            0x84 => (AddressingMode::ZeroPage, CPU::sty),
            0x94 => (AddressingMode::ZeroPage_X, CPU::sty),
            0x8C => (AddressingMode::Absolute, CPU::sty),

            //TSX
            0xBA => (AddressingMode::NoneAddressing, CPU::tsx),

            //TXA
            
            //TODO: FINISH OPCODE TABLE.
        };

        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            cpu_sp    : DEFAULT_SP, 
            cpu_status: DEFAULT_STATUS,
            cpu_pc    : 0, 
            memory    : [0; 0x10000],
            opcode_table: cpu_map,
        }
    }

    pub fn reset(&mut self) {
       self.register_a = 0;
       self.register_y = 0;
       self.register_x = 0;
       self.cpu_status = DEFAULT_STATUS;
       self.cpu_sp     = DEFAULT_SP;

       self.cpu_pc     = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }

    pub fn run(&mut self) {
        loop {
            let opcode: u8 = self.mem_read(self.cpu_pc);
            self.cpu_pc += 1;

            if (opcode == 0x00){
                break;
            }
            
            if let Some(&(mode, handler)) = self.opcode_table.get(&opcode) {
                handler(self, &mode);
            }
        }
    }

    //Test Function. Only Use in Tests
    pub (in crate::modules) fn run_test(&mut self, program: Vec<u8>) {
        self.load_and_run(program);
    }

    //Test Function. Only Use in Tests
    pub(in crate::modules) fn set_cpu(&mut self, attr:CpuATTR, val: CpuVal) -> () { // In case need to actualize CPU state outside impl
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

    //Test Function. Only Use in Tests
    pub(in crate::modules) fn get_cpu(&self, attr:CpuATTR) -> CpuVal { // In case need to get CPU state outside impl
        match attr {
            CpuATTR::RegisterA => CpuVal::U8(self.register_a),
            CpuATTR::RegisterX => CpuVal::U8(self.register_x),
            CpuATTR::RegisterY => CpuVal::U8(self.register_y),
            CpuATTR::CpuStatus => CpuVal::U8(self.cpu_status),
            CpuATTR::CpuSP     => CpuVal::U8(self.cpu_sp),
            CpuATTR::CpuPc     => CpuVal::U16(self.cpu_pc)
        }
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.cpu_status = self.cpu_status | 0b0000_0010;
        } else {
            self.cpu_status = self.cpu_status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.cpu_status = self.cpu_status | 0b1000_0000;
        } else {
            self.cpu_status = self.cpu_status & 0b0111_1111;
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

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        return (hi << 8) | (lo as u16)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) -> bool {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);

        if self.mem_read(pos) == lo && self.mem_read(pos + 1) == hi {
            return true;
        }

        return false;
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => {
                let addr = self.cpu_pc;
                self.cpu_pc += 1;
                addr
            }

            AddressingMode::ZeroPage => {
                let addr = self.mem_read(self.cpu_pc) as u16;
                self.cpu_pc += 1;
                addr
            }

            AddressingMode::Absolute => {
                let addr = self.mem_read_u16(self.cpu_pc);
                self.cpu_pc += 2;
                addr
            }

            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.cpu_pc);
                self.cpu_pc += 1;
                pos.wrapping_add(self.register_x) as u16
            }

            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.cpu_pc);
                self.cpu_pc += 1;
                pos.wrapping_add(self.register_y) as u16
            }

            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.cpu_pc);
                self.cpu_pc += 2;
                base.wrapping_add(self.register_x as u16)
            }

            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.cpu_pc);
                self.cpu_pc += 2;
                base.wrapping_add(self.register_y as u16)
            }

            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.cpu_pc);
                self.cpu_pc += 1;

                let ptr = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);

                ((hi as u16) << 8) | (lo as u16)
            }

            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.cpu_pc);
                self.cpu_pc += 1;

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read(base.wrapping_add(1) as u16);
                let deref_base = ((hi as u16) << 8) | (lo as u16);

                deref_base.wrapping_add(self.register_y as u16)
            }

            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn sta(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_x = value;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn stx(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }

    fn ldy(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_y = value;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn sty(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
    }   

    fn tax(&mut self, mode: &AddressingMode){
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn tay(&mut self, mode: &AddressingMode){
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn tsx(&mut self, mode: &AddressingMode){
        self.register_x = self.cpu_sp;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn txa(&mut self, mode: &AddressingMode){
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn txs(&mut self, mode: &AddressingMode){
        self.cpu_sp = self.register_x;
    }

    fn tya(&mut self, mode: &AddressingMode){
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }
    
}
