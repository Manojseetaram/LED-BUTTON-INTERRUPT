use crate::gpio;
use crate::mcu::*;
use crate::reg::*;

pub fn spi1_init() {
    // Enable SPI1 clock: RCC_APB2ENR offset 0x44, bit 12
    let rcc_apb2enr_addr = (RCC_BASE + 0x44) as *mut u32;
    reg_set_bit(rcc_apb2enr_addr, 12, true);

    gpio::enable_gpio_clock(GPIOA_BASE);
    gpio::set_gpio_mode_af(GPIOA_BASE, 5, 5); // SCK
    gpio::set_gpio_mode_af(GPIOA_BASE, 6, 5); // MISO
    gpio::set_gpio_mode_af(GPIOA_BASE, 7, 5); // MOSI

    // CS (PA4) as plain GPIO output, idle high
    gpio::set_gpio_mode_output(GPIOA_BASE, 4);
    gpio::set_gpio_output_type_push_pull(GPIOA_BASE, 4);
    cs_high();

    let cr1_addr = (SPI1_BASE + SPI_CR1_OFFSET) as *mut u32;
    let mut cr1_val: u32 = 0;
    cr1_val |= 1 << 2; // MSTR
    cr1_val |= 0b011 << 3; // BR: /16 prescaler
    cr1_val |= 1 << 9; // SSM
    cr1_val |= 1 << 8; // SSI
    reg_set_val(cr1_addr, cr1_val);
    reg_set_bit(cr1_addr, 6, true); // SPE
}

fn cs_low() {
    gpio::set_gpio_pin_state(GPIOA_BASE, GPIO_PIN_4, gpio::PinState::Low);
}
fn cs_high() {
    gpio::set_gpio_pin_state(GPIOA_BASE, GPIO_PIN_4, gpio::PinState::Hight);
}

pub fn spi1_transfer(byte: u8) -> u8 {
    let sr_addr = (SPI1_BASE + SPI_SR_OFFSET) as *mut u32;
    let dr_addr = (SPI1_BASE + SPI_DR_OFFSET) as *mut u32;
    while !reg_read_bit(sr_addr, 1) {} // TXE
    unsafe {
        write_register(dr_addr, byte as u32);
    }
    while !reg_read_bit(sr_addr, 0) {} // RXNE
    unsafe { read_register(dr_addr) as u8 }
}

pub fn cs_select() {
    cs_low();
}
pub fn cs_deselect() {
    cs_high();
}
