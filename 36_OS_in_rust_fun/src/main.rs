//step 1
#![no_std] //we can't use std libs in os, so we deactive it
#![no_main]


use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}


//step 2
//in cargo build after removing std libs,
//compiler needs panc_handler function + language item
//this step is for panic fn
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { // here ! means never return anything
    loop {}
}

fn main() {}
