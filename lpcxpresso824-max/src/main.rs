#![feature(panic_handler)]


#![no_main]
#![no_std]


#[macro_use]
extern crate cortex_m_rt;
extern crate lpc82x_hal;


use core::panic::PanicInfo;

use lpc82x_hal::{
    prelude::*,
    gpio::direction,
    swm::{
        pin_state,
        Pin,
        PinTrait,
    },
    Peripherals,
};


entry!(main);

fn main() -> ! {
    let p = Peripherals::take().unwrap();

    let swm = p.SWM.split();

    // Initialize LEDs
    let red = swm.pins.pio0_12
        .into_gpio_pin(&p.GPIO)
        .into_output();
    let green = swm.pins.pio0_16
        .into_gpio_pin(&p.GPIO)
        .into_output();
    let blue = swm.pins.pio0_27
        .into_gpio_pin(&p.GPIO)
        .into_output();

    let mut red   = Led::red(red);
    let mut green = Led::green(green);
    let mut blue  = Led::blue(blue);

    let mut i = 0;

    loop {
        // This should really be using the hardware support to generate the PWM
        // waves, but we don't have an API for that in the HAL yet.
        red.update(i);
        green.update(i);
        blue.update(i);

        i = (i + 1) % PWM_CYCLE;
    }
}


const PWM_CYCLE: i32 = 50;


pub struct Led<'gpio, T> where T: PinTrait {
    factor    : i32,
    factor_max: i32,
    step      : i32,

    pin: Pin<T, pin_state::Gpio<'gpio, direction::Output>>,
}

impl<'gpio, T> Led<'gpio, T> where T: PinTrait {
    pub fn red(pin: Pin<T, pin_state::Gpio<'gpio, direction::Output>>) -> Self {
        Led {
            factor    : 10_000,
            factor_max: 30_000,
            step      : 1,

            pin,
        }
    }

    pub fn green(pin: Pin<T, pin_state::Gpio<'gpio, direction::Output>>)
        -> Self
    {
        Led {
            factor    : 15_000,
            factor_max: 40_000,
            step      : 1,

            pin,
        }
    }

    pub fn blue(pin: Pin<T, pin_state::Gpio<'gpio, direction::Output>>)
        -> Self
    {
        Led {
            factor    : 20_000,
            factor_max: 50_000,
            step      : 1,

            pin,
        }
    }

    pub fn update(&mut self, i: i32) {
        self.factor += self.step;
        if self.factor < 0 {
            self.factor = 0;
            self.step *= -1;
        }
        if self.factor > self.factor_max {
            self.factor = self.factor_max;
            self.step *= -1;
        }

        if i <= PWM_CYCLE * self.factor / self.factor_max {
            self.pin.set_low();
        }
        else {
            self.pin.set_high();
        }
    }
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
