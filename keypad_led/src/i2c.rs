use crate::mcu::*;
use crate::reg::*;
use crate::gpio;

pub fn i2c1_init() {
    // Enable I2C1 clock: RCC_APB1ENR offset 0x40, bit 21
    let rcc_apb1enr_addr = (RCC_BASE + 0x40) as *mut u32;
    reg_set_bit(rcc_apb1enr_addr, 21, true);

    gpio::enable_gpio_clock(GPIOB_BASE);
    gpio::set_gpio_mode_af(GPIOB_BASE, 6, 4); // SCL, AF4
    gpio::set_gpio_mode_af(GPIOB_BASE, 7, 4); // SDA, AF4
    gpio::set_gpio_output_type_open_drain(GPIOB_BASE, 6);
    gpio::set_gpio_output_type_open_drain(GPIOB_BASE, 7);

    let cr1_addr  = (I2C1_BASE + I2C_CR1_OFFSET) as *mut u32;
    let cr2_addr  = (I2C1_BASE + I2C_CR2_OFFSET) as *mut u32;
    let ccr_addr  = (I2C1_BASE + I2C_CCR_OFFSET) as *mut u32;
    let trise_addr= (I2C1_BASE + I2C_TRISE_OFFSET) as *mut u32;

    // Disable I2C1 before configuring
    reg_set_bit(cr1_addr, 0, false);

    // FREQ = APB1 clock in MHz (assuming default HSI, APB1 = 16MHz, no PLL configured)
    reg_set_bits(cr2_addr, 16, 0, 6);

    // Standard mode 100kHz: CCR = APB1_clk / (2 * 100000) = 16_000_000 / 200_000 = 80
    reg_set_val(ccr_addr, 80);

    // TRISE = (APB1_clk_MHz) + 1 = 17
    reg_set_val(trise_addr, 17);

    // Enable I2C1
    reg_set_bit(cr1_addr, 0, true);
}

fn sr1() -> u32 { unsafe { read_register((I2C1_BASE + I2C_SR1_OFFSET) as *mut u32) } }
fn sr2() -> u32 { unsafe { read_register((I2C1_BASE + I2C_SR2_OFFSET) as *mut u32) } }
fn dr_write(b: u8) { unsafe { write_register((I2C1_BASE + I2C_DR_OFFSET) as *mut u32, b as u32); } }

fn wait_flag(check: impl Fn(u32) -> bool, timeout: u32) -> bool {
    let mut count = 0;
    loop {
        if check(sr1()) { return true; }
        count += 1;
        if count > timeout { return false; }
    }
}

pub fn i2c1_write(addr: u8, data: &[u8]) -> bool {
    let cr1_addr = (I2C1_BASE + I2C_CR1_OFFSET) as *mut u32;

    reg_set_bit(cr1_addr, 8, true); // START
    if !wait_flag(|s| s & 0x1 != 0, 2000) {
        reg_set_bit(cr1_addr, 9, true); // STOP, clean up before giving up
        return false;
    }

    dr_write((addr << 1) | 0);
    if !wait_flag(|s| s & 0x2 != 0, 2000) {
        reg_set_bit(cr1_addr, 9, true); // STOP
        return false;
    }
    let _ = sr2();

    for &b in data {
        if !wait_flag(|s| s & 0x80 != 0, 2000) {
            reg_set_bit(cr1_addr, 9, true); // STOP
            return false;
        }
        dr_write(b);
    }
    if !wait_flag(|s| s & 0x4 != 0, 2000) {
        reg_set_bit(cr1_addr, 9, true); // STOP
        return false;
    }

    reg_set_bit(cr1_addr, 9, true); // STOP
    true
}