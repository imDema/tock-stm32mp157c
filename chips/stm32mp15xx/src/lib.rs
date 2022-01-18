//! Peripheral implementations for the STM32F3xx MCU.
//!
//! STM32MP15xx: <https://www.st.com/en/microcontrollers-microprocessors/STM32MP15xx.html>

#![crate_name = "stm32mp15xx"]
#![crate_type = "rlib"]
#![feature(const_fn_trait_bound)]
#![no_std]
#![recursion_limit = "1024"]

pub mod chip;
// mod deferred_call_tasks;
pub mod nvic;

// Peripherals
// pub mod adc;
// pub mod dma1;
// pub mod exti;
// pub mod flash;
// pub mod gpio;
// pub mod i2c;
pub mod rcc;
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
    generic_isr,        // WWDG1
    generic_isr,        // PVD_AVD
    generic_isr,        // TAMP
    generic_isr,        // RTC_WKUP_ALARM
    generic_isr,        // RESERVED_4
    generic_isr,        // RCC
    generic_isr,        // EXTI0
    generic_isr,        // EXTI1
    generic_isr,        // EXTI2
    generic_isr,        // EXTI3
    generic_isr,        // EXTI4
    generic_isr,        // DMA1_Stream0
    generic_isr,        // DMA1_Stream1
    generic_isr,        // DMA1_Stream2
    generic_isr,        // DMA1_Stream3
    generic_isr,        // DMA1_Stream4
    generic_isr,        // DMA1_Stream5
    generic_isr,        // DMA1_Stream6
    generic_isr,        // ADC1
    generic_isr,        // FDCAN1_IT0
    generic_isr,        // FDCAN2_IT0
    generic_isr,        // FDCAN1_IT1
    generic_isr,        // FDCAN2_IT1
    generic_isr,        // EXTI5
    generic_isr,        // TIM1_BRK
    generic_isr,        // TIM1_UP
    generic_isr,        // TIM1_TRG_COM
    generic_isr,        // TIM1_CC
    generic_isr,        // TIM2
    generic_isr,        // TIM3
    generic_isr,        // TIM4
    generic_isr,        // I2C1_EV
    generic_isr,        // I2C1_ER
    generic_isr,        // I2C2_EV
    generic_isr,        // I2C2_ER
    generic_isr,        // SPI1
    generic_isr,        // SPI2
    generic_isr,        // USART1
    generic_isr,        // USART2
    generic_isr,        // USART3
    generic_isr,        // EXTI10
    generic_isr,        // RTC_TIMESTAMP
    generic_isr,        // EXTI11
    generic_isr,        // TIM8_BRK
    generic_isr,        // TIM8_UP
    generic_isr,        // TIM8_TRG_COM
    generic_isr,        // TIM8_CC
    generic_isr,        // DMA1_Stream7
    generic_isr,        // FMC
    generic_isr,        // SDMMC1
    generic_isr,        // TIM5
    generic_isr,        // SPI3
    generic_isr,        // UART4
    generic_isr,        // UART5
    generic_isr,        // TIM6
    generic_isr,        // TIM7
    generic_isr,        // DMA2_Stream0
    generic_isr,        // DMA2_Stream1
    generic_isr,        // DMA2_Stream2
    generic_isr,        // DMA2_Stream3
    generic_isr,        // DMA2_Stream4
    generic_isr,        // ETH1
    generic_isr,        // ETH1_WKUP
    generic_isr,        // FDCAN_CAL
    generic_isr,        // EXTI6
    generic_isr,        // EXTI7
    generic_isr,        // EXTI8
    generic_isr,        // EXTI9
    generic_isr,        // DMA2_Stream5
    generic_isr,        // DMA2_Stream6
    generic_isr,        // DMA2_Stream7
    generic_isr,        // USART6
    generic_isr,        // I2C3_EV
    generic_isr,        // I2C3_ER
    generic_isr,        // USBH_OHCI
    generic_isr,        // USBH_EHCI
    generic_isr,        // EXTI12
    generic_isr,        // EXTI13
    generic_isr,        // DCMI
    generic_isr,        // CRYP1
    generic_isr,        // HASH1
    generic_isr,        // FPU
    generic_isr,        // UART7
    generic_isr,        // UART8
    generic_isr,        // SPI4
    generic_isr,        // SPI5
    generic_isr,        // SPI6
    generic_isr,        // SAI1
    generic_isr,        // LTDC
    generic_isr,        // LTDC_ER
    generic_isr,        // ADC2
    generic_isr,        // SAI2
    generic_isr,        // QUADSPI
    generic_isr,        // LPTIM1
    generic_isr,        // CEC
    generic_isr,        // I2C4_EV
    generic_isr,        // I2C4_ER
    generic_isr,        // SPDIF_RX
    generic_isr,        // OTG
    generic_isr,        // RESERVED_99
    generic_isr,        // IPCC_RX0
    generic_isr,        // IPCC_TX0
    generic_isr,        // DMAMUX1_OVR
    generic_isr,        // IPCC_RX1
    generic_isr,        // IPCC_TX1
    generic_isr,        // CRYP2
    generic_isr,        // HASH2
    generic_isr,        // I2C5_EV
    generic_isr,        // I2C5_ER
    generic_isr,        // GPU
    generic_isr,        // DFSDM1_FLT0
    generic_isr,        // DFSDM1_FLT1
    generic_isr,        // DFSDM1_FLT2
    generic_isr,        // DFSDM1_FLT3
    generic_isr,        // SAI3
    generic_isr,        // DFSDM1_FLT4
    generic_isr,        // TIM15
    generic_isr,        // TIM16
    generic_isr,        // TIM17
    generic_isr,        // TIM12
    generic_isr,        // MDIOS
    generic_isr,        // EXTI14
    generic_isr,        // MDMA
    generic_isr,        // DSI
    generic_isr,        // SDMMC2
    generic_isr,        // HSEM_IT2
    generic_isr,        // DFSDM1_FLT5
    generic_isr,        // EXTI15
    generic_isr,        // nCTIIRQ1
    generic_isr,        // nCTIIRQ2
    generic_isr,        // TIM13
    generic_isr,        // TIM14
    generic_isr,        // DAC
    generic_isr,        // RNG1
    generic_isr,        // RNG2
    generic_isr,        // I2C6_EV
    generic_isr,        // I2C6_ER
    generic_isr,        // SDMMC3
    generic_isr,        // LPTIM2
    generic_isr,        // LPTIM3
    generic_isr,        // LPTIM4
    generic_isr,        // LPTIM5
    generic_isr,        // ETH1_LPI
    generic_isr,        // RESERVED_143
    generic_isr,        // MPU_SEV
    generic_isr,        // RCC_WAKEUP
    generic_isr,        // SAI4
    generic_isr,        // DTS
    generic_isr,        // RESERVED_148
    generic_isr,        // WAKEUP_PIN
];


pub unsafe fn init() {
    cortexm4::nvic::disable_all();
    cortexm4::nvic::clear_all_pending();
    cortexm4::nvic::enable_all();
}