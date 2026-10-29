#[cfg(test)]
pub mod test {
    use crate::modules::cpu::{CPU};
    use crate::modules::cpu_data;
    use super::*;

    #[test]
    pub fn test_lda_immediate(){
        let mut cpu = CPU::new();
        cpu.run_test(vec![0xa9, 0xc0, 0x00]);
        
        let cpu_register_a_cpu_val = cpu.get_cpu(cpu_data::CpuATTR::RegisterA);
        let cpu_status_cpu_val     = cpu.get_cpu(cpu_data::CpuATTR::CpuStatus);
        
        if let cpu_data::CpuVal::U8(register_a_val) = cpu_register_a_cpu_val {
            assert_eq!(register_a_val, 0xc0);
        } 

        if let cpu_data::CpuVal::U8(cpu_status_val) = cpu_status_cpu_val {
            assert!(cpu_status_val & 0b0000_0010 == 0b00);
            assert_eq!(cpu_status_val & 0b1000_0000, 0b1000_0000);
        }
    }

    #[test]
    pub fn test_lda_zero_flag(){
        let mut cpu = CPU::new();
        cpu.run_test(vec![0xa9, 0x00, 0x00]);
        
        let cpu_status_cpu_val = cpu.get_cpu(cpu_data::CpuATTR::CpuStatus);

        if let cpu_data::CpuVal::U8(cpu_status_val) = cpu_status_cpu_val {
            assert!(cpu_status_val & 0b0000_0010 == 0b10);
        }
    }

    #[test]
    #[ignore = "TEST UNDONE: test_5_ops_working_together."]
    fn test_5_ops_working_together() {
        todo!("test_5_ops_working_together");
    } 

    #[test]
    #[ignore = "TEST UNDONE: test_inx_overflow."]
    fn test_inx_overflow() {
        todo!("test_inx_overflow");
    }
    
    #[test]
    #[ignore = "TEST UNDONE: test_0xaa_tax_move_a_to_x."]
    fn test_0xaa_tax_move_a_to_x() {
        todo!("test_0xaa_tax_move_a_to_x");
    }
}