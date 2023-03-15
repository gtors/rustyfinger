use ::libc;
static mut program_buffer: [libc::c_char; 1024] = [0; 1024];
static mut pfile: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut gfile: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn get_progname() -> *mut libc::c_char {
    return program_buffer.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn get_probe_filename() -> *mut libc::c_char {
    return pfile;
}
#[no_mangle]
pub unsafe extern "C" fn get_gallery_filename() -> *mut libc::c_char {
    return gfile;
}
