#![no_std]
#![no_main]

use stm32f4::stm32f407;

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

fn get_random_data(rng: &stm32f407::RNG) -> u32 {
    loop {
        let seis_error = rng.sr.read().seis().bits();
        let ceis_error = rng.sr.read().ceis().bits();
        let error_detected = seis_error || ceis_error;
        if seis_error {
            hprintln!("seis error detected").unwrap();
        }
        if ceis_error {
            hprintln!("ceis error detected").unwrap();
        }

        let data_ready = rng.sr.read().drdy().bits();
        if !error_detected && data_ready {
            break rng.dr.read().bits()
        }
    }
}

fn reset_rng(rcc: &stm32f407::RCC) {
    rcc.ahb2rstr.modify(|_, w| w.rngrst().set_bit());
    rcc.ahb2rstr.modify(|_, w| w.rngrst().clear_bit());
}

fn set_pll(rcc: &stm32f407::RCC) {
    // set pll as microcontroller output 2
    rcc.cfgr.modify(|_, w| w.mco2().pll());
    // enable pll
    rcc.cr.modify(|_, w| w .pllon().set_bit());

    while !rcc.cr.read().pllrdy().is_ready() {

    }

    hprintln!("pll ready").unwrap();
}

#[entry]
fn main() -> ! {
    let peripherals = stm32f407::Peripherals::take().unwrap();

    let rcc = peripherals.RCC;
    set_pll(&rcc);

    // enable rng peripheral
    rcc.ahb2enr.modify(|_, w| w.rngen().set_bit());
    // give RNG_CLK time to start
    let _ = rcc.ahb2enr.read().rngen().is_enabled();

    reset_rng(&rcc);

    let rng = peripherals.RNG;
    // at the beginning data is not ready
    assert_eq!(false, rng.sr.read().drdy().bits());

    // enable random number generation
    rng.cr.modify(|_, w| w.rngen().set_bit());

    for i in 0..200 {
        let data = get_random_data(&rng);
        hprintln!("step {}: {}", i, data).unwrap();
    }

    loop {
    }
}
