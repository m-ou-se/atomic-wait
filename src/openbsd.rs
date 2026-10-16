use core::sync::atomic::AtomicU32;

#[inline]
pub fn wait(a: &AtomicU32, expected: u32) {
    unsafe {
        libc::futex(
            a as *const _ as *mut u32,
            libc::FUTEX_WAIT,
            expected as i32,
            core::ptr::null(),
            core::ptr::null_mut(),
        );
    };
}

#[inline]
pub fn wake_one(ptr: *const AtomicU32) {
    unsafe {
        libc::futex(
            ptr as *mut u32,
            libc::FUTEX_WAKE,
            1,
            core::ptr::null(),
            core::ptr::null_mut(),
        );
    };
}

#[inline]
pub fn wake_all(ptr: *const AtomicU32) {
    unsafe {
        libc::futex(
            ptr as *mut u32,
            libc::FUTEX_WAKE,
            i32::MAX,
            core::ptr::null(),
            core::ptr::null_mut(),
        );
    };
}
