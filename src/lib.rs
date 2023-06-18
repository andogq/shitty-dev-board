#![no_std]
pub extern crate stm32f0xx_hal as hal;

use core::ops::{Deref, DerefMut};

use hal::{gpio::Pin, prelude::*};
use stm32f0xx_hal::gpio::{
    gpiob::{PB12, PB13, PB14, PB15},
    Output, PushPull,
};

pub type Led0 = PB12<Output<PushPull>>;
pub type Led1 = PB13<Output<PushPull>>;
pub type Led2 = PB14<Output<PushPull>>;
pub type Led3 = PB15<Output<PushPull>>;
pub type Led = Pin<Output<PushPull>>;

pub struct Leds {
    leds: [Led; 4],
}
impl Leds {
    pub fn new(led0: Led0, led1: Led1, led2: Led2, led3: Led3) -> Self {
        Self {
            leds: [
                led0.downgrade(),
                led1.downgrade(),
                led2.downgrade(),
                led3.downgrade(),
            ],
        }
    }

    pub fn toggle_all(&mut self) {
        self.leds.iter_mut().for_each(|led| {
            led.toggle().ok();
        });
    }

    pub fn all_high(&mut self) {
        self.leds.iter_mut().for_each(|led| {
            led.set_high().ok();
        });
    }

    pub fn all_low(&mut self) {
        self.leds.iter_mut().for_each(|led| {
            led.set_low().ok();
        });
    }
}

impl Deref for Leds {
    type Target = [Led];

    fn deref(&self) -> &Self::Target {
        &self.leds
    }
}

impl DerefMut for Leds {
    fn deref_mut(&mut self) -> &mut [Led] {
        &mut self.leds
    }
}
