#![no_std]
pub mod layout;
pub mod page;

pub mod zone;
pub use zone::Zone;

pub mod locked_zone;
pub use locked_zone::LockedZone;

#[cfg(not(feature = "with_std"))]
pub mod nostdlib;
#[cfg(not(feature = "with_std"))]
pub use nostdlib::*;

#[cfg(feature = "with_std")]
pub mod stdlib;
#[cfg(feature = "with_std")]
pub use stdlib::*;
