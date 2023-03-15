use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
    fn pixelize_map(
        _: *mut *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
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
pub struct fp_minutia {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub ex: libc::c_int,
    pub ey: libc::c_int,
    pub direction: libc::c_int,
    pub reliability: libc::c_double,
    pub type_0: libc::c_int,
    pub appearing: libc::c_int,
    pub feature_id: libc::c_int,
    pub nbrs: *mut libc::c_int,
    pub ridge_counts: *mut libc::c_int,
    pub num_nbrs: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fp_minutiae {
    pub alloc: libc::c_int,
    pub num: libc::c_int,
    pub list: *mut *mut fp_minutia,
}
pub type MINUTIA = fp_minutia;
pub type MINUTIAE = fp_minutiae;
#[no_mangle]
pub unsafe extern "C" fn gen_quality_map(
    mut oqmap: *mut *mut libc::c_int,
    mut direction_map: *mut libc::c_int,
    mut low_contrast_map: *mut libc::c_int,
    mut low_flow_map: *mut libc::c_int,
    mut high_curve_map: *mut libc::c_int,
    map_w: libc::c_int,
    map_h: libc::c_int,
) -> libc::c_int {
    let mut QualMap: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut thisX: libc::c_int = 0;
    let mut thisY: libc::c_int = 0;
    let mut compX: libc::c_int = 0;
    let mut compY: libc::c_int = 0;
    let mut arrayPos: libc::c_int = 0;
    let mut arrayPos2: libc::c_int = 0;
    let mut QualOffset: libc::c_int = 0;
    let mut dest: gsize = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        let (fresh0, fresh1) = map_w.overflowing_mul(map_h);
        *(&mut dest as *mut gsize) = fresh0;
        if !fresh1 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/quality.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"gen_quality_map\0"))
                .as_ptr(),
            b"g_size_checked_mul(&dest, map_w, map_h)\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut dest_0: gsize = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        let (fresh2, fresh3) = (map_w * map_h)
            .overflowing_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
        *(&mut dest_0 as *mut gsize) = fresh2;
        if !fresh3 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/quality.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"gen_quality_map\0"))
                .as_ptr(),
            b"g_size_checked_mul(&dest, map_w * map_h, sizeof(int))\0" as *const u8
                as *const libc::c_char,
        );
    }
    QualMap = g_malloc(
        ((map_w * map_h) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    thisY = 0 as libc::c_int;
    while thisY < map_h {
        thisX = 0 as libc::c_int;
        while thisX < map_w {
            arrayPos = thisY * map_w + thisX;
            if *low_contrast_map.offset(arrayPos as isize) != 0
                || *direction_map.offset(arrayPos as isize) < 0 as libc::c_int
            {
                *QualMap.offset(arrayPos as isize) = 0 as libc::c_int;
            } else {
                if *low_flow_map.offset(arrayPos as isize) != 0
                    || *high_curve_map.offset(arrayPos as isize) != 0
                {
                    *QualMap.offset(arrayPos as isize) = 3 as libc::c_int;
                } else {
                    *QualMap.offset(arrayPos as isize) = 4 as libc::c_int;
                }
                if thisY < 2 as libc::c_int
                    || thisY > map_h - 1 as libc::c_int - 2 as libc::c_int
                    || thisX < 2 as libc::c_int
                    || thisX > map_w - 1 as libc::c_int - 2 as libc::c_int
                {
                    *QualMap.offset(arrayPos as isize) = 1 as libc::c_int;
                } else {
                    QualOffset = 0 as libc::c_int;
                    compY = thisY - 2 as libc::c_int;
                    while compY <= thisY + 2 as libc::c_int {
                        compX = thisX - 2 as libc::c_int;
                        while compX <= thisX + 2 as libc::c_int {
                            arrayPos2 = compY * map_w + compX;
                            if *low_contrast_map.offset(arrayPos2 as isize) != 0
                                || *direction_map.offset(arrayPos2 as isize)
                                    < 0 as libc::c_int
                            {
                                QualOffset = -(2 as libc::c_int);
                                break;
                            } else {
                                if *low_flow_map.offset(arrayPos2 as isize) != 0
                                    || *high_curve_map.offset(arrayPos2 as isize) != 0
                                {
                                    QualOffset = if QualOffset < -(1 as libc::c_int) {
                                        QualOffset
                                    } else {
                                        -(1 as libc::c_int)
                                    };
                                }
                                compX += 1;
                            }
                        }
                        compY += 1;
                    }
                    *QualMap.offset(arrayPos as isize) += QualOffset;
                }
            }
            thisX += 1;
        }
        thisY += 1;
    }
    *oqmap = QualMap;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn combined_minutia_quality(
    mut minutiae: *mut MINUTIAE,
    mut quality_map: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    blocksize: libc::c_int,
    mut idata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    id: libc::c_int,
    ppmm: libc::c_double,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut radius_pix: libc::c_int = 0;
    let mut pquality_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut qmap_value: libc::c_int = 0;
    let mut minutia: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut gs_reliability: libc::c_double = 0.;
    let mut reliability: libc::c_double = 0.;
    if id != 8 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR : combined_miutia_quality : \0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"image must pixel depth = %d must be 8 \0" as *const u8
                as *const libc::c_char,
            id,
        );
        fprintf(
            stderr,
            b"to compute reliability\n\0" as *const u8 as *const libc::c_char,
        );
        return -(2 as libc::c_int);
    }
    radius_pix = (if 11.0f64 / 19.69f64 * ppmm < 0 as libc::c_int as libc::c_double {
        11.0f64 / 19.69f64 * ppmm - 0.5f64
    } else {
        11.0f64 / 19.69f64 * ppmm + 0.5f64
    }) as libc::c_int;
    ret = pixelize_map(&mut pquality_map, iw, ih, quality_map, mw, mh, blocksize);
    if ret != 0 {
        return ret;
    }
    i = 0 as libc::c_int;
    while i < (*minutiae).num {
        minutia = *((*minutiae).list).offset(i as isize);
        gs_reliability = grayscale_reliability(minutia, idata, iw, ih, radius_pix);
        index = (*minutia).y * iw + (*minutia).x;
        qmap_value = *pquality_map.offset(index as isize);
        match qmap_value {
            4 => {
                reliability = 0.50f64 + 0.49f64 * gs_reliability;
            }
            3 => {
                reliability = 0.25f64 + 0.24f64 * gs_reliability;
            }
            2 => {
                reliability = 0.10f64 + 0.14f64 * gs_reliability;
            }
            1 => {
                reliability = 0.05f64 + 0.04f64 * gs_reliability;
            }
            0 => {
                reliability = 0.01f64;
            }
            _ => {
                fprintf(
                    stderr,
                    b"ERROR : combined_miutia_quality : \0" as *const u8
                        as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"unexpected quality map value %d \0" as *const u8
                        as *const libc::c_char,
                    qmap_value,
                );
                fprintf(
                    stderr,
                    b"not in range [0..4]\n\0" as *const u8 as *const libc::c_char,
                );
                g_free(pquality_map as gpointer);
                return -(3 as libc::c_int);
            }
        }
        (*minutia).reliability = reliability;
        i += 1;
    }
    g_free(pquality_map as gpointer);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn grayscale_reliability(
    mut minutia: *mut MINUTIA,
    mut idata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    radius_pix: libc::c_int,
) -> libc::c_double {
    let mut mean: libc::c_double = 0.;
    let mut stdev: libc::c_double = 0.;
    let mut reliability: libc::c_double = 0.;
    get_neighborhood_stats(&mut mean, &mut stdev, minutia, idata, iw, ih, radius_pix);
    reliability = if (if stdev > 64 as libc::c_int as libc::c_double {
        1.0f64
    } else {
        stdev / 64 as libc::c_int as libc::c_double
    })
        < 1.0f64
            - fabs(mean - 127 as libc::c_int as libc::c_double)
                / 127 as libc::c_int as libc::c_double
    {
        if stdev > 64 as libc::c_int as libc::c_double {
            1.0f64
        } else {
            stdev / 64 as libc::c_int as libc::c_double
        }
    } else {
        1.0f64
            - fabs(mean - 127 as libc::c_int as libc::c_double)
                / 127 as libc::c_int as libc::c_double
    };
    return reliability;
}
#[no_mangle]
pub unsafe extern "C" fn get_neighborhood_stats(
    mut mean: *mut libc::c_double,
    mut stdev: *mut libc::c_double,
    mut minutia: *mut MINUTIA,
    mut idata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    radius_pix: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut rows: libc::c_int = 0;
    let mut cols: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut sumX: libc::c_int = 0 as libc::c_int;
    let mut sumXX: libc::c_int = 0 as libc::c_int;
    let mut histogram: [libc::c_int; 256] = [0; 256];
    memset(
        histogram.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (256 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    x = (*minutia).x;
    y = (*minutia).y;
    if x < radius_pix || x > iw - radius_pix - 1 as libc::c_int || y < radius_pix
        || y > ih - radius_pix - 1 as libc::c_int
    {
        *mean = 0.0f64;
        *stdev = 0.0f64;
        return;
    }
    rows = y - radius_pix;
    while rows <= y + radius_pix {
        cols = x - radius_pix;
        while cols <= x + radius_pix {
            histogram[*idata.offset((rows * iw) as isize).offset(cols as isize) as usize]
                += 1;
            cols += 1;
        }
        rows += 1;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if histogram[i as usize] != 0 {
            sumX += i * histogram[i as usize];
            sumXX += i * i * histogram[i as usize];
            n += histogram[i as usize];
        }
        i += 1;
    }
    *mean = sumX as libc::c_double / n as libc::c_double;
    *stdev = sqrt(sumXX as libc::c_double / n as libc::c_double - *mean * *mean);
}
