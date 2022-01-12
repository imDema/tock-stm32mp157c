use kernel::platform::chip::ClockInterface;
use kernel::utilities::cells::{OptionalCell, TakeCell};
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, register_structs, ReadOnly, ReadWrite, WriteOnly};
use kernel::utilities::StaticRef;

use crate::nvic;
// use crate::rcc;
// use crate::spi;
use crate::usart;

register_structs! {
    /// DMA1
    Dma1Registers {
        /// DMA low interrupt status register
        (0x000 => lisr: ReadOnly<u32, LISR::Register>),
        /// DMA high interrupt status register
        (0x004 => hisr: ReadOnly<u32, HISR::Register>),
        /// DMA low interrupt flag clear register
        (0x008 => lifcr: WriteOnly<u32, LIFCR::Register>),
        /// DMA high interrupt flag clear register
        (0x00C => hifcr: WriteOnly<u32, HIFCR::Register>),
        /// This register is used to configure the concerned stream.
        (0x010 => s0cr: ReadWrite<u32, S0CR::Register>),
        /// DMA stream 0 number of data register
        (0x014 => s0ndtr: ReadWrite<u32>),
        /// DMA stream 0 peripheral address register
        (0x018 => s0par: ReadWrite<u32>),
        /// DMA stream 0 memory 0 address register
        (0x01C => s0m0ar: ReadWrite<u32>),
        /// DMA stream 0 memory 1 address register
        (0x020 => s0m1ar: ReadWrite<u32>),
        /// DMA stream 0 FIFO control register
        (0x024 => s0fcr: ReadWrite<u32, S0FCR::Register>),
        /// This register is used to configure the concerned stream.
        (0x028 => s1cr: ReadWrite<u32, S1CR::Register>),
        /// DMA stream 1 number of data register
        (0x02C => s1ndtr: ReadWrite<u32>),
        /// DMA stream 1 peripheral address register
        (0x030 => s1par: ReadWrite<u32>),
        /// DMA stream 1 memory 0 address register
        (0x034 => s1m0ar: ReadWrite<u32>),
        /// DMA stream 1 memory 1 address register
        (0x038 => s1m1ar: ReadWrite<u32>),
        /// DMA stream 1 FIFO control register
        (0x03C => s1fcr: ReadWrite<u32, S1FCR::Register>),
        /// This register is used to configure the concerned stream.
        (0x040 => s2cr: ReadWrite<u32, S2CR::Register>),
        /// DMA stream 2 number of data register
        (0x044 => s2ndtr: ReadWrite<u32>),
        /// DMA stream 2 peripheral address register
        (0x048 => s2par: ReadWrite<u32>),
        /// DMA stream 2 memory 0 address register
        (0x04C => s2m0ar: ReadWrite<u32>),
        /// DMA stream 2 memory 1 address register
        (0x050 => s2m1ar: ReadWrite<u32>),
        /// DMA stream 2 FIFO control register
        (0x054 => s2fcr: ReadWrite<u32, S2FCR::Register>),
        /// This register is used to configure the concerned stream.
        (0x058 => s3cr: ReadWrite<u32, S3CR::Register>),
        /// DMA stream 3 number of data register
        (0x05C => s3ndtr: ReadWrite<u32>),
        /// DMA stream 3 peripheral address register
        (0x060 => s3par: ReadWrite<u32>),
        /// DMA stream 3 memory 0 address register
        (0x064 => s3m0ar: ReadWrite<u32>),
        /// DMA stream 3 memory 1 address register
        (0x068 => s3m1ar: ReadWrite<u32>),
        /// DMA stream 3 FIFO control register
        (0x06C => s3fcr: ReadWrite<u32, S3FCR::Register>),
        /// This register is used to configure the concerned stream.
        (0x070 => s4cr: ReadWrite<u32, S4CR::Register>),
        /// DMA stream 4 number of data register
        (0x074 => s4ndtr: ReadWrite<u32>),
        /// DMA stream 4 peripheral address register
        (0x078 => s4par: ReadWrite<u32>),
        /// DMA stream 4 memory 0 address register
        (0x07C => s4m0ar: ReadWrite<u32>),
        /// DMA stream 4 memory 1 address register
        (0x080 => s4m1ar: ReadWrite<u32>),
        /// DMA stream 4 FIFO control register
        (0x084 => s4fcr: ReadWrite<u32, S4FCR::Register>),
        /// This register is used to configure the concerned stream.
        (0x088 => s5cr: ReadWrite<u32, S5CR::Register>),
        /// DMA stream 5 number of data register
        (0x08C => s5ndtr: ReadWrite<u32>),
        /// DMA stream 5 peripheral address register
        (0x090 => s5par: ReadWrite<u32>),
        /// DMA stream 5 memory 0 address register
        (0x094 => s5m0ar: ReadWrite<u32>),
        /// DMA stream 5 memory 1 address register
        (0x098 => s5m1ar: ReadWrite<u32>),
        /// DMA stream 5 FIFO control register
        (0x09C => s5fcr: ReadWrite<u32, S5FCR::Register>),
        /// This register is used to configure the concerned stream.
        (0x0A0 => s6cr: ReadWrite<u32, S6CR::Register>),
        /// DMA stream 6 number of data register
        (0x0A4 => s6ndtr: ReadWrite<u32>),
        /// DMA stream 6 peripheral address register
        (0x0A8 => s6par: ReadWrite<u32>),
        /// DMA stream 6 memory 0 address register
        (0x0AC => s6m0ar: ReadWrite<u32>),
        /// DMA stream 6 memory 1 address register
        (0x0B0 => s6m1ar: ReadWrite<u32>),
        /// DMA stream 6 FIFO control register
        (0x0B4 => s6fcr: ReadWrite<u32, S6FCR::Register>),
        /// This register is used to configure the concerned stream.
        (0x0B8 => s7cr: ReadWrite<u32, S7CR::Register>),
        /// DMA stream 7 number of data register
        (0x0BC => s7ndtr: ReadWrite<u32>),
        /// DMA stream 7 peripheral address register
        (0x0C0 => s7par: ReadWrite<u32>),
        /// DMA stream 7 memory 0 address register
        (0x0C4 => s7m0ar: ReadWrite<u32>),
        /// DMA stream 7 memory 1 address register
        (0x0C8 => s7m1ar: ReadWrite<u32>),
        /// DMA stream 7 FIFO control register
        (0x0CC => s7fcr: ReadWrite<u32, S7FCR::Register>),
        (0x0D0 => _reserved0),
        /// DMA hardware configuration 2register
        (0x3EC => hwcfgr2: ReadOnly<u32, HWCFGR2::Register>),
        /// DMA hardware configuration 1 register
        (0x3F0 => hwcfgr1: ReadOnly<u32, HWCFGR1::Register>),
        /// This register identifies the version of the IP.
        (0x3F4 => verr: ReadOnly<u32, VERR::Register>),
        /// DMA IP identification register
        (0x3F8 => ipdr: ReadOnly<u32>),
        /// DMA size identification register
        (0x3FC => sidr: ReadOnly<u32>),
        (0x400 => @END),
    }
}
register_bitfields![u32,
LISR [
    /// FEIF0
    FEIF0 OFFSET(0) NUMBITS(1) [],
    /// DMEIF0
    DMEIF0 OFFSET(2) NUMBITS(1) [],
    /// TEIF0
    TEIF0 OFFSET(3) NUMBITS(1) [],
    /// HTIF0
    HTIF0 OFFSET(4) NUMBITS(1) [],
    /// TCIF0
    TCIF0 OFFSET(5) NUMBITS(1) [],
    /// FEIF1
    FEIF1 OFFSET(6) NUMBITS(1) [],
    /// DMEIF1
    DMEIF1 OFFSET(8) NUMBITS(1) [],
    /// TEIF1
    TEIF1 OFFSET(9) NUMBITS(1) [],
    /// HTIF1
    HTIF1 OFFSET(10) NUMBITS(1) [],
    /// TCIF1
    TCIF1 OFFSET(11) NUMBITS(1) [],
    /// FEIF2
    FEIF2 OFFSET(16) NUMBITS(1) [],
    /// DMEIF2
    DMEIF2 OFFSET(18) NUMBITS(1) [],
    /// TEIF2
    TEIF2 OFFSET(19) NUMBITS(1) [],
    /// HTIF2
    HTIF2 OFFSET(20) NUMBITS(1) [],
    /// TCIF2
    TCIF2 OFFSET(21) NUMBITS(1) [],
    /// FEIF3
    FEIF3 OFFSET(22) NUMBITS(1) [],
    /// DMEIF3
    DMEIF3 OFFSET(24) NUMBITS(1) [],
    /// TEIF3
    TEIF3 OFFSET(25) NUMBITS(1) [],
    /// HTIF3
    HTIF3 OFFSET(26) NUMBITS(1) [],
    /// TCIF3
    TCIF3 OFFSET(27) NUMBITS(1) []
],
HISR [
    /// FEIF4
    FEIF4 OFFSET(0) NUMBITS(1) [],
    /// DMEIF4
    DMEIF4 OFFSET(2) NUMBITS(1) [],
    /// TEIF4
    TEIF4 OFFSET(3) NUMBITS(1) [],
    /// HTIF4
    HTIF4 OFFSET(4) NUMBITS(1) [],
    /// TCIF4
    TCIF4 OFFSET(5) NUMBITS(1) [],
    /// FEIF5
    FEIF5 OFFSET(6) NUMBITS(1) [],
    /// DMEIF5
    DMEIF5 OFFSET(8) NUMBITS(1) [],
    /// TEIF5
    TEIF5 OFFSET(9) NUMBITS(1) [],
    /// HTIF5
    HTIF5 OFFSET(10) NUMBITS(1) [],
    /// TCIF5
    TCIF5 OFFSET(11) NUMBITS(1) [],
    /// FEIF6
    FEIF6 OFFSET(16) NUMBITS(1) [],
    /// DMEIF6
    DMEIF6 OFFSET(18) NUMBITS(1) [],
    /// TEIF6
    TEIF6 OFFSET(19) NUMBITS(1) [],
    /// HTIF6
    HTIF6 OFFSET(20) NUMBITS(1) [],
    /// TCIF6
    TCIF6 OFFSET(21) NUMBITS(1) [],
    /// FEIF7
    FEIF7 OFFSET(22) NUMBITS(1) [],
    /// DMEIF7
    DMEIF7 OFFSET(24) NUMBITS(1) [],
    /// TEIF7
    TEIF7 OFFSET(25) NUMBITS(1) [],
    /// HTIF7
    HTIF7 OFFSET(26) NUMBITS(1) [],
    /// TCIF7
    TCIF7 OFFSET(27) NUMBITS(1) []
],
LIFCR [
    /// CFEIF0
    CFEIF0 OFFSET(0) NUMBITS(1) [],
    /// CDMEIF0
    CDMEIF0 OFFSET(2) NUMBITS(1) [],
    /// CTEIF0
    CTEIF0 OFFSET(3) NUMBITS(1) [],
    /// CHTIF0
    CHTIF0 OFFSET(4) NUMBITS(1) [],
    /// CTCIF0
    CTCIF0 OFFSET(5) NUMBITS(1) [],
    /// CFEIF1
    CFEIF1 OFFSET(6) NUMBITS(1) [],
    /// CDMEIF1
    CDMEIF1 OFFSET(8) NUMBITS(1) [],
    /// CTEIF1
    CTEIF1 OFFSET(9) NUMBITS(1) [],
    /// CHTIF1
    CHTIF1 OFFSET(10) NUMBITS(1) [],
    /// CTCIF1
    CTCIF1 OFFSET(11) NUMBITS(1) [],
    /// CFEIF2
    CFEIF2 OFFSET(16) NUMBITS(1) [],
    /// CDMEIF2
    CDMEIF2 OFFSET(18) NUMBITS(1) [],
    /// CTEIF2
    CTEIF2 OFFSET(19) NUMBITS(1) [],
    /// CHTIF2
    CHTIF2 OFFSET(20) NUMBITS(1) [],
    /// CTCIF2
    CTCIF2 OFFSET(21) NUMBITS(1) [],
    /// CFEIF3
    CFEIF3 OFFSET(22) NUMBITS(1) [],
    /// CDMEIF3
    CDMEIF3 OFFSET(24) NUMBITS(1) [],
    /// CTEIF3
    CTEIF3 OFFSET(25) NUMBITS(1) [],
    /// CHTIF3
    CHTIF3 OFFSET(26) NUMBITS(1) [],
    /// CTCIF3
    CTCIF3 OFFSET(27) NUMBITS(1) []
],
HIFCR [
    /// CFEIF4
    CFEIF4 OFFSET(0) NUMBITS(1) [],
    /// CDMEIF4
    CDMEIF4 OFFSET(2) NUMBITS(1) [],
    /// CTEIF4
    CTEIF4 OFFSET(3) NUMBITS(1) [],
    /// CHTIF4
    CHTIF4 OFFSET(4) NUMBITS(1) [],
    /// CTCIF4
    CTCIF4 OFFSET(5) NUMBITS(1) [],
    /// CFEIF5
    CFEIF5 OFFSET(6) NUMBITS(1) [],
    /// CDMEIF5
    CDMEIF5 OFFSET(8) NUMBITS(1) [],
    /// CTEIF5
    CTEIF5 OFFSET(9) NUMBITS(1) [],
    /// CHTIF5
    CHTIF5 OFFSET(10) NUMBITS(1) [],
    /// CTCIF5
    CTCIF5 OFFSET(11) NUMBITS(1) [],
    /// CFEIF6
    CFEIF6 OFFSET(16) NUMBITS(1) [],
    /// CDMEIF6
    CDMEIF6 OFFSET(18) NUMBITS(1) [],
    /// CTEIF6
    CTEIF6 OFFSET(19) NUMBITS(1) [],
    /// CHTIF6
    CHTIF6 OFFSET(20) NUMBITS(1) [],
    /// CTCIF6
    CTCIF6 OFFSET(21) NUMBITS(1) [],
    /// CFEIF7
    CFEIF7 OFFSET(22) NUMBITS(1) [],
    /// CDMEIF7
    CDMEIF7 OFFSET(24) NUMBITS(1) [],
    /// CTEIF7
    CTEIF7 OFFSET(25) NUMBITS(1) [],
    /// CHTIF7
    CHTIF7 OFFSET(26) NUMBITS(1) [],
    /// CTCIF7
    CTCIF7 OFFSET(27) NUMBITS(1) []
],
S0CR [
    /// EN
    EN OFFSET(0) NUMBITS(1) [],
    /// DMEIE
    DMEIE OFFSET(1) NUMBITS(1) [],
    /// TEIE
    TEIE OFFSET(2) NUMBITS(1) [],
    /// HTIE
    HTIE OFFSET(3) NUMBITS(1) [],
    /// TCIE
    TCIE OFFSET(4) NUMBITS(1) [],
    /// PFCTRL
    PFCTRL OFFSET(5) NUMBITS(1) [],
    /// DIR
    DIR OFFSET(6) NUMBITS(2) [],
    /// CIRC
    CIRC OFFSET(8) NUMBITS(1) [],
    /// PINC
    PINC OFFSET(9) NUMBITS(1) [],
    /// MINC
    MINC OFFSET(10) NUMBITS(1) [],
    /// PSIZE
    PSIZE OFFSET(11) NUMBITS(2) [],
    /// MSIZE
    MSIZE OFFSET(13) NUMBITS(2) [],
    /// PINCOS
    PINCOS OFFSET(15) NUMBITS(1) [],
    /// PL
    PL OFFSET(16) NUMBITS(2) [],
    /// DBM
    DBM OFFSET(18) NUMBITS(1) [],
    /// CT
    CT OFFSET(19) NUMBITS(1) [],
    /// PBURST
    PBURST OFFSET(21) NUMBITS(2) [],
    /// MBURST
    MBURST OFFSET(23) NUMBITS(2) []
],
S0NDTR [
    /// NDT
    NDT OFFSET(0) NUMBITS(16) []
],
S0PAR [
    /// PAR
    PAR OFFSET(0) NUMBITS(32) []
],
S0M0AR [
    /// M0A
    M0A OFFSET(0) NUMBITS(32) []
],
S0M1AR [
    /// M1A
    M1A OFFSET(0) NUMBITS(32) []
],
S0FCR [
    /// FTH
    FTH OFFSET(0) NUMBITS(2) [],
    /// DMDIS
    DMDIS OFFSET(2) NUMBITS(1) [],
    /// FS
    FS OFFSET(3) NUMBITS(3) [],
    /// FEIE
    FEIE OFFSET(7) NUMBITS(1) []
],
S1CR [
    /// EN
    EN OFFSET(0) NUMBITS(1) [],
    /// DMEIE
    DMEIE OFFSET(1) NUMBITS(1) [],
    /// TEIE
    TEIE OFFSET(2) NUMBITS(1) [],
    /// HTIE
    HTIE OFFSET(3) NUMBITS(1) [],
    /// TCIE
    TCIE OFFSET(4) NUMBITS(1) [],
    /// PFCTRL
    PFCTRL OFFSET(5) NUMBITS(1) [],
    /// DIR
    DIR OFFSET(6) NUMBITS(2) [],
    /// CIRC
    CIRC OFFSET(8) NUMBITS(1) [],
    /// PINC
    PINC OFFSET(9) NUMBITS(1) [],
    /// MINC
    MINC OFFSET(10) NUMBITS(1) [],
    /// PSIZE
    PSIZE OFFSET(11) NUMBITS(2) [],
    /// MSIZE
    MSIZE OFFSET(13) NUMBITS(2) [],
    /// PINCOS
    PINCOS OFFSET(15) NUMBITS(1) [],
    /// PL
    PL OFFSET(16) NUMBITS(2) [],
    /// DBM
    DBM OFFSET(18) NUMBITS(1) [],
    /// CT
    CT OFFSET(19) NUMBITS(1) [],
    /// PBURST
    PBURST OFFSET(21) NUMBITS(2) [],
    /// MBURST
    MBURST OFFSET(23) NUMBITS(2) []
],
S1NDTR [
    /// NDT
    NDT OFFSET(0) NUMBITS(16) []
],
S1PAR [
    /// PAR
    PAR OFFSET(0) NUMBITS(32) []
],
S1M0AR [
    /// M0A
    M0A OFFSET(0) NUMBITS(32) []
],
S1M1AR [
    /// M1A
    M1A OFFSET(0) NUMBITS(32) []
],
S1FCR [
    /// FTH
    FTH OFFSET(0) NUMBITS(2) [],
    /// DMDIS
    DMDIS OFFSET(2) NUMBITS(1) [],
    /// FS
    FS OFFSET(3) NUMBITS(3) [],
    /// FEIE
    FEIE OFFSET(7) NUMBITS(1) []
],
S2CR [
    /// EN
    EN OFFSET(0) NUMBITS(1) [],
    /// DMEIE
    DMEIE OFFSET(1) NUMBITS(1) [],
    /// TEIE
    TEIE OFFSET(2) NUMBITS(1) [],
    /// HTIE
    HTIE OFFSET(3) NUMBITS(1) [],
    /// TCIE
    TCIE OFFSET(4) NUMBITS(1) [],
    /// PFCTRL
    PFCTRL OFFSET(5) NUMBITS(1) [],
    /// DIR
    DIR OFFSET(6) NUMBITS(2) [],
    /// CIRC
    CIRC OFFSET(8) NUMBITS(1) [],
    /// PINC
    PINC OFFSET(9) NUMBITS(1) [],
    /// MINC
    MINC OFFSET(10) NUMBITS(1) [],
    /// PSIZE
    PSIZE OFFSET(11) NUMBITS(2) [],
    /// MSIZE
    MSIZE OFFSET(13) NUMBITS(2) [],
    /// PINCOS
    PINCOS OFFSET(15) NUMBITS(1) [],
    /// PL
    PL OFFSET(16) NUMBITS(2) [],
    /// DBM
    DBM OFFSET(18) NUMBITS(1) [],
    /// CT
    CT OFFSET(19) NUMBITS(1) [],
    /// PBURST
    PBURST OFFSET(21) NUMBITS(2) [],
    /// MBURST
    MBURST OFFSET(23) NUMBITS(2) []
],
S2NDTR [
    /// NDT
    NDT OFFSET(0) NUMBITS(16) []
],
S2PAR [
    /// PAR
    PAR OFFSET(0) NUMBITS(32) []
],
S2M0AR [
    /// M0A
    M0A OFFSET(0) NUMBITS(32) []
],
S2M1AR [
    /// M1A
    M1A OFFSET(0) NUMBITS(32) []
],
S2FCR [
    /// FTH
    FTH OFFSET(0) NUMBITS(2) [],
    /// DMDIS
    DMDIS OFFSET(2) NUMBITS(1) [],
    /// FS
    FS OFFSET(3) NUMBITS(3) [],
    /// FEIE
    FEIE OFFSET(7) NUMBITS(1) []
],
S3CR [
    /// EN
    EN OFFSET(0) NUMBITS(1) [],
    /// DMEIE
    DMEIE OFFSET(1) NUMBITS(1) [],
    /// TEIE
    TEIE OFFSET(2) NUMBITS(1) [],
    /// HTIE
    HTIE OFFSET(3) NUMBITS(1) [],
    /// TCIE
    TCIE OFFSET(4) NUMBITS(1) [],
    /// PFCTRL
    PFCTRL OFFSET(5) NUMBITS(1) [],
    /// DIR
    DIR OFFSET(6) NUMBITS(2) [],
    /// CIRC
    CIRC OFFSET(8) NUMBITS(1) [],
    /// PINC
    PINC OFFSET(9) NUMBITS(1) [],
    /// MINC
    MINC OFFSET(10) NUMBITS(1) [],
    /// PSIZE
    PSIZE OFFSET(11) NUMBITS(2) [],
    /// MSIZE
    MSIZE OFFSET(13) NUMBITS(2) [],
    /// PINCOS
    PINCOS OFFSET(15) NUMBITS(1) [],
    /// PL
    PL OFFSET(16) NUMBITS(2) [],
    /// DBM
    DBM OFFSET(18) NUMBITS(1) [],
    /// CT
    CT OFFSET(19) NUMBITS(1) [],
    /// PBURST
    PBURST OFFSET(21) NUMBITS(2) [],
    /// MBURST
    MBURST OFFSET(23) NUMBITS(2) []
],
S3NDTR [
    /// NDT
    NDT OFFSET(0) NUMBITS(16) []
],
S3PAR [
    /// PAR
    PAR OFFSET(0) NUMBITS(32) []
],
S3M0AR [
    /// M0A
    M0A OFFSET(0) NUMBITS(32) []
],
S3M1AR [
    /// M1A
    M1A OFFSET(0) NUMBITS(32) []
],
S3FCR [
    /// FTH
    FTH OFFSET(0) NUMBITS(2) [],
    /// DMDIS
    DMDIS OFFSET(2) NUMBITS(1) [],
    /// FS
    FS OFFSET(3) NUMBITS(3) [],
    /// FEIE
    FEIE OFFSET(7) NUMBITS(1) []
],
S4CR [
    /// EN
    EN OFFSET(0) NUMBITS(1) [],
    /// DMEIE
    DMEIE OFFSET(1) NUMBITS(1) [],
    /// TEIE
    TEIE OFFSET(2) NUMBITS(1) [],
    /// HTIE
    HTIE OFFSET(3) NUMBITS(1) [],
    /// TCIE
    TCIE OFFSET(4) NUMBITS(1) [],
    /// PFCTRL
    PFCTRL OFFSET(5) NUMBITS(1) [],
    /// DIR
    DIR OFFSET(6) NUMBITS(2) [],
    /// CIRC
    CIRC OFFSET(8) NUMBITS(1) [],
    /// PINC
    PINC OFFSET(9) NUMBITS(1) [],
    /// MINC
    MINC OFFSET(10) NUMBITS(1) [],
    /// PSIZE
    PSIZE OFFSET(11) NUMBITS(2) [],
    /// MSIZE
    MSIZE OFFSET(13) NUMBITS(2) [],
    /// PINCOS
    PINCOS OFFSET(15) NUMBITS(1) [],
    /// PL
    PL OFFSET(16) NUMBITS(2) [],
    /// DBM
    DBM OFFSET(18) NUMBITS(1) [],
    /// CT
    CT OFFSET(19) NUMBITS(1) [],
    /// PBURST
    PBURST OFFSET(21) NUMBITS(2) [],
    /// MBURST
    MBURST OFFSET(23) NUMBITS(2) []
],
S4NDTR [
    /// NDT
    NDT OFFSET(0) NUMBITS(16) []
],
S4PAR [
    /// PAR
    PAR OFFSET(0) NUMBITS(32) []
],
S4M0AR [
    /// M0A
    M0A OFFSET(0) NUMBITS(32) []
],
S4M1AR [
    /// M1A
    M1A OFFSET(0) NUMBITS(32) []
],
S4FCR [
    /// FTH
    FTH OFFSET(0) NUMBITS(2) [],
    /// DMDIS
    DMDIS OFFSET(2) NUMBITS(1) [],
    /// FS
    FS OFFSET(3) NUMBITS(3) [],
    /// FEIE
    FEIE OFFSET(7) NUMBITS(1) []
],
S5CR [
    /// EN
    EN OFFSET(0) NUMBITS(1) [],
    /// DMEIE
    DMEIE OFFSET(1) NUMBITS(1) [],
    /// TEIE
    TEIE OFFSET(2) NUMBITS(1) [],
    /// HTIE
    HTIE OFFSET(3) NUMBITS(1) [],
    /// TCIE
    TCIE OFFSET(4) NUMBITS(1) [],
    /// PFCTRL
    PFCTRL OFFSET(5) NUMBITS(1) [],
    /// DIR
    DIR OFFSET(6) NUMBITS(2) [],
    /// CIRC
    CIRC OFFSET(8) NUMBITS(1) [],
    /// PINC
    PINC OFFSET(9) NUMBITS(1) [],
    /// MINC
    MINC OFFSET(10) NUMBITS(1) [],
    /// PSIZE
    PSIZE OFFSET(11) NUMBITS(2) [],
    /// MSIZE
    MSIZE OFFSET(13) NUMBITS(2) [],
    /// PINCOS
    PINCOS OFFSET(15) NUMBITS(1) [],
    /// PL
    PL OFFSET(16) NUMBITS(2) [],
    /// DBM
    DBM OFFSET(18) NUMBITS(1) [],
    /// CT
    CT OFFSET(19) NUMBITS(1) [],
    /// PBURST
    PBURST OFFSET(21) NUMBITS(2) [],
    /// MBURST
    MBURST OFFSET(23) NUMBITS(2) []
],
S5NDTR [
    /// NDT
    NDT OFFSET(0) NUMBITS(16) []
],
S5PAR [
    /// PAR
    PAR OFFSET(0) NUMBITS(32) []
],
S5M0AR [
    /// M0A
    M0A OFFSET(0) NUMBITS(32) []
],
S5M1AR [
    /// M1A
    M1A OFFSET(0) NUMBITS(32) []
],
S5FCR [
    /// FTH
    FTH OFFSET(0) NUMBITS(2) [],
    /// DMDIS
    DMDIS OFFSET(2) NUMBITS(1) [],
    /// FS
    FS OFFSET(3) NUMBITS(3) [],
    /// FEIE
    FEIE OFFSET(7) NUMBITS(1) []
],
S6CR [
    /// EN
    EN OFFSET(0) NUMBITS(1) [],
    /// DMEIE
    DMEIE OFFSET(1) NUMBITS(1) [],
    /// TEIE
    TEIE OFFSET(2) NUMBITS(1) [],
    /// HTIE
    HTIE OFFSET(3) NUMBITS(1) [],
    /// TCIE
    TCIE OFFSET(4) NUMBITS(1) [],
    /// PFCTRL
    PFCTRL OFFSET(5) NUMBITS(1) [],
    /// DIR
    DIR OFFSET(6) NUMBITS(2) [],
    /// CIRC
    CIRC OFFSET(8) NUMBITS(1) [],
    /// PINC
    PINC OFFSET(9) NUMBITS(1) [],
    /// MINC
    MINC OFFSET(10) NUMBITS(1) [],
    /// PSIZE
    PSIZE OFFSET(11) NUMBITS(2) [],
    /// MSIZE
    MSIZE OFFSET(13) NUMBITS(2) [],
    /// PINCOS
    PINCOS OFFSET(15) NUMBITS(1) [],
    /// PL
    PL OFFSET(16) NUMBITS(2) [],
    /// DBM
    DBM OFFSET(18) NUMBITS(1) [],
    /// CT
    CT OFFSET(19) NUMBITS(1) [],
    /// PBURST
    PBURST OFFSET(21) NUMBITS(2) [],
    /// MBURST
    MBURST OFFSET(23) NUMBITS(2) []
],
S6NDTR [
    /// NDT
    NDT OFFSET(0) NUMBITS(16) []
],
S6PAR [
    /// PAR
    PAR OFFSET(0) NUMBITS(32) []
],
S6M0AR [
    /// M0A
    M0A OFFSET(0) NUMBITS(32) []
],
S6M1AR [
    /// M1A
    M1A OFFSET(0) NUMBITS(32) []
],
S6FCR [
    /// FTH
    FTH OFFSET(0) NUMBITS(2) [],
    /// DMDIS
    DMDIS OFFSET(2) NUMBITS(1) [],
    /// FS
    FS OFFSET(3) NUMBITS(3) [],
    /// FEIE
    FEIE OFFSET(7) NUMBITS(1) []
],
S7CR [
    /// EN
    EN OFFSET(0) NUMBITS(1) [],
    /// DMEIE
    DMEIE OFFSET(1) NUMBITS(1) [],
    /// TEIE
    TEIE OFFSET(2) NUMBITS(1) [],
    /// HTIE
    HTIE OFFSET(3) NUMBITS(1) [],
    /// TCIE
    TCIE OFFSET(4) NUMBITS(1) [],
    /// PFCTRL
    PFCTRL OFFSET(5) NUMBITS(1) [],
    /// DIR
    DIR OFFSET(6) NUMBITS(2) [],
    /// CIRC
    CIRC OFFSET(8) NUMBITS(1) [],
    /// PINC
    PINC OFFSET(9) NUMBITS(1) [],
    /// MINC
    MINC OFFSET(10) NUMBITS(1) [],
    /// PSIZE
    PSIZE OFFSET(11) NUMBITS(2) [],
    /// MSIZE
    MSIZE OFFSET(13) NUMBITS(2) [],
    /// PINCOS
    PINCOS OFFSET(15) NUMBITS(1) [],
    /// PL
    PL OFFSET(16) NUMBITS(2) [],
    /// DBM
    DBM OFFSET(18) NUMBITS(1) [],
    /// CT
    CT OFFSET(19) NUMBITS(1) [],
    /// PBURST
    PBURST OFFSET(21) NUMBITS(2) [],
    /// MBURST
    MBURST OFFSET(23) NUMBITS(2) []
],
S7NDTR [
    /// NDT
    NDT OFFSET(0) NUMBITS(16) []
],
S7PAR [
    /// PAR
    PAR OFFSET(0) NUMBITS(32) []
],
S7M0AR [
    /// M0A
    M0A OFFSET(0) NUMBITS(32) []
],
S7M1AR [
    /// M1A
    M1A OFFSET(0) NUMBITS(32) []
],
S7FCR [
    /// FTH
    FTH OFFSET(0) NUMBITS(2) [],
    /// DMDIS
    DMDIS OFFSET(2) NUMBITS(1) [],
    /// FS
    FS OFFSET(3) NUMBITS(3) [],
    /// FEIE
    FEIE OFFSET(7) NUMBITS(1) []
],
HWCFGR2 [
    /// FIFO_SIZE
    FIFO_SIZE OFFSET(0) NUMBITS(2) [],
    /// WRITE_BUFFERABLE
    WRITE_BUFFERABLE OFFSET(4) NUMBITS(1) [],
    /// CHSEL_WIDTH
    CHSEL_WIDTH OFFSET(8) NUMBITS(3) []
],
HWCFGR1 [
    /// DEF0
    DEF0 OFFSET(0) NUMBITS(2) [],
    /// DEF1
    DEF1 OFFSET(4) NUMBITS(2) [],
    /// DEF2
    DEF2 OFFSET(8) NUMBITS(2) [],
    /// DEF3
    DEF3 OFFSET(12) NUMBITS(2) [],
    /// DEF4
    DEF4 OFFSET(16) NUMBITS(2) [],
    /// DEF5
    DEF5 OFFSET(20) NUMBITS(2) [],
    /// DEF6
    DEF6 OFFSET(24) NUMBITS(2) [],
    /// DEF7
    DEF7 OFFSET(28) NUMBITS(2) []
],
VERR [
    /// MINREV
    MINREV OFFSET(0) NUMBITS(4) [],
    /// MAJREV
    MAJREV OFFSET(4) NUMBITS(4) []
],
IPDR [
    /// ID
    ID OFFSET(0) NUMBITS(32) []
],
SIDR [
    /// SID
    SID OFFSET(0) NUMBITS(32) []
]
];
const DMA1_BASE: StaticRef<Dma1Registers> =
    unsafe { StaticRef::new(0x48000000 as *const Dma1Registers) };

/// The DMA stream number. What other microcontrollers refer to as "channel",
/// STM32F446RE refers to as "streams". STM32F446RE has eight streams. A stream
/// transfers data between memory and peripheral.
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum StreamId {
    Stream0 = 0,
    Stream1 = 1,
    Stream2 = 2,
    Stream3 = 3,
    Stream4 = 4,
    Stream5 = 5,
    Stream6 = 6,
    Stream7 = 7,
}

/// Each stream can be selected among up to eight channel requests. This is
/// basically STM32F446RE's way of selecting the peripheral for the stream.
/// Nevertheless, the use of the term channel here is confusing. Table 28
/// describes the mapping between stream, channel, and peripherals.
#[allow(dead_code)]
#[repr(u32)]
enum ChannelId {
    Channel0 = 0b000,
    Channel1 = 0b001,
    Channel2 = 0b010,
    Channel3 = 0b011,
    Channel4 = 0b100,
    Channel5 = 0b101,
    Channel6 = 0b110,
    Channel7 = 0b111,
}

/// DMA transfer direction. Section 9.5.5
#[allow(dead_code)]
#[repr(u32)]
enum Direction {
    PeripheralToMemory = 0b00,
    MemoryToPeripheral = 0b01,
    MemoryToMemory = 0b10,
}

/// DMA data size. Section 9.5.5
#[allow(dead_code)]
#[repr(u32)]
enum Size {
    Byte = 0b00,
    HalfWord = 0b01,
    Word = 0b10,
}

struct Msize(Size);
struct Psize(Size);

/// DMA transfer mode. Section 9.5.10
#[allow(dead_code)]
#[repr(u32)]
enum FifoSize {
    Quarter = 0b00,
    Half = 0b01,
    ThreeFourths = 0b10,
    Full = 0b11,
}

#[allow(dead_code)]
enum TransferMode {
    Direct,
    Fifo(FifoSize),
}

/// List of peripherals managed by DMA1
#[allow(non_camel_case_types, non_snake_case)]
#[derive(Copy, Clone, PartialEq)]
pub enum Dma1Peripheral {
    USART2_TX,
    USART2_RX,
    USART3_TX,
    USART3_RX,
    // SPI3_TX,
    // SPI3_RX,
}

impl Dma1Peripheral {
    // Returns the IRQ number of the stream associated with the peripheral. Used
    // to enable interrupt on the NVIC.
    pub fn get_stream_irqn(&self) -> u32 {
        match self {
            // Dma1Peripheral::SPI3_TX => nvic::DMA1_Stream7,
            Dma1Peripheral::USART2_TX => nvic::DMA1_Stream6,
            Dma1Peripheral::USART2_RX => nvic::DMA1_Stream5,
            Dma1Peripheral::USART3_TX => nvic::DMA1_Stream3,
            // Dma1Peripheral::SPI3_RX => nvic::DMA1_Stream2,
            Dma1Peripheral::USART3_RX => nvic::DMA1_Stream1,
        }
    }

    pub fn get_stream_idx<'a>(&self) -> usize {
        usize::from(StreamId::from(*self) as u8)
    }
}

impl From<Dma1Peripheral> for StreamId {
    fn from(pid: Dma1Peripheral) -> StreamId {
        match pid {
            // Dma1Peripheral::SPI3_TX => StreamId::Stream7,
            Dma1Peripheral::USART2_TX => StreamId::Stream6,
            Dma1Peripheral::USART2_RX => StreamId::Stream5,
            Dma1Peripheral::USART3_TX => StreamId::Stream3,
            // Dma1Peripheral::SPI3_RX => StreamId::Stream2,
            Dma1Peripheral::USART3_RX => StreamId::Stream1,
        }
    }
}

pub struct Stream<'a> {
    streamid: StreamId,
    client: OptionalCell<&'a dyn StreamClient>,
    buffer: TakeCell<'static, [u8]>,
    peripheral: OptionalCell<Dma1Peripheral>,
    dma1: &'a Dma1,
}

pub fn new_dma1_stream<'a>(dma: &'a Dma1) -> [Stream<'a>; 8] {
    [
        Stream::new(StreamId::Stream0, dma),
        Stream::new(StreamId::Stream1, dma),
        Stream::new(StreamId::Stream2, dma),
        Stream::new(StreamId::Stream3, dma),
        Stream::new(StreamId::Stream4, dma),
        Stream::new(StreamId::Stream5, dma),
        Stream::new(StreamId::Stream6, dma),
        Stream::new(StreamId::Stream7, dma),
    ]
}

pub trait StreamClient {
    fn transfer_done(&self, pid: Dma1Peripheral);
}

impl<'a> Stream<'a> {
    const fn new(streamid: StreamId, dma1: &'a Dma1) -> Self {
        Self {
            streamid: streamid,
            buffer: TakeCell::empty(),
            client: OptionalCell::empty(),
            peripheral: OptionalCell::empty(),
            dma1,
        }
    }

    pub fn set_client(&self, client: &'a dyn StreamClient) {
        self.client.set(client);
    }

    pub fn handle_interrupt(&self) {
        self.clear_transfer_complete_flag();

        self.client.map(|client| {
            self.peripheral.map(|pid| {
                client.transfer_done(*pid);
            });
        });
    }

    pub fn setup(&self, pid: Dma1Peripheral) {
        self.peripheral.set(pid);

        // Setup is called before interrupts are enabled on the NVIC
        self.disable_interrupt();
        self.disable();

        // The numbers below are from Section 1.2 of AN4031. It looks like these
        // settings can be set only once. Trying to set them again, seems to
        // generate a hard-fault even when the stream is disabled.
        //
        // 8
        self.set_transfer_mode_for_peripheral();
        // 9
        self.set_data_width_for_peripheral();
    }

    pub fn do_transfer(&self, buf: &'static mut [u8], len: usize) {
        self.disable_interrupt();

        // The numbers below are from Section 1.2 of AN4031
        //
        // NOTE: We only clear TC flag here. Trying to clear any other flag,
        //       generates a hard-fault
        // 1
        self.disable();
        self.clear_transfer_complete_flag();
        // 2
        self.set_peripheral_address();
        // 3
        self.set_memory_address(&buf[0] as *const u8 as u32);
        // 4
        self.set_data_items(len as u32);
        // 5
        // self.set_channel();
        // 9
        self.set_direction();
        self.set_peripheral_address_increment();
        self.set_memory_address_increment();
        self.interrupt_enable();
        // 10
        self.enable();

        // NOTE: We still have to enable DMA on the peripheral side
        self.buffer.replace(buf);
    }

    pub fn abort_transfer(&self) -> (Option<&'static mut [u8]>, u32) {
        self.disable_interrupt();

        self.disable();

        (self.buffer.take(), self.get_data_items())
    }

    pub fn return_buffer(&self) -> Option<&'static mut [u8]> {
        self.buffer.take()
    }

    // fn set_channel(&self) {
    //     self.peripheral.map(|pid| {
    //         match pid {
    //             Dma1Peripheral::SPI3_TX => {
    //                 // SPI3_RX Stream 7, Channel 0
    //                 self.dma1
    //                     .registers
    //                     .s7cr
    //                     .modify(S7CR::CHSEL.val(ChannelId::Channel0 as u32));
    //             }
    //             Dma1Peripheral::USART2_TX => {
    //                 // USART2_TX Stream 6, Channel 4
    //                 self.dma1
    //                     .registers
    //                     .s6cr
    //                     .modify(S6CR::CHSEL.val(ChannelId::Channel4 as u32));
    //             }
    //             Dma1Peripheral::USART2_RX => {
    //                 // USART2_RX Stream 5, Channel 4
    //                 self.dma1
    //                     .registers
    //                     .s5cr
    //                     .modify(S5CR::CHSEL.val(ChannelId::Channel4 as u32));
    //             }
    //             Dma1Peripheral::USART3_TX => {
    //                 // USART3_TX Stream 3, Channel 4
    //                 self.dma1
    //                     .registers
    //                     .s3cr
    //                     .modify(S3CR::CHSEL.val(ChannelId::Channel4 as u32));
    //             }
    //             Dma1Peripheral::SPI3_RX => {
    //                 // SPI3_RX Stream 2, Channel 0
    //                 self.dma1
    //                     .registers
    //                     .s2cr
    //                     .modify(S2CR::CHSEL.val(ChannelId::Channel0 as u32));
    //             }
    //             Dma1Peripheral::USART3_RX => {
    //                 // USART3_RX Stream 1, Channel 4
    //                 self.dma1
    //                     .registers
    //                     .s1cr
    //                     .modify(S1CR::CHSEL.val(ChannelId::Channel4 as u32));
    //             }
    //         }
    //     });
    // }

    fn set_direction(&self) {
        self.peripheral.map(|pid| {
            match pid {
                // Dma1Peripheral::SPI3_TX => {
                //     // SPI3_TX Stream 7
                //     self.dma1
                //         .registers
                //         .s7cr
                //         .modify(S7CR::DIR.val(Direction::MemoryToPeripheral as u32));
                // }
                Dma1Peripheral::USART2_TX => {
                    // USART2_TX Stream 6
                    self.dma1
                        .registers
                        .s6cr
                        .modify(S6CR::DIR.val(Direction::MemoryToPeripheral as u32));
                }
                Dma1Peripheral::USART2_RX => {
                    // USART2_RX Stream 5
                    self.dma1
                        .registers
                        .s5cr
                        .modify(S5CR::DIR.val(Direction::PeripheralToMemory as u32));
                }
                Dma1Peripheral::USART3_TX => {
                    // USART3_TX Stream 3
                    self.dma1
                        .registers
                        .s3cr
                        .modify(S3CR::DIR.val(Direction::MemoryToPeripheral as u32));
                }
                // Dma1Peripheral::SPI3_RX => {
                //     // SPI3_RX Stream 2
                //     self.dma1
                //         .registers
                //         .s2cr
                //         .modify(S2CR::DIR.val(Direction::PeripheralToMemory as u32));
                // }
                Dma1Peripheral::USART3_RX => {
                    // USART3_RX Stream 1
                    self.dma1
                        .registers
                        .s1cr
                        .modify(S1CR::DIR.val(Direction::PeripheralToMemory as u32));
                }
            }
        });
    }

    fn set_peripheral_address(&self) {
        self.peripheral.map(|pid| {
            match pid {
                // Dma1Peripheral::SPI3_TX => {
                //     // SPI3_TX Stream 7
                //     self.dma1
                //         .registers
                //         .s7par
                //         .set(spi::get_address_dr(spi::SPI3_BASE));
                // }
                Dma1Peripheral::USART2_TX => {
                    // USART2_TX Stream 6
                    self.dma1
                        .registers
                        .s6par
                        .set(usart::get_address_dr(usart::USART2_BASE));
                }
                Dma1Peripheral::USART2_RX => {
                    // USART2_RX Stream 5
                    self.dma1
                        .registers
                        .s5par
                        .set(usart::get_address_dr(usart::USART2_BASE));
                }
                Dma1Peripheral::USART3_TX => {
                    // USART3_TX Stream 3
                    self.dma1
                        .registers
                        .s3par
                        .set(usart::get_address_dr(usart::USART3_BASE));
                }
                // Dma1Peripheral::SPI3_RX => {
                //     // SPI3_RX Stream 2
                //     self.dma1
                //         .registers
                //         .s2par
                //         .set(spi::get_address_dr(spi::SPI3_BASE));
                // }
                Dma1Peripheral::USART3_RX => {
                    // USART3_RX Stream 1
                    self.dma1
                        .registers
                        .s1par
                        .set(usart::get_address_dr(usart::USART3_BASE));
                }
            }
        });
    }

    fn set_peripheral_address_increment(&self) {
        self.peripheral.map(|pid| {
            match pid {
                // Dma1Peripheral::SPI3_TX => {
                //     // SPI3_TX Stream 7
                //     self.dma1.registers.s7cr.modify(S7CR::PINC::CLEAR);
                // }
                Dma1Peripheral::USART2_TX => {
                    // USART2_TX Stream 6
                    self.dma1.registers.s6cr.modify(S6CR::PINC::CLEAR);
                }
                Dma1Peripheral::USART2_RX => {
                    // USART2_RX Stream 5
                    self.dma1.registers.s5cr.modify(S5CR::PINC::CLEAR);
                }
                Dma1Peripheral::USART3_TX => {
                    // USART3_TX Stream 3
                    self.dma1.registers.s3cr.modify(S3CR::PINC::CLEAR);
                }
                // Dma1Peripheral::SPI3_RX => {
                //     // SPI3_RX Stream 2
                //     self.dma1.registers.s2cr.modify(S2CR::PINC::CLEAR);
                // }
                Dma1Peripheral::USART3_RX => {
                    // USART3_RX Stream 1
                    self.dma1.registers.s1cr.modify(S1CR::PINC::CLEAR);
                }
            }
        });
    }

    fn set_memory_address(&self, buf_addr: u32) {
        self.peripheral.map(|pid| {
            match pid {
                // Dma1Peripheral::SPI3_TX => {
                //     // SPI3_TX Stream 7
                //     self.dma1.registers.s7m0ar.set(buf_addr);
                // }
                Dma1Peripheral::USART2_TX => {
                    // USART2_TX Stream 6
                    self.dma1.registers.s6m0ar.set(buf_addr);
                }
                Dma1Peripheral::USART2_RX => {
                    // USART2_RX Stream 5
                    self.dma1.registers.s5m0ar.set(buf_addr);
                }
                Dma1Peripheral::USART3_TX => {
                    // USART3_TX Stream 3
                    self.dma1.registers.s3m0ar.set(buf_addr);
                }
                // Dma1Peripheral::SPI3_RX => {
                //     // SPI3_RX Stream 2
                //     self.dma1.registers.s2m0ar.set(buf_addr);
                // }
                Dma1Peripheral::USART3_RX => {
                    // USART3_RX Stream 1
                    self.dma1.registers.s1m0ar.set(buf_addr);
                }
            }
        });
    }

    fn set_memory_address_increment(&self) {
        self.peripheral.map(|pid| {
            match pid {
                // Dma1Peripheral::SPI3_TX => {
                //     // SPI3_TX Stream 7
                //     self.dma1.registers.s7cr.modify(S7CR::MINC::SET);
                // }
                Dma1Peripheral::USART2_TX => {
                    // USART2_TX Stream 6
                    self.dma1.registers.s6cr.modify(S6CR::MINC::SET);
                }
                Dma1Peripheral::USART2_RX => {
                    // USART2_RX Stream 5
                    self.dma1.registers.s5cr.modify(S5CR::MINC::SET);
                }
                Dma1Peripheral::USART3_TX => {
                    // USART3_TX Stream 3
                    self.dma1.registers.s3cr.modify(S3CR::MINC::SET);
                }
                // Dma1Peripheral::SPI3_RX => {
                //     // SPI3_RX Stream 2
                //     self.dma1.registers.s2cr.modify(S2CR::MINC::SET);
                // }
                Dma1Peripheral::USART3_RX => {
                    // USART3_RX Stream 1
                    self.dma1.registers.s1cr.modify(S1CR::MINC::SET);
                }
            }
        });
    }

    fn get_data_items(&self) -> u32 {
        match self.streamid {
            StreamId::Stream0 => self.dma1.registers.s0ndtr.get(),
            StreamId::Stream1 => self.dma1.registers.s1ndtr.get(),
            StreamId::Stream2 => self.dma1.registers.s2ndtr.get(),
            StreamId::Stream3 => self.dma1.registers.s3ndtr.get(),
            StreamId::Stream4 => self.dma1.registers.s4ndtr.get(),
            StreamId::Stream5 => self.dma1.registers.s5ndtr.get(),
            StreamId::Stream6 => self.dma1.registers.s6ndtr.get(),
            StreamId::Stream7 => self.dma1.registers.s7ndtr.get(),
        }
    }

    fn set_data_items(&self, data_items: u32) {
        match self.streamid {
            StreamId::Stream0 => {
                self.dma1.registers.s0ndtr.set(data_items);
            }
            StreamId::Stream1 => {
                self.dma1.registers.s1ndtr.set(data_items);
            }
            StreamId::Stream2 => {
                self.dma1.registers.s2ndtr.set(data_items);
            }
            StreamId::Stream3 => {
                self.dma1.registers.s3ndtr.set(data_items);
            }
            StreamId::Stream4 => {
                self.dma1.registers.s4ndtr.set(data_items);
            }
            StreamId::Stream5 => {
                self.dma1.registers.s5ndtr.set(data_items);
            }
            StreamId::Stream6 => {
                self.dma1.registers.s6ndtr.set(data_items);
            }
            StreamId::Stream7 => {
                self.dma1.registers.s7ndtr.set(data_items);
            }
        }
    }

    fn set_data_width_for_peripheral(&self) {
        self.peripheral.map(|pid| match pid {
            // Dma1Peripheral::SPI3_TX => {
            //     self.stream_set_data_width(Msize(Size::Byte), Psize(Size::Byte))
            // }
            Dma1Peripheral::USART2_TX => {
                self.stream_set_data_width(Msize(Size::Byte), Psize(Size::Byte))
            }
            Dma1Peripheral::USART2_RX => {
                self.stream_set_data_width(Msize(Size::Byte), Psize(Size::Byte))
            }
            Dma1Peripheral::USART3_TX => {
                self.stream_set_data_width(Msize(Size::Byte), Psize(Size::Byte))
            }
            // Dma1Peripheral::SPI3_RX => {
            //     self.stream_set_data_width(Msize(Size::Byte), Psize(Size::Byte))
            // }
            Dma1Peripheral::USART3_RX => {
                self.stream_set_data_width(Msize(Size::Byte), Psize(Size::Byte))
            }
        });
    }

    fn stream_set_data_width(&self, msize: Msize, psize: Psize) {
        match self.streamid {
            StreamId::Stream0 => {
                self.dma1
                    .registers
                    .s0cr
                    .modify(S0CR::PSIZE.val(psize.0 as u32));
                self.dma1
                    .registers
                    .s0cr
                    .modify(S0CR::MSIZE.val(msize.0 as u32));
            }
            StreamId::Stream1 => {
                self.dma1
                    .registers
                    .s1cr
                    .modify(S1CR::PSIZE.val(psize.0 as u32));
                self.dma1
                    .registers
                    .s1cr
                    .modify(S1CR::MSIZE.val(msize.0 as u32));
            }
            StreamId::Stream2 => {
                self.dma1
                    .registers
                    .s2cr
                    .modify(S2CR::PSIZE.val(psize.0 as u32));
                self.dma1
                    .registers
                    .s2cr
                    .modify(S2CR::MSIZE.val(msize.0 as u32));
            }
            StreamId::Stream3 => {
                self.dma1
                    .registers
                    .s3cr
                    .modify(S3CR::PSIZE.val(psize.0 as u32));
                self.dma1
                    .registers
                    .s3cr
                    .modify(S3CR::MSIZE.val(msize.0 as u32));
            }
            StreamId::Stream4 => {
                self.dma1
                    .registers
                    .s4cr
                    .modify(S4CR::PSIZE.val(psize.0 as u32));
                self.dma1
                    .registers
                    .s4cr
                    .modify(S4CR::MSIZE.val(msize.0 as u32));
            }
            StreamId::Stream5 => {
                self.dma1
                    .registers
                    .s5cr
                    .modify(S5CR::PSIZE.val(psize.0 as u32));
                self.dma1
                    .registers
                    .s5cr
                    .modify(S5CR::MSIZE.val(msize.0 as u32));
            }
            StreamId::Stream6 => {
                self.dma1
                    .registers
                    .s6cr
                    .modify(S6CR::PSIZE.val(psize.0 as u32));
                self.dma1
                    .registers
                    .s6cr
                    .modify(S6CR::MSIZE.val(msize.0 as u32));
            }
            StreamId::Stream7 => {
                self.dma1
                    .registers
                    .s7cr
                    .modify(S7CR::PSIZE.val(psize.0 as u32));
                self.dma1
                    .registers
                    .s7cr
                    .modify(S7CR::MSIZE.val(msize.0 as u32));
            }
        }
    }

    fn set_transfer_mode_for_peripheral(&self) {
        self.peripheral.map(|pid| match pid {
            // Dma1Peripheral::SPI3_TX => {
            //     self.stream_set_transfer_mode(TransferMode::Fifo(FifoSize::Full));
            // }
            Dma1Peripheral::USART2_TX => {
                self.stream_set_transfer_mode(TransferMode::Fifo(FifoSize::Full));
            }
            Dma1Peripheral::USART2_RX => {
                self.stream_set_transfer_mode(TransferMode::Fifo(FifoSize::Full));
            }
            Dma1Peripheral::USART3_TX => {
                self.stream_set_transfer_mode(TransferMode::Fifo(FifoSize::Full));
            }
            // Dma1Peripheral::SPI3_RX => {
            //     self.stream_set_transfer_mode(TransferMode::Fifo(FifoSize::Full));
            // }
            Dma1Peripheral::USART3_RX => {
                self.stream_set_transfer_mode(TransferMode::Fifo(FifoSize::Full));
            }
        });
    }

    fn stream_set_transfer_mode(&self, transfer_mode: TransferMode) {
        match self.streamid {
            StreamId::Stream0 => match transfer_mode {
                TransferMode::Direct => {
                    self.dma1.registers.s0fcr.modify(S0FCR::DMDIS::CLEAR);
                }
                TransferMode::Fifo(s) => {
                    self.dma1.registers.s0fcr.modify(S0FCR::DMDIS::SET);
                    self.dma1.registers.s0fcr.modify(S0FCR::FTH.val(s as u32));
                }
            },
            StreamId::Stream1 => match transfer_mode {
                TransferMode::Direct => {
                    self.dma1.registers.s1fcr.modify(S1FCR::DMDIS::CLEAR);
                }
                TransferMode::Fifo(s) => {
                    self.dma1.registers.s1fcr.modify(S1FCR::DMDIS::SET);
                    self.dma1.registers.s1fcr.modify(S1FCR::FTH.val(s as u32));
                }
            },
            StreamId::Stream2 => match transfer_mode {
                TransferMode::Direct => {
                    self.dma1.registers.s2fcr.modify(S2FCR::DMDIS::CLEAR);
                }
                TransferMode::Fifo(s) => {
                    self.dma1.registers.s2fcr.modify(S2FCR::DMDIS::SET);
                    self.dma1.registers.s2fcr.modify(S2FCR::FTH.val(s as u32));
                }
            },
            StreamId::Stream3 => match transfer_mode {
                TransferMode::Direct => {
                    self.dma1.registers.s3fcr.modify(S3FCR::DMDIS::CLEAR);
                }
                TransferMode::Fifo(s) => {
                    self.dma1.registers.s3fcr.modify(S3FCR::DMDIS::SET);
                    self.dma1.registers.s3fcr.modify(S3FCR::FTH.val(s as u32));
                }
            },
            StreamId::Stream4 => match transfer_mode {
                TransferMode::Direct => {
                    self.dma1.registers.s4fcr.modify(S4FCR::DMDIS::CLEAR);
                }
                TransferMode::Fifo(s) => {
                    self.dma1.registers.s4fcr.modify(S4FCR::DMDIS::SET);
                    self.dma1.registers.s4fcr.modify(S4FCR::FTH.val(s as u32));
                }
            },
            StreamId::Stream5 => match transfer_mode {
                TransferMode::Direct => {
                    self.dma1.registers.s5fcr.modify(S5FCR::DMDIS::CLEAR);
                }
                TransferMode::Fifo(s) => {
                    self.dma1.registers.s5fcr.modify(S5FCR::DMDIS::SET);
                    self.dma1.registers.s5fcr.modify(S5FCR::FTH.val(s as u32));
                }
            },
            StreamId::Stream6 => match transfer_mode {
                TransferMode::Direct => {
                    self.dma1.registers.s6fcr.modify(S6FCR::DMDIS::CLEAR);
                }
                TransferMode::Fifo(s) => {
                    self.dma1.registers.s6fcr.modify(S6FCR::DMDIS::SET);
                    self.dma1.registers.s6fcr.modify(S6FCR::FTH.val(s as u32));
                }
            },
            StreamId::Stream7 => match transfer_mode {
                TransferMode::Direct => {
                    self.dma1.registers.s7fcr.modify(S7FCR::DMDIS::CLEAR);
                }
                TransferMode::Fifo(s) => {
                    self.dma1.registers.s7fcr.modify(S7FCR::DMDIS::SET);
                    self.dma1.registers.s7fcr.modify(S7FCR::FTH.val(s as u32));
                }
            },
        }
    }

    fn enable(&self) {
        match self.streamid {
            StreamId::Stream0 => self.dma1.registers.s0cr.modify(S0CR::EN::SET),
            StreamId::Stream1 => self.dma1.registers.s1cr.modify(S1CR::EN::SET),
            StreamId::Stream2 => self.dma1.registers.s2cr.modify(S2CR::EN::SET),
            StreamId::Stream3 => self.dma1.registers.s3cr.modify(S3CR::EN::SET),
            StreamId::Stream4 => self.dma1.registers.s4cr.modify(S4CR::EN::SET),
            StreamId::Stream5 => self.dma1.registers.s5cr.modify(S5CR::EN::SET),
            StreamId::Stream6 => self.dma1.registers.s6cr.modify(S6CR::EN::SET),
            StreamId::Stream7 => self.dma1.registers.s7cr.modify(S7CR::EN::SET),
        }
    }

    fn disable(&self) {
        match self.streamid {
            StreamId::Stream0 => self.dma1.registers.s0cr.modify(S0CR::EN::CLEAR),
            StreamId::Stream1 => self.dma1.registers.s1cr.modify(S1CR::EN::CLEAR),
            StreamId::Stream2 => self.dma1.registers.s2cr.modify(S2CR::EN::CLEAR),
            StreamId::Stream3 => self.dma1.registers.s3cr.modify(S3CR::EN::CLEAR),
            StreamId::Stream4 => self.dma1.registers.s4cr.modify(S4CR::EN::CLEAR),
            StreamId::Stream5 => self.dma1.registers.s5cr.modify(S5CR::EN::CLEAR),
            StreamId::Stream6 => self.dma1.registers.s6cr.modify(S6CR::EN::CLEAR),
            StreamId::Stream7 => self.dma1.registers.s7cr.modify(S7CR::EN::CLEAR),
        }
    }

    fn clear_transfer_complete_flag(&self) {
        match self.streamid {
            StreamId::Stream0 => {
                self.dma1.registers.lifcr.write(LIFCR::CTCIF0::SET);
            }
            StreamId::Stream1 => {
                self.dma1.registers.lifcr.write(LIFCR::CTCIF1::SET);
            }
            StreamId::Stream2 => {
                self.dma1.registers.lifcr.write(LIFCR::CTCIF2::SET);
            }
            StreamId::Stream3 => {
                self.dma1.registers.lifcr.write(LIFCR::CTCIF3::SET);
            }
            StreamId::Stream4 => {
                self.dma1.registers.hifcr.write(HIFCR::CTCIF4::SET);
            }
            StreamId::Stream5 => {
                self.dma1.registers.hifcr.write(HIFCR::CTCIF5::SET);
            }
            StreamId::Stream6 => {
                self.dma1.registers.hifcr.write(HIFCR::CTCIF6::SET);
            }
            StreamId::Stream7 => {
                self.dma1.registers.hifcr.write(HIFCR::CTCIF7::SET);
            }
        }
    }

    // We only interrupt on TC (Transfer Complete)
    fn interrupt_enable(&self) {
        match self.streamid {
            StreamId::Stream0 => self.dma1.registers.s0cr.modify(S0CR::TCIE::SET),
            StreamId::Stream1 => self.dma1.registers.s1cr.modify(S1CR::TCIE::SET),
            StreamId::Stream2 => self.dma1.registers.s2cr.modify(S2CR::TCIE::SET),
            StreamId::Stream3 => self.dma1.registers.s3cr.modify(S3CR::TCIE::SET),
            StreamId::Stream4 => self.dma1.registers.s4cr.modify(S4CR::TCIE::SET),
            StreamId::Stream5 => self.dma1.registers.s5cr.modify(S5CR::TCIE::SET),
            StreamId::Stream6 => self.dma1.registers.s6cr.modify(S6CR::TCIE::SET),
            StreamId::Stream7 => self.dma1.registers.s7cr.modify(S7CR::TCIE::SET),
        }
    }

    // We only interrupt on TC (Transfer Complete)
    fn disable_interrupt(&self) {
        match self.streamid {
            StreamId::Stream0 => self.dma1.registers.s0cr.modify(S0CR::TCIE::CLEAR),
            StreamId::Stream1 => self.dma1.registers.s1cr.modify(S1CR::TCIE::CLEAR),
            StreamId::Stream2 => self.dma1.registers.s2cr.modify(S2CR::TCIE::CLEAR),
            StreamId::Stream3 => self.dma1.registers.s3cr.modify(S3CR::TCIE::CLEAR),
            StreamId::Stream4 => self.dma1.registers.s4cr.modify(S4CR::TCIE::CLEAR),
            StreamId::Stream5 => self.dma1.registers.s5cr.modify(S5CR::TCIE::CLEAR),
            StreamId::Stream6 => self.dma1.registers.s6cr.modify(S6CR::TCIE::CLEAR),
            StreamId::Stream7 => self.dma1.registers.s7cr.modify(S7CR::TCIE::CLEAR),
        }
    }
}

pub struct Dma1 {
    registers: StaticRef<Dma1Registers>,
}

impl Dma1 {
    pub const fn new() -> Dma1 {
        Dma1 {
            registers: DMA1_BASE,
        }
    }
}
