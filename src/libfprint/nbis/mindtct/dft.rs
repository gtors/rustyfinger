use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn bubble_sort_double_dec_2(
        _: *mut libc::c_double,
        _: *mut libc::c_int,
        _: libc::c_int,
    );
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
pub struct dftwave {
    pub cos: *mut libc::c_double,
    pub sin: *mut libc::c_double,
}
pub type DFTWAVE = dftwave;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dftwaves {
    pub nwaves: libc::c_int,
    pub wavelen: libc::c_int,
    pub waves: *mut *mut DFTWAVE,
}
pub type DFTWAVES = dftwaves;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rotgrids {
    pub pad: libc::c_int,
    pub relative2: libc::c_int,
    pub start_angle: libc::c_double,
    pub ngrids: libc::c_int,
    pub grid_w: libc::c_int,
    pub grid_h: libc::c_int,
    pub grids: *mut *mut libc::c_int,
}
pub type ROTGRIDS = rotgrids;
#[no_mangle]
pub unsafe extern "C" fn dft_dir_powers(
    mut powers: *mut *mut libc::c_double,
    mut pdata: *mut libc::c_uchar,
    blkoffset: libc::c_int,
    pw: libc::c_int,
    ph: libc::c_int,
    mut dftwaves: *const DFTWAVES,
    mut dftgrids: *const ROTGRIDS,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut rowsums: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut blkptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (*dftgrids).grid_w != (*dftgrids).grid_h {
        fprintf(
            stderr,
            b"ERROR : dft_dir_powers : DFT grids must be square\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(90 as libc::c_int);
    }
    rowsums = g_malloc(
        ((*dftgrids).grid_w as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    memset(
        rowsums as *mut libc::c_void,
        0 as libc::c_int,
        ((*dftgrids).grid_w as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    dir = 0 as libc::c_int;
    while dir < (*dftgrids).ngrids {
        blkptr = pdata.offset(blkoffset as isize);
        sum_rot_block_rows(
            rowsums,
            blkptr,
            *((*dftgrids).grids).offset(dir as isize),
            (*dftgrids).grid_w,
        );
        w = 0 as libc::c_int;
        while w < (*dftwaves).nwaves {
            dft_power(
                &mut *(*powers.offset(w as isize)).offset(dir as isize),
                rowsums,
                *((*dftwaves).waves).offset(w as isize),
                (*dftwaves).wavelen,
            );
            w += 1;
        }
        dir += 1;
    }
    g_free(rowsums as gpointer);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sum_rot_block_rows(
    mut rowsums: *mut libc::c_int,
    mut blkptr: *const libc::c_uchar,
    mut grid_offsets: *const libc::c_int,
    blocksize: libc::c_int,
) {
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut gi: libc::c_int = 0;
    gi = 0 as libc::c_int;
    iy = 0 as libc::c_int;
    while iy < blocksize {
        *rowsums.offset(iy as isize) = 0 as libc::c_int;
        ix = 0 as libc::c_int;
        while ix < blocksize {
            *rowsums.offset(iy as isize)
                += *blkptr.offset(*grid_offsets.offset(gi as isize) as isize)
                    as libc::c_int;
            gi += 1;
            ix += 1;
        }
        iy += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dft_power(
    mut power: *mut libc::c_double,
    mut rowsums: *const libc::c_int,
    mut wave: *const DFTWAVE,
    wavelen: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut cospart: libc::c_double = 0.;
    let mut sinpart: libc::c_double = 0.;
    cospart = 0.0f64;
    sinpart = 0.0f64;
    i = 0 as libc::c_int;
    while i < wavelen {
        cospart
            += *rowsums.offset(i as isize) as libc::c_double
                * *((*wave).cos).offset(i as isize);
        sinpart
            += *rowsums.offset(i as isize) as libc::c_double
                * *((*wave).sin).offset(i as isize);
        i += 1;
    }
    *power = cospart * cospart + sinpart * sinpart;
}
#[no_mangle]
pub unsafe extern "C" fn dft_power_stats(
    mut wis: *mut libc::c_int,
    mut powmaxs: *mut libc::c_double,
    mut powmax_dirs: *mut libc::c_int,
    mut pownorms: *mut libc::c_double,
    mut powers: *mut *mut libc::c_double,
    fw: libc::c_int,
    tw: libc::c_int,
    ndirs: libc::c_int,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    w = fw;
    i = 0 as libc::c_int;
    while w < tw {
        get_max_norm(
            &mut *powmaxs.offset(i as isize),
            &mut *powmax_dirs.offset(i as isize),
            &mut *pownorms.offset(i as isize),
            *powers.offset(w as isize),
            ndirs,
        );
        w += 1;
        i += 1;
    }
    ret = sort_dft_waves(wis, powmaxs, pownorms, tw - fw);
    if ret != 0 {
        return ret;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_max_norm(
    mut powmax: *mut libc::c_double,
    mut powmax_dir: *mut libc::c_int,
    mut pownorm: *mut libc::c_double,
    mut power_vector: *const libc::c_double,
    ndirs: libc::c_int,
) {
    let mut dir: libc::c_int = 0;
    let mut max_v: libc::c_double = 0.;
    let mut powsum: libc::c_double = 0.;
    let mut max_i: libc::c_int = 0;
    let mut powmean: libc::c_double = 0.;
    max_v = *power_vector.offset(0 as libc::c_int as isize);
    max_i = 0 as libc::c_int;
    powsum = *power_vector.offset(0 as libc::c_int as isize);
    dir = 1 as libc::c_int;
    while dir < ndirs {
        powsum += *power_vector.offset(dir as isize);
        if *power_vector.offset(dir as isize) > max_v {
            max_v = *power_vector.offset(dir as isize);
            max_i = dir;
        }
        dir += 1;
    }
    *powmax = max_v;
    *powmax_dir = max_i;
    powmean = (if powsum > 10.0f64 { powsum } else { 10.0f64 })
        / ndirs as libc::c_double;
    *pownorm = *powmax / powmean;
}
#[no_mangle]
pub unsafe extern "C" fn sort_dft_waves(
    mut wis: *mut libc::c_int,
    mut powmaxs: *const libc::c_double,
    mut pownorms: *const libc::c_double,
    nstats: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut pownorms2: *mut libc::c_double = 0 as *mut libc::c_double;
    pownorms2 = g_malloc(
        (nstats as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < nstats {
        *wis.offset(i as isize) = i;
        *pownorms2
            .offset(
                i as isize,
            ) = *powmaxs.offset(i as isize) * *pownorms.offset(i as isize);
        i += 1;
    }
    bubble_sort_double_dec_2(pownorms2, wis, nstats);
    g_free(pownorms2 as gpointer);
    return 0 as libc::c_int;
}
