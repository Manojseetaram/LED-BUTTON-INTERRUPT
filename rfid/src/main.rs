#![no_std]
#![no_main]
#![allow(dead_code)]
use core::panic::PanicInfo;
use rtt_target::{rprintln, rtt_init_print};

mod board;
mod button;
mod critical_section_impl;
mod exti;
mod gpio;
mod led;
mod mcu;
mod mfrc522;
mod proc;
mod reg;
mod spi;
mod startup_stm32f401;

fn delay(cycles: u32) {
    for _ in 0..cycles {
        unsafe {
            core::arch::asm!("nop");
        }
    }
}

#[unsafe(no_mangle)]
fn main() {
    rtt_init_print!();
    rprintln!("RFID reader starting...");

    led::led_init(board::LED_PORT, board::LED_PIN);
    led::led_off(board::LED_PORT, board::LED_PIN);
    mfrc522::mfrc522_init();

    let version = mfrc522::read_version();
    rprintln!("MFRC522 version register: 0x{:02X}", version);

   loop {
    if let Some(uid) = mfrc522::read_card_uid() {
        rtt_target::rprintln!("Card UID: {:02X} {:02X} {:02X} {:02X}", uid[0], uid[1], uid[2], uid[3]);
        for _ in 0..4 {
            led::led_on(board::LED_PORT, board::LED_PIN);
            delay(200_000);
            led::led_off(board::LED_PORT, board::LED_PIN);
            delay(200_000);
        }
        delay(500_00); // short pause so one tap doesn't print 50 times in a row
    }
    delay(100_00); // much shorter poll interval
}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
