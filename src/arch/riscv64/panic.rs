use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {

    println!("panicked: {:?}", info.payload().downcast_ref::<&str>().unwrap());
    if let Some(location) = info.location() {
        println!("Panic occurred in '{}':{}", location.file(),
            location.line());
    } else {
        println!("Panic occurred but can't get location information...");
    }
    abort();
}

#[no_mangle]
extern "C"
fn abort() -> ! {
  loop {
    unsafe {
      llvm_asm!("wfi"::::"volatile");
    }
  }
}
