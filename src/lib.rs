#![doc = "Peripheral access API for MK20D7 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
extern "C" {
    fn INT_DMA0();
    fn INT_DMA1();
    fn INT_DMA2();
    fn INT_DMA3();
    fn INT_DMA4();
    fn INT_DMA5();
    fn INT_DMA6();
    fn INT_DMA7();
    fn INT_DMA8();
    fn INT_DMA9();
    fn INT_DMA10();
    fn INT_DMA11();
    fn INT_DMA12();
    fn INT_DMA13();
    fn INT_DMA14();
    fn INT_DMA15();
    fn INT_DMA_ERROR();
    fn INT_FTFL();
    fn INT_LVD_LVW();
    fn INT_LLW();
    fn INT_WATCHDOG();
    fn INT_I2C0();
    fn INT_I2C1();
    fn INT_SPI0();
    fn INT_SPI1();
    fn INT_CAN0_ORED_MESSAGE_BUFFER();
    fn INT_CAN0_BUS_OFF();
    fn INT_CAN0_ERROR();
    fn INT_CAN0_TX_WARNING();
    fn INT_CAN0_RX_WARNING();
    fn INT_CAN0_WAKE_UP();
    fn INT_I2S0_TX();
    fn INT_I2S0_RX();
    fn INT_UART0_LON();
    fn INT_UART0_RX_TX();
    fn INT_UART0_ERR();
    fn INT_UART1_RX_TX();
    fn INT_UART1_ERR();
    fn INT_UART2_RX_TX();
    fn INT_UART2_ERR();
    fn INT_UART3_RX_TX();
    fn INT_UART3_ERR();
    fn INT_UART4_RX_TX();
    fn INT_UART4_ERR();
    fn INT_ADC0();
    fn INT_ADC1();
    fn INT_CMP0();
    fn INT_CMP1();
    fn INT_CMP2();
    fn INT_FTM0();
    fn INT_FTM1();
    fn INT_FTM2();
    fn INT_CMT();
    fn INT_RTC();
    fn INT_RTC_SECONDS();
    fn INT_PIT0();
    fn INT_PIT1();
    fn INT_PIT2();
    fn INT_PIT3();
    fn INT_PDB0();
    fn INT_USB0();
    fn INT_USBDCD();
    fn INT_RESERVED95();
    fn INT_DAC0();
    fn INT_TSI0();
    fn INT_LPTIMER();
    fn INT_PORTA();
    fn INT_PORTB();
    fn INT_PORTC();
    fn INT_PORTD();
    fn INT_PORTE();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 108] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: INT_DMA0 },
    Vector { _handler: INT_DMA1 },
    Vector { _handler: INT_DMA2 },
    Vector { _handler: INT_DMA3 },
    Vector { _handler: INT_DMA4 },
    Vector { _handler: INT_DMA5 },
    Vector { _handler: INT_DMA6 },
    Vector { _handler: INT_DMA7 },
    Vector { _handler: INT_DMA8 },
    Vector { _handler: INT_DMA9 },
    Vector {
        _handler: INT_DMA10,
    },
    Vector {
        _handler: INT_DMA11,
    },
    Vector {
        _handler: INT_DMA12,
    },
    Vector {
        _handler: INT_DMA13,
    },
    Vector {
        _handler: INT_DMA14,
    },
    Vector {
        _handler: INT_DMA15,
    },
    Vector {
        _handler: INT_DMA_ERROR,
    },
    Vector { _reserved: 0 },
    Vector { _handler: INT_FTFL },
    Vector { _reserved: 0 },
    Vector {
        _handler: INT_LVD_LVW,
    },
    Vector { _handler: INT_LLW },
    Vector {
        _handler: INT_WATCHDOG,
    },
    Vector { _reserved: 0 },
    Vector { _handler: INT_I2C0 },
    Vector { _handler: INT_I2C1 },
    Vector { _handler: INT_SPI0 },
    Vector { _handler: INT_SPI1 },
    Vector { _reserved: 0 },
    Vector {
        _handler: INT_CAN0_ORED_MESSAGE_BUFFER,
    },
    Vector {
        _handler: INT_CAN0_BUS_OFF,
    },
    Vector {
        _handler: INT_CAN0_ERROR,
    },
    Vector {
        _handler: INT_CAN0_TX_WARNING,
    },
    Vector {
        _handler: INT_CAN0_RX_WARNING,
    },
    Vector {
        _handler: INT_CAN0_WAKE_UP,
    },
    Vector {
        _handler: INT_I2S0_TX,
    },
    Vector {
        _handler: INT_I2S0_RX,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: INT_UART0_LON,
    },
    Vector {
        _handler: INT_UART0_RX_TX,
    },
    Vector {
        _handler: INT_UART0_ERR,
    },
    Vector {
        _handler: INT_UART1_RX_TX,
    },
    Vector {
        _handler: INT_UART1_ERR,
    },
    Vector {
        _handler: INT_UART2_RX_TX,
    },
    Vector {
        _handler: INT_UART2_ERR,
    },
    Vector {
        _handler: INT_UART3_RX_TX,
    },
    Vector {
        _handler: INT_UART3_ERR,
    },
    Vector {
        _handler: INT_UART4_RX_TX,
    },
    Vector {
        _handler: INT_UART4_ERR,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: INT_ADC0 },
    Vector { _handler: INT_ADC1 },
    Vector { _handler: INT_CMP0 },
    Vector { _handler: INT_CMP1 },
    Vector { _handler: INT_CMP2 },
    Vector { _handler: INT_FTM0 },
    Vector { _handler: INT_FTM1 },
    Vector { _handler: INT_FTM2 },
    Vector { _handler: INT_CMT },
    Vector { _handler: INT_RTC },
    Vector {
        _handler: INT_RTC_SECONDS,
    },
    Vector { _handler: INT_PIT0 },
    Vector { _handler: INT_PIT1 },
    Vector { _handler: INT_PIT2 },
    Vector { _handler: INT_PIT3 },
    Vector { _handler: INT_PDB0 },
    Vector { _handler: INT_USB0 },
    Vector {
        _handler: INT_USBDCD,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: INT_RESERVED95,
    },
    Vector { _reserved: 0 },
    Vector { _handler: INT_DAC0 },
    Vector { _reserved: 0 },
    Vector { _handler: INT_TSI0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: INT_LPTIMER,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: INT_PORTA,
    },
    Vector {
        _handler: INT_PORTB,
    },
    Vector {
        _handler: INT_PORTC,
    },
    Vector {
        _handler: INT_PORTD,
    },
    Vector {
        _handler: INT_PORTE,
    },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "16 - INT_DMA0"]
    INT_DMA0,
    #[doc = "17 - INT_DMA1"]
    INT_DMA1,
    #[doc = "18 - INT_DMA2"]
    INT_DMA2,
    #[doc = "19 - INT_DMA3"]
    INT_DMA3,
    #[doc = "20 - INT_DMA4"]
    INT_DMA4,
    #[doc = "21 - INT_DMA5"]
    INT_DMA5,
    #[doc = "22 - INT_DMA6"]
    INT_DMA6,
    #[doc = "23 - INT_DMA7"]
    INT_DMA7,
    #[doc = "24 - INT_DMA8"]
    INT_DMA8,
    #[doc = "25 - INT_DMA9"]
    INT_DMA9,
    #[doc = "26 - INT_DMA10"]
    INT_DMA10,
    #[doc = "27 - INT_DMA11"]
    INT_DMA11,
    #[doc = "28 - INT_DMA12"]
    INT_DMA12,
    #[doc = "29 - INT_DMA13"]
    INT_DMA13,
    #[doc = "30 - INT_DMA14"]
    INT_DMA14,
    #[doc = "31 - INT_DMA15"]
    INT_DMA15,
    #[doc = "32 - INT_DMA_Error"]
    INT_DMA_ERROR,
    #[doc = "34 - INT_FTFL"]
    INT_FTFL,
    #[doc = "36 - INT_LVD_LVW"]
    INT_LVD_LVW,
    #[doc = "37 - INT_LLW"]
    INT_LLW,
    #[doc = "38 - INT_Watchdog"]
    INT_WATCHDOG,
    #[doc = "40 - INT_I2C0"]
    INT_I2C0,
    #[doc = "41 - INT_I2C1"]
    INT_I2C1,
    #[doc = "42 - INT_SPI0"]
    INT_SPI0,
    #[doc = "43 - INT_SPI1"]
    INT_SPI1,
    #[doc = "45 - INT_CAN0_ORed_Message_buffer"]
    INT_CAN0_ORED_MESSAGE_BUFFER,
    #[doc = "46 - INT_CAN0_Bus_Off"]
    INT_CAN0_BUS_OFF,
    #[doc = "47 - INT_CAN0_Error"]
    INT_CAN0_ERROR,
    #[doc = "48 - INT_CAN0_Tx_Warning"]
    INT_CAN0_TX_WARNING,
    #[doc = "49 - INT_CAN0_Rx_Warning"]
    INT_CAN0_RX_WARNING,
    #[doc = "50 - INT_CAN0_Wake_Up"]
    INT_CAN0_WAKE_UP,
    #[doc = "51 - INT_I2S0_Tx"]
    INT_I2S0_TX,
    #[doc = "52 - INT_I2S0_Rx"]
    INT_I2S0_RX,
    #[doc = "60 - INT_UART0_LON"]
    INT_UART0_LON,
    #[doc = "61 - INT_UART0_RX_TX"]
    INT_UART0_RX_TX,
    #[doc = "62 - INT_UART0_ERR"]
    INT_UART0_ERR,
    #[doc = "63 - INT_UART1_RX_TX"]
    INT_UART1_RX_TX,
    #[doc = "64 - INT_UART1_ERR"]
    INT_UART1_ERR,
    #[doc = "65 - INT_UART2_RX_TX"]
    INT_UART2_RX_TX,
    #[doc = "66 - INT_UART2_ERR"]
    INT_UART2_ERR,
    #[doc = "67 - INT_UART3_RX_TX"]
    INT_UART3_RX_TX,
    #[doc = "68 - INT_UART3_ERR"]
    INT_UART3_ERR,
    #[doc = "69 - INT_UART4_RX_TX"]
    INT_UART4_RX_TX,
    #[doc = "70 - INT_UART4_ERR"]
    INT_UART4_ERR,
    #[doc = "73 - INT_ADC0"]
    INT_ADC0,
    #[doc = "74 - INT_ADC1"]
    INT_ADC1,
    #[doc = "75 - INT_CMP0"]
    INT_CMP0,
    #[doc = "76 - INT_CMP1"]
    INT_CMP1,
    #[doc = "77 - INT_CMP2"]
    INT_CMP2,
    #[doc = "78 - INT_FTM0"]
    INT_FTM0,
    #[doc = "79 - INT_FTM1"]
    INT_FTM1,
    #[doc = "80 - INT_FTM2"]
    INT_FTM2,
    #[doc = "81 - INT_CMT"]
    INT_CMT,
    #[doc = "82 - INT_RTC"]
    INT_RTC,
    #[doc = "83 - INT_RTC_Seconds"]
    INT_RTC_SECONDS,
    #[doc = "84 - INT_PIT0"]
    INT_PIT0,
    #[doc = "85 - INT_PIT1"]
    INT_PIT1,
    #[doc = "86 - INT_PIT2"]
    INT_PIT2,
    #[doc = "87 - INT_PIT3"]
    INT_PIT3,
    #[doc = "88 - INT_PDB0"]
    INT_PDB0,
    #[doc = "89 - INT_USB0"]
    INT_USB0,
    #[doc = "90 - INT_USBDCD"]
    INT_USBDCD,
    #[doc = "95 - INT_Reserved95"]
    INT_RESERVED95,
    #[doc = "97 - INT_DAC0"]
    INT_DAC0,
    #[doc = "99 - INT_TSI0"]
    INT_TSI0,
    #[doc = "101 - INT_LPTimer"]
    INT_LPTIMER,
    #[doc = "103 - INT_PORTA"]
    INT_PORTA,
    #[doc = "104 - INT_PORTB"]
    INT_PORTB,
    #[doc = "105 - INT_PORTC"]
    INT_PORTC,
    #[doc = "106 - INT_PORTD"]
    INT_PORTD,
    #[doc = "107 - INT_PORTE"]
    INT_PORTE,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::INT_DMA0 => 16,
            Interrupt::INT_DMA1 => 17,
            Interrupt::INT_DMA2 => 18,
            Interrupt::INT_DMA3 => 19,
            Interrupt::INT_DMA4 => 20,
            Interrupt::INT_DMA5 => 21,
            Interrupt::INT_DMA6 => 22,
            Interrupt::INT_DMA7 => 23,
            Interrupt::INT_DMA8 => 24,
            Interrupt::INT_DMA9 => 25,
            Interrupt::INT_DMA10 => 26,
            Interrupt::INT_DMA11 => 27,
            Interrupt::INT_DMA12 => 28,
            Interrupt::INT_DMA13 => 29,
            Interrupt::INT_DMA14 => 30,
            Interrupt::INT_DMA15 => 31,
            Interrupt::INT_DMA_ERROR => 32,
            Interrupt::INT_FTFL => 34,
            Interrupt::INT_LVD_LVW => 36,
            Interrupt::INT_LLW => 37,
            Interrupt::INT_WATCHDOG => 38,
            Interrupt::INT_I2C0 => 40,
            Interrupt::INT_I2C1 => 41,
            Interrupt::INT_SPI0 => 42,
            Interrupt::INT_SPI1 => 43,
            Interrupt::INT_CAN0_ORED_MESSAGE_BUFFER => 45,
            Interrupt::INT_CAN0_BUS_OFF => 46,
            Interrupt::INT_CAN0_ERROR => 47,
            Interrupt::INT_CAN0_TX_WARNING => 48,
            Interrupt::INT_CAN0_RX_WARNING => 49,
            Interrupt::INT_CAN0_WAKE_UP => 50,
            Interrupt::INT_I2S0_TX => 51,
            Interrupt::INT_I2S0_RX => 52,
            Interrupt::INT_UART0_LON => 60,
            Interrupt::INT_UART0_RX_TX => 61,
            Interrupt::INT_UART0_ERR => 62,
            Interrupt::INT_UART1_RX_TX => 63,
            Interrupt::INT_UART1_ERR => 64,
            Interrupt::INT_UART2_RX_TX => 65,
            Interrupt::INT_UART2_ERR => 66,
            Interrupt::INT_UART3_RX_TX => 67,
            Interrupt::INT_UART3_ERR => 68,
            Interrupt::INT_UART4_RX_TX => 69,
            Interrupt::INT_UART4_ERR => 70,
            Interrupt::INT_ADC0 => 73,
            Interrupt::INT_ADC1 => 74,
            Interrupt::INT_CMP0 => 75,
            Interrupt::INT_CMP1 => 76,
            Interrupt::INT_CMP2 => 77,
            Interrupt::INT_FTM0 => 78,
            Interrupt::INT_FTM1 => 79,
            Interrupt::INT_FTM2 => 80,
            Interrupt::INT_CMT => 81,
            Interrupt::INT_RTC => 82,
            Interrupt::INT_RTC_SECONDS => 83,
            Interrupt::INT_PIT0 => 84,
            Interrupt::INT_PIT1 => 85,
            Interrupt::INT_PIT2 => 86,
            Interrupt::INT_PIT3 => 87,
            Interrupt::INT_PDB0 => 88,
            Interrupt::INT_USB0 => 89,
            Interrupt::INT_USBDCD => 90,
            Interrupt::INT_RESERVED95 => 95,
            Interrupt::INT_DAC0 => 97,
            Interrupt::INT_TSI0 => 99,
            Interrupt::INT_LPTIMER => 101,
            Interrupt::INT_PORTA => 103,
            Interrupt::INT_PORTB => 104,
            Interrupt::INT_PORTC => 105,
            Interrupt::INT_PORTD => 106,
            Interrupt::INT_PORTE => 107,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Flash Memory Interface"]
pub struct FTFL_FLASHCONFIG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTFL_FLASHCONFIG {}
impl FTFL_FLASHCONFIG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ftfl_flash_config::RegisterBlock {
        1024 as *const _
    }
}
impl Deref for FTFL_FLASHCONFIG {
    type Target = ftfl_flash_config::RegisterBlock;
    fn deref(&self) -> &ftfl_flash_config::RegisterBlock {
        unsafe { &*FTFL_FLASHCONFIG::ptr() }
    }
}
#[doc = "Flash Memory Interface"]
pub mod ftfl_flash_config;
#[doc = "AIPS-Lite Bridge"]
pub struct AIPS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AIPS0 {}
impl AIPS0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aips0::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for AIPS0 {
    type Target = aips0::RegisterBlock;
    fn deref(&self) -> &aips0::RegisterBlock {
        unsafe { &*AIPS0::ptr() }
    }
}
#[doc = "AIPS-Lite Bridge"]
pub mod aips0;
#[doc = "AIPS-Lite Bridge"]
pub struct AIPS1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AIPS1 {}
impl AIPS1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aips1::RegisterBlock {
        1074266112 as *const _
    }
}
impl Deref for AIPS1 {
    type Target = aips1::RegisterBlock;
    fn deref(&self) -> &aips1::RegisterBlock {
        unsafe { &*AIPS1::ptr() }
    }
}
#[doc = "AIPS-Lite Bridge"]
pub mod aips1;
#[doc = "Crossbar switch"]
pub struct AXBS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AXBS {}
impl AXBS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const axbs::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for AXBS {
    type Target = axbs::RegisterBlock;
    fn deref(&self) -> &axbs::RegisterBlock {
        unsafe { &*AXBS::ptr() }
    }
}
#[doc = "Crossbar switch"]
pub mod axbs;
#[doc = "Enhanced direct memory access controller"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    fn deref(&self) -> &dma::RegisterBlock {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "Enhanced direct memory access controller"]
pub mod dma;
#[doc = "FlexBus external bus interface"]
pub struct FB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FB {}
impl FB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fb::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for FB {
    type Target = fb::RegisterBlock;
    fn deref(&self) -> &fb::RegisterBlock {
        unsafe { &*FB::ptr() }
    }
}
#[doc = "FlexBus external bus interface"]
pub mod fb;
#[doc = "Flash Memory Controller"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fmc::RegisterBlock {
        1073868800 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    fn deref(&self) -> &fmc::RegisterBlock {
        unsafe { &*FMC::ptr() }
    }
}
#[doc = "Flash Memory Controller"]
pub mod fmc;
#[doc = "Flash Memory Interface"]
pub struct FTFL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTFL {}
impl FTFL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ftfl::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for FTFL {
    type Target = ftfl::RegisterBlock;
    fn deref(&self) -> &ftfl::RegisterBlock {
        unsafe { &*FTFL::ptr() }
    }
}
#[doc = "Flash Memory Interface"]
pub mod ftfl;
#[doc = "DMA channel multiplexor"]
pub struct DMAMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX {}
impl DMAMUX {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dmamux::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for DMAMUX {
    type Target = dmamux::RegisterBlock;
    fn deref(&self) -> &dmamux::RegisterBlock {
        unsafe { &*DMAMUX::ptr() }
    }
}
#[doc = "DMA channel multiplexor"]
pub mod dmamux;
#[doc = "Flex Controller Area Network module"]
pub struct CAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN0 {}
impl CAN0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can0::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for CAN0 {
    type Target = can0::RegisterBlock;
    fn deref(&self) -> &can0::RegisterBlock {
        unsafe { &*CAN0::ptr() }
    }
}
#[doc = "Flex Controller Area Network module"]
pub mod can0;
#[doc = "Deserial Serial Peripheral Interface"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1073922048 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Deserial Serial Peripheral Interface"]
pub mod spi0;
#[doc = "Deserial Serial Peripheral Interface"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073926144 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Deserial Serial Peripheral Interface"]
pub mod spi1;
#[doc = "Inter-IC Sound / Synchronous Audio Interface"]
pub struct I2S0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S0 {}
impl I2S0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s0::RegisterBlock {
        1073934336 as *const _
    }
}
impl Deref for I2S0 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &i2s0::RegisterBlock {
        unsafe { &*I2S0::ptr() }
    }
}
#[doc = "Inter-IC Sound / Synchronous Audio Interface"]
pub mod i2s0;
#[doc = "Cyclic Redundancy Check"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1073946624 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cyclic Redundancy Check"]
pub mod crc;
#[doc = "USB Device Charger Detection module"]
pub struct USBDCD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBDCD {}
impl USBDCD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usbdcd::RegisterBlock {
        1073958912 as *const _
    }
}
impl Deref for USBDCD {
    type Target = usbdcd::RegisterBlock;
    fn deref(&self) -> &usbdcd::RegisterBlock {
        unsafe { &*USBDCD::ptr() }
    }
}
#[doc = "USB Device Charger Detection module"]
pub mod usbdcd;
#[doc = "Programmable Delay Block"]
pub struct PDB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDB0 {}
impl PDB0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pdb0::RegisterBlock {
        1073963008 as *const _
    }
}
impl Deref for PDB0 {
    type Target = pdb0::RegisterBlock;
    fn deref(&self) -> &pdb0::RegisterBlock {
        unsafe { &*PDB0::ptr() }
    }
}
#[doc = "Programmable Delay Block"]
pub mod pdb0;
#[doc = "Periodic Interrupt Timer"]
pub struct PIT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIT {}
impl PIT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pit::RegisterBlock {
        1073967104 as *const _
    }
}
impl Deref for PIT {
    type Target = pit::RegisterBlock;
    fn deref(&self) -> &pit::RegisterBlock {
        unsafe { &*PIT::ptr() }
    }
}
#[doc = "Periodic Interrupt Timer"]
pub mod pit;
#[doc = "FlexTimer Module"]
pub struct FTM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTM0 {}
impl FTM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ftm0::RegisterBlock {
        1073971200 as *const _
    }
}
impl Deref for FTM0 {
    type Target = ftm0::RegisterBlock;
    fn deref(&self) -> &ftm0::RegisterBlock {
        unsafe { &*FTM0::ptr() }
    }
}
#[doc = "FlexTimer Module"]
pub mod ftm0;
#[doc = "FlexTimer Module"]
pub struct FTM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTM1 {}
impl FTM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ftm1::RegisterBlock {
        1073975296 as *const _
    }
}
impl Deref for FTM1 {
    type Target = ftm1::RegisterBlock;
    fn deref(&self) -> &ftm1::RegisterBlock {
        unsafe { &*FTM1::ptr() }
    }
}
#[doc = "FlexTimer Module"]
pub mod ftm1;
#[doc = "FlexTimer Module"]
pub struct FTM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTM2 {}
impl FTM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ftm2::RegisterBlock {
        1074495488 as *const _
    }
}
impl Deref for FTM2 {
    type Target = ftm2::RegisterBlock;
    fn deref(&self) -> &ftm2::RegisterBlock {
        unsafe { &*FTM2::ptr() }
    }
}
#[doc = "FlexTimer Module"]
pub mod ftm2;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc0::RegisterBlock {
        1073983488 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &adc0::RegisterBlock {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc0;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc1::RegisterBlock {
        1074507776 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc1;
#[doc = "Secure Real Time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073991680 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Secure Real Time Clock"]
pub mod rtc;
#[doc = "VBAT register file"]
pub struct RFVBAT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFVBAT {}
impl RFVBAT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rfvbat::RegisterBlock {
        1073995776 as *const _
    }
}
impl Deref for RFVBAT {
    type Target = rfvbat::RegisterBlock;
    fn deref(&self) -> &rfvbat::RegisterBlock {
        unsafe { &*RFVBAT::ptr() }
    }
}
#[doc = "VBAT register file"]
pub mod rfvbat;
#[doc = "Low Power Timer"]
pub struct LPTMR0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTMR0 {}
impl LPTMR0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lptmr0::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for LPTMR0 {
    type Target = lptmr0::RegisterBlock;
    fn deref(&self) -> &lptmr0::RegisterBlock {
        unsafe { &*LPTMR0::ptr() }
    }
}
#[doc = "Low Power Timer"]
pub mod lptmr0;
#[doc = "System register file"]
pub struct RFSYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFSYS {}
impl RFSYS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rfsys::RegisterBlock {
        1074008064 as *const _
    }
}
impl Deref for RFSYS {
    type Target = rfsys::RegisterBlock;
    fn deref(&self) -> &rfsys::RegisterBlock {
        unsafe { &*RFSYS::ptr() }
    }
}
#[doc = "System register file"]
pub mod rfsys;
#[doc = "Touch Sensing Input"]
pub struct TSI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TSI0 {}
impl TSI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tsi0::RegisterBlock {
        1074024448 as *const _
    }
}
impl Deref for TSI0 {
    type Target = tsi0::RegisterBlock;
    fn deref(&self) -> &tsi0::RegisterBlock {
        unsafe { &*TSI0::ptr() }
    }
}
#[doc = "Touch Sensing Input"]
pub mod tsi0;
#[doc = "System Integration Module"]
pub struct SIM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SIM {}
impl SIM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sim::RegisterBlock {
        1074032640 as *const _
    }
}
impl Deref for SIM {
    type Target = sim::RegisterBlock;
    fn deref(&self) -> &sim::RegisterBlock {
        unsafe { &*SIM::ptr() }
    }
}
#[doc = "System Integration Module"]
pub mod sim;
#[doc = "Pin Control and Interrupts"]
pub struct PORTA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTA {}
impl PORTA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const porta::RegisterBlock {
        1074040832 as *const _
    }
}
impl Deref for PORTA {
    type Target = porta::RegisterBlock;
    fn deref(&self) -> &porta::RegisterBlock {
        unsafe { &*PORTA::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod porta;
#[doc = "Pin Control and Interrupts"]
pub struct PORTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTB {}
impl PORTB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portb::RegisterBlock {
        1074044928 as *const _
    }
}
impl Deref for PORTB {
    type Target = portb::RegisterBlock;
    fn deref(&self) -> &portb::RegisterBlock {
        unsafe { &*PORTB::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portb;
#[doc = "Pin Control and Interrupts"]
pub struct PORTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTC {}
impl PORTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portc::RegisterBlock {
        1074049024 as *const _
    }
}
impl Deref for PORTC {
    type Target = portc::RegisterBlock;
    fn deref(&self) -> &portc::RegisterBlock {
        unsafe { &*PORTC::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portc;
#[doc = "Pin Control and Interrupts"]
pub struct PORTD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTD {}
impl PORTD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portd::RegisterBlock {
        1074053120 as *const _
    }
}
impl Deref for PORTD {
    type Target = portd::RegisterBlock;
    fn deref(&self) -> &portd::RegisterBlock {
        unsafe { &*PORTD::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portd;
#[doc = "Pin Control and Interrupts"]
pub struct PORTE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTE {}
impl PORTE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const porte::RegisterBlock {
        1074057216 as *const _
    }
}
impl Deref for PORTE {
    type Target = porte::RegisterBlock;
    fn deref(&self) -> &porte::RegisterBlock {
        unsafe { &*PORTE::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod porte;
#[doc = "Generation 2008 Watchdog Timer"]
pub struct WDOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG {}
impl WDOG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdog::RegisterBlock {
        1074077696 as *const _
    }
}
impl Deref for WDOG {
    type Target = wdog::RegisterBlock;
    fn deref(&self) -> &wdog::RegisterBlock {
        unsafe { &*WDOG::ptr() }
    }
}
#[doc = "Generation 2008 Watchdog Timer"]
pub mod wdog;
#[doc = "External Watchdog Monitor"]
pub struct EWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EWM {}
impl EWM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ewm::RegisterBlock {
        1074139136 as *const _
    }
}
impl Deref for EWM {
    type Target = ewm::RegisterBlock;
    fn deref(&self) -> &ewm::RegisterBlock {
        unsafe { &*EWM::ptr() }
    }
}
#[doc = "External Watchdog Monitor"]
pub mod ewm;
#[doc = "Carrier Modulator Transmitter"]
pub struct CMT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMT {}
impl CMT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cmt::RegisterBlock {
        1074143232 as *const _
    }
}
impl Deref for CMT {
    type Target = cmt::RegisterBlock;
    fn deref(&self) -> &cmt::RegisterBlock {
        unsafe { &*CMT::ptr() }
    }
}
#[doc = "Carrier Modulator Transmitter"]
pub mod cmt;
#[doc = "Multipurpose Clock Generator module"]
pub struct MCG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCG {}
impl MCG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mcg::RegisterBlock {
        1074151424 as *const _
    }
}
impl Deref for MCG {
    type Target = mcg::RegisterBlock;
    fn deref(&self) -> &mcg::RegisterBlock {
        unsafe { &*MCG::ptr() }
    }
}
#[doc = "Multipurpose Clock Generator module"]
pub mod mcg;
#[doc = "Oscillator"]
pub struct OSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSC {}
impl OSC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const osc::RegisterBlock {
        1074155520 as *const _
    }
}
impl Deref for OSC {
    type Target = osc::RegisterBlock;
    fn deref(&self) -> &osc::RegisterBlock {
        unsafe { &*OSC::ptr() }
    }
}
#[doc = "Oscillator"]
pub mod osc;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1074159616 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c0;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1074163712 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c1;
#[doc = "Serial Communication Interface"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1074176000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub mod uart0;
#[doc = "Serial Communication Interface"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart1::RegisterBlock {
        1074180096 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &uart1::RegisterBlock {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub mod uart1;
#[doc = "Serial Communication Interface"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart2::RegisterBlock {
        1074184192 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart2::RegisterBlock;
    fn deref(&self) -> &uart2::RegisterBlock {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub mod uart2;
#[doc = "Serial Communication Interface"]
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART3 {}
impl UART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart3::RegisterBlock {
        1074188288 as *const _
    }
}
impl Deref for UART3 {
    type Target = uart3::RegisterBlock;
    fn deref(&self) -> &uart3::RegisterBlock {
        unsafe { &*UART3::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub mod uart3;
#[doc = "Serial Communication Interface"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart4::RegisterBlock {
        1074700288 as *const _
    }
}
impl Deref for UART4 {
    type Target = uart4::RegisterBlock;
    fn deref(&self) -> &uart4::RegisterBlock {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub mod uart4;
#[doc = "Universal Serial Bus, OTG Capable Controller"]
pub struct USB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0 {}
impl USB0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0::RegisterBlock {
        1074208768 as *const _
    }
}
impl Deref for USB0 {
    type Target = usb0::RegisterBlock;
    fn deref(&self) -> &usb0::RegisterBlock {
        unsafe { &*USB0::ptr() }
    }
}
#[doc = "Universal Serial Bus, OTG Capable Controller"]
pub mod usb0;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP0 {}
impl CMP0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cmp0::RegisterBlock {
        1074212864 as *const _
    }
}
impl Deref for CMP0 {
    type Target = cmp0::RegisterBlock;
    fn deref(&self) -> &cmp0::RegisterBlock {
        unsafe { &*CMP0::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp0;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP1 {}
impl CMP1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cmp1::RegisterBlock {
        1074212872 as *const _
    }
}
impl Deref for CMP1 {
    type Target = cmp1::RegisterBlock;
    fn deref(&self) -> &cmp1::RegisterBlock {
        unsafe { &*CMP1::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp1;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP2 {}
impl CMP2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cmp2::RegisterBlock {
        1074212880 as *const _
    }
}
impl Deref for CMP2 {
    type Target = cmp2::RegisterBlock;
    fn deref(&self) -> &cmp2::RegisterBlock {
        unsafe { &*CMP2::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp2;
#[doc = "Voltage Reference"]
pub struct VREF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREF {}
impl VREF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vref::RegisterBlock {
        1074216960 as *const _
    }
}
impl Deref for VREF {
    type Target = vref::RegisterBlock;
    fn deref(&self) -> &vref::RegisterBlock {
        unsafe { &*VREF::ptr() }
    }
}
#[doc = "Voltage Reference"]
pub mod vref;
#[doc = "Low leakage wakeup unit"]
pub struct LLWU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LLWU {}
impl LLWU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const llwu::RegisterBlock {
        1074249728 as *const _
    }
}
impl Deref for LLWU {
    type Target = llwu::RegisterBlock;
    fn deref(&self) -> &llwu::RegisterBlock {
        unsafe { &*LLWU::ptr() }
    }
}
#[doc = "Low leakage wakeup unit"]
pub mod llwu;
#[doc = "Power Management Controller"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmc::RegisterBlock {
        1074253824 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    fn deref(&self) -> &pmc::RegisterBlock {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "Power Management Controller"]
pub mod pmc;
#[doc = "System Mode Controller"]
pub struct SMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMC {}
impl SMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const smc::RegisterBlock {
        1074257920 as *const _
    }
}
impl Deref for SMC {
    type Target = smc::RegisterBlock;
    fn deref(&self) -> &smc::RegisterBlock {
        unsafe { &*SMC::ptr() }
    }
}
#[doc = "System Mode Controller"]
pub mod smc;
#[doc = "Reset Control Module"]
pub struct RCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCM {}
impl RCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rcm::RegisterBlock {
        1074262016 as *const _
    }
}
impl Deref for RCM {
    type Target = rcm::RegisterBlock;
    fn deref(&self) -> &rcm::RegisterBlock {
        unsafe { &*RCM::ptr() }
    }
}
#[doc = "Reset Control Module"]
pub mod rcm;
#[doc = "12-Bit Digital-to-Analog Converter"]
pub struct DAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC0 {}
impl DAC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac0::RegisterBlock {
        1074577408 as *const _
    }
}
impl Deref for DAC0 {
    type Target = dac0::RegisterBlock;
    fn deref(&self) -> &dac0::RegisterBlock {
        unsafe { &*DAC0::ptr() }
    }
}
#[doc = "12-Bit Digital-to-Analog Converter"]
pub mod dac0;
#[doc = "General Purpose Input/Output"]
pub struct PTA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PTA {}
impl PTA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pta::RegisterBlock {
        1074786304 as *const _
    }
}
impl Deref for PTA {
    type Target = pta::RegisterBlock;
    fn deref(&self) -> &pta::RegisterBlock {
        unsafe { &*PTA::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod pta;
#[doc = "General Purpose Input/Output"]
pub struct PTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PTB {}
impl PTB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ptb::RegisterBlock {
        1074786368 as *const _
    }
}
impl Deref for PTB {
    type Target = ptb::RegisterBlock;
    fn deref(&self) -> &ptb::RegisterBlock {
        unsafe { &*PTB::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod ptb;
#[doc = "General Purpose Input/Output"]
pub struct PTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PTC {}
impl PTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ptc::RegisterBlock {
        1074786432 as *const _
    }
}
impl Deref for PTC {
    type Target = ptc::RegisterBlock;
    fn deref(&self) -> &ptc::RegisterBlock {
        unsafe { &*PTC::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod ptc;
#[doc = "General Purpose Input/Output"]
pub struct PTD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PTD {}
impl PTD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ptd::RegisterBlock {
        1074786496 as *const _
    }
}
impl Deref for PTD {
    type Target = ptd::RegisterBlock;
    fn deref(&self) -> &ptd::RegisterBlock {
        unsafe { &*PTD::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod ptd;
#[doc = "General Purpose Input/Output"]
pub struct PTE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PTE {}
impl PTE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pte::RegisterBlock {
        1074786560 as *const _
    }
}
impl Deref for PTE {
    type Target = pte::RegisterBlock;
    fn deref(&self) -> &pte::RegisterBlock {
        unsafe { &*PTE::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod pte;
#[doc = "System Control Registers"]
pub struct SYSTEMCONTROL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEMCONTROL {}
impl SYSTEMCONTROL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const system_control::RegisterBlock {
        3758153728 as *const _
    }
}
impl Deref for SYSTEMCONTROL {
    type Target = system_control::RegisterBlock;
    fn deref(&self) -> &system_control::RegisterBlock {
        unsafe { &*SYSTEMCONTROL::ptr() }
    }
}
#[doc = "System Control Registers"]
pub mod system_control;
#[doc = "System timer"]
pub struct SYSTICK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTICK {}
impl SYSTICK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sys_tick::RegisterBlock {
        3758153744 as *const _
    }
}
impl Deref for SYSTICK {
    type Target = sys_tick::RegisterBlock;
    fn deref(&self) -> &sys_tick::RegisterBlock {
        unsafe { &*SYSTICK::ptr() }
    }
}
#[doc = "System timer"]
pub mod sys_tick;
#[doc = "Core Platform Miscellaneous Control Module"]
pub struct MCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCM {}
impl MCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mcm::RegisterBlock {
        3758620672 as *const _
    }
}
impl Deref for MCM {
    type Target = mcm::RegisterBlock;
    fn deref(&self) -> &mcm::RegisterBlock {
        unsafe { &*MCM::ptr() }
    }
}
#[doc = "Core Platform Miscellaneous Control Module"]
pub mod mcm;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FTFL_FLASHCONFIG"]
    pub FTFL_FLASHCONFIG: FTFL_FLASHCONFIG,
    #[doc = "AIPS0"]
    pub AIPS0: AIPS0,
    #[doc = "AIPS1"]
    pub AIPS1: AIPS1,
    #[doc = "AXBS"]
    pub AXBS: AXBS,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "FB"]
    pub FB: FB,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "FTFL"]
    pub FTFL: FTFL,
    #[doc = "DMAMUX"]
    pub DMAMUX: DMAMUX,
    #[doc = "CAN0"]
    pub CAN0: CAN0,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "I2S0"]
    pub I2S0: I2S0,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "USBDCD"]
    pub USBDCD: USBDCD,
    #[doc = "PDB0"]
    pub PDB0: PDB0,
    #[doc = "PIT"]
    pub PIT: PIT,
    #[doc = "FTM0"]
    pub FTM0: FTM0,
    #[doc = "FTM1"]
    pub FTM1: FTM1,
    #[doc = "FTM2"]
    pub FTM2: FTM2,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "RFVBAT"]
    pub RFVBAT: RFVBAT,
    #[doc = "LPTMR0"]
    pub LPTMR0: LPTMR0,
    #[doc = "RFSYS"]
    pub RFSYS: RFSYS,
    #[doc = "TSI0"]
    pub TSI0: TSI0,
    #[doc = "SIM"]
    pub SIM: SIM,
    #[doc = "PORTA"]
    pub PORTA: PORTA,
    #[doc = "PORTB"]
    pub PORTB: PORTB,
    #[doc = "PORTC"]
    pub PORTC: PORTC,
    #[doc = "PORTD"]
    pub PORTD: PORTD,
    #[doc = "PORTE"]
    pub PORTE: PORTE,
    #[doc = "WDOG"]
    pub WDOG: WDOG,
    #[doc = "EWM"]
    pub EWM: EWM,
    #[doc = "CMT"]
    pub CMT: CMT,
    #[doc = "MCG"]
    pub MCG: MCG,
    #[doc = "OSC"]
    pub OSC: OSC,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "USB0"]
    pub USB0: USB0,
    #[doc = "CMP0"]
    pub CMP0: CMP0,
    #[doc = "CMP1"]
    pub CMP1: CMP1,
    #[doc = "CMP2"]
    pub CMP2: CMP2,
    #[doc = "VREF"]
    pub VREF: VREF,
    #[doc = "LLWU"]
    pub LLWU: LLWU,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "SMC"]
    pub SMC: SMC,
    #[doc = "RCM"]
    pub RCM: RCM,
    #[doc = "DAC0"]
    pub DAC0: DAC0,
    #[doc = "PTA"]
    pub PTA: PTA,
    #[doc = "PTB"]
    pub PTB: PTB,
    #[doc = "PTC"]
    pub PTC: PTC,
    #[doc = "PTD"]
    pub PTD: PTD,
    #[doc = "PTE"]
    pub PTE: PTE,
    #[doc = "SYSTEMCONTROL"]
    pub SYSTEMCONTROL: SYSTEMCONTROL,
    #[doc = "SYSTICK"]
    pub SYSTICK: SYSTICK,
    #[doc = "MCM"]
    pub MCM: MCM,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            FTFL_FLASHCONFIG: FTFL_FLASHCONFIG {
                _marker: PhantomData,
            },
            AIPS0: AIPS0 {
                _marker: PhantomData,
            },
            AIPS1: AIPS1 {
                _marker: PhantomData,
            },
            AXBS: AXBS {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            FB: FB {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            FTFL: FTFL {
                _marker: PhantomData,
            },
            DMAMUX: DMAMUX {
                _marker: PhantomData,
            },
            CAN0: CAN0 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            I2S0: I2S0 {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            USBDCD: USBDCD {
                _marker: PhantomData,
            },
            PDB0: PDB0 {
                _marker: PhantomData,
            },
            PIT: PIT {
                _marker: PhantomData,
            },
            FTM0: FTM0 {
                _marker: PhantomData,
            },
            FTM1: FTM1 {
                _marker: PhantomData,
            },
            FTM2: FTM2 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            RFVBAT: RFVBAT {
                _marker: PhantomData,
            },
            LPTMR0: LPTMR0 {
                _marker: PhantomData,
            },
            RFSYS: RFSYS {
                _marker: PhantomData,
            },
            TSI0: TSI0 {
                _marker: PhantomData,
            },
            SIM: SIM {
                _marker: PhantomData,
            },
            PORTA: PORTA {
                _marker: PhantomData,
            },
            PORTB: PORTB {
                _marker: PhantomData,
            },
            PORTC: PORTC {
                _marker: PhantomData,
            },
            PORTD: PORTD {
                _marker: PhantomData,
            },
            PORTE: PORTE {
                _marker: PhantomData,
            },
            WDOG: WDOG {
                _marker: PhantomData,
            },
            EWM: EWM {
                _marker: PhantomData,
            },
            CMT: CMT {
                _marker: PhantomData,
            },
            MCG: MCG {
                _marker: PhantomData,
            },
            OSC: OSC {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            UART3: UART3 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            USB0: USB0 {
                _marker: PhantomData,
            },
            CMP0: CMP0 {
                _marker: PhantomData,
            },
            CMP1: CMP1 {
                _marker: PhantomData,
            },
            CMP2: CMP2 {
                _marker: PhantomData,
            },
            VREF: VREF {
                _marker: PhantomData,
            },
            LLWU: LLWU {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            SMC: SMC {
                _marker: PhantomData,
            },
            RCM: RCM {
                _marker: PhantomData,
            },
            DAC0: DAC0 {
                _marker: PhantomData,
            },
            PTA: PTA {
                _marker: PhantomData,
            },
            PTB: PTB {
                _marker: PhantomData,
            },
            PTC: PTC {
                _marker: PhantomData,
            },
            PTD: PTD {
                _marker: PhantomData,
            },
            PTE: PTE {
                _marker: PhantomData,
            },
            SYSTEMCONTROL: SYSTEMCONTROL {
                _marker: PhantomData,
            },
            SYSTICK: SYSTICK {
                _marker: PhantomData,
            },
            MCM: MCM {
                _marker: PhantomData,
            },
        }
    }
}
