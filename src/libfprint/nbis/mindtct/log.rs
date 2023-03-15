use ::libc;
#[no_mangle]
pub unsafe extern "C" fn open_logfile() -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn print2log(mut fmt: *mut libc::c_char, mut args: ...) {}
#[no_mangle]
pub unsafe extern "C" fn close_logfile() -> libc::c_int {
    return 0 as libc::c_int;
}
