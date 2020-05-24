#![feature(llvm_asm)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(global_asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[macro_use]
pub mod arch;

#[no_mangle]
extern "C" fn kinit() -> ! {
  loop {}
}

#[no_mangle]
extern "C" fn kinit_hart(_hartid: usize) {
    // We aren't going to do anything here until we get SMP going.
    // All non-0 harts initialize here.
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
