use crate::{
    exti::{self, gpio::set_edge},
    gpio,
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
        }
        Mode::Input => {}
    }
}
pub fn button_configure_interrupt(port: u32, pin: i32) {}
pub fn button_read_status(port: u32, pin: i32) -> ButtonStatus {
    ButtonStatus::Released
}
