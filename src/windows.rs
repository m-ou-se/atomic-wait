use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
use windows_sys::Win32::System::{
    Threading::{WaitOnAddress, WakeByAddressAll, WakeByAddressSingle},
    WindowsProgramming::INFINITE,
};

#[inline]
pub fn wait(a: &AtomicU32, expected: u32) -> u32 {
    let ptr: *const AtomicU32 = a;
    let expected_ptr: *const u32 = &expected;
    loop {
        let current = a.load(Relaxed);
        if current != expected {
            return current;
        }
        unsafe { WaitOnAddress(ptr.cast(), expected_ptr.cast(), 4, INFINITE) };
    }
}

#[inline]
pub fn wake_one(a: &AtomicU32) {
    let ptr: *const AtomicU32 = a;
    unsafe { WakeByAddressSingle(ptr.cast()) };
}

#[inline]
pub fn wake_all(a: &AtomicU32) {
    let ptr: *const AtomicU32 = a;
    unsafe { WakeByAddressAll(ptr.cast()) };
}
