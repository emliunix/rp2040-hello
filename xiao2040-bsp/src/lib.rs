#![no_std]

pub extern crate rp2040_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

pub use hal::pac;

hal::bsp_pins!(
    Gpio0 {
        name: tx,
        aliases: { FunctionUart: UartTx }
    },
    Gpio1 {
        name: rx,
        aliases: { FunctionUart: UartRx }
    },
    Gpio2 {
        name: sclk,
        aliases: { FunctionSpi: Sclk }
    },
    Gpio3 {
        name: mosi,
        aliases: { FunctionSpi: Mosi }
    },
    Gpio4 {
        name: miso,
        aliases: { FunctionSpi: Miso }
    },
    Gpio6 {
        name: sda,
        aliases: { FunctionI2C: Sda }
    },
    Gpio7 {
        name: scl,
        aliases: { FunctionI2C: Scl }
    },
    Gpio11 { name: neopwr },
    Gpio12 { name: neopix },
    Gpio16 { name: rgb_g },
    Gpio17 { name: rgb_r },
    Gpio25 { name: rgb_b },

    Gpio26 { name: a0 },
    Gpio27 { name: a1 },
    Gpio28 { name: a2 },
    Gpio29 { name: a3 },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
