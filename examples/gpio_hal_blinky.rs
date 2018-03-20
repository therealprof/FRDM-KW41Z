#![feature(used)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
extern crate mkw41z_hal as hal;

use hal::prelude::*;
use hal::mkw41z;
use hal::delay::Delay;

fn main() {
    if let (Some(cp), Some(p)) = (
        cortex_m::peripheral::Peripherals::take(),
        mkw41z::Peripherals::take(),
    ) {
        // Disable watchdog
        p.SIM.copc.write(|w| w.copt()._00());

        // Initialise default clocks
        let clocks = hal::clock::clocks();

        // Initialise delay driver
        let mut delay = Delay::new(cp.SYST, clocks);

        // Split GPIO pins
        let mut gpioa = p.GPIOA.split();
        let mut gpioc = p.GPIOC.split();

        // Setup GPIO pins for built-in RGB LED
        let mut rled = gpioc.ptc1.into_push_pull_output();
        let mut gled = gpioa.pta19.into_push_pull_output();
        let mut bled = gpioa.pta18.into_push_pull_output();

        // Switch LEDs on and off one after another
        loop {
            rled.set_low();
            delay.delay_ms(1_000_u16);
            rled.set_high();
            delay.delay_ms(100_u16);
            gled.set_low();
            delay.delay_ms(1_000_u16);
            gled.set_high();
            delay.delay_ms(100_u16);
            bled.set_low();
            delay.delay_ms(1_000_u16);
            bled.set_high();
            delay.delay_ms(100_u16);
        }
    }
}
