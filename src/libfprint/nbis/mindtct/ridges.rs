use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
    fn squared_distance(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn find_incr_position_dbl(
        _: libc::c_double,
        _: *mut libc::c_double,
        _: libc::c_int,
    ) -> libc::c_int;
    fn angle2line(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn bubble_sort_double_inc_2(
        _: *mut libc::c_double,
        _: *mut libc::c_int,
        _: libc::c_int,
    );
    fn free_contour(
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    );
    fn trace_contour(
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn fix_edge_pixel_pair(
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn line_points(
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn sort_minutiae_x_y(
        _: *mut MINUTIAE,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn rm_dup_minutiae(_: *mut MINUTIAE) -> libc::c_int;
    fn print2log(_: *mut libc::c_char, _: ...);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct g_lfsparms {
    pub pad_value: libc::c_int,
    pub join_line_radius: libc::c_int,
    pub blocksize: libc::c_int,
    pub windowsize: libc::c_int,
    pub windowoffset: libc::c_int,
    pub num_directions: libc::c_int,
    pub start_dir_angle: libc::c_double,
    pub rmv_valid_nbr_min: libc::c_int,
    pub dir_strength_min: libc::c_double,
    pub dir_distance_max: libc::c_int,
    pub smth_valid_nbr_min: libc::c_int,
    pub vort_valid_nbr_min: libc::c_int,
    pub highcurv_vorticity_min: libc::c_int,
    pub highcurv_curvature_min: libc::c_int,
    pub min_interpolate_nbrs: libc::c_int,
    pub percentile_min_max: libc::c_int,
    pub min_contrast_delta: libc::c_int,
    pub num_dft_waves: libc::c_int,
    pub powmax_min: libc::c_double,
    pub pownorm_min: libc::c_double,
    pub powmax_max: libc::c_double,
    pub fork_interval: libc::c_int,
    pub fork_pct_powmax: libc::c_double,
    pub fork_pct_pownorm: libc::c_double,
    pub dirbin_grid_w: libc::c_int,
    pub dirbin_grid_h: libc::c_int,
    pub isobin_grid_dim: libc::c_int,
    pub num_fill_holes: libc::c_int,
    pub max_minutia_delta: libc::c_int,
    pub max_high_curve_theta: libc::c_double,
    pub high_curve_half_contour: libc::c_int,
    pub min_loop_len: libc::c_int,
    pub min_loop_aspect_dist: libc::c_double,
    pub min_loop_aspect_ratio: libc::c_double,
    pub link_table_dim: libc::c_int,
    pub max_link_dist: libc::c_int,
    pub min_theta_dist: libc::c_int,
    pub maxtrans: libc::c_int,
    pub score_theta_norm: libc::c_double,
    pub score_dist_norm: libc::c_double,
    pub score_dist_weight: libc::c_double,
    pub score_numerator: libc::c_double,
    pub max_rmtest_dist: libc::c_int,
    pub max_hook_len: libc::c_int,
    pub max_half_loop: libc::c_int,
    pub trans_dir_pix: libc::c_int,
    pub small_loop_len: libc::c_int,
    pub side_half_contour: libc::c_int,
    pub inv_block_margin: libc::c_int,
    pub rm_valid_nbr_min: libc::c_int,
    pub max_overlap_dist: libc::c_int,
    pub max_overlap_join_dist: libc::c_int,
    pub malformation_steps_1: libc::c_int,
    pub malformation_steps_2: libc::c_int,
    pub min_malformation_ratio: libc::c_double,
    pub max_malformation_dist: libc::c_int,
    pub pores_trans_r: libc::c_int,
    pub pores_perp_steps: libc::c_int,
    pub pores_steps_fwd: libc::c_int,
    pub pores_steps_bwd: libc::c_int,
    pub pores_min_dist2: libc::c_double,
    pub pores_max_ratio: libc::c_double,
    pub remove_perimeter_pts: libc::c_int,
    pub min_pp_distance: libc::c_int,
    pub max_nbrs: libc::c_int,
    pub max_ridge_steps: libc::c_int,
}
pub type LFSPARMS = g_lfsparms;
#[no_mangle]
pub unsafe extern "C" fn count_minutiae_ridges(
    mut minutiae: *mut MINUTIAE,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    print2log(
        b"\nFINDING NBRS AND COUNTING RIDGES:\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    ret = sort_minutiae_x_y(minutiae, iw, ih);
    if ret != 0 {
        return ret;
    }
    ret = rm_dup_minutiae(minutiae);
    if ret != 0 {
        return ret;
    }
    i = 0 as libc::c_int;
    while i < (*minutiae).num - 1 as libc::c_int {
        ret = count_minutia_ridges(i, minutiae, bdata, iw, ih, lfsparms);
        if ret != 0 {
            return ret;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn count_minutia_ridges(
    first: libc::c_int,
    mut minutiae: *mut MINUTIAE,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut nbr_list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nbr_nridges: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nnbrs: libc::c_int = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*lfsparms).max_nbrs > 0 as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/ridges.c\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"count_minutia_ridges\0"))
                .as_ptr(),
            b"lfsparms->max_nbrs > 0\0" as *const u8 as *const libc::c_char,
        );
    }
    nbr_list = 0 as *mut libc::c_int;
    ret = find_neighbors(
        &mut nbr_list,
        &mut nnbrs,
        (*lfsparms).max_nbrs,
        first,
        minutiae,
    );
    if ret != 0 {
        if !nbr_list.is_null() {
            g_free(nbr_list as gpointer);
        }
        return ret;
    }
    print2log(
        b"NBRS FOUND: %d,%d = %d\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (**((*minutiae).list).offset(first as isize)).x,
        (**((*minutiae).list).offset(first as isize)).y,
        nnbrs,
    );
    if nnbrs == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    ret = sort_neighbors(nbr_list, nnbrs, first, minutiae);
    if ret != 0 {
        g_free(nbr_list as gpointer);
        return ret;
    }
    nbr_nridges = g_malloc(
        (nnbrs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < nnbrs {
        ret = ridge_count(
            first,
            *nbr_list.offset(i as isize),
            minutiae,
            bdata,
            iw,
            ih,
            lfsparms,
        );
        if ret < 0 as libc::c_int {
            g_free(nbr_list as gpointer);
            g_free(nbr_nridges as gpointer);
            return ret;
        }
        *nbr_nridges.offset(i as isize) = ret;
        i += 1;
    }
    let ref mut fresh0 = (**((*minutiae).list).offset(first as isize)).nbrs;
    *fresh0 = nbr_list;
    let ref mut fresh1 = (**((*minutiae).list).offset(first as isize)).ridge_counts;
    *fresh1 = nbr_nridges;
    (**((*minutiae).list).offset(first as isize)).num_nbrs = nnbrs;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn find_neighbors(
    mut onbr_list: *mut *mut libc::c_int,
    mut onnbrs: *mut libc::c_int,
    max_nbrs: libc::c_int,
    first: libc::c_int,
    mut minutiae: *mut MINUTIAE,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut second: libc::c_int = 0;
    let mut last_nbr: libc::c_int = 0;
    let mut minutia1: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut minutia2: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut nbr_list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nnbrs: libc::c_int = 0;
    let mut nbr_sqr_dists: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut xdist: libc::c_double = 0.;
    let mut xdist2: libc::c_double = 0.;
    nbr_list = g_malloc(
        (max_nbrs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    nbr_sqr_dists = g_malloc(
        (max_nbrs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    nnbrs = 0 as libc::c_int;
    second = first + 1 as libc::c_int;
    last_nbr = max_nbrs - 1 as libc::c_int;
    while second < (*minutiae).num {
        minutia1 = *((*minutiae).list).offset(first as isize);
        minutia2 = *((*minutiae).list).offset(second as isize);
        xdist = ((*minutia2).x - (*minutia1).x) as libc::c_double;
        xdist2 = xdist * xdist;
        if !(nnbrs < max_nbrs || xdist2 < *nbr_sqr_dists.offset(last_nbr as isize)) {
            break;
        }
        ret = update_nbr_dists(
            nbr_list,
            nbr_sqr_dists,
            &mut nnbrs,
            max_nbrs,
            first,
            second,
            minutiae,
        );
        if ret != 0 {
            g_free(nbr_sqr_dists as gpointer);
            g_free(nbr_list as gpointer);
            return ret;
        }
        second += 1;
    }
    g_free(nbr_sqr_dists as gpointer);
    if nnbrs == 0 as libc::c_int {
        g_free(nbr_list as gpointer);
        *onnbrs = 0 as libc::c_int;
    } else {
        *onbr_list = nbr_list;
        *onnbrs = nnbrs;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn update_nbr_dists(
    mut nbr_list: *mut libc::c_int,
    mut nbr_sqr_dists: *mut libc::c_double,
    mut nnbrs: *mut libc::c_int,
    max_nbrs: libc::c_int,
    first: libc::c_int,
    second: libc::c_int,
    mut minutiae: *mut MINUTIAE,
) -> libc::c_int {
    let mut dist2: libc::c_double = 0.;
    let mut minutia1: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut minutia2: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut pos: libc::c_int = 0;
    let mut last_nbr: libc::c_int = 0;
    last_nbr = max_nbrs - 1 as libc::c_int;
    minutia1 = *((*minutiae).list).offset(first as isize);
    minutia2 = *((*minutiae).list).offset(second as isize);
    dist2 = squared_distance((*minutia1).x, (*minutia1).y, (*minutia2).x, (*minutia2).y);
    if *nnbrs < max_nbrs || dist2 < *nbr_sqr_dists.offset(last_nbr as isize) {
        pos = find_incr_position_dbl(dist2, nbr_sqr_dists, *nnbrs);
        if pos >= max_nbrs {
            fprintf(
                stderr,
                b"ERROR : update_nbr_dists : illegal position for new neighbor\n\0"
                    as *const u8 as *const libc::c_char,
            );
            return -(470 as libc::c_int);
        }
        if insert_neighbor(pos, second, dist2, nbr_list, nbr_sqr_dists, nnbrs, max_nbrs)
            != 0
        {
            return -(471 as libc::c_int);
        }
        return 0 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn insert_neighbor(
    pos: libc::c_int,
    nbr_index: libc::c_int,
    nbr_dist2: libc::c_double,
    mut nbr_list: *mut libc::c_int,
    mut nbr_sqr_dists: *mut libc::c_double,
    mut nnbrs: *mut libc::c_int,
    max_nbrs: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if pos >= 0 as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/ridges.c\0" as *const u8 as *const libc::c_char,
            412 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"insert_neighbor\0"))
                .as_ptr(),
            b"pos >= 0\0" as *const u8 as *const libc::c_char,
        );
    }
    if pos > *nnbrs || pos >= max_nbrs {
        fprintf(
            stderr,
            b"ERROR : insert_neighbor : insertion point exceeds lists\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(480 as libc::c_int);
    }
    if *nnbrs < max_nbrs {
        i = *nnbrs - 1 as libc::c_int;
        *nnbrs += 1;
    } else if *nnbrs == max_nbrs {
        i = *nnbrs - 2 as libc::c_int;
    } else {
        fprintf(
            stderr,
            b"ERROR : insert_neighbor : overflow in neighbor lists\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(481 as libc::c_int);
    }
    while i >= pos {
        *nbr_list.offset((i + 1 as libc::c_int) as isize) = *nbr_list.offset(i as isize);
        *nbr_sqr_dists
            .offset((i + 1 as libc::c_int) as isize) = *nbr_sqr_dists.offset(i as isize);
        i -= 1;
    }
    *nbr_list.offset(pos as isize) = nbr_index;
    *nbr_sqr_dists.offset(pos as isize) = nbr_dist2;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sort_neighbors(
    mut nbr_list: *mut libc::c_int,
    nnbrs: libc::c_int,
    first: libc::c_int,
    mut minutiae: *mut MINUTIAE,
) -> libc::c_int {
    let mut join_thetas: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut theta: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    static mut pi2: libc::c_double = 3.14159265358979323846f64 * 2.0f64;
    join_thetas = g_malloc(
        (nnbrs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < nnbrs {
        theta = angle2line(
            (**((*minutiae).list).offset(*nbr_list.offset(i as isize) as isize)).y,
            (**((*minutiae).list).offset(*nbr_list.offset(i as isize) as isize)).x,
            (**((*minutiae).list).offset(first as isize)).y,
            (**((*minutiae).list).offset(first as isize)).x,
        );
        theta += pi2;
        theta = fmod(theta, pi2);
        *join_thetas.offset(i as isize) = theta;
        i += 1;
    }
    bubble_sort_double_inc_2(join_thetas, nbr_list, nnbrs);
    g_free(join_thetas as gpointer);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ridge_count(
    first: libc::c_int,
    second: libc::c_int,
    mut minutiae: *mut MINUTIAE,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut minutia1: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut minutia2: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut xlist: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ylist: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut num: libc::c_int = 0;
    let mut ridge_count_0: libc::c_int = 0;
    let mut ridge_start: libc::c_int = 0;
    let mut ridge_end: libc::c_int = 0;
    let mut prevpix: libc::c_int = 0;
    let mut curpix: libc::c_int = 0;
    minutia1 = *((*minutiae).list).offset(first as isize);
    minutia2 = *((*minutiae).list).offset(second as isize);
    if (*minutia1).x == (*minutia2).x && (*minutia1).y == (*minutia2).y {
        return 0 as libc::c_int;
    }
    ret = line_points(
        &mut xlist,
        &mut ylist,
        &mut num,
        (*minutia1).x,
        (*minutia1).y,
        (*minutia2).x,
        (*minutia2).y,
    );
    if ret != 0 {
        return ret;
    }
    if num == 0 as libc::c_int {
        g_free(xlist as gpointer);
        g_free(ylist as gpointer);
        return 0 as libc::c_int;
    }
    prevpix = *bdata
        .offset((*ylist.offset(0 as libc::c_int as isize) * iw) as isize)
        .offset(*xlist.offset(0 as libc::c_int as isize) as isize) as libc::c_int;
    i = 1 as libc::c_int;
    found = 0 as libc::c_int;
    while i < num {
        curpix = *bdata
            .offset((*ylist.offset(i as isize) * iw) as isize)
            .offset(*xlist.offset(i as isize) as isize) as libc::c_int;
        if curpix != prevpix {
            found = (0 as libc::c_int == 0) as libc::c_int;
            break;
        } else {
            i += 1;
        }
    }
    if found == 0 {
        g_free(xlist as gpointer);
        g_free(ylist as gpointer);
        return 0 as libc::c_int;
    }
    ridge_count_0 = 0 as libc::c_int;
    print2log(
        b"RIDGE COUNT: %d,%d to %d,%d \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*minutia1).x,
        (*minutia1).y,
        (*minutia2).x,
        (*minutia2).y,
    );
    while i < num {
        if find_transition(
            &mut i,
            0 as libc::c_int,
            1 as libc::c_int,
            xlist,
            ylist,
            num,
            bdata,
            iw,
            ih,
        ) == 0
        {
            g_free(xlist as gpointer);
            g_free(ylist as gpointer);
            print2log(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            return ridge_count_0;
        }
        ridge_start = i;
        print2log(
            b": RS %d,%d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            *xlist.offset(i as isize),
            *ylist.offset(i as isize),
        );
        if find_transition(
            &mut i,
            1 as libc::c_int,
            0 as libc::c_int,
            xlist,
            ylist,
            num,
            bdata,
            iw,
            ih,
        ) == 0
        {
            g_free(xlist as gpointer);
            g_free(ylist as gpointer);
            print2log(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            return ridge_count_0;
        }
        ridge_end = i;
        print2log(
            b"; RE %d,%d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            *xlist.offset(i as isize),
            *ylist.offset(i as isize),
        );
        ret = validate_ridge_crossing(
            ridge_start,
            ridge_end,
            xlist,
            ylist,
            num,
            bdata,
            iw,
            ih,
            (*lfsparms).max_ridge_steps,
        );
        if ret < 0 as libc::c_int {
            g_free(xlist as gpointer);
            g_free(ylist as gpointer);
            return ret;
        }
        print2log(
            b"; V%d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ret,
        );
        if ret != 0 {
            ridge_count_0 += 1;
        }
    }
    g_free(xlist as gpointer);
    g_free(ylist as gpointer);
    print2log(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    return ridge_count_0;
}
#[no_mangle]
pub unsafe extern "C" fn find_transition(
    mut iptr: *mut libc::c_int,
    pix1: libc::c_int,
    pix2: libc::c_int,
    mut xlist: *const libc::c_int,
    mut ylist: *const libc::c_int,
    num: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = *iptr;
    j = i + 1 as libc::c_int;
    while i < num - 1 as libc::c_int {
        if *bdata
            .offset((*ylist.offset(i as isize) * iw) as isize)
            .offset(*xlist.offset(i as isize) as isize) as libc::c_int == pix1
            && *bdata
                .offset((*ylist.offset(j as isize) * iw) as isize)
                .offset(*xlist.offset(j as isize) as isize) as libc::c_int == pix2
        {
            *iptr = j;
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        i += 1;
        j += 1;
    }
    *iptr = num;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn validate_ridge_crossing(
    ridge_start: libc::c_int,
    ridge_end: libc::c_int,
    mut xlist: *const libc::c_int,
    mut ylist: *const libc::c_int,
    num: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    max_ridge_steps: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut feat_x: libc::c_int = 0;
    let mut feat_y: libc::c_int = 0;
    let mut edge_x: libc::c_int = 0;
    let mut edge_y: libc::c_int = 0;
    let mut contour_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ncontour: libc::c_int = 0;
    feat_x = *xlist.offset(ridge_end as isize);
    feat_y = *ylist.offset(ridge_end as isize);
    edge_x = *xlist.offset((ridge_end - 1 as libc::c_int) as isize);
    edge_y = *ylist.offset((ridge_end - 1 as libc::c_int) as isize);
    fix_edge_pixel_pair(
        &mut feat_x,
        &mut feat_y,
        &mut edge_x,
        &mut edge_y,
        bdata,
        iw,
        ih,
    );
    ret = trace_contour(
        &mut contour_x,
        &mut contour_y,
        &mut contour_ex,
        &mut contour_ey,
        &mut ncontour,
        max_ridge_steps,
        *xlist.offset((ridge_start - 1 as libc::c_int) as isize),
        *ylist.offset((ridge_start - 1 as libc::c_int) as isize),
        feat_x,
        feat_y,
        edge_x,
        edge_y,
        0 as libc::c_int,
        bdata,
        iw,
        ih,
    );
    if ret < 0 as libc::c_int {
        return ret;
    }
    if ret != 2 as libc::c_int {
        free_contour(contour_x, contour_y, contour_ex, contour_ey);
    }
    if ret != 2 as libc::c_int && ret != 1 as libc::c_int {
        ret = trace_contour(
            &mut contour_x,
            &mut contour_y,
            &mut contour_ex,
            &mut contour_ey,
            &mut ncontour,
            max_ridge_steps,
            *xlist.offset((ridge_start - 1 as libc::c_int) as isize),
            *ylist.offset((ridge_start - 1 as libc::c_int) as isize),
            feat_x,
            feat_y,
            edge_x,
            edge_y,
            1 as libc::c_int,
            bdata,
            iw,
            ih,
        );
        if ret < 0 as libc::c_int {
            return ret;
        }
        if ret != 2 as libc::c_int {
            free_contour(contour_x, contour_y, contour_ex, contour_ey);
        }
        if ret != 2 as libc::c_int && ret != 1 as libc::c_int {
            return (0 as libc::c_int == 0) as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
