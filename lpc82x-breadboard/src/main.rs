#![feature(panic_handler)]


#![no_main]
#![no_std]


#[macro_use]
extern crate cortex_m_rt;
extern crate lpc82x_hal;


use core::panic::PanicInfo;

use lpc82x_hal::{
    prelude::*,
    clock::Ticks,
    gpio::direction,
    sleep,
    swm::{
        pin_state,
        Pin,
        PinTrait,
    },
    syscon::IrcDerivedClock,
    Peripherals,
};


entry!(main);

fn main() -> ! {
    // Create the struct we're going to use to access all the peripherals. This
    // is unsafe, because we're only allowed to create one instance.
    let p = Peripherals::take().unwrap();

    // Other peripherals need to be initialized. Trying to use the API before
    // initializing them will actually lead to compile-time errors.
    let mut swm    = p.SWM.split();
    let mut syscon = p.SYSCON.split();
    let mut wkt    = p.WKT.enable(&mut syscon.handle);

    // We're going to need a clock for sleeping. Let's use the IRC-derived clock
    // that runs at 750 kHz.
    let clock = syscon.irc_derived_clock;

    // Configure PIO0_3. It has the fixed function SWCLK assigned to it by
    // default. We need to unassign it, before we can use it for something else.
    let (_, pio0_3) = swm.fixed_functions.swclk.unassign(
        swm.pins.pio0_3,
        &mut swm.handle,
    );

    // Now we can use PIO0_3 for general-purpose I/O.
    let mut pio0_3 = pio0_3
        .into_unused_pin()
        .into_gpio_pin(&p.GPIO)
        .into_output();

    // Since this is a simple example, we don't want to deal with interrupts
    // here. Let's just use busy waiting as a sleeping strategy.
    let mut sleep = sleep::Busy::prepare(&mut wkt);

    // Blink the LED
    loop {
        blink(
            37_000,
            712_500,
            &clock,
            &mut pio0_3,
            &mut sleep,
        );
    }
}


fn blink<'gpio, P: PinTrait>(
    high : u32,
    low  : u32,
    clock: &IrcDerivedClock,
    led  : &mut Pin<P, pin_state::Gpio<'gpio, direction::Output>>,
    sleep: &mut sleep::Busy,
) {
    let high = Ticks { value: high, clock: clock };
    let low  = Ticks { value: low , clock: clock };

    led.set_high();
    sleep.sleep(high);
    led.set_low();
    sleep.sleep(low);
}


#[panic_handler]
#[no_mangle]
pub fn panic(_: &PanicInfo) -> ! {
    // We're about to reset the system, so it won't matter that what we're going
    // to do might conflict with other code using the peripherals. In addition,
    // we're careful not to do anything in this function that relies on the
    // initial type state of the peripherals in a way that could conflict with
    // the actual peripheral state.
    let mut p = unsafe { Peripherals::steal() };

    p.SCB.system_reset()
}
