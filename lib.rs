#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
pub use f3::hal::stm32f30x::{gpioc, rcc};

use f3::hal::stm32f30x::{self, GPIOD, RCC};

pub fn init() -> (&'static gpioc::RegisterBlock, &'static rcc::RegisterBlock,ITM) {
    // restrict access to the other peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();
    unsafe { (&*GPIOD::ptr(), &*RCC::ptr(),cp.ITM) }
}
