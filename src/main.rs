#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m::interrupt;
use cortex_m_rt::entry;
use panic_halt as _;
use shitty_dev_board_bringup::{
    hal::{pac, prelude::*, rcc::HSEBypassMode, timers},
    Leds, Usart,
};

enum Mode {
    Toggle,
    Index(usize),
}

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let mut rcc = peripherals
        .RCC
        .configure()
        // .sysclk(8.mhz())
        .hse(16.mhz(), HSEBypassMode::NotBypassed)
        .freeze(&mut peripherals.FLASH);

    let gpioa = peripherals.GPIOA.split(&mut rcc);
    let gpiob = peripherals.GPIOB.split(&mut rcc);

    // Initialise serial port
    let (pa9, pa10) = interrupt::free(|cs| {
        (
            gpioa.pa9.into_alternate_af1(cs),
            gpioa.pa10.into_alternate_af1(cs),
        )
    });
    let mut usart = Usart::new(peripherals.USART1, pa9, pa10, 9600.bps(), &mut rcc);

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

    let mut mode = Mode::Toggle;

    loop {
        // Receive a command from USART
        if let Ok(command) = usart.read() {
            match command {
                b't' => mode = Mode::Toggle,
                b'0'..=b'3' => mode = Mode::Index((command - b'0') as usize),
                _ => (),
            }

            leds.all_low();
        }

        match mode {
            Mode::Toggle => {
                // Flash LED
                if timer.wait().is_ok() {
                    usart.write_str("toggling\n").ok();
                    leds.toggle_all();
                }
            }
            Mode::Index(i) => {
                leds.all_low();
                leds[i].set_high().ok();
            }
        }
    }
}
