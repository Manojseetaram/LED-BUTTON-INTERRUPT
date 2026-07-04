use crate::{
    exti::{self, gpio::set_edge}, gpio, mcu, proc,
};

pub enum ButtonStatus {
    Pressed,
    Released,
}
pub enum Trigger {
    FallingEdge,
    RaisingEdge,
}
pub enum Mode {
    Input,
    Interrupt(Trigger),
}

pub fn button_init(port: u32, pin: u32, mode: Mode) {
    gpio::enable_gpio_clock(port);
    gpio::set_gpio_mode_input(port, pin);
    exti::gpio::configure_syscfg(port, pin);
    match mode {
        Mode::Interrupt(trigger) => {
            match trigger {
                Trigger::FallingEdge => {
                    //Congigure the pin for falling edge detection
                    set_edge(pin, exti::gpio::EdgeTrigger::Falling);
                }
                Trigger::RaisingEdge => {
                    //Configure the pin for raising edge detection
                    set_edge(pin, exti::gpio::EdgeTrigger::Rising);
                }
            }
            //enable the interrupt in exti
            // exti::enable_interrupt(exti::ExtiLine);
            if let Some(exti_line) = exti::ExtiLine::from_pin(pin) {
                exti::enable_interrupt(exti_line);
                if let Some(irq_num ) = mcu::IRQn::from_pin(pin){
                    proc::enable_irq(irq_num);
                }
            }
          
        }
        Mode::Input => {}
    }
}


pub fn button_read_status(port: u32, pin: u32) -> ButtonStatus {
    if gpio::get_gpio_pin_state(port, pin) {
        ButtonStatus::Pressed
    } else {
        ButtonStatus::Released
    }
}


pub fn button_clear_interrupt(pin: u32) {
    if let Some(exti_line) = exti::ExtiLine::from_pin(pin) {
        exti::clear_pending_interrupt(exti_line);
    }
}