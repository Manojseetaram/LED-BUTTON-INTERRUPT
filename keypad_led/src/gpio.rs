use crate::{
    mcu::{GPIOA_BASE, GPIOB_BASE, GPIOC_BASE, RCC_BASE},
    reg::*,
};
pub fn set_gpio_mode_af(port: u32, pin: u32, af: u32) {
    let moder_addr = (port + 0x00) as *mut u32;
    reg_set_bits(moder_addr, 0b10, pin * 2, 2);

    let (afr_addr, afr_bit) = if pin < 8 {
        ((port + 0x20) as *mut u32, pin * 4)
    } else {
        ((port + 0x24) as *mut u32, (pin - 8) * 4)
    };
    reg_set_bits(afr_addr, af, afr_bit, 4);
}
pub fn enable_gpio_clock(port: u32) {
    let rcc_ahb1enr_addr = (RCC_BASE + 0x30) as *mut u32;

    match port {
        GPIOA_BASE => {
            reg_set_bit(rcc_ahb1enr_addr, 0, true);
        }
        GPIOB_BASE => {
            reg_set_bit(rcc_ahb1enr_addr, 1, true);
        }
        GPIOC_BASE => {
            reg_set_bit(rcc_ahb1enr_addr, 2, true);
        }
        _ => {}
    }
}
pub fn set_gpio_output_type_open_drain(port: u32, pin: u32) {
    let gpio_op_type_reg_addr = (port + 0x04) as *mut u32;
    reg_set_bits(gpio_op_type_reg_addr, 1, pin, 1); // 1 = open-drain
}
pub fn set_gpio_mode_output(port: u32, pin: u32) {
    let gpio_mode_reg_addr = (port + 0x00) as *mut u32;
    let bit_position = pin * 2;
    let mode_value = 0x1;

    reg_set_bits(gpio_mode_reg_addr, mode_value, bit_position, 2);
}

pub fn set_gpio_mode_input(port: u32, pin: u32) {
    let gpio_mode_reg_addr = (port + 0x00) as *mut u32;
    let bit_position = pin * 2;
    let mode_value = 0;

    reg_set_bits(gpio_mode_reg_addr, mode_value, bit_position, 2);
}

pub fn set_gpio_output_type_push_pull(port: u32, pin: u32) {
    let gpio_op_type_reg_addr = (port + 0x04) as *mut u32;
    let bit_position = pin;
    let bit_value = 0;

    reg_set_bits(gpio_op_type_reg_addr, bit_value, bit_position, 1);
}
pub enum PinState {
    Hight,
    Low,
    Toggle,
}
pub fn set_gpio_pin_state(port: u32, pin: u32, pin_state: PinState) {
    let gpio_bsrr_addr = (port + 0x18) as *mut u32;
    match pin_state {
        PinState::Hight => {
            reg_set_val(gpio_bsrr_addr, 1 << pin);
        }
        PinState::Low => {
            reg_set_val(gpio_bsrr_addr, 1 << (pin + 16));
        }

        PinState::Toggle => {
            let gpio_odr_addr = (port + 0x14) as *mut u32;
            if reg_read_bit(gpio_odr_addr, pin) {
                reg_set_val(gpio_bsrr_addr, 1 << (pin + 16));
            } else {
                reg_set_val(gpio_bsrr_addr, 1 << pin);
            }
        }
    }
}
pub fn get_gpio_pin_state(port: u32, pin: u32) -> bool {
    let gpio_idr_addr = (port + 0x10) as *mut u32;
    reg_read_bit(gpio_idr_addr, pin)
}
