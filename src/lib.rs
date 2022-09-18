#![no_std]
#![doc = include_str!("../README.md")]

use core::sync::atomic::AtomicU32;

#[cfg(any(target_os = "linux", target_os = "android"))]
#[path = "linux.rs"]
mod platform;

#[cfg(any(target_os = "macos", target_os = "ios", target_os = "watchos"))]
#[path = "macos.rs"]
mod platform;

#[cfg(windows)]
#[path = "windows.rs"]
mod platform;

pub trait AtomicWait {
    /// The type of value stored in this atomic (`u32` for `AtomicU32`).
    type Value;

    /// While the value is `value`, wait until woken up.
    ///
    /// Returns the new value,
    /// which is guaranteed to be different than `value`.
    fn wait(&self, value: Self::Value) -> Self::Value;

    /// Wake one thread that's waiting on this atomic.
    fn wake_one(&self);

    /// Wake all thread that's waiting on this atomic.
    fn wake_all(&self);
}

impl AtomicWait for AtomicU32 {
    type Value = u32;

    #[inline]
    fn wait(&self, value: u32) -> u32 {
        platform::wait(self, value)
    }

    #[inline]
    fn wake_one(&self) {
        platform::wake_one(self);
    }

    #[inline]
    fn wake_all(&self) {
        platform::wake_all(self);
    }
}
