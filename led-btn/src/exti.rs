use crate::{mcu::EXTI_BASE, reg::reg_set_bit};

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

pub enum ExtiLine {
    Line0 = 0,
    Line1,
    Line2,
    Line3,
}

impl ExtiLine {
    pub fn from_pin(pin: u32) -> Option<ExtiLine> {
        match pin {
            0 => Some(ExtiLine::Line0),
            1 => Some(ExtiLine::Line1),
            2 => Some(ExtiLine::Line2),
            _ => None,
        }
    }
}
fn configure_interrupt(exti_line: ExtiLine, is_enable: bool) {
    let exti_imr_addr = (EXTI_BASE + 0x00) as *mut u32;
    let line = exti_line as u32;
    match line {
        0..=22 => {
            reg_set_bit(exti_imr_addr, line, is_enable);
        }
        _ => (),
    }
}

pub fn enable_interrupt(exti_line: ExtiLine) {
    configure_interrupt(exti_line, true);
}

pub fn disable_interrupt(exti_line: ExtiLine) {
    configure_interrupt(exti_line, false);
}
