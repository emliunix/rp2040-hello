//! Rainbow effect color wheel using the onboard NeoPixel on an Adafruit Feather RP2040 board
//!
//! This flows smoothly through various colors on the onboard NeoPixel.
//! Uses the `ws2812_pio` driver to control the NeoPixel, which in turns uses the
//! RP2040's PIO block.
#![no_std]
#![no_main]

use cortex_m::prelude::*;
use embedded_time::{duration::Extensions, rate::Extensions as RateExtensions};
use ssd1306::{size::DisplaySize128x64, I2CDisplayInterface, rotation::DisplayRotation, mode::DisplayConfig};
use xiao2040_bsp as bsp;
use bsp::{
    hal::{
        clocks::{init_clocks_and_plls},
        pac,
        timer::Timer,
        watchdog::Watchdog,
        Sio, gpio::{PinId, PinMode, ValidPinMode, PushPullOutput, DynPin, Pin}, I2C,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};
use core::{convert::Infallible, fmt::Write};
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use defmt;
use defmt_rtt as _;
use panic_probe as _;

trait AsDynPin {
    fn as_dyn(self: Self) -> DynPin;
}

impl <I: PinId, M: PinMode + ValidPinMode<I>> AsDynPin for Pin<I, M> {
    #[inline]
    fn as_dyn(self: Self) -> DynPin {
        self.into()
    }
}

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    defmt::info!("Board initialized...");

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut delay = timer.count_down();

    configure_led(
        &mut pins.rgb_r.into_push_pull_output(),
        &mut pins.rgb_g.into_push_pull_output(),
        &mut pins.rgb_b.into_push_pull_output(),
    ).unwrap();

    let i2c = I2C::i2c1(
        pac.I2C1,
        pins.sda.into_mode(), pins.scl.into_mode(),
        100_u32.kHz(),
        &mut pac.RESETS,
        XOSC_CRYSTAL_FREQ.Hz()
    );

    let i2c_i = I2CDisplayInterface::new_custom_address(i2c, 0x3C);

    let mut oled_screen = ssd1306::Ssd1306::new(i2c_i, DisplaySize128x64 {}, DisplayRotation::Rotate0)
        .into_terminal_mode();
    oled_screen.init().unwrap();

    let mut n = 1;
    loop {
        defmt::info!("tick");
        oled_screen.clear().unwrap();
        write!(oled_screen, "Hello world!!!\ntick = {}", n).unwrap();
        n += 1;
        delay.start(1_u32.seconds());
        nb::block!(delay.wait()).unwrap();
    }
}

fn configure_led<PR,PG,PB,E>(pr: &mut PR, pg: &mut PG, pb: &mut PB) -> Result<(), E>
where PR: OutputPin<Error = E>, PG: OutputPin<Error = E>, PB: OutputPin<Error = E>
{
    pr.set_high()?;
    pg.set_low()?;
    pb.set_low()?;
    Ok(())
}