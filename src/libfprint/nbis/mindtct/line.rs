use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type gsize = libc::c_ulong;
pub type gpointer = *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn line_points(
    mut ox_list: *mut *mut libc::c_int,
    mut oy_list: *mut *mut libc::c_int,
    mut onum: *mut libc::c_int,
    x1: libc::c_int,
    y1: libc::c_int,
    x2: libc::c_int,
    y2: libc::c_int,
) -> libc::c_int {
    let mut asize: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut adx: libc::c_int = 0;
    let mut ady: libc::c_int = 0;
    let mut x_incr: libc::c_int = 0;
    let mut y_incr: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut inx: libc::c_int = 0;
    let mut iny: libc::c_int = 0;
    let mut intx: libc::c_int = 0;
    let mut inty: libc::c_int = 0;
    let mut x_factor: libc::c_double = 0.;
    let mut y_factor: libc::c_double = 0.;
    let mut rx: libc::c_double = 0.;
    let mut ry: libc::c_double = 0.;
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut x_list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut y_list: *mut libc::c_int = 0 as *mut libc::c_int;
    asize = if abs(x2 - x1) + 2 as libc::c_int > abs(y2 - y1) + 2 as libc::c_int {
        abs(x2 - x1) + 2 as libc::c_int
    } else {
        abs(y2 - y1) + 2 as libc::c_int
    };
    x_list = g_malloc(
        (asize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    y_list = g_malloc(
        (asize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    dx = x2 - x1;
    dy = y2 - y1;
    if dx >= 0 as libc::c_int {
        x_incr = 1 as libc::c_int;
    } else {
        x_incr = -(1 as libc::c_int);
    }
    if dy >= 0 as libc::c_int {
        y_incr = 1 as libc::c_int;
    } else {
        y_incr = -(1 as libc::c_int);
    }
    adx = abs(dx);
    ady = abs(dy);
    if adx > ady {
        inx = 1 as libc::c_int;
    } else {
        inx = 0 as libc::c_int;
    }
    if ady > adx {
        iny = 1 as libc::c_int;
    } else {
        iny = 0 as libc::c_int;
    }
    intx = 1 as libc::c_int - iny;
    inty = 1 as libc::c_int - inx;
    x_factor = (inx * x_incr) as libc::c_double
        + iny as libc::c_double
            * (dx as libc::c_double
                / (if 1 as libc::c_int > ady { 1 as libc::c_int } else { ady })
                    as libc::c_double);
    y_factor = (iny * y_incr) as libc::c_double
        + inx as libc::c_double
            * (dy as libc::c_double
                / (if 1 as libc::c_int > adx { 1 as libc::c_int } else { adx })
                    as libc::c_double);
    ix = x1;
    iy = y1;
    rx = x1 as libc::c_double;
    ry = y1 as libc::c_double;
    i = 0 as libc::c_int;
    *x_list.offset(i as isize) = x1;
    let fresh0 = i;
    i = i + 1;
    *y_list.offset(fresh0 as isize) = y1;
    while ix != x2 || iy != y2 {
        if i >= asize {
            fprintf(
                stderr,
                b"ERROR : line_points : coord list overflow\n\0" as *const u8
                    as *const libc::c_char,
            );
            g_free(x_list as gpointer);
            g_free(y_list as gpointer);
            return -(412 as libc::c_int);
        }
        rx += x_factor;
        ry += y_factor;
        rx = if rx < 0.0f64 {
            (rx * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
        } else {
            (rx * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
        };
        ry = if ry < 0.0f64 {
            (ry * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
        } else {
            (ry * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
        };
        ix = intx * (ix + x_incr) + iny * (rx + 0.5f64) as libc::c_int;
        iy = inty * (iy + y_incr) + inx * (ry + 0.5f64) as libc::c_int;
        *x_list.offset(i as isize) = ix;
        let fresh1 = i;
        i = i + 1;
        *y_list.offset(fresh1 as isize) = iy;
    }
    *ox_list = x_list;
    *oy_list = y_list;
    *onum = i;
    return 0 as libc::c_int;
}
