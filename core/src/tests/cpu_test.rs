#[cfg(test)]
mod test {
    use crate::modules::cpu::{CPU, CpuVal};
    use super::*;

    #[test]
    fn test_lda_immediate(){
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        
        let cpu_register_a_cpu_val = cpu.getCPU(crate::modules::cpu::CpuATTR::RegisterA);
        let cpu_status_cpu_val     = cpu.getCPU(crate::modules::cpu::CpuATTR::CpuStatus);
        
        if let CpuVal::U8(register_a_val) = cpu_register_a_cpu_val {
            assert_eq!(register_a_val, 0x05);
        } 

        if let CpuVal::U8(cpu_status_val) = cpu_status_cpu_val {
            assert!(cpu_status_val & 0b0000_0010 == 0b00);
            assert!(cpu_status_val & 0b1000_0000 == 0);
        }
    }

    #[test]
    fn test_lda_zero_flag(){
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        
        let cpu_status_cpu_val = cpu.getCPU(crate::modules::cpu::CpuATTR::CpuStatus);

        if let CpuVal::U8(cpu_status_val) = cpu_status_cpu_val {
            assert!(cpu_status_val & 0b0000_0010 == 0b10);
        }
    }
}