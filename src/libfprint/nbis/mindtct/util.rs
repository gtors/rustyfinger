use ::libc;
extern "C" {
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn g_malloc(n_bytes: gsize) -> gpointer;
}
pub type gsize = libc::c_ulong;
pub type gpointer = *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn maxv(
    mut list: *const libc::c_int,
    num: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut maxval: libc::c_int = 0;
    maxval = *list.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < num {
        if *list.offset(i as isize) > maxval {
            maxval = *list.offset(i as isize);
        }
        i += 1;
    }
    return maxval;
}
#[no_mangle]
pub unsafe extern "C" fn minv(
    mut list: *const libc::c_int,
    num: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut minval: libc::c_int = 0;
    minval = *list.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < num {
        if *list.offset(i as isize) < minval {
            minval = *list.offset(i as isize);
        }
        i += 1;
    }
    return minval;
}
#[no_mangle]
pub unsafe extern "C" fn minmaxs(
    mut ominmax_val: *mut *mut libc::c_int,
    mut ominmax_type: *mut *mut libc::c_int,
    mut ominmax_i: *mut *mut libc::c_int,
    mut ominmax_alloc: *mut libc::c_int,
    mut ominmax_num: *mut libc::c_int,
    mut items: *const libc::c_int,
    num: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut state: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut loc: libc::c_int = 0;
    let mut minmax_val: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut minmax_type: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut minmax_i: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut minmax_alloc: libc::c_int = 0;
    let mut minmax_num: libc::c_int = 0;
    if num < 3 as libc::c_int {
        *ominmax_alloc = 0 as libc::c_int;
        *ominmax_num = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    minmax_alloc = num - 2 as libc::c_int;
    minmax_val = g_malloc(
        (minmax_alloc as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    minmax_type = g_malloc(
        (minmax_alloc as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    minmax_i = g_malloc(
        (minmax_alloc as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    minmax_num = 0 as libc::c_int;
    i = 0 as libc::c_int;
    diff = *items.offset(1 as libc::c_int as isize)
        - *items.offset(0 as libc::c_int as isize);
    if diff > 0 as libc::c_int {
        state = 1 as libc::c_int;
    } else if diff < 0 as libc::c_int {
        state = -(1 as libc::c_int);
    } else {
        state = 0 as libc::c_int;
    }
    start = 0 as libc::c_int;
    i += 1;
    while i < num - 1 as libc::c_int {
        diff = *items.offset((i + 1 as libc::c_int) as isize)
            - *items.offset(i as isize);
        if diff > 0 as libc::c_int {
            if state == 1 as libc::c_int {
                start = i;
            } else if state == -(1 as libc::c_int) {
                loc = (start + i) / 2 as libc::c_int;
                *minmax_val.offset(minmax_num as isize) = *items.offset(loc as isize);
                *minmax_type.offset(minmax_num as isize) = -(1 as libc::c_int);
                let fresh0 = minmax_num;
                minmax_num = minmax_num + 1;
                *minmax_i.offset(fresh0 as isize) = loc;
                state = 1 as libc::c_int;
                start = i;
            } else if i - start > 1 as libc::c_int {
                loc = (start + i) / 2 as libc::c_int;
                *minmax_val.offset(minmax_num as isize) = *items.offset(loc as isize);
                *minmax_type.offset(minmax_num as isize) = -(1 as libc::c_int);
                let fresh1 = minmax_num;
                minmax_num = minmax_num + 1;
                *minmax_i.offset(fresh1 as isize) = loc;
                state = 1 as libc::c_int;
                start = i;
            } else {
                state = 1 as libc::c_int;
                start = i;
            }
        } else if diff < 0 as libc::c_int {
            if state == -(1 as libc::c_int) {
                start = i;
            } else if state == 1 as libc::c_int {
                loc = (start + i) / 2 as libc::c_int;
                *minmax_val.offset(minmax_num as isize) = *items.offset(loc as isize);
                *minmax_type.offset(minmax_num as isize) = 1 as libc::c_int;
                let fresh2 = minmax_num;
                minmax_num = minmax_num + 1;
                *minmax_i.offset(fresh2 as isize) = loc;
                state = -(1 as libc::c_int);
                start = i;
            } else if i - start > 1 as libc::c_int {
                loc = (start + i) / 2 as libc::c_int;
                *minmax_val.offset(minmax_num as isize) = *items.offset(loc as isize);
                *minmax_type.offset(minmax_num as isize) = 1 as libc::c_int;
                let fresh3 = minmax_num;
                minmax_num = minmax_num + 1;
                *minmax_i.offset(fresh3 as isize) = loc;
                state = -(1 as libc::c_int);
                start = i;
            } else {
                state = -(1 as libc::c_int);
                start = i;
            }
        }
        i += 1;
    }
    *ominmax_val = minmax_val;
    *ominmax_type = minmax_type;
    *ominmax_i = minmax_i;
    *ominmax_alloc = minmax_alloc;
    *ominmax_num = minmax_num;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn distance(
    x1: libc::c_int,
    y1: libc::c_int,
    x2: libc::c_int,
    y2: libc::c_int,
) -> libc::c_double {
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut dist: libc::c_double = 0.;
    dx = (x1 - x2) as libc::c_double;
    dy = (y1 - y2) as libc::c_double;
    dist = dx * dx + dy * dy;
    dist = sqrt(dist);
    return dist;
}
#[no_mangle]
pub unsafe extern "C" fn squared_distance(
    x1: libc::c_int,
    y1: libc::c_int,
    x2: libc::c_int,
    y2: libc::c_int,
) -> libc::c_double {
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut dist: libc::c_double = 0.;
    dx = (x1 - x2) as libc::c_double;
    dy = (y1 - y2) as libc::c_double;
    dist = dx * dx + dy * dy;
    return dist;
}
#[no_mangle]
pub unsafe extern "C" fn in_int_list(
    item: libc::c_int,
    mut list: *const libc::c_int,
    len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        if *list.offset(i as isize) == item {
            return i;
        }
        i += 1;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn find_incr_position_dbl(
    val: libc::c_double,
    mut list: *mut libc::c_double,
    num: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num {
        if val < *list.offset(i as isize) {
            return i;
        }
        i += 1;
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn angle2line(
    fx: libc::c_int,
    fy: libc::c_int,
    tx: libc::c_int,
    ty: libc::c_int,
) -> libc::c_double {
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut theta: libc::c_double = 0.;
    dy = (fy - ty) as libc::c_double;
    dx = (tx - fx) as libc::c_double;
    if fabs(dx) < 0.5f64 && fabs(dy) < 0.5f64 {
        theta = 0.0f64;
    } else {
        theta = atan2(dy, dx);
    }
    return theta;
}
#[no_mangle]
pub unsafe extern "C" fn line2direction(
    fx: libc::c_int,
    fy: libc::c_int,
    tx: libc::c_int,
    ty: libc::c_int,
    ndirs: libc::c_int,
) -> libc::c_int {
    let mut theta: libc::c_double = 0.;
    let mut pi_factor: libc::c_double = 0.;
    let mut idir: libc::c_int = 0;
    let mut full_ndirs: libc::c_int = 0;
    static mut pi2: libc::c_double = 3.14159265358979323846f64 * 2.0f64;
    theta = angle2line(ty, tx, fy, fx);
    theta += pi2;
    theta = fmod(theta, pi2);
    full_ndirs = ndirs << 1 as libc::c_int;
    pi_factor = full_ndirs as libc::c_double / pi2;
    theta *= pi_factor;
    theta = if theta < 0.0f64 {
        (theta * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
    } else {
        (theta * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
    };
    idir = (if theta < 0 as libc::c_int as libc::c_double {
        theta - 0.5f64
    } else {
        theta + 0.5f64
    }) as libc::c_int;
    idir %= full_ndirs;
    return idir;
}
#[no_mangle]
pub unsafe extern "C" fn closest_dir_dist(
    dir1: libc::c_int,
    dir2: libc::c_int,
    ndirs: libc::c_int,
) -> libc::c_int {
    let mut d1: libc::c_int = 0;
    let mut d2: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    dist = -(1 as libc::c_int);
    if dir1 >= 0 as libc::c_int && dir2 >= 0 as libc::c_int {
        d1 = abs(dir2 - dir1);
        d2 = ndirs - d1;
        dist = if d1 < d2 { d1 } else { d2 };
    }
    return dist;
}
