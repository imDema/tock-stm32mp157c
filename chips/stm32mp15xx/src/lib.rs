//! Peripheral implementations for the STM32F3xx MCU.
//!
//! STM32MP15xx: <https://www.st.com/en/microcontrollers-microprocessors/STM32MP15xx.html>

#![crate_name = "stm32mp15xx"]
#![crate_type = "rlib"]
#![feature(const_fn_trait_bound)]
#![no_std]

pub mod chip;
// mod deferred_call_tasks;
// pub mod nvic;

// Peripherals
// pub mod adc;
// pub mod dma;
// pub mod exti;
// pub mod flash;
// pub mod gpio;
// pub mod i2c;
// pub mod rcc;
// pub mod spi;
// pub mod syscfg;
// pub mod tim2;
// pub mod usart;
// pub mod wdt;

use cortexm4::{
    generic_isr, hard_fault_handler, initialize_ram_jump_to_main, svc_handler, systick_handler,
    unhandled_interrupt,
};

extern "C" {
    // _estack is not really a function, but it makes the types work
    // You should never actually invoke it!!
    fn _estack();
}

#[cfg_attr(
    all(target_arch = "arm", target_os = "none"),
    link_section = ".vectors"
)]
// used Ensures that the symbol is kept until the final binary
#[cfg_attr(all(target_arch = "arm", target_os = "none"), used)]
pub static BASE_VECTORS: [unsafe extern "C" fn(); 16] = [
    _estack,
    initialize_ram_jump_to_main,
    unhandled_interrupt, // NMI
    hard_fault_handler,  // Hard Fault
    unhandled_interrupt, // MemManage
    unhandled_interrupt, // BusFault
    unhandled_interrupt, // UsageFault
    unhandled_interrupt,
    unhandled_interrupt,
    unhandled_interrupt,
    unhandled_interrupt,
    svc_handler,         // SVC
    unhandled_interrupt, // DebugMon
    unhandled_interrupt,
    unhandled_interrupt, // PendSV
    systick_handler,     // SysTick
];


// STM32F412g has total of 97 interrupts
#[cfg_attr(all(target_arch = "arm", target_os = "none"), link_section = ".irqs")]
// `used` ensures that the symbol is kept until the final binary. However, as of
// May 2020, due to the compilation process, there must be some other compiled
// code here to make sure the object file is kept around. That means at minimum
// there must be an `init()` function here so that compiler does not just ignore
// the `IRQS` object. See https://github.com/rust-lang/rust/issues/56639 for a
// related discussion.
#[cfg_attr(all(target_arch = "arm", target_os = "none"), used)]
pub static IRQS: [unsafe extern "C" fn(); 150] = [
    generic_isr,        // WWDG1_IRQn
    generic_isr,        // PVD_AVD_IRQn
    generic_isr,        // TAMP_IRQn
    generic_isr,        // RTC_WKUP_ALARM_IRQn
    generic_isr,        // RESERVED_4
    generic_isr,        // RCC_IRQn
    generic_isr,        // EXTI0_IRQn
    generic_isr,        // EXTI1_IRQn
    generic_isr,        // EXTI2_IRQn
    generic_isr,        // EXTI3_IRQn
    generic_isr,        // EXTI4_IRQn
    generic_isr,        // DMA1_Stream0_IRQn
    generic_isr,        // DMA1_Stream1_IRQn
    generic_isr,        // DMA1_Stream2_IRQn
    generic_isr,        // DMA1_Stream3_IRQn
    generic_isr,        // DMA1_Stream4_IRQn
    generic_isr,        // DMA1_Stream5_IRQn
    generic_isr,        // DMA1_Stream6_IRQn
    generic_isr,        // ADC1_IRQn
    generic_isr,        // FDCAN1_IT0_IRQn
    generic_isr,        // FDCAN2_IT0_IRQn
    generic_isr,        // FDCAN1_IT1_IRQn
    generic_isr,        // FDCAN2_IT1_IRQn
    generic_isr,        // EXTI5_IRQn
    generic_isr,        // TIM1_BRK_IRQn
    generic_isr,        // TIM1_UP_IRQn
    generic_isr,        // TIM1_TRG_COM_IRQn
    generic_isr,        // TIM1_CC_IRQn
    generic_isr,        // TIM2_IRQn
    generic_isr,        // TIM3_IRQn
    generic_isr,        // TIM4_IRQn
    generic_isr,        // I2C1_EV_IRQn
    generic_isr,        // I2C1_ER_IRQn
    generic_isr,        // I2C2_EV_IRQn
    generic_isr,        // I2C2_ER_IRQn
    generic_isr,        // SPI1_IRQn
    generic_isr,        // SPI2_IRQn
    generic_isr,        // USART1_IRQn
    generic_isr,        // USART2_IRQn
    generic_isr,        // USART3_IRQn
    generic_isr,        // EXTI10_IRQn
    generic_isr,        // RTC_TIMESTAMP_IRQn
    generic_isr,        // EXTI11_IRQn
    generic_isr,        // TIM8_BRK_IRQn
    generic_isr,        // TIM8_UP_IRQn
    generic_isr,        // TIM8_TRG_COM_IRQn
    generic_isr,        // TIM8_CC_IRQn
    generic_isr,        // DMA1_Stream7_IRQn
    generic_isr,        // FMC_IRQn
    generic_isr,        // SDMMC1_IRQn
    generic_isr,        // TIM5_IRQn
    generic_isr,        // SPI3_IRQn
    generic_isr,        // UART4_IRQn
    generic_isr,        // UART5_IRQn
    generic_isr,        // TIM6_IRQn
    generic_isr,        // TIM7_IRQn
    generic_isr,        // DMA2_Stream0_IRQn
    generic_isr,        // DMA2_Stream1_IRQn
    generic_isr,        // DMA2_Stream2_IRQn
    generic_isr,        // DMA2_Stream3_IRQn
    generic_isr,        // DMA2_Stream4_IRQn
    generic_isr,        // ETH1_IRQn
    generic_isr,        // ETH1_WKUP_IRQn
    generic_isr,        // FDCAN_CAL_IRQn
    generic_isr,        // EXTI6_IRQn
    generic_isr,        // EXTI7_IRQn
    generic_isr,        // EXTI8_IRQn
    generic_isr,        // EXTI9_IRQn
    generic_isr,        // DMA2_Stream5_IRQn
    generic_isr,        // DMA2_Stream6_IRQn
    generic_isr,        // DMA2_Stream7_IRQn
    generic_isr,        // USART6_IRQn
    generic_isr,        // I2C3_EV_IRQn
    generic_isr,        // I2C3_ER_IRQn
    generic_isr,        // USBH_OHCI_IRQn
    generic_isr,        // USBH_EHCI_IRQn
    generic_isr,        // EXTI12_IRQn
    generic_isr,        // EXTI13_IRQn
    generic_isr,        // DCMI_IRQn
    generic_isr,        // CRYP1_IRQn
    generic_isr,        // HASH1_IRQn
    generic_isr,        // FPU_IRQn
    generic_isr,        // UART7_IRQn
    generic_isr,        // UART8_IRQn
    generic_isr,        // SPI4_IRQn
    generic_isr,        // SPI5_IRQn
    generic_isr,        // SPI6_IRQn
    generic_isr,        // SAI1_IRQn
    generic_isr,        // LTDC_IRQn
    generic_isr,        // LTDC_ER_IRQn
    generic_isr,        // ADC2_IRQn
    generic_isr,        // SAI2_IRQn
    generic_isr,        // QUADSPI_IRQn
    generic_isr,        // LPTIM1_IRQn
    generic_isr,        // CEC_IRQn
    generic_isr,        // I2C4_EV_IRQn
    generic_isr,        // I2C4_ER_IRQn
    generic_isr,        // SPDIF_RX_IRQn
    generic_isr,        // OTG_IRQn
    generic_isr,        // RESERVED_99
    generic_isr,        // IPCC_RX0_IRQn
    generic_isr,        // IPCC_TX0_IRQn
    generic_isr,        // DMAMUX1_OVR_IRQn
    generic_isr,        // IPCC_RX1_IRQn
    generic_isr,        // IPCC_TX1_IRQn
    generic_isr,        // CRYP2_IRQn
    generic_isr,        // HASH2_IRQn
    generic_isr,        // I2C5_EV_IRQn
    generic_isr,        // I2C5_ER_IRQn
    generic_isr,        // GPU_IRQn
    generic_isr,        // DFSDM1_FLT0_IRQn
    generic_isr,        // DFSDM1_FLT1_IRQn
    generic_isr,        // DFSDM1_FLT2_IRQn
    generic_isr,        // DFSDM1_FLT3_IRQn
    generic_isr,        // SAI3_IRQn
    generic_isr,        // DFSDM1_FLT4_IRQn
    generic_isr,        // TIM15_IRQn
    generic_isr,        // TIM16_IRQn
    generic_isr,        // TIM17_IRQn
    generic_isr,        // TIM12_IRQn
    generic_isr,        // MDIOS_IRQn
    generic_isr,        // EXTI14_IRQn
    generic_isr,        // MDMA_IRQn
    generic_isr,        // DSI_IRQn
    generic_isr,        // SDMMC2_IRQn
    generic_isr,        // HSEM_IT2_IRQn
    generic_isr,        // DFSDM1_FLT5_IRQn
    generic_isr,        // EXTI15_IRQn
    generic_isr,        // nCTIIRQ1_IRQn
    generic_isr,        // nCTIIRQ2_IRQn
    generic_isr,        // TIM13_IRQn
    generic_isr,        // TIM14_IRQn
    generic_isr,        // DAC_IRQn
    generic_isr,        // RNG1_IRQn
    generic_isr,        // RNG2_IRQn
    generic_isr,        // I2C6_EV_IRQn
    generic_isr,        // I2C6_ER_IRQn
    generic_isr,        // SDMMC3_IRQn
    generic_isr,        // LPTIM2_IRQn
    generic_isr,        // LPTIM3_IRQn
    generic_isr,        // LPTIM4_IRQn
    generic_isr,        // LPTIM5_IRQn
    generic_isr,        // ETH1_LPI_IRQn
    generic_isr,        // RESERVED_143
    generic_isr,        // MPU_SEV_IRQn
    generic_isr,        // RCC_WAKEUP_IRQn
    generic_isr,        // SAI4_IRQn
    generic_isr,        // DTS_IRQn
    generic_isr,        // RESERVED_148
    generic_isr,        // WAKEUP_PIN_IRQn
];


pub unsafe fn init() {
    cortexm4::nvic::disable_all();
    cortexm4::nvic::clear_all_pending();
    cortexm4::nvic::enable_all();
}