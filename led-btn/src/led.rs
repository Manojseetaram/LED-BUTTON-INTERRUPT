

use crate::gpio::{PinState, enable_gpio_clock, set_gpio_mode_output, set_gpio_output_type_push_pull, set_gpio_pin_state};

fn clear_bits(value : u32 , mask : u32 )-> u32 {
    value & !mask
}
fn set_bits(value : u32 , mask : u32)-> u32 {
    value | mask 
}

pub fn led_init(port: u32, pin: u32){
   
    enable_gpio_clock(port);

    set_gpio_mode_output(port, pin);

    set_gpio_output_type_push_pull(port, pin);
    
}
pub fn led_on(port: u32, pin: u32){
      set_gpio_pin_state(port, pin, PinState::Low);  
  }
  pub fn led_off(port: u32, pin: u32){
      set_gpio_pin_state(port, pin, PinState::Hight);  
  }
pub fn led_toggle(port : u32  , pin : u32 ){
     set_gpio_pin_state(port, pin, PinState::Toggle);
}