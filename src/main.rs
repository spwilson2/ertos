#![feature(panic_info_message)]
#![feature(custom_test_frameworks)]
#![feature(llvm_asm)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(global_asm)]
#![no_std]
#![no_main]
#![test_runner(crate::test_runner)]

#[macro_use]
pub mod arch;

#[no_mangle]
extern "C" fn kinit() -> ! {
    let mut uart = arch::drivers::uart::Uart::new(0x10000000);
    uart.init();
    println!("Hello");

    panic!("bye");
    //arch::panic::abort();
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
