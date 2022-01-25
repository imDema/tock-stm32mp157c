//! Board file for Stm32mp157cDiscovery Kit development board
//!
//! - <https://www.st.com/en/evaluation-tools/stm32mp157cdiscovery.html>

#![no_std]
// Disable this attribute when documenting, as a workaround for
// https://github.com/rust-lang/rust/issues/62184.
#![cfg_attr(not(doc), no_main)]
#![deny(missing_docs)]

use capsules::virtual_alarm::VirtualMuxAlarm;
// use components::gpio::GpioComponent;
use kernel::capabilities;
use kernel::component::Component;
use kernel::dynamic_deferred_call::{DynamicDeferredCall, DynamicDeferredCallClientState};
use kernel::hil::gpio::Configure;
use kernel::hil::led::LedLow;
use kernel::hil::uart::Transmit;
// use kernel::hil::gpio::Configure;
// use kernel::hil::gpio::Output;
// use kernel::hil::led::LedHigh;
// use kernel::hil::time::Counter;
use kernel::platform::{KernelResources, SyscallDriverLookup};
use kernel::scheduler::round_robin::RoundRobinSched;
use kernel::{create_capability, debug, static_init};
use stm32mp15xx::chip::Stm32mp15xxDefaultPeripherals;
use stm32mp15xx::gpio::Mode;

/// Support routines for debugging I/O.
pub mod io;
pub mod resource_table;

// Number of concurrent processes this platform supports.
const NUM_PROCS: usize = 4;

// Actual memory for holding the active process structures.
static mut PROCESSES: [Option<&'static dyn kernel::process::Process>; NUM_PROCS] =
    [None, None, None, None];

// Static reference to chip for panic dumps.
static mut CHIP: Option<&'static stm32mp15xx::chip::Stm32mp15xx<Stm32mp15xxDefaultPeripherals>> = None;
// Static reference to process printer for panic dumps.
static mut PROCESS_PRINTER: Option<&'static kernel::process::ProcessPrinterText> = None;

// How should the kernel respond when a process faults.
const FAULT_RESPONSE: kernel::process::PanicFaultPolicy = kernel::process::PanicFaultPolicy {};

/// Dummy buffer that causes the linker to reserve enough space for the stack.
#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u8; 0x2000] = [0; 0x2000];

/// A structure representing this platform that holds references to all
/// capsules for this platform.
struct Stm32mp157cDiscovery {
    console: &'static capsules::console::Console<'static>,
    // ipc: kernel::ipc::IPC<NUM_PROCS>,
    // gpio: &'static capsules::gpio::GPIO<'static, stm32mp15xx::gpio::GpioPin<'static>>,
    led: &'static capsules::led::LedDriver<
        'static,
        LedLow<'static, stm32mp15xx::gpio::GpioPin<'static>>,
        2,
    >,
    // button: &'static capsules::button::Button<'static, stm32mp15xx::gpio::Pin<'static>>,
    // ninedof: &'static capsules::ninedof::NineDof<'static>,
    // l3gd20: &'static capsules::l3gd20::L3gd20Spi<'static>,
    // lsm303dlhc: &'static capsules::lsm303dlhc::Lsm303dlhcI2C<'static>,
    // temp: &'static capsules::temperature::TemperatureSensor<'static>,
    alarm: &'static capsules::alarm::AlarmDriver<
        'static,
        VirtualMuxAlarm<'static, stm32mp15xx::tim2::Tim2<'static>>,
    >,
    // adc: &'static capsules::adc::AdcVirtualized<'static>,
    // nonvolatile_storage: &'static capsules::nonvolatile_storage_driver::NonvolatileStorage<'static>,

    scheduler: &'static RoundRobinSched<'static>,
    systick: cortexm4::systick::SysTick,
    // watchdog: &'static wdt::WindoWdg<'static>,
}

/// Mapping of integer syscalls to objects that implement syscalls.
impl SyscallDriverLookup for Stm32mp157cDiscovery {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
    where
        F: FnOnce(Option<&dyn kernel::syscall::SyscallDriver>) -> R,
    {
        match driver_num {
            capsules::console::DRIVER_NUM => f(Some(self.console)),
            capsules::alarm::DRIVER_NUM => f(Some(self.alarm)),
            // capsules::gpio::DRIVER_NUM => f(Some(self.gpio)),
            capsules::led::DRIVER_NUM => f(Some(self.led)),
            // capsules::button::DRIVER_NUM => f(Some(self.button)),
            // capsules::l3gd20::DRIVER_NUM => f(Some(self.l3gd20)),
            // capsules::lsm303dlhc::DRIVER_NUM => f(Some(self.lsm303dlhc)),
            // capsules::ninedof::DRIVER_NUM => f(Some(self.ninedof)),
            // capsules::temperature::DRIVER_NUM => f(Some(self.temp)),
            // kernel::ipc::DRIVER_NUM => f(Some(&self.ipc)),
            // capsules::adc::DRIVER_NUM => f(Some(self.adc)),
            // capsules::nonvolatile_storage_driver::DRIVER_NUM => f(Some(self.nonvolatile_storage)),
            _ => f(None),
        }
    }
}

impl
    KernelResources<
        stm32mp15xx::chip::Stm32mp15xx<
            'static,
            stm32mp15xx::chip::Stm32mp15xxDefaultPeripherals<'static>,
        >,
    > for Stm32mp157cDiscovery
{
    type SyscallDriverLookup = Self;
    type SyscallFilter = ();
    type ProcessFault = ();
    type Scheduler = RoundRobinSched<'static>;
    type SchedulerTimer = cortexm4::systick::SysTick;
    type WatchDog = (); // wdt::WindoWdg<'static>;
    type ContextSwitchCallback = ();

    fn syscall_driver_lookup(&self) -> &Self::SyscallDriverLookup {
        &self
    }
    fn syscall_filter(&self) -> &Self::SyscallFilter {
        &()
    }
    fn process_fault(&self) -> &Self::ProcessFault {
        &()
    }
    fn scheduler(&self) -> &Self::Scheduler {
        self.scheduler
    }
    fn scheduler_timer(&self) -> &Self::SchedulerTimer {
        &self.systick
    }
    fn watchdog(&self) -> &Self::WatchDog {
        &()// self.watchdog
    }
    fn context_switch_callback(&self) -> &Self::ContextSwitchCallback {
        &()
    }
}

/// Helper function for miscellaneous peripheral functions
unsafe fn setup_peripherals(tim2: &stm32mp15xx::tim2::Tim2) {
    cortexm4::nvic::Nvic::new(stm32mp15xx::nvic::USART3).enable();

    tim2.enable_clock();
    let _ = tim2.start();
    cortexm4::nvic::Nvic::new(stm32mp15xx::nvic::TIM2).enable();
}

/// Statically initialize the core peripherals for the chip.
///
/// This is in a separate, inline(never) function so that its stack frame is
/// removed when this function returns. Otherwise, the stack space used for
/// these static_inits is wasted.
#[inline(never)]
unsafe fn get_peripherals() -> (
    &'static mut Stm32mp15xxDefaultPeripherals<'static>,
    &'static stm32mp15xx::rcc::Rcc,
) {
    let rcc = static_init!(stm32mp15xx::rcc::Rcc, stm32mp15xx::rcc::Rcc::new());

    let peripherals = static_init!(
        Stm32mp15xxDefaultPeripherals,
        Stm32mp15xxDefaultPeripherals::new(rcc)
    );

    (peripherals, rcc)
}


/// Helper function called during bring-up that configures multiplexed I/O.
unsafe fn set_pin_primary_functions(
    gpioa: &'static stm32mp15xx::gpio::GpioPort<'static>,
    gpiob: &'static stm32mp15xx::gpio::GpioPort<'static>,
) {
    // use stm32mp15xx::gpio::{AlternateFunction, Mode, PinId, PortId};

    gpioa.enable_clock();
    gpiob.enable_clock();

    // gpio_ports.get_pin(PinId::PE14).map(|pin| {
    //     pin.make_output();
    //     pin.set();
    // });

    // // User LD4 is connected to PA14. Configure PA14 as `debug_gpio!(0, ...)`
    // gpio_ports.get_pin(PinId::PE09).map(|pin| {
    //     pin.make_output();

    //     // Configure kernel debug gpios as early as possible
    //     kernel::debug::assign_gpios(Some(pin), None, None);
    // });
}

fn run_test() {
    use kernel::hil::led::*;
    let rcc = stm32mp15xx::rcc::Rcc::new();
    let gpioa = stm32mp15xx::gpio::GpioPort::new(&rcc, stm32mp15xx::gpio::PortId::GPIOA);
    gpioa.enable_clock();
    let gpioh = stm32mp15xx::gpio::GpioPort::new(&rcc, stm32mp15xx::gpio::PortId::GPIOH);
    gpioh.enable_clock();

    let pa13 = &gpioa[13];
    pa13.set_ports_ref(&gpioa);
    let ld6 = &mut LedLow::new(pa13); // RED
    
    let pa14 = &gpioa[14];
    pa14.set_ports_ref(&gpioa);
    pa14.set_mode(Mode::GeneralPurposeOutputMode);
    let ld5 = &mut LedLow::new(pa14); // GREEN

    let ph7 = &gpioh[7];
    ph7.set_ports_ref(&gpioh);
    ph7.set_mode(Mode::GeneralPurposeOutputMode);
    let ld7 = &mut LedHigh::new(ph7); // ORANGE

    loop {
        for _ in 0..500000 {
            ld5.on();
            ld6.off();
            ld7.off()
        }
        for _ in 0..500000 {
            ld5.off();
            ld6.on();
            ld7.off()
        }
        for _ in 0..500000 {
            ld5.off();
            ld6.off();
            ld7.on()
        } 
    }
}

/// Main function.
///
/// This is called after RAM initialization is complete.
#[no_mangle]
pub unsafe fn main() {
    run_test();

    stm32mp15xx::init();

    let (peripherals, _rcc) = get_peripherals();
    peripherals.setup_circular_deps();

    set_pin_primary_functions(
        &peripherals.gpioa,
        &peripherals.gpiob,
    );

    setup_peripherals(&peripherals.tim2);

    let board_kernel = static_init!(kernel::Kernel, kernel::Kernel::new(&PROCESSES));
    let dynamic_deferred_call_clients =
        static_init!([DynamicDeferredCallClientState; 3], Default::default());
    let dynamic_deferred_caller = static_init!(
        DynamicDeferredCall,
        DynamicDeferredCall::new(dynamic_deferred_call_clients)
    );
    DynamicDeferredCall::set_global_instance(dynamic_deferred_caller);

    let chip = static_init!(
        stm32mp15xx::chip::Stm32mp15xx<Stm32mp15xxDefaultPeripherals>,
        stm32mp15xx::chip::Stm32mp15xx::new(peripherals)
    );
    CHIP = Some(chip);

    // LED

    let led = components::led::LedsComponent::new().finalize(components::led_component_helper!(
        LedLow<'static, stm32mp15xx::gpio::GpioPin<'static>>,
        LedLow::new(&peripherals.gpioa[14]),
        LedLow::new(&peripherals.gpioa[13]),
    ));

    // UART

    // Create a shared UART channel for kernel debug.
    peripherals.usart3.enable_clock();
    let uart_mux = components::console::UartMuxComponent::new(
        &peripherals.usart3,
        115200,
        dynamic_deferred_caller,
    )
    .finalize(());

    // `finalize()` configures the underlying USART, so we need to
    // tell `send_byte()` not to configure the USART again.
    io::WRITER.set_initialized();

    // Create capabilities that the board needs to call certain protected kernel
    // functions.
    let memory_allocation_capability = create_capability!(capabilities::MemoryAllocationCapability);
    let main_loop_capability = create_capability!(capabilities::MainLoopCapability);
    let process_management_capability =
        create_capability!(capabilities::ProcessManagementCapability);

    // Setup the console.
    let console = components::console::ConsoleComponent::new(
        board_kernel,
        capsules::console::DRIVER_NUM,
        uart_mux,
    )
    .finalize(());
    // Create the debugger object that handles calls to `debug!()`.
    components::debug_writer::DebugWriterComponent::new(uart_mux).finalize(());

    // ALARM

    let tim2 = &peripherals.tim2;
    let mux_alarm = components::alarm::AlarmMuxComponent::new(tim2).finalize(
        components::alarm_mux_component_helper!(stm32mp15xx::tim2::Tim2),
    );

    let alarm = components::alarm::AlarmDriverComponent::new(
        board_kernel,
        capsules::alarm::DRIVER_NUM,
        mux_alarm,
    )
    .finalize(components::alarm_component_helper!(stm32mp15xx::tim2::Tim2));

    let process_printer =
        components::process_printer::ProcessPrinterTextComponent::new().finalize(());
    PROCESS_PRINTER = Some(process_printer);

    // PROCESS CONSOLE
    let process_console = components::process_console::ProcessConsoleComponent::new(
        board_kernel,
        uart_mux,
        mux_alarm,
        process_printer,
    )
    .finalize(components::process_console_component_helper!(
        stm32mp15xx::tim2::Tim2
    ));
    let _ = process_console.start();

    let scheduler = components::sched::round_robin::RoundRobinComponent::new(&PROCESSES)
        .finalize(components::rr_component_helper!(NUM_PROCS));

    let stm32mp157cdiscovery = Stm32mp157cDiscovery {
        console,
        led,
        // gpio,
        alarm,
        scheduler,
        systick: cortexm4::systick::SysTick::new(),
    };

    // // Optional kernel tests
    // //
    // // See comment in `boards/imix/src/main.rs`
    // virtual_uart_rx_test::run_virtual_uart_receive(mux_uart);

    debug!("Initialization complete. Entering main loop");

    /// These symbols are defined in the linker script.
    extern "C" {
        /// Beginning of the ROM region containing app images.
        static _sapps: u8;
        /// End of the ROM region containing app images.
        static _eapps: u8;
        /// Beginning of the RAM region for app memory.
        static mut _sappmem: u8;
        /// End of the RAM region for app memory.
        static _eappmem: u8;
    }

    kernel::process::load_processes(
        board_kernel,
        chip,
        core::slice::from_raw_parts(
            &_sapps as *const u8,
            &_eapps as *const u8 as usize - &_sapps as *const u8 as usize,
        ),
        core::slice::from_raw_parts_mut(
            &mut _sappmem as *mut u8,
            &_eappmem as *const u8 as usize - &_sappmem as *const u8 as usize,
        ),
        &mut PROCESSES,
        &FAULT_RESPONSE,
        &process_management_capability,
    )
    .unwrap_or_else(|err| {
        debug!("Error loading processes!");
        debug!("{:?}", err);
    });

    // Uncomment this to enable the watchdog
    // peripherals.watchdog.enable();

    //Uncomment to run multi alarm test
    /*components::test::multi_alarm_test::MultiAlarmTestComponent::new(mux_alarm)
    .finalize(components::multi_alarm_test_component_buf!(stm32mp15xx::tim2::Tim2))
    .run();*/
    board_kernel.kernel_loop::<_, _, NUM_PROCS>(
        &stm32mp157cdiscovery,
        chip,
        None,// Some(&stm32mp157cdiscovery.ipc),
        &main_loop_capability,
    );
}
