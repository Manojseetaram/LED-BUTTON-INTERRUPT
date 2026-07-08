#![no_std]
#![no_main]
#![allow(dead_code)]
use core::panic::PanicInfo;
use rtt_target::{rtt_init_print, rprintln};

mod board;
mod button;
mod proc;
mod exti;
mod gpio;
mod led;
mod mcu;
mod reg;
mod spi;
mod mfrc522;
mod i2c;
mod lcd;
mod critical_section_impl;
mod startup_stm32f401;

fn delay(cycles: u32) {
    for _ in 0..cycles {
        unsafe { core::arch::asm!("nop"); }
    }
}

#[unsafe(no_mangle)]
fn main() {
    rtt_init_print!();
    rprintln!("I2C scanner starting...");

    i2c::i2c1_init();

    rprintln!("Scanning I2C bus...");
    let mut found_any = false;
    for addr in 1u8..127 {
        if i2c::i2c1_write(addr, &[]) {
            rprintln!("Found device at address: 0x{:02X}", addr);
            found_any = true;
        }
    }
    if !found_any {
        rprintln!("No I2C devices found!");
    }

    loop {}
}
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}