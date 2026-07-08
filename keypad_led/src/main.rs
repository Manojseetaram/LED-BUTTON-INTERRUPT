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
    rprintln!("RFID reader starting...");

    led::led_init(board::LED_PORT, board::LED_PIN);
    led::led_off(board::LED_PORT, board::LED_PIN);
    mfrc522::mfrc522_init();

    let mut card_present = false;

    loop {
        match mfrc522::read_card_uid() {
            Some(uid) => {
                if !card_present {
                    rprintln!("UID: {:02X} {:02X} {:02X} {:02X}", uid[0], uid[1], uid[2], uid[3]);
                    led::led_on(board::LED_PORT, board::LED_PIN);
                    delay(200_000);
                    led::led_off(board::LED_PORT, board::LED_PIN);
                    card_present = true;
                }
            }
            None => {
                card_present = false;
            }
        }
        delay(50_000); // fast polling for good tap responsiveness
    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}