#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![allow(dead_code)]
use core::panic::PanicInfo;
use led::*;
use board::*;
mod reg;
mod gpio;
mod startup_stm32f401;
mod led;
mod mcu;
mod board;
fn delay(cycles: u32) {
    for _ in 0..cycles {
        unsafe {
            core::arch::asm!("nop");
        }
    }
}

#[unsafe(no_mangle)]
fn main(){
    led_init(LED_PORT, LED_PIN);
   loop {
        led_on(LED_PORT, LED_PIN);
        delay(3_000_000);
        led_off(LED_PORT, LED_PIN);
        delay(3_000_000);
    }
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

fn EXTI0_Handler(){
    led_toggle(LED_PORT, LED_PIN)
}