#![feature(llvm_asm)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() {

}

// These functions and traits are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"] extern fn eh_personality() {}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop{} }
