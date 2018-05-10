#![feature(used)]
#![feature(const_fn)]
#![no_std]

extern crate mkw41z_hal as hal;
extern crate panic_abort;

use hal::mkw41z;
use hal::prelude::*;

extern crate cortex_m;

fn main() {
    if let Some(p) = mkw41z::Peripherals::take() {
        // Disable watchdog
        p.SIM.copc.write(|w| w.copt()._00());

        // Initialise default clocks
        let _ = hal::clock::clocks();

        // Split GPIO pins
        let mut gpioa = p.GPIOA.split();

        // Setup GPIO pins for built-in blue color of RGB LED
        let mut led = gpioa.pta18.into_push_pull_output();

        // Blink blue LED
        loop {
            led.set_low();
            for _ in 0..1_000_000 {
                cortex_m::asm::nop();
            }
            led.set_high();
            for _ in 0..1_000_000 {
                cortex_m::asm::nop();
            }
        }
    }
}
