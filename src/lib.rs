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

#[cfg(target_os = "freebsd")]
#[path = "freebsd.rs"]
mod platform;

/// If the value is `value`, wait until woken up.
///
/// This function might also return spuriously,
/// without a corresponding wake operation.
#[inline]
pub fn wait(atomic: &AtomicU32, value: u32) {
    platform::wait(atomic, value)
}

/// Wake one thread that is waiting on this atomic.
///
/// It's okay if the pointer dangles or is null.
#[inline]
pub fn wake_one(atomic: *const AtomicU32) {
    platform::wake_one(atomic);
}

/// Wake up to n threads that are waiting on this atomic.
///
/// It's okay if the pointer dangles or is null.
///
/// ______________________
/// **Remarks:**
///
/// On platforms where this is not _natively_ supported by the kernel,
/// it is emulated with a loop to wake_one. Don't use excessively
/// large `wake_count` if you're targeting Windows or OSX, or this
/// loop will be too costly.
///
/// This is an optimization for small `wake_count`s, like 2-10, on
/// linux and freebsd.
/// If you're unsure, just use `wake_all()` or `wake_one()` which are
/// universally supported.
#[inline]
pub fn wake_n(atomic: *const AtomicU32, wake_count: u32) {
    platform::wake_n(atomic, wake_count);
}

/// Wake all threads that are waiting on this atomic.
///
/// It's okay if the pointer dangles or is null.
#[inline]
pub fn wake_all(atomic: *const AtomicU32) {
    platform::wake_all(atomic);
}
