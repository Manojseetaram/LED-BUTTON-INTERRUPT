#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
use core::panic:: PanicInfo;

mod startup_stm32f401;


#[unsafe(no_mangle)] 

fn main(){


   loop {

    }
}
#[panic_handler]
fn panic_handler(_info : &PanicInfo)-> ! {
    loop {
        
    }
}