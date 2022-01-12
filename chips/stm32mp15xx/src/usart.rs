use core::cell::Cell;
use kernel::hil;
use kernel::platform::chip::ClockInterface;
use kernel::utilities::cells::{OptionalCell, TakeCell};
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{
    register_bitfields, register_structs, ReadOnly, ReadWrite, WriteOnly,
};
use kernel::utilities::StaticRef;
use kernel::ErrorCode;

use crate::dma1;
use crate::dma1::Dma1Peripheral;
use crate::rcc;

register_structs! {
    /// Universal synchronous asynchronous receiver       transmitter
    UsartRegisters {
        /// Control register 1
        (0x000 => cr1: ReadWrite<u32, CR1::Register>),
        /// Control register 2
        (0x004 => cr2: ReadWrite<u32, CR2::Register>),
        /// Control register 3
        (0x008 => cr3: ReadWrite<u32, CR3::Register>),
        /// Baud rate register
        (0x00C => brr: ReadWrite<u32, BRR::Register>),
        /// Guard time and prescaler           register
        (0x010 => gtpr: ReadWrite<u32, GTPR::Register>),
        /// Receiver timeout register
        (0x014 => rtor: ReadWrite<u32, RTOR::Register>),
        /// Request register
        (0x018 => rqr: WriteOnly<u32, RQR::Register>),
        /// Interrupt & status           register
        (0x01C => isr: ReadOnly<u32, ISR::Register>),
        /// Interrupt flag clear register
        (0x020 => icr: WriteOnly<u32, ICR::Register>),
        /// Receive data register
        (0x024 => rdr: ReadOnly<u32>),
        /// Transmit data register
        (0x028 => tdr: ReadWrite<u32>),
        /// Prescaler register
        (0x02C => presc: ReadWrite<u32>),
        (0x030 => _reserved0),
        /// USART Hardware Configuration register           2
        (0x3EC => hwcfgr2: ReadOnly<u32, HWCFGR2::Register>),
        /// USART Hardware Configuration register           1
        (0x3F0 => hwcfgr1: ReadOnly<u32, HWCFGR1::Register>),
        /// EXTI IP Version register
        (0x3F4 => verr: ReadOnly<u32, VERR::Register>),
        /// EXTI Identification register
        (0x3F8 => ipidr: ReadOnly<u32>),
        /// EXTI Size ID register
        (0x3FC => sidr: ReadOnly<u32>),
        (0x400 => @END),
    }
}
register_bitfields![u32,
CR1 [
    /// RXFIFO Full interrupt               enable
    RXFFIE OFFSET(31) NUMBITS(1) [],
    /// TXFIFO empty interrupt               enable
    TXFEIE OFFSET(30) NUMBITS(1) [],
    /// FIFO mode enable
    FIFOEN OFFSET(29) NUMBITS(1) [],
    /// Word length
    M1 OFFSET(28) NUMBITS(1) [],
    /// End of Block interrupt               enable
    EOBIE OFFSET(27) NUMBITS(1) [],
    /// Receiver timeout interrupt               enable
    RTOIE OFFSET(26) NUMBITS(1) [],
    /// DEAT
    DEAT OFFSET(21) NUMBITS(5) [],
    /// DEDT
    DEDT OFFSET(16) NUMBITS(5) [],
    /// Oversampling mode
    OVER8 OFFSET(15) NUMBITS(1) [],
    /// Character match interrupt               enable
    CMIE OFFSET(14) NUMBITS(1) [],
    /// Mute mode enable
    MME OFFSET(13) NUMBITS(1) [],
    /// Word length
    M0 OFFSET(12) NUMBITS(1) [],
    /// Receiver wakeup method
    WAKE OFFSET(11) NUMBITS(1) [],
    /// Parity control enable
    PCE OFFSET(10) NUMBITS(1) [],
    /// Parity selection
    PS OFFSET(9) NUMBITS(1) [],
    /// PE interrupt enable
    PEIE OFFSET(8) NUMBITS(1) [],
    /// interrupt enable
    TXEIE OFFSET(7) NUMBITS(1) [],
    /// Transmission complete interrupt               enable
    TCIE OFFSET(6) NUMBITS(1) [],
    /// RXNE interrupt enable
    RXNEIE OFFSET(5) NUMBITS(1) [],
    /// IDLE interrupt enable
    IDLEIE OFFSET(4) NUMBITS(1) [],
    /// Transmitter enable
    TE OFFSET(3) NUMBITS(1) [],
    /// Receiver enable
    RE OFFSET(2) NUMBITS(1) [],
    /// USART enable in Stop mode
    UESM OFFSET(1) NUMBITS(1) [],
    /// USART enable
    UE OFFSET(0) NUMBITS(1) []
],
CR2 [
    /// Address of the USART node
    ADD4_7 OFFSET(28) NUMBITS(4) [],
    /// Address of the USART node
    ADD0_3 OFFSET(24) NUMBITS(4) [],
    /// Receiver timeout enable
    RTOEN OFFSET(23) NUMBITS(1) [],
    /// Auto baud rate mode
    ABRMOD OFFSET(21) NUMBITS(2) [],
    /// Auto baud rate enable
    ABREN OFFSET(20) NUMBITS(1) [],
    /// Most significant bit first
    MSBFIRST OFFSET(19) NUMBITS(1) [],
    /// Binary data inversion
    TAINV OFFSET(18) NUMBITS(1) [],
    /// TX pin active level               inversion
    TXINV OFFSET(17) NUMBITS(1) [],
    /// RX pin active level               inversion
    RXINV OFFSET(16) NUMBITS(1) [],
    /// Swap TX/RX pins
    SWAP OFFSET(15) NUMBITS(1) [],
    /// LIN mode enable
    LINEN OFFSET(14) NUMBITS(1) [],
    /// STOP bits
    STOP OFFSET(12) NUMBITS(2) [],
    /// Clock enable
    CLKEN OFFSET(11) NUMBITS(1) [],
    /// Clock polarity
    CPOL OFFSET(10) NUMBITS(1) [],
    /// Clock phase
    CPHA OFFSET(9) NUMBITS(1) [],
    /// Last bit clock pulse
    LBCL OFFSET(8) NUMBITS(1) [],
    /// LIN break detection interrupt               enable
    LBDIE OFFSET(6) NUMBITS(1) [],
    /// LIN break detection length
    LBDL OFFSET(5) NUMBITS(1) [],
    /// 7-bit Address Detection/4-bit Address               Detection
    ADDM7 OFFSET(4) NUMBITS(1) [],
    /// When the DSI_NSS bit is set, the NSS pin               input will be ignored
    DIS_NSS OFFSET(3) NUMBITS(1) [],
    /// Synchronous Slave mode               enable
    SLVEN OFFSET(0) NUMBITS(1) []
],
CR3 [
    /// TXFIFO threshold               configuration
    TXFTCFG OFFSET(29) NUMBITS(3) [],
    /// RXFIFO threshold interrupt               enable
    RXFTIE OFFSET(28) NUMBITS(1) [],
    /// Receive FIFO threshold               configuration
    RXFTCFG OFFSET(25) NUMBITS(3) [],
    /// Tr Complete before guard time, interrupt               enable
    TCBGTIE OFFSET(24) NUMBITS(1) [],
    /// threshold interrupt enable
    TXFTIE OFFSET(23) NUMBITS(1) [],
    /// Wakeup from Stop mode interrupt               enable
    WUFIE OFFSET(22) NUMBITS(1) [],
    /// Wakeup from Stop mode interrupt flag               selection
    WUS OFFSET(20) NUMBITS(2) [],
    /// Smartcard auto-retry count
    SCARCNT OFFSET(17) NUMBITS(3) [],
    /// Driver enable polarity               selection
    DEP OFFSET(15) NUMBITS(1) [],
    /// Driver enable mode
    DEM OFFSET(14) NUMBITS(1) [],
    /// DMA Disable on Reception               Error
    DDRE OFFSET(13) NUMBITS(1) [],
    /// Overrun Disable
    OVRDIS OFFSET(12) NUMBITS(1) [],
    /// One sample bit method               enable
    ONEBIT OFFSET(11) NUMBITS(1) [],
    /// CTS interrupt enable
    CTSIE OFFSET(10) NUMBITS(1) [],
    /// CTS enable
    CTSE OFFSET(9) NUMBITS(1) [],
    /// RTS enable
    RTSE OFFSET(8) NUMBITS(1) [],
    /// DMA enable transmitter
    DMAT OFFSET(7) NUMBITS(1) [],
    /// DMA enable receiver
    DMAR OFFSET(6) NUMBITS(1) [],
    /// Smartcard mode enable
    SCEN OFFSET(5) NUMBITS(1) [],
    /// Smartcard NACK enable
    NACK OFFSET(4) NUMBITS(1) [],
    /// Half-duplex selection
    HDSEL OFFSET(3) NUMBITS(1) [],
    /// Ir low-power
    IRLP OFFSET(2) NUMBITS(1) [],
    /// Ir mode enable
    IREN OFFSET(1) NUMBITS(1) [],
    /// Error interrupt enable
    EIE OFFSET(0) NUMBITS(1) []
],
BRR [
    /// BRR_4_15
    BRR_4_15 OFFSET(4) NUMBITS(12) [],
    /// BRR_0_3
    BRR_0_3 OFFSET(0) NUMBITS(4) []
],
GTPR [
    /// Guard time value
    GT OFFSET(8) NUMBITS(8) [],
    /// Prescaler value
    PSC OFFSET(0) NUMBITS(8) []
],
RTOR [
    /// Block Length
    BLEN OFFSET(24) NUMBITS(8) [],
    /// Receiver timeout value
    RTO OFFSET(0) NUMBITS(24) []
],
RQR [
    /// Transmit data flush               request
    TXFRQ OFFSET(4) NUMBITS(1) [],
    /// Receive data flush request
    RXFRQ OFFSET(3) NUMBITS(1) [],
    /// Mute mode request
    MMRQ OFFSET(2) NUMBITS(1) [],
    /// Send break request
    SBKRQ OFFSET(1) NUMBITS(1) [],
    /// Auto baud rate request
    ABRRQ OFFSET(0) NUMBITS(1) []
],
ISR [
    /// TXFIFO threshold flag
    TXFT OFFSET(27) NUMBITS(1) [],
    /// RXFIFO threshold flag
    RXFT OFFSET(26) NUMBITS(1) [],
    /// Transmission complete before guard time               flag
    TCBGT OFFSET(25) NUMBITS(1) [],
    /// RXFIFO Full
    RXFF OFFSET(24) NUMBITS(1) [],
    /// TXFIFO Empty
    TXFE OFFSET(23) NUMBITS(1) [],
    /// REACK
    REACK OFFSET(22) NUMBITS(1) [],
    /// TEACK
    TEACK OFFSET(21) NUMBITS(1) [],
    /// WUF
    WUF OFFSET(20) NUMBITS(1) [],
    /// RWU
    RWU OFFSET(19) NUMBITS(1) [],
    /// SBKF
    SBKF OFFSET(18) NUMBITS(1) [],
    /// CMF
    CMF OFFSET(17) NUMBITS(1) [],
    /// BUSY
    BUSY OFFSET(16) NUMBITS(1) [],
    /// ABRF
    ABRF OFFSET(15) NUMBITS(1) [],
    /// ABRE
    ABRE OFFSET(14) NUMBITS(1) [],
    /// SPI slave underrun error               flag
    UDR OFFSET(13) NUMBITS(1) [],
    /// EOBF
    EOBF OFFSET(12) NUMBITS(1) [],
    /// RTOF
    RTOF OFFSET(11) NUMBITS(1) [],
    /// CTS
    CTS OFFSET(10) NUMBITS(1) [],
    /// CTSIF
    CTSIF OFFSET(9) NUMBITS(1) [],
    /// LBDF
    LBDF OFFSET(8) NUMBITS(1) [],
    /// TXE
    TXE OFFSET(7) NUMBITS(1) [],
    /// TC
    TC OFFSET(6) NUMBITS(1) [],
    /// RXNE
    RXNE OFFSET(5) NUMBITS(1) [],
    /// IDLE
    IDLE OFFSET(4) NUMBITS(1) [],
    /// ORE
    ORE OFFSET(3) NUMBITS(1) [],
    /// NF
    NF OFFSET(2) NUMBITS(1) [],
    /// FE
    FE OFFSET(1) NUMBITS(1) [],
    /// PE
    PE OFFSET(0) NUMBITS(1) []
],
ICR [
    /// Wakeup from Stop mode clear               flag
    WUCF OFFSET(20) NUMBITS(1) [],
    /// Character match clear flag
    CMCF OFFSET(17) NUMBITS(1) [],
    /// SPI slave underrun clear               flag
    UDRCF OFFSET(13) NUMBITS(1) [],
    /// End of block clear flag
    EOBCF OFFSET(12) NUMBITS(1) [],
    /// Receiver timeout clear               flag
    RTOCF OFFSET(11) NUMBITS(1) [],
    /// CTS clear flag
    CTSCF OFFSET(9) NUMBITS(1) [],
    /// LIN break detection clear               flag
    LBDCF OFFSET(8) NUMBITS(1) [],
    /// Transmission complete before Guard time               clear flag
    TCBGTCF OFFSET(7) NUMBITS(1) [],
    /// Transmission complete clear               flag
    TCCF OFFSET(6) NUMBITS(1) [],
    /// TXFIFO empty clear flag
    TXFECF OFFSET(5) NUMBITS(1) [],
    /// Idle line detected clear               flag
    IDLECF OFFSET(4) NUMBITS(1) [],
    /// Overrun error clear flag
    ORECF OFFSET(3) NUMBITS(1) [],
    /// Noise detected clear flag
    NCF OFFSET(2) NUMBITS(1) [],
    /// Framing error clear flag
    FECF OFFSET(1) NUMBITS(1) [],
    /// Parity error clear flag
    PECF OFFSET(0) NUMBITS(1) []
],
RDR [
    /// Receive data value
    RDR OFFSET(0) NUMBITS(9) []
],
TDR [
    /// Transmit data value
    TDR OFFSET(0) NUMBITS(9) []
],
PRESC [
    /// Clock prescaler
    PRESCALER OFFSET(0) NUMBITS(4) []
],
HWCFGR2 [
    /// CFG1
    CFG1 OFFSET(0) NUMBITS(4) [],
    /// CFG2
    CFG2 OFFSET(4) NUMBITS(4) []
],
HWCFGR1 [
    /// CFG1
    CFG1 OFFSET(0) NUMBITS(4) [],
    /// CFG2
    CFG2 OFFSET(4) NUMBITS(4) [],
    /// CFG3
    CFG3 OFFSET(8) NUMBITS(4) [],
    /// CFG4
    CFG4 OFFSET(12) NUMBITS(4) [],
    /// CFG5
    CFG5 OFFSET(16) NUMBITS(4) [],
    /// CFG6
    CFG6 OFFSET(20) NUMBITS(4) [],
    /// CFG7
    CFG7 OFFSET(24) NUMBITS(4) [],
    /// CFG8
    CFG8 OFFSET(28) NUMBITS(4) []
],
VERR [
    /// Minor Revision number
    MINREV OFFSET(0) NUMBITS(4) [],
    /// Major Revision number
    MAJREV OFFSET(4) NUMBITS(4) []
],
IPIDR [
    /// IP Identification
    IPID OFFSET(0) NUMBITS(32) []
],
SIDR [
    /// Size Identification
    SID OFFSET(0) NUMBITS(32) []
]
];
const USART2_BASE: StaticRef<UsartRegisters> =
    unsafe { StaticRef::new(0x4000E000 as *const UsartRegisters) };

// for use by dma1
pub(crate) fn get_address_dr(regs: StaticRef<UsartRegisters>) -> u32 {
    &regs.dr as *const ReadWrite<u32> as u32
}


#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq)]
enum USARTStateTX {
    Idle,
    Transmitting,
    AbortRequested,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq)]
enum USARTStateRX {
    Idle,
    Receiving,
    AbortRequested,
}

pub struct Usart<'a> {
    registers: StaticRef<UsartRegisters>,
    clock: UsartClock<'a>,

    tx_client: OptionalCell<&'a dyn hil::uart::TransmitClient>,
    rx_client: OptionalCell<&'a dyn hil::uart::ReceiveClient>,

    tx_buffer: TakeCell<'static, [u8]>,
    tx_position: Cell<usize>,
    tx_len: Cell<usize>,
    tx_status: Cell<USARTStateTX>,

    rx_buffer: TakeCell<'static, [u8]>,
    rx_position: Cell<usize>,
    rx_len: Cell<usize>,
    rx_status: Cell<USARTStateRX>,
}

impl<'a> Usart<'a> {
    const fn new(base_addr: StaticRef<UsartRegisters>, clock: UsartClock<'a>) -> Self {
        Self {
            registers: base_addr,
            clock: clock,

            tx_client: OptionalCell::empty(),
            rx_client: OptionalCell::empty(),

            tx_buffer: TakeCell::empty(),
            tx_position: Cell::new(0),
            tx_len: Cell::new(0),
            tx_status: Cell::new(USARTStateTX::Idle),

            rx_buffer: TakeCell::empty(),
            rx_position: Cell::new(0),
            rx_len: Cell::new(0),
            rx_status: Cell::new(USARTStateRX::Idle),
        }
    }

    pub const fn new_usart2(rcc: &'a rcc::Rcc) -> Self {
        Self::new(
            USART2_BASE,
            UsartClock(rcc::PeripheralClock::new(
                rcc::PeripheralClockType::APB1(rcc::PCLK1::USART2),
                rcc,
            )),
        )
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

    // for use by panic in io.rs
    pub fn send_byte(&self, byte: u8) {
        // loop till TXE (Transmit data register empty) becomes 1
        while !self.registers.isr.is_set(ISR::TXE) {}
        self.registers.tdr.set(byte.into());
    }

    fn enable_transmit_interrupt(&self) {
        self.registers.cr1.modify(CR1::TXEIE::SET);
    }

    fn disable_transmit_interrupt(&self) {
        self.registers.cr1.modify(CR1::TXEIE::CLEAR);
    }

    fn enable_receive_interrupt(&self) {
        self.registers.cr1.modify(CR1::RXNEIE::SET);
    }

    fn disable_receive_interrupt(&self) {
        self.registers.cr1.modify(CR1::RXNEIE::CLEAR);
    }

    fn clear_overrun(&self) {
        self.registers.icr.write(ICR::ORECF::SET);
    }

    pub fn handle_interrupt(&self) {
        if self.registers.isr.is_set(ISR::TXE) {
            self.disable_transmit_interrupt();

            // ignore IRQ if not transmitting
            if self.tx_status.get() == USARTStateTX::Transmitting {
                if self.tx_position.get() < self.tx_len.get() {
                    self.tx_buffer.map(|buf| {
                        self.registers.tdr.set(buf[self.tx_position.get()].into());
                        self.tx_position.replace(self.tx_position.get() + 1);
                    });
                }
                if self.tx_position.get() == self.tx_len.get() {
                    // transmission done
                    self.tx_status.replace(USARTStateTX::Idle);
                } else {
                    self.enable_transmit_interrupt();
                }
                // notify client if transfer is done
                if self.tx_status.get() == USARTStateTX::Idle {
                    self.tx_client.map(|client| {
                        if let Some(buf) = self.tx_buffer.take() {
                            client.transmitted_buffer(buf, self.tx_len.get(), Ok(()));
                        }
                    });
                }
            } else if self.tx_status.get() == USARTStateTX::AbortRequested {
                self.tx_status.replace(USARTStateTX::Idle);
                self.tx_client.map(|client| {
                    if let Some(buf) = self.tx_buffer.take() {
                        client.transmitted_buffer(
                            buf,
                            self.tx_position.get(),
                            Err(ErrorCode::CANCEL),
                        );
                    }
                });
            }
        }

        if self.registers.isr.is_set(ISR::RXNE) {
            let byte = self.registers.rdr.get() as u8;
            self.disable_receive_interrupt();

            // ignore IRQ if not receiving
            if self.rx_status.get() == USARTStateRX::Receiving {
                if self.rx_position.get() < self.rx_len.get() {
                    self.rx_buffer.map(|buf| {
                        buf[self.rx_position.get()] = byte;
                        self.rx_position.replace(self.rx_position.get() + 1);
                    });
                }
                if self.rx_position.get() == self.rx_len.get() {
                    // reception done
                    self.rx_status.replace(USARTStateRX::Idle);
                } else {
                    self.enable_receive_interrupt();
                }
                // notify client if transfer is done
                if self.rx_status.get() == USARTStateRX::Idle {
                    self.rx_client.map(|client| {
                        if let Some(buf) = self.rx_buffer.take() {
                            client.received_buffer(
                                buf,
                                self.rx_len.get(),
                                Ok(()),
                                hil::uart::Error::None,
                            );
                        }
                    });
                }
            } else if self.rx_status.get() == USARTStateRX::AbortRequested {
                self.rx_status.replace(USARTStateRX::Idle);
                self.rx_client.map(|client| {
                    if let Some(buf) = self.rx_buffer.take() {
                        client.received_buffer(
                            buf,
                            self.rx_position.get(),
                            Err(ErrorCode::CANCEL),
                            hil::uart::Error::Aborted,
                        );
                    }
                });
            }
        }

        if self.registers.isr.is_set(ISR::ORE) {
            self.clear_overrun();
            self.rx_status.replace(USARTStateRX::Idle);
            self.rx_client.map(|client| {
                if let Some(buf) = self.rx_buffer.take() {
                    client.received_buffer(
                        buf,
                        self.rx_position.get(),
                        Err(ErrorCode::CANCEL),
                        hil::uart::Error::OverrunError,
                    );
                }
            });
        }
    }
}

impl<'a> hil::uart::Transmit<'a> for Usart<'a> {
    fn set_transmit_client(&self, client: &'a dyn hil::uart::TransmitClient) {
        self.tx_client.set(client);
    }

    fn transmit_buffer(
        &self,
        tx_data: &'static mut [u8],
        tx_len: usize,
    ) -> Result<(), (ErrorCode, &'static mut [u8])> {
        if self.tx_status.get() == USARTStateTX::Idle {
            if tx_len <= tx_data.len() {
                self.tx_buffer.put(Some(tx_data));
                self.tx_position.set(0);
                self.tx_len.set(tx_len);
                self.tx_status.set(USARTStateTX::Transmitting);
                self.enable_transmit_interrupt();
                Ok(())
            } else {
                Err((ErrorCode::SIZE, tx_data))
            }
        } else {
            Err((ErrorCode::BUSY, tx_data))
        }
    }

    fn transmit_word(&self, _word: u32) -> Result<(), ErrorCode> {
        Err(ErrorCode::FAIL)
    }

    fn transmit_abort(&self) -> Result<(), ErrorCode> {
        if self.tx_status.get() != USARTStateTX::Idle {
            self.tx_status.set(USARTStateTX::AbortRequested);
            Err(ErrorCode::BUSY)
        } else {
            Ok(())
        }
    }
}

impl hil::uart::Configure for Usart<'_> {
    fn configure(&self, params: hil::uart::Parameters) -> Result<(), ErrorCode> {
        if params.baud_rate != 115200
            || params.stop_bits != hil::uart::StopBits::One
            || params.parity != hil::uart::Parity::None
            || params.hw_flow_control != false
            || params.width != hil::uart::Width::Eight
        {
            panic!(
                "Currently we only support uart setting of 115200bps 8N1, no hardware flow control"
            );
        }

        // Configure the word length - 0: 1 Start bit, 8 Data bits, n Stop bits
        self.registers.cr1.modify(CR1::M0::CLEAR);
        self.registers.cr1.modify(CR1::M1::CLEAR);

        // Set the stop bit length - 00: 1 Stop bits
        self.registers.cr2.modify(CR2::STOP.val(0b00 as u32));

        // Set no parity
        self.registers.cr1.modify(CR1::PCE::CLEAR);

        // Set the baud rate. By default OVER8 is 0 (oversampling by 16) and
        // PCLK1 is at 8Mhz. The desired baud rate is 115.2KBps. So according
        // to Table 159 of reference manual, the value for BRR is 69.444 (0x45)
        // DIV_Fraction = 0x5
        // DIV_Mantissa = 0x4
        self.registers.brr.modify(BRR::BRR_0_3.val(0x5 as u32)); // TODO: Check, may be reversed
        self.registers.brr.modify(BRR::BRR_4_15.val(0x4 as u32));

        // Enable transmit block
        self.registers.cr1.modify(CR1::TE::SET);

        // Enable receive block
        self.registers.cr1.modify(CR1::RE::SET);

        // Enable USART
        self.registers.cr1.modify(CR1::UE::SET);

        Ok(())
    }
}

impl<'a> hil::uart::Receive<'a> for Usart<'a> {
    fn set_receive_client(&self, client: &'a dyn hil::uart::ReceiveClient) {
        self.rx_client.set(client);
    }

    fn receive_buffer(
        &self,
        rx_buffer: &'static mut [u8],
        rx_len: usize,
    ) -> Result<(), (ErrorCode, &'static mut [u8])> {
        if self.rx_status.get() == USARTStateRX::Idle {
            if rx_len <= rx_buffer.len() {
                self.rx_buffer.put(Some(rx_buffer));
                self.rx_position.set(0);
                self.rx_len.set(rx_len);
                self.rx_status.set(USARTStateRX::Receiving);
                self.enable_receive_interrupt();
                Ok(())
            } else {
                Err((ErrorCode::SIZE, rx_buffer))
            }
        } else {
            Err((ErrorCode::BUSY, rx_buffer))
        }
    }

    fn receive_word(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::FAIL)
    }

    fn receive_abort(&self) -> Result<(), ErrorCode> {
        if self.rx_status.get() != USARTStateRX::Idle {
            self.rx_status.set(USARTStateRX::AbortRequested);
            Err(ErrorCode::BUSY)
        } else {
            Ok(())
        }
    }
}

struct UsartClock<'a>(rcc::PeripheralClock<'a>);

impl ClockInterface for UsartClock<'_> {
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
