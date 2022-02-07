use cortexm4;
use cortexm4::support::atomic;
use kernel::hil::time::{
    Alarm, AlarmClient, Counter, OverflowClient, Ticks, Ticks32, Time, Freq32KHz
};
use kernel::platform::chip::ClockInterface;
use kernel::utilities::cells::OptionalCell;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, register_structs, ReadWrite, WriteOnly};
use kernel::utilities::StaticRef;
use kernel::ErrorCode;

use crate::nvic;
use crate::rcc;

pub enum TIMN {
    TIM2,
    TIM3,
    TIM4,
    TIM5,
}

pub struct Tim<'a> {
    registers: StaticRef<TimRegisters>,
    clock: TimClock<'a>,
    client: OptionalCell<&'a dyn AlarmClient>,
    irqn: u32,
}

impl<'a> Tim<'a> {
    pub const fn new(rcc: &'a rcc::Rcc, n: TIMN) -> Self {
        let registers = match n {
            TIMN::TIM2 => BASE_TIM2,
            TIMN::TIM3 => BASE_TIM3,
            TIMN::TIM4 => BASE_TIM4,
            TIMN::TIM5 => BASE_TIM5,
        };
        let clk = match n {
            TIMN::TIM2 => rcc::PeripheralClockType::TIM2,
            TIMN::TIM3 => rcc::PeripheralClockType::TIM3,
            TIMN::TIM4 => rcc::PeripheralClockType::TIM4,
            TIMN::TIM5 => rcc::PeripheralClockType::TIM5,
        };
        let irqn = match n {
            TIMN::TIM2 => nvic::TIM2,
            TIMN::TIM3 => nvic::TIM3,
            TIMN::TIM4 => nvic::TIM4,
            TIMN::TIM5 => nvic::TIM5,
        };
        
        Self {
            registers,
            clock: TimClock(rcc::PeripheralClock::new(
                clk,
                rcc,
            )),
            client: OptionalCell::empty(),
            irqn,
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

    pub fn handle_interrupt(&self) {
        self.registers.sr.modify(SR::CC1IF::CLEAR);

        self.client.map(|client| client.alarm());
    }

    // starts the timer
    pub fn start(&self) {
        self.registers.arr.set(0xFFFF - 1);
        
        self.registers.psc.set((999 - 1) as u16);
        self.registers.egr.write(EGR::UG::SET);
        self.registers.cr1.modify(CR1::CEN::SET);
    }
}

impl Time for Tim<'_> {
    type Frequency = Freq32KHz;
    type Ticks = Ticks32;

    fn now(&self) -> Ticks32 {
        Ticks32::from(self.registers.cnt.get())
    }
}

impl<'a> Counter<'a> for Tim<'a> {
    fn set_overflow_client(&self, _client: &'a dyn OverflowClient) {}

    // starts the timer
    fn start(&self) -> Result<(), ErrorCode> {
        self.start();
        Ok(())
    }

    fn stop(&self) -> Result<(), ErrorCode> {
        self.registers.cr1.modify(CR1::CEN::CLEAR);
        self.registers.sr.modify(SR::CC1IF::CLEAR);
        Ok(())
    }

    fn reset(&self) -> Result<(), ErrorCode> {
        self.registers.cnt.set(0);
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.registers.cr1.is_set(CR1::CEN)
    }
}

impl<'a> Alarm<'a> for Tim<'a> {
    fn set_alarm_client(&self, client: &'a dyn AlarmClient) {
        self.client.set(client);
    }

    fn set_alarm(&self, reference: Self::Ticks, dt: Self::Ticks) {
        let mut expire = reference.wrapping_add(dt);
        let now = self.now();
        if !now.within_range(reference, expire) {
            expire = now;
        }

        if expire.wrapping_sub(now) < self.minimum_dt() {
            expire = now.wrapping_add(self.minimum_dt());
        }

        let _ = self.disarm();
        self.registers.ccr1.set(expire.into_u32() as u16);
        self.registers.dier.modify(DIER::CC1IE::SET);
    }

    fn get_alarm(&self) -> Self::Ticks {
        Self::Ticks::from(self.registers.ccr1.get() as u32)
    }

    fn disarm(&self) -> Result<(), ErrorCode> {
        unsafe {
            atomic(|| {
                // Disable counter
                self.registers.dier.modify(DIER::CC1IE::CLEAR);
                cortexm4::nvic::Nvic::new(self.irqn).clear_pending();
            });
        }
        Ok(())
    }

    fn is_armed(&self) -> bool {
        // If counter is enabled, then CC1IE is set
        self.registers.dier.is_set(DIER::CC1IE)
    }

    fn minimum_dt(&self) -> Self::Ticks {
        Self::Ticks::from(1)
    }
}

struct TimClock<'a>(rcc::PeripheralClock<'a>);

impl ClockInterface for TimClock<'_> {
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
    /// TIM
    TimRegisters {
        /// TIM control register 1
        (0x000 => cr1: ReadWrite<u16, CR1::Register>),
        (0x002 => _reserved0),
        /// TIM control register 2
        (0x004 => cr2: ReadWrite<u32, CR2::Register>),
        /// TIM slave mode control register
        (0x008 => smcr: ReadWrite<u32, SMCR::Register>),
        /// TIM DMA/interrupt enable register
        (0x00C => dier: ReadWrite<u16, DIER::Register>),
        (0x00E => _reserved1),
        /// TIM status register
        (0x010 => sr: ReadWrite<u32, SR::Register>),
        /// TIM event generation register
        (0x014 => egr: WriteOnly<u16, EGR::Register>),
        (0x016 => _reserved2),
        /// The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
        (0x018 => ccmr1alternate2: ReadWrite<u32, CCMR1ALTERNATE2::Register>),
        /// The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
        (0x01C => ccmr2alternate18: ReadWrite<u32, CCMR2ALTERNATE18::Register>),
        /// TIM capture/compare enable register
        (0x020 => ccer: ReadWrite<u32, CCER::Register>),
        /// TIM counter
        (0x024 => cnt: ReadWrite<u32, CNT::Register>),
        /// TIM prescaler
        (0x028 => psc: ReadWrite<u16>),
        (0x02A => _reserved3),
        /// TIM auto-reload register
        (0x02C => arr: ReadWrite<u16>),
        (0x02E => _reserved4),
        /// TIM repetition counter register
        (0x030 => rcr: ReadWrite<u16>),
        (0x032 => _reserved5),
        /// TIM capture/compare register 1
        (0x034 => ccr1: ReadWrite<u16>),
        (0x036 => _reserved6),
        /// TIM capture/compare register 2
        (0x038 => ccr2: ReadWrite<u16>),
        (0x03A => _reserved7),
        /// TIM capture/compare register 3
        (0x03C => ccr3: ReadWrite<u16>),
        (0x03E => _reserved8),
        /// TIM capture/compare register 4
        (0x040 => ccr4: ReadWrite<u16>),
        (0x042 => _reserved9),
        /// As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F[3:0], BKF[3:0], AOE, BKP, BKE, OSSI, OSSR and DTG[7:0] can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
        (0x044 => bdtr: ReadWrite<u32, BDTR::Register>),
        /// TIM DMA control register
        (0x048 => dcr: ReadWrite<u16, DCR::Register>),
        (0x04A => _reserved10),
        /// TIM DMA address for full transfer
        (0x04C => dmar: ReadWrite<u32>),
        (0x050 => _reserved11),
        /// The channels 5 and 6 can only be configured in output. Output compare mode:
        (0x054 => ccmr3: ReadWrite<u32, CCMR3::Register>),
        /// TIM capture/compare register 5
        (0x058 => ccr5: ReadWrite<u32, CCR5::Register>),
        /// TIM capture/compare register 6
        (0x05C => ccr6: ReadWrite<u16>),
        (0x05E => @END),
    }
}
register_bitfields![u16,
CR1 [
    /// CEN
    CEN OFFSET(0) NUMBITS(1) [],
    /// UDIS
    UDIS OFFSET(1) NUMBITS(1) [],
    /// URS
    URS OFFSET(2) NUMBITS(1) [],
    /// OPM
    OPM OFFSET(3) NUMBITS(1) [],
    /// DIR
    DIR OFFSET(4) NUMBITS(1) [],
    /// CMS
    CMS OFFSET(5) NUMBITS(2) [],
    /// ARPE
    ARPE OFFSET(7) NUMBITS(1) [],
    /// CKD
    CKD OFFSET(8) NUMBITS(2) [],
    /// UIFREMAP
    UIFREMAP OFFSET(11) NUMBITS(1) []
],
DIER [
    /// UIE
    UIE OFFSET(0) NUMBITS(1) [],
    /// CC1IE
    CC1IE OFFSET(1) NUMBITS(1) [],
    /// CC2IE
    CC2IE OFFSET(2) NUMBITS(1) [],
    /// CC3IE
    CC3IE OFFSET(3) NUMBITS(1) [],
    /// CC4IE
    CC4IE OFFSET(4) NUMBITS(1) [],
    /// COMIE
    COMIE OFFSET(5) NUMBITS(1) [],
    /// TIE
    TIE OFFSET(6) NUMBITS(1) [],
    /// BIE
    BIE OFFSET(7) NUMBITS(1) [],
    /// UDE
    UDE OFFSET(8) NUMBITS(1) [],
    /// CC1DE
    CC1DE OFFSET(9) NUMBITS(1) [],
    /// CC2DE
    CC2DE OFFSET(10) NUMBITS(1) [],
    /// CC3DE
    CC3DE OFFSET(11) NUMBITS(1) [],
    /// CC4DE
    CC4DE OFFSET(12) NUMBITS(1) [],
    /// COMDE
    COMDE OFFSET(13) NUMBITS(1) [],
    /// TDE
    TDE OFFSET(14) NUMBITS(1) []
],
EGR [
    /// UG
    UG OFFSET(0) NUMBITS(1) [],
    /// CC1G
    CC1G OFFSET(1) NUMBITS(1) [],
    /// CC2G
    CC2G OFFSET(2) NUMBITS(1) [],
    /// CC3G
    CC3G OFFSET(3) NUMBITS(1) [],
    /// CC4G
    CC4G OFFSET(4) NUMBITS(1) [],
    /// COMG
    COMG OFFSET(5) NUMBITS(1) [],
    /// TG
    TG OFFSET(6) NUMBITS(1) [],
    /// BG
    BG OFFSET(7) NUMBITS(1) [],
    /// B2G
    B2G OFFSET(8) NUMBITS(1) []
],
PSC [
    /// PSC
    PSC OFFSET(0) NUMBITS(16) []
],
ARR [
    /// ARR
    ARR OFFSET(0) NUMBITS(16) []
],
RCR [
    /// REP
    REP OFFSET(0) NUMBITS(16) []
],
CCR1 [
    /// CCR1
    CCR1 OFFSET(0) NUMBITS(16) []
],
CCR2 [
    /// CCR2
    CCR2 OFFSET(0) NUMBITS(16) []
],
CCR3 [
    /// CCR3
    CCR3 OFFSET(0) NUMBITS(16) []
],
CCR4 [
    /// CCR4
    CCR4 OFFSET(0) NUMBITS(16) []
],
DCR [
    /// DBA
    DBA OFFSET(0) NUMBITS(5) [],
    /// DBL
    DBL OFFSET(8) NUMBITS(5) []
],
CCR6 [
    /// CCR6
    CCR6 OFFSET(0) NUMBITS(16) []
]
];

register_bitfields![u32,
CR2 [
    /// CCPC
    CCPC OFFSET(0) NUMBITS(1) [],
    /// CCUS
    CCUS OFFSET(2) NUMBITS(1) [],
    /// CCDS
    CCDS OFFSET(3) NUMBITS(1) [],
    /// MMS
    MMS OFFSET(4) NUMBITS(3) [],
    /// TI1S
    TI1S OFFSET(7) NUMBITS(1) [],
    /// OIS1
    OIS1 OFFSET(8) NUMBITS(1) [],
    /// OIS1N
    OIS1N OFFSET(9) NUMBITS(1) [],
    /// OIS2
    OIS2 OFFSET(10) NUMBITS(1) [],
    /// OIS2N
    OIS2N OFFSET(11) NUMBITS(1) [],
    /// OIS3
    OIS3 OFFSET(12) NUMBITS(1) [],
    /// OIS3N
    OIS3N OFFSET(13) NUMBITS(1) [],
    /// OIS4
    OIS4 OFFSET(14) NUMBITS(1) [],
    /// OIS5
    OIS5 OFFSET(16) NUMBITS(1) [],
    /// OIS6
    OIS6 OFFSET(18) NUMBITS(1) [],
    /// MMS2
    MMS2 OFFSET(20) NUMBITS(4) []
],
SMCR [
    /// SMS
    SMS OFFSET(0) NUMBITS(3) [],
    /// TS
    TS OFFSET(4) NUMBITS(3) [],
    /// MSM
    MSM OFFSET(7) NUMBITS(1) [],
    /// ETF
    ETF OFFSET(8) NUMBITS(4) [],
    /// ETPS
    ETPS OFFSET(12) NUMBITS(2) [],
    /// ECE
    ECE OFFSET(14) NUMBITS(1) [],
    /// ETP
    ETP OFFSET(15) NUMBITS(1) [],
    /// SMS3
    SMS3 OFFSET(16) NUMBITS(1) [],
    /// TS3
    TS3 OFFSET(20) NUMBITS(1) [],
    /// TS4
    TS4 OFFSET(21) NUMBITS(1) []
],
SR [
    /// UIF
    UIF OFFSET(0) NUMBITS(1) [],
    /// CC1IF
    CC1IF OFFSET(1) NUMBITS(1) [],
    /// CC2IF
    CC2IF OFFSET(2) NUMBITS(1) [],
    /// CC3IF
    CC3IF OFFSET(3) NUMBITS(1) [],
    /// CC4IF
    CC4IF OFFSET(4) NUMBITS(1) [],
    /// COMIF
    COMIF OFFSET(5) NUMBITS(1) [],
    /// TIF
    TIF OFFSET(6) NUMBITS(1) [],
    /// BIF
    BIF OFFSET(7) NUMBITS(1) [],
    /// B2IF
    B2IF OFFSET(8) NUMBITS(1) [],
    /// CC1OF
    CC1OF OFFSET(9) NUMBITS(1) [],
    /// CC2OF
    CC2OF OFFSET(10) NUMBITS(1) [],
    /// CC3OF
    CC3OF OFFSET(11) NUMBITS(1) [],
    /// CC4OF
    CC4OF OFFSET(12) NUMBITS(1) [],
    /// SBIF
    SBIF OFFSET(13) NUMBITS(1) [],
    /// CC5IF
    CC5IF OFFSET(16) NUMBITS(1) [],
    /// CC6IF
    CC6IF OFFSET(17) NUMBITS(1) []
],
CCMR1ALTERNATE2 [
    /// CC1S
    CC1S OFFSET(0) NUMBITS(2) [],
    /// IC1PSC
    IC1PSC OFFSET(2) NUMBITS(2) [],
    /// IC1F
    IC1F OFFSET(4) NUMBITS(4) [],
    /// CC2S
    CC2S OFFSET(8) NUMBITS(2) [],
    /// IC2PSC
    IC2PSC OFFSET(10) NUMBITS(2) [],
    /// IC2F
    IC2F OFFSET(12) NUMBITS(4) []
],
CCMR2ALTERNATE18 [
    /// CC3S
    CC3S OFFSET(0) NUMBITS(2) [],
    /// IC3PSC
    IC3PSC OFFSET(2) NUMBITS(2) [],
    /// IC3F
    IC3F OFFSET(4) NUMBITS(4) [],
    /// CC4S
    CC4S OFFSET(8) NUMBITS(2) [],
    /// IC4PSC
    IC4PSC OFFSET(10) NUMBITS(2) [],
    /// IC4F
    IC4F OFFSET(12) NUMBITS(4) []
],
CCER [
    /// CC1E
    CC1E OFFSET(0) NUMBITS(1) [],
    /// CC1P
    CC1P OFFSET(1) NUMBITS(1) [],
    /// CC1NE
    CC1NE OFFSET(2) NUMBITS(1) [],
    /// CC1NP
    CC1NP OFFSET(3) NUMBITS(1) [],
    /// CC2E
    CC2E OFFSET(4) NUMBITS(1) [],
    /// CC2P
    CC2P OFFSET(5) NUMBITS(1) [],
    /// CC2NE
    CC2NE OFFSET(6) NUMBITS(1) [],
    /// CC2NP
    CC2NP OFFSET(7) NUMBITS(1) [],
    /// CC3E
    CC3E OFFSET(8) NUMBITS(1) [],
    /// CC3P
    CC3P OFFSET(9) NUMBITS(1) [],
    /// CC3NE
    CC3NE OFFSET(10) NUMBITS(1) [],
    /// CC3NP
    CC3NP OFFSET(11) NUMBITS(1) [],
    /// CC4E
    CC4E OFFSET(12) NUMBITS(1) [],
    /// CC4P
    CC4P OFFSET(13) NUMBITS(1) [],
    /// CC4NP
    CC4NP OFFSET(15) NUMBITS(1) [],
    /// CC5E
    CC5E OFFSET(16) NUMBITS(1) [],
    /// CC5P
    CC5P OFFSET(17) NUMBITS(1) [],
    /// CC6E
    CC6E OFFSET(20) NUMBITS(1) [],
    /// CC6P
    CC6P OFFSET(21) NUMBITS(1) []
],
CNT [
    /// CNT
    CNT OFFSET(0) NUMBITS(16) [],
    /// UIFCPY
    UIFCPY OFFSET(31) NUMBITS(1) []
],
BDTR [
    /// DTG
    DTG OFFSET(0) NUMBITS(8) [],
    /// LOCK
    LOCK OFFSET(8) NUMBITS(2) [],
    /// OSSI
    OSSI OFFSET(10) NUMBITS(1) [],
    /// OSSR
    OSSR OFFSET(11) NUMBITS(1) [],
    /// BKE
    BKE OFFSET(12) NUMBITS(1) [],
    /// BKP
    BKP OFFSET(13) NUMBITS(1) [],
    /// AOE
    AOE OFFSET(14) NUMBITS(1) [],
    /// MOE
    MOE OFFSET(15) NUMBITS(1) [],
    /// BKF
    BKF OFFSET(16) NUMBITS(4) [],
    /// BK2F
    BK2F OFFSET(20) NUMBITS(4) [],
    /// BK2E
    BK2E OFFSET(24) NUMBITS(1) [],
    /// BK2P
    BK2P OFFSET(25) NUMBITS(1) [],
    /// BKDSRM
    BKDSRM OFFSET(26) NUMBITS(1) [],
    /// BK2DSRM
    BK2DSRM OFFSET(27) NUMBITS(1) [],
    /// BKBID
    BKBID OFFSET(28) NUMBITS(1) [],
    /// BK2BID
    BK2BID OFFSET(29) NUMBITS(1) []
],
DMAR [
    /// DMAB
    DMAB OFFSET(0) NUMBITS(32) []
],
CCMR3 [
    /// OC5FE
    OC5FE OFFSET(2) NUMBITS(1) [],
    /// OC5PE
    OC5PE OFFSET(3) NUMBITS(1) [],
    /// OC5M
    OC5M OFFSET(4) NUMBITS(3) [],
    /// OC5CE
    OC5CE OFFSET(7) NUMBITS(1) [],
    /// OC6FE
    OC6FE OFFSET(10) NUMBITS(1) [],
    /// OC6PE
    OC6PE OFFSET(11) NUMBITS(1) [],
    /// OC6M
    OC6M OFFSET(12) NUMBITS(3) [],
    /// OC6CE
    OC6CE OFFSET(15) NUMBITS(1) [],
    /// OC5M3
    OC5M3 OFFSET(16) NUMBITS(1) [],
    /// OC6M3
    OC6M3 OFFSET(24) NUMBITS(1) []
],
CCR5 [
    /// CCR5
    CCR5 OFFSET(0) NUMBITS(16) [],
    /// GC5C1
    GC5C1 OFFSET(29) NUMBITS(1) [],
    /// GC5C2
    GC5C2 OFFSET(30) NUMBITS(1) [],
    /// GC5C3
    GC5C3 OFFSET(31) NUMBITS(1) []
],
];
const BASE_TIM2: StaticRef<TimRegisters> =
    unsafe { StaticRef::new(0x40000000 as *const TimRegisters) };
const BASE_TIM3: StaticRef<TimRegisters> =
    unsafe { StaticRef::new(0x40001000 as *const TimRegisters) };
const BASE_TIM4: StaticRef<TimRegisters> =
    unsafe { StaticRef::new(0x40002000 as *const TimRegisters) };
const BASE_TIM5: StaticRef<TimRegisters> =
    unsafe { StaticRef::new(0x40003000 as *const TimRegisters) };
