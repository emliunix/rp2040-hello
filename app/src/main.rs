//! Rainbow effect color wheel using the onboard NeoPixel on an Adafruit Feather RP2040 board
//!
//! This flows smoothly through various colors on the onboard NeoPixel.
//! Uses the `ws2812_pio` driver to control the NeoPixel, which in turns uses the
//! RP2040's PIO block.
#![no_std]
#![no_main]

use xiao2040_bsp::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        pio::PIOExt,
        timer::Timer,
        watchdog::Watchdog,
        Sio, gpio::OutputDriveStrength,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};
use core::iter::once;
use cortex_m_rt::entry;
use embedded_hal::{timer::CountDown, digital::v2::OutputPin};
use embedded_time::duration::Extensions;
use panic_halt as _;
use smart_leds::{brightness, SmartLedsWrite, RGB8};
use ws2812_pio::Ws2812;
use defmt;
use defmt_rtt as _;

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

    let mut rgb_r = pins.rgb_r.into_push_pull_output();
    let mut rgb_g = pins.rgb_g.into_push_pull_output();
    let mut rgb_b = pins.rgb_b.into_push_pull_output();

    rgb_r.set_low().unwrap();
    rgb_g.set_high().unwrap();
    rgb_b.set_high().unwrap();

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut delay = timer.count_down();

    // let mut i = 0;
    // loop {
    //     if i % 2 == 0 {
    //         rgb_r.set_high().unwrap();
    //     } else {
    //         rgb_r.set_low().unwrap();
    //     }
    //     defmt::info!("tick {}", i);
    //     i += 1;
    //     delay.start(500.milliseconds());
    //     let _ = nb::block!(delay.wait());
    // }

    let mut neopwr = pins.neopwr.into_push_pull_output();
    neopwr.set_drive_strength(OutputDriveStrength::TwoMilliAmps);
    neopwr.set_high().unwrap();

    // Configure the addressable LED
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let mut ws = Ws2812::new(
        // The onboard NeoPixel is attached to GPIO pin #16 on the Feather RP2040.
        pins.neopix.into_mode(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    // // Infinite colour wheel loop
    let mut n: u16 = 65535 / 2;
    loop {
        ws.write(brightness(once(wheel(n)), 32)).unwrap();
        n = n.wrapping_add(1);

        delay.start(1.milliseconds());
        let _ = nb::block!(delay.wait());
    }
}

/// Convert a number from `0..=255` to an RGB color triplet.
///
/// The colours are a transition from red, to green, to blue and back to red.
fn wheel(mut wheel_pos: u16) -> RGB8 {
    if wheel_pos < 21845 {
        // No green in this sector - red and blue only
        (((21845 - wheel_pos) / 86) as u8, 0, (wheel_pos / 86) as u8).into()
    } else if wheel_pos < 43690 {
        // No red in this sector - green and blue only
        wheel_pos -= 21845;
        (0, (wheel_pos / 86) as u8, ((21845 - wheel_pos) / 86) as u8).into()
    } else {
        // No blue in this sector - red and green only
        wheel_pos -= 43690;
        ((wheel_pos / 86) as u8, 255 - ((21845 - wheel_pos) / 86) as u8, 0).into()
    }
}