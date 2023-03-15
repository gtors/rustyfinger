use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
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
pub struct dir2rad {
    pub ndirs: libc::c_int,
    pub cos: *mut libc::c_double,
    pub sin: *mut libc::c_double,
}
pub type DIR2RAD = dir2rad;
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
pub unsafe extern "C" fn init_dir2rad(
    mut optr: *mut *mut DIR2RAD,
    ndirs: libc::c_int,
) -> libc::c_int {
    let mut dir2rad: *mut DIR2RAD = 0 as *mut DIR2RAD;
    let mut i: libc::c_int = 0;
    let mut theta: libc::c_double = 0.;
    let mut pi_factor: libc::c_double = 0.;
    let mut cs: libc::c_double = 0.;
    let mut sn: libc::c_double = 0.;
    dir2rad = g_malloc(::core::mem::size_of::<DIR2RAD>() as libc::c_ulong)
        as *mut DIR2RAD;
    (*dir2rad).ndirs = ndirs;
    (*dir2rad)
        .cos = g_malloc(
        (ndirs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*dir2rad)
        .sin = g_malloc(
        (ndirs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    pi_factor = 2.0f64 * 3.14159265358979323846f64 / ndirs as libc::c_double;
    i = 0 as libc::c_int;
    while i < ndirs {
        theta = i as libc::c_double * pi_factor;
        cs = cos(theta);
        sn = sin(theta);
        cs = if cs < 0.0f64 {
            (cs * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
        } else {
            (cs * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
        };
        sn = if sn < 0.0f64 {
            (sn * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
        } else {
            (sn * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
        };
        *((*dir2rad).cos).offset(i as isize) = cs;
        *((*dir2rad).sin).offset(i as isize) = sn;
        i += 1;
    }
    *optr = dir2rad;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn init_dftwaves(
    mut optr: *mut *mut DFTWAVES,
    mut dft_coefs: *const libc::c_double,
    nwaves: libc::c_int,
    blocksize: libc::c_int,
) -> libc::c_int {
    let mut dftwaves: *mut DFTWAVES = 0 as *mut DFTWAVES;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pi_factor: libc::c_double = 0.;
    let mut freq: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut cptr: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut sptr: *mut libc::c_double = 0 as *mut libc::c_double;
    dftwaves = g_malloc(::core::mem::size_of::<DFTWAVES>() as libc::c_ulong)
        as *mut DFTWAVES;
    (*dftwaves).nwaves = nwaves;
    (*dftwaves).wavelen = blocksize;
    (*dftwaves)
        .waves = g_malloc(
        (nwaves as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut DFTWAVE>() as libc::c_ulong),
    ) as *mut *mut DFTWAVE;
    if dftwaves.is_null() {
        g_free(dftwaves as gpointer);
        fprintf(
            stderr,
            b"ERROR : init_dftwaves : malloc : dftwaves->waves\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(21 as libc::c_int);
    }
    pi_factor = 2.0f64 * 3.14159265358979323846f64 / blocksize as libc::c_double;
    i = 0 as libc::c_int;
    while i < nwaves {
        let ref mut fresh0 = *((*dftwaves).waves).offset(i as isize);
        *fresh0 = g_malloc(::core::mem::size_of::<DFTWAVE>() as libc::c_ulong)
            as *mut DFTWAVE;
        let ref mut fresh1 = (**((*dftwaves).waves).offset(i as isize)).cos;
        *fresh1 = g_malloc(
            (blocksize as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double;
        let ref mut fresh2 = (**((*dftwaves).waves).offset(i as isize)).sin;
        *fresh2 = g_malloc(
            (blocksize as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double;
        cptr = (**((*dftwaves).waves).offset(i as isize)).cos;
        sptr = (**((*dftwaves).waves).offset(i as isize)).sin;
        freq = pi_factor * *dft_coefs.offset(i as isize);
        j = 0 as libc::c_int;
        while j < blocksize {
            x = freq * j as libc::c_double;
            let fresh3 = cptr;
            cptr = cptr.offset(1);
            *fresh3 = cos(x);
            let fresh4 = sptr;
            sptr = sptr.offset(1);
            *fresh4 = sin(x);
            j += 1;
        }
        i += 1;
    }
    *optr = dftwaves;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_max_padding_V2(
    map_windowsize: libc::c_int,
    map_windowoffset: libc::c_int,
    dirbin_grid_w: libc::c_int,
    dirbin_grid_h: libc::c_int,
) -> libc::c_int {
    let mut dft_pad: libc::c_int = 0;
    let mut dirbin_pad: libc::c_int = 0;
    let mut max_pad: libc::c_int = 0;
    let mut diag: libc::c_double = 0.;
    let mut pad: libc::c_double = 0.;
    diag = sqrt(
        2.0f64 * map_windowsize as libc::c_double * map_windowsize as libc::c_double,
    );
    pad = (diag - map_windowsize as libc::c_double) / 2.0f64;
    pad = if pad < 0.0f64 {
        (pad * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
    } else {
        (pad * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
    };
    dft_pad = (if pad < 0 as libc::c_int as libc::c_double {
        pad - 0.5f64
    } else {
        pad + 0.5f64
    }) as libc::c_int + map_windowoffset;
    diag = sqrt(
        (dirbin_grid_w * dirbin_grid_w + dirbin_grid_h * dirbin_grid_h) as libc::c_double,
    );
    pad = (diag - 1 as libc::c_int as libc::c_double) / 2.0f64;
    pad = if pad < 0.0f64 {
        (pad * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
    } else {
        (pad * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
    };
    dirbin_pad = (if pad < 0 as libc::c_int as libc::c_double {
        pad - 0.5f64
    } else {
        pad + 0.5f64
    }) as libc::c_int;
    max_pad = if dft_pad > dirbin_pad { dft_pad } else { dirbin_pad };
    return max_pad;
}
#[no_mangle]
pub unsafe extern "C" fn init_rotgrids(
    mut optr: *mut *mut ROTGRIDS,
    iw: libc::c_int,
    ih: libc::c_int,
    ipad: libc::c_int,
    start_dir_angle: libc::c_double,
    ndirs: libc::c_int,
    grid_w: libc::c_int,
    grid_h: libc::c_int,
    relative2: libc::c_int,
) -> libc::c_int {
    let mut rotgrids: *mut ROTGRIDS = 0 as *mut ROTGRIDS;
    let mut pi_offset: libc::c_double = 0.;
    let mut pi_incr: libc::c_double = 0.;
    let mut dir: libc::c_int = 0;
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut grid_size: libc::c_int = 0;
    let mut pw: libc::c_int = 0;
    let mut grid_pad: libc::c_int = 0;
    let mut min_dim: libc::c_int = 0;
    let mut grid: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut diag: libc::c_double = 0.;
    let mut theta: libc::c_double = 0.;
    let mut cs: libc::c_double = 0.;
    let mut sn: libc::c_double = 0.;
    let mut cx: libc::c_double = 0.;
    let mut cy: libc::c_double = 0.;
    let mut fxm: libc::c_double = 0.;
    let mut fym: libc::c_double = 0.;
    let mut fx: libc::c_double = 0.;
    let mut fy: libc::c_double = 0.;
    let mut ixt: libc::c_int = 0;
    let mut iyt: libc::c_int = 0;
    let mut pad: libc::c_double = 0.;
    rotgrids = g_malloc(::core::mem::size_of::<ROTGRIDS>() as libc::c_ulong)
        as *mut ROTGRIDS;
    (*rotgrids).ngrids = ndirs;
    (*rotgrids).grid_w = grid_w;
    (*rotgrids).grid_h = grid_h;
    (*rotgrids).start_angle = start_dir_angle;
    (*rotgrids).relative2 = relative2;
    diag = sqrt((grid_w * grid_w + grid_h * grid_h) as libc::c_double);
    match relative2 {
        0 => {
            pad = (diag - 1 as libc::c_int as libc::c_double) / 2.0f64;
            pad = if pad < 0.0f64 {
                (pad * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
            } else {
                (pad * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
            };
            grid_pad = (if pad < 0 as libc::c_int as libc::c_double {
                pad - 0.5f64
            } else {
                pad + 0.5f64
            }) as libc::c_int;
        }
        1 => {
            min_dim = if grid_w < grid_h { grid_w } else { grid_h };
            pad = (diag - min_dim as libc::c_double) / 2.0f64;
            pad = if pad < 0.0f64 {
                (pad * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
            } else {
                (pad * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
            };
            grid_pad = (if pad < 0 as libc::c_int as libc::c_double {
                pad - 0.5f64
            } else {
                pad + 0.5f64
            }) as libc::c_int;
        }
        _ => {
            fprintf(
                stderr,
                b"ERROR : init_rotgrids : Illegal relative flag : %d\n\0" as *const u8
                    as *const libc::c_char,
                relative2,
            );
            g_free(rotgrids as gpointer);
            return -(31 as libc::c_int);
        }
    }
    if ipad == -(1 as libc::c_int) {
        (*rotgrids).pad = grid_pad;
    } else {
        if ipad < grid_pad {
            fprintf(
                stderr,
                b"ERROR : init_rotgrids : Pad passed is too small\n\0" as *const u8
                    as *const libc::c_char,
            );
            g_free(rotgrids as gpointer);
            return -(32 as libc::c_int);
        }
        (*rotgrids).pad = ipad;
    }
    grid_size = grid_w * grid_h;
    pw = iw + ((*rotgrids).pad << 1 as libc::c_int);
    cx = (grid_w - 1 as libc::c_int) as libc::c_double / 2.0f64;
    cy = (grid_h - 1 as libc::c_int) as libc::c_double / 2.0f64;
    (*rotgrids)
        .grids = g_malloc(
        (ndirs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    pi_offset = start_dir_angle;
    pi_incr = 3.14159265358979323846f64 / ndirs as libc::c_double;
    dir = 0 as libc::c_int;
    theta = pi_offset;
    while dir < ndirs {
        let ref mut fresh5 = *((*rotgrids).grids).offset(dir as isize);
        *fresh5 = g_malloc(
            (grid_size as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        grid = *((*rotgrids).grids).offset(dir as isize);
        cs = cos(theta);
        sn = sin(theta);
        iy = 0 as libc::c_int;
        while iy < grid_h {
            fxm = -1.0f64 * ((iy as libc::c_double - cy) * sn);
            fym = (iy as libc::c_double - cy) * cs;
            if relative2 == 1 as libc::c_int {
                fxm += cx;
                fym += cy;
            }
            ix = 0 as libc::c_int;
            while ix < grid_w {
                fx = fxm + (ix as libc::c_double - cx) * cs;
                fy = fym + (ix as libc::c_double - cx) * sn;
                fx = if fx < 0.0f64 {
                    (fx * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double
                        / 16384.0f64
                } else {
                    (fx * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double
                        / 16384.0f64
                };
                fy = if fy < 0.0f64 {
                    (fy * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double
                        / 16384.0f64
                } else {
                    (fy * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double
                        / 16384.0f64
                };
                ixt = (if fx < 0 as libc::c_int as libc::c_double {
                    fx - 0.5f64
                } else {
                    fx + 0.5f64
                }) as libc::c_int;
                iyt = (if fy < 0 as libc::c_int as libc::c_double {
                    fy - 0.5f64
                } else {
                    fy + 0.5f64
                }) as libc::c_int;
                let fresh6 = grid;
                grid = grid.offset(1);
                *fresh6 = ixt + iyt * pw;
                ix += 1;
            }
            iy += 1;
        }
        dir += 1;
        theta += pi_incr;
    }
    *optr = rotgrids;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn alloc_dir_powers(
    mut opowers: *mut *mut *mut libc::c_double,
    nwaves: libc::c_int,
    ndirs: libc::c_int,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut powers: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    powers = g_malloc(
        (nwaves as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_double>() as libc::c_ulong),
    ) as *mut *mut libc::c_double;
    w = 0 as libc::c_int;
    while w < nwaves {
        let ref mut fresh7 = *powers.offset(w as isize);
        *fresh7 = g_malloc(
            (ndirs as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double;
        w += 1;
    }
    *opowers = powers;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn alloc_power_stats(
    mut owis: *mut *mut libc::c_int,
    mut opowmaxs: *mut *mut libc::c_double,
    mut opowmax_dirs: *mut *mut libc::c_int,
    mut opownorms: *mut *mut libc::c_double,
    nstats: libc::c_int,
) -> libc::c_int {
    let mut wis: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut powmax_dirs: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut powmaxs: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pownorms: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dest: gsize = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        let (fresh8, fresh9) = nstats
            .overflowing_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
        *(&mut dest as *mut gsize) = fresh8;
        if !fresh9 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/init.c\0" as *const u8 as *const libc::c_char,
            600 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"alloc_power_stats\0"))
                .as_ptr(),
            b"g_size_checked_mul(&dest, nstats, sizeof(int))\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut dest_0: gsize = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        let (fresh10, fresh11) = nstats
            .overflowing_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong);
        *(&mut dest_0 as *mut gsize) = fresh10;
        if !fresh11 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/init.c\0" as *const u8 as *const libc::c_char,
            601 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"alloc_power_stats\0"))
                .as_ptr(),
            b"g_size_checked_mul(&dest, nstats, sizeof(double))\0" as *const u8
                as *const libc::c_char,
        );
    }
    wis = g_malloc(
        (nstats as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    powmaxs = g_malloc(
        (nstats as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    powmax_dirs = g_malloc(
        (nstats as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    pownorms = g_malloc(
        (nstats as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    *owis = wis;
    *opowmaxs = powmaxs;
    *opowmax_dirs = powmax_dirs;
    *opownorms = pownorms;
    return 0 as libc::c_int;
}
