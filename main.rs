#![no_std]
#![no_main]
#[allow(unsafe_code)]

use core::ptr;
use aux8::{entry, iprint, iprintln};
#[entry]
fn main () -> ! {
    
    let (gpiod,rcc,mut itm) = aux8::init();
    // const GPIOD_ODR: u32 = 0x4800_0C14;
    // let y = GPIOD_ODR as *const u16;
    // To enable gpioe peripheral
    rcc.ahbenr.modify(|_,w| w.iopden().set_bit());

    gpiod.moder.modify(|_,w| {
        w.moder0().input();
        w.moder9().output()
    });

    
    loop{
            while gpiod.idr.read().idr0().bit_is_set() {}
            while gpiod.idr.read().idr0().bit_is_clear() {}
            gpiod.odr.write(|w| w.odr9().set_bit());
            // for x in 0..10 {}
            while gpiod.idr.read().idr0().bit_is_set() {}
            while gpiod.idr.read().idr0().bit_is_clear() {}
            gpiod.odr.write(|w| w.odr9().clear_bit());
            
        
        // iprintln!(&mut itm.stim[0],"ODR = 0x{:04x}",ptr::read_volatile(y));
        }
