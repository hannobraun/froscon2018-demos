#![feature(panic_handler)]


#![no_main]
#![no_std]


#[macro_use] extern crate cortex_m_rt;

extern crate cortex_m;
extern crate stm32f0;


use core::panic::PanicInfo;

use cortex_m::asm;
use stm32f0::stm32f0x2::{
    CorePeripherals,
    Peripherals,
};


entry!(main);

fn main() -> ! {
    let p = Peripherals::take().unwrap();

    // Enable peripheral clocks
    p.RCC.ahbenr.modify(|_, w|
        w
            .iopaen().set_bit()
            .iopcen().set_bit()
    );

    // Set up PA0 ("USER" button) for input
    p.GPIOA.moder.modify(|_, w| w.moder0().input());

    // Set up PC7 (blue LED) for output
    // Set up LEDs for output
    p.GPIOC.moder.modify(|_, w| w.moder6().output()); // PC6 - red    - up
    p.GPIOC.moder.modify(|_, w| w.moder8().output()); // PC8 - orange - left
    p.GPIOC.moder.modify(|_, w| w.moder7().output()); // PC7 - blue   - down
    p.GPIOC.moder.modify(|_, w| w.moder9().output()); // PC9 - green  - right

    let mut delay;

    loop {
        // Is button pressed?
        if p.GPIOA.idr.read().idr0().is_high() {
            delay = 50_000;
        }
        else {
            delay = 100_000;
        }

        p.GPIOC.bsrr.write(|w| w.bs6().set_bit()); // enable red
        sleep(delay);
        p.GPIOC.bsrr.write(|w| w.br6().set_bit()); // disable red

        p.GPIOC.bsrr.write(|w| w.bs8().set_bit()); // enable orange
        sleep(delay);
        p.GPIOC.bsrr.write(|w| w.br8().set_bit()); // disable orange

        p.GPIOC.bsrr.write(|w| w.bs7().set_bit()); // enable blue
        sleep(delay);
        p.GPIOC.bsrr.write(|w| w.br7().set_bit()); // disable blue

        p.GPIOC.bsrr.write(|w| w.bs9().set_bit()); // enable green
        sleep(delay);
        p.GPIOC.bsrr.write(|w| w.br9().set_bit()); // disable green
    }
}

fn sleep(delay: u32) {
    for _ in 0..delay {
        asm::nop();
    }
}


#[panic_handler]
#[no_mangle]
pub fn panic(_: &PanicInfo) -> ! {
    // We're about to reset the system, so it won't matter that what we're going
    // to do might conflict with other code using the peripherals.
    let mut cp = unsafe { CorePeripherals::steal() };

    cp.SCB.system_reset()
}
