#![no_std]
#![no_main]
  
use cortex_m_rt::{entry, exception};
use panic_halt as _;

#[entry]

  fn main()-> !{
       
loop {
    
}

  }

  #[exception]
  fn SysTick (){
    
  }