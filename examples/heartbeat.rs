//! Heartbeat blinking pattern

#![no_std]

#[macro_use]
extern crate photon;
extern crate photon_hal as hal;

use hal::{D7, PinMode};
use photon::App;

app! {
    setup: setup,
    loop: loop_,
}

fn setup(_: App) {
    D7.pin_mode(PinMode::Output);
}

fn loop_(ref mut app: App) {
    D7.high();
    app.delay_ms(200);

    D7.low();
    app.delay_ms(50);

    D7.high();
    app.delay_ms(250);

    D7.low();
    app.delay_ms(500);
}
