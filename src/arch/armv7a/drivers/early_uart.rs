#![allow(dead_code)]

const UDR: isize = 0x00;
const UFR: isize = 0x18;

const TXFE: u8 = 0x80;
const RXFF: u8 = 0x40;
const TXFF: u8 = 0x20;
const RXFE: u8 = 0x10;
const BUSY: u8 = 0x08;

use core::ptr::read_volatile;
use core::ptr::write_volatile;

pub struct Uart {
  base: *mut u8,
}

pub static mut UARTS: [Uart; 4] = [
  Uart {base: 0x101F1000 as *mut u8},
  Uart {base: 0x101F2000 as *mut u8},
  Uart {base: 0x101F3000 as *mut u8},
  Uart {base: 0x101F4000 as *mut u8},
];

impl Uart {
  pub fn getc(&mut self) -> char {
    self.getu8() as char
  }
  pub fn getu8(&mut self) -> u8 {
    loop {
      unsafe {
        if read_volatile(self.base.offset(UFR)) & RXFE == 0 {
          return read_volatile(self.base.offset(UDR));
        }
      }
    }
  }

  pub fn put(&mut self, data: u8) {
    loop {
      unsafe {
        if read_volatile(self.base.offset(UFR))  & TXFF == 0 {
          write_volatile(self.base.offset(UDR), data);
          return;
        }
      }
    }
  }

  pub fn puts(&mut self, data: &str) {
    for c in data.as_bytes() {
      self.put(*c);
    }
  }
}
