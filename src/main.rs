#![no_std]
#![no_main]

use panic_halt as _;

use embedded_hal::prelude::*;
use attiny_hal as hal;

#[avr_device::entry]
fn main() -> ! {
    let dp = attiny_hal::Peripherals::take().unwrap();
    let pins = attiny_hal::pins!(dp);

    let mut delay = crate::hal::delay::Delay::<hal::clock::MHz1>::new();

    let mut led = pins.pd5.into_output();
    
    loop {
        led.toggle();
        delay.delay_ms(1000u16);
    }
}
