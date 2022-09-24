use core::{
    ffi::c_void,
    sync::atomic::{AtomicU32, Ordering::Relaxed},
};

#[inline]
pub fn wait(a: &AtomicU32, expected: u32) -> u32 {
    loop {
        let current = a.load(Relaxed);
        if current != expected {
            return current;
        }
        unsafe {
            libc::syscall(
                libc::SYS_futex,
                a,
                libc::FUTEX_WAIT | libc::FUTEX_PRIVATE_FLAG,
                expected,
                core::ptr::null_mut::<c_void>(),
            );
        };
    }
}

#[inline]
pub fn wake_one(ptr: *const AtomicU32) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,
            ptr,
            libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
            1i32,
        );
    };
}

#[inline]
pub fn wake_all(ptr: *const AtomicU32) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,
            ptr,
            libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
            i32::MAX,
        );
    };
}
