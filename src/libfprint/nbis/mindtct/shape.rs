use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn contour_limits(
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *const libc::c_int,
        _: *const libc::c_int,
        _: libc::c_int,
    );
    fn in_int_list(_: libc::c_int, _: *const libc::c_int, _: libc::c_int) -> libc::c_int;
    fn bubble_sort_int_inc(_: *mut libc::c_int, _: libc::c_int);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rows {
    pub y: libc::c_int,
    pub xs: *mut libc::c_int,
    pub alloc: libc::c_int,
    pub npts: libc::c_int,
}
pub type ROW = rows;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shape {
    pub ymin: libc::c_int,
    pub ymax: libc::c_int,
    pub rows: *mut *mut ROW,
    pub alloc: libc::c_int,
    pub nrows: libc::c_int,
}
pub type SHAPE = shape;
#[no_mangle]
pub unsafe extern "C" fn alloc_shape(
    mut oshape: *mut *mut SHAPE,
    xmin: libc::c_int,
    ymin: libc::c_int,
    xmax: libc::c_int,
    ymax: libc::c_int,
) -> libc::c_int {
    let mut shape: *mut SHAPE = 0 as *mut SHAPE;
    let mut alloc_rows: libc::c_int = 0;
    let mut alloc_pts: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    alloc_rows = ymax - ymin + 1 as libc::c_int;
    alloc_pts = xmax - xmin + 1 as libc::c_int;
    shape = g_malloc(::core::mem::size_of::<SHAPE>() as libc::c_ulong) as *mut SHAPE;
    (*shape)
        .rows = g_malloc(
        (alloc_rows as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut ROW>() as libc::c_ulong),
    ) as *mut *mut ROW;
    (*shape).ymin = ymin;
    (*shape).ymax = ymax;
    (*shape).alloc = alloc_rows;
    (*shape).nrows = alloc_rows;
    i = 0 as libc::c_int;
    y = ymin;
    while i < alloc_rows {
        let ref mut fresh0 = *((*shape).rows).offset(i as isize);
        *fresh0 = g_malloc(::core::mem::size_of::<ROW>() as libc::c_ulong) as *mut ROW;
        let ref mut fresh1 = (**((*shape).rows).offset(i as isize)).xs;
        *fresh1 = g_malloc(
            (alloc_pts as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        (**((*shape).rows).offset(i as isize)).y = y;
        (**((*shape).rows).offset(i as isize)).alloc = alloc_pts;
        (**((*shape).rows).offset(i as isize)).npts = 0 as libc::c_int;
        i += 1;
        y += 1;
    }
    *oshape = shape;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn free_shape(mut shape: *mut SHAPE) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*shape).alloc {
        g_free((**((*shape).rows).offset(i as isize)).xs as gpointer);
        g_free(*((*shape).rows).offset(i as isize) as gpointer);
        i += 1;
    }
    g_free((*shape).rows as gpointer);
    g_free(shape as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn shape_from_contour(
    mut oshape: *mut *mut SHAPE,
    mut contour_x: *const libc::c_int,
    mut contour_y: *const libc::c_int,
    ncontour: libc::c_int,
) -> libc::c_int {
    let mut shape: *mut SHAPE = 0 as *mut SHAPE;
    let mut row: *mut ROW = 0 as *mut ROW;
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut xmin: libc::c_int = 0;
    let mut ymin: libc::c_int = 0;
    let mut xmax: libc::c_int = 0;
    let mut ymax: libc::c_int = 0;
    contour_limits(
        &mut xmin,
        &mut ymin,
        &mut xmax,
        &mut ymax,
        contour_x,
        contour_y,
        ncontour,
    );
    ret = alloc_shape(&mut shape, xmin, ymin, xmax, ymax);
    if ret != 0 {
        return ret;
    }
    i = 0 as libc::c_int;
    while i < ncontour {
        row = *((*shape).rows).offset((*contour_y.offset(i as isize) - ymin) as isize);
        if in_int_list(*contour_x.offset(i as isize), (*row).xs, (*row).npts)
            < 0 as libc::c_int
        {
            if (*row).npts >= (*row).alloc {
                g_free(shape as gpointer);
                fprintf(
                    stderr,
                    b"ERROR : shape_from_contour : row overflow\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(260 as libc::c_int);
            }
            let fresh2 = (*row).npts;
            (*row).npts = (*row).npts + 1;
            *((*row).xs).offset(fresh2 as isize) = *contour_x.offset(i as isize);
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*shape).nrows {
        sort_row_on_x(*((*shape).rows).offset(i as isize));
        i += 1;
    }
    *oshape = shape;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sort_row_on_x(mut row: *mut ROW) {
    bubble_sort_int_inc((*row).xs, (*row).npts);
}
