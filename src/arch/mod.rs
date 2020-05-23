
//Namespace the arch dir.
#[cfg(target_arch = "riscv")]
#[macro_use]
pub mod riscv;
#[cfg(target_arch = "riscv")]
pub use self::riscv::*;
