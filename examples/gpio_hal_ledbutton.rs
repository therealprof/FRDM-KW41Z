#![feature(used)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
extern crate mkw41z_hal as hal;
extern crate panic_abort;

use hal::delay::Delay;
use hal::mkw41z;
use hal::prelude::*;

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
        let gpioc = p.GPIOC.split();

        // Setup GPIO pin for red color of built-in RGB LED
        let mut rled = gpioc.ptc1.into_push_pull_output();

        // Configure button GPIOs for SW3 and SW4 as input
        let button3 = gpioc.ptc4.into_pull_up_input();
        let button4 = gpioc.ptc5.into_pull_up_input();

        loop {
            rled.set_high();

            if button3.is_low() {
                rled.set_low();
                delay.delay_ms(1_000_u16);
            }

            rled.set_high();
            if button4.is_low() {
                rled.set_low();
                delay.delay_ms(1_000_u16);
                delay.delay_ms(1_000_u16);
                delay.delay_ms(1_000_u16);
                delay.delay_ms(1_000_u16);
                delay.delay_ms(1_000_u16);
            }
        }
    }
}
