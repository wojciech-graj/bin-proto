#[cfg(feature = "std")]
use std::alloc::System;

#[cfg(feature = "alloc")]
use alloc::alloc::Global;

/// A trait for context types capable of supplying an owned instance of `T`.
pub trait HasOwned<T> {
    /// Returns an owned instance of `T`.
    fn get_owned(&self) -> T;
}

#[cfg(feature = "alloc")]
impl<T> HasOwned<Global> for T {
    fn get_owned(&self) -> Global {
        Global
    }
}

#[cfg(feature = "std")]
impl<T> HasOwned<System> for T {
    fn get_owned(&self) -> System {
        System
    }
}
