//! Rainbow effect color wheel using the onboard NeoPixel on an Adafruit Feather RP2040 board
//!
//! This flows smoothly through various colors on the onboard NeoPixel.
//! Uses the `ws2812_pio` driver to control the NeoPixel, which in turns uses the
//! RP2040's PIO block.
#![no_std]
#![no_main]

use cortex_m::prelude::_embedded_hal_timer_CountDown;
use embedded_time::duration::Extensions;
use xiao2040_bsp as bsp;
use bsp::{
    hal::{
        clocks::{init_clocks_and_plls},
        pac,
        timer::Timer,
        watchdog::Watchdog,
        Sio, gpio::{PinId, PinMode, ValidPinMode, PushPullOutput, DynPin, Pin},
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

    setup_led(
        pins.rgb_r.into_push_pull_output().as_dyn(),
        pins.rgb_g.into_push_pull_output().as_dyn(),
        pins.rgb_b.into_push_pull_output().as_dyn(),
    ).unwrap();

    loop {
        defmt::info!("tick");
        delay.start(1_u32.seconds());
        nb::block!(delay.wait());
    }
}

fn setup_led<P, E>(mut r: P, mut g: P, mut b: P) -> Result<(), E>
where
    P: OutputPin<Error = E>
{
    r.set_low()?;
    g.set_high()?;
    b.set_high()?;
    Ok(())
}
