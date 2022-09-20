use super::AtomicWaitInternal;
use windows_sys::Win32::System::{
    Threading::{WaitOnAddress, WakeByAddressAll, WakeByAddressSingle},
    WindowsProgramming::INFINITE,
};

#[inline]
pub(crate) fn wait<T: AtomicWaitInternal>(a: &T, expected: T::Value) -> T::Value
where
    T::Value: Copy,
{
    let size = core::mem::size_of::<T>();
    let ptr: *const T = a;
    let expected_ptr: *const T::Value = &expected;
    loop {
        if let Err(current) = a.check_value(expected) {
            return current;
        }
        unsafe { WaitOnAddress(ptr.cast(), expected_ptr.cast(), size, INFINITE) };
    }
}

#[inline]
pub fn wake_one(ptr: *const ()) {
    unsafe { WakeByAddressSingle(ptr.cast()) };
}

#[inline]
pub fn wake_all(ptr: *const ()) {
    unsafe { WakeByAddressAll(ptr.cast()) };
}
