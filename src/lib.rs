#![crate_type="staticlib"]

#![feature(llvm_asm)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![no_std]

use core::panic::PanicInfo;

#[macro_use]
mod arch;
use crate::arch::*;

#[no_mangle]
pub extern "C" fn main() -> !{
  loop {
  }
}

// These functions and traits are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"] extern fn eh_personality() {}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop{} }




// Fix llbv landing pads.
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1 () -> ! {loop{}}
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0 () -> ! {loop{}}
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn _Unwind_Resume() -> ! {loop{}}
