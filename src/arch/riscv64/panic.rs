use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {

    if let Some(location) = info.location() {
        println!("Panic occurred in '{}':{}", location.file(), location.line());
    } else {
        println!("Panic occurred but can't get location information...");
    }

    if let Some(msg) = info.message() {
        println!("panic-message: {}", msg);
    }
    if let Some(payload) = info.payload().downcast_ref::<&str>() {
        println!("panic-payload: {:?}", payload);
    }

    abort();
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
  loop {
    unsafe {
      llvm_asm!("wfi"::::"volatile");
    }
  }
}
