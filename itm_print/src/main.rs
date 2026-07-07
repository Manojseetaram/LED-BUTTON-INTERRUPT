#![no_std]
#![no_main]
#![allow(dead_code)]
use core::ptr::addr_of_mut;

use cortex_m::peripheral::{self, Peripherals, syst};
use cortex_m_rt::{entry, exception};
use panic_halt as _;



mod led;
mod gpio;
mod reg;
mod board;
mod mcu;

static mut PERIPHERALS : Option<Peripherals> = None;

#[entry]

  fn main()-> !{
    led::led_init(board::LED_PORT, board::LED_PIN);
    led::led_on(board::LED_PORT, board::LED_PIN);
       let mut  peripherals = Peripherals::take().unwrap();
    
        itm_init(&mut peripherals);
       let stim0 = &mut peripherals.ITM.stim[0];
       cortex_m::iprintln!( stim0 , "{}" , "This is message 1");
          
        systick_init(&mut  peripherals);
          
unsafe  {
  PERIPHERALS = Some(peripherals);
}



    
loop { 
    
}

  }

  fn systick_init(peripherals : &mut cortex_m::peripheral::Peripherals){
    let systick = &mut peripherals.SYST;
           systick.set_clock_source(syst::SystClkSource::Core);
       systick.set_reload(4_000_000);
       systick.clear_current();
       systick.enable_interrupt();
       systick.enable_counter();
  }

  fn itm_init(peripherals : &mut peripheral::Peripherals){
   unsafe {
    let prescaler = 7 ;

     peripherals.TPIU.acpr.write(prescaler);
   peripherals.DCB.enable_trace();

     peripherals.ITM.ter[0].write(1);
     peripherals.TPIU.sppr.write(0b01);
 
  }

  }

  #[exception]
  fn SysTick (){
    unsafe {
    if let Some(peripherals) = (*addr_of_mut!(PERIPHERALS)).as_mut(){
        let stim0 = &mut peripherals.ITM.stim[0];
        cortex_m::iprintln!(stim0 , "{}" , "Hello from systick");
    }

    }

led::led_toggle(board::LED_PORT, board::LED_PIN);
  }