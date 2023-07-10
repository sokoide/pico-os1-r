//! Blinks the LED on a Pico board
#![no_std]
#![no_main]

use bsp::entry;
use cortex_m_rt::exception;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;

use volatile_register::{RO, RW};

static mut COUNTER: u32 = 0;
const TIMER_PERIOD: u32 = 10;
const TMCLK_KHZ: u32 = 125 * 1000;

// const SYST_COUNTER_MASK: u32 = 0x00ff_ffff;
const SYST_CSR_ENABLE: u32 = 1 << 0;
const SYST_CSR_TICKINT: u32 = 1 << 1;
const SYST_CSR_CLKSOURCE: u32 = 1 << 2;
const SYST_CSR_COUNTFLAG: u32 = 1 << 16;

pub struct SystemTimer {
    p: &'static mut RegisterBlock,
}

#[repr(C)]
struct RegisterBlock {
    pub csr: RW<u32>,
    pub rvr: RW<u32>,
    pub cvr: RW<u32>,
    pub calib: RO<u32>,
}

impl Default for SystemTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemTimer {
    pub fn new() -> SystemTimer {
        SystemTimer {
            p: unsafe { &mut *(0xE000_E010 as *mut RegisterBlock) },
        }
    }

    pub fn init(&mut self) {
        unsafe {
            // Stop SysTick
            self.p.csr.write(SYST_CSR_CLKSOURCE | SYST_CSR_TICKINT);
            // Set reload
            self.p.rvr.write(TIMER_PERIOD * TMCLK_KHZ - 1);
            // Set counter
            self.p.cvr.write(TIMER_PERIOD * TMCLK_KHZ - 1);
            // Start SysTick
            self.p
                .csr
                .write(SYST_CSR_CLKSOURCE | SYST_CSR_TICKINT | SYST_CSR_ENABLE);
        }
    }

    #[inline]
    pub fn clear_current(&mut self) {
        unsafe { self.p.cvr.write(0) }
    }

    #[inline]
    pub fn enable_counter(&mut self) {
        unsafe { self.p.csr.modify(|v| v | SYST_CSR_ENABLE) }
    }

    #[inline]
    pub fn enable_interrupt(&mut self) {
        unsafe { self.p.csr.modify(|v| v | SYST_CSR_TICKINT) }
    }

    #[inline]
    pub fn disable_counter(&mut self) {
        unsafe { self.p.csr.modify(|v| v & !SYST_CSR_ENABLE) }
    }

    #[inline]
    pub fn disable_interrupt(&mut self) {
        unsafe { self.p.csr.modify(|v| v & !SYST_CSR_TICKINT) }
    }

    #[inline]
    pub fn get_csr(&self) -> u32 {
        self.p.csr.read()
    }

    #[inline]
    pub fn set_reload(&mut self, reload_value: u32) {
        unsafe { self.p.rvr.write(reload_value) }
    }

    #[inline]
    pub fn has_wrapped(&mut self) -> bool {
        self.p.csr.read() & SYST_CSR_COUNTFLAG != 0
    }
}

fn delay_ms(ms: u32) {
    let mut counter = ms / TIMER_PERIOD;
    let mut st = SystemTimer::new();
    while counter > 0 {
        if st.has_wrapped() {
            counter -= 1;
        }
    }
}

#[entry]
fn main() -> ! {
    info!("Program start");

    let mut st = SystemTimer::new();
    st.init();

    loop {
        info!("1");
        print_counter();
        delay_ms(500);

        info!("2");
        print_counter();
        delay_ms(500);
    }
}

fn print_counter() {
    unsafe {
        info!("unsafe counter: {}", COUNTER);
    }
}

#[exception]
fn SysTick() {
    unsafe {
        if COUNTER == 0xFFFF_FFFF {
            COUNTER = 0;
        } else {
            COUNTER += 1;
        }
    }
}

#[exception]
unsafe fn DefaultHandler(_irqn: i16) {}
