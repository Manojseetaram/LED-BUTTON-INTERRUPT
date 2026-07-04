ENTRY(reset_handler)
/* Define memory regions */
ENTRY(reset_handler)
/* Define memory regions */

MEMORY
{
    FLASH (rx)  : ORIGIN = 0x08000000, LENGTH = 256K
    RAM   (rwx) : ORIGIN = 0x20000000, LENGTH = 64K
}

_start_of_stack = ORIGIN(RAM) + LENGTH(RAM);
_min_stack_size = 0x400;
_min_heap_size = 0x400;

SECTIONS 
{
    .text  : 
    {
    . = ORIGIN(FLASH); 
    LONG(_start_of_stack);
    KEEP(*(.isr_vector));
    *(.text)
    *(.text.*)
    . = ALIGN(4);
    } > FLASH 

    .rodata :
    { 
    . = ALIGN(4);
   *(.rodata)
   *(.rodata*)
    . = ALIGN(4);
    } > FLASH
    .data :
    {
        _sidata = LOADADDR(.data);
            . = ALIGN(4);
        _sdata = .;
      *(.data)
      *(.data*)
    . = ALIGN(4);
      _edata = .;
    } > RAM AT> FLASH
    .bss :
    {
            . = ALIGN(4);
        _sbss = .;
        *(.bss)
        *(.bss*)
            . = ALIGN(4);
        _ebss = .;
    } > RAM


    .ram_usage_check : 
    {
        . = . + _min_stack_size ;
        . = . + _min_heap_size ;
    } > RAM
}




PROVIDE(mem_manage_Handler = default_Handler);
PROVIDE(busfault_Handler = default_Handler);
PROVIDE(usagefault_Handler = default_Handler);
PROVIDE(SVCall_Handler = default_Handler);
PROVIDE(PendSV_Handler = default_Handler);
PROVIDE(SysTick_Handler = default_Handler);
PROVIDE(WWDG_Handler = default_Handler);
PROVIDE(PVD_Handler = default_Handler);
PROVIDE(TAMP_STAMP_Handler = default_Handler);
PROVIDE(RTC_WKUP_Handler = default_Handler);
PROVIDE(FLASH_Handler = default_Handler);
PROVIDE(RCC_Handler = default_Handler);
PROVIDE(EXTI0_Handler = default_Handler);
PROVIDE(EXTI1_Handler = default_Handler);
PROVIDE(EXTI2_Handler = default_Handler);
PROVIDE(EXTI3_Handler = default_Handler);
PROVIDE(EXTI4_Handler = default_Handler);
PROVIDE(DMA1_Stream0_Handler = default_Handler);
PROVIDE(DMA1_Stream1_Handler = default_Handler);
PROVIDE(DMA1_Stream2_Handler = default_Handler);
PROVIDE(DMA1_Stream3_Handler = default_Handler);
PROVIDE(DMA1_Stream4_Handler = default_Handler);
PROVIDE(DMA1_Stream5_Handler = default_Handler);
PROVIDE(DMA1_Stream6_Handler = default_Handler);
PROVIDE(ADC_Handler = default_Handler);
PROVIDE(EXTI9_5_Handler = default_Handler);
PROVIDE(TIM1_BRK_TIM9_Handler = default_Handler);
PROVIDE(TIM1_UP_TIM10_Handler = default_Handler);
PROVIDE(TIM1_TRG_COM_TIM11_Handler = default_Handler);
PROVIDE(TIM1_CC_Handler = default_Handler);
PROVIDE(TIM2_Handler = default_Handler);
PROVIDE(TIM3_Handler = default_Handler);
PROVIDE(TIM4_Handler = default_Handler);
PROVIDE(I2C1_EV_Handler = default_Handler);
PROVIDE(I2C1_ER_Handler = default_Handler);
PROVIDE(I2C2_EV_Handler = default_Handler);
PROVIDE(I2C2_ER_Handler = default_Handler);
PROVIDE(SPI1_Handler = default_Handler);
PROVIDE(SPI2_Handler = default_Handler);
PROVIDE(USART1_Handler = default_Handler);
PROVIDE(USART2_Handler = default_Handler);
PROVIDE(EXTI15_10_Handler = default_Handler);
PROVIDE(RTC_Alarm_Handler = default_Handler);
PROVIDE(OTG_FS_WKUP_Handler = default_Handler);
PROVIDE(DMA1_Stream7_Handler = default_Handler);
PROVIDE(SDIO_Handler = default_Handler);
PROVIDE(TIM5_Handler = default_Handler);
PROVIDE(SPI3_Handler = default_Handler);
PROVIDE(DMA2_Stream0_Handler = default_Handler);
PROVIDE(DMA2_Stream1_Handler = default_Handler);
PROVIDE(DMA2_Stream2_Handler = default_Handler);
PROVIDE(DMA2_Stream3_Handler = default_Handler);
PROVIDE(DMA2_Stream4_Handler = default_Handler);
PROVIDE(OTG_FS_Handler = default_Handler);
PROVIDE(DMA2_Stream5_Handler = default_Handler);
PROVIDE(DMA2_Stream6_Handler = default_Handler);
PROVIDE(DMA2_Stream7_Handler = default_Handler);
PROVIDE(USART6_Handler = default_Handler);
PROVIDE(I2C3_EV_Handler = default_Handler);
PROVIDE(I2C3_ER_Handler = default_Handler);
PROVIDE(FPU_Handler = default_Handler);
PROVIDE(SPI4_Handler = default_Handler);