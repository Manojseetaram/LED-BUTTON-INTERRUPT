pub const GPIOA_BASE: u32 = 0x4002_0000;
pub const GPIOB_BASE: u32 = GPIOA_BASE + 0x400;
pub const GPIOC_BASE: u32 = GPIOA_BASE + 0x800;

pub const RCC_BASE: u32 = 0x4002_3800;
pub const EXTI_BASE: u32 = 0x4001_3C00;
pub const SYSCFG_BASE: u32 = 0x4001_3800;
pub const GPIO_PIN_0: u32 = 0;
pub const GPIO_PIN_1: u32 = 1;
pub const GPIO_PIN_2: u32 = 2;
pub const GPIO_PIN_3: u32 = 3;
pub const GPIO_PIN_13: u32 = 13;



#[allow(non_camel_case_types)]
pub enum IRQn {
    WWDG = 0,
    PVD = 1,              // EXTI16
    TAMP_STAMP = 2,        // EXTI21
    RTC_WKUP = 3,          // EXTI22
    FLASH = 4,
    RCC = 5,
    EXTI0 = 6,
    EXTI1 = 7,
    EXTI2 = 8,
    EXTI3 = 9,
    EXTI4 = 10,
    DMA1_Stream0 = 11,
    DMA1_Stream1 = 12,
    DMA1_Stream2 = 13,
    DMA1_Stream3 = 14,
    DMA1_Stream4 = 15,
    DMA1_Stream5 = 16,
    DMA1_Stream6 = 17,
    ADC = 18,
    // 19-22 reserved
    EXTI9_5 = 23,
    TIM1_BRK_TIM9 = 24,
    TIM1_UP_TIM10 = 25,
    TIM1_TRG_COM_TIM11 = 26,
    TIM1_CC = 27,
    TIM2 = 28,
    TIM3 = 29,
    TIM4 = 30,
    I2C1_EV = 31,
    I2C1_ER = 32,
    I2C2_EV = 33,
    I2C2_ER = 34,
    SPI1 = 35,
    SPI2 = 36,
    USART1 = 37,
    USART2 = 38,
    // 39 reserved
    EXTI15_10 = 40,
    RTC_Alarm = 41,        // EXTI17
    OTG_FS_WKUP = 42,      // EXTI18
    // 43-46 reserved
    DMA1_Stream7 = 47,
    // 48 reserved
    SDIO = 49,
    TIM5 = 50,
    SPI3 = 51,
    // 52-55 reserved
    DMA2_Stream0 = 56,
    DMA2_Stream1 = 57,
    DMA2_Stream2 = 58,
    DMA2_Stream3 = 59,
    DMA2_Stream4 = 60,
    // 61-66 reserved
    OTG_FS = 67,
    DMA2_Stream5 = 68,
    DMA2_Stream6 = 69,
    DMA2_Stream7 = 70,
    USART6 = 71,
    I2C3_EV = 72,
    I2C3_ER = 73,
    // 74-80 reserved
    FPU = 81,
    // 82-83 reserved
    SPI4 = 84,
}

impl IRQn {
    pub fn from_pin(pin: u32) -> Option<u32> {
        match pin {
            0 => Some(IRQn::EXTI0 as u32),
            1 => Some(IRQn::EXTI1 as u32),
            2 => Some(IRQn::EXTI2 as u32),
            3 => Some(IRQn::EXTI3 as u32),
            4 => Some(IRQn::EXTI4 as u32),
            5..=9 => Some(IRQn::EXTI9_5 as u32),
            10..=15 => Some(IRQn::EXTI15_10 as u32),
            _ => None, 
        }
    }
}