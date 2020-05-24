#![feature(custom_test_frameworks)]
#![feature(llvm_asm)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(global_asm)]
#![no_std]
#![no_main]

#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;

#[macro_use]
pub mod arch;

#[no_mangle]
extern "C" fn kinit() -> ! {
  loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
