use ::libc;
extern "C" {
    static mut g_feature_patterns: [FEATURE_PATTERN; 0];
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct feature_pattern {
    pub type_0: libc::c_int,
    pub appearing: libc::c_int,
    pub first: [libc::c_int; 2],
    pub second: [libc::c_int; 2],
    pub third: [libc::c_int; 2],
}
pub type FEATURE_PATTERN = feature_pattern;
#[no_mangle]
pub unsafe extern "C" fn match_1st_pair(
    mut p1: libc::c_uchar,
    mut p2: libc::c_uchar,
    mut possible: *mut libc::c_int,
    mut nposs: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    *nposs = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        if p1 as libc::c_int
            == (*g_feature_patterns.as_mut_ptr().offset(i as isize))
                .first[0 as libc::c_int as usize]
            && p2 as libc::c_int
                == (*g_feature_patterns.as_mut_ptr().offset(i as isize))
                    .first[1 as libc::c_int as usize]
        {
            *possible.offset(*nposs as isize) = i;
            *nposs += 1;
        }
        i += 1;
    }
    return *nposs;
}
#[no_mangle]
pub unsafe extern "C" fn match_2nd_pair(
    mut p1: libc::c_uchar,
    mut p2: libc::c_uchar,
    mut possible: *mut libc::c_int,
    mut nposs: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tnposs: libc::c_int = 0;
    tnposs = *nposs;
    *nposs = 0 as libc::c_int;
    if p1 as libc::c_int == p2 as libc::c_int {
        return *nposs;
    }
    i = 0 as libc::c_int;
    while i < tnposs {
        if p1 as libc::c_int
            == (*g_feature_patterns
                .as_mut_ptr()
                .offset(*possible.offset(i as isize) as isize))
                .second[0 as libc::c_int as usize]
            && p2 as libc::c_int
                == (*g_feature_patterns
                    .as_mut_ptr()
                    .offset(*possible.offset(i as isize) as isize))
                    .second[1 as libc::c_int as usize]
        {
            *possible.offset(*nposs as isize) = *possible.offset(i as isize);
            *nposs += 1;
        }
        i += 1;
    }
    return *nposs;
}
#[no_mangle]
pub unsafe extern "C" fn match_3rd_pair(
    mut p1: libc::c_uchar,
    mut p2: libc::c_uchar,
    mut possible: *mut libc::c_int,
    mut nposs: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tnposs: libc::c_int = 0;
    tnposs = *nposs;
    *nposs = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < tnposs {
        if p1 as libc::c_int
            == (*g_feature_patterns
                .as_mut_ptr()
                .offset(*possible.offset(i as isize) as isize))
                .third[0 as libc::c_int as usize]
            && p2 as libc::c_int
                == (*g_feature_patterns
                    .as_mut_ptr()
                    .offset(*possible.offset(i as isize) as isize))
                    .third[1 as libc::c_int as usize]
        {
            *possible.offset(*nposs as isize) = *possible.offset(i as isize);
            *nposs += 1;
        }
        i += 1;
    }
    return *nposs;
}
#[no_mangle]
pub unsafe extern "C" fn skip_repeated_horizontal_pair(
    mut cx: *mut libc::c_int,
    ex: libc::c_int,
    mut p1ptr: *mut *mut libc::c_uchar,
    mut p2ptr: *mut *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) {
    let mut old1: libc::c_int = 0;
    let mut old2: libc::c_int = 0;
    old1 = **p1ptr as libc::c_int;
    old2 = **p2ptr as libc::c_int;
    *cx += 1;
    *p1ptr = (*p1ptr).offset(1);
    *p2ptr = (*p2ptr).offset(1);
    while *cx < ex {
        if **p1ptr as libc::c_int != old1 || **p2ptr as libc::c_int != old2 {
            return;
        }
        *cx += 1;
        *p1ptr = (*p1ptr).offset(1);
        *p2ptr = (*p2ptr).offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn skip_repeated_vertical_pair(
    mut cy: *mut libc::c_int,
    ey: libc::c_int,
    mut p1ptr: *mut *mut libc::c_uchar,
    mut p2ptr: *mut *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) {
    let mut old1: libc::c_int = 0;
    let mut old2: libc::c_int = 0;
    old1 = **p1ptr as libc::c_int;
    old2 = **p2ptr as libc::c_int;
    *cy += 1;
    *p1ptr = (*p1ptr).offset(iw as isize);
    *p2ptr = (*p2ptr).offset(iw as isize);
    while *cy < ey {
        if **p1ptr as libc::c_int != old1 || **p2ptr as libc::c_int != old2 {
            return;
        }
        *cy += 1;
        *p1ptr = (*p1ptr).offset(iw as isize);
        *p2ptr = (*p2ptr).offset(iw as isize);
    }
}
