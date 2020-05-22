
//Namespace the arch dir.
#[cfg(target_arch = "arm")]
#[macro_use]
pub mod armv7a;
#[cfg(target_arch = "arm")]
pub use self::armv7a::*;
