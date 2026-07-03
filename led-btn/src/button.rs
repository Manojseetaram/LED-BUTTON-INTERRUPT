pub enum ButtonStatus {
    Pressed ,
    Released
}


pub fn button_init(port : u32 , pin : u32 ){

} 
pub fn button_configure_interrupt(pin : i32){

}
pub fn button_read_status(pin : i32 )-> ButtonStatus{
   ButtonStatus::Released
}