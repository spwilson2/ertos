// trap.rs
// Trap routines
// Stephen Marz
// 10 October 2019

pub use super::cpu::{TrapFrame};

#[no_mangle]
/// The m_trap stands for "machine trap". Right now, we are handling
/// all traps at machine mode. In this mode, we can figure out what's
/// going on and send a trap where it needs to be. Remember, in machine
/// mode and in this trap, interrupts are disabled and the MMU is off.
extern "C" fn m_trap(
    _epc: usize,
    _tval: usize,
    _cause: usize,
    _hart: usize,
    _status: usize,
    _frame: *mut TrapFrame,
) -> usize {
  return 0;
}
