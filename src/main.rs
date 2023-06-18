#![no_std]
#![no_main]

use cortex_m::{asm, interrupt};
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use shitty_dev_board_bringup::Leds;
use stm32f0xx_hal::{pac, prelude::*, rcc::HSEBypassMode, timers};

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let mut rcc = peripherals
        .RCC
        .configure()
        // .sysclk(8.mhz())
        .hse(16.mhz(), HSEBypassMode::NotBypassed)
        .freeze(&mut peripherals.FLASH);

    let gpiob = peripherals.GPIOB.split(&mut rcc);

    // Initialise and start the timer
    let mut timer = timers::Timer::tim1(peripherals.TIM1, 1.hz(), &mut rcc);

    // Configure PB12 as output
    let mut leds = interrupt::free(|cs| {
        Leds::new(
            gpiob.pb12.into_push_pull_output(cs),
            gpiob.pb13.into_push_pull_output(cs),
            gpiob.pb14.into_push_pull_output(cs),
            gpiob.pb15.into_push_pull_output(cs),
        )
    });

    loop {
        // Flash LED
        if timer.wait().is_ok() {
            leds.toggle_all();
        }
    }
}
