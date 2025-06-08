use core::sync::atomic::AtomicU32;

#[inline]
pub fn wait(a: &AtomicU32, expected: u32) {
    let ptr: *const AtomicU32 = a;
    unsafe {
        libc::_umtx_op(
            ptr as *mut libc::c_void,
            libc::UMTX_OP_WAIT_UINT_PRIVATE,
            expected as libc::c_ulong,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
    };
}

#[inline]
pub fn wake_one(ptr: *const AtomicU32) {
    unsafe {
        libc::_umtx_op(
            ptr as *mut libc::c_void,
            libc::UMTX_OP_WAKE_PRIVATE,
            1 as libc::c_ulong,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
    };
}

#[inline]
pub fn wake_n(ptr: *const AtomicU32, wake_count: u32) {
    unsafe {
        libc::_umtx_op(
            ptr as *mut libc::c_void,
            libc::UMTX_OP_WAKE_PRIVATE,
            wake_count as libc::c_ulong,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
    };
}

#[inline]
pub fn wake_all(ptr: *const AtomicU32) {
    unsafe {
        libc::_umtx_op(
            ptr as *mut libc::c_void,
            libc::UMTX_OP_WAKE_PRIVATE,
            i32::MAX as libc::c_ulong,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
    };
}
