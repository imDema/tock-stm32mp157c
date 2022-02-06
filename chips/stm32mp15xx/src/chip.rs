//! Chip trait setup.

use core::fmt::Write;
use cortexm4;
use kernel::deferred_call;
use kernel::platform::chip::Chip;
use kernel::platform::chip::InterruptService;

use crate::nvic;

use crate::deferred_calls::DeferredCallTask;

pub struct Stm32mp15xx<'a, I: InterruptService<DeferredCallTask> + 'a> {
    mpu: cortexm4::mpu::MPU,
    userspace_kernel_boundary: cortexm4::syscall::SysCall,
    interrupt_service: &'a I,
}

pub struct Stm32mp15xxDefaultPeripherals<'a> {
    pub usart1: crate::usart::Usart<'a>,
    pub usart2: crate::usart::Usart<'a>,
    pub usart3: crate::usart::Usart<'a>,
    pub tim2: crate::tim::Tim<'a>,
    pub tim3: crate::tim::Tim<'a>,
    pub tim4: crate::tim::Tim<'a>,
    pub tim5: crate::tim::Tim<'a>,
    pub gpioa: crate::gpio::GpioPort<'a>,
    pub gpiob: crate::gpio::GpioPort<'a>,
    pub gpiod: crate::gpio::GpioPort<'a>,
    pub gpioh: crate::gpio::GpioPort<'a>,
}

impl<'a> Stm32mp15xxDefaultPeripherals<'a> {
    pub fn new(
        rcc: &'a crate::rcc::Rcc,
    ) -> Self {
        Self {
            usart1: crate::usart::Usart::new_usart1(rcc),
            usart2: crate::usart::Usart::new_usart2(rcc),
            usart3: crate::usart::Usart::new_usart3(rcc),
            tim2: crate::tim::Tim::new(rcc, crate::tim::TIMN::TIM2),
            tim3: crate::tim::Tim::new(rcc, crate::tim::TIMN::TIM3),
            tim4: crate::tim::Tim::new(rcc, crate::tim::TIMN::TIM4),
            tim5: crate::tim::Tim::new(rcc, crate::tim::TIMN::TIM5),
            gpioa: crate::gpio::GpioPort::new(rcc, crate::gpio::PortId::GPIOA),
            gpiob: crate::gpio::GpioPort::new(rcc, crate::gpio::PortId::GPIOB),
            gpiod: crate::gpio::GpioPort::new(rcc, crate::gpio::PortId::GPIOD),
            gpioh: crate::gpio::GpioPort::new(rcc, crate::gpio::PortId::GPIOH),
        }
    }

    pub fn setup_circular_deps(&'a self) {
        self.gpioa.setup_circular_deps();
        self.gpiob.setup_circular_deps();
        self.gpiod.setup_circular_deps();
        self.gpioh.setup_circular_deps();
    }
}

impl<'a> InterruptService<DeferredCallTask> for Stm32mp15xxDefaultPeripherals<'a> {
    unsafe fn service_interrupt(&self, interrupt: u32) -> bool {
        match interrupt {
            nvic::USART1    => self.usart1.handle_interrupt(),
            nvic::USART2    => self.usart2.handle_interrupt(),
            nvic::USART3    => self.usart3.handle_interrupt(),
            nvic::TIM2      => self.tim2.handle_interrupt(),
            nvic::TIM3      => self.tim3.handle_interrupt(),
            nvic::TIM4      => self.tim4.handle_interrupt(),
            nvic::TIM5      => self.tim5.handle_interrupt(),
            
            _ => return false,
        }
        true
    }

    unsafe fn service_deferred_call(&self, _task: DeferredCallTask) -> bool {
        // match task {
        //     // DeferredCallTask::Fsmc => self.fsmc.handle_interrupt(),
        // }
        true
    }
}

impl<'a, I: InterruptService<DeferredCallTask> + 'a> Stm32mp15xx<'a, I> {
    pub unsafe fn new(interrupt_service: &'a I) -> Self {
        Self {
            mpu: cortexm4::mpu::MPU::new(),
            userspace_kernel_boundary: cortexm4::syscall::SysCall::new(),
            interrupt_service,
        }
    }
}

impl<'a, I: InterruptService<DeferredCallTask> + 'a> Chip for Stm32mp15xx<'a, I> {
    type MPU = cortexm4::mpu::MPU;
    type UserspaceKernelBoundary = cortexm4::syscall::SysCall;

    fn service_pending_interrupts(&self) {
        unsafe {
            loop {
                if let Some(task) = deferred_call::DeferredCall::next_pending() {
                    if !self.interrupt_service.service_deferred_call(task) {
                        panic!("Unhandled deferred call");
                    }
                } else if let Some(interrupt) = cortexm4::nvic::next_pending() {
                    if !self.interrupt_service.service_interrupt(interrupt) {
                        panic!("unhandled interrupt {}", interrupt);
                    }

                    let n = cortexm4::nvic::Nvic::new(interrupt);
                    n.clear_pending();
                    n.enable();
                } else {
                    break;
                }
            }
        }
    }

    fn has_pending_interrupts(&self) -> bool {
        unsafe { cortexm4::nvic::has_pending() || deferred_call::has_tasks() }
    }

    fn mpu(&self) -> &cortexm4::mpu::MPU {
        &self.mpu
    }

    fn userspace_kernel_boundary(&self) -> &cortexm4::syscall::SysCall {
        &self.userspace_kernel_boundary
    }

    fn sleep(&self) {
        unsafe {
            cortexm4::scb::unset_sleepdeep();
            cortexm4::support::wfi();
        }
    }

    unsafe fn atomic<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        cortexm4::support::atomic(f)
    }

    unsafe fn print_state(&self, write: &mut dyn Write) {
        cortexm4::print_cortexm4_state(write);
    }
}
