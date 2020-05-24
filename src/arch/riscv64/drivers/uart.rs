// Referenced material:
// - reference/open-core-NS16550.pdf
// - http://docplayer.net/21367798-Uart-ip-core-specification-author-jacob-gorban-gorban-opencores-org.html

const RBR: usize = 0x00;
const THR: usize = 0x00;

const FCR: usize = 0x02;
const LCR: usize = 0x03;
const LSR: usize = 0x05;

const LSR_DR: u8 = 0b1;

const DLL: usize = 0x00;
const DLH: usize = 0x00;

use core::fmt::{Error, Write};

pub struct Uart {
    base_address: usize,
}

impl Write for Uart {
    fn write_str(&mut self, out: &str) -> Result<(), Error> {
        for c in out.bytes() {
            self.put(c);
        }
        Ok(())
    }
}

impl Uart {
    pub fn new(base_address: usize) -> Self {
        Uart { base_address }
    }

    pub fn init(&mut self) {
        let ptr = self.base_address as *mut u8;
        unsafe {
            // Turn on the divisor latch selector
            let lcr = ptr.add(LCR).read_volatile();
            ptr.add(LCR).write_volatile(lcr | 0x1 << 7);

            // Set baud to 115200
            // clock-frequency device tree: 0x384000
            //
            // 115200 = clock-freq / divisor
            // divisor = 0x384000 / 115200
            ptr.add(DLL).write_volatile(32);
            ptr.add(DLH).write_volatile(0);

            // De-select the DLH/DLL latches
            // Set data bits to 8
            ptr.add(LCR).write_volatile(0b11);

            // Enable FIFO
            ptr.add(FCR).write_volatile(1 << 0);

            // NOTE: Interrupts not enabled.
        }
    }

    pub fn put(&mut self, c: u8) {
        let ptr = self.base_address as *mut u8;
        unsafe {
            ptr.add(THR).write_volatile(c);
        }
    }

    pub fn get(&mut self) -> Option<u8> {
        let ptr = self.base_address as *mut u8;
        unsafe {
            if ptr.add(LSR).read_volatile() & LSR_DR == 0 {
                // The DR bit is 0, meaning no data
                None
            } else {
                // The DR bit is 1, meaning data!
                Some(ptr.add(RBR).read_volatile())
            }
        }
    }
}
