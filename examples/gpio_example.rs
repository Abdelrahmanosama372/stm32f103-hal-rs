#![no_std]
#![no_main]

use stm32f103_hal::{entry, peripherals};
use peripherals::gpio::PinSpeed;

entry!(main);
fn main() -> ! {

    // Get peripherals
    let dp = peripherals::Peripherals::take();
   
    // get Rcc 
    let rcc = dp.rcc;

    // enable GpioA clock
    rcc.enable_gpioa();

    // get GpioA
    let gpioa = dp.gpioa;

    // Configure PA1 as output
    let pa1 = gpioa.p1().into_output_pushpull(PinSpeed::Speed2Hz);

    // Toggle the LED on PA1
    loop {
        pa1.set();
        delay(100_000);
        pa1.reset();
        delay(100_000);
    }
}

// Simple delay function
fn delay(cycles: u32) {
    for _ in 0..cycles {
        // do nothing
    }
}

