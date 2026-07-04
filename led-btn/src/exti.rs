

pub mod gpio {
    use crate::reg::reg_set_bit;

use crate::mcu::*;
    pub enum EdgeTrigger {
        Rising,
        Falling,
    }

    pub fn set_edge(pin: u32, edge: EdgeTrigger) {

        let exti_rtsr_addr = (EXTI_BASE + 0x08) as *mut u32;
        let exti_ftsr_addr = (EXTI_BASE + 0x0C) as *mut u32;

        match edge {
            EdgeTrigger::Falling => {
                reg_set_bit(exti_ftsr_addr, pin, true);
            }
            EdgeTrigger::Rising => {
                reg_set_bit(exti_rtsr_addr, pin, true);
            }
        }
    }
}
