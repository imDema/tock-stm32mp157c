use core::fmt::Write;
use core::panic::PanicInfo;

use kernel::debug;
use kernel::debug::IoWrite;
use kernel::hil::led;

use kernel::utilities::cells::MapCell;
use stm32mp15xx;
use stm32mp15xx::trace;

use crate::CHIP;
use crate::PROCESSES;
use crate::PROCESS_PRINTER;

/// Writer is used by kernel::debug to panic message to the serial port.
pub struct Writer {
    trace: MapCell<trace::TraceBuffer<'static>>,
}

/// Global static for debug writer
pub static mut WRITER: Writer = Writer { trace: MapCell::empty() };

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}

impl IoWrite for Writer {
    fn write(&mut self, buf: &[u8]) {
        if self.trace.is_none() {
            self.trace.put(unsafe { trace::steal_trace() });
        }

        for &c in buf {
            self.trace.map(|t| t.write_char(c as char));
        }
    }
}

/// Panic handler.
#[no_mangle]
#[panic_handler]
pub unsafe extern "C" fn panic_fmt(info: &PanicInfo) -> ! {
    // Have to reinitialize peripherals because otherwise can't access them here.
    let rcc = stm32mp15xx::rcc::Rcc::new();

    let gpioa = stm32mp15xx::gpio::GpioPort::new(&rcc, stm32mp15xx::gpio::PortId::GPIOA);
    gpioa.enable_clock();
    let pa13 = &gpioa[13];
    pa13.set_ports_ref(&gpioa);
    let ld6 = &mut led::LedLow::new(pa13); // RED

    let writer = &mut WRITER;
    debug::panic(
        &mut [ld6],
        writer,
        info,
        &cortexm4::support::nop,
        &PROCESSES,
        &CHIP,
        &PROCESS_PRINTER,
    )
}
