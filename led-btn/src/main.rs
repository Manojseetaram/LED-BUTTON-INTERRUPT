#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![allow(dead_code)]
use board::*;
use button::*;
use core::panic::PanicInfo;
use led::*;

use crate::button::Trigger::FallingEdge;
mod board;
mod button;
mod exti;
mod gpio;
mod led;
mod mcu;
mod reg;
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
    led_init(LED_PORT, LED_PIN);
    button_init(USER_BTN_PORT, USER_BTN_PIN, Mode::Interrupt(FallingEdge));
    loop {
        led_on(LED_PORT, LED_PIN);
        delay(3_000_000);
        led_off(LED_PORT, LED_PIN);
        delay(3_000_000);
    }
    //Todo add button code
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

fn EXTI0_Handler() {
    led_toggle(LED_PORT, LED_PIN)
}
