
//Namespace the arch dir.
#[cfg(target_arch = "riscv64")]
#[macro_use]
pub mod riscv64;
#[cfg(target_arch = "riscv64")]
pub use self::riscv64::*;
