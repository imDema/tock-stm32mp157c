use core::fmt::Write;
use core::panic::PanicInfo;

use kernel::debug;
use kernel::debug::IoWrite;
use kernel::hil::led;
use kernel::hil::uart;
use kernel::hil::uart::Configure;

use stm32mp15xx;
// use stm32mp15xx::gpio::PinId;

use crate::CHIP;
use crate::PROCESSES;
use crate::PROCESS_PRINTER;

/// Writer is used by kernel::debug to panic message to the serial port.
pub struct Writer {
    initialized: bool,
}

/// Global static for debug writer
pub static mut WRITER: Writer = Writer { initialized: false };

impl Writer {
    /// Indicate that USART has already been initialized. Trying to double
    /// initialize USART2 causes STM32F446RE to go into in in-deterministic state.
    pub fn set_initialized(&mut self) {
        self.initialized = true;
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}

impl IoWrite for Writer {
    fn write(&mut self, buf: &[u8]) {
        let rcc = stm32mp15xx::rcc::Rcc::new();
        let uart = stm32mp15xx::usart::Usart::new_usart3(&rcc);

        if !self.initialized {
            self.initialized = true;

            let _ = uart.configure(uart::Parameters {
                baud_rate: 11500,
                stop_bits: uart::StopBits::One,
                parity: uart::Parity::None,
                hw_flow_control: false,
                width: uart::Width::Eight,
            });
        }

        for &c in buf {
            uart.send_byte(c);
        }
    }
}

/// Panic handler.
#[no_mangle]
#[panic_handler]
pub unsafe extern "C" fn panic_fmt(info: &PanicInfo) -> ! {
    // Have to reinitialize several peripherals because otherwise can't access them here.
    let rcc = stm32mp15xx::rcc::Rcc::new();
    // let syscfg = stm32mp15xx::syscfg::Syscfg::new(&rcc);
    // let exti = stm32mp15xx::exti::Exti::new(&syscfg);
    let gpioa = stm32mp15xx::gpio::GpioPort::new(&rcc, stm32mp15xx::gpio::PortId::GPIOA);
    let pin = &gpioa[13];
    pin.set_ports_ref(&gpioa);

    let led = &mut led::LedLow::new(pin);
    let writer = &mut WRITER;

    debug::panic(
        &mut [led],
        writer,
        info,
        &cortexm4::support::nop,
        &PROCESSES,
        &CHIP,
        &PROCESS_PRINTER,
    )
}
