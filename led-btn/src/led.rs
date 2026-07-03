use core::mem::offset_of;


use core::ptr;
unsafe fn read_register(addr : *mut u32 )-> u32 {
       ptr::read_volatile(addr)
}
unsafe fn write_register ( addr : *mut u32 , value : u32 ){
    ptr::write_volatile(addr, value);
}
pub fn led_init(port : u32 , pin : u32 ){
   let offset = 0 ;
   let gpio_mode_reg_addr = (port + offset) as *mut u32 ;
   let gpio_mode_reg_value = unsafe {
       ptr::read_volatile(gpio_mode_reg_addr)
   };
}
pub fn led_on(port : u32 , pin : u32){

}
pub fn led_off(port : u32 ,  pin : u32){

}
pub fn led_toggle(port : u32  , pin : u32 ){

}