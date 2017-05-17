//! Blinks the user LED

#![no_std]

#[macro_use]
extern crate photon;
extern crate photon_hal as hal;

use hal::{LED, PinMode};
use photon::App;

const PERIOD: u32 = 250; // ms

app! {
    setup: setup,
    loop: loop_,
}

fn setup(_: App) {
    LED.pin_mode(PinMode::Output);
}

fn loop_(ref mut app: App) {
    LED.low();
    app.delay_ms(PERIOD);
    LED.high();
    app.delay_ms(PERIOD);
}
