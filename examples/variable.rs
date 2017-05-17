//! Example for the Particle.variable API

#![feature(const_fn)]
#![no_std]

#[macro_use]
extern crate photon;
extern crate photon_hal as hal;

use core::cell::Cell;

use hal::{LED, PinMode, cloud};
use photon::{App, Resource};

app! {
    setup: setup,
    loop: loop_,
}

const PERIOD: u32 = 1_000;
static COUNT: Resource<Cell<i32>> = Resource::new(Cell::new(0));

fn setup(ref app: App) {
    LED.pin_mode(PinMode::Output);

    if cloud::variable("count", COUNT.access(app)).is_ok() {
        LED.high()
    } else {
        LED.low()
    }
}

fn loop_(ref mut app: App) {
    LED.high();
    app.delay_ms(PERIOD / 2);

    LED.low();
    app.delay_ms(PERIOD / 2);

    let count = COUNT.access(app);
    count.set(count.get() + 1);
}
