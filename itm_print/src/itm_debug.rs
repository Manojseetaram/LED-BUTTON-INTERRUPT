use cortex_m::peripheral;

 pub fn itm_init(peripherals : &mut peripheral::Peripherals){
   unsafe {
    let prescaler = 7 ;

     peripherals.TPIU.acpr.write(prescaler);
   peripherals.DCB.enable_trace();

     peripherals.ITM.ter[0].write(1);
    // peripherals.TPIU.sppr.write(0b10);  
 
  }

  }
  pub fn itm_print(peripherals : &mut peripheral::Peripherals , msg : &str){
     let stim0 = &mut peripherals.ITM.stim[0];
       cortex_m::iprintln!( stim0 , "{}" , msg);
  }