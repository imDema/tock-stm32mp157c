use core::marker::PhantomData;

use kernel::platform::chip::ClockInterface;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};
use kernel::utilities::registers::{register_bitfields, register_structs, ReadWrite};
use kernel::utilities::StaticRef;

// use kernel::common::StaticRef;
// use kernel::common::registers::{self, register_bitfields, register_structs, ReadOnly, ReadWrite, WriteOnly};
    
// register_structs! {
//     /// RCC
//     RccRegisters {
//         /// This register is used to switch the RCC into secure mode. This register can only
//         (0x000 => tzcr: ReadWrite<u32, TZCR::Register>),
//         (0x004 => _reserved0),
//         /// This register is used to control the oscillators.Writing  to this register has n
//         (0x00C => ocensetr: ReadWrite<u32, OCENSETR::Register>),
//         /// This register is used to control the oscillators.Writing  to this register has n
//         (0x010 => ocenclrr: ReadWrite<u32, OCENCLRR::Register>),
//         (0x014 => _reserved1),
//         /// This register is used to configure the HSI. If TZEN = , this register can only b
//         (0x018 => hsicfgr: ReadWrite<u32, HSICFGR::Register>),
//         /// This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this
//         (0x01C => csicfgr: ReadWrite<u32, CSICFGR::Register>),
//         /// This register is used to select the clock source for the MPU. If TZEN = , this r
//         (0x020 => mpckselr: ReadWrite<u32, MPCKSELR::Register>),
//         /// This register is used to select the clock source for the AXI sub-system. If TZEN
//         (0x024 => assckselr: ReadWrite<u32, ASSCKSELR::Register>),
//         /// This register is used to select the reference clock for PLL1 and PLL2. If TZEN =
//         (0x028 => rck12selr: ReadWrite<u32, RCK12SELR::Register>),
//         /// This register is used to control the MPU clock prescaler. Refer to Section: Sub-
//         (0x02C => mpckdivr: ReadWrite<u32, MPCKDIVR::Register>),
//         /// This register is used to control the AXI Matrix clock prescaler. Refer to Sectio
//         (0x030 => axidivr: ReadWrite<u32, AXIDIVR::Register>),
//         (0x034 => _reserved2),
//         /// This register is used to control the APB4 clock divider. Refer to Section: Sub-s
//         (0x03C => apb4divr: ReadWrite<u32, APB4DIVR::Register>),
//         /// This register is used to control the APB5 clock divider. Refer to Section: Sub-s
//         (0x040 => apb5divr: ReadWrite<u32, APB5DIVR::Register>),
//         /// This register is used to divide the HSE clock for RTC. If TZEN = , this register
//         (0x044 => rtcdivr: ReadWrite<u32>),
//         /// This register is used to select the clock source for the MCU sub-system, includi
//         (0x048 => mssckselr: ReadWrite<u32, MSSCKSELR::Register>),
//         (0x04C => _reserved3),
//         /// This register is used to control the PLL1. If TZEN = , this register can only be
//         (0x080 => pll1cr: ReadWrite<u32, PLL1CR::Register>),
//         /// This register is used to configure the PLL1. If TZEN = , this register can only
//         (0x084 => pll1cfgr1: ReadWrite<u32, PLL1CFGR1::Register>),
//         /// This register is used to configure the PLL1. If TZEN = , this register can only
//         (0x088 => pll1cfgr2: ReadWrite<u32, PLL1CFGR2::Register>),
//         /// This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , th
//         (0x08C => pll1fracr: ReadWrite<u32, PLL1FRACR::Register>),
//         /// This register is used to configure the PLL1.It is not recommended to change the
//         (0x090 => pll1csgr: ReadWrite<u32, PLL1CSGR::Register>),
//         /// This register is used to control the PLL2. If TZEN = , this register can only be
//         (0x094 => pll2cr: ReadWrite<u32, PLL2CR::Register>),
//         /// This register is used to configure the PLL2. If TZEN = , this register can only
//         (0x098 => pll2cfgr1: ReadWrite<u32, PLL2CFGR1::Register>),
//         /// This register is used to configure the PLL2. If TZEN = , this register can only
//         (0x09C => pll2cfgr2: ReadWrite<u32, PLL2CFGR2::Register>),
//         /// This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , th
//         (0x0A0 => pll2fracr: ReadWrite<u32, PLL2FRACR::Register>),
//         /// This register is used to configure the PLL2. It is not recommended to change the
//         (0x0A4 => pll2csgr: ReadWrite<u32, PLL2CSGR::Register>),
//         (0x0A8 => _reserved4),
//         /// This register is used to control the selection of the kernel clock for the I2C4
//         (0x0C0 => i2c46ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the SPI6.
//         (0x0C4 => spi6ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the USART
//         (0x0C8 => uart1ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the RNG1.
//         (0x0CC => rng1ckselr: ReadWrite<u32>),
//         /// This register is used to select an oscillator source as kernel clock for the per
//         (0x0D0 => cperckselr: ReadWrite<u32>),
//         /// This register is used to select the peripheral clock for the STGEN block. Note t
//         (0x0D4 => stgenckselr: ReadWrite<u32>),
//         /// This register is used to control the DDR interface, including the DDRC and DDRPH
//         (0x0D8 => ddritfcr: ReadWrite<u32, DDRITFCR::Register>),
//         (0x0DC => _reserved5),
//         /// This register is used to control the HOLD boot function when the system exits fr
//         (0x100 => mp_bootcr: ReadWrite<u32, MP_BOOTCR::Register>),
//         /// Writing  has no effect, reading will return the values of the bits. Writing a  s
//         (0x104 => mp_sreqsetr: ReadWrite<u32, MP_SREQSETR::Register>),
//         /// Writing  has no effect, reading will return the effective values of the bits. Wr
//         (0x108 => mp_sreqclrr: ReadWrite<u32, MP_SREQCLRR::Register>),
//         /// The register contains global control bits. If TZEN = , this register can only be
//         (0x10C => mp_gcr: ReadWrite<u32>),
//         /// This register is used to control the behavior of the warm reset. If TZEN = , thi
//         (0x110 => mp_aprstcr: ReadWrite<u32, MP_APRSTCR::Register>),
//         /// This register provides a status of the RDCTL. If TZEN = , this register can only
//         (0x114 => mp_aprstsr: ReadOnly<u32>),
//         (0x118 => _reserved6),
//         /// This register is used to control the LSE function. Wait states are inserted in c
//         (0x140 => bdcr: ReadWrite<u32, BDCR::Register>),
//         /// This register is used to control the minimum NRST active duration and LSI functi
//         (0x144 => rdlsicr: ReadWrite<u32, RDLSICR::Register>),
//         (0x148 => _reserved7),
//         /// This register is used to activate the reset of the corresponding peripheral. Wri
//         (0x180 => apb4rstsetr: ReadWrite<u32, APB4RSTSETR::Register>),
//         /// This register is used to release the reset of the corresponding peripheral. Writ
//         (0x184 => apb4rstclrr: ReadWrite<u32, APB4RSTCLRR::Register>),
//         /// This register is used to activate the reset of the corresponding peripheral. Wri
//         (0x188 => apb5rstsetr: ReadWrite<u32, APB5RSTSETR::Register>),
//         /// This register is used to release the reset of the corresponding peripheral. Writ
//         (0x18C => apb5rstclrr: ReadWrite<u32, APB5RSTCLRR::Register>),
//         /// This register is used to activate the reset of the corresponding peripheral. Wri
//         (0x190 => ahb5rstsetr: ReadWrite<u32, AHB5RSTSETR::Register>),
//         /// This register is used to release the reset of the corresponding peripheral. Writ
//         (0x194 => ahb5rstclrr: ReadWrite<u32, AHB5RSTCLRR::Register>),
//         /// This register is used to activate the reset of the corresponding peripheral. Wri
//         (0x198 => ahb6rstsetr: ReadWrite<u32, AHB6RSTSETR::Register>),
//         /// This register is used to release the reset of the corresponding peripheral. Writ
//         (0x19C => ahb6rstclrr: ReadWrite<u32, AHB6RSTCLRR::Register>),
//         /// This register is used to activate the reset of the corresponding peripheral. Wri
//         (0x1A0 => tzahb6rstsetr: ReadWrite<u32>),
//         /// This register is used to release the reset of the corresponding peripheral. Writ
//         (0x1A4 => tzahb6rstclrr: ReadWrite<u32>),
//         (0x1A8 => _reserved8),
//         /// This register is used to set the peripheral clock enable bit of the correspondin
//         (0x200 => mp_apb4ensetr: ReadWrite<u32, MP_APB4ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit of the correspond
//         (0x204 => mp_apb4enclrr: ReadWrite<u32, MP_APB4ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit of the correspondin
//         (0x208 => mp_apb5ensetr: ReadWrite<u32, MP_APB5ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit of the correspond
//         (0x20C => mp_apb5enclrr: ReadWrite<u32, MP_APB5ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit of the correspondin
//         (0x210 => mp_ahb5ensetr: ReadWrite<u32, MP_AHB5ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit of the correspond
//         (0x214 => mp_ahb5enclrr: ReadWrite<u32, MP_AHB5ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit of the correspondin
//         (0x218 => mp_ahb6ensetr: ReadWrite<u32, MP_AHB6ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit of the correspond
//         (0x21C => mp_ahb6enclrr: ReadWrite<u32, MP_AHB6ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit of the correspondin
//         (0x220 => mp_tzahb6ensetr: ReadWrite<u32>),
//         /// This register is used to clear the peripheral clock enable bit of the correspond
//         (0x224 => mp_tzahb6enclrr: ReadWrite<u32>),
//         (0x228 => _reserved9),
//         /// This register is used to set the peripheral clock enable bit
//         (0x280 => mc_apb4ensetr: ReadWrite<u32, MC_APB4ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit
//         (0x284 => mc_apb4enclrr: ReadWrite<u32, MC_APB4ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit
//         (0x288 => mc_apb5ensetr: ReadWrite<u32, MC_APB5ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit
//         (0x28C => mc_apb5enclrr: ReadWrite<u32, MC_APB5ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit If TZEN = , this re
//         (0x290 => mc_ahb5ensetr: ReadWrite<u32, MC_AHB5ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit If TZEN = , this
//         (0x294 => mc_ahb5enclrr: ReadWrite<u32, MC_AHB5ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit
//         (0x298 => mc_ahb6ensetr: ReadWrite<u32, MC_AHB6ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit
//         (0x29C => mc_ahb6enclrr: ReadWrite<u32, MC_AHB6ENCLRR::Register>),
//         (0x2A0 => _reserved10),
//         /// This register is used by the MCU in order to clear the PERxLPEN bits
//         (0x300 => mp_apb4lpensetr: ReadWrite<u32, MP_APB4LPENSETR::Register>),
//         /// This register is used by the MCU
//         (0x304 => mp_apb4lpenclrr: ReadWrite<u32, MP_APB4LPENCLRR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = ,
//         (0x308 => mp_apb5lpensetr: ReadWrite<u32, MP_APB5LPENSETR::Register>),
//         /// This register is used by the Mpu.
//         (0x30C => mp_apb5lpenclrr: ReadWrite<u32, MP_APB5LPENCLRR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = ,
//         (0x310 => mp_ahb5lpensetr: ReadWrite<u32, MP_AHB5LPENSETR::Register>),
//         /// This register is used by the MCU
//         (0x314 => mp_ahb5lpenclrr: ReadWrite<u32, MP_AHB5LPENCLRR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bits
//         (0x318 => mp_ahb6lpensetr: ReadWrite<u32, MP_AHB6LPENSETR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bits
//         (0x31C => mp_ahb6lpenclrr: ReadWrite<u32, MP_AHB6LPENCLRR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = ,
//         (0x320 => mp_tzahb6lpensetr: ReadWrite<u32>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = ,
//         (0x324 => mp_tzahb6lpenclrr: ReadWrite<u32>),
//         (0x328 => _reserved11),
//         /// This register is used by the MCU in order to set the PERxLPEN bit.
//         (0x380 => mc_apb4lpensetr: ReadWrite<u32, MC_APB4LPENSETR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bit
//         (0x384 => mc_apb4lpenclrr: ReadWrite<u32, MC_APB4LPENCLRR::Register>),
//         /// This register is used by the MCU in order to set the PERxLPEN bit.
//         (0x388 => mc_apb5lpensetr: ReadWrite<u32, MC_APB5LPENSETR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bit
//         (0x38C => mc_apb5lpenclrr: ReadWrite<u32, MC_APB5LPENCLRR::Register>),
//         /// This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , t
//         (0x390 => mc_ahb5lpensetr: ReadWrite<u32, MC_AHB5LPENSETR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = ,
//         (0x394 => mc_ahb5lpenclrr: ReadWrite<u32, MC_AHB5LPENCLRR::Register>),
//         /// This register is used by the MCU in order to set the PERxLPEN bit.
//         (0x398 => mc_ahb6lpensetr: ReadWrite<u32, MC_AHB6LPENSETR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bit
//         (0x39C => mc_ahb6lpenclrr: ReadWrite<u32, MC_AHB6LPENCLRR::Register>),
//         (0x3A0 => _reserved12),
//         /// This register is used by the BOOTROM to check the reset source. Writing  has no
//         (0x400 => br_rstsclrr: ReadWrite<u32, BR_RSTSCLRR::Register>),
//         /// This register is used by the MPU in order to generate either a MCU reset or a sy
//         (0x404 => mp_grstcsetr: ReadWrite<u32, MP_GRSTCSETR::Register>),
//         /// This register is used by the MPU to check the reset source. This register is upd
//         (0x408 => mp_rstsclrr: ReadWrite<u32, MP_RSTSCLRR::Register>),
//         /// This register is used by the BOOTROM in order to freeze the IWDGs clocks. After
//         (0x40C => mp_iwdgfzsetr: ReadWrite<u32, MP_IWDGFZSETR::Register>),
//         /// This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writ
//         (0x410 => mp_iwdgfzclrr: ReadWrite<u32, MP_IWDGFZCLRR::Register>),
//         /// This register shall be used by the MPU to control the interrupt source enable. R
//         (0x414 => mp_cier: ReadWrite<u32, MP_CIER::Register>),
//         /// This register shall be used by the MPU in order to read and clear the interrupt
//         (0x418 => mp_cifr: ReadWrite<u32, MP_CIFR::Register>),
//         /// This register is used to program the delay between the moment where the system e
//         (0x41C => pwrlpdlycr: ReadWrite<u32, PWRLPDLYCR::Register>),
//         /// This register is dedicated to the BOOTROM code in order to update the reset sour
//         (0x420 => mp_rstssetr: ReadWrite<u32, MP_RSTSSETR::Register>),
//         (0x424 => _reserved13),
//         /// This register is used to select the clock generated on MCO1 output.
//         (0x800 => mco1cfgr: ReadWrite<u32, MCO1CFGR::Register>),
//         /// This register is used to select the clock generated on MCO2 output.
//         (0x804 => mco2cfgr: ReadWrite<u32, MCO2CFGR::Register>),
//         /// This is a read-only access register, It contains the status flags of oscillators
//         (0x808 => ocrdyr: ReadOnly<u32, OCRDYR::Register>),
//         /// This is register contains the enable control of the debug and trace function, an
//         (0x80C => dbgcfgr: ReadWrite<u32, DBGCFGR::Register>),
//         (0x810 => _reserved14),
//         /// This register is used to select the reference clock for PLL3. If TZEN = MCKPROT
//         (0x820 => rck3selr: ReadWrite<u32, RCK3SELR::Register>),
//         /// This register is used to select the reference clock for PLL4.
//         (0x824 => rck4selr: ReadWrite<u32, RCK4SELR::Register>),
//         /// This register is used to control the prescaler value of timers located into APB1
//         (0x828 => timg1prer: ReadWrite<u32, TIMG1PRER::Register>),
//         /// This register is used to control the prescaler value of timers located into APB2
//         (0x82C => timg2prer: ReadWrite<u32, TIMG2PRER::Register>),
//         /// This register is used to control the MCU sub-system clock prescaler. Refer to Se
//         (0x830 => mcudivr: ReadWrite<u32, MCUDIVR::Register>),
//         /// This register is used to control the APB1 clock prescaler. Refer to section Sect
//         (0x834 => apb1divr: ReadWrite<u32, APB1DIVR::Register>),
//         /// This register is used to control the APB2 clock prescaler. Refer to Section: Sub
//         (0x838 => apb2divr: ReadWrite<u32, APB2DIVR::Register>),
//         /// This register is used to control the APB3 clock prescaler. Refer to Section: Sub
//         (0x83C => apb3divr: ReadWrite<u32, APB3DIVR::Register>),
//         (0x840 => _reserved15),
//         /// This register is used to control the PLL3. If TZEN = MCKPROT = , this register c
//         (0x880 => pll3cr: ReadWrite<u32, PLL3CR::Register>),
//         /// This register is used to configure the PLL3. If TZEN = MCKPROT = , this register
//         (0x884 => pll3cfgr1: ReadWrite<u32, PLL3CFGR1::Register>),
//         /// This register is used to configure the PLL3. If TZEN = MCKPROT = , this register
//         (0x888 => pll3cfgr2: ReadWrite<u32, PLL3CFGR2::Register>),
//         /// This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKP
//         (0x88C => pll3fracr: ReadWrite<u32, PLL3FRACR::Register>),
//         /// This register is used to configure the PLL3.It is not recommended to change the
//         (0x890 => pll3csgr: ReadWrite<u32, PLL3CSGR::Register>),
//         /// This register is used to control the PLL4.
//         (0x894 => pll4cr: ReadWrite<u32, PLL4CR::Register>),
//         /// This register is used to configure the PLL4.
//         (0x898 => pll4cfgr1: ReadWrite<u32, PLL4CFGR1::Register>),
//         /// This register is used to configure the PLL4.
//         (0x89C => pll4cfgr2: ReadWrite<u32, PLL4CFGR2::Register>),
//         /// This register is used to fine-tune the frequency of the PLL4 VCO.
//         (0x8A0 => pll4fracr: ReadWrite<u32, PLL4FRACR::Register>),
//         /// This register is used to configure the PLL4.It is not recommended to change the
//         (0x8A4 => pll4csgr: ReadWrite<u32, PLL4CSGR::Register>),
//         (0x8A8 => _reserved16),
//         /// This register is used to control the selection of the kernel clock for the I2C1
//         (0x8C0 => i2c12ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the I2C3
//         (0x8C4 => i2c35ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the SAI1
//         (0x8C8 => sai1ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the SAI2.
//         (0x8CC => sai2ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the SAI3.
//         (0x8D0 => sai3ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the SAI4.
//         (0x8D4 => sai4ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the SPI/I
//         (0x8D8 => spi2s1ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the SPI/I
//         (0x8DC => spi2s23ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the SPI4,
//         (0x8E0 => spi45ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the USART
//         (0x8E4 => uart6ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the USART
//         (0x8E8 => uart24ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the USART
//         (0x8EC => uart35ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the UART7
//         (0x8F0 => uart78ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the SDMMC
//         (0x8F4 => sdmmc12ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the SDMMC
//         (0x8F8 => sdmmc3ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the ETH b
//         (0x8FC => ethckselr: ReadWrite<u32, ETHCKSELR::Register>),
//         /// This register is used to control the selection of the kernel clock for the QUADS
//         (0x900 => qspickselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the FMC b
//         (0x904 => fmcckselr: ReadWrite<u32>),
//         (0x908 => _reserved17),
//         /// This register is used to control the selection of the kernel clock for the FDCAN
//         (0x90C => fdcanckselr: ReadWrite<u32>),
//         (0x910 => _reserved18),
//         /// This register is used to control the selection of the kernel clock for the SPDIF
//         (0x914 => spdifckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the CEC-H
//         (0x918 => cecckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the USBPH
//         (0x91C => usbckselr: ReadWrite<u32, USBCKSELR::Register>),
//         /// This register is used to control the selection of the kernel clock for the RNG2.
//         (0x920 => rng2ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the DSI b
//         (0x924 => dsickselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the ADC b
//         (0x928 => adcckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the LPTIM
//         (0x92C => lptim45ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the LPTIM
//         (0x930 => lptim23ckselr: ReadWrite<u32>),
//         /// This register is used to control the selection of the kernel clock for the LPTIM
//         (0x934 => lptim1ckselr: ReadWrite<u32>),
//         (0x938 => _reserved19),
//         /// This register is used to activate the reset of the corresponding peripheral.
//         (0x980 => apb1rstsetr: ReadWrite<u32, APB1RSTSETR::Register>),
//         /// This register is used to release the reset of the corresponding peripheral.
//         (0x984 => apb1rstclrr: ReadWrite<u32, APB1RSTCLRR::Register>),
//         /// This register is used to activate the reset of the corresponding peripheral.
//         (0x988 => apb2rstsetr: ReadWrite<u32, APB2RSTSETR::Register>),
//         /// This register is used to release the reset of the corresponding peripheral.
//         (0x98C => apb2rstclrr: ReadWrite<u32, APB2RSTCLRR::Register>),
//         /// This register is used to activate the reset of the corresponding peripheral.
//         (0x990 => apb3rstsetr: ReadWrite<u32, APB3RSTSETR::Register>),
//         /// This register is used to release the reset of the corresponding peripheral.
//         (0x994 => apb3rstclrr: ReadWrite<u32, APB3RSTCLRR::Register>),
//         /// This register is used to activate the reset of the corresponding peripheral.
//         (0x998 => ahb2rstsetr: ReadWrite<u32, AHB2RSTSETR::Register>),
//         /// This register is used to release the reset of the corresponding peripheral.
//         (0x99C => ahb2rstclrr: ReadWrite<u32, AHB2RSTCLRR::Register>),
//         /// This register is used to activate the reset of the corresponding peripheral.
//         (0x9A0 => ahb3rstsetr: ReadWrite<u32, AHB3RSTSETR::Register>),
//         /// This register is used to release the reset of the corresponding peripheral.
//         (0x9A4 => ahb3rstclrr: ReadWrite<u32, AHB3RSTCLRR::Register>),
//         /// This register is used to activate the reset of the corresponding peripheral
//         (0x9A8 => ahb4rstsetr: ReadWrite<u32, AHB4RSTSETR::Register>),
//         /// This register is used to release the reset of the corresponding peripheral.
//         (0x9AC => ahb4rstclrr: ReadWrite<u32, AHB4RSTCLRR::Register>),
//         (0x9B0 => _reserved20),
//         /// This register is used to set the peripheral clock enable bit
//         (0xA00 => mp_apb1ensetr: ReadWrite<u32, MP_APB1ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit
//         (0xA04 => mp_apb1enclrr: ReadWrite<u32, MP_APB1ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit
//         (0xA08 => mp_apb2ensetr: ReadWrite<u32, MP_APB2ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit of the correspond
//         (0xA0C => mp_apb2enclrr: ReadWrite<u32, MP_APB2ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit
//         (0xA10 => mp_apb3ensetr: ReadWrite<u32, MP_APB3ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit of the correspond
//         (0xA14 => mp_apb3enclrr: ReadWrite<u32, MP_APB3ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit of the correspondin
//         (0xA18 => mp_ahb2ensetr: ReadWrite<u32, MP_AHB2ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit of the correspond
//         (0xA1C => mp_ahb2enclrr: ReadWrite<u32, MP_AHB2ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit of the correspondin
//         (0xA20 => mp_ahb3ensetr: ReadWrite<u32, MP_AHB3ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit of the correspond
//         (0xA24 => mp_ahb3enclrr: ReadWrite<u32, MP_AHB3ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit of the correspondin
//         (0xA28 => mp_ahb4ensetr: ReadWrite<u32, MP_AHB4ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit
//         (0xA2C => mp_ahb4enclrr: ReadWrite<u32, MP_AHB4ENCLRR::Register>),
//         (0xA30 => _reserved21),
//         /// This register is used to set the peripheral clock enable bit
//         (0xA38 => mp_mlahbensetr: ReadWrite<u32>),
//         /// This register is used to clear the peripheral clock enable bit.
//         (0xA3C => mp_mlahbenclrr: ReadWrite<u32>),
//         (0xA40 => _reserved22),
//         /// This register is used to set the peripheral clock enable bit of the correspondin
//         (0xA80 => mc_apb1ensetr: ReadWrite<u32, MC_APB1ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit of the correspond
//         (0xA84 => mc_apb1enclrr: ReadWrite<u32, MC_APB1ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit
//         (0xA88 => mc_apb2ensetr: ReadWrite<u32, MC_APB2ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit
//         (0xA8C => mc_apb2enclrr: ReadWrite<u32, MC_APB2ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit
//         (0xA90 => mc_apb3ensetr: ReadWrite<u32, MC_APB3ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit
//         (0xA94 => mc_apb3enclrr: ReadWrite<u32, MC_APB3ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit
//         (0xA98 => mc_ahb2ensetr: ReadWrite<u32, MC_AHB2ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit
//         (0xA9C => mc_ahb2enclrr: ReadWrite<u32, MC_AHB2ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit
//         (0xAA0 => mc_ahb3ensetr: ReadWrite<u32, MC_AHB3ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit
//         (0xAA4 => mc_ahb3enclrr: ReadWrite<u32, MC_AHB3ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit
//         (0xAA8 => mc_ahb4ensetr: ReadWrite<u32, MC_AHB4ENSETR::Register>),
//         /// This register is used to clear the peripheral clock enable bit
//         (0xAAC => mc_ahb4enclrr: ReadWrite<u32, MC_AHB4ENCLRR::Register>),
//         /// This register is used to set the peripheral clock enable bit
//         (0xAB0 => mc_aximensetr: ReadWrite<u32>),
//         /// This register is used to clear the peripheral clock enable bit
//         (0xAB4 => mc_aximenclrr: ReadWrite<u32>),
//         /// This register is used to set the peripheral clock enable bit
//         (0xAB8 => mc_mlahbensetr: ReadWrite<u32>),
//         /// This register is used to clear the peripheral clock enable bit
//         (0xABC => mc_mlahbenclrr: ReadWrite<u32>),
//         (0xAC0 => _reserved23),
//         /// This register is used by the MCU in order to clear the PERxLPEN bits
//         (0xB00 => mp_apb1lpensetr: ReadWrite<u32, MP_APB1LPENSETR::Register>),
//         /// This register is used by the MPU in order to clear the PERxLPEN bits .
//         (0xB04 => mp_apb1lpenclrr: ReadWrite<u32, MP_APB1LPENCLRR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bits
//         (0xB08 => mp_apb2lpensetr: ReadWrite<u32, MP_APB2LPENSETR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bits
//         (0xB0C => mp_apb2lpenclrr: ReadWrite<u32, MP_APB2LPENCLRR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bits
//         (0xB10 => mp_apb3lpensetr: ReadWrite<u32, MP_APB3LPENSETR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bits
//         (0xB14 => mp_apb3lpenclrr: ReadWrite<u32, MP_APB3LPENCLRR::Register>),
//         /// This register is used by the MPU in order to set the PERxLPEN bit.
//         (0xB18 => mp_ahb2lpensetr: ReadWrite<u32, MP_AHB2LPENSETR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bits
//         (0xB1C => mp_ahb2lpenclrr: ReadWrite<u32, MP_AHB2LPENCLRR::Register>),
//         /// This register is used by the MPU
//         (0xB20 => mp_ahb3lpensetr: ReadWrite<u32, MP_AHB3LPENSETR::Register>),
//         /// This register is used by the MPU in order to clear the PERxLPEN bit
//         (0xB24 => mp_ahb3lpenclrr: ReadWrite<u32, MP_AHB3LPENCLRR::Register>),
//         /// This register is used by the MPU
//         (0xB28 => mp_ahb4lpensetr: ReadWrite<u32, MP_AHB4LPENSETR::Register>),
//         /// This register is used by the MPU
//         (0xB2C => mp_ahb4lpenclrr: ReadWrite<u32, MP_AHB4LPENCLRR::Register>),
//         /// This register is used by the MPU
//         (0xB30 => mp_aximlpensetr: ReadWrite<u32>),
//         /// This register is used by the MPU
//         (0xB34 => mp_aximlpenclrr: ReadWrite<u32>),
//         /// This register is used by the MPU in order to set the PERxLPEN bit
//         (0xB38 => mp_mlahblpensetr: ReadWrite<u32, MP_MLAHBLPENSETR::Register>),
//         /// This register is used by the MPU in order to clear the PERxLPEN bit
//         (0xB3C => mp_mlahblpenclrr: ReadWrite<u32, MP_MLAHBLPENCLRR::Register>),
//         (0xB40 => _reserved24),
//         /// This register is used by the MCU in order to set the PERxLPEN bit.
//         (0xB80 => mc_apb1lpensetr: ReadWrite<u32, MC_APB1LPENSETR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bits
//         (0xB84 => mc_apb1lpenclrr: ReadWrite<u32, MC_APB1LPENCLRR::Register>),
//         /// This register is used by the MCU in order to set the PERxLPEN bit.
//         (0xB88 => mc_apb2lpensetr: ReadWrite<u32, MC_APB2LPENSETR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bit
//         (0xB8C => mc_apb2lpenclrr: ReadWrite<u32, MC_APB2LPENCLRR::Register>),
//         /// This register is used by the MCU in order to set the PERxLPEN bit.
//         (0xB90 => mc_apb3lpensetr: ReadWrite<u32, MC_APB3LPENSETR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bit
//         (0xB94 => mc_apb3lpenclrr: ReadWrite<u32, MC_APB3LPENCLRR::Register>),
//         /// This register is used by the MCU in order to set the PERxLPEN bit.
//         (0xB98 => mc_ahb2lpensetr: ReadWrite<u32, MC_AHB2LPENSETR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bit
//         (0xB9C => mc_ahb2lpenclrr: ReadWrite<u32, MC_AHB2LPENCLRR::Register>),
//         /// This register is used by the MCU in order to set the PERxLPEN bit.
//         (0xBA0 => mc_ahb3lpensetr: ReadWrite<u32, MC_AHB3LPENSETR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bit
//         (0xBA4 => mc_ahb3lpenclrr: ReadWrite<u32, MC_AHB3LPENCLRR::Register>),
//         /// This register is used by the MCU in order to set the PERxLPEN bit.
//         (0xBA8 => mc_ahb4lpensetr: ReadWrite<u32, MC_AHB4LPENSETR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bit of the corre
//         (0xBAC => mc_ahb4lpenclrr: ReadWrite<u32, MC_AHB4LPENCLRR::Register>),
//         /// This register is used by the MCU in order to set the PERxLPEN bit of the corresp
//         (0xBB0 => mc_aximlpensetr: ReadWrite<u32>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bit of the corre
//         (0xBB4 => mc_aximlpenclrr: ReadWrite<u32>),
//         /// This register is used by the MCU in order to set the PERxLPEN bit of the corresp
//         (0xBB8 => mc_mlahblpensetr: ReadWrite<u32, MC_MLAHBLPENSETR::Register>),
//         /// This register is used by the MCU in order to clear the PERxLPEN bit of the corre
//         (0xBBC => mc_mlahblpenclrr: ReadWrite<u32, MC_MLAHBLPENCLRR::Register>),
//         (0xBC0 => _reserved25),
//         /// This register is used by the MCU to check the reset source.
//         (0xC00 => mc_rstsclrr: ReadWrite<u32, MC_RSTSCLRR::Register>),
//         (0xC04 => _reserved26),
//         /// This register shall be used by the MCU to control the interrupt source enable. R
//         (0xC14 => mc_cier: ReadWrite<u32, MC_CIER::Register>),
//         /// This register shall be used by the MCU in order to read and clear the interrupt
//         (0xC18 => mc_cifr: ReadWrite<u32, MC_CIFR::Register>),
//         (0xC1C => _reserved27),
//         /// This register gives the IP version
//         (0xFF4 => verr: ReadOnly<u32, VERR::Register>),
//         /// This register gives the unique identifier of the RCC
//         (0xFF8 => idr: ReadOnly<u32>),
//         /// This register gives the decoding space, which is for the RCC of 4 kB.
//         (0xFFC => sidr: ReadOnly<u32>),
//         (0x1000 => @END),
//     }
// }
// register_bitfields![u32,
// TZCR [
//     /// TZEN
//     TZEN OFFSET(0) NUMBITS(1) [],
//     /// MCKPROT
//     MCKPROT OFFSET(1) NUMBITS(1) []
// ],
// OCENSETR [
//     /// HSION
//     HSION OFFSET(0) NUMBITS(1) [],
//     /// HSIKERON
//     HSIKERON OFFSET(1) NUMBITS(1) [],
//     /// CSION
//     CSION OFFSET(4) NUMBITS(1) [],
//     /// CSIKERON
//     CSIKERON OFFSET(5) NUMBITS(1) [],
//     /// DIGBYP
//     DIGBYP OFFSET(7) NUMBITS(1) [],
//     /// HSEON
//     HSEON OFFSET(8) NUMBITS(1) [],
//     /// HSEKERON
//     HSEKERON OFFSET(9) NUMBITS(1) [],
//     /// HSEBYP
//     HSEBYP OFFSET(10) NUMBITS(1) [],
//     /// HSECSSON
//     HSECSSON OFFSET(11) NUMBITS(1) []
// ],
// OCENCLRR [
//     /// HSION
//     HSION OFFSET(0) NUMBITS(1) [],
//     /// HSIKERON
//     HSIKERON OFFSET(1) NUMBITS(1) [],
//     /// CSION
//     CSION OFFSET(4) NUMBITS(1) [],
//     /// CSIKERON
//     CSIKERON OFFSET(5) NUMBITS(1) [],
//     /// DIGBYP
//     DIGBYP OFFSET(7) NUMBITS(1) [],
//     /// HSEON
//     HSEON OFFSET(8) NUMBITS(1) [],
//     /// HSEKERON
//     HSEKERON OFFSET(9) NUMBITS(1) [],
//     /// HSEBYP
//     HSEBYP OFFSET(10) NUMBITS(1) []
// ],
// HSICFGR [
//     /// HSIDIV
//     HSIDIV OFFSET(0) NUMBITS(2) [],
//     /// HSITRIM
//     HSITRIM OFFSET(8) NUMBITS(7) [],
//     /// HSICAL
//     HSICAL OFFSET(16) NUMBITS(12) []
// ],
// CSICFGR [
//     /// CSITRIM
//     CSITRIM OFFSET(8) NUMBITS(5) [],
//     /// CSICAL
//     CSICAL OFFSET(16) NUMBITS(8) []
// ],
// MPCKSELR [
//     /// MPUSRC
//     MPUSRC OFFSET(0) NUMBITS(2) [],
//     /// MPUSRCRDY
//     MPUSRCRDY OFFSET(31) NUMBITS(1) []
// ],
// ASSCKSELR [
//     /// AXISSRC
//     AXISSRC OFFSET(0) NUMBITS(3) [],
//     /// AXISSRCRDY
//     AXISSRCRDY OFFSET(31) NUMBITS(1) []
// ],
// RCK12SELR [
//     /// PLL12SRC
//     PLL12SRC OFFSET(0) NUMBITS(2) [],
//     /// PLL12SRCRDY
//     PLL12SRCRDY OFFSET(31) NUMBITS(1) []
// ],
// MPCKDIVR [
//     /// MPUDIV
//     MPUDIV OFFSET(0) NUMBITS(3) [],
//     /// MPUDIVRDY
//     MPUDIVRDY OFFSET(31) NUMBITS(1) []
// ],
// AXIDIVR [
//     /// AXIDIV
//     AXIDIV OFFSET(0) NUMBITS(3) [],
//     /// AXIDIVRDY
//     AXIDIVRDY OFFSET(31) NUMBITS(1) []
// ],
// APB4DIVR [
//     /// APB4DIV
//     APB4DIV OFFSET(0) NUMBITS(3) [],
//     /// APB4DIVRDY
//     APB4DIVRDY OFFSET(31) NUMBITS(1) []
// ],
// APB5DIVR [
//     /// APB5DIV
//     APB5DIV OFFSET(0) NUMBITS(3) [],
//     /// APB5DIVRDY
//     APB5DIVRDY OFFSET(31) NUMBITS(1) []
// ],
// RTCDIVR [
//     /// RTCDIV
//     RTCDIV OFFSET(0) NUMBITS(6) []
// ],
// MSSCKSELR [
//     /// MCUSSRC
//     MCUSSRC OFFSET(0) NUMBITS(2) [],
//     /// MCUSSRCRDY
//     MCUSSRCRDY OFFSET(31) NUMBITS(1) []
// ],
// PLL1CR [
//     /// PLLON
//     PLLON OFFSET(0) NUMBITS(1) [],
//     /// PLL1RDY
//     PLL1RDY OFFSET(1) NUMBITS(1) [],
//     /// SSCG_CTRL
//     SSCG_CTRL OFFSET(2) NUMBITS(1) [],
//     /// DIVPEN
//     DIVPEN OFFSET(4) NUMBITS(1) [],
//     /// DIVQEN
//     DIVQEN OFFSET(5) NUMBITS(1) [],
//     /// DIVREN
//     DIVREN OFFSET(6) NUMBITS(1) []
// ],
// PLL1CFGR1 [
//     /// DIVN
//     DIVN OFFSET(0) NUMBITS(9) [],
//     /// DIVM1
//     DIVM1 OFFSET(16) NUMBITS(6) []
// ],
// PLL1CFGR2 [
//     /// DIVP
//     DIVP OFFSET(0) NUMBITS(7) [],
//     /// DIVQ
//     DIVQ OFFSET(8) NUMBITS(7) [],
//     /// DIVR
//     DIVR OFFSET(16) NUMBITS(7) []
// ],
// PLL1FRACR [
//     /// FRACV
//     FRACV OFFSET(3) NUMBITS(13) [],
//     /// FRACLE
//     FRACLE OFFSET(16) NUMBITS(1) []
// ],
// PLL1CSGR [
//     /// MOD_PER
//     MOD_PER OFFSET(0) NUMBITS(13) [],
//     /// TPDFN_DIS
//     TPDFN_DIS OFFSET(13) NUMBITS(1) [],
//     /// RPDFN_DIS
//     RPDFN_DIS OFFSET(14) NUMBITS(1) [],
//     /// SSCG_MODE
//     SSCG_MODE OFFSET(15) NUMBITS(1) [],
//     /// INC_STEP
//     INC_STEP OFFSET(16) NUMBITS(15) []
// ],
// PLL2CR [
//     /// PLLON
//     PLLON OFFSET(0) NUMBITS(1) [],
//     /// PLL2RDY
//     PLL2RDY OFFSET(1) NUMBITS(1) [],
//     /// SSCG_CTRL
//     SSCG_CTRL OFFSET(2) NUMBITS(1) [],
//     /// DIVPEN
//     DIVPEN OFFSET(4) NUMBITS(1) [],
//     /// DIVQEN
//     DIVQEN OFFSET(5) NUMBITS(1) [],
//     /// DIVREN
//     DIVREN OFFSET(6) NUMBITS(1) []
// ],
// PLL2CFGR1 [
//     /// DIVN
//     DIVN OFFSET(0) NUMBITS(9) [],
//     /// DIVM2
//     DIVM2 OFFSET(16) NUMBITS(6) []
// ],
// PLL2CFGR2 [
//     /// DIVP
//     DIVP OFFSET(0) NUMBITS(7) [],
//     /// DIVQ
//     DIVQ OFFSET(8) NUMBITS(7) [],
//     /// DIVR
//     DIVR OFFSET(16) NUMBITS(7) []
// ],
// PLL2FRACR [
//     /// FRACV
//     FRACV OFFSET(3) NUMBITS(13) [],
//     /// FRACLE
//     FRACLE OFFSET(16) NUMBITS(1) []
// ],
// PLL2CSGR [
//     /// MOD_PER
//     MOD_PER OFFSET(0) NUMBITS(13) [],
//     /// TPDFN_DIS
//     TPDFN_DIS OFFSET(13) NUMBITS(1) [],
//     /// RPDFN_DIS
//     RPDFN_DIS OFFSET(14) NUMBITS(1) [],
//     /// SSCG_MODE
//     SSCG_MODE OFFSET(15) NUMBITS(1) [],
//     /// INC_STEP
//     INC_STEP OFFSET(16) NUMBITS(15) []
// ],
// I2C46CKSELR [
//     /// I2C46SRC
//     I2C46SRC OFFSET(0) NUMBITS(3) []
// ],
// SPI6CKSELR [
//     /// SPI6SRC
//     SPI6SRC OFFSET(0) NUMBITS(3) []
// ],
// UART1CKSELR [
//     /// UART1SRC
//     UART1SRC OFFSET(0) NUMBITS(3) []
// ],
// RNG1CKSELR [
//     /// RNG1SRC
//     RNG1SRC OFFSET(0) NUMBITS(2) []
// ],
// CPERCKSELR [
//     /// CKPERSRC
//     CKPERSRC OFFSET(0) NUMBITS(2) []
// ],
// STGENCKSELR [
//     /// STGENSRC
//     STGENSRC OFFSET(0) NUMBITS(2) []
// ],
// DDRITFCR [
//     /// DDRC1EN
//     DDRC1EN OFFSET(0) NUMBITS(1) [],
//     /// DDRC1LPEN
//     DDRC1LPEN OFFSET(1) NUMBITS(1) [],
//     /// DDRC2EN
//     DDRC2EN OFFSET(2) NUMBITS(1) [],
//     /// DDRC2LPEN
//     DDRC2LPEN OFFSET(3) NUMBITS(1) [],
//     /// DDRPHYCEN
//     DDRPHYCEN OFFSET(4) NUMBITS(1) [],
//     /// DDRPHYCLPEN
//     DDRPHYCLPEN OFFSET(5) NUMBITS(1) [],
//     /// DDRCAPBEN
//     DDRCAPBEN OFFSET(6) NUMBITS(1) [],
//     /// DDRCAPBLPEN
//     DDRCAPBLPEN OFFSET(7) NUMBITS(1) [],
//     /// AXIDCGEN
//     AXIDCGEN OFFSET(8) NUMBITS(1) [],
//     /// DDRPHYCAPBEN
//     DDRPHYCAPBEN OFFSET(9) NUMBITS(1) [],
//     /// DDRPHYCAPBLPEN
//     DDRPHYCAPBLPEN OFFSET(10) NUMBITS(1) [],
//     /// KERDCG_DLY
//     KERDCG_DLY OFFSET(11) NUMBITS(3) [],
//     /// DDRCAPBRST
//     DDRCAPBRST OFFSET(14) NUMBITS(1) [],
//     /// DDRCAXIRST
//     DDRCAXIRST OFFSET(15) NUMBITS(1) [],
//     /// DDRCORERST
//     DDRCORERST OFFSET(16) NUMBITS(1) [],
//     /// DPHYAPBRST
//     DPHYAPBRST OFFSET(17) NUMBITS(1) [],
//     /// DPHYRST
//     DPHYRST OFFSET(18) NUMBITS(1) [],
//     /// DPHYCTLRST
//     DPHYCTLRST OFFSET(19) NUMBITS(1) [],
//     /// DDRCKMOD
//     DDRCKMOD OFFSET(20) NUMBITS(3) [],
//     /// GSKPMOD
//     GSKPMOD OFFSET(23) NUMBITS(1) [],
//     /// GSKPCTRL
//     GSKPCTRL OFFSET(24) NUMBITS(1) [],
//     /// DFILP_WIDTH
//     DFILP_WIDTH OFFSET(25) NUMBITS(3) [],
//     /// GSKP_DUR
//     GSKP_DUR OFFSET(28) NUMBITS(4) []
// ],
// MP_BOOTCR [
//     /// MCU_BEN
//     MCU_BEN OFFSET(0) NUMBITS(1) [],
//     /// MPU_BEN
//     MPU_BEN OFFSET(1) NUMBITS(1) []
// ],
// MP_SREQSETR [
//     /// STPREQ_P0
//     STPREQ_P0 OFFSET(0) NUMBITS(1) [],
//     /// STPREQ_P1
//     STPREQ_P1 OFFSET(1) NUMBITS(1) []
// ],
// MP_SREQCLRR [
//     /// STPREQ_P0
//     STPREQ_P0 OFFSET(0) NUMBITS(1) [],
//     /// STPREQ_P1
//     STPREQ_P1 OFFSET(1) NUMBITS(1) []
// ],
// MP_GCR [
//     /// BOOT_MCU
//     BOOT_MCU OFFSET(0) NUMBITS(1) []
// ],
// MP_APRSTCR [
//     /// RDCTLEN
//     RDCTLEN OFFSET(0) NUMBITS(1) [],
//     /// RSTTO
//     RSTTO OFFSET(8) NUMBITS(7) []
// ],
// MP_APRSTSR [
//     /// RSTTOV
//     RSTTOV OFFSET(8) NUMBITS(7) []
// ],
// BDCR [
//     /// LSEON
//     LSEON OFFSET(0) NUMBITS(1) [],
//     /// LSEBYP
//     LSEBYP OFFSET(1) NUMBITS(1) [],
//     /// LSERDY
//     LSERDY OFFSET(2) NUMBITS(1) [],
//     /// DIGBYP
//     DIGBYP OFFSET(3) NUMBITS(1) [],
//     /// LSEDRV
//     LSEDRV OFFSET(4) NUMBITS(2) [],
//     /// LSECSSON
//     LSECSSON OFFSET(8) NUMBITS(1) [],
//     /// LSECSSD
//     LSECSSD OFFSET(9) NUMBITS(1) [],
//     /// RTCSRC
//     RTCSRC OFFSET(16) NUMBITS(2) [],
//     /// RTCCKEN
//     RTCCKEN OFFSET(20) NUMBITS(1) [],
//     /// VSWRST
//     VSWRST OFFSET(31) NUMBITS(1) []
// ],
// RDLSICR [
//     /// LSION
//     LSION OFFSET(0) NUMBITS(1) [],
//     /// LSIRDY
//     LSIRDY OFFSET(1) NUMBITS(1) [],
//     /// MRD
//     MRD OFFSET(16) NUMBITS(5) [],
//     /// EADLY
//     EADLY OFFSET(24) NUMBITS(3) [],
//     /// SPARE
//     SPARE OFFSET(27) NUMBITS(5) []
// ],
// APB4RSTSETR [
//     /// LTDCRST
//     LTDCRST OFFSET(0) NUMBITS(1) [],
//     /// DSIRST
//     DSIRST OFFSET(4) NUMBITS(1) [],
//     /// DDRPERFMRST
//     DDRPERFMRST OFFSET(8) NUMBITS(1) [],
//     /// USBPHYRST
//     USBPHYRST OFFSET(16) NUMBITS(1) []
// ],
// APB4RSTCLRR [
//     /// LTDCRST
//     LTDCRST OFFSET(0) NUMBITS(1) [],
//     /// DSIRST
//     DSIRST OFFSET(4) NUMBITS(1) [],
//     /// DDRPERFMRST
//     DDRPERFMRST OFFSET(8) NUMBITS(1) [],
//     /// USBPHYRST
//     USBPHYRST OFFSET(16) NUMBITS(1) []
// ],
// APB5RSTSETR [
//     /// SPI6RST
//     SPI6RST OFFSET(0) NUMBITS(1) [],
//     /// I2C4RST
//     I2C4RST OFFSET(2) NUMBITS(1) [],
//     /// I2C6RST
//     I2C6RST OFFSET(3) NUMBITS(1) [],
//     /// USART1RST
//     USART1RST OFFSET(4) NUMBITS(1) [],
//     /// STGENRST
//     STGENRST OFFSET(20) NUMBITS(1) []
// ],
// APB5RSTCLRR [
//     /// SPI6RST
//     SPI6RST OFFSET(0) NUMBITS(1) [],
//     /// I2C4RST
//     I2C4RST OFFSET(2) NUMBITS(1) [],
//     /// I2C6RST
//     I2C6RST OFFSET(3) NUMBITS(1) [],
//     /// USART1RST
//     USART1RST OFFSET(4) NUMBITS(1) [],
//     /// STGENRST
//     STGENRST OFFSET(20) NUMBITS(1) []
// ],
// AHB5RSTSETR [
//     /// GPIOZRST
//     GPIOZRST OFFSET(0) NUMBITS(1) [],
//     /// CRYP1RST
//     CRYP1RST OFFSET(4) NUMBITS(1) [],
//     /// HASH1RST
//     HASH1RST OFFSET(5) NUMBITS(1) [],
//     /// RNG1RST
//     RNG1RST OFFSET(6) NUMBITS(1) [],
//     /// AXIMCRST
//     AXIMCRST OFFSET(16) NUMBITS(1) []
// ],
// AHB5RSTCLRR [
//     /// GPIOZRST
//     GPIOZRST OFFSET(0) NUMBITS(1) [],
//     /// CRYP1RST
//     CRYP1RST OFFSET(4) NUMBITS(1) [],
//     /// HASH1RST
//     HASH1RST OFFSET(5) NUMBITS(1) [],
//     /// RNG1RST
//     RNG1RST OFFSET(6) NUMBITS(1) [],
//     /// AXIMCRST
//     AXIMCRST OFFSET(16) NUMBITS(1) []
// ],
// AHB6RSTSETR [
//     /// GPURST
//     GPURST OFFSET(5) NUMBITS(1) [],
//     /// ETHMACRST
//     ETHMACRST OFFSET(10) NUMBITS(1) [],
//     /// FMCRST
//     FMCRST OFFSET(12) NUMBITS(1) [],
//     /// QSPIRST
//     QSPIRST OFFSET(14) NUMBITS(1) [],
//     /// SDMMC1RST
//     SDMMC1RST OFFSET(16) NUMBITS(1) [],
//     /// SDMMC2RST
//     SDMMC2RST OFFSET(17) NUMBITS(1) [],
//     /// CRC1RST
//     CRC1RST OFFSET(20) NUMBITS(1) [],
//     /// USBHRST
//     USBHRST OFFSET(24) NUMBITS(1) []
// ],
// AHB6RSTCLRR [
//     /// ETHMACRST
//     ETHMACRST OFFSET(10) NUMBITS(1) [],
//     /// FMCRST
//     FMCRST OFFSET(12) NUMBITS(1) [],
//     /// QSPIRST
//     QSPIRST OFFSET(14) NUMBITS(1) [],
//     /// SDMMC1RST
//     SDMMC1RST OFFSET(16) NUMBITS(1) [],
//     /// SDMMC2RST
//     SDMMC2RST OFFSET(17) NUMBITS(1) [],
//     /// CRC1RST
//     CRC1RST OFFSET(20) NUMBITS(1) [],
//     /// USBHRST
//     USBHRST OFFSET(24) NUMBITS(1) []
// ],
// TZAHB6RSTSETR [
//     /// MDMARST
//     MDMARST OFFSET(0) NUMBITS(1) []
// ],
// TZAHB6RSTCLRR [
//     /// MDMARST
//     MDMARST OFFSET(0) NUMBITS(1) []
// ],
// MP_APB4ENSETR [
//     /// LTDCEN
//     LTDCEN OFFSET(0) NUMBITS(1) [],
//     /// DSIEN
//     DSIEN OFFSET(4) NUMBITS(1) [],
//     /// DDRPERFMEN
//     DDRPERFMEN OFFSET(8) NUMBITS(1) [],
//     /// IWDG2APBEN
//     IWDG2APBEN OFFSET(15) NUMBITS(1) [],
//     /// USBPHYEN
//     USBPHYEN OFFSET(16) NUMBITS(1) [],
//     /// STGENROEN
//     STGENROEN OFFSET(20) NUMBITS(1) []
// ],
// MP_APB4ENCLRR [
//     /// LTDCEN
//     LTDCEN OFFSET(0) NUMBITS(1) [],
//     /// DSIEN
//     DSIEN OFFSET(4) NUMBITS(1) [],
//     /// DDRPERFMEN
//     DDRPERFMEN OFFSET(8) NUMBITS(1) [],
//     /// IWDG2APBEN
//     IWDG2APBEN OFFSET(15) NUMBITS(1) [],
//     /// USBPHYEN
//     USBPHYEN OFFSET(16) NUMBITS(1) [],
//     /// STGENROEN
//     STGENROEN OFFSET(20) NUMBITS(1) []
// ],
// MP_APB5ENSETR [
//     /// SPI6EN
//     SPI6EN OFFSET(0) NUMBITS(1) [],
//     /// I2C4EN
//     I2C4EN OFFSET(2) NUMBITS(1) [],
//     /// I2C6EN
//     I2C6EN OFFSET(3) NUMBITS(1) [],
//     /// USART1EN
//     USART1EN OFFSET(4) NUMBITS(1) [],
//     /// RTCAPBEN
//     RTCAPBEN OFFSET(8) NUMBITS(1) [],
//     /// TZC1EN
//     TZC1EN OFFSET(11) NUMBITS(1) [],
//     /// TZC2EN
//     TZC2EN OFFSET(12) NUMBITS(1) [],
//     /// TZPCEN
//     TZPCEN OFFSET(13) NUMBITS(1) [],
//     /// IWDG1APBEN
//     IWDG1APBEN OFFSET(15) NUMBITS(1) [],
//     /// BSECEN
//     BSECEN OFFSET(16) NUMBITS(1) [],
//     /// STGENEN
//     STGENEN OFFSET(20) NUMBITS(1) []
// ],
// MP_APB5ENCLRR [
//     /// SPI6EN
//     SPI6EN OFFSET(0) NUMBITS(1) [],
//     /// I2C4EN
//     I2C4EN OFFSET(2) NUMBITS(1) [],
//     /// I2C6EN
//     I2C6EN OFFSET(3) NUMBITS(1) [],
//     /// USART1EN
//     USART1EN OFFSET(4) NUMBITS(1) [],
//     /// RTCAPBEN
//     RTCAPBEN OFFSET(8) NUMBITS(1) [],
//     /// TZC1EN
//     TZC1EN OFFSET(11) NUMBITS(1) [],
//     /// TZC2EN
//     TZC2EN OFFSET(12) NUMBITS(1) [],
//     /// TZPCEN
//     TZPCEN OFFSET(13) NUMBITS(1) [],
//     /// IWDG1APBEN
//     IWDG1APBEN OFFSET(15) NUMBITS(1) [],
//     /// BSECEN
//     BSECEN OFFSET(16) NUMBITS(1) [],
//     /// STGENEN
//     STGENEN OFFSET(20) NUMBITS(1) []
// ],
// MP_AHB5ENSETR [
//     /// GPIOZEN
//     GPIOZEN OFFSET(0) NUMBITS(1) [],
//     /// CRYP1EN
//     CRYP1EN OFFSET(4) NUMBITS(1) [],
//     /// HASH1EN
//     HASH1EN OFFSET(5) NUMBITS(1) [],
//     /// RNG1EN
//     RNG1EN OFFSET(6) NUMBITS(1) [],
//     /// BKPSRAMEN
//     BKPSRAMEN OFFSET(8) NUMBITS(1) [],
//     /// AXIMCEN
//     AXIMCEN OFFSET(16) NUMBITS(1) []
// ],
// MP_AHB5ENCLRR [
//     /// GPIOZEN
//     GPIOZEN OFFSET(0) NUMBITS(1) [],
//     /// CRYP1EN
//     CRYP1EN OFFSET(4) NUMBITS(1) [],
//     /// HASH1EN
//     HASH1EN OFFSET(5) NUMBITS(1) [],
//     /// RNG1EN
//     RNG1EN OFFSET(6) NUMBITS(1) [],
//     /// BKPSRAMEN
//     BKPSRAMEN OFFSET(8) NUMBITS(1) [],
//     /// AXIMCEN
//     AXIMCEN OFFSET(16) NUMBITS(1) []
// ],
// MP_AHB6ENSETR [
//     /// MDMAEN
//     MDMAEN OFFSET(0) NUMBITS(1) [],
//     /// GPUEN
//     GPUEN OFFSET(5) NUMBITS(1) [],
//     /// ETHCKEN
//     ETHCKEN OFFSET(7) NUMBITS(1) [],
//     /// ETHTXEN
//     ETHTXEN OFFSET(8) NUMBITS(1) [],
//     /// ETHRXEN
//     ETHRXEN OFFSET(9) NUMBITS(1) [],
//     /// ETHMACEN
//     ETHMACEN OFFSET(10) NUMBITS(1) [],
//     /// FMCEN
//     FMCEN OFFSET(12) NUMBITS(1) [],
//     /// QSPIEN
//     QSPIEN OFFSET(14) NUMBITS(1) [],
//     /// SDMMC1EN
//     SDMMC1EN OFFSET(16) NUMBITS(1) [],
//     /// SDMMC2EN
//     SDMMC2EN OFFSET(17) NUMBITS(1) [],
//     /// CRC1EN
//     CRC1EN OFFSET(20) NUMBITS(1) [],
//     /// USBHEN
//     USBHEN OFFSET(24) NUMBITS(1) []
// ],
// MP_AHB6ENCLRR [
//     /// MDMAEN
//     MDMAEN OFFSET(0) NUMBITS(1) [],
//     /// GPUEN
//     GPUEN OFFSET(5) NUMBITS(1) [],
//     /// ETHCKEN
//     ETHCKEN OFFSET(7) NUMBITS(1) [],
//     /// ETHTXEN
//     ETHTXEN OFFSET(8) NUMBITS(1) [],
//     /// ETHRXEN
//     ETHRXEN OFFSET(9) NUMBITS(1) [],
//     /// ETHMACEN
//     ETHMACEN OFFSET(10) NUMBITS(1) [],
//     /// FMCEN
//     FMCEN OFFSET(12) NUMBITS(1) [],
//     /// QSPIEN
//     QSPIEN OFFSET(14) NUMBITS(1) [],
//     /// SDMMC1EN
//     SDMMC1EN OFFSET(16) NUMBITS(1) [],
//     /// SDMMC2EN
//     SDMMC2EN OFFSET(17) NUMBITS(1) [],
//     /// CRC1EN
//     CRC1EN OFFSET(20) NUMBITS(1) [],
//     /// USBHEN
//     USBHEN OFFSET(24) NUMBITS(1) []
// ],
// MP_TZAHB6ENSETR [
//     /// MDMAEN
//     MDMAEN OFFSET(0) NUMBITS(1) []
// ],
// MP_TZAHB6ENCLRR [
//     /// MDMAEN
//     MDMAEN OFFSET(0) NUMBITS(1) []
// ],
// MC_APB4ENSETR [
//     /// LTDCEN
//     LTDCEN OFFSET(0) NUMBITS(1) [],
//     /// DSIEN
//     DSIEN OFFSET(4) NUMBITS(1) [],
//     /// DDRPERFMEN
//     DDRPERFMEN OFFSET(8) NUMBITS(1) [],
//     /// USBPHYEN
//     USBPHYEN OFFSET(16) NUMBITS(1) [],
//     /// STGENROEN
//     STGENROEN OFFSET(20) NUMBITS(1) []
// ],
// MC_APB4ENCLRR [
//     /// LTDCEN
//     LTDCEN OFFSET(0) NUMBITS(1) [],
//     /// DSIEN
//     DSIEN OFFSET(4) NUMBITS(1) [],
//     /// DDRPERFMEN
//     DDRPERFMEN OFFSET(8) NUMBITS(1) [],
//     /// USBPHYEN
//     USBPHYEN OFFSET(16) NUMBITS(1) [],
//     /// STGENROEN
//     STGENROEN OFFSET(20) NUMBITS(1) []
// ],
// MC_APB5ENSETR [
//     /// SPI6EN
//     SPI6EN OFFSET(0) NUMBITS(1) [],
//     /// I2C4EN
//     I2C4EN OFFSET(2) NUMBITS(1) [],
//     /// I2C6EN
//     I2C6EN OFFSET(3) NUMBITS(1) [],
//     /// USART1EN
//     USART1EN OFFSET(4) NUMBITS(1) [],
//     /// RTCAPBEN
//     RTCAPBEN OFFSET(8) NUMBITS(1) [],
//     /// TZC1EN
//     TZC1EN OFFSET(11) NUMBITS(1) [],
//     /// TZC2EN
//     TZC2EN OFFSET(12) NUMBITS(1) [],
//     /// TZPCEN
//     TZPCEN OFFSET(13) NUMBITS(1) [],
//     /// BSECEN
//     BSECEN OFFSET(16) NUMBITS(1) [],
//     /// STGENEN
//     STGENEN OFFSET(20) NUMBITS(1) []
// ],
// MC_APB5ENCLRR [
//     /// SPI6EN
//     SPI6EN OFFSET(0) NUMBITS(1) [],
//     /// I2C4EN
//     I2C4EN OFFSET(2) NUMBITS(1) [],
//     /// I2C6EN
//     I2C6EN OFFSET(3) NUMBITS(1) [],
//     /// USART1EN
//     USART1EN OFFSET(4) NUMBITS(1) [],
//     /// RTCAPBEN
//     RTCAPBEN OFFSET(8) NUMBITS(1) [],
//     /// TZC1EN
//     TZC1EN OFFSET(11) NUMBITS(1) [],
//     /// TZC2EN
//     TZC2EN OFFSET(12) NUMBITS(1) [],
//     /// TZPCEN
//     TZPCEN OFFSET(13) NUMBITS(1) [],
//     /// BSECEN
//     BSECEN OFFSET(16) NUMBITS(1) [],
//     /// STGENEN
//     STGENEN OFFSET(20) NUMBITS(1) []
// ],
// MC_AHB5ENSETR [
//     /// GPIOZEN
//     GPIOZEN OFFSET(0) NUMBITS(1) [],
//     /// CRYP1EN
//     CRYP1EN OFFSET(4) NUMBITS(1) [],
//     /// HASH1EN
//     HASH1EN OFFSET(5) NUMBITS(1) [],
//     /// RNG1EN
//     RNG1EN OFFSET(6) NUMBITS(1) [],
//     /// BKPSRAMEN
//     BKPSRAMEN OFFSET(8) NUMBITS(1) []
// ],
// MC_AHB5ENCLRR [
//     /// GPIOZEN
//     GPIOZEN OFFSET(0) NUMBITS(1) [],
//     /// CRYP1EN
//     CRYP1EN OFFSET(4) NUMBITS(1) [],
//     /// HASH1EN
//     HASH1EN OFFSET(5) NUMBITS(1) [],
//     /// RNG1EN
//     RNG1EN OFFSET(6) NUMBITS(1) [],
//     /// BKPSRAMEN
//     BKPSRAMEN OFFSET(8) NUMBITS(1) []
// ],
// MC_AHB6ENSETR [
//     /// MDMAEN
//     MDMAEN OFFSET(0) NUMBITS(1) [],
//     /// GPUEN
//     GPUEN OFFSET(5) NUMBITS(1) [],
//     /// ETHCKEN
//     ETHCKEN OFFSET(7) NUMBITS(1) [],
//     /// ETHTXEN
//     ETHTXEN OFFSET(8) NUMBITS(1) [],
//     /// ETHRXEN
//     ETHRXEN OFFSET(9) NUMBITS(1) [],
//     /// ETHMACEN
//     ETHMACEN OFFSET(10) NUMBITS(1) [],
//     /// FMCEN
//     FMCEN OFFSET(12) NUMBITS(1) [],
//     /// QSPIEN
//     QSPIEN OFFSET(14) NUMBITS(1) [],
//     /// SDMMC1EN
//     SDMMC1EN OFFSET(16) NUMBITS(1) [],
//     /// SDMMC2EN
//     SDMMC2EN OFFSET(17) NUMBITS(1) [],
//     /// CRC1EN
//     CRC1EN OFFSET(20) NUMBITS(1) [],
//     /// USBHEN
//     USBHEN OFFSET(24) NUMBITS(1) []
// ],
// MC_AHB6ENCLRR [
//     /// MDMAEN
//     MDMAEN OFFSET(0) NUMBITS(1) [],
//     /// GPUEN
//     GPUEN OFFSET(5) NUMBITS(1) [],
//     /// ETHCKEN
//     ETHCKEN OFFSET(7) NUMBITS(1) [],
//     /// ETHTXEN
//     ETHTXEN OFFSET(8) NUMBITS(1) [],
//     /// ETHRXEN
//     ETHRXEN OFFSET(9) NUMBITS(1) [],
//     /// ETHMACEN
//     ETHMACEN OFFSET(10) NUMBITS(1) [],
//     /// FMCEN
//     FMCEN OFFSET(12) NUMBITS(1) [],
//     /// QSPIEN
//     QSPIEN OFFSET(14) NUMBITS(1) [],
//     /// SDMMC1EN
//     SDMMC1EN OFFSET(16) NUMBITS(1) [],
//     /// SDMMC2EN
//     SDMMC2EN OFFSET(17) NUMBITS(1) [],
//     /// CRC1EN
//     CRC1EN OFFSET(20) NUMBITS(1) [],
//     /// USBHEN
//     USBHEN OFFSET(24) NUMBITS(1) []
// ],
// MP_APB4LPENSETR [
//     /// LTDCLPEN
//     LTDCLPEN OFFSET(0) NUMBITS(1) [],
//     /// DSILPEN
//     DSILPEN OFFSET(4) NUMBITS(1) [],
//     /// DDRPERFMLPEN
//     DDRPERFMLPEN OFFSET(8) NUMBITS(1) [],
//     /// IWDG2APBLPEN
//     IWDG2APBLPEN OFFSET(15) NUMBITS(1) [],
//     /// USBPHYLPEN
//     USBPHYLPEN OFFSET(16) NUMBITS(1) [],
//     /// STGENROLPEN
//     STGENROLPEN OFFSET(20) NUMBITS(1) [],
//     /// STGENROSTPEN
//     STGENROSTPEN OFFSET(21) NUMBITS(1) []
// ],
// MP_APB4LPENCLRR [
//     /// LTDCLPEN
//     LTDCLPEN OFFSET(0) NUMBITS(1) [],
//     /// DSILPEN
//     DSILPEN OFFSET(4) NUMBITS(1) [],
//     /// DDRPERFMLPEN
//     DDRPERFMLPEN OFFSET(8) NUMBITS(1) [],
//     /// IWDG2APBLPEN
//     IWDG2APBLPEN OFFSET(15) NUMBITS(1) [],
//     /// USBPHYLPEN
//     USBPHYLPEN OFFSET(16) NUMBITS(1) [],
//     /// STGENROLPEN
//     STGENROLPEN OFFSET(20) NUMBITS(1) [],
//     /// STGENROSTPEN
//     STGENROSTPEN OFFSET(21) NUMBITS(1) []
// ],
// MP_APB5LPENSETR [
//     /// SPI6LPEN
//     SPI6LPEN OFFSET(0) NUMBITS(1) [],
//     /// I2C4LPEN
//     I2C4LPEN OFFSET(2) NUMBITS(1) [],
//     /// I2C6LPEN
//     I2C6LPEN OFFSET(3) NUMBITS(1) [],
//     /// USART1LPEN
//     USART1LPEN OFFSET(4) NUMBITS(1) [],
//     /// RTCAPBLPEN
//     RTCAPBLPEN OFFSET(8) NUMBITS(1) [],
//     /// TZC1LPEN
//     TZC1LPEN OFFSET(11) NUMBITS(1) [],
//     /// TZC2LPEN
//     TZC2LPEN OFFSET(12) NUMBITS(1) [],
//     /// TZPCLPEN
//     TZPCLPEN OFFSET(13) NUMBITS(1) [],
//     /// IWDG1APBLPEN
//     IWDG1APBLPEN OFFSET(15) NUMBITS(1) [],
//     /// BSECLPEN
//     BSECLPEN OFFSET(16) NUMBITS(1) [],
//     /// STGENLPEN
//     STGENLPEN OFFSET(20) NUMBITS(1) [],
//     /// STGENSTPEN
//     STGENSTPEN OFFSET(21) NUMBITS(1) []
// ],
// MP_APB5LPENCLRR [
//     /// SPI6LPEN
//     SPI6LPEN OFFSET(0) NUMBITS(1) [],
//     /// I2C4LPEN
//     I2C4LPEN OFFSET(2) NUMBITS(1) [],
//     /// I2C6LPEN
//     I2C6LPEN OFFSET(3) NUMBITS(1) [],
//     /// USART1LPEN
//     USART1LPEN OFFSET(4) NUMBITS(1) [],
//     /// RTCAPBLPEN
//     RTCAPBLPEN OFFSET(8) NUMBITS(1) [],
//     /// TZC1LPEN
//     TZC1LPEN OFFSET(11) NUMBITS(1) [],
//     /// TZC2LPEN
//     TZC2LPEN OFFSET(12) NUMBITS(1) [],
//     /// TZPCLPEN
//     TZPCLPEN OFFSET(13) NUMBITS(1) [],
//     /// IWDG1APBLPEN
//     IWDG1APBLPEN OFFSET(15) NUMBITS(1) [],
//     /// BSECLPEN
//     BSECLPEN OFFSET(16) NUMBITS(1) [],
//     /// STGENLPEN
//     STGENLPEN OFFSET(20) NUMBITS(1) [],
//     /// STGENSTPEN
//     STGENSTPEN OFFSET(21) NUMBITS(1) []
// ],
// MP_AHB5LPENSETR [
//     /// GPIOZLPEN
//     GPIOZLPEN OFFSET(0) NUMBITS(1) [],
//     /// CRYP1LPEN
//     CRYP1LPEN OFFSET(4) NUMBITS(1) [],
//     /// HASH1LPEN
//     HASH1LPEN OFFSET(5) NUMBITS(1) [],
//     /// RNG1LPEN
//     RNG1LPEN OFFSET(6) NUMBITS(1) [],
//     /// BKPSRAMLPEN
//     BKPSRAMLPEN OFFSET(8) NUMBITS(1) []
// ],
// MP_AHB5LPENCLRR [
//     /// GPIOZLPEN
//     GPIOZLPEN OFFSET(0) NUMBITS(1) [],
//     /// CRYP1LPEN
//     CRYP1LPEN OFFSET(4) NUMBITS(1) [],
//     /// HASH1LPEN
//     HASH1LPEN OFFSET(5) NUMBITS(1) [],
//     /// RNG1LPEN
//     RNG1LPEN OFFSET(6) NUMBITS(1) [],
//     /// BKPSRAMLPEN
//     BKPSRAMLPEN OFFSET(8) NUMBITS(1) []
// ],
// MP_AHB6LPENSETR [
//     /// MDMALPEN
//     MDMALPEN OFFSET(0) NUMBITS(1) [],
//     /// GPULPEN
//     GPULPEN OFFSET(5) NUMBITS(1) [],
//     /// ETHCKLPEN
//     ETHCKLPEN OFFSET(7) NUMBITS(1) [],
//     /// ETHTXLPEN
//     ETHTXLPEN OFFSET(8) NUMBITS(1) [],
//     /// ETHRXLPEN
//     ETHRXLPEN OFFSET(9) NUMBITS(1) [],
//     /// ETHMACLPEN
//     ETHMACLPEN OFFSET(10) NUMBITS(1) [],
//     /// ETHSTPEN
//     ETHSTPEN OFFSET(11) NUMBITS(1) [],
//     /// FMCLPEN
//     FMCLPEN OFFSET(12) NUMBITS(1) [],
//     /// QSPILPEN
//     QSPILPEN OFFSET(14) NUMBITS(1) [],
//     /// SDMMC1LPEN
//     SDMMC1LPEN OFFSET(16) NUMBITS(1) [],
//     /// SDMMC2LPEN
//     SDMMC2LPEN OFFSET(17) NUMBITS(1) [],
//     /// CRC1LPEN
//     CRC1LPEN OFFSET(20) NUMBITS(1) [],
//     /// USBHLPEN
//     USBHLPEN OFFSET(24) NUMBITS(1) []
// ],
// MP_AHB6LPENCLRR [
//     /// MDMALPEN
//     MDMALPEN OFFSET(0) NUMBITS(1) [],
//     /// GPULPEN
//     GPULPEN OFFSET(5) NUMBITS(1) [],
//     /// ETHCKLPEN
//     ETHCKLPEN OFFSET(7) NUMBITS(1) [],
//     /// ETHTXLPEN
//     ETHTXLPEN OFFSET(8) NUMBITS(1) [],
//     /// ETHRXLPEN
//     ETHRXLPEN OFFSET(9) NUMBITS(1) [],
//     /// ETHMACLPEN
//     ETHMACLPEN OFFSET(10) NUMBITS(1) [],
//     /// ETHSTPEN
//     ETHSTPEN OFFSET(11) NUMBITS(1) [],
//     /// FMCLPEN
//     FMCLPEN OFFSET(12) NUMBITS(1) [],
//     /// QSPILPEN
//     QSPILPEN OFFSET(14) NUMBITS(1) [],
//     /// SDMMC1LPEN
//     SDMMC1LPEN OFFSET(16) NUMBITS(1) [],
//     /// SDMMC2LPEN
//     SDMMC2LPEN OFFSET(17) NUMBITS(1) [],
//     /// CRC1LPEN
//     CRC1LPEN OFFSET(20) NUMBITS(1) [],
//     /// USBHLPEN
//     USBHLPEN OFFSET(24) NUMBITS(1) []
// ],
// MP_TZAHB6LPENSETR [
//     /// MDMALPEN
//     MDMALPEN OFFSET(0) NUMBITS(1) []
// ],
// MP_TZAHB6LPENCLRR [
//     /// MDMALPEN
//     MDMALPEN OFFSET(0) NUMBITS(1) []
// ],
// MC_APB4LPENSETR [
//     /// LTDCLPEN
//     LTDCLPEN OFFSET(0) NUMBITS(1) [],
//     /// DSILPEN
//     DSILPEN OFFSET(4) NUMBITS(1) [],
//     /// DDRPERFMLPEN
//     DDRPERFMLPEN OFFSET(8) NUMBITS(1) [],
//     /// USBPHYLPEN
//     USBPHYLPEN OFFSET(16) NUMBITS(1) [],
//     /// STGENROLPEN
//     STGENROLPEN OFFSET(20) NUMBITS(1) [],
//     /// STGENROSTPEN
//     STGENROSTPEN OFFSET(21) NUMBITS(1) []
// ],
// MC_APB4LPENCLRR [
//     /// LTDCLPEN
//     LTDCLPEN OFFSET(0) NUMBITS(1) [],
//     /// DSILPEN
//     DSILPEN OFFSET(4) NUMBITS(1) [],
//     /// DDRPERFMLPEN
//     DDRPERFMLPEN OFFSET(8) NUMBITS(1) [],
//     /// USBPHYLPEN
//     USBPHYLPEN OFFSET(16) NUMBITS(1) [],
//     /// STGENROLPEN
//     STGENROLPEN OFFSET(20) NUMBITS(1) [],
//     /// STGENROSTPEN
//     STGENROSTPEN OFFSET(21) NUMBITS(1) []
// ],
// MC_APB5LPENSETR [
//     /// SPI6LPEN
//     SPI6LPEN OFFSET(0) NUMBITS(1) [],
//     /// I2C4LPEN
//     I2C4LPEN OFFSET(2) NUMBITS(1) [],
//     /// I2C6LPEN
//     I2C6LPEN OFFSET(3) NUMBITS(1) [],
//     /// USART1LPEN
//     USART1LPEN OFFSET(4) NUMBITS(1) [],
//     /// RTCAPBLPEN
//     RTCAPBLPEN OFFSET(8) NUMBITS(1) [],
//     /// TZC1LPEN
//     TZC1LPEN OFFSET(11) NUMBITS(1) [],
//     /// TZC2LPEN
//     TZC2LPEN OFFSET(12) NUMBITS(1) [],
//     /// TZPCLPEN
//     TZPCLPEN OFFSET(13) NUMBITS(1) [],
//     /// BSECLPEN
//     BSECLPEN OFFSET(16) NUMBITS(1) [],
//     /// STGENLPEN
//     STGENLPEN OFFSET(20) NUMBITS(1) [],
//     /// STGENSTPEN
//     STGENSTPEN OFFSET(21) NUMBITS(1) []
// ],
// MC_APB5LPENCLRR [
//     /// SPI6LPEN
//     SPI6LPEN OFFSET(0) NUMBITS(1) [],
//     /// I2C4LPEN
//     I2C4LPEN OFFSET(2) NUMBITS(1) [],
//     /// I2C6LPEN
//     I2C6LPEN OFFSET(3) NUMBITS(1) [],
//     /// USART1LPEN
//     USART1LPEN OFFSET(4) NUMBITS(1) [],
//     /// RTCAPBLPEN
//     RTCAPBLPEN OFFSET(8) NUMBITS(1) [],
//     /// TZC1LPEN
//     TZC1LPEN OFFSET(11) NUMBITS(1) [],
//     /// TZC2LPEN
//     TZC2LPEN OFFSET(12) NUMBITS(1) [],
//     /// TZPCLPEN
//     TZPCLPEN OFFSET(13) NUMBITS(1) [],
//     /// BSECLPEN
//     BSECLPEN OFFSET(16) NUMBITS(1) [],
//     /// STGENLPEN
//     STGENLPEN OFFSET(20) NUMBITS(1) [],
//     /// STGENSTPEN
//     STGENSTPEN OFFSET(21) NUMBITS(1) []
// ],
// MC_AHB5LPENSETR [
//     /// GPIOZLPEN
//     GPIOZLPEN OFFSET(0) NUMBITS(1) [],
//     /// CRYP1LPEN
//     CRYP1LPEN OFFSET(4) NUMBITS(1) [],
//     /// HASH1LPEN
//     HASH1LPEN OFFSET(5) NUMBITS(1) [],
//     /// RNG1LPEN
//     RNG1LPEN OFFSET(6) NUMBITS(1) [],
//     /// BKPSRAMLPEN
//     BKPSRAMLPEN OFFSET(8) NUMBITS(1) []
// ],
// MC_AHB5LPENCLRR [
//     /// GPIOZLPEN
//     GPIOZLPEN OFFSET(0) NUMBITS(1) [],
//     /// CRYP1LPEN
//     CRYP1LPEN OFFSET(4) NUMBITS(1) [],
//     /// HASH1LPEN
//     HASH1LPEN OFFSET(5) NUMBITS(1) [],
//     /// RNG1LPEN
//     RNG1LPEN OFFSET(6) NUMBITS(1) [],
//     /// BKPSRAMLPEN
//     BKPSRAMLPEN OFFSET(8) NUMBITS(1) []
// ],
// MC_AHB6LPENSETR [
//     /// MDMALPEN
//     MDMALPEN OFFSET(0) NUMBITS(1) [],
//     /// GPULPEN
//     GPULPEN OFFSET(5) NUMBITS(1) [],
//     /// ETHCKLPEN
//     ETHCKLPEN OFFSET(7) NUMBITS(1) [],
//     /// ETHTXLPEN
//     ETHTXLPEN OFFSET(8) NUMBITS(1) [],
//     /// ETHRXLPEN
//     ETHRXLPEN OFFSET(9) NUMBITS(1) [],
//     /// ETHMACLPEN
//     ETHMACLPEN OFFSET(10) NUMBITS(1) [],
//     /// ETHSTPEN
//     ETHSTPEN OFFSET(11) NUMBITS(1) [],
//     /// FMCLPEN
//     FMCLPEN OFFSET(12) NUMBITS(1) [],
//     /// QSPILPEN
//     QSPILPEN OFFSET(14) NUMBITS(1) [],
//     /// SDMMC1LPEN
//     SDMMC1LPEN OFFSET(16) NUMBITS(1) [],
//     /// SDMMC2LPEN
//     SDMMC2LPEN OFFSET(17) NUMBITS(1) [],
//     /// CRC1LPEN
//     CRC1LPEN OFFSET(20) NUMBITS(1) [],
//     /// USBHLPEN
//     USBHLPEN OFFSET(24) NUMBITS(1) []
// ],
// MC_AHB6LPENCLRR [
//     /// MDMALPEN
//     MDMALPEN OFFSET(0) NUMBITS(1) [],
//     /// GPULPEN
//     GPULPEN OFFSET(5) NUMBITS(1) [],
//     /// ETHCKLPEN
//     ETHCKLPEN OFFSET(7) NUMBITS(1) [],
//     /// ETHTXLPEN
//     ETHTXLPEN OFFSET(8) NUMBITS(1) [],
//     /// ETHRXLPEN
//     ETHRXLPEN OFFSET(9) NUMBITS(1) [],
//     /// ETHMACLPEN
//     ETHMACLPEN OFFSET(10) NUMBITS(1) [],
//     /// ETHSTPEN
//     ETHSTPEN OFFSET(11) NUMBITS(1) [],
//     /// FMCLPEN
//     FMCLPEN OFFSET(12) NUMBITS(1) [],
//     /// QSPILPEN
//     QSPILPEN OFFSET(14) NUMBITS(1) [],
//     /// SDMMC1LPEN
//     SDMMC1LPEN OFFSET(16) NUMBITS(1) [],
//     /// SDMMC2LPEN
//     SDMMC2LPEN OFFSET(17) NUMBITS(1) [],
//     /// CRC1LPEN
//     CRC1LPEN OFFSET(20) NUMBITS(1) [],
//     /// USBHLPEN
//     USBHLPEN OFFSET(24) NUMBITS(1) []
// ],
// BR_RSTSCLRR [
//     /// PORRSTF
//     PORRSTF OFFSET(0) NUMBITS(1) [],
//     /// BORRSTF
//     BORRSTF OFFSET(1) NUMBITS(1) [],
//     /// PADRSTF
//     PADRSTF OFFSET(2) NUMBITS(1) [],
//     /// HCSSRSTF
//     HCSSRSTF OFFSET(3) NUMBITS(1) [],
//     /// VCORERSTF
//     VCORERSTF OFFSET(4) NUMBITS(1) [],
//     /// MPSYSRSTF
//     MPSYSRSTF OFFSET(6) NUMBITS(1) [],
//     /// MCSYSRSTF
//     MCSYSRSTF OFFSET(7) NUMBITS(1) [],
//     /// IWDG1RSTF
//     IWDG1RSTF OFFSET(8) NUMBITS(1) [],
//     /// IWDG2RSTF
//     IWDG2RSTF OFFSET(9) NUMBITS(1) [],
//     /// MPUP0RSTF
//     MPUP0RSTF OFFSET(13) NUMBITS(1) [],
//     /// MPUP1RSTF
//     MPUP1RSTF OFFSET(14) NUMBITS(1) []
// ],
// MP_GRSTCSETR [
//     /// MPSYSRST
//     MPSYSRST OFFSET(0) NUMBITS(1) [],
//     /// MCURST
//     MCURST OFFSET(1) NUMBITS(1) [],
//     /// MPUP0RST
//     MPUP0RST OFFSET(4) NUMBITS(1) [],
//     /// MPUP1RST
//     MPUP1RST OFFSET(5) NUMBITS(1) []
// ],
// MP_RSTSCLRR [
//     /// PORRSTF
//     PORRSTF OFFSET(0) NUMBITS(1) [],
//     /// BORRSTF
//     BORRSTF OFFSET(1) NUMBITS(1) [],
//     /// PADRSTF
//     PADRSTF OFFSET(2) NUMBITS(1) [],
//     /// HCSSRSTF
//     HCSSRSTF OFFSET(3) NUMBITS(1) [],
//     /// VCORERSTF
//     VCORERSTF OFFSET(4) NUMBITS(1) [],
//     /// MPSYSRSTF
//     MPSYSRSTF OFFSET(6) NUMBITS(1) [],
//     /// MCSYSRSTF
//     MCSYSRSTF OFFSET(7) NUMBITS(1) [],
//     /// IWDG1RSTF
//     IWDG1RSTF OFFSET(8) NUMBITS(1) [],
//     /// IWDG2RSTF
//     IWDG2RSTF OFFSET(9) NUMBITS(1) [],
//     /// STDBYRSTF
//     STDBYRSTF OFFSET(11) NUMBITS(1) [],
//     /// CSTDBYRSTF
//     CSTDBYRSTF OFFSET(12) NUMBITS(1) [],
//     /// MPUP0RSTF
//     MPUP0RSTF OFFSET(13) NUMBITS(1) [],
//     /// MPUP1RSTF
//     MPUP1RSTF OFFSET(14) NUMBITS(1) [],
//     /// SPARE
//     SPARE OFFSET(15) NUMBITS(1) []
// ],
// MP_IWDGFZSETR [
//     /// FZ_IWDG1
//     FZ_IWDG1 OFFSET(0) NUMBITS(1) [],
//     /// FZ_IWDG2
//     FZ_IWDG2 OFFSET(1) NUMBITS(1) []
// ],
// MP_IWDGFZCLRR [
//     /// FZ_IWDG1
//     FZ_IWDG1 OFFSET(0) NUMBITS(1) [],
//     /// FZ_IWDG2
//     FZ_IWDG2 OFFSET(1) NUMBITS(1) []
// ],
// MP_CIER [
//     /// LSIRDYIE
//     LSIRDYIE OFFSET(0) NUMBITS(1) [],
//     /// LSERDYIE
//     LSERDYIE OFFSET(1) NUMBITS(1) [],
//     /// HSIRDYIE
//     HSIRDYIE OFFSET(2) NUMBITS(1) [],
//     /// HSERDYIE
//     HSERDYIE OFFSET(3) NUMBITS(1) [],
//     /// CSIRDYIE
//     CSIRDYIE OFFSET(4) NUMBITS(1) [],
//     /// PLL1DYIE
//     PLL1DYIE OFFSET(8) NUMBITS(1) [],
//     /// PLL2DYIE
//     PLL2DYIE OFFSET(9) NUMBITS(1) [],
//     /// PLL3DYIE
//     PLL3DYIE OFFSET(10) NUMBITS(1) [],
//     /// PLL4DYIE
//     PLL4DYIE OFFSET(11) NUMBITS(1) [],
//     /// LSECSSIE
//     LSECSSIE OFFSET(16) NUMBITS(1) [],
//     /// WKUPIE
//     WKUPIE OFFSET(20) NUMBITS(1) []
// ],
// MP_CIFR [
//     /// LSIRDYF
//     LSIRDYF OFFSET(0) NUMBITS(1) [],
//     /// LSERDYF
//     LSERDYF OFFSET(1) NUMBITS(1) [],
//     /// HSIRDYF
//     HSIRDYF OFFSET(2) NUMBITS(1) [],
//     /// HSERDYF
//     HSERDYF OFFSET(3) NUMBITS(1) [],
//     /// CSIRDYF
//     CSIRDYF OFFSET(4) NUMBITS(1) [],
//     /// PLL1DYF
//     PLL1DYF OFFSET(8) NUMBITS(1) [],
//     /// PLL2DYF
//     PLL2DYF OFFSET(9) NUMBITS(1) [],
//     /// PLL3DYF
//     PLL3DYF OFFSET(10) NUMBITS(1) [],
//     /// PLL4DYF
//     PLL4DYF OFFSET(11) NUMBITS(1) [],
//     /// LSECSSF
//     LSECSSF OFFSET(16) NUMBITS(1) [],
//     /// WKUPF
//     WKUPF OFFSET(20) NUMBITS(1) []
// ],
// PWRLPDLYCR [
//     /// PWRLP_DLY
//     PWRLP_DLY OFFSET(0) NUMBITS(22) [],
//     /// MCTMPSKP
//     MCTMPSKP OFFSET(24) NUMBITS(1) []
// ],
// MP_RSTSSETR [
//     /// PORRSTF
//     PORRSTF OFFSET(0) NUMBITS(1) [],
//     /// BORRSTF
//     BORRSTF OFFSET(1) NUMBITS(1) [],
//     /// PADRSTF
//     PADRSTF OFFSET(2) NUMBITS(1) [],
//     /// HCSSRSTF
//     HCSSRSTF OFFSET(3) NUMBITS(1) [],
//     /// VCORERSTF
//     VCORERSTF OFFSET(4) NUMBITS(1) [],
//     /// MPSYSRSTF
//     MPSYSRSTF OFFSET(6) NUMBITS(1) [],
//     /// MCSYSRSTF
//     MCSYSRSTF OFFSET(7) NUMBITS(1) [],
//     /// IWDG1RSTF
//     IWDG1RSTF OFFSET(8) NUMBITS(1) [],
//     /// IWDG2RSTF
//     IWDG2RSTF OFFSET(9) NUMBITS(1) [],
//     /// STDBYRSTF
//     STDBYRSTF OFFSET(11) NUMBITS(1) [],
//     /// CSTDBYRSTF
//     CSTDBYRSTF OFFSET(12) NUMBITS(1) [],
//     /// MPUP0RSTF
//     MPUP0RSTF OFFSET(13) NUMBITS(1) [],
//     /// MPUP1RSTF
//     MPUP1RSTF OFFSET(14) NUMBITS(1) [],
//     /// SPARE
//     SPARE OFFSET(15) NUMBITS(1) []
// ],
// MCO1CFGR [
//     /// MCO1SEL
//     MCO1SEL OFFSET(0) NUMBITS(3) [],
//     /// MCO1DIV
//     MCO1DIV OFFSET(4) NUMBITS(4) [],
//     /// MCO1ON
//     MCO1ON OFFSET(12) NUMBITS(1) []
// ],
// MCO2CFGR [
//     /// MCO2SEL
//     MCO2SEL OFFSET(0) NUMBITS(3) [],
//     /// MCO2DIV
//     MCO2DIV OFFSET(4) NUMBITS(4) [],
//     /// MCO2ON
//     MCO2ON OFFSET(12) NUMBITS(1) []
// ],
// OCRDYR [
//     /// HSIRDY
//     HSIRDY OFFSET(0) NUMBITS(1) [],
//     /// HSIDIVRDY
//     HSIDIVRDY OFFSET(2) NUMBITS(1) [],
//     /// CSIRDY
//     CSIRDY OFFSET(4) NUMBITS(1) [],
//     /// HSERDY
//     HSERDY OFFSET(8) NUMBITS(1) [],
//     /// MPUCKRDY
//     MPUCKRDY OFFSET(23) NUMBITS(1) [],
//     /// AXICKRDY
//     AXICKRDY OFFSET(24) NUMBITS(1) [],
//     /// CKREST
//     CKREST OFFSET(25) NUMBITS(1) []
// ],
// DBGCFGR [
//     /// TRACEDIV
//     TRACEDIV OFFSET(0) NUMBITS(3) [],
//     /// DBGCKEN
//     DBGCKEN OFFSET(8) NUMBITS(1) [],
//     /// TRACECKEN
//     TRACECKEN OFFSET(9) NUMBITS(1) [],
//     /// DBGRST
//     DBGRST OFFSET(12) NUMBITS(1) []
// ],
// RCK3SELR [
//     /// PLL3SRC
//     PLL3SRC OFFSET(0) NUMBITS(2) [],
//     /// PLL3SRCRDY
//     PLL3SRCRDY OFFSET(31) NUMBITS(1) []
// ],
// RCK4SELR [
//     /// PLL4SRC
//     PLL4SRC OFFSET(0) NUMBITS(2) [],
//     /// PLL4SRCRDY
//     PLL4SRCRDY OFFSET(31) NUMBITS(1) []
// ],
// TIMG1PRER [
//     /// TIMG1PRE
//     TIMG1PRE OFFSET(0) NUMBITS(1) [],
//     /// TIMG1PRERDY
//     TIMG1PRERDY OFFSET(31) NUMBITS(1) []
// ],
// TIMG2PRER [
//     /// TIMG2PRE
//     TIMG2PRE OFFSET(0) NUMBITS(1) [],
//     /// TIMG2PRERDY
//     TIMG2PRERDY OFFSET(31) NUMBITS(1) []
// ],
// MCUDIVR [
//     /// MCUDIV
//     MCUDIV OFFSET(0) NUMBITS(4) [],
//     /// MCUDIVRDY
//     MCUDIVRDY OFFSET(31) NUMBITS(1) []
// ],
// APB1DIVR [
//     /// APB1DIV
//     APB1DIV OFFSET(0) NUMBITS(3) [],
//     /// APB1DIVRDY
//     APB1DIVRDY OFFSET(31) NUMBITS(1) []
// ],
// APB2DIVR [
//     /// APB2DIV
//     APB2DIV OFFSET(0) NUMBITS(3) [],
//     /// APB2DIVRDY
//     APB2DIVRDY OFFSET(31) NUMBITS(1) []
// ],
// APB3DIVR [
//     /// APB3DIV
//     APB3DIV OFFSET(0) NUMBITS(3) [],
//     /// APB3DIVRDY
//     APB3DIVRDY OFFSET(31) NUMBITS(1) []
// ],
// PLL3CR [
//     /// PLLON
//     PLLON OFFSET(0) NUMBITS(1) [],
//     /// PLL3RDY
//     PLL3RDY OFFSET(1) NUMBITS(1) [],
//     /// SSCG_CTRL
//     SSCG_CTRL OFFSET(2) NUMBITS(1) [],
//     /// DIVPEN
//     DIVPEN OFFSET(4) NUMBITS(1) [],
//     /// DIVQEN
//     DIVQEN OFFSET(5) NUMBITS(1) [],
//     /// DIVREN
//     DIVREN OFFSET(6) NUMBITS(1) []
// ],
// PLL3CFGR1 [
//     /// DIVN
//     DIVN OFFSET(0) NUMBITS(9) [],
//     /// DIVM3
//     DIVM3 OFFSET(16) NUMBITS(6) [],
//     /// IFRGE
//     IFRGE OFFSET(24) NUMBITS(2) []
// ],
// PLL3CFGR2 [
//     /// DIVP
//     DIVP OFFSET(0) NUMBITS(7) [],
//     /// DIVQ
//     DIVQ OFFSET(8) NUMBITS(7) [],
//     /// DIVR
//     DIVR OFFSET(16) NUMBITS(7) []
// ],
// PLL3FRACR [
//     /// FRACV
//     FRACV OFFSET(3) NUMBITS(13) [],
//     /// FRACLE
//     FRACLE OFFSET(16) NUMBITS(1) []
// ],
// PLL3CSGR [
//     /// MOD_PER
//     MOD_PER OFFSET(0) NUMBITS(13) [],
//     /// TPDFN_DIS
//     TPDFN_DIS OFFSET(13) NUMBITS(1) [],
//     /// RPDFN_DIS
//     RPDFN_DIS OFFSET(14) NUMBITS(1) [],
//     /// SSCG_MODE
//     SSCG_MODE OFFSET(15) NUMBITS(1) [],
//     /// INC_STEP
//     INC_STEP OFFSET(16) NUMBITS(15) []
// ],
// PLL4CR [
//     /// PLLON
//     PLLON OFFSET(0) NUMBITS(1) [],
//     /// PLL4RDY
//     PLL4RDY OFFSET(1) NUMBITS(1) [],
//     /// SSCG_CTRL
//     SSCG_CTRL OFFSET(2) NUMBITS(1) [],
//     /// DIVPEN
//     DIVPEN OFFSET(4) NUMBITS(1) [],
//     /// DIVQEN
//     DIVQEN OFFSET(5) NUMBITS(1) [],
//     /// DIVREN
//     DIVREN OFFSET(6) NUMBITS(1) []
// ],
// PLL4CFGR1 [
//     /// DIVN
//     DIVN OFFSET(0) NUMBITS(9) [],
//     /// DIVM4
//     DIVM4 OFFSET(16) NUMBITS(6) [],
//     /// IFRGE
//     IFRGE OFFSET(24) NUMBITS(2) []
// ],
// PLL4CFGR2 [
//     /// DIVP
//     DIVP OFFSET(0) NUMBITS(7) [],
//     /// DIVQ
//     DIVQ OFFSET(8) NUMBITS(7) [],
//     /// DIVR
//     DIVR OFFSET(16) NUMBITS(7) []
// ],
// PLL4FRACR [
//     /// FRACV
//     FRACV OFFSET(3) NUMBITS(13) [],
//     /// FRACLE
//     FRACLE OFFSET(16) NUMBITS(1) []
// ],
// PLL4CSGR [
//     /// MOD_PER
//     MOD_PER OFFSET(0) NUMBITS(13) [],
//     /// TPDFN_DIS
//     TPDFN_DIS OFFSET(13) NUMBITS(1) [],
//     /// RPDFN_DIS
//     RPDFN_DIS OFFSET(14) NUMBITS(1) [],
//     /// SSCG_MODE
//     SSCG_MODE OFFSET(15) NUMBITS(1) [],
//     /// INC_STEP
//     INC_STEP OFFSET(16) NUMBITS(15) []
// ],
// I2C12CKSELR [
//     /// I2C12SRC
//     I2C12SRC OFFSET(0) NUMBITS(3) []
// ],
// I2C35CKSELR [
//     /// I2C35SRC
//     I2C35SRC OFFSET(0) NUMBITS(3) []
// ],
// SAI1CKSELR [
//     /// SAI1SRC
//     SAI1SRC OFFSET(0) NUMBITS(3) []
// ],
// SAI2CKSELR [
//     /// SAI2SRC
//     SAI2SRC OFFSET(0) NUMBITS(3) []
// ],
// SAI3CKSELR [
//     /// SAI3SRC
//     SAI3SRC OFFSET(0) NUMBITS(3) []
// ],
// SAI4CKSELR [
//     /// SAI4SRC
//     SAI4SRC OFFSET(0) NUMBITS(3) []
// ],
// SPI2S1CKSELR [
//     /// SPI1SRC
//     SPI1SRC OFFSET(0) NUMBITS(3) []
// ],
// SPI2S23CKSELR [
//     /// SPI23SRC
//     SPI23SRC OFFSET(0) NUMBITS(3) []
// ],
// SPI45CKSELR [
//     /// SPI45SRC
//     SPI45SRC OFFSET(0) NUMBITS(3) []
// ],
// UART6CKSELR [
//     /// UART6SRC
//     UART6SRC OFFSET(0) NUMBITS(3) []
// ],
// UART24CKSELR [
//     /// UART24SRC
//     UART24SRC OFFSET(0) NUMBITS(3) []
// ],
// UART35CKSELR [
//     /// UART35SRC
//     UART35SRC OFFSET(0) NUMBITS(3) []
// ],
// UART78CKSELR [
//     /// UART78SRC
//     UART78SRC OFFSET(0) NUMBITS(3) []
// ],
// SDMMC12CKSELR [
//     /// SDMMC12SRC
//     SDMMC12SRC OFFSET(0) NUMBITS(3) []
// ],
// SDMMC3CKSELR [
//     /// SDMMC3SRC
//     SDMMC3SRC OFFSET(0) NUMBITS(3) []
// ],
// ETHCKSELR [
//     /// ETHSRC
//     ETHSRC OFFSET(0) NUMBITS(2) [],
//     /// ETHPTPDIV
//     ETHPTPDIV OFFSET(4) NUMBITS(4) []
// ],
// QSPICKSELR [
//     /// QSPISRC
//     QSPISRC OFFSET(0) NUMBITS(2) []
// ],
// FMCCKSELR [
//     /// FMCSRC
//     FMCSRC OFFSET(0) NUMBITS(2) []
// ],
// FDCANCKSELR [
//     /// FDCANSRC
//     FDCANSRC OFFSET(0) NUMBITS(2) []
// ],
// SPDIFCKSELR [
//     /// SPDIFSRC
//     SPDIFSRC OFFSET(0) NUMBITS(2) []
// ],
// CECCKSELR [
//     /// CECSRC
//     CECSRC OFFSET(0) NUMBITS(2) []
// ],
// USBCKSELR [
//     /// USBPHYSRC
//     USBPHYSRC OFFSET(0) NUMBITS(2) [],
//     /// USBOSRC
//     USBOSRC OFFSET(4) NUMBITS(1) []
// ],
// RNG2CKSELR [
//     /// RNG2SRC
//     RNG2SRC OFFSET(0) NUMBITS(2) []
// ],
// DSICKSELR [
//     /// DSISRC
//     DSISRC OFFSET(0) NUMBITS(1) []
// ],
// ADCCKSELR [
//     /// ADCSRC
//     ADCSRC OFFSET(0) NUMBITS(2) []
// ],
// LPTIM45CKSELR [
//     /// LPTIM45SRC
//     LPTIM45SRC OFFSET(0) NUMBITS(3) []
// ],
// LPTIM23CKSELR [
//     /// LPTIM23SRC
//     LPTIM23SRC OFFSET(0) NUMBITS(3) []
// ],
// LPTIM1CKSELR [
//     /// LPTIM1SRC
//     LPTIM1SRC OFFSET(0) NUMBITS(3) []
// ],
// APB1RSTSETR [
//     /// TIM2RST
//     TIM2RST OFFSET(0) NUMBITS(1) [],
//     /// TIM3RST
//     TIM3RST OFFSET(1) NUMBITS(1) [],
//     /// TIM4RST
//     TIM4RST OFFSET(2) NUMBITS(1) [],
//     /// TIM5RST
//     TIM5RST OFFSET(3) NUMBITS(1) [],
//     /// TIM6RST
//     TIM6RST OFFSET(4) NUMBITS(1) [],
//     /// TIM7RST
//     TIM7RST OFFSET(5) NUMBITS(1) [],
//     /// TIM12RST
//     TIM12RST OFFSET(6) NUMBITS(1) [],
//     /// TIM13RST
//     TIM13RST OFFSET(7) NUMBITS(1) [],
//     /// TIM14RST
//     TIM14RST OFFSET(8) NUMBITS(1) [],
//     /// LPTIM1RST
//     LPTIM1RST OFFSET(9) NUMBITS(1) [],
//     /// SPI2RST
//     SPI2RST OFFSET(11) NUMBITS(1) [],
//     /// SPI3RST
//     SPI3RST OFFSET(12) NUMBITS(1) [],
//     /// USART2RST
//     USART2RST OFFSET(14) NUMBITS(1) [],
//     /// USART3RST
//     USART3RST OFFSET(15) NUMBITS(1) [],
//     /// UART4RST
//     UART4RST OFFSET(16) NUMBITS(1) [],
//     /// UART5RST
//     UART5RST OFFSET(17) NUMBITS(1) [],
//     /// UART7RST
//     UART7RST OFFSET(18) NUMBITS(1) [],
//     /// UART8RST
//     UART8RST OFFSET(19) NUMBITS(1) [],
//     /// I2C1RST
//     I2C1RST OFFSET(21) NUMBITS(1) [],
//     /// I2C2RST
//     I2C2RST OFFSET(22) NUMBITS(1) [],
//     /// I2C3RST
//     I2C3RST OFFSET(23) NUMBITS(1) [],
//     /// I2C5RST
//     I2C5RST OFFSET(24) NUMBITS(1) [],
//     /// SPDIFRST
//     SPDIFRST OFFSET(26) NUMBITS(1) [],
//     /// CECRST
//     CECRST OFFSET(27) NUMBITS(1) [],
//     /// DAC12RST
//     DAC12RST OFFSET(29) NUMBITS(1) [],
//     /// MDIOSRST
//     MDIOSRST OFFSET(31) NUMBITS(1) []
// ],
// APB1RSTCLRR [
//     /// TIM2RST
//     TIM2RST OFFSET(0) NUMBITS(1) [],
//     /// TIM3RST
//     TIM3RST OFFSET(1) NUMBITS(1) [],
//     /// TIM4RST
//     TIM4RST OFFSET(2) NUMBITS(1) [],
//     /// TIM5RST
//     TIM5RST OFFSET(3) NUMBITS(1) [],
//     /// TIM6RST
//     TIM6RST OFFSET(4) NUMBITS(1) [],
//     /// TIM7RST
//     TIM7RST OFFSET(5) NUMBITS(1) [],
//     /// TIM12RST
//     TIM12RST OFFSET(6) NUMBITS(1) [],
//     /// TIM13RST
//     TIM13RST OFFSET(7) NUMBITS(1) [],
//     /// TIM14RST
//     TIM14RST OFFSET(8) NUMBITS(1) [],
//     /// LPTIM1RST
//     LPTIM1RST OFFSET(9) NUMBITS(1) [],
//     /// SPI2RST
//     SPI2RST OFFSET(11) NUMBITS(1) [],
//     /// SPI3RST
//     SPI3RST OFFSET(12) NUMBITS(1) [],
//     /// USART2RST
//     USART2RST OFFSET(14) NUMBITS(1) [],
//     /// USART3RST
//     USART3RST OFFSET(15) NUMBITS(1) [],
//     /// UART4RST
//     UART4RST OFFSET(16) NUMBITS(1) [],
//     /// UART5RST
//     UART5RST OFFSET(17) NUMBITS(1) [],
//     /// UART7RST
//     UART7RST OFFSET(18) NUMBITS(1) [],
//     /// UART8RST
//     UART8RST OFFSET(19) NUMBITS(1) [],
//     /// I2C1RST
//     I2C1RST OFFSET(21) NUMBITS(1) [],
//     /// I2C2RST
//     I2C2RST OFFSET(22) NUMBITS(1) [],
//     /// I2C3RST
//     I2C3RST OFFSET(23) NUMBITS(1) [],
//     /// I2C5RST
//     I2C5RST OFFSET(24) NUMBITS(1) [],
//     /// SPDIFRST
//     SPDIFRST OFFSET(26) NUMBITS(1) [],
//     /// CECRST
//     CECRST OFFSET(27) NUMBITS(1) [],
//     /// DAC12RST
//     DAC12RST OFFSET(29) NUMBITS(1) [],
//     /// MDIOSRST
//     MDIOSRST OFFSET(31) NUMBITS(1) []
// ],
// APB2RSTSETR [
//     /// TIM1RST
//     TIM1RST OFFSET(0) NUMBITS(1) [],
//     /// TIM8RST
//     TIM8RST OFFSET(1) NUMBITS(1) [],
//     /// TIM15RST
//     TIM15RST OFFSET(2) NUMBITS(1) [],
//     /// TIM16RST
//     TIM16RST OFFSET(3) NUMBITS(1) [],
//     /// TIM17RST
//     TIM17RST OFFSET(4) NUMBITS(1) [],
//     /// SPI1RST
//     SPI1RST OFFSET(8) NUMBITS(1) [],
//     /// SPI4RST
//     SPI4RST OFFSET(9) NUMBITS(1) [],
//     /// SPI5RST
//     SPI5RST OFFSET(10) NUMBITS(1) [],
//     /// USART6RST
//     USART6RST OFFSET(13) NUMBITS(1) [],
//     /// SAI1RST
//     SAI1RST OFFSET(16) NUMBITS(1) [],
//     /// SAI2RST
//     SAI2RST OFFSET(17) NUMBITS(1) [],
//     /// SAI3RST
//     SAI3RST OFFSET(18) NUMBITS(1) [],
//     /// DFSDMRST
//     DFSDMRST OFFSET(20) NUMBITS(1) [],
//     /// FDCANRST
//     FDCANRST OFFSET(24) NUMBITS(1) []
// ],
// APB2RSTCLRR [
//     /// TIM1RST
//     TIM1RST OFFSET(0) NUMBITS(1) [],
//     /// TIM8RST
//     TIM8RST OFFSET(1) NUMBITS(1) [],
//     /// TIM15RST
//     TIM15RST OFFSET(2) NUMBITS(1) [],
//     /// TIM16RST
//     TIM16RST OFFSET(3) NUMBITS(1) [],
//     /// TIM17RST
//     TIM17RST OFFSET(4) NUMBITS(1) [],
//     /// SPI1RST
//     SPI1RST OFFSET(8) NUMBITS(1) [],
//     /// SPI4RST
//     SPI4RST OFFSET(9) NUMBITS(1) [],
//     /// SPI5RST
//     SPI5RST OFFSET(10) NUMBITS(1) [],
//     /// USART6RST
//     USART6RST OFFSET(13) NUMBITS(1) [],
//     /// SAI1RST
//     SAI1RST OFFSET(16) NUMBITS(1) [],
//     /// SAI2RST
//     SAI2RST OFFSET(17) NUMBITS(1) [],
//     /// SAI3RST
//     SAI3RST OFFSET(18) NUMBITS(1) [],
//     /// DFSDMRST
//     DFSDMRST OFFSET(20) NUMBITS(1) [],
//     /// FDCANRST
//     FDCANRST OFFSET(24) NUMBITS(1) []
// ],
// APB3RSTSETR [
//     /// LPTIM2RST
//     LPTIM2RST OFFSET(0) NUMBITS(1) [],
//     /// LPTIM3RST
//     LPTIM3RST OFFSET(1) NUMBITS(1) [],
//     /// LPTIM4RST
//     LPTIM4RST OFFSET(2) NUMBITS(1) [],
//     /// LPTIM5RST
//     LPTIM5RST OFFSET(3) NUMBITS(1) [],
//     /// SAI4RST
//     SAI4RST OFFSET(8) NUMBITS(1) [],
//     /// SYSCFGRST
//     SYSCFGRST OFFSET(11) NUMBITS(1) [],
//     /// VREFRST
//     VREFRST OFFSET(13) NUMBITS(1) [],
//     /// DTSRST
//     DTSRST OFFSET(16) NUMBITS(1) []
// ],
// APB3RSTCLRR [
//     /// LPTIM2RST
//     LPTIM2RST OFFSET(0) NUMBITS(1) [],
//     /// LPTIM3RST
//     LPTIM3RST OFFSET(1) NUMBITS(1) [],
//     /// LPTIM4RST
//     LPTIM4RST OFFSET(2) NUMBITS(1) [],
//     /// LPTIM5RST
//     LPTIM5RST OFFSET(3) NUMBITS(1) [],
//     /// SAI4RST
//     SAI4RST OFFSET(8) NUMBITS(1) [],
//     /// SYSCFGRST
//     SYSCFGRST OFFSET(11) NUMBITS(1) [],
//     /// VREFRST
//     VREFRST OFFSET(13) NUMBITS(1) [],
//     /// DTSRST
//     DTSRST OFFSET(16) NUMBITS(1) []
// ],
// AHB2RSTSETR [
//     /// DMA1RST
//     DMA1RST OFFSET(0) NUMBITS(1) [],
//     /// DMA2RST
//     DMA2RST OFFSET(1) NUMBITS(1) [],
//     /// DMAMUXRST
//     DMAMUXRST OFFSET(2) NUMBITS(1) [],
//     /// ADC12RST
//     ADC12RST OFFSET(5) NUMBITS(1) [],
//     /// USBORST
//     USBORST OFFSET(8) NUMBITS(1) [],
//     /// SDMMC3RST
//     SDMMC3RST OFFSET(16) NUMBITS(1) []
// ],
// AHB2RSTCLRR [
//     /// DMA1RST
//     DMA1RST OFFSET(0) NUMBITS(1) [],
//     /// DMA2RST
//     DMA2RST OFFSET(1) NUMBITS(1) [],
//     /// DMAMUXRST
//     DMAMUXRST OFFSET(2) NUMBITS(1) [],
//     /// ADC12RST
//     ADC12RST OFFSET(5) NUMBITS(1) [],
//     /// USBORST
//     USBORST OFFSET(8) NUMBITS(1) [],
//     /// SDMMC3RST
//     SDMMC3RST OFFSET(16) NUMBITS(1) []
// ],
// AHB3RSTSETR [
//     /// DCMIRST
//     DCMIRST OFFSET(0) NUMBITS(1) [],
//     /// CRYP2RST
//     CRYP2RST OFFSET(4) NUMBITS(1) [],
//     /// HASH2RST
//     HASH2RST OFFSET(5) NUMBITS(1) [],
//     /// RNG2RST
//     RNG2RST OFFSET(6) NUMBITS(1) [],
//     /// CRC2RST
//     CRC2RST OFFSET(7) NUMBITS(1) [],
//     /// HSEMRST
//     HSEMRST OFFSET(11) NUMBITS(1) [],
//     /// IPCCRST
//     IPCCRST OFFSET(12) NUMBITS(1) []
// ],
// AHB3RSTCLRR [
//     /// DCMIRST
//     DCMIRST OFFSET(0) NUMBITS(1) [],
//     /// CRYP2RST
//     CRYP2RST OFFSET(4) NUMBITS(1) [],
//     /// HASH2RST
//     HASH2RST OFFSET(5) NUMBITS(1) [],
//     /// RNG2RST
//     RNG2RST OFFSET(6) NUMBITS(1) [],
//     /// CRC2RST
//     CRC2RST OFFSET(7) NUMBITS(1) [],
//     /// HSEMRST
//     HSEMRST OFFSET(11) NUMBITS(1) [],
//     /// IPCCRST
//     IPCCRST OFFSET(12) NUMBITS(1) []
// ],
// AHB4RSTSETR [
//     /// GPIOARST
//     GPIOARST OFFSET(0) NUMBITS(1) [],
//     /// GPIOBRST
//     GPIOBRST OFFSET(1) NUMBITS(1) [],
//     /// GPIOCRST
//     GPIOCRST OFFSET(2) NUMBITS(1) [],
//     /// GPIODRST
//     GPIODRST OFFSET(3) NUMBITS(1) [],
//     /// GPIOERST
//     GPIOERST OFFSET(4) NUMBITS(1) [],
//     /// GPIOFRST
//     GPIOFRST OFFSET(5) NUMBITS(1) [],
//     /// GPIOGRST
//     GPIOGRST OFFSET(6) NUMBITS(1) [],
//     /// GPIOHRST
//     GPIOHRST OFFSET(7) NUMBITS(1) [],
//     /// GPIOIRST
//     GPIOIRST OFFSET(8) NUMBITS(1) [],
//     /// GPIOJRST
//     GPIOJRST OFFSET(9) NUMBITS(1) [],
//     /// GPIOKRST
//     GPIOKRST OFFSET(10) NUMBITS(1) []
// ],
// AHB4RSTCLRR [
//     /// GPIOARST
//     GPIOARST OFFSET(0) NUMBITS(1) [],
//     /// GPIOBRST
//     GPIOBRST OFFSET(1) NUMBITS(1) [],
//     /// GPIOCRST
//     GPIOCRST OFFSET(2) NUMBITS(1) [],
//     /// GPIODRST
//     GPIODRST OFFSET(3) NUMBITS(1) [],
//     /// GPIOERST
//     GPIOERST OFFSET(4) NUMBITS(1) [],
//     /// GPIOFRST
//     GPIOFRST OFFSET(5) NUMBITS(1) [],
//     /// GPIOGRST
//     GPIOGRST OFFSET(6) NUMBITS(1) [],
//     /// GPIOHRST
//     GPIOHRST OFFSET(7) NUMBITS(1) [],
//     /// GPIOIRST
//     GPIOIRST OFFSET(8) NUMBITS(1) [],
//     /// GPIOJRST
//     GPIOJRST OFFSET(9) NUMBITS(1) [],
//     /// GPIOKRST
//     GPIOKRST OFFSET(10) NUMBITS(1) []
// ],
// MP_APB1ENSETR [
//     /// TIM2EN
//     TIM2EN OFFSET(0) NUMBITS(1) [],
//     /// TIM3EN
//     TIM3EN OFFSET(1) NUMBITS(1) [],
//     /// TIM4EN
//     TIM4EN OFFSET(2) NUMBITS(1) [],
//     /// TIM5EN
//     TIM5EN OFFSET(3) NUMBITS(1) [],
//     /// TIM6EN
//     TIM6EN OFFSET(4) NUMBITS(1) [],
//     /// TIM7EN
//     TIM7EN OFFSET(5) NUMBITS(1) [],
//     /// TIM12EN
//     TIM12EN OFFSET(6) NUMBITS(1) [],
//     /// TIM13EN
//     TIM13EN OFFSET(7) NUMBITS(1) [],
//     /// TIM14EN
//     TIM14EN OFFSET(8) NUMBITS(1) [],
//     /// LPTIM1EN
//     LPTIM1EN OFFSET(9) NUMBITS(1) [],
//     /// SPI2EN
//     SPI2EN OFFSET(11) NUMBITS(1) [],
//     /// SPI3EN
//     SPI3EN OFFSET(12) NUMBITS(1) [],
//     /// USART2EN
//     USART2EN OFFSET(14) NUMBITS(1) [],
//     /// USART3EN
//     USART3EN OFFSET(15) NUMBITS(1) [],
//     /// UART4EN
//     UART4EN OFFSET(16) NUMBITS(1) [],
//     /// UART5EN
//     UART5EN OFFSET(17) NUMBITS(1) [],
//     /// UART7EN
//     UART7EN OFFSET(18) NUMBITS(1) [],
//     /// UART8EN
//     UART8EN OFFSET(19) NUMBITS(1) [],
//     /// I2C1EN
//     I2C1EN OFFSET(21) NUMBITS(1) [],
//     /// I2C2EN
//     I2C2EN OFFSET(22) NUMBITS(1) [],
//     /// I2C3EN
//     I2C3EN OFFSET(23) NUMBITS(1) [],
//     /// I2C5EN
//     I2C5EN OFFSET(24) NUMBITS(1) [],
//     /// SPDIFEN
//     SPDIFEN OFFSET(26) NUMBITS(1) [],
//     /// CECEN
//     CECEN OFFSET(27) NUMBITS(1) [],
//     /// DAC12EN
//     DAC12EN OFFSET(29) NUMBITS(1) [],
//     /// MDIOSEN
//     MDIOSEN OFFSET(31) NUMBITS(1) []
// ],
// MP_APB1ENCLRR [
//     /// TIM2EN
//     TIM2EN OFFSET(0) NUMBITS(1) [],
//     /// TIM3EN
//     TIM3EN OFFSET(1) NUMBITS(1) [],
//     /// TIM4EN
//     TIM4EN OFFSET(2) NUMBITS(1) [],
//     /// TIM5EN
//     TIM5EN OFFSET(3) NUMBITS(1) [],
//     /// TIM6EN
//     TIM6EN OFFSET(4) NUMBITS(1) [],
//     /// TIM7EN
//     TIM7EN OFFSET(5) NUMBITS(1) [],
//     /// TIM12EN
//     TIM12EN OFFSET(6) NUMBITS(1) [],
//     /// TIM13EN
//     TIM13EN OFFSET(7) NUMBITS(1) [],
//     /// TIM14EN
//     TIM14EN OFFSET(8) NUMBITS(1) [],
//     /// LPTIM1EN
//     LPTIM1EN OFFSET(9) NUMBITS(1) [],
//     /// SPI2EN
//     SPI2EN OFFSET(11) NUMBITS(1) [],
//     /// SPI3EN
//     SPI3EN OFFSET(12) NUMBITS(1) [],
//     /// USART2EN
//     USART2EN OFFSET(14) NUMBITS(1) [],
//     /// USART3EN
//     USART3EN OFFSET(15) NUMBITS(1) [],
//     /// UART4EN
//     UART4EN OFFSET(16) NUMBITS(1) [],
//     /// UART5EN
//     UART5EN OFFSET(17) NUMBITS(1) [],
//     /// UART7EN
//     UART7EN OFFSET(18) NUMBITS(1) [],
//     /// UART8EN
//     UART8EN OFFSET(19) NUMBITS(1) [],
//     /// I2C1EN
//     I2C1EN OFFSET(21) NUMBITS(1) [],
//     /// I2C2EN
//     I2C2EN OFFSET(22) NUMBITS(1) [],
//     /// I2C3EN
//     I2C3EN OFFSET(23) NUMBITS(1) [],
//     /// I2C5EN
//     I2C5EN OFFSET(24) NUMBITS(1) [],
//     /// SPDIFEN
//     SPDIFEN OFFSET(26) NUMBITS(1) [],
//     /// CECEN
//     CECEN OFFSET(27) NUMBITS(1) [],
//     /// DAC12EN
//     DAC12EN OFFSET(29) NUMBITS(1) [],
//     /// MDIOSEN
//     MDIOSEN OFFSET(31) NUMBITS(1) []
// ],
// MP_APB2ENSETR [
//     /// TIM1EN
//     TIM1EN OFFSET(0) NUMBITS(1) [],
//     /// TIM8EN
//     TIM8EN OFFSET(1) NUMBITS(1) [],
//     /// TIM15EN
//     TIM15EN OFFSET(2) NUMBITS(1) [],
//     /// TIM16EN
//     TIM16EN OFFSET(3) NUMBITS(1) [],
//     /// TIM17EN
//     TIM17EN OFFSET(4) NUMBITS(1) [],
//     /// SPI1EN
//     SPI1EN OFFSET(8) NUMBITS(1) [],
//     /// SPI4EN
//     SPI4EN OFFSET(9) NUMBITS(1) [],
//     /// SPI5EN
//     SPI5EN OFFSET(10) NUMBITS(1) [],
//     /// USART6EN
//     USART6EN OFFSET(13) NUMBITS(1) [],
//     /// SAI1EN
//     SAI1EN OFFSET(16) NUMBITS(1) [],
//     /// SAI2EN
//     SAI2EN OFFSET(17) NUMBITS(1) [],
//     /// SAI3EN
//     SAI3EN OFFSET(18) NUMBITS(1) [],
//     /// DFSDMEN
//     DFSDMEN OFFSET(20) NUMBITS(1) [],
//     /// ADFSDMEN
//     ADFSDMEN OFFSET(21) NUMBITS(1) [],
//     /// FDCANEN
//     FDCANEN OFFSET(24) NUMBITS(1) []
// ],
// MP_APB2ENCLRR [
//     /// TIM1EN
//     TIM1EN OFFSET(0) NUMBITS(1) [],
//     /// TIM8EN
//     TIM8EN OFFSET(1) NUMBITS(1) [],
//     /// TIM15EN
//     TIM15EN OFFSET(2) NUMBITS(1) [],
//     /// TIM16EN
//     TIM16EN OFFSET(3) NUMBITS(1) [],
//     /// TIM17EN
//     TIM17EN OFFSET(4) NUMBITS(1) [],
//     /// SPI1EN
//     SPI1EN OFFSET(8) NUMBITS(1) [],
//     /// SPI4EN
//     SPI4EN OFFSET(9) NUMBITS(1) [],
//     /// SPI5EN
//     SPI5EN OFFSET(10) NUMBITS(1) [],
//     /// USART6EN
//     USART6EN OFFSET(13) NUMBITS(1) [],
//     /// SAI1EN
//     SAI1EN OFFSET(16) NUMBITS(1) [],
//     /// SAI2EN
//     SAI2EN OFFSET(17) NUMBITS(1) [],
//     /// SAI3EN
//     SAI3EN OFFSET(18) NUMBITS(1) [],
//     /// DFSDMEN
//     DFSDMEN OFFSET(20) NUMBITS(1) [],
//     /// ADFSDMEN
//     ADFSDMEN OFFSET(21) NUMBITS(1) [],
//     /// FDCANEN
//     FDCANEN OFFSET(24) NUMBITS(1) []
// ],
// MP_APB3ENSETR [
//     /// LPTIM2EN
//     LPTIM2EN OFFSET(0) NUMBITS(1) [],
//     /// LPTIM3EN
//     LPTIM3EN OFFSET(1) NUMBITS(1) [],
//     /// LPTIM4EN
//     LPTIM4EN OFFSET(2) NUMBITS(1) [],
//     /// LPTIM5EN
//     LPTIM5EN OFFSET(3) NUMBITS(1) [],
//     /// SAI4EN
//     SAI4EN OFFSET(8) NUMBITS(1) [],
//     /// SYSCFGEN
//     SYSCFGEN OFFSET(11) NUMBITS(1) [],
//     /// VREFEN
//     VREFEN OFFSET(13) NUMBITS(1) [],
//     /// DTSEN
//     DTSEN OFFSET(16) NUMBITS(1) [],
//     /// HDPEN
//     HDPEN OFFSET(20) NUMBITS(1) []
// ],
// MP_APB3ENCLRR [
//     /// LPTIM2EN
//     LPTIM2EN OFFSET(0) NUMBITS(1) [],
//     /// LPTIM3EN
//     LPTIM3EN OFFSET(1) NUMBITS(1) [],
//     /// LPTIM4EN
//     LPTIM4EN OFFSET(2) NUMBITS(1) [],
//     /// LPTIM5EN
//     LPTIM5EN OFFSET(3) NUMBITS(1) [],
//     /// SAI4EN
//     SAI4EN OFFSET(8) NUMBITS(1) [],
//     /// SYSCFGEN
//     SYSCFGEN OFFSET(11) NUMBITS(1) [],
//     /// VREFEN
//     VREFEN OFFSET(13) NUMBITS(1) [],
//     /// DTSEN
//     DTSEN OFFSET(16) NUMBITS(1) [],
//     /// HDPEN
//     HDPEN OFFSET(20) NUMBITS(1) []
// ],
// MP_AHB2ENSETR [
//     /// DMA1EN
//     DMA1EN OFFSET(0) NUMBITS(1) [],
//     /// DMA2EN
//     DMA2EN OFFSET(1) NUMBITS(1) [],
//     /// DMAMUXEN
//     DMAMUXEN OFFSET(2) NUMBITS(1) [],
//     /// ADC12EN
//     ADC12EN OFFSET(5) NUMBITS(1) [],
//     /// USBOEN
//     USBOEN OFFSET(8) NUMBITS(1) [],
//     /// SDMMC3EN
//     SDMMC3EN OFFSET(16) NUMBITS(1) []
// ],
// MP_AHB2ENCLRR [
//     /// DMA1EN
//     DMA1EN OFFSET(0) NUMBITS(1) [],
//     /// DMA2EN
//     DMA2EN OFFSET(1) NUMBITS(1) [],
//     /// DMAMUXEN
//     DMAMUXEN OFFSET(2) NUMBITS(1) [],
//     /// ADC12EN
//     ADC12EN OFFSET(5) NUMBITS(1) [],
//     /// USBOEN
//     USBOEN OFFSET(8) NUMBITS(1) [],
//     /// SDMMC3EN
//     SDMMC3EN OFFSET(16) NUMBITS(1) []
// ],
// MP_AHB3ENSETR [
//     /// DCMIEN
//     DCMIEN OFFSET(0) NUMBITS(1) [],
//     /// CRYP2EN
//     CRYP2EN OFFSET(4) NUMBITS(1) [],
//     /// HASH2EN
//     HASH2EN OFFSET(5) NUMBITS(1) [],
//     /// RNG2EN
//     RNG2EN OFFSET(6) NUMBITS(1) [],
//     /// CRC2EN
//     CRC2EN OFFSET(7) NUMBITS(1) [],
//     /// HSEMEN
//     HSEMEN OFFSET(11) NUMBITS(1) [],
//     /// IPCCEN
//     IPCCEN OFFSET(12) NUMBITS(1) []
// ],
// MP_AHB3ENCLRR [
//     /// DCMIEN
//     DCMIEN OFFSET(0) NUMBITS(1) [],
//     /// CRYP2EN
//     CRYP2EN OFFSET(4) NUMBITS(1) [],
//     /// HASH2EN
//     HASH2EN OFFSET(5) NUMBITS(1) [],
//     /// RNG2EN
//     RNG2EN OFFSET(6) NUMBITS(1) [],
//     /// CRC2EN
//     CRC2EN OFFSET(7) NUMBITS(1) [],
//     /// HSEMEN
//     HSEMEN OFFSET(11) NUMBITS(1) [],
//     /// IPCCEN
//     IPCCEN OFFSET(12) NUMBITS(1) []
// ],
// MP_AHB4ENSETR [
//     /// GPIOAEN
//     GPIOAEN OFFSET(0) NUMBITS(1) [],
//     /// GPIOBEN
//     GPIOBEN OFFSET(1) NUMBITS(1) [],
//     /// GPIOCEN
//     GPIOCEN OFFSET(2) NUMBITS(1) [],
//     /// GPIODEN
//     GPIODEN OFFSET(3) NUMBITS(1) [],
//     /// GPIOEEN
//     GPIOEEN OFFSET(4) NUMBITS(1) [],
//     /// GPIOFEN
//     GPIOFEN OFFSET(5) NUMBITS(1) [],
//     /// GPIOGEN
//     GPIOGEN OFFSET(6) NUMBITS(1) [],
//     /// GPIOHEN
//     GPIOHEN OFFSET(7) NUMBITS(1) [],
//     /// GPIOIEN
//     GPIOIEN OFFSET(8) NUMBITS(1) [],
//     /// GPIOJEN
//     GPIOJEN OFFSET(9) NUMBITS(1) [],
//     /// GPIOKEN
//     GPIOKEN OFFSET(10) NUMBITS(1) []
// ],
// MP_AHB4ENCLRR [
//     /// GPIOAEN
//     GPIOAEN OFFSET(0) NUMBITS(1) [],
//     /// GPIOBEN
//     GPIOBEN OFFSET(1) NUMBITS(1) [],
//     /// GPIOCEN
//     GPIOCEN OFFSET(2) NUMBITS(1) [],
//     /// GPIODEN
//     GPIODEN OFFSET(3) NUMBITS(1) [],
//     /// GPIOEEN
//     GPIOEEN OFFSET(4) NUMBITS(1) [],
//     /// GPIOFEN
//     GPIOFEN OFFSET(5) NUMBITS(1) [],
//     /// GPIOGEN
//     GPIOGEN OFFSET(6) NUMBITS(1) [],
//     /// GPIOHEN
//     GPIOHEN OFFSET(7) NUMBITS(1) [],
//     /// GPIOIEN
//     GPIOIEN OFFSET(8) NUMBITS(1) [],
//     /// GPIOJEN
//     GPIOJEN OFFSET(9) NUMBITS(1) [],
//     /// GPIOKEN
//     GPIOKEN OFFSET(10) NUMBITS(1) []
// ],
// MP_MLAHBENSETR [
//     /// RETRAMEN
//     RETRAMEN OFFSET(4) NUMBITS(1) []
// ],
// MP_MLAHBENCLRR [
//     /// RETRAMEN
//     RETRAMEN OFFSET(4) NUMBITS(1) []
// ],
// MC_APB1ENSETR [
//     /// TIM2EN
//     TIM2EN OFFSET(0) NUMBITS(1) [],
//     /// TIM3EN
//     TIM3EN OFFSET(1) NUMBITS(1) [],
//     /// TIM4EN
//     TIM4EN OFFSET(2) NUMBITS(1) [],
//     /// TIM5EN
//     TIM5EN OFFSET(3) NUMBITS(1) [],
//     /// TIM6EN
//     TIM6EN OFFSET(4) NUMBITS(1) [],
//     /// TIM7EN
//     TIM7EN OFFSET(5) NUMBITS(1) [],
//     /// TIM12EN
//     TIM12EN OFFSET(6) NUMBITS(1) [],
//     /// TIM13EN
//     TIM13EN OFFSET(7) NUMBITS(1) [],
//     /// TIM14EN
//     TIM14EN OFFSET(8) NUMBITS(1) [],
//     /// LPTIM1EN
//     LPTIM1EN OFFSET(9) NUMBITS(1) [],
//     /// SPI2EN
//     SPI2EN OFFSET(11) NUMBITS(1) [],
//     /// SPI3EN
//     SPI3EN OFFSET(12) NUMBITS(1) [],
//     /// USART2EN
//     USART2EN OFFSET(14) NUMBITS(1) [],
//     /// USART3EN
//     USART3EN OFFSET(15) NUMBITS(1) [],
//     /// UART4EN
//     UART4EN OFFSET(16) NUMBITS(1) [],
//     /// UART5EN
//     UART5EN OFFSET(17) NUMBITS(1) [],
//     /// UART7EN
//     UART7EN OFFSET(18) NUMBITS(1) [],
//     /// UART8EN
//     UART8EN OFFSET(19) NUMBITS(1) [],
//     /// I2C1EN
//     I2C1EN OFFSET(21) NUMBITS(1) [],
//     /// I2C2EN
//     I2C2EN OFFSET(22) NUMBITS(1) [],
//     /// I2C3EN
//     I2C3EN OFFSET(23) NUMBITS(1) [],
//     /// I2C5EN
//     I2C5EN OFFSET(24) NUMBITS(1) [],
//     /// SPDIFEN
//     SPDIFEN OFFSET(26) NUMBITS(1) [],
//     /// CECEN
//     CECEN OFFSET(27) NUMBITS(1) [],
//     /// WWDG1EN
//     WWDG1EN OFFSET(28) NUMBITS(1) [],
//     /// DAC12EN
//     DAC12EN OFFSET(29) NUMBITS(1) [],
//     /// MDIOSEN
//     MDIOSEN OFFSET(31) NUMBITS(1) []
// ],
// MC_APB1ENCLRR [
//     /// TIM2EN
//     TIM2EN OFFSET(0) NUMBITS(1) [],
//     /// TIM3EN
//     TIM3EN OFFSET(1) NUMBITS(1) [],
//     /// TIM4EN
//     TIM4EN OFFSET(2) NUMBITS(1) [],
//     /// TIM5EN
//     TIM5EN OFFSET(3) NUMBITS(1) [],
//     /// TIM6EN
//     TIM6EN OFFSET(4) NUMBITS(1) [],
//     /// TIM7EN
//     TIM7EN OFFSET(5) NUMBITS(1) [],
//     /// TIM12EN
//     TIM12EN OFFSET(6) NUMBITS(1) [],
//     /// TIM13EN
//     TIM13EN OFFSET(7) NUMBITS(1) [],
//     /// TIM14EN
//     TIM14EN OFFSET(8) NUMBITS(1) [],
//     /// LPTIM1EN
//     LPTIM1EN OFFSET(9) NUMBITS(1) [],
//     /// SPI2EN
//     SPI2EN OFFSET(11) NUMBITS(1) [],
//     /// SPI3EN
//     SPI3EN OFFSET(12) NUMBITS(1) [],
//     /// USART2EN
//     USART2EN OFFSET(14) NUMBITS(1) [],
//     /// USART3EN
//     USART3EN OFFSET(15) NUMBITS(1) [],
//     /// UART4EN
//     UART4EN OFFSET(16) NUMBITS(1) [],
//     /// UART5EN
//     UART5EN OFFSET(17) NUMBITS(1) [],
//     /// UART7EN
//     UART7EN OFFSET(18) NUMBITS(1) [],
//     /// UART8EN
//     UART8EN OFFSET(19) NUMBITS(1) [],
//     /// I2C1EN
//     I2C1EN OFFSET(21) NUMBITS(1) [],
//     /// I2C2EN
//     I2C2EN OFFSET(22) NUMBITS(1) [],
//     /// I2C3EN
//     I2C3EN OFFSET(23) NUMBITS(1) [],
//     /// I2C5EN
//     I2C5EN OFFSET(24) NUMBITS(1) [],
//     /// SPDIFEN
//     SPDIFEN OFFSET(26) NUMBITS(1) [],
//     /// CECEN
//     CECEN OFFSET(27) NUMBITS(1) [],
//     /// DAC12EN
//     DAC12EN OFFSET(29) NUMBITS(1) [],
//     /// MDIOSEN
//     MDIOSEN OFFSET(31) NUMBITS(1) []
// ],
// MC_APB2ENSETR [
//     /// TIM1EN
//     TIM1EN OFFSET(0) NUMBITS(1) [],
//     /// TIM8EN
//     TIM8EN OFFSET(1) NUMBITS(1) [],
//     /// TIM15EN
//     TIM15EN OFFSET(2) NUMBITS(1) [],
//     /// TIM16EN
//     TIM16EN OFFSET(3) NUMBITS(1) [],
//     /// TIM17EN
//     TIM17EN OFFSET(4) NUMBITS(1) [],
//     /// SPI1EN
//     SPI1EN OFFSET(8) NUMBITS(1) [],
//     /// SPI4EN
//     SPI4EN OFFSET(9) NUMBITS(1) [],
//     /// SPI5EN
//     SPI5EN OFFSET(10) NUMBITS(1) [],
//     /// USART6EN
//     USART6EN OFFSET(13) NUMBITS(1) [],
//     /// SAI1EN
//     SAI1EN OFFSET(16) NUMBITS(1) [],
//     /// SAI2EN
//     SAI2EN OFFSET(17) NUMBITS(1) [],
//     /// SAI3EN
//     SAI3EN OFFSET(18) NUMBITS(1) [],
//     /// DFSDMEN
//     DFSDMEN OFFSET(20) NUMBITS(1) [],
//     /// ADFSDMEN
//     ADFSDMEN OFFSET(21) NUMBITS(1) [],
//     /// FDCANEN
//     FDCANEN OFFSET(24) NUMBITS(1) []
// ],
// MC_APB2ENCLRR [
//     /// TIM1EN
//     TIM1EN OFFSET(0) NUMBITS(1) [],
//     /// TIM8EN
//     TIM8EN OFFSET(1) NUMBITS(1) [],
//     /// TIM15EN
//     TIM15EN OFFSET(2) NUMBITS(1) [],
//     /// TIM16EN
//     TIM16EN OFFSET(3) NUMBITS(1) [],
//     /// TIM17EN
//     TIM17EN OFFSET(4) NUMBITS(1) [],
//     /// SPI1EN
//     SPI1EN OFFSET(8) NUMBITS(1) [],
//     /// SPI4EN
//     SPI4EN OFFSET(9) NUMBITS(1) [],
//     /// SPI5EN
//     SPI5EN OFFSET(10) NUMBITS(1) [],
//     /// USART6EN
//     USART6EN OFFSET(13) NUMBITS(1) [],
//     /// SAI1EN
//     SAI1EN OFFSET(16) NUMBITS(1) [],
//     /// SAI2EN
//     SAI2EN OFFSET(17) NUMBITS(1) [],
//     /// SAI3EN
//     SAI3EN OFFSET(18) NUMBITS(1) [],
//     /// DFSDMEN
//     DFSDMEN OFFSET(20) NUMBITS(1) [],
//     /// ADFSDMEN
//     ADFSDMEN OFFSET(21) NUMBITS(1) [],
//     /// FDCANEN
//     FDCANEN OFFSET(24) NUMBITS(1) []
// ],
// MC_APB3ENSETR [
//     /// LPTIM2EN
//     LPTIM2EN OFFSET(0) NUMBITS(1) [],
//     /// LPTIM3EN
//     LPTIM3EN OFFSET(1) NUMBITS(1) [],
//     /// LPTIM4EN
//     LPTIM4EN OFFSET(2) NUMBITS(1) [],
//     /// LPTIM5EN
//     LPTIM5EN OFFSET(3) NUMBITS(1) [],
//     /// SAI4EN
//     SAI4EN OFFSET(8) NUMBITS(1) [],
//     /// SYSCFGEN
//     SYSCFGEN OFFSET(11) NUMBITS(1) [],
//     /// VREFEN
//     VREFEN OFFSET(13) NUMBITS(1) [],
//     /// DTSEN
//     DTSEN OFFSET(16) NUMBITS(1) [],
//     /// HDPEN
//     HDPEN OFFSET(20) NUMBITS(1) []
// ],
// MC_APB3ENCLRR [
//     /// LPTIM2EN
//     LPTIM2EN OFFSET(0) NUMBITS(1) [],
//     /// LPTIM3EN
//     LPTIM3EN OFFSET(1) NUMBITS(1) [],
//     /// LPTIM4EN
//     LPTIM4EN OFFSET(2) NUMBITS(1) [],
//     /// LPTIM5EN
//     LPTIM5EN OFFSET(3) NUMBITS(1) [],
//     /// SAI4EN
//     SAI4EN OFFSET(8) NUMBITS(1) [],
//     /// SYSCFGEN
//     SYSCFGEN OFFSET(11) NUMBITS(1) [],
//     /// VREFEN
//     VREFEN OFFSET(13) NUMBITS(1) [],
//     /// DTSEN
//     DTSEN OFFSET(16) NUMBITS(1) [],
//     /// HDPEN
//     HDPEN OFFSET(20) NUMBITS(1) []
// ],
// MC_AHB2ENSETR [
//     /// DMA1EN
//     DMA1EN OFFSET(0) NUMBITS(1) [],
//     /// DMA2EN
//     DMA2EN OFFSET(1) NUMBITS(1) [],
//     /// DMAMUXEN
//     DMAMUXEN OFFSET(2) NUMBITS(1) [],
//     /// ADC12EN
//     ADC12EN OFFSET(5) NUMBITS(1) [],
//     /// USBOEN
//     USBOEN OFFSET(8) NUMBITS(1) [],
//     /// SDMMC3EN
//     SDMMC3EN OFFSET(16) NUMBITS(1) []
// ],
// MC_AHB2ENCLRR [
//     /// DMA1EN
//     DMA1EN OFFSET(0) NUMBITS(1) [],
//     /// DMA2EN
//     DMA2EN OFFSET(1) NUMBITS(1) [],
//     /// DMAMUXEN
//     DMAMUXEN OFFSET(2) NUMBITS(1) [],
//     /// ADC12EN
//     ADC12EN OFFSET(5) NUMBITS(1) [],
//     /// USBOEN
//     USBOEN OFFSET(8) NUMBITS(1) [],
//     /// SDMMC3EN
//     SDMMC3EN OFFSET(16) NUMBITS(1) []
// ],
// MC_AHB3ENSETR [
//     /// DCMIEN
//     DCMIEN OFFSET(0) NUMBITS(1) [],
//     /// CRYP2EN
//     CRYP2EN OFFSET(4) NUMBITS(1) [],
//     /// HASH2EN
//     HASH2EN OFFSET(5) NUMBITS(1) [],
//     /// RNG2EN
//     RNG2EN OFFSET(6) NUMBITS(1) [],
//     /// CRC2EN
//     CRC2EN OFFSET(7) NUMBITS(1) [],
//     /// HSEMEN
//     HSEMEN OFFSET(11) NUMBITS(1) [],
//     /// IPCCEN
//     IPCCEN OFFSET(12) NUMBITS(1) []
// ],
// MC_AHB3ENCLRR [
//     /// DCMIEN
//     DCMIEN OFFSET(0) NUMBITS(1) [],
//     /// CRYP2EN
//     CRYP2EN OFFSET(4) NUMBITS(1) [],
//     /// HASH2EN
//     HASH2EN OFFSET(5) NUMBITS(1) [],
//     /// RNG2EN
//     RNG2EN OFFSET(6) NUMBITS(1) [],
//     /// CRC2EN
//     CRC2EN OFFSET(7) NUMBITS(1) [],
//     /// HSEMEN
//     HSEMEN OFFSET(11) NUMBITS(1) [],
//     /// IPCCEN
//     IPCCEN OFFSET(12) NUMBITS(1) []
// ],
// MC_AHB4ENSETR [
//     /// GPIOAEN
//     GPIOAEN OFFSET(0) NUMBITS(1) [],
//     /// GPIOBEN
//     GPIOBEN OFFSET(1) NUMBITS(1) [],
//     /// GPIOCEN
//     GPIOCEN OFFSET(2) NUMBITS(1) [],
//     /// GPIODEN
//     GPIODEN OFFSET(3) NUMBITS(1) [],
//     /// GPIOEEN
//     GPIOEEN OFFSET(4) NUMBITS(1) [],
//     /// GPIOFEN
//     GPIOFEN OFFSET(5) NUMBITS(1) [],
//     /// GPIOGEN
//     GPIOGEN OFFSET(6) NUMBITS(1) [],
//     /// GPIOHEN
//     GPIOHEN OFFSET(7) NUMBITS(1) [],
//     /// GPIOIEN
//     GPIOIEN OFFSET(8) NUMBITS(1) [],
//     /// GPIOJEN
//     GPIOJEN OFFSET(9) NUMBITS(1) [],
//     /// GPIOKEN
//     GPIOKEN OFFSET(10) NUMBITS(1) []
// ],
// MC_AHB4ENCLRR [
//     /// GPIOAEN
//     GPIOAEN OFFSET(0) NUMBITS(1) [],
//     /// GPIOBEN
//     GPIOBEN OFFSET(1) NUMBITS(1) [],
//     /// GPIOCEN
//     GPIOCEN OFFSET(2) NUMBITS(1) [],
//     /// GPIODEN
//     GPIODEN OFFSET(3) NUMBITS(1) [],
//     /// GPIOEEN
//     GPIOEEN OFFSET(4) NUMBITS(1) [],
//     /// GPIOFEN
//     GPIOFEN OFFSET(5) NUMBITS(1) [],
//     /// GPIOGEN
//     GPIOGEN OFFSET(6) NUMBITS(1) [],
//     /// GPIOHEN
//     GPIOHEN OFFSET(7) NUMBITS(1) [],
//     /// GPIOIEN
//     GPIOIEN OFFSET(8) NUMBITS(1) [],
//     /// GPIOJEN
//     GPIOJEN OFFSET(9) NUMBITS(1) [],
//     /// GPIOKEN
//     GPIOKEN OFFSET(10) NUMBITS(1) []
// ],
// MC_AXIMENSETR [
//     /// SYSRAMEN
//     SYSRAMEN OFFSET(0) NUMBITS(1) []
// ],
// MC_AXIMENCLRR [
//     /// SYSRAMEN
//     SYSRAMEN OFFSET(0) NUMBITS(1) []
// ],
// MC_MLAHBENSETR [
//     /// RETRAMEN
//     RETRAMEN OFFSET(4) NUMBITS(1) []
// ],
// MC_MLAHBENCLRR [
//     /// RETRAMEN
//     RETRAMEN OFFSET(4) NUMBITS(1) []
// ],
// MP_APB1LPENSETR [
//     /// TIM2LPEN
//     TIM2LPEN OFFSET(0) NUMBITS(1) [],
//     /// TIM3LPEN
//     TIM3LPEN OFFSET(1) NUMBITS(1) [],
//     /// TIM4LPEN
//     TIM4LPEN OFFSET(2) NUMBITS(1) [],
//     /// TIM5LPEN
//     TIM5LPEN OFFSET(3) NUMBITS(1) [],
//     /// TIM6LPEN
//     TIM6LPEN OFFSET(4) NUMBITS(1) [],
//     /// TIM7LPEN
//     TIM7LPEN OFFSET(5) NUMBITS(1) [],
//     /// TIM12LPEN
//     TIM12LPEN OFFSET(6) NUMBITS(1) [],
//     /// TIM13LPEN
//     TIM13LPEN OFFSET(7) NUMBITS(1) [],
//     /// TIM14LPEN
//     TIM14LPEN OFFSET(8) NUMBITS(1) [],
//     /// LPTIM1LPEN
//     LPTIM1LPEN OFFSET(9) NUMBITS(1) [],
//     /// SPI2LPEN
//     SPI2LPEN OFFSET(11) NUMBITS(1) [],
//     /// SPI3LPEN
//     SPI3LPEN OFFSET(12) NUMBITS(1) [],
//     /// USART2LPEN
//     USART2LPEN OFFSET(14) NUMBITS(1) [],
//     /// USART3LPEN
//     USART3LPEN OFFSET(15) NUMBITS(1) [],
//     /// UART4LPEN
//     UART4LPEN OFFSET(16) NUMBITS(1) [],
//     /// UART5LPEN
//     UART5LPEN OFFSET(17) NUMBITS(1) [],
//     /// UART7LPEN
//     UART7LPEN OFFSET(18) NUMBITS(1) [],
//     /// UART8LPEN
//     UART8LPEN OFFSET(19) NUMBITS(1) [],
//     /// I2C1LPEN
//     I2C1LPEN OFFSET(21) NUMBITS(1) [],
//     /// I2C2LPEN
//     I2C2LPEN OFFSET(22) NUMBITS(1) [],
//     /// I2C3LPEN
//     I2C3LPEN OFFSET(23) NUMBITS(1) [],
//     /// I2C5LPEN
//     I2C5LPEN OFFSET(24) NUMBITS(1) [],
//     /// SPDIFLPEN
//     SPDIFLPEN OFFSET(26) NUMBITS(1) [],
//     /// CECLPEN
//     CECLPEN OFFSET(27) NUMBITS(1) [],
//     /// DAC12LPEN
//     DAC12LPEN OFFSET(29) NUMBITS(1) [],
//     /// MDIOSLPEN
//     MDIOSLPEN OFFSET(31) NUMBITS(1) []
// ],
// MP_APB1LPENCLRR [
//     /// TIM2LPEN
//     TIM2LPEN OFFSET(0) NUMBITS(1) [],
//     /// TIM3LPEN
//     TIM3LPEN OFFSET(1) NUMBITS(1) [],
//     /// TIM4LPEN
//     TIM4LPEN OFFSET(2) NUMBITS(1) [],
//     /// TIM5LPEN
//     TIM5LPEN OFFSET(3) NUMBITS(1) [],
//     /// TIM6LPEN
//     TIM6LPEN OFFSET(4) NUMBITS(1) [],
//     /// TIM7LPEN
//     TIM7LPEN OFFSET(5) NUMBITS(1) [],
//     /// TIM12LPEN
//     TIM12LPEN OFFSET(6) NUMBITS(1) [],
//     /// TIM13LPEN
//     TIM13LPEN OFFSET(7) NUMBITS(1) [],
//     /// TIM14LPEN
//     TIM14LPEN OFFSET(8) NUMBITS(1) [],
//     /// LPTIM1LPEN
//     LPTIM1LPEN OFFSET(9) NUMBITS(1) [],
//     /// SPI2LPEN
//     SPI2LPEN OFFSET(11) NUMBITS(1) [],
//     /// SPI3LPEN
//     SPI3LPEN OFFSET(12) NUMBITS(1) [],
//     /// USART2LPEN
//     USART2LPEN OFFSET(14) NUMBITS(1) [],
//     /// USART3LPEN
//     USART3LPEN OFFSET(15) NUMBITS(1) [],
//     /// UART4LPEN
//     UART4LPEN OFFSET(16) NUMBITS(1) [],
//     /// UART5LPEN
//     UART5LPEN OFFSET(17) NUMBITS(1) [],
//     /// UART7LPEN
//     UART7LPEN OFFSET(18) NUMBITS(1) [],
//     /// UART8LPEN
//     UART8LPEN OFFSET(19) NUMBITS(1) [],
//     /// I2C1LPEN
//     I2C1LPEN OFFSET(21) NUMBITS(1) [],
//     /// I2C2LPEN
//     I2C2LPEN OFFSET(22) NUMBITS(1) [],
//     /// I2C3LPEN
//     I2C3LPEN OFFSET(23) NUMBITS(1) [],
//     /// I2C5LPEN
//     I2C5LPEN OFFSET(24) NUMBITS(1) [],
//     /// SPDIFLPEN
//     SPDIFLPEN OFFSET(26) NUMBITS(1) [],
//     /// CECLPEN
//     CECLPEN OFFSET(27) NUMBITS(1) [],
//     /// DAC12LPEN
//     DAC12LPEN OFFSET(29) NUMBITS(1) [],
//     /// MDIOSLPEN
//     MDIOSLPEN OFFSET(31) NUMBITS(1) []
// ],
// MP_APB2LPENSETR [
//     /// TIM1LPEN
//     TIM1LPEN OFFSET(0) NUMBITS(1) [],
//     /// TIM8LPEN
//     TIM8LPEN OFFSET(1) NUMBITS(1) [],
//     /// TIM15LPEN
//     TIM15LPEN OFFSET(2) NUMBITS(1) [],
//     /// TIM16LPEN
//     TIM16LPEN OFFSET(3) NUMBITS(1) [],
//     /// TIM17LPEN
//     TIM17LPEN OFFSET(4) NUMBITS(1) [],
//     /// SPI1LPEN
//     SPI1LPEN OFFSET(8) NUMBITS(1) [],
//     /// SPI4LPEN
//     SPI4LPEN OFFSET(9) NUMBITS(1) [],
//     /// SPI5LPEN
//     SPI5LPEN OFFSET(10) NUMBITS(1) [],
//     /// USART6LPEN
//     USART6LPEN OFFSET(13) NUMBITS(1) [],
//     /// SAI1LPEN
//     SAI1LPEN OFFSET(16) NUMBITS(1) [],
//     /// SAI2LPEN
//     SAI2LPEN OFFSET(17) NUMBITS(1) [],
//     /// SAI3LPEN
//     SAI3LPEN OFFSET(18) NUMBITS(1) [],
//     /// DFSDMLPEN
//     DFSDMLPEN OFFSET(20) NUMBITS(1) [],
//     /// ADFSDMLPEN
//     ADFSDMLPEN OFFSET(21) NUMBITS(1) [],
//     /// FDCANLPEN
//     FDCANLPEN OFFSET(24) NUMBITS(1) []
// ],
// MP_APB2LPENCLRR [
//     /// TIM1LPEN
//     TIM1LPEN OFFSET(0) NUMBITS(1) [],
//     /// TIM8LPEN
//     TIM8LPEN OFFSET(1) NUMBITS(1) [],
//     /// TIM15LPEN
//     TIM15LPEN OFFSET(2) NUMBITS(1) [],
//     /// TIM16LPEN
//     TIM16LPEN OFFSET(3) NUMBITS(1) [],
//     /// TIM17LPEN
//     TIM17LPEN OFFSET(4) NUMBITS(1) [],
//     /// SPI1LPEN
//     SPI1LPEN OFFSET(8) NUMBITS(1) [],
//     /// SPI4LPEN
//     SPI4LPEN OFFSET(9) NUMBITS(1) [],
//     /// SPI5LPEN
//     SPI5LPEN OFFSET(10) NUMBITS(1) [],
//     /// USART6LPEN
//     USART6LPEN OFFSET(13) NUMBITS(1) [],
//     /// SAI1LPEN
//     SAI1LPEN OFFSET(16) NUMBITS(1) [],
//     /// SAI2LPEN
//     SAI2LPEN OFFSET(17) NUMBITS(1) [],
//     /// SAI3LPEN
//     SAI3LPEN OFFSET(18) NUMBITS(1) [],
//     /// DFSDMLPEN
//     DFSDMLPEN OFFSET(20) NUMBITS(1) [],
//     /// ADFSDMLPEN
//     ADFSDMLPEN OFFSET(21) NUMBITS(1) [],
//     /// FDCANLPEN
//     FDCANLPEN OFFSET(24) NUMBITS(1) []
// ],
// MP_APB3LPENSETR [
//     /// LPTIM2LPEN
//     LPTIM2LPEN OFFSET(0) NUMBITS(1) [],
//     /// LPTIM3LPEN
//     LPTIM3LPEN OFFSET(1) NUMBITS(1) [],
//     /// LPTIM4LPEN
//     LPTIM4LPEN OFFSET(2) NUMBITS(1) [],
//     /// LPTIM5LPEN
//     LPTIM5LPEN OFFSET(3) NUMBITS(1) [],
//     /// SAI4LPEN
//     SAI4LPEN OFFSET(8) NUMBITS(1) [],
//     /// SYSCFGLPEN
//     SYSCFGLPEN OFFSET(11) NUMBITS(1) [],
//     /// VREFLPEN
//     VREFLPEN OFFSET(13) NUMBITS(1) [],
//     /// DTSLPEN
//     DTSLPEN OFFSET(16) NUMBITS(1) []
// ],
// MP_APB3LPENCLRR [
//     /// LPTIM2LPEN
//     LPTIM2LPEN OFFSET(0) NUMBITS(1) [],
//     /// LPTIM3LPEN
//     LPTIM3LPEN OFFSET(1) NUMBITS(1) [],
//     /// LPTIM4LPEN
//     LPTIM4LPEN OFFSET(2) NUMBITS(1) [],
//     /// LPTIM5LPEN
//     LPTIM5LPEN OFFSET(3) NUMBITS(1) [],
//     /// SAI4LPEN
//     SAI4LPEN OFFSET(8) NUMBITS(1) [],
//     /// SYSCFGLPEN
//     SYSCFGLPEN OFFSET(11) NUMBITS(1) [],
//     /// VREFLPEN
//     VREFLPEN OFFSET(13) NUMBITS(1) [],
//     /// DTSLPEN
//     DTSLPEN OFFSET(16) NUMBITS(1) []
// ],
// MP_AHB2LPENSETR [
//     /// DMA1LPEN
//     DMA1LPEN OFFSET(0) NUMBITS(1) [],
//     /// DMA2LPEN
//     DMA2LPEN OFFSET(1) NUMBITS(1) [],
//     /// DMAMUXLPEN
//     DMAMUXLPEN OFFSET(2) NUMBITS(1) [],
//     /// ADC12LPEN
//     ADC12LPEN OFFSET(5) NUMBITS(1) [],
//     /// USBOLPEN
//     USBOLPEN OFFSET(8) NUMBITS(1) [],
//     /// SDMMC3LPEN
//     SDMMC3LPEN OFFSET(16) NUMBITS(1) []
// ],
// MP_AHB2LPENCLRR [
//     /// DMA1LPEN
//     DMA1LPEN OFFSET(0) NUMBITS(1) [],
//     /// DMA2LPEN
//     DMA2LPEN OFFSET(1) NUMBITS(1) [],
//     /// DMAMUXLPEN
//     DMAMUXLPEN OFFSET(2) NUMBITS(1) [],
//     /// ADC12LPEN
//     ADC12LPEN OFFSET(5) NUMBITS(1) [],
//     /// USBOLPEN
//     USBOLPEN OFFSET(8) NUMBITS(1) [],
//     /// SDMMC3LPEN
//     SDMMC3LPEN OFFSET(16) NUMBITS(1) []
// ],
// MP_AHB3LPENSETR [
//     /// DCMILPEN
//     DCMILPEN OFFSET(0) NUMBITS(1) [],
//     /// CRYP2LPEN
//     CRYP2LPEN OFFSET(4) NUMBITS(1) [],
//     /// HASH2LPEN
//     HASH2LPEN OFFSET(5) NUMBITS(1) [],
//     /// RNG2LPEN
//     RNG2LPEN OFFSET(6) NUMBITS(1) [],
//     /// CRC2LPEN
//     CRC2LPEN OFFSET(7) NUMBITS(1) [],
//     /// HSEMLPEN
//     HSEMLPEN OFFSET(11) NUMBITS(1) [],
//     /// IPCCLPEN
//     IPCCLPEN OFFSET(12) NUMBITS(1) []
// ],
// MP_AHB3LPENCLRR [
//     /// DCMILPEN
//     DCMILPEN OFFSET(0) NUMBITS(1) [],
//     /// CRYP2LPEN
//     CRYP2LPEN OFFSET(4) NUMBITS(1) [],
//     /// HASH2LPEN
//     HASH2LPEN OFFSET(5) NUMBITS(1) [],
//     /// RNG2LPEN
//     RNG2LPEN OFFSET(6) NUMBITS(1) [],
//     /// CRC2LPEN
//     CRC2LPEN OFFSET(7) NUMBITS(1) [],
//     /// HSEMLPEN
//     HSEMLPEN OFFSET(11) NUMBITS(1) [],
//     /// IPCCLPEN
//     IPCCLPEN OFFSET(12) NUMBITS(1) []
// ],
// MP_AHB4LPENSETR [
//     /// GPIOALPEN
//     GPIOALPEN OFFSET(0) NUMBITS(1) [],
//     /// GPIOBLPEN
//     GPIOBLPEN OFFSET(1) NUMBITS(1) [],
//     /// GPIOCLPEN
//     GPIOCLPEN OFFSET(2) NUMBITS(1) [],
//     /// GPIODLPEN
//     GPIODLPEN OFFSET(3) NUMBITS(1) [],
//     /// GPIOELPEN
//     GPIOELPEN OFFSET(4) NUMBITS(1) [],
//     /// GPIOFLPEN
//     GPIOFLPEN OFFSET(5) NUMBITS(1) [],
//     /// GPIOGLPEN
//     GPIOGLPEN OFFSET(6) NUMBITS(1) [],
//     /// GPIOHLPEN
//     GPIOHLPEN OFFSET(7) NUMBITS(1) [],
//     /// GPIOILPEN
//     GPIOILPEN OFFSET(8) NUMBITS(1) [],
//     /// GPIOJLPEN
//     GPIOJLPEN OFFSET(9) NUMBITS(1) [],
//     /// GPIOKLPEN
//     GPIOKLPEN OFFSET(10) NUMBITS(1) []
// ],
// MP_AHB4LPENCLRR [
//     /// GPIOALPEN
//     GPIOALPEN OFFSET(0) NUMBITS(1) [],
//     /// GPIOBLPEN
//     GPIOBLPEN OFFSET(1) NUMBITS(1) [],
//     /// GPIOCLPEN
//     GPIOCLPEN OFFSET(2) NUMBITS(1) [],
//     /// GPIODLPEN
//     GPIODLPEN OFFSET(3) NUMBITS(1) [],
//     /// GPIOELPEN
//     GPIOELPEN OFFSET(4) NUMBITS(1) [],
//     /// GPIOFLPEN
//     GPIOFLPEN OFFSET(5) NUMBITS(1) [],
//     /// GPIOGLPEN
//     GPIOGLPEN OFFSET(6) NUMBITS(1) [],
//     /// GPIOHLPEN
//     GPIOHLPEN OFFSET(7) NUMBITS(1) [],
//     /// GPIOILPEN
//     GPIOILPEN OFFSET(8) NUMBITS(1) [],
//     /// GPIOJLPEN
//     GPIOJLPEN OFFSET(9) NUMBITS(1) [],
//     /// GPIOKLPEN
//     GPIOKLPEN OFFSET(10) NUMBITS(1) []
// ],
// MP_AXIMLPENSETR [
//     /// SYSRAMLPEN
//     SYSRAMLPEN OFFSET(0) NUMBITS(1) []
// ],
// MP_AXIMLPENCLRR [
//     /// SYSRAMLPEN
//     SYSRAMLPEN OFFSET(0) NUMBITS(1) []
// ],
// MP_MLAHBLPENSETR [
//     /// SRAM1LPEN
//     SRAM1LPEN OFFSET(0) NUMBITS(1) [],
//     /// SRAM2LPEN
//     SRAM2LPEN OFFSET(1) NUMBITS(1) [],
//     /// SRAM34LPEN
//     SRAM34LPEN OFFSET(2) NUMBITS(1) [],
//     /// RETRAMLPEN
//     RETRAMLPEN OFFSET(4) NUMBITS(1) []
// ],
// MP_MLAHBLPENCLRR [
//     /// SRAM1LPEN
//     SRAM1LPEN OFFSET(0) NUMBITS(1) [],
//     /// SRAM2LPEN
//     SRAM2LPEN OFFSET(1) NUMBITS(1) [],
//     /// SRAM34LPEN
//     SRAM34LPEN OFFSET(2) NUMBITS(1) [],
//     /// RETRAMLPEN
//     RETRAMLPEN OFFSET(4) NUMBITS(1) []
// ],
// MC_APB1LPENSETR [
//     /// TIM2LPEN
//     TIM2LPEN OFFSET(0) NUMBITS(1) [],
//     /// TIM3LPEN
//     TIM3LPEN OFFSET(1) NUMBITS(1) [],
//     /// TIM4LPEN
//     TIM4LPEN OFFSET(2) NUMBITS(1) [],
//     /// TIM5LPEN
//     TIM5LPEN OFFSET(3) NUMBITS(1) [],
//     /// TIM6LPEN
//     TIM6LPEN OFFSET(4) NUMBITS(1) [],
//     /// TIM7LPEN
//     TIM7LPEN OFFSET(5) NUMBITS(1) [],
//     /// TIM12LPEN
//     TIM12LPEN OFFSET(6) NUMBITS(1) [],
//     /// TIM13LPEN
//     TIM13LPEN OFFSET(7) NUMBITS(1) [],
//     /// TIM14LPEN
//     TIM14LPEN OFFSET(8) NUMBITS(1) [],
//     /// LPTIM1LPEN
//     LPTIM1LPEN OFFSET(9) NUMBITS(1) [],
//     /// SPI2LPEN
//     SPI2LPEN OFFSET(11) NUMBITS(1) [],
//     /// SPI3LPEN
//     SPI3LPEN OFFSET(12) NUMBITS(1) [],
//     /// USART2LPEN
//     USART2LPEN OFFSET(14) NUMBITS(1) [],
//     /// USART3LPEN
//     USART3LPEN OFFSET(15) NUMBITS(1) [],
//     /// UART4LPEN
//     UART4LPEN OFFSET(16) NUMBITS(1) [],
//     /// UART5LPEN
//     UART5LPEN OFFSET(17) NUMBITS(1) [],
//     /// UART7LPEN
//     UART7LPEN OFFSET(18) NUMBITS(1) [],
//     /// UART8LPEN
//     UART8LPEN OFFSET(19) NUMBITS(1) [],
//     /// I2C1LPEN
//     I2C1LPEN OFFSET(21) NUMBITS(1) [],
//     /// I2C2LPEN
//     I2C2LPEN OFFSET(22) NUMBITS(1) [],
//     /// I2C3LPEN
//     I2C3LPEN OFFSET(23) NUMBITS(1) [],
//     /// I2C5LPEN
//     I2C5LPEN OFFSET(24) NUMBITS(1) [],
//     /// SPDIFLPEN
//     SPDIFLPEN OFFSET(26) NUMBITS(1) [],
//     /// CECLPEN
//     CECLPEN OFFSET(27) NUMBITS(1) [],
//     /// WWDG1LPEN
//     WWDG1LPEN OFFSET(28) NUMBITS(1) [],
//     /// DAC12LPEN
//     DAC12LPEN OFFSET(29) NUMBITS(1) [],
//     /// MDIOSLPEN
//     MDIOSLPEN OFFSET(31) NUMBITS(1) []
// ],
// MC_APB1LPENCLRR [
//     /// TIM2LPEN
//     TIM2LPEN OFFSET(0) NUMBITS(1) [],
//     /// TIM3LPEN
//     TIM3LPEN OFFSET(1) NUMBITS(1) [],
//     /// TIM4LPEN
//     TIM4LPEN OFFSET(2) NUMBITS(1) [],
//     /// TIM5LPEN
//     TIM5LPEN OFFSET(3) NUMBITS(1) [],
//     /// TIM6LPEN
//     TIM6LPEN OFFSET(4) NUMBITS(1) [],
//     /// TIM7LPEN
//     TIM7LPEN OFFSET(5) NUMBITS(1) [],
//     /// TIM12LPEN
//     TIM12LPEN OFFSET(6) NUMBITS(1) [],
//     /// TIM13LPEN
//     TIM13LPEN OFFSET(7) NUMBITS(1) [],
//     /// TIM14LPEN
//     TIM14LPEN OFFSET(8) NUMBITS(1) [],
//     /// LPTIM1LPEN
//     LPTIM1LPEN OFFSET(9) NUMBITS(1) [],
//     /// SPI2LPEN
//     SPI2LPEN OFFSET(11) NUMBITS(1) [],
//     /// SPI3LPEN
//     SPI3LPEN OFFSET(12) NUMBITS(1) [],
//     /// USART2LPEN
//     USART2LPEN OFFSET(14) NUMBITS(1) [],
//     /// USART3LPEN
//     USART3LPEN OFFSET(15) NUMBITS(1) [],
//     /// UART4LPEN
//     UART4LPEN OFFSET(16) NUMBITS(1) [],
//     /// UART5LPEN
//     UART5LPEN OFFSET(17) NUMBITS(1) [],
//     /// UART7LPEN
//     UART7LPEN OFFSET(18) NUMBITS(1) [],
//     /// UART8LPEN
//     UART8LPEN OFFSET(19) NUMBITS(1) [],
//     /// I2C1LPEN
//     I2C1LPEN OFFSET(21) NUMBITS(1) [],
//     /// I2C2LPEN
//     I2C2LPEN OFFSET(22) NUMBITS(1) [],
//     /// I2C3LPEN
//     I2C3LPEN OFFSET(23) NUMBITS(1) [],
//     /// I2C5LPEN
//     I2C5LPEN OFFSET(24) NUMBITS(1) [],
//     /// SPDIFLPEN
//     SPDIFLPEN OFFSET(26) NUMBITS(1) [],
//     /// CECLPEN
//     CECLPEN OFFSET(27) NUMBITS(1) [],
//     /// WWDG1LPEN
//     WWDG1LPEN OFFSET(28) NUMBITS(1) [],
//     /// DAC12LPEN
//     DAC12LPEN OFFSET(29) NUMBITS(1) [],
//     /// MDIOSLPEN
//     MDIOSLPEN OFFSET(31) NUMBITS(1) []
// ],
// MC_APB2LPENSETR [
//     /// TIM1LPEN
//     TIM1LPEN OFFSET(0) NUMBITS(1) [],
//     /// TIM8LPEN
//     TIM8LPEN OFFSET(1) NUMBITS(1) [],
//     /// TIM15LPEN
//     TIM15LPEN OFFSET(2) NUMBITS(1) [],
//     /// TIM16LPEN
//     TIM16LPEN OFFSET(3) NUMBITS(1) [],
//     /// TIM17LPEN
//     TIM17LPEN OFFSET(4) NUMBITS(1) [],
//     /// SPI1LPEN
//     SPI1LPEN OFFSET(8) NUMBITS(1) [],
//     /// SPI4LPEN
//     SPI4LPEN OFFSET(9) NUMBITS(1) [],
//     /// SPI5LPEN
//     SPI5LPEN OFFSET(10) NUMBITS(1) [],
//     /// USART6LPEN
//     USART6LPEN OFFSET(13) NUMBITS(1) [],
//     /// SAI1LPEN
//     SAI1LPEN OFFSET(16) NUMBITS(1) [],
//     /// SAI2LPEN
//     SAI2LPEN OFFSET(17) NUMBITS(1) [],
//     /// SAI3LPEN
//     SAI3LPEN OFFSET(18) NUMBITS(1) [],
//     /// DFSDMLPEN
//     DFSDMLPEN OFFSET(20) NUMBITS(1) [],
//     /// ADFSDMLPEN
//     ADFSDMLPEN OFFSET(21) NUMBITS(1) [],
//     /// FDCANLPEN
//     FDCANLPEN OFFSET(24) NUMBITS(1) []
// ],
// MC_APB2LPENCLRR [
//     /// TIM1LPEN
//     TIM1LPEN OFFSET(0) NUMBITS(1) [],
//     /// TIM8LPEN
//     TIM8LPEN OFFSET(1) NUMBITS(1) [],
//     /// TIM15LPEN
//     TIM15LPEN OFFSET(2) NUMBITS(1) [],
//     /// TIM16LPEN
//     TIM16LPEN OFFSET(3) NUMBITS(1) [],
//     /// TIM17LPEN
//     TIM17LPEN OFFSET(4) NUMBITS(1) [],
//     /// SPI1LPEN
//     SPI1LPEN OFFSET(8) NUMBITS(1) [],
//     /// SPI4LPEN
//     SPI4LPEN OFFSET(9) NUMBITS(1) [],
//     /// SPI5LPEN
//     SPI5LPEN OFFSET(10) NUMBITS(1) [],
//     /// USART6LPEN
//     USART6LPEN OFFSET(13) NUMBITS(1) [],
//     /// SAI1LPEN
//     SAI1LPEN OFFSET(16) NUMBITS(1) [],
//     /// SAI2LPEN
//     SAI2LPEN OFFSET(17) NUMBITS(1) [],
//     /// SAI3LPEN
//     SAI3LPEN OFFSET(18) NUMBITS(1) [],
//     /// DFSDMLPEN
//     DFSDMLPEN OFFSET(20) NUMBITS(1) [],
//     /// ADFSDMLPEN
//     ADFSDMLPEN OFFSET(21) NUMBITS(1) [],
//     /// FDCANLPEN
//     FDCANLPEN OFFSET(24) NUMBITS(1) []
// ],
// MC_APB3LPENSETR [
//     /// LPTIM2LPEN
//     LPTIM2LPEN OFFSET(0) NUMBITS(1) [],
//     /// LPTIM3LPEN
//     LPTIM3LPEN OFFSET(1) NUMBITS(1) [],
//     /// LPTIM4LPEN
//     LPTIM4LPEN OFFSET(2) NUMBITS(1) [],
//     /// LPTIM5LPEN
//     LPTIM5LPEN OFFSET(3) NUMBITS(1) [],
//     /// SAI4LPEN
//     SAI4LPEN OFFSET(8) NUMBITS(1) [],
//     /// SYSCFGLPEN
//     SYSCFGLPEN OFFSET(11) NUMBITS(1) [],
//     /// VREFLPEN
//     VREFLPEN OFFSET(13) NUMBITS(1) [],
//     /// DTSLPEN
//     DTSLPEN OFFSET(16) NUMBITS(1) []
// ],
// MC_APB3LPENCLRR [
//     /// LPTIM2LPEN
//     LPTIM2LPEN OFFSET(0) NUMBITS(1) [],
//     /// LPTIM3LPEN
//     LPTIM3LPEN OFFSET(1) NUMBITS(1) [],
//     /// LPTIM4LPEN
//     LPTIM4LPEN OFFSET(2) NUMBITS(1) [],
//     /// LPTIM5LPEN
//     LPTIM5LPEN OFFSET(3) NUMBITS(1) [],
//     /// SAI4LPEN
//     SAI4LPEN OFFSET(8) NUMBITS(1) [],
//     /// SYSCFGLPEN
//     SYSCFGLPEN OFFSET(11) NUMBITS(1) [],
//     /// VREFLPEN
//     VREFLPEN OFFSET(13) NUMBITS(1) [],
//     /// DTSLPEN
//     DTSLPEN OFFSET(16) NUMBITS(1) []
// ],
// MC_AHB2LPENSETR [
//     /// DMA1LPEN
//     DMA1LPEN OFFSET(0) NUMBITS(1) [],
//     /// DMA2LPEN
//     DMA2LPEN OFFSET(1) NUMBITS(1) [],
//     /// DMAMUXLPEN
//     DMAMUXLPEN OFFSET(2) NUMBITS(1) [],
//     /// ADC12LPEN
//     ADC12LPEN OFFSET(5) NUMBITS(1) [],
//     /// USBOLPEN
//     USBOLPEN OFFSET(8) NUMBITS(1) [],
//     /// SDMMC3LPEN
//     SDMMC3LPEN OFFSET(16) NUMBITS(1) []
// ],
// MC_AHB2LPENCLRR [
//     /// DMA1LPEN
//     DMA1LPEN OFFSET(0) NUMBITS(1) [],
//     /// DMA2LPEN
//     DMA2LPEN OFFSET(1) NUMBITS(1) [],
//     /// DMAMUXLPEN
//     DMAMUXLPEN OFFSET(2) NUMBITS(1) [],
//     /// ADC12LPEN
//     ADC12LPEN OFFSET(5) NUMBITS(1) [],
//     /// USBOLPEN
//     USBOLPEN OFFSET(8) NUMBITS(1) [],
//     /// SDMMC3LPEN
//     SDMMC3LPEN OFFSET(16) NUMBITS(1) []
// ],
// MC_AHB3LPENSETR [
//     /// DCMILPEN
//     DCMILPEN OFFSET(0) NUMBITS(1) [],
//     /// CRYP2LPEN
//     CRYP2LPEN OFFSET(4) NUMBITS(1) [],
//     /// HASH2LPEN
//     HASH2LPEN OFFSET(5) NUMBITS(1) [],
//     /// RNG2LPEN
//     RNG2LPEN OFFSET(6) NUMBITS(1) [],
//     /// CRC2LPEN
//     CRC2LPEN OFFSET(7) NUMBITS(1) [],
//     /// HSEMLPEN
//     HSEMLPEN OFFSET(11) NUMBITS(1) [],
//     /// IPCCLPEN
//     IPCCLPEN OFFSET(12) NUMBITS(1) []
// ],
// MC_AHB3LPENCLRR [
//     /// DCMILPEN
//     DCMILPEN OFFSET(0) NUMBITS(1) [],
//     /// CRYP2LPEN
//     CRYP2LPEN OFFSET(4) NUMBITS(1) [],
//     /// HASH2LPEN
//     HASH2LPEN OFFSET(5) NUMBITS(1) [],
//     /// RNG2LPEN
//     RNG2LPEN OFFSET(6) NUMBITS(1) [],
//     /// CRC2LPEN
//     CRC2LPEN OFFSET(7) NUMBITS(1) [],
//     /// HSEMLPEN
//     HSEMLPEN OFFSET(11) NUMBITS(1) [],
//     /// IPCCLPEN
//     IPCCLPEN OFFSET(12) NUMBITS(1) []
// ],
// MC_AHB4LPENSETR [
//     /// GPIOALPEN
//     GPIOALPEN OFFSET(0) NUMBITS(1) [],
//     /// GPIOBLPEN
//     GPIOBLPEN OFFSET(1) NUMBITS(1) [],
//     /// GPIOCLPEN
//     GPIOCLPEN OFFSET(2) NUMBITS(1) [],
//     /// GPIODLPEN
//     GPIODLPEN OFFSET(3) NUMBITS(1) [],
//     /// GPIOELPEN
//     GPIOELPEN OFFSET(4) NUMBITS(1) [],
//     /// GPIOFLPEN
//     GPIOFLPEN OFFSET(5) NUMBITS(1) [],
//     /// GPIOGLPEN
//     GPIOGLPEN OFFSET(6) NUMBITS(1) [],
//     /// GPIOHLPEN
//     GPIOHLPEN OFFSET(7) NUMBITS(1) [],
//     /// GPIOILPEN
//     GPIOILPEN OFFSET(8) NUMBITS(1) [],
//     /// GPIOJLPEN
//     GPIOJLPEN OFFSET(9) NUMBITS(1) [],
//     /// GPIOKLPEN
//     GPIOKLPEN OFFSET(10) NUMBITS(1) []
// ],
// MC_AHB4LPENCLRR [
//     /// GPIOALPEN
//     GPIOALPEN OFFSET(0) NUMBITS(1) [],
//     /// GPIOBLPEN
//     GPIOBLPEN OFFSET(1) NUMBITS(1) [],
//     /// GPIOCLPEN
//     GPIOCLPEN OFFSET(2) NUMBITS(1) [],
//     /// GPIODLPEN
//     GPIODLPEN OFFSET(3) NUMBITS(1) [],
//     /// GPIOELPEN
//     GPIOELPEN OFFSET(4) NUMBITS(1) [],
//     /// GPIOFLPEN
//     GPIOFLPEN OFFSET(5) NUMBITS(1) [],
//     /// GPIOGLPEN
//     GPIOGLPEN OFFSET(6) NUMBITS(1) [],
//     /// GPIOHLPEN
//     GPIOHLPEN OFFSET(7) NUMBITS(1) [],
//     /// GPIOILPEN
//     GPIOILPEN OFFSET(8) NUMBITS(1) [],
//     /// GPIOJLPEN
//     GPIOJLPEN OFFSET(9) NUMBITS(1) [],
//     /// GPIOKLPEN
//     GPIOKLPEN OFFSET(10) NUMBITS(1) []
// ],
// MC_AXIMLPENSETR [
//     /// SYSRAMLPEN
//     SYSRAMLPEN OFFSET(0) NUMBITS(1) []
// ],
// MC_AXIMLPENCLRR [
//     /// SYSRAMLPEN
//     SYSRAMLPEN OFFSET(0) NUMBITS(1) []
// ],
// MC_MLAHBLPENSETR [
//     /// SRAM1LPEN
//     SRAM1LPEN OFFSET(0) NUMBITS(1) [],
//     /// SRAM2LPEN
//     SRAM2LPEN OFFSET(1) NUMBITS(1) [],
//     /// SRAM34LPEN
//     SRAM34LPEN OFFSET(2) NUMBITS(1) [],
//     /// RETRAMLPEN
//     RETRAMLPEN OFFSET(4) NUMBITS(1) []
// ],
// MC_MLAHBLPENCLRR [
//     /// SRAM1LPEN
//     SRAM1LPEN OFFSET(0) NUMBITS(1) [],
//     /// SRAM2LPEN
//     SRAM2LPEN OFFSET(1) NUMBITS(1) [],
//     /// SRAM34LPEN
//     SRAM34LPEN OFFSET(2) NUMBITS(1) [],
//     /// RETRAMLPEN
//     RETRAMLPEN OFFSET(4) NUMBITS(1) []
// ],
// MC_RSTSCLRR [
//     /// PORRSTF
//     PORRSTF OFFSET(0) NUMBITS(1) [],
//     /// BORRSTF
//     BORRSTF OFFSET(1) NUMBITS(1) [],
//     /// PADRSTF
//     PADRSTF OFFSET(2) NUMBITS(1) [],
//     /// HCSSRSTF
//     HCSSRSTF OFFSET(3) NUMBITS(1) [],
//     /// VCORERSTF
//     VCORERSTF OFFSET(4) NUMBITS(1) [],
//     /// MCURSTF
//     MCURSTF OFFSET(5) NUMBITS(1) [],
//     /// MPSYSRSTF
//     MPSYSRSTF OFFSET(6) NUMBITS(1) [],
//     /// MCSYSRSTF
//     MCSYSRSTF OFFSET(7) NUMBITS(1) [],
//     /// IWDG1RSTF
//     IWDG1RSTF OFFSET(8) NUMBITS(1) [],
//     /// IWDG2RSTF
//     IWDG2RSTF OFFSET(9) NUMBITS(1) [],
//     /// WWDG1RSTF
//     WWDG1RSTF OFFSET(10) NUMBITS(1) []
// ],
// MC_CIER [
//     /// LSIRDYIE
//     LSIRDYIE OFFSET(0) NUMBITS(1) [],
//     /// LSERDYIE
//     LSERDYIE OFFSET(1) NUMBITS(1) [],
//     /// HSIRDYIE
//     HSIRDYIE OFFSET(2) NUMBITS(1) [],
//     /// HSERDYIE
//     HSERDYIE OFFSET(3) NUMBITS(1) [],
//     /// CSIRDYIE
//     CSIRDYIE OFFSET(4) NUMBITS(1) [],
//     /// PLL1DYIE
//     PLL1DYIE OFFSET(8) NUMBITS(1) [],
//     /// PLL2DYIE
//     PLL2DYIE OFFSET(9) NUMBITS(1) [],
//     /// PLL3DYIE
//     PLL3DYIE OFFSET(10) NUMBITS(1) [],
//     /// PLL4DYIE
//     PLL4DYIE OFFSET(11) NUMBITS(1) [],
//     /// LSECSSIE
//     LSECSSIE OFFSET(16) NUMBITS(1) [],
//     /// WKUPIE
//     WKUPIE OFFSET(20) NUMBITS(1) []
// ],
// MC_CIFR [
//     /// LSIRDYF
//     LSIRDYF OFFSET(0) NUMBITS(1) [],
//     /// LSERDYF
//     LSERDYF OFFSET(1) NUMBITS(1) [],
//     /// HSIRDYF
//     HSIRDYF OFFSET(2) NUMBITS(1) [],
//     /// HSERDYF
//     HSERDYF OFFSET(3) NUMBITS(1) [],
//     /// CSIRDYF
//     CSIRDYF OFFSET(4) NUMBITS(1) [],
//     /// PLL1DYF
//     PLL1DYF OFFSET(8) NUMBITS(1) [],
//     /// PLL2DYF
//     PLL2DYF OFFSET(9) NUMBITS(1) [],
//     /// PLL3DYF
//     PLL3DYF OFFSET(10) NUMBITS(1) [],
//     /// PLL4DYF
//     PLL4DYF OFFSET(11) NUMBITS(1) [],
//     /// LSECSSF
//     LSECSSF OFFSET(16) NUMBITS(1) [],
//     /// WKUPF
//     WKUPF OFFSET(20) NUMBITS(1) []
// ],
// VERR [
//     /// MINREV
//     MINREV OFFSET(0) NUMBITS(4) [],
//     /// MAJREV
//     MAJREV OFFSET(4) NUMBITS(4) []
// ],
// IDR [
//     /// ID
//     ID OFFSET(0) NUMBITS(32) []
// ],
// SIDR [
//     /// SID
//     SID OFFSET(0) NUMBITS(32) []
// ]
// ];
// const RCC_BASE: StaticRef<RccRegisters> =
//     unsafe { StaticRef::new(0x50000000 as *const RccRegisters) };

// pub struct Rcc {
//     registers: StaticRef<RccRegisters>,
// }

// impl Rcc {
//     pub const fn new() -> Rcc {
//         Rcc {
//             registers: RCC_BASE,
//         }
//     }

//     fn configure_rng_clock(&self) {
//         self.registers.pllcfgr.modify(PLLCFGR::PLLQ.val(2));
//         self.registers.cr.modify(CR::PLLON::SET);
//     }

//     // I2C1 clock

//     fn is_enabled_i2c1_clock(&self) -> bool {
//         self.registers.apb1enr.is_set(APB1ENR::I2C1EN)
//     }

//     fn enable_i2c1_clock(&self) {
//         self.registers.apb1enr.modify(APB1ENR::I2C1EN::SET);
//         self.registers.apb1rstr.modify(APB1RSTR::I2C1RST::SET);
//         self.registers.apb1rstr.modify(APB1RSTR::I2C1RST::CLEAR);
//     }

//     fn disable_i2c1_clock(&self) {
//         self.registers.apb1enr.modify(APB1ENR::I2C1EN::CLEAR)
//     }

//     // SPI3 clock

//     fn is_enabled_spi3_clock(&self) -> bool {
//         self.registers.apb1enr.is_set(APB1ENR::SPI3EN)
//     }

//     fn enable_spi3_clock(&self) {
//         self.registers.apb1enr.modify(APB1ENR::SPI3EN::SET)
//     }

//     fn disable_spi3_clock(&self) {
//         self.registers.apb1enr.modify(APB1ENR::SPI3EN::CLEAR)
//     }

//     // TIM2 clock

//     fn is_enabled_tim2_clock(&self) -> bool {
//         self.registers.apb1enr.is_set(APB1ENR::TIM2EN)
//     }

//     fn enable_tim2_clock(&self) {
//         self.registers.apb1enr.modify(APB1ENR::TIM2EN::SET)
//     }

//     fn disable_tim2_clock(&self) {
//         self.registers.apb1enr.modify(APB1ENR::TIM2EN::CLEAR)
//     }

//     // SYSCFG clock

//     fn is_enabled_syscfg_clock(&self) -> bool {
//         self.registers.apb2enr.is_set(APB2ENR::SYSCFGEN)
//     }

//     fn enable_syscfg_clock(&self) {
//         self.registers.apb2enr.modify(APB2ENR::SYSCFGEN::SET)
//     }

//     fn disable_syscfg_clock(&self) {
//         self.registers.apb2enr.modify(APB2ENR::SYSCFGEN::CLEAR)
//     }

//     // DMA1 clock

//     fn is_enabled_dma1_clock(&self) -> bool {
//         self.registers.ahb1enr.is_set(AHB1ENR::DMA1EN)
//     }

//     fn enable_dma1_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::DMA1EN::SET)
//     }

//     fn disable_dma1_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::DMA1EN::CLEAR)
//     }

//     // GPIOH clock

//     fn is_enabled_gpioh_clock(&self) -> bool {
//         self.registers.ahb1enr.is_set(AHB1ENR::GPIOHEN)
//     }

//     fn enable_gpioh_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::GPIOHEN::SET)
//     }

//     fn disable_gpioh_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::GPIOHEN::CLEAR)
//     }

//     // GPIOG clock

//     fn is_enabled_gpiog_clock(&self) -> bool {
//         self.registers.ahb1enr.is_set(AHB1ENR::GPIOGEN)
//     }

//     fn enable_gpiog_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::GPIOGEN::SET)
//     }

//     fn disable_gpiog_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::GPIOGEN::CLEAR)
//     }

//     // GPIOF clock

//     fn is_enabled_gpiof_clock(&self) -> bool {
//         self.registers.ahb1enr.is_set(AHB1ENR::GPIOFEN)
//     }

//     fn enable_gpiof_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::GPIOFEN::SET)
//     }

//     fn disable_gpiof_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::GPIOFEN::CLEAR)
//     }

//     // GPIOE clock

//     fn is_enabled_gpioe_clock(&self) -> bool {
//         self.registers.ahb1enr.is_set(AHB1ENR::GPIOEEN)
//     }

//     fn enable_gpioe_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::GPIOEEN::SET)
//     }

//     fn disable_gpioe_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::GPIOEEN::CLEAR)
//     }

//     // GPIOD clock

//     fn is_enabled_gpiod_clock(&self) -> bool {
//         self.registers.ahb1enr.is_set(AHB1ENR::GPIODEN)
//     }

//     fn enable_gpiod_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::GPIODEN::SET)
//     }

//     fn disable_gpiod_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::GPIODEN::CLEAR)
//     }

//     // GPIOC clock

//     fn is_enabled_gpioc_clock(&self) -> bool {
//         self.registers.ahb1enr.is_set(AHB1ENR::GPIOCEN)
//     }

//     fn enable_gpioc_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::GPIOCEN::SET)
//     }

//     fn disable_gpioc_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::GPIOCEN::CLEAR)
//     }

//     // GPIOB clock

//     fn is_enabled_gpiob_clock(&self) -> bool {
//         self.registers.ahb1enr.is_set(AHB1ENR::GPIOBEN)
//     }

//     fn enable_gpiob_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::GPIOBEN::SET)
//     }

//     fn disable_gpiob_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::GPIOBEN::CLEAR)
//     }

//     // GPIOA clock

//     fn is_enabled_gpioa_clock(&self) -> bool {
//         self.registers.ahb1enr.is_set(AHB1ENR::GPIOAEN)
//     }

//     fn enable_gpioa_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::GPIOAEN::SET)
//     }

//     fn disable_gpioa_clock(&self) {
//         self.registers.ahb1enr.modify(AHB1ENR::GPIOAEN::CLEAR)
//     }

//     // FMC

//     fn is_enabled_fmc_clock(&self) -> bool {
//         self.registers.ahb3enr.is_set(AHB3ENR::FMCEN)
//     }

//     fn enable_fmc_clock(&self) {
//         self.registers.ahb3enr.modify(AHB3ENR::FMCEN::SET)
//     }

//     fn disable_fmc_clock(&self) {
//         self.registers.ahb3enr.modify(AHB3ENR::FMCEN::CLEAR)
//     }

//     // USART2 clock

//     fn is_enabled_usart2_clock(&self) -> bool {
//         self.registers.apb1enr.is_set(APB1ENR::USART2EN)
//     }

//     fn enable_usart2_clock(&self) {
//         self.registers.apb1enr.modify(APB1ENR::USART2EN::SET)
//     }

//     fn disable_usart2_clock(&self) {
//         self.registers.apb1enr.modify(APB1ENR::USART2EN::CLEAR)
//     }

//     // USART3 clock

//     fn is_enabled_usart3_clock(&self) -> bool {
//         self.registers.apb1enr.is_set(APB1ENR::USART3EN)
//     }

//     fn enable_usart3_clock(&self) {
//         self.registers.apb1enr.modify(APB1ENR::USART3EN::SET)
//     }

//     fn disable_usart3_clock(&self) {
//         self.registers.apb1enr.modify(APB1ENR::USART3EN::CLEAR)
//     }

//     // ADC1 clock

//     fn is_enabled_adc1_clock(&self) -> bool {
//         self.registers.apb2enr.is_set(APB2ENR::ADC1EN)
//     }

//     fn enable_adc1_clock(&self) {
//         self.registers.apb2enr.modify(APB2ENR::ADC1EN::SET)
//     }

//     fn disable_adc1_clock(&self) {
//         self.registers.apb2enr.modify(APB2ENR::ADC1EN::CLEAR)
//     }

//     // RNG clock

//     fn is_enabled_rng_clock(&self) -> bool {
//         self.registers.ahb2enr.is_set(AHB2ENR::RNGEN)
//     }

//     fn enable_rng_clock(&self) {
//         self.registers.ahb2enr.modify(AHB2ENR::RNGEN::SET);
//     }

//     fn disable_rng_clock(&self) {
//         self.registers.ahb2enr.modify(AHB2ENR::RNGEN::CLEAR);
//     }

//     // OTGFS clock

//     fn is_enabled_otgfs_clock(&self) -> bool {
//         self.registers.ahb2enr.is_set(AHB2ENR::OTGFSEN)
//     }

//     fn enable_otgfs_clock(&self) {
//         self.registers.ahb2enr.modify(AHB2ENR::OTGFSEN::SET);
//     }

//     fn disable_otgfs_clock(&self) {
//         self.registers.ahb2enr.modify(AHB2ENR::OTGFSEN::CLEAR);
//     }
// }

// /// Clock sources for CPU
// pub enum CPUClock {
//     HSE,
//     HSI,
//     PLLCLK,
//     PPLLR,
// }


/// Clock initialized by Cortex-A7

pub struct PeripheralClock<'a> (&'a PhantomData<()>);

impl<'a> ClockInterface for PeripheralClock<'a> {
    fn is_enabled(&self) -> bool {
        true
    }

    fn enable(&self) {
        
    }

    fn disable(&self) {
        
    }
}

// pub struct PeripheralClock<'a> {
//     pub clock: PeripheralClockType,
//     rcc: &'a Rcc,
// }

// /// Bus + Clock name for the peripherals
// pub enum PeripheralClockType {
//     AHB1(HCLK1),
//     AHB2(HCLK2),
//     AHB3(HCLK3),
//     APB1(PCLK1),
//     APB2(PCLK2),
// }

// /// Peripherals clocked by HCLK1
// pub enum HCLK1 {
//     DMA1,
//     GPIOH,
//     GPIOG,
//     GPIOF,
//     GPIOE,
//     GPIOD,
//     GPIOC,
//     GPIOB,
//     GPIOA,
// }

// /// Peripherals clocked by HCLK3
// pub enum HCLK3 {
//     FMC,
// }

// /// Peripherals clocked by HCLK2
// pub enum HCLK2 {
//     RNG,
//     OTGFS,
// }

// /// Peripherals clocked by PCLK1
// pub enum PCLK1 {
//     TIM2,
//     USART2,
//     USART3,
//     SPI3,
//     I2C1,
// }

// /// Peripherals clocked by PCLK2
// pub enum PCLK2 {
//     ADC1,
//     SYSCFG,
// }

// impl<'a> PeripheralClock<'a> {
//     pub const fn new(clock: PeripheralClockType, rcc: &'a Rcc) -> Self {
//         Self { clock, rcc }
//     }

//     pub fn configure_rng_clock(&self) {
//         self.rcc.configure_rng_clock();
//     }
// }

// impl<'a> ClockInterface for PeripheralClock<'a> {
//     fn is_enabled(&self) -> bool {
//         match self.clock {
//             PeripheralClockType::AHB1(ref v) => match v {
//                 HCLK1::DMA1 => self.rcc.is_enabled_dma1_clock(),
//                 HCLK1::GPIOH => self.rcc.is_enabled_gpioh_clock(),
//                 HCLK1::GPIOG => self.rcc.is_enabled_gpiog_clock(),
//                 HCLK1::GPIOF => self.rcc.is_enabled_gpiof_clock(),
//                 HCLK1::GPIOE => self.rcc.is_enabled_gpioe_clock(),
//                 HCLK1::GPIOD => self.rcc.is_enabled_gpiod_clock(),
//                 HCLK1::GPIOC => self.rcc.is_enabled_gpioc_clock(),
//                 HCLK1::GPIOB => self.rcc.is_enabled_gpiob_clock(),
//                 HCLK1::GPIOA => self.rcc.is_enabled_gpioa_clock(),
//             },
//             PeripheralClockType::AHB2(ref v) => match v {
//                 HCLK2::RNG => self.rcc.is_enabled_rng_clock(),
//                 HCLK2::OTGFS => self.rcc.is_enabled_otgfs_clock(),
//             },
//             PeripheralClockType::AHB3(ref v) => match v {
//                 HCLK3::FMC => self.rcc.is_enabled_fmc_clock(),
//             },
//             PeripheralClockType::APB1(ref v) => match v {
//                 PCLK1::TIM2 => self.rcc.is_enabled_tim2_clock(),
//                 PCLK1::USART2 => self.rcc.is_enabled_usart2_clock(),
//                 PCLK1::USART3 => self.rcc.is_enabled_usart3_clock(),
//                 PCLK1::I2C1 => self.rcc.is_enabled_i2c1_clock(),
//                 PCLK1::SPI3 => self.rcc.is_enabled_spi3_clock(),
//             },
//             PeripheralClockType::APB2(ref v) => match v {
//                 PCLK2::ADC1 => self.rcc.is_enabled_adc1_clock(),
//                 PCLK2::SYSCFG => self.rcc.is_enabled_syscfg_clock(),
//             },
//         }
//     }

//     fn enable(&self) {
//         match self.clock {
//             PeripheralClockType::AHB1(ref v) => match v {
//                 HCLK1::DMA1 => {
//                     self.rcc.enable_dma1_clock();
//                 }
//                 HCLK1::GPIOH => {
//                     self.rcc.enable_gpioh_clock();
//                 }
//                 HCLK1::GPIOG => {
//                     self.rcc.enable_gpiog_clock();
//                 }
//                 HCLK1::GPIOF => {
//                     self.rcc.enable_gpiof_clock();
//                 }
//                 HCLK1::GPIOE => {
//                     self.rcc.enable_gpioe_clock();
//                 }
//                 HCLK1::GPIOD => {
//                     self.rcc.enable_gpiod_clock();
//                 }
//                 HCLK1::GPIOC => {
//                     self.rcc.enable_gpioc_clock();
//                 }
//                 HCLK1::GPIOB => {
//                     self.rcc.enable_gpiob_clock();
//                 }
//                 HCLK1::GPIOA => {
//                     self.rcc.enable_gpioa_clock();
//                 }
//             },
//             PeripheralClockType::AHB2(ref v) => match v {
//                 HCLK2::RNG => {
//                     self.rcc.enable_rng_clock();
//                 }
//                 HCLK2::OTGFS => {
//                     self.rcc.enable_otgfs_clock();
//                 }
//             },
//             PeripheralClockType::AHB3(ref v) => match v {
//                 HCLK3::FMC => self.rcc.enable_fmc_clock(),
//             },
//             PeripheralClockType::APB1(ref v) => match v {
//                 PCLK1::TIM2 => {
//                     self.rcc.enable_tim2_clock();
//                 }
//                 PCLK1::USART2 => {
//                     self.rcc.enable_usart2_clock();
//                 }
//                 PCLK1::USART3 => {
//                     self.rcc.enable_usart3_clock();
//                 }
//                 PCLK1::I2C1 => {
//                     self.rcc.enable_i2c1_clock();
//                 }
//                 PCLK1::SPI3 => {
//                     self.rcc.enable_spi3_clock();
//                 }
//             },
//             PeripheralClockType::APB2(ref v) => match v {
//                 PCLK2::ADC1 => {
//                     self.rcc.enable_adc1_clock();
//                 }
//                 PCLK2::SYSCFG => {
//                     self.rcc.enable_syscfg_clock();
//                 }
//             },
//         }
//     }

//     fn disable(&self) {
//         match self.clock {
//             PeripheralClockType::AHB1(ref v) => match v {
//                 HCLK1::DMA1 => {
//                     self.rcc.disable_dma1_clock();
//                 }
//                 HCLK1::GPIOH => {
//                     self.rcc.disable_gpioh_clock();
//                 }
//                 HCLK1::GPIOG => {
//                     self.rcc.disable_gpiog_clock();
//                 }
//                 HCLK1::GPIOF => {
//                     self.rcc.disable_gpiof_clock();
//                 }
//                 HCLK1::GPIOE => {
//                     self.rcc.disable_gpioe_clock();
//                 }
//                 HCLK1::GPIOD => {
//                     self.rcc.disable_gpiod_clock();
//                 }
//                 HCLK1::GPIOC => {
//                     self.rcc.disable_gpioc_clock();
//                 }
//                 HCLK1::GPIOB => {
//                     self.rcc.disable_gpiob_clock();
//                 }
//                 HCLK1::GPIOA => {
//                     self.rcc.disable_gpioa_clock();
//                 }
//             },
//             PeripheralClockType::AHB2(ref v) => match v {
//                 HCLK2::RNG => {
//                     self.rcc.disable_rng_clock();
//                 }
//                 HCLK2::OTGFS => {
//                     self.rcc.disable_otgfs_clock();
//                 }
//             },
//             PeripheralClockType::AHB3(ref v) => match v {
//                 HCLK3::FMC => self.rcc.disable_fmc_clock(),
//             },
//             PeripheralClockType::APB1(ref v) => match v {
//                 PCLK1::TIM2 => {
//                     self.rcc.disable_tim2_clock();
//                 }
//                 PCLK1::USART2 => {
//                     self.rcc.disable_usart2_clock();
//                 }
//                 PCLK1::USART3 => {
//                     self.rcc.disable_usart3_clock();
//                 }
//                 PCLK1::I2C1 => {
//                     self.rcc.disable_i2c1_clock();
//                 }
//                 PCLK1::SPI3 => {
//                     self.rcc.disable_spi3_clock();
//                 }
//             },
//             PeripheralClockType::APB2(ref v) => match v {
//                 PCLK2::ADC1 => {
//                     self.rcc.disable_adc1_clock();
//                 }
//                 PCLK2::SYSCFG => {
//                     self.rcc.disable_syscfg_clock();
//                 }
//             },
//         }
//     }
// }
