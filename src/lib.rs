#![no_std]
pub extern crate stm32f0xx_hal as hal;

use core::ops::{Deref, DerefMut};

use hal::{
    gpio::{
        gpioa::{PA10, PA9},
        gpiob::{PB12, PB13, PB14, PB15},
        Alternate, Output, Pin, PushPull, AF1,
    },
    pac::USART1,
    prelude::*,
    rcc::Rcc,
    serial::Serial,
    time::Bps,
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
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.leds
    }
}

type Usart1 = Serial<USART1, PA9<Alternate<AF1>>, PA10<Alternate<AF1>>>;
pub struct Usart(Usart1);
impl Usart {
    pub fn new(
        usart1: USART1,
        pa9: PA9<Alternate<AF1>>,
        pa10: PA10<Alternate<AF1>>,
        baud_rate: Bps,
        rcc: &mut Rcc,
    ) -> Self {
        Self(Serial::usart1(usart1, (pa9, pa10), baud_rate, rcc))
    }
}

impl Deref for Usart {
    type Target = Usart1;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Usart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
