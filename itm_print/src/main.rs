#![no_std]
#![no_main]
#![allow(dead_code)]
use cortex_m::peripheral::{Peripherals, syst};
use cortex_m_rt::{entry, exception};
use panic_halt as _;



mod led;
mod gpio;
mod reg;
mod board;
mod mcu;
#[entry]

  fn main()-> !{
    led::led_init(board::LED_PORT, board::LED_PIN);
    led::led_on(board::LED_PORT, board::LED_PIN);
       let mut  peripherals = Peripherals::take().unwrap();
       let systick = &mut  peripherals.SYST;
       systick.set_clock_source(syst::SystClkSource::Core);
       systick.set_reload(4_000_000);
       systick.clear_current();
       systick.enable_interrupt();
       systick.enable_counter();

    
loop {
    
}

  }

  #[exception]
  fn SysTick (){
led::led_toggle(board::LED_PORT, board::LED_PIN);
  }