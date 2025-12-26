#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openpty(
    _amaster: *mut libc::c_int,
    _aslave: *mut libc::c_int,
    _name: *mut libc::c_char,
    _termp: *const libc::termios,
    _winp: *const libc::winsize,
) -> libc::c_int {
    -1
}
