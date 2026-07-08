use core::ptr;
unsafe extern "C" {
    fn mem_manage_Handler();
    fn busfault_Handler();
    fn usagefault_Handler();
    fn SVCall_Handler();
    fn PendSV_Handler();
    fn SysTick_Handler();

    fn WWDG_Handler();
    fn PVD_Handler();
    fn TAMP_STAMP_Handler();
    fn RTC_WKUP_Handler();
    fn FLASH_Handler();
    fn RCC_Handler();

    fn EXTI0_Handler();
    fn EXTI1_Handler();
    fn EXTI2_Handler();
    fn EXTI3_Handler();
    fn EXTI4_Handler();

    fn DMA1_Stream0_Handler();
    fn DMA1_Stream1_Handler();
    fn DMA1_Stream2_Handler();
    fn DMA1_Stream3_Handler();
    fn DMA1_Stream4_Handler();
    fn DMA1_Stream5_Handler();
    fn DMA1_Stream6_Handler();

    fn ADC_Handler();

    fn EXTI9_5_Handler();

    fn TIM1_BRK_TIM9_Handler();
    fn TIM1_UP_TIM10_Handler();
    fn TIM1_TRG_COM_TIM11_Handler();
    fn TIM1_CC_Handler();

    fn TIM2_Handler();
    fn TIM3_Handler();
    fn TIM4_Handler();

    fn I2C1_EV_Handler();
    fn I2C1_ER_Handler();
    fn I2C2_EV_Handler();
    fn I2C2_ER_Handler();

    fn SPI1_Handler();
    fn SPI2_Handler();

    fn USART1_Handler();
    fn USART2_Handler();

    fn EXTI15_10_Handler();
    fn RTC_Alarm_Handler();
    fn OTG_FS_WKUP_Handler();

    fn DMA1_Stream7_Handler();

    fn SDIO_Handler();
    fn TIM5_Handler();
    fn SPI3_Handler();

    fn DMA2_Stream0_Handler();
    fn DMA2_Stream1_Handler();
    fn DMA2_Stream2_Handler();
    fn DMA2_Stream3_Handler();
    fn DMA2_Stream4_Handler();

    fn OTG_FS_Handler();
    fn DMA2_Stream5_Handler();
    fn DMA2_Stream6_Handler();
    fn DMA2_Stream7_Handler();

    fn USART6_Handler();

    fn I2C3_EV_Handler();
    fn I2C3_ER_Handler();

    fn FPU_Handler();

    fn SPI4_Handler();
}

#[unsafe(link_section = ".isr_vector")]
#[used]
static VECTOR_TABLE: [Option<unsafe extern "C" fn()>; 97] = [
    Some(reset_Handler),
    Some(nmi_Handler),
    Some(hardfault_Handler),
    Some(mem_manage_Handler),
    Some(busfault_Handler),
    Some(usagefault_Handler),
    None,
    None,
    None,
    None,
    Some(SVCall_Handler),
    Some(PendSV_Handler),
    Some(SysTick_Handler),
    // ---- IRQ0 onward (peripheral interrupts) ----
    Some(WWDG_Handler),               // IRQ0
    Some(PVD_Handler),                // IRQ1
    Some(TAMP_STAMP_Handler),         // IRQ2
    Some(RTC_WKUP_Handler),           // IRQ3
    Some(FLASH_Handler),              // IRQ4
    Some(RCC_Handler),                // IRQ5
    Some(EXTI0_Handler),              // IRQ6
    Some(EXTI1_Handler),              // IRQ7
    Some(EXTI2_Handler),              // IRQ8
    Some(EXTI3_Handler),              // IRQ9
    Some(EXTI4_Handler),              // IRQ10
    Some(DMA1_Stream0_Handler),       // IRQ11
    Some(DMA1_Stream1_Handler),       // IRQ12
    Some(DMA1_Stream2_Handler),       // IRQ13
    Some(DMA1_Stream3_Handler),       // IRQ14
    Some(DMA1_Stream4_Handler),       // IRQ15
    Some(DMA1_Stream5_Handler),       // IRQ16
    Some(DMA1_Stream6_Handler),       // IRQ17
    Some(ADC_Handler),                // IRQ18
    None,                             // IRQ19 reserved
    None,                             // IRQ20 reserved
    None,                             // IRQ21 reserved
    Some(EXTI9_5_Handler),            // IRQ22
    Some(TIM1_BRK_TIM9_Handler),      // IRQ23
    Some(TIM1_UP_TIM10_Handler),      // IRQ24
    Some(TIM1_TRG_COM_TIM11_Handler), // IRQ25
    Some(TIM1_CC_Handler),            // IRQ26
    Some(TIM2_Handler),               // IRQ27
    Some(TIM3_Handler),               // IRQ28
    Some(TIM4_Handler),               // IRQ29
    Some(I2C1_EV_Handler),            // IRQ30
    Some(I2C1_ER_Handler),            // IRQ31
    Some(I2C2_EV_Handler),            // IRQ32
    Some(I2C2_ER_Handler),            // IRQ33
    Some(SPI1_Handler),               // IRQ34
    Some(SPI2_Handler),               // IRQ35
    Some(USART1_Handler),             // IRQ36
    Some(USART2_Handler),             // IRQ37
    None,                             // IRQ38 reserved
    Some(EXTI15_10_Handler),          // IRQ39
    Some(RTC_Alarm_Handler),          // IRQ40
    Some(OTG_FS_WKUP_Handler),        // IRQ41
    None,                             // IRQ42
    None,                             // IRQ43
    None,                             // IRQ44
    Some(DMA1_Stream7_Handler),       // IRQ45
    None,                             // IRQ46
    Some(SDIO_Handler),               // IRQ47
    Some(TIM5_Handler),               // IRQ48
    Some(SPI3_Handler),               // IRQ49
    None,                             // IRQ50
    None,                             // IRQ51
    None,                             // IRQ52
    None,                             // IRQ53
    Some(DMA2_Stream0_Handler),       // IRQ54
    Some(DMA2_Stream1_Handler),       // IRQ55
    Some(DMA2_Stream2_Handler),       // IRQ56
    Some(DMA2_Stream3_Handler),       // IRQ57
    Some(DMA2_Stream4_Handler),       // IRQ58
    None,                             // IRQ59
    None,                             // IRQ60
    None,                             // IRQ61
    None,                             // IRQ62
    None,                             // IRQ63
    Some(OTG_FS_Handler),             // IRQ64
    Some(DMA2_Stream5_Handler),       // IRQ65
    Some(DMA2_Stream6_Handler),       // IRQ66
    Some(DMA2_Stream7_Handler),       // IRQ67
    Some(USART6_Handler),             // IRQ68
    Some(I2C3_EV_Handler),            // IRQ69
    Some(I2C3_ER_Handler),            // IRQ70
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,               // IRQ71-79 reserved
    Some(FPU_Handler),  // IRQ80
    None,               // IRQ81
    None,               // IRQ82
    Some(SPI4_Handler), // IRQ83
];

#[unsafe(no_mangle)]
extern "C" fn default_Handler() {
    loop {}
}
#[unsafe(no_mangle)]
extern "C" fn hardfault_Handler() {
    loop {}
}
#[unsafe(no_mangle)]
extern "C" fn nmi_Handler() {
    loop {}
}

unsafe extern "C" {
    static _sidata: u32;
    static mut _sdata: u32;
    static mut _edata: u32;
    static mut _sbss: u32;
    static mut _ebss: u32;
}

#[unsafe(no_mangle)]
extern "C" fn reset_Handler() {
    unsafe {
        let mut src_is_flash = ptr::addr_of!(_sidata);
        let mut dest_is_ram = ptr::addr_of_mut!(_sdata);
        let data_end_in_ram = ptr::addr_of_mut!(_edata);

        while dest_is_ram < data_end_in_ram {
            *dest_is_ram = *src_is_flash;
            dest_is_ram = dest_is_ram.add(1);
            src_is_flash = src_is_flash.add(1);
        }

        let mut bss = ptr::addr_of_mut!(_sbss);
        let bss_end = ptr::addr_of_mut!(_ebss);

        while bss < bss_end {
            *bss = 0;
            bss = bss.add(1);
        }
    }
    crate::main()
}
