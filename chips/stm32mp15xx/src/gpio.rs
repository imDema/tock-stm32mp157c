use core::ops::{Index, IndexMut};
use enum_primitive::cast::FromPrimitive;
use enum_primitive::enum_from_primitive;
use kernel::hil::gpio;
use kernel::platform::chip::ClockInterface;
use kernel::utilities::cells::OptionalCell;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{
    register_bitfields, register_structs, ReadOnly, ReadWrite, WriteOnly,
};
use kernel::utilities::StaticRef;

use crate::rcc;

#[rustfmt::skip]
#[derive(Copy, Clone)]
pub enum PortId {
    GPIOA,
    GPIOB,
    GPIOD,
    GPIOG,
    GPIOH,
}

#[rustfmt::skip]
#[derive(Copy, Clone)]
pub enum PinId {
    Pin00 = 0,  Pin01 = 1,  Pin02 = 2,  Pin03 = 3,
    Pin04 = 4,  Pin05 = 5,  Pin06 = 6,  Pin07 = 7,
    Pin08 = 8,  Pin09 = 9,  Pin10 = 10, Pin11 = 11,
    Pin12 = 12, Pin13 = 13, Pin14 = 14, Pin15 = 15,
}

enum_from_primitive! {
    #[repr(u32)]
    /// GPIO pin internal pull-up and pull-down [^1]
    ///
    /// [^1]: Section 7.4.4, page 189 of reference manual
    enum PullUpPullDown {
        NoPullUpPullDown = 0b00,
        PullUp = 0b01,
        PullDown = 0b10,
    }
}

enum_from_primitive! {
    #[repr(u32)]
    #[derive(PartialEq)]
    /// GPIO pin mode [^1]
    ///
    /// [^1]: Section 7.1.4, page 187 of reference manual
    pub enum Mode {
        Input = 0b00,
        GeneralPurposeOutputMode = 0b01,
        AlternateFunctionMode = 0b10,
        AnalogMode = 0b11,
    }
}

#[repr(u32)]
pub enum AlternateFunction {
    AF0 = 0b0000,
    AF1 = 0b0001,
    AF2 = 0b0010,
    AF3 = 0b0011,
    AF4 = 0b0100,
    AF5 = 0b0101,
    AF6 = 0b0110,
    AF7 = 0b0111,
    AF8 = 0b1000,
    AF9 = 0b1001,
    AF10 = 0b1010,
    AF11 = 0b1011,
    AF12 = 0b1100,
    AF13 = 0b1101,
    AF14 = 0b1110,
    AF15 = 0b1111,
}

// `exti_lineid` is used to configure EXTI settings for the Pin.
pub struct GpioPin<'a> {
    pinid: PinId,
    ports_ref: OptionalCell<&'a GpioPort<'a>>,
    client: OptionalCell<&'a dyn gpio::Client>,
}

impl<'a> GpioPin<'a> {
    pub const fn new(pinid: PinId) -> Self {
        Self {
            pinid,
            ports_ref: OptionalCell::empty(),
            client: OptionalCell::empty(),
        }
    }

    pub fn set_ports_ref(&self, ports: &'a GpioPort<'a>) {
        self.ports_ref.set(ports);
    }

    pub fn set_client(&self, client: &'a dyn gpio::Client) {
        self.client.set(client);
    }

    // pub fn handle_interrupt(&self) {
    //     self.client.map(|client| client.fired());
    // }

    pub fn get_mode(&self) -> Mode {
        let port = self.ports_ref.unwrap_or_panic(); // Unwrap fail =

        let val = match self.pinid {
            PinId::Pin00 => port.registers.moder.read(MODER::MODER0),
            PinId::Pin01 => port.registers.moder.read(MODER::MODER1),
            PinId::Pin02 => port.registers.moder.read(MODER::MODER2),
            PinId::Pin03 => port.registers.moder.read(MODER::MODER3),
            PinId::Pin04 => port.registers.moder.read(MODER::MODER4),
            PinId::Pin05 => port.registers.moder.read(MODER::MODER5),
            PinId::Pin06 => port.registers.moder.read(MODER::MODER6),
            PinId::Pin07 => port.registers.moder.read(MODER::MODER7),
            PinId::Pin08 => port.registers.moder.read(MODER::MODER8),
            PinId::Pin09 => port.registers.moder.read(MODER::MODER9),
            PinId::Pin10 => port.registers.moder.read(MODER::MODER10),
            PinId::Pin11 => port.registers.moder.read(MODER::MODER11),
            PinId::Pin12 => port.registers.moder.read(MODER::MODER12),
            PinId::Pin13 => port.registers.moder.read(MODER::MODER13),
            PinId::Pin14 => port.registers.moder.read(MODER::MODER14),
            PinId::Pin15 => port.registers.moder.read(MODER::MODER15),
        };

        Mode::from_u32(val).unwrap_or(Mode::Input)
    }

    pub fn set_mode(&self, mode: Mode) {
        let port = self.ports_ref.unwrap_or_panic(); // Unwrap fail =

        match self.pinid {
            PinId::Pin00 => port.registers.moder.modify(MODER::MODER0.val(mode as u32)),
            PinId::Pin01 => port.registers.moder.modify(MODER::MODER1.val(mode as u32)),
            PinId::Pin02 => port.registers.moder.modify(MODER::MODER2.val(mode as u32)),
            PinId::Pin03 => port.registers.moder.modify(MODER::MODER3.val(mode as u32)),
            PinId::Pin04 => port.registers.moder.modify(MODER::MODER4.val(mode as u32)),
            PinId::Pin05 => port.registers.moder.modify(MODER::MODER5.val(mode as u32)),
            PinId::Pin06 => port.registers.moder.modify(MODER::MODER6.val(mode as u32)),
            PinId::Pin07 => port.registers.moder.modify(MODER::MODER7.val(mode as u32)),
            PinId::Pin08 => port.registers.moder.modify(MODER::MODER8.val(mode as u32)),
            PinId::Pin09 => port.registers.moder.modify(MODER::MODER9.val(mode as u32)),
            PinId::Pin10 => port.registers.moder.modify(MODER::MODER10.val(mode as u32)),
            PinId::Pin11 => port.registers.moder.modify(MODER::MODER11.val(mode as u32)),
            PinId::Pin12 => port.registers.moder.modify(MODER::MODER12.val(mode as u32)),
            PinId::Pin13 => port.registers.moder.modify(MODER::MODER13.val(mode as u32)),
            PinId::Pin14 => port.registers.moder.modify(MODER::MODER14.val(mode as u32)),
            PinId::Pin15 => port.registers.moder.modify(MODER::MODER15.val(mode as u32)),
        }
    }

    pub fn set_alternate_function(&self, af: AlternateFunction) {
        let port = self.ports_ref.unwrap_or_panic(); // Unwrap fail =

        match self.pinid {
            PinId::Pin00 => port.registers.afrl.modify(AFRL::AFR0.val(af as u32)),
            PinId::Pin01 => port.registers.afrl.modify(AFRL::AFR1.val(af as u32)),
            PinId::Pin02 => port.registers.afrl.modify(AFRL::AFR2.val(af as u32)),
            PinId::Pin03 => port.registers.afrl.modify(AFRL::AFR3.val(af as u32)),
            PinId::Pin04 => port.registers.afrl.modify(AFRL::AFR4.val(af as u32)),
            PinId::Pin05 => port.registers.afrl.modify(AFRL::AFR5.val(af as u32)),
            PinId::Pin06 => port.registers.afrl.modify(AFRL::AFR6.val(af as u32)),
            PinId::Pin07 => port.registers.afrl.modify(AFRL::AFR7.val(af as u32)),
            PinId::Pin08 => port.registers.afrh.modify(AFRH::AFR8.val(af as u32)),
            PinId::Pin09 => port.registers.afrh.modify(AFRH::AFR9.val(af as u32)),
            PinId::Pin10 => port.registers.afrh.modify(AFRH::AFR10.val(af as u32)),
            PinId::Pin11 => port.registers.afrh.modify(AFRH::AFR11.val(af as u32)),
            PinId::Pin12 => port.registers.afrh.modify(AFRH::AFR12.val(af as u32)),
            PinId::Pin13 => port.registers.afrh.modify(AFRH::AFR13.val(af as u32)),
            PinId::Pin14 => port.registers.afrh.modify(AFRH::AFR14.val(af as u32)),
            PinId::Pin15 => port.registers.afrh.modify(AFRH::AFR15.val(af as u32)),
        }
    }

    pub fn get_pinid(&self) -> PinId {
        self.pinid
    }

    fn set_mode_output_pushpull(&self) {
        let port = self.ports_ref.unwrap_or_panic(); // Unwrap fail =

        match self.pinid {
            PinId::Pin00 => port.registers.otyper.modify(OTYPER::OT0::CLEAR),
            PinId::Pin01 => port.registers.otyper.modify(OTYPER::OT1::CLEAR),
            PinId::Pin02 => port.registers.otyper.modify(OTYPER::OT2::CLEAR),
            PinId::Pin03 => port.registers.otyper.modify(OTYPER::OT3::CLEAR),
            PinId::Pin04 => port.registers.otyper.modify(OTYPER::OT4::CLEAR),
            PinId::Pin05 => port.registers.otyper.modify(OTYPER::OT5::CLEAR),
            PinId::Pin06 => port.registers.otyper.modify(OTYPER::OT6::CLEAR),
            PinId::Pin07 => port.registers.otyper.modify(OTYPER::OT7::CLEAR),
            PinId::Pin08 => port.registers.otyper.modify(OTYPER::OT8::CLEAR),
            PinId::Pin09 => port.registers.otyper.modify(OTYPER::OT9::CLEAR),
            PinId::Pin10 => port.registers.otyper.modify(OTYPER::OT10::CLEAR),
            PinId::Pin11 => port.registers.otyper.modify(OTYPER::OT11::CLEAR),
            PinId::Pin12 => port.registers.otyper.modify(OTYPER::OT12::CLEAR),
            PinId::Pin13 => port.registers.otyper.modify(OTYPER::OT13::CLEAR),
            PinId::Pin14 => port.registers.otyper.modify(OTYPER::OT14::CLEAR),
            PinId::Pin15 => port.registers.otyper.modify(OTYPER::OT15::CLEAR),
        }
    }

    pub fn set_speed(&self) {
        let port = self.ports_ref.unwrap_or_panic(); // Unwrap fail =

        match self.pinid {
            PinId::Pin00 => port.registers.ospeedr.modify(OSPEEDR::OSPEEDR0.val(0b11)),
            PinId::Pin01 => port.registers.ospeedr.modify(OSPEEDR::OSPEEDR1.val(0b11)),
            PinId::Pin02 => port.registers.ospeedr.modify(OSPEEDR::OSPEEDR2.val(0b11)),
            PinId::Pin03 => port.registers.ospeedr.modify(OSPEEDR::OSPEEDR3.val(0b11)),
            PinId::Pin04 => port.registers.ospeedr.modify(OSPEEDR::OSPEEDR4.val(0b11)),
            PinId::Pin05 => port.registers.ospeedr.modify(OSPEEDR::OSPEEDR5.val(0b11)),
            PinId::Pin06 => port.registers.ospeedr.modify(OSPEEDR::OSPEEDR6.val(0b11)),
            PinId::Pin07 => port.registers.ospeedr.modify(OSPEEDR::OSPEEDR7.val(0b11)),
            PinId::Pin08 => port.registers.ospeedr.modify(OSPEEDR::OSPEEDR8.val(0b11)),
            PinId::Pin09 => port.registers.ospeedr.modify(OSPEEDR::OSPEEDR9.val(0b11)),
            PinId::Pin10 => port.registers.ospeedr.modify(OSPEEDR::OSPEEDR10.val(0b11)),
            PinId::Pin11 => port.registers.ospeedr.modify(OSPEEDR::OSPEEDR11.val(0b11)),
            PinId::Pin12 => port.registers.ospeedr.modify(OSPEEDR::OSPEEDR12.val(0b11)),
            PinId::Pin13 => port.registers.ospeedr.modify(OSPEEDR::OSPEEDR13.val(0b11)),
            PinId::Pin14 => port.registers.ospeedr.modify(OSPEEDR::OSPEEDR14.val(0b11)),
            PinId::Pin15 => port.registers.ospeedr.modify(OSPEEDR::OSPEEDR15.val(0b11)),
        }
    }

    pub fn set_mode_output_opendrain(&self) {
        let port = self.ports_ref.unwrap_or_panic(); // Unwrap fail =

        match self.pinid {
            PinId::Pin00 => port.registers.otyper.modify(OTYPER::OT0::SET),
            PinId::Pin01 => port.registers.otyper.modify(OTYPER::OT1::SET),
            PinId::Pin02 => port.registers.otyper.modify(OTYPER::OT2::SET),
            PinId::Pin03 => port.registers.otyper.modify(OTYPER::OT3::SET),
            PinId::Pin04 => port.registers.otyper.modify(OTYPER::OT4::SET),
            PinId::Pin05 => port.registers.otyper.modify(OTYPER::OT5::SET),
            PinId::Pin06 => port.registers.otyper.modify(OTYPER::OT6::SET),
            PinId::Pin07 => port.registers.otyper.modify(OTYPER::OT7::SET),
            PinId::Pin08 => port.registers.otyper.modify(OTYPER::OT8::SET),
            PinId::Pin09 => port.registers.otyper.modify(OTYPER::OT9::SET),
            PinId::Pin10 => port.registers.otyper.modify(OTYPER::OT10::SET),
            PinId::Pin11 => port.registers.otyper.modify(OTYPER::OT11::SET),
            PinId::Pin12 => port.registers.otyper.modify(OTYPER::OT12::SET),
            PinId::Pin13 => port.registers.otyper.modify(OTYPER::OT13::SET),
            PinId::Pin14 => port.registers.otyper.modify(OTYPER::OT14::SET),
            PinId::Pin15 => port.registers.otyper.modify(OTYPER::OT15::SET),
        }
    }

    fn get_pullup_pulldown(&self) -> PullUpPullDown {
        let port = self.ports_ref.unwrap_or_panic(); // Unwrap fail =

        let val = match self.pinid {
            PinId::Pin00 => port.registers.pupdr.read(PUPDR::PUPDR0),
            PinId::Pin01 => port.registers.pupdr.read(PUPDR::PUPDR1),
            PinId::Pin02 => port.registers.pupdr.read(PUPDR::PUPDR2),
            PinId::Pin03 => port.registers.pupdr.read(PUPDR::PUPDR3),
            PinId::Pin04 => port.registers.pupdr.read(PUPDR::PUPDR4),
            PinId::Pin05 => port.registers.pupdr.read(PUPDR::PUPDR5),
            PinId::Pin06 => port.registers.pupdr.read(PUPDR::PUPDR6),
            PinId::Pin07 => port.registers.pupdr.read(PUPDR::PUPDR7),
            PinId::Pin08 => port.registers.pupdr.read(PUPDR::PUPDR8),
            PinId::Pin09 => port.registers.pupdr.read(PUPDR::PUPDR9),
            PinId::Pin10 => port.registers.pupdr.read(PUPDR::PUPDR10),
            PinId::Pin11 => port.registers.pupdr.read(PUPDR::PUPDR11),
            PinId::Pin12 => port.registers.pupdr.read(PUPDR::PUPDR12),
            PinId::Pin13 => port.registers.pupdr.read(PUPDR::PUPDR13),
            PinId::Pin14 => port.registers.pupdr.read(PUPDR::PUPDR14),
            PinId::Pin15 => port.registers.pupdr.read(PUPDR::PUPDR15),
        };

        PullUpPullDown::from_u32(val).unwrap_or(PullUpPullDown::NoPullUpPullDown)
    }

    fn set_pullup_pulldown(&self, pupd: PullUpPullDown) {
        let port = self.ports_ref.unwrap_or_panic(); // Unwrap fail =

        match self.pinid {
            PinId::Pin00 => port.registers.pupdr.modify(PUPDR::PUPDR0.val(pupd as u32)),
            PinId::Pin01 => port.registers.pupdr.modify(PUPDR::PUPDR1.val(pupd as u32)),
            PinId::Pin02 => port.registers.pupdr.modify(PUPDR::PUPDR2.val(pupd as u32)),
            PinId::Pin03 => port.registers.pupdr.modify(PUPDR::PUPDR3.val(pupd as u32)),
            PinId::Pin04 => port.registers.pupdr.modify(PUPDR::PUPDR4.val(pupd as u32)),
            PinId::Pin05 => port.registers.pupdr.modify(PUPDR::PUPDR5.val(pupd as u32)),
            PinId::Pin06 => port.registers.pupdr.modify(PUPDR::PUPDR6.val(pupd as u32)),
            PinId::Pin07 => port.registers.pupdr.modify(PUPDR::PUPDR7.val(pupd as u32)),
            PinId::Pin08 => port.registers.pupdr.modify(PUPDR::PUPDR8.val(pupd as u32)),
            PinId::Pin09 => port.registers.pupdr.modify(PUPDR::PUPDR9.val(pupd as u32)),
            PinId::Pin10 => port.registers.pupdr.modify(PUPDR::PUPDR10.val(pupd as u32)),
            PinId::Pin11 => port.registers.pupdr.modify(PUPDR::PUPDR11.val(pupd as u32)),
            PinId::Pin12 => port.registers.pupdr.modify(PUPDR::PUPDR12.val(pupd as u32)),
            PinId::Pin13 => port.registers.pupdr.modify(PUPDR::PUPDR13.val(pupd as u32)),
            PinId::Pin14 => port.registers.pupdr.modify(PUPDR::PUPDR14.val(pupd as u32)),
            PinId::Pin15 => port.registers.pupdr.modify(PUPDR::PUPDR15.val(pupd as u32)),
        }
    }

    fn set_output_high(&self) {
        let port = self.ports_ref.unwrap_or_panic(); // Unwrap fail =

        match self.pinid {
            PinId::Pin00 => port.registers.bsrr.write(BSRR::BS0::SET),
            PinId::Pin01 => port.registers.bsrr.write(BSRR::BS1::SET),
            PinId::Pin02 => port.registers.bsrr.write(BSRR::BS2::SET),
            PinId::Pin03 => port.registers.bsrr.write(BSRR::BS3::SET),
            PinId::Pin04 => port.registers.bsrr.write(BSRR::BS4::SET),
            PinId::Pin05 => port.registers.bsrr.write(BSRR::BS5::SET),
            PinId::Pin06 => port.registers.bsrr.write(BSRR::BS6::SET),
            PinId::Pin07 => port.registers.bsrr.write(BSRR::BS7::SET),
            PinId::Pin08 => port.registers.bsrr.write(BSRR::BS8::SET),
            PinId::Pin09 => port.registers.bsrr.write(BSRR::BS9::SET),
            PinId::Pin10 => port.registers.bsrr.write(BSRR::BS10::SET),
            PinId::Pin11 => port.registers.bsrr.write(BSRR::BS11::SET),
            PinId::Pin12 => port.registers.bsrr.write(BSRR::BS12::SET),
            PinId::Pin13 => port.registers.bsrr.write(BSRR::BS13::SET),
            PinId::Pin14 => port.registers.bsrr.write(BSRR::BS14::SET),
            PinId::Pin15 => port.registers.bsrr.write(BSRR::BS15::SET),
        }
    }

    fn set_output_low(&self) {
        let port = self.ports_ref.unwrap_or_panic(); // Unwrap fail =

        match self.pinid {
            PinId::Pin00 => port.registers.bsrr.write(BSRR::BR0::SET),
            PinId::Pin01 => port.registers.bsrr.write(BSRR::BR1::SET),
            PinId::Pin02 => port.registers.bsrr.write(BSRR::BR2::SET),
            PinId::Pin03 => port.registers.bsrr.write(BSRR::BR3::SET),
            PinId::Pin04 => port.registers.bsrr.write(BSRR::BR4::SET),
            PinId::Pin05 => port.registers.bsrr.write(BSRR::BR5::SET),
            PinId::Pin06 => port.registers.bsrr.write(BSRR::BR6::SET),
            PinId::Pin07 => port.registers.bsrr.write(BSRR::BR7::SET),
            PinId::Pin08 => port.registers.bsrr.write(BSRR::BR8::SET),
            PinId::Pin09 => port.registers.bsrr.write(BSRR::BR9::SET),
            PinId::Pin10 => port.registers.bsrr.write(BSRR::BR10::SET),
            PinId::Pin11 => port.registers.bsrr.write(BSRR::BR11::SET),
            PinId::Pin12 => port.registers.bsrr.write(BSRR::BR12::SET),
            PinId::Pin13 => port.registers.bsrr.write(BSRR::BR13::SET),
            PinId::Pin14 => port.registers.bsrr.write(BSRR::BR14::SET),
            PinId::Pin15 => port.registers.bsrr.write(BSRR::BR15::SET),
        }
    }

    fn is_output_high(&self) -> bool {
        let port = self.ports_ref.unwrap_or_panic(); // Unwrap fail =

        match self.pinid {
            PinId::Pin00 => port.registers.odr.is_set(ODR::ODR0),
            PinId::Pin01 => port.registers.odr.is_set(ODR::ODR1),
            PinId::Pin02 => port.registers.odr.is_set(ODR::ODR2),
            PinId::Pin03 => port.registers.odr.is_set(ODR::ODR3),
            PinId::Pin04 => port.registers.odr.is_set(ODR::ODR4),
            PinId::Pin05 => port.registers.odr.is_set(ODR::ODR5),
            PinId::Pin06 => port.registers.odr.is_set(ODR::ODR6),
            PinId::Pin07 => port.registers.odr.is_set(ODR::ODR7),
            PinId::Pin08 => port.registers.odr.is_set(ODR::ODR8),
            PinId::Pin09 => port.registers.odr.is_set(ODR::ODR9),
            PinId::Pin10 => port.registers.odr.is_set(ODR::ODR10),
            PinId::Pin11 => port.registers.odr.is_set(ODR::ODR11),
            PinId::Pin12 => port.registers.odr.is_set(ODR::ODR12),
            PinId::Pin13 => port.registers.odr.is_set(ODR::ODR13),
            PinId::Pin14 => port.registers.odr.is_set(ODR::ODR14),
            PinId::Pin15 => port.registers.odr.is_set(ODR::ODR15),
        }
    }

    fn toggle_output(&self) -> bool {
        if self.is_output_high() {
            self.set_output_low();
            false
        } else {
            self.set_output_high();
            true
        }
    }

    fn read_input(&self) -> bool {
        let port = self.ports_ref.unwrap_or_panic(); // Unwrap fail =

        match self.pinid {
            PinId::Pin00 => port.registers.idr.is_set(IDR::IDR0),
            PinId::Pin01 => port.registers.idr.is_set(IDR::IDR1),
            PinId::Pin02 => port.registers.idr.is_set(IDR::IDR2),
            PinId::Pin03 => port.registers.idr.is_set(IDR::IDR3),
            PinId::Pin04 => port.registers.idr.is_set(IDR::IDR4),
            PinId::Pin05 => port.registers.idr.is_set(IDR::IDR5),
            PinId::Pin06 => port.registers.idr.is_set(IDR::IDR6),
            PinId::Pin07 => port.registers.idr.is_set(IDR::IDR7),
            PinId::Pin08 => port.registers.idr.is_set(IDR::IDR8),
            PinId::Pin09 => port.registers.idr.is_set(IDR::IDR9),
            PinId::Pin10 => port.registers.idr.is_set(IDR::IDR10),
            PinId::Pin11 => port.registers.idr.is_set(IDR::IDR11),
            PinId::Pin12 => port.registers.idr.is_set(IDR::IDR12),
            PinId::Pin13 => port.registers.idr.is_set(IDR::IDR13),
            PinId::Pin14 => port.registers.idr.is_set(IDR::IDR14),
            PinId::Pin15 => port.registers.idr.is_set(IDR::IDR15),
        }
    }
}

impl gpio::Configure for GpioPin<'_> {
    /// Output mode default is push-pull
    fn make_output(&self) -> gpio::Configuration {
        self.set_mode(Mode::GeneralPurposeOutputMode);
        self.set_mode_output_pushpull();
        gpio::Configuration::Output
    }

    /// Input mode default is no internal pull-up, no pull-down (i.e.,
    /// floating). Also upon setting the mode as input, the internal schmitt
    /// trigger is automatically activated. Schmitt trigger is deactivated in
    /// AnalogMode.
    fn make_input(&self) -> gpio::Configuration {
        self.set_mode(Mode::Input);
        gpio::Configuration::Input
    }

    /// According to AN4899, Section 6.1, setting to AnalogMode, disables
    /// internal schmitt trigger. We do not disable clock to the GPIO port,
    /// because there could be other pins active on the port.
    fn deactivate_to_low_power(&self) {
        self.set_mode(Mode::AnalogMode);
    }

    fn disable_output(&self) -> gpio::Configuration {
        self.set_mode(Mode::AnalogMode);
        gpio::Configuration::LowPower
    }

    fn disable_input(&self) -> gpio::Configuration {
        self.set_mode(Mode::AnalogMode);
        gpio::Configuration::LowPower
    }

    fn set_floating_state(&self, mode: gpio::FloatingState) {
        match mode {
            gpio::FloatingState::PullUp => self.set_pullup_pulldown(PullUpPullDown::PullUp),
            gpio::FloatingState::PullDown => self.set_pullup_pulldown(PullUpPullDown::PullDown),
            gpio::FloatingState::PullNone => {
                self.set_pullup_pulldown(PullUpPullDown::NoPullUpPullDown)
            }
        }
    }

    fn floating_state(&self) -> gpio::FloatingState {
        match self.get_pullup_pulldown() {
            PullUpPullDown::PullUp => gpio::FloatingState::PullUp,
            PullUpPullDown::PullDown => gpio::FloatingState::PullDown,
            PullUpPullDown::NoPullUpPullDown => gpio::FloatingState::PullNone,
        }
    }

    fn configuration(&self) -> gpio::Configuration {
        match self.get_mode() {
            Mode::Input => gpio::Configuration::Input,
            Mode::GeneralPurposeOutputMode => gpio::Configuration::Output,
            Mode::AnalogMode => gpio::Configuration::LowPower,
            Mode::AlternateFunctionMode => gpio::Configuration::Function,
        }
    }

    fn is_input(&self) -> bool {
        self.get_mode() == Mode::Input
    }

    fn is_output(&self) -> bool {
        self.get_mode() == Mode::GeneralPurposeOutputMode
    }
}

impl gpio::Input for GpioPin<'_> {
    fn read(&self) -> bool {
        self.read_input()
    }
}

impl gpio::Output for GpioPin<'_> {
    fn set(&self) {
        self.set_output_high();
    }

    fn clear(&self) {
        self.set_output_low();
    }

    fn toggle(&self) -> bool {
        self.toggle_output()
    }
}

pub struct GpioPort<'a> {
    registers: StaticRef<GpioRegisters>,
    pins: [GpioPin<'a>; 16],
    clock: PortClock<'a>,
}

impl<'a> GpioPort<'a> {
    pub const fn new(rcc: &'a rcc::Rcc, port: PortId) -> Self {
        let pins = [
            GpioPin::new(PinId::Pin00),
            GpioPin::new(PinId::Pin01),
            GpioPin::new(PinId::Pin02),
            GpioPin::new(PinId::Pin03),
            GpioPin::new(PinId::Pin04),
            GpioPin::new(PinId::Pin05),
            GpioPin::new(PinId::Pin06),
            GpioPin::new(PinId::Pin07),
            GpioPin::new(PinId::Pin08),
            GpioPin::new(PinId::Pin09),
            GpioPin::new(PinId::Pin10),
            GpioPin::new(PinId::Pin11),
            GpioPin::new(PinId::Pin12),
            GpioPin::new(PinId::Pin13),
            GpioPin::new(PinId::Pin14),
            GpioPin::new(PinId::Pin15),
        ];
        let clock = match port {
            PortId::GPIOA => PortClock(rcc::PeripheralClock::new(
                rcc::PeripheralClockType::GPIOA,
                rcc,
            )),
            PortId::GPIOB => PortClock(rcc::PeripheralClock::new(
                rcc::PeripheralClockType::GPIOB,
                rcc,
            )),
            PortId::GPIOD => PortClock(rcc::PeripheralClock::new(
                rcc::PeripheralClockType::GPIOD,
                rcc,
            )),
            PortId::GPIOG => PortClock(rcc::PeripheralClock::new(
                rcc::PeripheralClockType::GPIOG,
                rcc,
            )),
            PortId::GPIOH => PortClock(rcc::PeripheralClock::new(
                rcc::PeripheralClockType::GPIOH,
                rcc,
            )),
        };
        let registers = match port {
            PortId::GPIOA => GPIOA_BASE,
            PortId::GPIOB => GPIOB_BASE,
            PortId::GPIOD => GPIOD_BASE,
            PortId::GPIOG => GPIOG_BASE,
            PortId::GPIOH => GPIOH_BASE,
        };
        Self {
            pins,
            registers,
            clock,
        }
    }

    pub fn is_enabled_clock(&self) -> bool {
        self.clock.is_enabled()
    }

    pub fn enable_clock(&self) {
        self.clock.enable();
    }

    pub fn disable_clock(&self) {
        self.clock.disable();
    }

    pub fn setup_circular_deps(&'a self) {
        self.pins.iter()
            .for_each(|p| p.set_ports_ref(self));
    }
}

impl<'a> Index<usize> for GpioPort<'a> {
    type Output = GpioPin<'a>;

    fn index(&self, index: usize) -> &GpioPin<'a> {
        &self.pins[index]
    }
}

impl<'a> IndexMut<usize> for GpioPort<'a> {
    fn index_mut(&mut self, index: usize) -> &mut GpioPin<'a> {
        &mut self.pins[index]
    }
}

struct PortClock<'a>(rcc::PeripheralClock<'a>);

impl ClockInterface for PortClock<'_> {
    fn is_enabled(&self) -> bool {
        self.0.is_enabled()
    }

    fn enable(&self) {
        self.0.enable();
    }

    fn disable(&self) {
        self.0.disable();
    }
}

register_structs! {
    /// GPIOA
    GpioRegisters {
        /// GPIO port mode register
        (0x000 => moder: ReadWrite<u32, MODER::Register>),
        /// GPIO port output type register
        (0x004 => otyper: ReadWrite<u32, OTYPER::Register>),
        /// GPIO port output speed register
        (0x008 => ospeedr: ReadWrite<u32, OSPEEDR::Register>),
        /// GPIO port pull-up/pull-down register
        (0x00C => pupdr: ReadWrite<u32, PUPDR::Register>),
        /// GPIO port input data register
        (0x010 => idr: ReadOnly<u32, IDR::Register>),
        /// GPIO port output data register
        (0x014 => odr: ReadWrite<u32, ODR::Register>),
        /// GPIO port bit set/reset register
        (0x018 => bsrr: WriteOnly<u32, BSRR::Register>),
        /// This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits [15:0] is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR[15:0] must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).
        (0x01C => lckr: ReadWrite<u32, LCKR::Register>),
        /// GPIO alternate function low register
        (0x020 => afrl: ReadWrite<u32, AFRL::Register>),
        /// GPIO alternate function high register
        (0x024 => afrh: ReadWrite<u32, AFRH::Register>),
        /// GPIO port bit reset register
        (0x028 => brr: WriteOnly<u32, BRR::Register>),
        (0x02C => _reserved0),
        /// For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
        (0x3C8 => hwcfgr10: ReadOnly<u32, HWCFGR10::Register>),
        /// For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
        (0x3CC => hwcfgr9: ReadOnly<u32>),
        /// For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
        (0x3D0 => hwcfgr8: ReadOnly<u32, HWCFGR8::Register>),
        /// GPIO hardware configuration register 7
        (0x3D4 => hwcfgr7: ReadOnly<u32, HWCFGR7::Register>),
        /// GPIO hardware configuration register 6
        (0x3D8 => hwcfgr6: ReadOnly<u32>),
        /// GPIO hardware configuration register 5
        (0x3DC => hwcfgr5: ReadOnly<u32>),
        /// GPIO hardware configuration register 4
        (0x3E0 => hwcfgr4: ReadOnly<u32>),
        /// GPIO hardware configuration register 3
        (0x3E4 => hwcfgr3: ReadOnly<u32, HWCFGR3::Register>),
        /// GPIO hardware configuration register 2
        (0x3E8 => hwcfgr2: ReadOnly<u32>),
        /// GPIO hardware configuration register 1
        (0x3EC => hwcfgr1: ReadOnly<u32>),
        /// GPIO hardware configuration register 0
        (0x3F0 => hwcfgr0: ReadOnly<u32>),
        /// GPIO version register
        (0x3F4 => verr: ReadOnly<u32, VERR::Register>),
        /// GPIO identification register
        (0x3F8 => ipidr: ReadOnly<u32>),
        /// GPIO size identification register
        (0x3FC => sidr: ReadOnly<u32>),
        (0x400 => @END),
    }
}
register_bitfields![u32,
MODER [
    /// MODER0
    MODER0 OFFSET(0) NUMBITS(2) [],
    /// MODER1
    MODER1 OFFSET(2) NUMBITS(2) [],
    /// MODER2
    MODER2 OFFSET(4) NUMBITS(2) [],
    /// MODER3
    MODER3 OFFSET(6) NUMBITS(2) [],
    /// MODER4
    MODER4 OFFSET(8) NUMBITS(2) [],
    /// MODER5
    MODER5 OFFSET(10) NUMBITS(2) [],
    /// MODER6
    MODER6 OFFSET(12) NUMBITS(2) [],
    /// MODER7
    MODER7 OFFSET(14) NUMBITS(2) [],
    /// MODER8
    MODER8 OFFSET(16) NUMBITS(2) [],
    /// MODER9
    MODER9 OFFSET(18) NUMBITS(2) [],
    /// MODER10
    MODER10 OFFSET(20) NUMBITS(2) [],
    /// MODER11
    MODER11 OFFSET(22) NUMBITS(2) [],
    /// MODER12
    MODER12 OFFSET(24) NUMBITS(2) [],
    /// MODER13
    MODER13 OFFSET(26) NUMBITS(2) [],
    /// MODER14
    MODER14 OFFSET(28) NUMBITS(2) [],
    /// MODER15
    MODER15 OFFSET(30) NUMBITS(2) []
],
OTYPER [
    /// OT0
    OT0 OFFSET(0) NUMBITS(1) [],
    /// OT1
    OT1 OFFSET(1) NUMBITS(1) [],
    /// OT2
    OT2 OFFSET(2) NUMBITS(1) [],
    /// OT3
    OT3 OFFSET(3) NUMBITS(1) [],
    /// OT4
    OT4 OFFSET(4) NUMBITS(1) [],
    /// OT5
    OT5 OFFSET(5) NUMBITS(1) [],
    /// OT6
    OT6 OFFSET(6) NUMBITS(1) [],
    /// OT7
    OT7 OFFSET(7) NUMBITS(1) [],
    /// OT8
    OT8 OFFSET(8) NUMBITS(1) [],
    /// OT9
    OT9 OFFSET(9) NUMBITS(1) [],
    /// OT10
    OT10 OFFSET(10) NUMBITS(1) [],
    /// OT11
    OT11 OFFSET(11) NUMBITS(1) [],
    /// OT12
    OT12 OFFSET(12) NUMBITS(1) [],
    /// OT13
    OT13 OFFSET(13) NUMBITS(1) [],
    /// OT14
    OT14 OFFSET(14) NUMBITS(1) [],
    /// OT15
    OT15 OFFSET(15) NUMBITS(1) []
],
OSPEEDR [
    /// OSPEEDR0
    OSPEEDR0 OFFSET(0) NUMBITS(2) [],
    /// OSPEEDR1
    OSPEEDR1 OFFSET(2) NUMBITS(2) [],
    /// OSPEEDR2
    OSPEEDR2 OFFSET(4) NUMBITS(2) [],
    /// OSPEEDR3
    OSPEEDR3 OFFSET(6) NUMBITS(2) [],
    /// OSPEEDR4
    OSPEEDR4 OFFSET(8) NUMBITS(2) [],
    /// OSPEEDR5
    OSPEEDR5 OFFSET(10) NUMBITS(2) [],
    /// OSPEEDR6
    OSPEEDR6 OFFSET(12) NUMBITS(2) [],
    /// OSPEEDR7
    OSPEEDR7 OFFSET(14) NUMBITS(2) [],
    /// OSPEEDR8
    OSPEEDR8 OFFSET(16) NUMBITS(2) [],
    /// OSPEEDR9
    OSPEEDR9 OFFSET(18) NUMBITS(2) [],
    /// OSPEEDR10
    OSPEEDR10 OFFSET(20) NUMBITS(2) [],
    /// OSPEEDR11
    OSPEEDR11 OFFSET(22) NUMBITS(2) [],
    /// OSPEEDR12
    OSPEEDR12 OFFSET(24) NUMBITS(2) [],
    /// OSPEEDR13
    OSPEEDR13 OFFSET(26) NUMBITS(2) [],
    /// OSPEEDR14
    OSPEEDR14 OFFSET(28) NUMBITS(2) [],
    /// OSPEEDR15
    OSPEEDR15 OFFSET(30) NUMBITS(2) []
],
PUPDR [
    /// PUPDR0
    PUPDR0 OFFSET(0) NUMBITS(2) [],
    /// PUPDR1
    PUPDR1 OFFSET(2) NUMBITS(2) [],
    /// PUPDR2
    PUPDR2 OFFSET(4) NUMBITS(2) [],
    /// PUPDR3
    PUPDR3 OFFSET(6) NUMBITS(2) [],
    /// PUPDR4
    PUPDR4 OFFSET(8) NUMBITS(2) [],
    /// PUPDR5
    PUPDR5 OFFSET(10) NUMBITS(2) [],
    /// PUPDR6
    PUPDR6 OFFSET(12) NUMBITS(2) [],
    /// PUPDR7
    PUPDR7 OFFSET(14) NUMBITS(2) [],
    /// PUPDR8
    PUPDR8 OFFSET(16) NUMBITS(2) [],
    /// PUPDR9
    PUPDR9 OFFSET(18) NUMBITS(2) [],
    /// PUPDR10
    PUPDR10 OFFSET(20) NUMBITS(2) [],
    /// PUPDR11
    PUPDR11 OFFSET(22) NUMBITS(2) [],
    /// PUPDR12
    PUPDR12 OFFSET(24) NUMBITS(2) [],
    /// PUPDR13
    PUPDR13 OFFSET(26) NUMBITS(2) [],
    /// PUPDR14
    PUPDR14 OFFSET(28) NUMBITS(2) [],
    /// PUPDR15
    PUPDR15 OFFSET(30) NUMBITS(2) []
],
IDR [
    /// IDR0
    IDR0 OFFSET(0) NUMBITS(1) [],
    /// IDR1
    IDR1 OFFSET(1) NUMBITS(1) [],
    /// IDR2
    IDR2 OFFSET(2) NUMBITS(1) [],
    /// IDR3
    IDR3 OFFSET(3) NUMBITS(1) [],
    /// IDR4
    IDR4 OFFSET(4) NUMBITS(1) [],
    /// IDR5
    IDR5 OFFSET(5) NUMBITS(1) [],
    /// IDR6
    IDR6 OFFSET(6) NUMBITS(1) [],
    /// IDR7
    IDR7 OFFSET(7) NUMBITS(1) [],
    /// IDR8
    IDR8 OFFSET(8) NUMBITS(1) [],
    /// IDR9
    IDR9 OFFSET(9) NUMBITS(1) [],
    /// IDR10
    IDR10 OFFSET(10) NUMBITS(1) [],
    /// IDR11
    IDR11 OFFSET(11) NUMBITS(1) [],
    /// IDR12
    IDR12 OFFSET(12) NUMBITS(1) [],
    /// IDR13
    IDR13 OFFSET(13) NUMBITS(1) [],
    /// IDR14
    IDR14 OFFSET(14) NUMBITS(1) [],
    /// IDR15
    IDR15 OFFSET(15) NUMBITS(1) []
],
ODR [
    /// ODR0
    ODR0 OFFSET(0) NUMBITS(1) [],
    /// ODR1
    ODR1 OFFSET(1) NUMBITS(1) [],
    /// ODR2
    ODR2 OFFSET(2) NUMBITS(1) [],
    /// ODR3
    ODR3 OFFSET(3) NUMBITS(1) [],
    /// ODR4
    ODR4 OFFSET(4) NUMBITS(1) [],
    /// ODR5
    ODR5 OFFSET(5) NUMBITS(1) [],
    /// ODR6
    ODR6 OFFSET(6) NUMBITS(1) [],
    /// ODR7
    ODR7 OFFSET(7) NUMBITS(1) [],
    /// ODR8
    ODR8 OFFSET(8) NUMBITS(1) [],
    /// ODR9
    ODR9 OFFSET(9) NUMBITS(1) [],
    /// ODR10
    ODR10 OFFSET(10) NUMBITS(1) [],
    /// ODR11
    ODR11 OFFSET(11) NUMBITS(1) [],
    /// ODR12
    ODR12 OFFSET(12) NUMBITS(1) [],
    /// ODR13
    ODR13 OFFSET(13) NUMBITS(1) [],
    /// ODR14
    ODR14 OFFSET(14) NUMBITS(1) [],
    /// ODR15
    ODR15 OFFSET(15) NUMBITS(1) []
],
BSRR [
    /// BS0
    BS0 OFFSET(0) NUMBITS(1) [],
    /// BS1
    BS1 OFFSET(1) NUMBITS(1) [],
    /// BS2
    BS2 OFFSET(2) NUMBITS(1) [],
    /// BS3
    BS3 OFFSET(3) NUMBITS(1) [],
    /// BS4
    BS4 OFFSET(4) NUMBITS(1) [],
    /// BS5
    BS5 OFFSET(5) NUMBITS(1) [],
    /// BS6
    BS6 OFFSET(6) NUMBITS(1) [],
    /// BS7
    BS7 OFFSET(7) NUMBITS(1) [],
    /// BS8
    BS8 OFFSET(8) NUMBITS(1) [],
    /// BS9
    BS9 OFFSET(9) NUMBITS(1) [],
    /// BS10
    BS10 OFFSET(10) NUMBITS(1) [],
    /// BS11
    BS11 OFFSET(11) NUMBITS(1) [],
    /// BS12
    BS12 OFFSET(12) NUMBITS(1) [],
    /// BS13
    BS13 OFFSET(13) NUMBITS(1) [],
    /// BS14
    BS14 OFFSET(14) NUMBITS(1) [],
    /// BS15
    BS15 OFFSET(15) NUMBITS(1) [],
    /// BR0
    BR0 OFFSET(16) NUMBITS(1) [],
    /// BR1
    BR1 OFFSET(17) NUMBITS(1) [],
    /// BR2
    BR2 OFFSET(18) NUMBITS(1) [],
    /// BR3
    BR3 OFFSET(19) NUMBITS(1) [],
    /// BR4
    BR4 OFFSET(20) NUMBITS(1) [],
    /// BR5
    BR5 OFFSET(21) NUMBITS(1) [],
    /// BR6
    BR6 OFFSET(22) NUMBITS(1) [],
    /// BR7
    BR7 OFFSET(23) NUMBITS(1) [],
    /// BR8
    BR8 OFFSET(24) NUMBITS(1) [],
    /// BR9
    BR9 OFFSET(25) NUMBITS(1) [],
    /// BR10
    BR10 OFFSET(26) NUMBITS(1) [],
    /// BR11
    BR11 OFFSET(27) NUMBITS(1) [],
    /// BR12
    BR12 OFFSET(28) NUMBITS(1) [],
    /// BR13
    BR13 OFFSET(29) NUMBITS(1) [],
    /// BR14
    BR14 OFFSET(30) NUMBITS(1) [],
    /// BR15
    BR15 OFFSET(31) NUMBITS(1) []
],
LCKR [
    /// LCK0
    LCK0 OFFSET(0) NUMBITS(1) [],
    /// LCK1
    LCK1 OFFSET(1) NUMBITS(1) [],
    /// LCK2
    LCK2 OFFSET(2) NUMBITS(1) [],
    /// LCK3
    LCK3 OFFSET(3) NUMBITS(1) [],
    /// LCK4
    LCK4 OFFSET(4) NUMBITS(1) [],
    /// LCK5
    LCK5 OFFSET(5) NUMBITS(1) [],
    /// LCK6
    LCK6 OFFSET(6) NUMBITS(1) [],
    /// LCK7
    LCK7 OFFSET(7) NUMBITS(1) [],
    /// LCK8
    LCK8 OFFSET(8) NUMBITS(1) [],
    /// LCK9
    LCK9 OFFSET(9) NUMBITS(1) [],
    /// LCK10
    LCK10 OFFSET(10) NUMBITS(1) [],
    /// LCK11
    LCK11 OFFSET(11) NUMBITS(1) [],
    /// LCK12
    LCK12 OFFSET(12) NUMBITS(1) [],
    /// LCK13
    LCK13 OFFSET(13) NUMBITS(1) [],
    /// LCK14
    LCK14 OFFSET(14) NUMBITS(1) [],
    /// LCK15
    LCK15 OFFSET(15) NUMBITS(1) [],
    /// LCKK
    LCKK OFFSET(16) NUMBITS(1) []
],
AFRL [
    /// AFR0
    AFR0 OFFSET(0) NUMBITS(4) [],
    /// AFR1
    AFR1 OFFSET(4) NUMBITS(4) [],
    /// AFR2
    AFR2 OFFSET(8) NUMBITS(4) [],
    /// AFR3
    AFR3 OFFSET(12) NUMBITS(4) [],
    /// AFR4
    AFR4 OFFSET(16) NUMBITS(4) [],
    /// AFR5
    AFR5 OFFSET(20) NUMBITS(4) [],
    /// AFR6
    AFR6 OFFSET(24) NUMBITS(4) [],
    /// AFR7
    AFR7 OFFSET(28) NUMBITS(4) []
],
AFRH [
    /// AFR8
    AFR8 OFFSET(0) NUMBITS(4) [],
    /// AFR9
    AFR9 OFFSET(4) NUMBITS(4) [],
    /// AFR10
    AFR10 OFFSET(8) NUMBITS(4) [],
    /// AFR11
    AFR11 OFFSET(12) NUMBITS(4) [],
    /// AFR12
    AFR12 OFFSET(16) NUMBITS(4) [],
    /// AFR13
    AFR13 OFFSET(20) NUMBITS(4) [],
    /// AFR14
    AFR14 OFFSET(24) NUMBITS(4) [],
    /// AFR15
    AFR15 OFFSET(28) NUMBITS(4) []
],
BRR [
    /// BR0
    BR0 OFFSET(0) NUMBITS(1) [],
    /// BR1
    BR1 OFFSET(1) NUMBITS(1) [],
    /// BR2
    BR2 OFFSET(2) NUMBITS(1) [],
    /// BR3
    BR3 OFFSET(3) NUMBITS(1) [],
    /// BR4
    BR4 OFFSET(4) NUMBITS(1) [],
    /// BR5
    BR5 OFFSET(5) NUMBITS(1) [],
    /// BR6
    BR6 OFFSET(6) NUMBITS(1) [],
    /// BR7
    BR7 OFFSET(7) NUMBITS(1) [],
    /// BR8
    BR8 OFFSET(8) NUMBITS(1) [],
    /// BR9
    BR9 OFFSET(9) NUMBITS(1) [],
    /// BR10
    BR10 OFFSET(10) NUMBITS(1) [],
    /// BR11
    BR11 OFFSET(11) NUMBITS(1) [],
    /// BR12
    BR12 OFFSET(12) NUMBITS(1) [],
    /// BR13
    BR13 OFFSET(13) NUMBITS(1) [],
    /// BR14
    BR14 OFFSET(14) NUMBITS(1) [],
    /// BR15
    BR15 OFFSET(15) NUMBITS(1) []
],
HWCFGR10 [
    /// AHB_IOP
    AHB_IOP OFFSET(0) NUMBITS(4) [],
    /// AF_SIZE
    AF_SIZE OFFSET(4) NUMBITS(4) [],
    /// SPEED_CFG
    SPEED_CFG OFFSET(8) NUMBITS(4) [],
    /// LOCK_CFG
    LOCK_CFG OFFSET(12) NUMBITS(4) [],
    /// SEC_CFG
    SEC_CFG OFFSET(16) NUMBITS(4) [],
    /// OR_CFG
    OR_CFG OFFSET(20) NUMBITS(4) []
],
HWCFGR9 [
    /// EN_IO
    EN_IO OFFSET(0) NUMBITS(16) []
],
HWCFGR8 [
    /// AF_PRIO8
    AF_PRIO8 OFFSET(0) NUMBITS(4) [],
    /// AF_PRIO9
    AF_PRIO9 OFFSET(4) NUMBITS(4) [],
    /// AF_PRIO10
    AF_PRIO10 OFFSET(8) NUMBITS(4) [],
    /// AF_PRIO11
    AF_PRIO11 OFFSET(12) NUMBITS(4) [],
    /// AF_PRIO12
    AF_PRIO12 OFFSET(16) NUMBITS(4) [],
    /// AF_PRIO13
    AF_PRIO13 OFFSET(20) NUMBITS(4) [],
    /// AF_PRIO14
    AF_PRIO14 OFFSET(24) NUMBITS(4) [],
    /// AF_PRIO15
    AF_PRIO15 OFFSET(28) NUMBITS(4) []
],
HWCFGR7 [
    /// AF_PRIO0
    AF_PRIO0 OFFSET(0) NUMBITS(4) [],
    /// AF_PRIO1
    AF_PRIO1 OFFSET(4) NUMBITS(4) [],
    /// AF_PRIO2
    AF_PRIO2 OFFSET(8) NUMBITS(4) [],
    /// AF_PRIO3
    AF_PRIO3 OFFSET(12) NUMBITS(4) [],
    /// AF_PRIO4
    AF_PRIO4 OFFSET(16) NUMBITS(4) [],
    /// AF_PRIO5
    AF_PRIO5 OFFSET(20) NUMBITS(4) [],
    /// AF_PRIO6
    AF_PRIO6 OFFSET(24) NUMBITS(4) [],
    /// AF_PRIO7
    AF_PRIO7 OFFSET(28) NUMBITS(4) []
],
HWCFGR6 [
    /// MODER_RES
    MODER_RES OFFSET(0) NUMBITS(32) []
],
HWCFGR5 [
    /// PUPDR_RES
    PUPDR_RES OFFSET(0) NUMBITS(32) []
],
HWCFGR4 [
    /// OSPEED_RES
    OSPEED_RES OFFSET(0) NUMBITS(32) []
],
HWCFGR3 [
    /// ODR_RES
    ODR_RES OFFSET(0) NUMBITS(16) [],
    /// OTYPER_RES
    OTYPER_RES OFFSET(16) NUMBITS(16) []
],
HWCFGR2 [
    /// AFRL_RES
    AFRL_RES OFFSET(0) NUMBITS(32) []
],
HWCFGR1 [
    /// AFRH_RES
    AFRH_RES OFFSET(0) NUMBITS(32) []
],
HWCFGR0 [
    /// OR_RES
    OR_RES OFFSET(0) NUMBITS(16) []
],
VERR [
    /// MINREV
    MINREV OFFSET(0) NUMBITS(4) [],
    /// MAJREV
    MAJREV OFFSET(4) NUMBITS(4) []
],
IPIDR [
    /// IPIDR
    IPIDR OFFSET(0) NUMBITS(32) []
],
SIDR [
    /// SIDR
    SIDR OFFSET(0) NUMBITS(32) []
]
];
const GPIOA_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x50002000 as *const GpioRegisters) };
const GPIOB_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x50003000 as *const GpioRegisters) };
const GPIOD_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x50005000 as *const GpioRegisters) };
const GPIOG_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x50008000 as *const GpioRegisters) };
const GPIOH_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x50009000 as *const GpioRegisters) };
