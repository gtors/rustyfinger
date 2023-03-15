use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn erode_charimage_2(
    mut inp: *mut libc::c_uchar,
    mut out: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) {
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut itr: *mut libc::c_uchar = inp;
    let mut otr: *mut libc::c_uchar = out;
    memcpy(
        out as *mut libc::c_void,
        inp as *const libc::c_void,
        (iw * ih) as libc::c_ulong,
    );
    row = 0 as libc::c_int;
    while row < ih {
        col = 0 as libc::c_int;
        while col < iw {
            if *itr != 0 {
                if !(get_west8_2(itr as *mut libc::c_char, col, 1 as libc::c_int)
                    as libc::c_int != 0
                    && get_east8_2(itr as *mut libc::c_char, col, iw, 1 as libc::c_int)
                        as libc::c_int != 0
                    && get_north8_2(itr as *mut libc::c_char, row, iw, 1 as libc::c_int)
                        as libc::c_int != 0
                    && get_south8_2(
                        itr as *mut libc::c_char,
                        row,
                        iw,
                        ih,
                        1 as libc::c_int,
                    ) as libc::c_int != 0)
                {
                    *otr = 0 as libc::c_int as libc::c_uchar;
                }
            }
            itr = itr.offset(1);
            otr = otr.offset(1);
            col += 1;
        }
        row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dilate_charimage_2(
    mut inp: *mut libc::c_uchar,
    mut out: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) {
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut itr: *mut libc::c_uchar = inp;
    let mut otr: *mut libc::c_uchar = out;
    memcpy(
        out as *mut libc::c_void,
        inp as *const libc::c_void,
        (iw * ih) as libc::c_ulong,
    );
    row = 0 as libc::c_int;
    while row < ih {
        col = 0 as libc::c_int;
        while col < iw {
            if *itr == 0 {
                if get_west8_2(itr as *mut libc::c_char, col, 0 as libc::c_int)
                    as libc::c_int != 0
                    || get_east8_2(itr as *mut libc::c_char, col, iw, 0 as libc::c_int)
                        as libc::c_int != 0
                    || get_north8_2(itr as *mut libc::c_char, row, iw, 0 as libc::c_int)
                        as libc::c_int != 0
                    || get_south8_2(
                        itr as *mut libc::c_char,
                        row,
                        iw,
                        ih,
                        0 as libc::c_int,
                    ) as libc::c_int != 0
                {
                    *otr = 1 as libc::c_int as libc::c_uchar;
                }
            }
            itr = itr.offset(1);
            otr = otr.offset(1);
            col += 1;
        }
        row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_south8_2(
    mut ptr: *mut libc::c_char,
    row: libc::c_int,
    iw: libc::c_int,
    ih: libc::c_int,
    failcode: libc::c_int,
) -> libc::c_char {
    if row >= ih - 1 as libc::c_int {
        return failcode as libc::c_char;
    }
    return *ptr.offset(iw as isize);
}
#[no_mangle]
pub unsafe extern "C" fn get_north8_2(
    mut ptr: *mut libc::c_char,
    row: libc::c_int,
    iw: libc::c_int,
    failcode: libc::c_int,
) -> libc::c_char {
    if row < 1 as libc::c_int {
        return failcode as libc::c_char;
    }
    return *ptr.offset(-(iw as isize));
}
#[no_mangle]
pub unsafe extern "C" fn get_east8_2(
    mut ptr: *mut libc::c_char,
    col: libc::c_int,
    iw: libc::c_int,
    failcode: libc::c_int,
) -> libc::c_char {
    if col >= iw - 1 as libc::c_int {
        return failcode as libc::c_char;
    }
    return *ptr.offset(1 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn get_west8_2(
    mut ptr: *mut libc::c_char,
    col: libc::c_int,
    failcode: libc::c_int,
) -> libc::c_char {
    if col < 1 as libc::c_int {
        return failcode as libc::c_char;
    }
    return *ptr.offset(-(1 as libc::c_int as isize));
}
