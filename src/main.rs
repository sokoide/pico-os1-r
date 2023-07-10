//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use bsp::entry;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::exception;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{pac, sio::Sio};

use volatile_register::{RO, RW};

#[repr(C)]
struct SysTick {
    pub csr: RW<u32>,
    pub rvr: RW<u32>,
    pub cvr: RW<u32>,
    pub calib: RO<u32>,
}

fn get_systick() -> &'static mut SysTick {
    unsafe { &mut *(0xE000_E010 as *mut SysTick) }
}

fn get_cvr() -> u32 {
    let systick = get_systick();
    systick.cvr.read()
}

fn get_csr() -> u32 {
    let systick = get_systick();
    systick.csr.read()
}

fn delay_ms(ms: u32) {
    let mut counter = ms / 10;
    while counter > 0 {
        // csr's bit 17 is set when it passes reload ticks
        if get_csr() & (1 << 16) != 0 {
            counter -= 1;
        }
    }
}

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // This is the correct pin on the Raspberry Pico board. On other boards, even if they have an
    // on-board LED, it might need to be changed.
    // Notably, on the Pico W, the LED is not connected to any of the RP2040 GPIOs but to the cyw43 module instead. If you have
    // a Pico W and want to toggle a LED with a simple GPIO output pin, you can connect an external
    // LED to one of the GPIO pins, and reference that pin here.
    let mut led_pin: bsp::hal::gpio::Pin<
        bsp::hal::gpio::bank0::Gpio25,
        bsp::hal::gpio::Output<bsp::hal::gpio::PushPull>,
    > = pins.led.into_push_pull_output();

    let mut syst = core.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(12_000_000u32 / 10);
    syst.clear_current();
    syst.enable_counter();
    // syst.enable_interrupt();

    loop {
        info!("on!");
        led_pin.set_high().unwrap();
        let mut t = get_cvr();
        info!(
            "counter: {}, {}, {}, {}",
            t,
            cortex_m::peripheral::SYST::get_reload(),
            cortex_m::peripheral::SYST::get_current(),
            cortex_m::peripheral::SYST::get_ticks_per_10ms()
        );

        // delay.delay_ms(500);
        delay_ms(1000);
        info!("off!");
        led_pin.set_low().unwrap();
        // delay.delay_ms(500);
        t = get_cvr();
        info!(
            "counter: {}, {}, {}, {}",
            t,
            cortex_m::peripheral::SYST::get_reload(),
            cortex_m::peripheral::SYST::get_current(),
            cortex_m::peripheral::SYST::get_ticks_per_10ms()
        );
        delay_ms(500);
    }
}

#[exception]
fn SysTick() {}

#[exception]
unsafe fn DefaultHandler(_irqn: i16) {}
