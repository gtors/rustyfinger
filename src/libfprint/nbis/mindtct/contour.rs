use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn minv(_: *const libc::c_int, _: libc::c_int) -> libc::c_int;
    fn maxv(_: *const libc::c_int, _: libc::c_int) -> libc::c_int;
    fn angle2line(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    static mut g_nbr8_dx: [libc::c_int; 0];
    static mut g_nbr8_dy: [libc::c_int; 0];
    fn abs(_: libc::c_int) -> libc::c_int;
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
pub type gsize = libc::c_ulong;
pub type gpointer = *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn allocate_contour(
    mut ocontour_x: *mut *mut libc::c_int,
    mut ocontour_y: *mut *mut libc::c_int,
    mut ocontour_ex: *mut *mut libc::c_int,
    mut ocontour_ey: *mut *mut libc::c_int,
    ncontour: libc::c_int,
) -> libc::c_int {
    let mut contour_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dest: gsize = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        let (fresh0, fresh1) = ncontour
            .overflowing_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
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
            b"../libfprint/nbis/mindtct/contour.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"allocate_contour\0"))
                .as_ptr(),
            b"g_size_checked_mul(&dest, ncontour, sizeof(int))\0" as *const u8
                as *const libc::c_char,
        );
    }
    contour_x = g_malloc(
        (ncontour as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    contour_y = g_malloc(
        (ncontour as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    contour_ex = g_malloc(
        (ncontour as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    contour_ey = g_malloc(
        (ncontour as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    *ocontour_x = contour_x;
    *ocontour_y = contour_y;
    *ocontour_ex = contour_ex;
    *ocontour_ey = contour_ey;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn free_contour(
    mut contour_x: *mut libc::c_int,
    mut contour_y: *mut libc::c_int,
    mut contour_ex: *mut libc::c_int,
    mut contour_ey: *mut libc::c_int,
) {
    g_free(contour_x as gpointer);
    g_free(contour_y as gpointer);
    g_free(contour_ex as gpointer);
    g_free(contour_ey as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn get_high_curvature_contour(
    mut ocontour_x: *mut *mut libc::c_int,
    mut ocontour_y: *mut *mut libc::c_int,
    mut ocontour_ex: *mut *mut libc::c_int,
    mut ocontour_ey: *mut *mut libc::c_int,
    mut oncontour: *mut libc::c_int,
    half_contour: libc::c_int,
    x_loc: libc::c_int,
    y_loc: libc::c_int,
    x_edge: libc::c_int,
    y_edge: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) -> libc::c_int {
    let mut max_contour: libc::c_int = 0;
    let mut half1_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut half1_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut half1_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut half1_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nhalf1: libc::c_int = 0;
    let mut half2_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut half2_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut half2_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut half2_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nhalf2: libc::c_int = 0;
    let mut contour_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ncontour: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    max_contour = (half_contour << 1 as libc::c_int) + 1 as libc::c_int;
    *oncontour = 0 as libc::c_int;
    ret = trace_contour(
        &mut half1_x,
        &mut half1_y,
        &mut half1_ex,
        &mut half1_ey,
        &mut nhalf1,
        half_contour,
        x_loc,
        y_loc,
        x_loc,
        y_loc,
        x_edge,
        y_edge,
        0 as libc::c_int,
        bdata,
        iw,
        ih,
    );
    if ret != 0 {
        if ret == 2 as libc::c_int {
            return 0 as libc::c_int;
        }
        if ret == 1 as libc::c_int {
            ncontour = nhalf1 + 1 as libc::c_int;
            ret = allocate_contour(
                &mut contour_x,
                &mut contour_y,
                &mut contour_ex,
                &mut contour_ey,
                ncontour,
            );
            if ret != 0 {
                free_contour(half1_x, half1_y, half1_ex, half1_ey);
                return ret;
            }
            *contour_x.offset(0 as libc::c_int as isize) = x_loc;
            *contour_y.offset(0 as libc::c_int as isize) = y_loc;
            *contour_ex.offset(0 as libc::c_int as isize) = x_edge;
            *contour_ey.offset(0 as libc::c_int as isize) = y_edge;
            i = 1 as libc::c_int;
            j = nhalf1 - 1 as libc::c_int;
            while i < ncontour {
                *contour_x.offset(i as isize) = *half1_x.offset(j as isize);
                *contour_y.offset(i as isize) = *half1_y.offset(j as isize);
                *contour_ex.offset(i as isize) = *half1_ex.offset(j as isize);
                *contour_ey.offset(i as isize) = *half1_ey.offset(j as isize);
                i += 1;
                j -= 1;
            }
            free_contour(half1_x, half1_y, half1_ex, half1_ey);
            *ocontour_x = contour_x;
            *ocontour_y = contour_y;
            *ocontour_ex = contour_ex;
            *ocontour_ey = contour_ey;
            *oncontour = ncontour;
            return 1 as libc::c_int;
        }
        return ret;
    }
    if nhalf1 < half_contour {
        free_contour(half1_x, half1_y, half1_ex, half1_ey);
        return 0 as libc::c_int;
    }
    ret = trace_contour(
        &mut half2_x,
        &mut half2_y,
        &mut half2_ex,
        &mut half2_ey,
        &mut nhalf2,
        half_contour,
        *half1_x.offset((nhalf1 - 1 as libc::c_int) as isize),
        *half1_y.offset((nhalf1 - 1 as libc::c_int) as isize),
        x_loc,
        y_loc,
        x_edge,
        y_edge,
        1 as libc::c_int,
        bdata,
        iw,
        ih,
    );
    if ret != 0 {
        if ret == 2 as libc::c_int {
            free_contour(half1_x, half1_y, half1_ex, half1_ey);
            return 0 as libc::c_int;
        }
        if ret != 1 as libc::c_int {
            free_contour(half1_x, half1_y, half1_ex, half1_ey);
            return ret;
        }
    }
    if ret != 1 as libc::c_int && nhalf2 < half_contour {
        free_contour(half1_x, half1_y, half1_ex, half1_ey);
        free_contour(half2_x, half2_y, half2_ex, half2_ey);
        return 0 as libc::c_int;
    }
    ret = allocate_contour(
        &mut contour_x,
        &mut contour_y,
        &mut contour_ex,
        &mut contour_ey,
        max_contour,
    );
    if ret != 0 {
        free_contour(half1_x, half1_y, half1_ex, half1_ey);
        free_contour(half2_x, half2_y, half2_ex, half2_ey);
        return ret;
    }
    ncontour = 0 as libc::c_int;
    i = 0 as libc::c_int;
    j = nhalf1 - 1 as libc::c_int;
    while i < nhalf1 {
        *contour_x.offset(i as isize) = *half1_x.offset(j as isize);
        *contour_y.offset(i as isize) = *half1_y.offset(j as isize);
        *contour_ex.offset(i as isize) = *half1_ex.offset(j as isize);
        *contour_ey.offset(i as isize) = *half1_ey.offset(j as isize);
        ncontour += 1;
        i += 1;
        j -= 1;
    }
    free_contour(half1_x, half1_y, half1_ex, half1_ey);
    *contour_x.offset(nhalf1 as isize) = x_loc;
    *contour_y.offset(nhalf1 as isize) = y_loc;
    *contour_ex.offset(nhalf1 as isize) = x_edge;
    *contour_ey.offset(nhalf1 as isize) = y_edge;
    ncontour += 1;
    i = 0 as libc::c_int;
    j = nhalf1 + 1 as libc::c_int;
    while i < nhalf2 {
        *contour_x.offset(j as isize) = *half2_x.offset(i as isize);
        *contour_y.offset(j as isize) = *half2_y.offset(i as isize);
        *contour_ex.offset(j as isize) = *half2_ex.offset(i as isize);
        *contour_ey.offset(j as isize) = *half2_ey.offset(i as isize);
        ncontour += 1;
        i += 1;
        j += 1;
    }
    free_contour(half2_x, half2_y, half2_ex, half2_ey);
    *ocontour_x = contour_x;
    *ocontour_y = contour_y;
    *ocontour_ex = contour_ex;
    *ocontour_ey = contour_ey;
    *oncontour = ncontour;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn get_centered_contour(
    mut ocontour_x: *mut *mut libc::c_int,
    mut ocontour_y: *mut *mut libc::c_int,
    mut ocontour_ex: *mut *mut libc::c_int,
    mut ocontour_ey: *mut *mut libc::c_int,
    mut oncontour: *mut libc::c_int,
    half_contour: libc::c_int,
    x_loc: libc::c_int,
    y_loc: libc::c_int,
    x_edge: libc::c_int,
    y_edge: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) -> libc::c_int {
    let mut max_contour: libc::c_int = 0;
    let mut half1_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut half1_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut half1_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut half1_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nhalf1: libc::c_int = 0;
    let mut half2_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut half2_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut half2_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut half2_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nhalf2: libc::c_int = 0;
    let mut contour_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ncontour: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if half_contour > 0 as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/contour.c\0" as *const u8 as *const libc::c_char,
            443 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"get_centered_contour\0"))
                .as_ptr(),
            b"half_contour > 0\0" as *const u8 as *const libc::c_char,
        );
    }
    max_contour = (half_contour << 1 as libc::c_int) + 1 as libc::c_int;
    *oncontour = 0 as libc::c_int;
    ret = trace_contour(
        &mut half1_x,
        &mut half1_y,
        &mut half1_ex,
        &mut half1_ey,
        &mut nhalf1,
        half_contour,
        x_loc,
        y_loc,
        x_loc,
        y_loc,
        x_edge,
        y_edge,
        0 as libc::c_int,
        bdata,
        iw,
        ih,
    );
    if ret < 0 as libc::c_int {
        return ret;
    }
    if ret == 2 as libc::c_int {
        return 2 as libc::c_int;
    }
    if ret == 1 as libc::c_int {
        free_contour(half1_x, half1_y, half1_ex, half1_ey);
        return 1 as libc::c_int;
    }
    if nhalf1 < half_contour {
        free_contour(half1_x, half1_y, half1_ex, half1_ey);
        return 3 as libc::c_int;
    }
    ret = trace_contour(
        &mut half2_x,
        &mut half2_y,
        &mut half2_ex,
        &mut half2_ey,
        &mut nhalf2,
        half_contour,
        *half1_x.offset((nhalf1 - 1 as libc::c_int) as isize),
        *half1_y.offset((nhalf1 - 1 as libc::c_int) as isize),
        x_loc,
        y_loc,
        x_edge,
        y_edge,
        1 as libc::c_int,
        bdata,
        iw,
        ih,
    );
    if ret < 0 as libc::c_int {
        free_contour(half1_x, half1_y, half1_ex, half1_ey);
        return ret;
    }
    if ret == 2 as libc::c_int {
        free_contour(half1_x, half1_y, half1_ex, half1_ey);
        return 2 as libc::c_int;
    }
    if ret == 1 as libc::c_int {
        free_contour(half1_x, half1_y, half1_ex, half1_ey);
        free_contour(half2_x, half2_y, half2_ex, half2_ey);
        return 1 as libc::c_int;
    }
    if nhalf2 < half_contour {
        free_contour(half1_x, half1_y, half1_ex, half1_ey);
        free_contour(half2_x, half2_y, half2_ex, half2_ey);
        return 3 as libc::c_int;
    }
    ret = allocate_contour(
        &mut contour_x,
        &mut contour_y,
        &mut contour_ex,
        &mut contour_ey,
        max_contour,
    );
    if ret != 0 {
        free_contour(half1_x, half1_y, half1_ex, half1_ey);
        free_contour(half2_x, half2_y, half2_ex, half2_ey);
        return ret;
    }
    ncontour = 0 as libc::c_int;
    i = 0 as libc::c_int;
    j = nhalf1 - 1 as libc::c_int;
    while i < nhalf1 {
        *contour_x.offset(i as isize) = *half1_x.offset(j as isize);
        *contour_y.offset(i as isize) = *half1_y.offset(j as isize);
        *contour_ex.offset(i as isize) = *half1_ex.offset(j as isize);
        *contour_ey.offset(i as isize) = *half1_ey.offset(j as isize);
        ncontour += 1;
        i += 1;
        j -= 1;
    }
    free_contour(half1_x, half1_y, half1_ex, half1_ey);
    *contour_x.offset(nhalf1 as isize) = x_loc;
    *contour_y.offset(nhalf1 as isize) = y_loc;
    *contour_ex.offset(nhalf1 as isize) = x_edge;
    *contour_ey.offset(nhalf1 as isize) = y_edge;
    ncontour += 1;
    i = 0 as libc::c_int;
    j = nhalf1 + 1 as libc::c_int;
    while i < nhalf2 {
        *contour_x.offset(j as isize) = *half2_x.offset(i as isize);
        *contour_y.offset(j as isize) = *half2_y.offset(i as isize);
        *contour_ex.offset(j as isize) = *half2_ex.offset(i as isize);
        *contour_ey.offset(j as isize) = *half2_ey.offset(i as isize);
        ncontour += 1;
        i += 1;
        j += 1;
    }
    free_contour(half2_x, half2_y, half2_ex, half2_ey);
    *ocontour_x = contour_x;
    *ocontour_y = contour_y;
    *ocontour_ex = contour_ex;
    *ocontour_ey = contour_ey;
    *oncontour = ncontour;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trace_contour(
    mut ocontour_x: *mut *mut libc::c_int,
    mut ocontour_y: *mut *mut libc::c_int,
    mut ocontour_ex: *mut *mut libc::c_int,
    mut ocontour_ey: *mut *mut libc::c_int,
    mut oncontour: *mut libc::c_int,
    max_len: libc::c_int,
    x_loop: libc::c_int,
    y_loop: libc::c_int,
    x_loc: libc::c_int,
    y_loc: libc::c_int,
    x_edge: libc::c_int,
    y_edge: libc::c_int,
    scan_clock: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) -> libc::c_int {
    let mut contour_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ncontour: libc::c_int = 0;
    let mut cur_x_loc: libc::c_int = 0;
    let mut cur_y_loc: libc::c_int = 0;
    let mut cur_x_edge: libc::c_int = 0;
    let mut cur_y_edge: libc::c_int = 0;
    let mut next_x_loc: libc::c_int = 0;
    let mut next_y_loc: libc::c_int = 0;
    let mut next_x_edge: libc::c_int = 0;
    let mut next_y_edge: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if *bdata.offset((y_loc * iw) as isize).offset(x_loc as isize) as libc::c_int
        == *bdata.offset((y_edge * iw) as isize).offset(x_edge as isize) as libc::c_int
    {
        return 2 as libc::c_int;
    }
    ret = allocate_contour(
        &mut contour_x,
        &mut contour_y,
        &mut contour_ex,
        &mut contour_ey,
        max_len,
    );
    if ret != 0 {
        return ret;
    }
    ncontour = 0 as libc::c_int;
    cur_x_loc = x_loc;
    cur_y_loc = y_loc;
    cur_x_edge = x_edge;
    cur_y_edge = y_edge;
    i = 0 as libc::c_int;
    while i < max_len {
        if next_contour_pixel(
            &mut next_x_loc,
            &mut next_y_loc,
            &mut next_x_edge,
            &mut next_y_edge,
            cur_x_loc,
            cur_y_loc,
            cur_x_edge,
            cur_y_edge,
            scan_clock,
            bdata,
            iw,
            ih,
        ) != 0
        {
            if next_x_loc == x_loop && next_y_loc == y_loop {
                *ocontour_x = contour_x;
                *ocontour_y = contour_y;
                *ocontour_ex = contour_ex;
                *ocontour_ey = contour_ey;
                *oncontour = ncontour;
                return 1 as libc::c_int;
            }
            *contour_x.offset(i as isize) = next_x_loc;
            *contour_y.offset(i as isize) = next_y_loc;
            *contour_ex.offset(i as isize) = next_x_edge;
            *contour_ey.offset(i as isize) = next_y_edge;
            ncontour += 1;
            cur_x_loc = next_x_loc;
            cur_y_loc = next_y_loc;
            cur_x_edge = next_x_edge;
            cur_y_edge = next_y_edge;
        } else {
            *ocontour_x = contour_x;
            *ocontour_y = contour_y;
            *ocontour_ex = contour_ex;
            *ocontour_ey = contour_ey;
            *oncontour = ncontour;
            return 0 as libc::c_int;
        }
        i += 1;
    }
    *ocontour_x = contour_x;
    *ocontour_y = contour_y;
    *ocontour_ex = contour_ex;
    *ocontour_ey = contour_ey;
    *oncontour = ncontour;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn search_contour(
    x_search: libc::c_int,
    y_search: libc::c_int,
    search_len: libc::c_int,
    x_loc: libc::c_int,
    y_loc: libc::c_int,
    x_edge: libc::c_int,
    y_edge: libc::c_int,
    scan_clock: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) -> libc::c_int {
    let mut cur_x_loc: libc::c_int = 0;
    let mut cur_y_loc: libc::c_int = 0;
    let mut cur_x_edge: libc::c_int = 0;
    let mut cur_y_edge: libc::c_int = 0;
    let mut next_x_loc: libc::c_int = 0;
    let mut next_y_loc: libc::c_int = 0;
    let mut next_x_edge: libc::c_int = 0;
    let mut next_y_edge: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    cur_x_loc = x_loc;
    cur_y_loc = y_loc;
    cur_x_edge = x_edge;
    cur_y_edge = y_edge;
    i = 0 as libc::c_int;
    while i < search_len {
        if next_contour_pixel(
            &mut next_x_loc,
            &mut next_y_loc,
            &mut next_x_edge,
            &mut next_y_edge,
            cur_x_loc,
            cur_y_loc,
            cur_x_edge,
            cur_y_edge,
            scan_clock,
            bdata,
            iw,
            ih,
        ) != 0
        {
            if next_x_loc == x_search && next_y_loc == y_search {
                return (0 as libc::c_int == 0) as libc::c_int;
            }
            cur_x_loc = next_x_loc;
            cur_y_loc = next_y_loc;
            cur_x_edge = next_x_edge;
            cur_y_edge = next_y_edge;
        } else {
            return 0 as libc::c_int
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn next_contour_pixel(
    mut next_x_loc: *mut libc::c_int,
    mut next_y_loc: *mut libc::c_int,
    mut next_x_edge: *mut libc::c_int,
    mut next_y_edge: *mut libc::c_int,
    cur_x_loc: libc::c_int,
    cur_y_loc: libc::c_int,
    cur_x_edge: libc::c_int,
    cur_y_edge: libc::c_int,
    scan_clock: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) -> libc::c_int {
    let mut feature_pix: libc::c_int = 0;
    let mut edge_pix: libc::c_int = 0;
    let mut prev_nbr_pix: libc::c_int = 0;
    let mut prev_nbr_x: libc::c_int = 0;
    let mut prev_nbr_y: libc::c_int = 0;
    let mut cur_nbr_pix: libc::c_int = 0;
    let mut cur_nbr_x: libc::c_int = 0;
    let mut cur_nbr_y: libc::c_int = 0;
    let mut ni: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut ny: libc::c_int = 0;
    let mut npix: libc::c_int = 0;
    let mut nbr_i: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    feature_pix = *bdata.offset((cur_y_loc * iw) as isize).offset(cur_x_loc as isize)
        as libc::c_int;
    edge_pix = *bdata.offset((cur_y_edge * iw) as isize).offset(cur_x_edge as isize)
        as libc::c_int;
    nbr_i = start_scan_nbr(cur_x_loc, cur_y_loc, cur_x_edge, cur_y_edge);
    cur_nbr_x = cur_x_edge;
    cur_nbr_y = cur_y_edge;
    cur_nbr_pix = edge_pix;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        prev_nbr_x = cur_nbr_x;
        prev_nbr_y = cur_nbr_y;
        prev_nbr_pix = cur_nbr_pix;
        nbr_i = next_scan_nbr(nbr_i, scan_clock);
        cur_nbr_x = cur_x_loc + *g_nbr8_dx.as_mut_ptr().offset(nbr_i as isize);
        cur_nbr_y = cur_y_loc + *g_nbr8_dy.as_mut_ptr().offset(nbr_i as isize);
        if cur_nbr_x < 0 as libc::c_int || cur_nbr_x >= iw
            || cur_nbr_y < 0 as libc::c_int || cur_nbr_y >= ih
        {
            return 0 as libc::c_int;
        }
        cur_nbr_pix = *bdata.offset((cur_nbr_y * iw) as isize).offset(cur_nbr_x as isize)
            as libc::c_int;
        if cur_nbr_pix == feature_pix && prev_nbr_pix == edge_pix {
            if nbr_i % 2 as libc::c_int != 0 {
                ni = next_scan_nbr(nbr_i, scan_clock);
                nx = cur_x_loc + *g_nbr8_dx.as_mut_ptr().offset(ni as isize);
                ny = cur_y_loc + *g_nbr8_dy.as_mut_ptr().offset(ni as isize);
                if nx < 0 as libc::c_int || nx >= iw || ny < 0 as libc::c_int || ny >= ih
                {
                    return 0 as libc::c_int;
                }
                npix = *bdata.offset((ny * iw) as isize).offset(nx as isize)
                    as libc::c_int;
                if npix == feature_pix {
                    *next_x_loc = cur_nbr_x;
                    *next_y_loc = cur_nbr_y;
                    *next_x_edge = prev_nbr_x;
                    *next_y_edge = prev_nbr_y;
                    return (0 as libc::c_int == 0) as libc::c_int;
                } else {
                    cur_nbr_x = nx;
                    cur_nbr_y = ny;
                    cur_nbr_pix = npix;
                    nbr_i = ni;
                    i += 1;
                }
            } else {
                *next_x_loc = cur_nbr_x;
                *next_y_loc = cur_nbr_y;
                *next_x_edge = prev_nbr_x;
                *next_y_edge = prev_nbr_y;
                return (0 as libc::c_int == 0) as libc::c_int;
            }
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn start_scan_nbr(
    x_prev: libc::c_int,
    y_prev: libc::c_int,
    x_next: libc::c_int,
    y_next: libc::c_int,
) -> libc::c_int {
    if x_prev == x_next && y_next > y_prev {
        return 4 as libc::c_int
    } else {
        if x_prev == x_next && y_next < y_prev {
            return 0 as libc::c_int
        } else {
            if x_next > x_prev && y_prev == y_next {
                return 2 as libc::c_int
            } else {
                if x_next < x_prev && y_prev == y_next {
                    return 6 as libc::c_int;
                }
            }
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn next_scan_nbr(
    nbr_i: libc::c_int,
    scan_clock: libc::c_int,
) -> libc::c_int {
    let mut new_i: libc::c_int = 0;
    if scan_clock == 0 as libc::c_int {
        new_i = (nbr_i + 1 as libc::c_int) % 8 as libc::c_int;
    } else {
        new_i = (nbr_i + 7 as libc::c_int) % 8 as libc::c_int;
    }
    return new_i;
}
#[no_mangle]
pub unsafe extern "C" fn min_contour_theta(
    mut omin_i: *mut libc::c_int,
    mut omin_theta: *mut libc::c_double,
    angle_edge: libc::c_int,
    mut contour_x: *const libc::c_int,
    mut contour_y: *const libc::c_int,
    ncontour: libc::c_int,
) -> libc::c_int {
    let mut pleft: libc::c_int = 0;
    let mut pcenter: libc::c_int = 0;
    let mut pright: libc::c_int = 0;
    let mut theta1: libc::c_double = 0.;
    let mut theta2: libc::c_double = 0.;
    let mut dtheta: libc::c_double = 0.;
    let mut min_i: libc::c_int = 0;
    let mut min_theta: libc::c_double = 0.;
    if ncontour < (angle_edge << 1 as libc::c_int) + 1 as libc::c_int {
        return 2 as libc::c_int;
    }
    min_theta = 3.14159265358979323846f64;
    min_theta = if min_theta < 0.0f64 {
        (min_theta * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
    } else {
        (min_theta * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
    };
    min_i = -(1 as libc::c_int);
    pleft = 0 as libc::c_int;
    pcenter = angle_edge;
    pright = pcenter + angle_edge;
    while pright < ncontour {
        theta1 = angle2line(
            *contour_x.offset(pcenter as isize),
            *contour_y.offset(pcenter as isize),
            *contour_x.offset(pleft as isize),
            *contour_y.offset(pleft as isize),
        );
        theta2 = angle2line(
            *contour_x.offset(pcenter as isize),
            *contour_y.offset(pcenter as isize),
            *contour_x.offset(pright as isize),
            *contour_y.offset(pright as isize),
        );
        dtheta = fabs(theta2 - theta1);
        dtheta = if dtheta < 3.14159265358979323846f64 * 2.0f64 - dtheta {
            dtheta
        } else {
            3.14159265358979323846f64 * 2.0f64 - dtheta
        };
        dtheta = if dtheta < 0.0f64 {
            (dtheta * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
        } else {
            (dtheta * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
        };
        if dtheta < min_theta {
            min_i = pcenter;
            min_theta = dtheta;
        }
        pleft += 1;
        pcenter += 1;
        pright += 1;
    }
    if min_i == -(1 as libc::c_int) {
        *omin_i = ncontour >> 1 as libc::c_int;
        *omin_theta = min_theta;
    } else {
        *omin_i = min_i;
        *omin_theta = min_theta;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn contour_limits(
    mut xmin: *mut libc::c_int,
    mut ymin: *mut libc::c_int,
    mut xmax: *mut libc::c_int,
    mut ymax: *mut libc::c_int,
    mut contour_x: *const libc::c_int,
    mut contour_y: *const libc::c_int,
    ncontour: libc::c_int,
) {
    *xmin = minv(contour_x, ncontour);
    *ymin = minv(contour_y, ncontour);
    *xmax = maxv(contour_x, ncontour);
    *ymax = maxv(contour_y, ncontour);
}
#[no_mangle]
pub unsafe extern "C" fn fix_edge_pixel_pair(
    mut feat_x: *mut libc::c_int,
    mut feat_y: *mut libc::c_int,
    mut edge_x: *mut libc::c_int,
    mut edge_y: *mut libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) {
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut px: libc::c_int = 0;
    let mut py: libc::c_int = 0;
    let mut cx: libc::c_int = 0;
    let mut cy: libc::c_int = 0;
    let mut feature_pix: libc::c_int = 0;
    feature_pix = *bdata.offset((*feat_y * iw) as isize).offset(*feat_x as isize)
        as libc::c_int;
    cx = *feat_x;
    cy = *feat_y;
    px = *edge_x;
    py = *edge_y;
    dx = px - cx;
    dy = py - cy;
    if abs(dx) == 1 as libc::c_int && abs(dy) == 1 as libc::c_int {
        if *bdata.offset((py * iw) as isize).offset((px - dx) as isize) as libc::c_int
            != feature_pix
        {
            px -= dx;
        } else if *bdata.offset(((py - dy) * iw) as isize).offset(px as isize)
            as libc::c_int != feature_pix
        {
            py -= dy;
        } else {
            cy += dy;
        }
        *feat_x = cx;
        *feat_y = cy;
        *edge_x = px;
        *edge_y = py;
    }
}
