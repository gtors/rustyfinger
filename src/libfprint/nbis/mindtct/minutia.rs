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
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn free_contour(
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    );
    fn get_high_curvature_contour(
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
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn search_contour(
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
    fn min_contour_theta(
        _: *mut libc::c_int,
        _: *mut libc::c_double,
        _: libc::c_int,
        _: *const libc::c_int,
        _: *const libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn is_loop_clockwise(
        _: *const libc::c_int,
        _: *const libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn process_loop_V2(
        _: *mut MINUTIAE,
        _: *const libc::c_int,
        _: *const libc::c_int,
        _: *const libc::c_int,
        _: *const libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: *const LFSPARMS,
    ) -> libc::c_int;
    fn pixelize_map(
        _: *mut *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn match_1st_pair(
        _: libc::c_uchar,
        _: libc::c_uchar,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn match_2nd_pair(
        _: libc::c_uchar,
        _: libc::c_uchar,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn match_3rd_pair(
        _: libc::c_uchar,
        _: libc::c_uchar,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn skip_repeated_horizontal_pair(
        _: *mut libc::c_int,
        _: libc::c_int,
        _: *mut *mut libc::c_uchar,
        _: *mut *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn skip_repeated_vertical_pair(
        _: *mut libc::c_int,
        _: libc::c_int,
        _: *mut *mut libc::c_uchar,
        _: *mut *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
    );
    static mut g_feature_patterns: [FEATURE_PATTERN; 0];
    fn line2direction(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn sort_indices_int_inc(
        _: *mut *mut libc::c_int,
        _: *mut libc::c_int,
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
pub unsafe extern "C" fn alloc_minutiae(
    mut ominutiae: *mut *mut MINUTIAE,
    DEFAULT_BOZORTH_MINUTIAE: libc::c_int,
) -> libc::c_int {
    let mut minutiae: *mut MINUTIAE = 0 as *mut MINUTIAE;
    minutiae = g_malloc(::core::mem::size_of::<MINUTIAE>() as libc::c_ulong)
        as *mut MINUTIAE;
    (*minutiae)
        .list = g_malloc(
        (DEFAULT_BOZORTH_MINUTIAE as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut MINUTIA>() as libc::c_ulong),
    ) as *mut *mut MINUTIA;
    (*minutiae).alloc = DEFAULT_BOZORTH_MINUTIAE;
    (*minutiae).num = 0 as libc::c_int;
    *ominutiae = minutiae;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn realloc_minutiae(
    mut minutiae: *mut MINUTIAE,
    incr_minutiae: libc::c_int,
) -> libc::c_int {
    (*minutiae).alloc += incr_minutiae;
    (*minutiae)
        .list = g_realloc(
        (*minutiae).list as gpointer,
        ((*minutiae).alloc as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut MINUTIA>() as libc::c_ulong),
    ) as *mut *mut MINUTIA;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn detect_minutiae_V2(
    mut minutiae: *mut MINUTIAE,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut direction_map: *mut libc::c_int,
    mut low_flow_map: *mut libc::c_int,
    mut high_curve_map: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut pdirection_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut plow_flow_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut phigh_curve_map: *mut libc::c_int = 0 as *mut libc::c_int;
    ret = pixelize_map(
        &mut pdirection_map,
        iw,
        ih,
        direction_map,
        mw,
        mh,
        (*lfsparms).blocksize,
    );
    if ret != 0 {
        return ret;
    }
    ret = pixelize_map(
        &mut plow_flow_map,
        iw,
        ih,
        low_flow_map,
        mw,
        mh,
        (*lfsparms).blocksize,
    );
    if ret != 0 {
        g_free(pdirection_map as gpointer);
        return ret;
    }
    ret = pixelize_map(
        &mut phigh_curve_map,
        iw,
        ih,
        high_curve_map,
        mw,
        mh,
        (*lfsparms).blocksize,
    );
    if ret != 0 {
        g_free(pdirection_map as gpointer);
        g_free(plow_flow_map as gpointer);
        return ret;
    }
    ret = scan4minutiae_horizontally_V2(
        minutiae,
        bdata,
        iw,
        ih,
        pdirection_map,
        plow_flow_map,
        phigh_curve_map,
        lfsparms,
    );
    if ret != 0 {
        g_free(pdirection_map as gpointer);
        g_free(plow_flow_map as gpointer);
        g_free(phigh_curve_map as gpointer);
        return ret;
    }
    ret = scan4minutiae_vertically_V2(
        minutiae,
        bdata,
        iw,
        ih,
        pdirection_map,
        plow_flow_map,
        phigh_curve_map,
        lfsparms,
    );
    if ret != 0 {
        g_free(pdirection_map as gpointer);
        g_free(plow_flow_map as gpointer);
        g_free(phigh_curve_map as gpointer);
        return ret;
    }
    g_free(pdirection_map as gpointer);
    g_free(plow_flow_map as gpointer);
    g_free(phigh_curve_map as gpointer);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn update_minutiae(
    mut minutiae: *mut MINUTIAE,
    mut minutia: *mut MINUTIA,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut delta_dir: libc::c_int = 0;
    let mut qtr_ndirs: libc::c_int = 0;
    let mut full_ndirs: libc::c_int = 0;
    if (*minutiae).num >= (*minutiae).alloc {
        ret = realloc_minutiae(minutiae, 1000 as libc::c_int);
        if ret != 0 {
            return ret;
        }
    }
    qtr_ndirs = (*lfsparms).num_directions >> 2 as libc::c_int;
    full_ndirs = (*lfsparms).num_directions << 1 as libc::c_int;
    if (*minutiae).num > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*minutiae).num {
            dx = abs((**((*minutiae).list).offset(i as isize)).x - (*minutia).x);
            if dx < (*lfsparms).max_minutia_delta {
                dy = abs((**((*minutiae).list).offset(i as isize)).y - (*minutia).y);
                if dy < (*lfsparms).max_minutia_delta {
                    if (**((*minutiae).list).offset(i as isize)).type_0
                        == (*minutia).type_0
                    {
                        delta_dir = abs(
                            (**((*minutiae).list).offset(i as isize)).direction
                                - (*minutia).direction,
                        );
                        delta_dir = if delta_dir < full_ndirs - delta_dir {
                            delta_dir
                        } else {
                            full_ndirs - delta_dir
                        };
                        if delta_dir <= qtr_ndirs {
                            if dx == 0 as libc::c_int && dy == 0 as libc::c_int {
                                return 2 as libc::c_int;
                            }
                            if search_contour(
                                (*minutia).x,
                                (*minutia).y,
                                (*lfsparms).max_minutia_delta,
                                (**((*minutiae).list).offset(i as isize)).x,
                                (**((*minutiae).list).offset(i as isize)).y,
                                (**((*minutiae).list).offset(i as isize)).ex,
                                (**((*minutiae).list).offset(i as isize)).ey,
                                0 as libc::c_int,
                                bdata,
                                iw,
                                ih,
                            ) != 0
                            {
                                return 2 as libc::c_int;
                            }
                            if search_contour(
                                (*minutia).x,
                                (*minutia).y,
                                (*lfsparms).max_minutia_delta,
                                (**((*minutiae).list).offset(i as isize)).x,
                                (**((*minutiae).list).offset(i as isize)).y,
                                (**((*minutiae).list).offset(i as isize)).ex,
                                (**((*minutiae).list).offset(i as isize)).ey,
                                1 as libc::c_int,
                                bdata,
                                iw,
                                ih,
                            ) != 0
                            {
                                return 2 as libc::c_int;
                            }
                        }
                    }
                }
            }
            i += 1;
        }
    }
    let ref mut fresh0 = *((*minutiae).list).offset((*minutiae).num as isize);
    *fresh0 = minutia;
    (*minutiae).num += 1;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn update_minutiae_V2(
    mut minutiae: *mut MINUTIAE,
    mut minutia: *mut MINUTIA,
    scan_dir: libc::c_int,
    dmapval: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut delta_dir: libc::c_int = 0;
    let mut qtr_ndirs: libc::c_int = 0;
    let mut full_ndirs: libc::c_int = 0;
    let mut map_scan_dir: libc::c_int = 0;
    if (*minutiae).num >= (*minutiae).alloc {
        ret = realloc_minutiae(minutiae, 1000 as libc::c_int);
        if ret != 0 {
            return ret;
        }
    }
    qtr_ndirs = (*lfsparms).num_directions >> 2 as libc::c_int;
    full_ndirs = (*lfsparms).num_directions << 1 as libc::c_int;
    if (*minutiae).num > 0 as libc::c_int {
        i = (*minutiae).num - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            dx = abs((**((*minutiae).list).offset(i as isize)).x - (*minutia).x);
            if dx < (*lfsparms).max_minutia_delta {
                dy = abs((**((*minutiae).list).offset(i as isize)).y - (*minutia).y);
                if dy < (*lfsparms).max_minutia_delta {
                    if (**((*minutiae).list).offset(i as isize)).type_0
                        == (*minutia).type_0
                    {
                        delta_dir = abs(
                            (**((*minutiae).list).offset(i as isize)).direction
                                - (*minutia).direction,
                        );
                        delta_dir = if delta_dir < full_ndirs - delta_dir {
                            delta_dir
                        } else {
                            full_ndirs - delta_dir
                        };
                        if delta_dir <= qtr_ndirs {
                            if dx == 0 as libc::c_int && dy == 0 as libc::c_int {
                                return 2 as libc::c_int;
                            }
                            if search_contour(
                                (*minutia).x,
                                (*minutia).y,
                                (*lfsparms).max_minutia_delta,
                                (**((*minutiae).list).offset(i as isize)).x,
                                (**((*minutiae).list).offset(i as isize)).y,
                                (**((*minutiae).list).offset(i as isize)).ex,
                                (**((*minutiae).list).offset(i as isize)).ey,
                                0 as libc::c_int,
                                bdata,
                                iw,
                                ih,
                            ) != 0
                                || search_contour(
                                    (*minutia).x,
                                    (*minutia).y,
                                    (*lfsparms).max_minutia_delta,
                                    (**((*minutiae).list).offset(i as isize)).x,
                                    (**((*minutiae).list).offset(i as isize)).y,
                                    (**((*minutiae).list).offset(i as isize)).ex,
                                    (**((*minutiae).list).offset(i as isize)).ey,
                                    1 as libc::c_int,
                                    bdata,
                                    iw,
                                    ih,
                                ) != 0
                            {
                                if dmapval >= 0 as libc::c_int {
                                    map_scan_dir = choose_scan_direction(
                                        dmapval,
                                        (*lfsparms).num_directions,
                                    );
                                    if map_scan_dir == scan_dir {
                                        ret = remove_minutia(i, minutiae);
                                        if ret != 0 {
                                            return ret;
                                        }
                                    } else {
                                        return 2 as libc::c_int
                                    }
                                } else {
                                    return 2 as libc::c_int
                                }
                            }
                        }
                    }
                }
            }
            i -= 1;
        }
    }
    let ref mut fresh1 = *((*minutiae).list).offset((*minutiae).num as isize);
    *fresh1 = minutia;
    (*minutiae).num += 1;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sort_minutiae_y_x(
    mut minutiae: *mut MINUTIAE,
    iw: libc::c_int,
    ih: libc::c_int,
) -> libc::c_int {
    let mut ranks: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut order: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut newlist: *mut *mut MINUTIA = 0 as *mut *mut MINUTIA;
    ranks = g_malloc(
        ((*minutiae).num as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*minutiae).num {
        *ranks
            .offset(
                i as isize,
            ) = (**((*minutiae).list).offset(i as isize)).y * iw
            + (**((*minutiae).list).offset(i as isize)).x;
        i += 1;
    }
    ret = sort_indices_int_inc(&mut order, ranks, (*minutiae).num);
    if ret != 0 {
        g_free(ranks as gpointer);
        return ret;
    }
    newlist = g_malloc(
        ((*minutiae).num as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut MINUTIA>() as libc::c_ulong),
    ) as *mut *mut MINUTIA;
    i = 0 as libc::c_int;
    while i < (*minutiae).num {
        let ref mut fresh2 = *newlist.offset(i as isize);
        *fresh2 = *((*minutiae).list).offset(*order.offset(i as isize) as isize);
        i += 1;
    }
    g_free((*minutiae).list as gpointer);
    (*minutiae).list = newlist;
    g_free(order as gpointer);
    g_free(ranks as gpointer);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sort_minutiae_x_y(
    mut minutiae: *mut MINUTIAE,
    iw: libc::c_int,
    ih: libc::c_int,
) -> libc::c_int {
    let mut ranks: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut order: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut newlist: *mut *mut MINUTIA = 0 as *mut *mut MINUTIA;
    ranks = g_malloc(
        ((*minutiae).num as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*minutiae).num {
        *ranks
            .offset(
                i as isize,
            ) = (**((*minutiae).list).offset(i as isize)).x * iw
            + (**((*minutiae).list).offset(i as isize)).y;
        i += 1;
    }
    ret = sort_indices_int_inc(&mut order, ranks, (*minutiae).num);
    if ret != 0 {
        g_free(ranks as gpointer);
        return ret;
    }
    newlist = g_malloc(
        ((*minutiae).num as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut MINUTIA>() as libc::c_ulong),
    ) as *mut *mut MINUTIA;
    i = 0 as libc::c_int;
    while i < (*minutiae).num {
        let ref mut fresh3 = *newlist.offset(i as isize);
        *fresh3 = *((*minutiae).list).offset(*order.offset(i as isize) as isize);
        i += 1;
    }
    g_free((*minutiae).list as gpointer);
    (*minutiae).list = newlist;
    g_free(order as gpointer);
    g_free(ranks as gpointer);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rm_dup_minutiae(mut minutiae: *mut MINUTIAE) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut minutia1: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut minutia2: *mut MINUTIA = 0 as *mut MINUTIA;
    i = (*minutiae).num - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        minutia1 = *((*minutiae).list).offset(i as isize);
        minutia2 = *((*minutiae).list).offset((i - 1 as libc::c_int) as isize);
        if (*minutia1).x == (*minutia2).x && (*minutia1).y == (*minutia2).y {
            ret = remove_minutia(i - 1 as libc::c_int, minutiae);
            if ret != 0 {
                return ret;
            }
        }
        i -= 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn create_minutia(
    mut ominutia: *mut *mut MINUTIA,
    x_loc: libc::c_int,
    y_loc: libc::c_int,
    x_edge: libc::c_int,
    y_edge: libc::c_int,
    idir: libc::c_int,
    reliability: libc::c_double,
    type_0: libc::c_int,
    appearing: libc::c_int,
    feature_id: libc::c_int,
) -> libc::c_int {
    let mut minutia: *mut MINUTIA = 0 as *mut MINUTIA;
    minutia = g_malloc(::core::mem::size_of::<MINUTIA>() as libc::c_ulong)
        as *mut MINUTIA;
    (*minutia).x = x_loc;
    (*minutia).y = y_loc;
    (*minutia).ex = x_edge;
    (*minutia).ey = y_edge;
    (*minutia).direction = idir;
    (*minutia).reliability = reliability;
    (*minutia).type_0 = type_0;
    (*minutia).appearing = appearing;
    (*minutia).feature_id = feature_id;
    (*minutia).nbrs = 0 as *mut libc::c_void as *mut libc::c_int;
    (*minutia).ridge_counts = 0 as *mut libc::c_void as *mut libc::c_int;
    (*minutia).num_nbrs = 0 as libc::c_int;
    *ominutia = minutia;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn free_minutiae(mut minutiae: *mut MINUTIAE) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*minutiae).num {
        free_minutia(*((*minutiae).list).offset(i as isize));
        i += 1;
    }
    g_free((*minutiae).list as gpointer);
    g_free(minutiae as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn free_minutia(mut minutia: *mut MINUTIA) {
    if !((*minutia).nbrs).is_null() {
        g_free((*minutia).nbrs as gpointer);
    }
    if !((*minutia).ridge_counts).is_null() {
        g_free((*minutia).ridge_counts as gpointer);
    }
    g_free(minutia as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn remove_minutia(
    index: libc::c_int,
    mut minutiae: *mut MINUTIAE,
) -> libc::c_int {
    let mut fr: libc::c_int = 0;
    let mut to: libc::c_int = 0;
    if index < 0 as libc::c_int && index >= (*minutiae).num {
        fprintf(
            stderr,
            b"ERROR : remove_minutia : index out of range\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(380 as libc::c_int);
    }
    free_minutia(*((*minutiae).list).offset(index as isize));
    to = index;
    fr = index + 1 as libc::c_int;
    while fr < (*minutiae).num {
        let ref mut fresh4 = *((*minutiae).list).offset(to as isize);
        *fresh4 = *((*minutiae).list).offset(fr as isize);
        to += 1;
        fr += 1;
    }
    (*minutiae).num -= 1;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn minutia_type(feature_pix: libc::c_int) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    if feature_pix == 0 as libc::c_int {
        type_0 = 0 as libc::c_int;
    } else {
        type_0 = 1 as libc::c_int;
    }
    return type_0;
}
#[no_mangle]
pub unsafe extern "C" fn is_minutia_appearing(
    x_loc: libc::c_int,
    y_loc: libc::c_int,
    x_edge: libc::c_int,
    y_edge: libc::c_int,
) -> libc::c_int {
    if x_edge < x_loc {
        return 1 as libc::c_int;
    }
    if x_edge > x_loc {
        return 0 as libc::c_int;
    }
    if y_edge < y_loc {
        return 1 as libc::c_int;
    }
    if y_edge > y_loc {
        return 0 as libc::c_int;
    }
    fprintf(
        stderr,
        b"ERROR : is_minutia_appearing : bad configuration of pixels\n\0" as *const u8
            as *const libc::c_char,
    );
    return -(240 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn choose_scan_direction(
    imapval: libc::c_int,
    ndirs: libc::c_int,
) -> libc::c_int {
    let mut qtr_ndirs: libc::c_int = 0;
    qtr_ndirs = ndirs >> 2 as libc::c_int;
    if imapval <= qtr_ndirs || imapval > qtr_ndirs * 3 as libc::c_int {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn scan4minutiae_horizontally_V2(
    mut minutiae: *mut MINUTIAE,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut pdirection_map: *mut libc::c_int,
    mut plow_flow_map: *mut libc::c_int,
    mut phigh_curve_map: *mut libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut sx: libc::c_int = 0;
    let mut sy: libc::c_int = 0;
    let mut ex: libc::c_int = 0;
    let mut ey: libc::c_int = 0;
    let mut cx: libc::c_int = 0;
    let mut cy: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut p1ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p2ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut possible: [libc::c_int; 10] = [0; 10];
    let mut nposs: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    sx = 0 as libc::c_int;
    ex = iw;
    sy = 0 as libc::c_int;
    ey = ih;
    cy = sy;
    while (cy + 1 as libc::c_int) < ey {
        cx = sx;
        while cx < ex {
            p1ptr = bdata.offset((cy * iw) as isize).offset(cx as isize);
            p2ptr = bdata
                .offset(((cy + 1 as libc::c_int) * iw) as isize)
                .offset(cx as isize);
            if match_1st_pair(*p1ptr, *p2ptr, possible.as_mut_ptr(), &mut nposs) != 0 {
                cx += 1;
                p1ptr = p1ptr.offset(1);
                p2ptr = p2ptr.offset(1);
                if cx < ex {
                    if match_2nd_pair(*p1ptr, *p2ptr, possible.as_mut_ptr(), &mut nposs)
                        != 0
                    {
                        x2 = cx;
                        skip_repeated_horizontal_pair(
                            &mut cx,
                            ex,
                            &mut p1ptr,
                            &mut p2ptr,
                            iw,
                            ih,
                        );
                        if cx < ex {
                            if match_3rd_pair(
                                *p1ptr,
                                *p2ptr,
                                possible.as_mut_ptr(),
                                &mut nposs,
                            ) != 0
                            {
                                ret = process_horizontal_scan_minutia_V2(
                                    minutiae,
                                    cx,
                                    cy,
                                    x2,
                                    possible[0 as libc::c_int as usize],
                                    bdata,
                                    iw,
                                    ih,
                                    pdirection_map,
                                    plow_flow_map,
                                    phigh_curve_map,
                                    lfsparms,
                                );
                                if ret != 0 {
                                    if ret < 0 as libc::c_int {
                                        return ret;
                                    }
                                }
                            }
                            if *p1ptr as libc::c_int != *p2ptr as libc::c_int {
                                cx -= 1;
                            }
                        }
                    }
                }
            } else {
                cx += 1;
            }
        }
        cy += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scan4minutiae_vertically_V2(
    mut minutiae: *mut MINUTIAE,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut pdirection_map: *mut libc::c_int,
    mut plow_flow_map: *mut libc::c_int,
    mut phigh_curve_map: *mut libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut sx: libc::c_int = 0;
    let mut sy: libc::c_int = 0;
    let mut ex: libc::c_int = 0;
    let mut ey: libc::c_int = 0;
    let mut cx: libc::c_int = 0;
    let mut cy: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut p1ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p2ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut possible: [libc::c_int; 10] = [0; 10];
    let mut nposs: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    sx = 0 as libc::c_int;
    ex = iw;
    sy = 0 as libc::c_int;
    ey = ih;
    cx = sx;
    while (cx + 1 as libc::c_int) < ex {
        cy = sy;
        while cy < ey {
            p1ptr = bdata.offset((cy * iw) as isize).offset(cx as isize);
            p2ptr = p1ptr.offset(1 as libc::c_int as isize);
            if match_1st_pair(*p1ptr, *p2ptr, possible.as_mut_ptr(), &mut nposs) != 0 {
                cy += 1;
                p1ptr = p1ptr.offset(iw as isize);
                p2ptr = p2ptr.offset(iw as isize);
                if cy < ey {
                    if match_2nd_pair(*p1ptr, *p2ptr, possible.as_mut_ptr(), &mut nposs)
                        != 0
                    {
                        y2 = cy;
                        skip_repeated_vertical_pair(
                            &mut cy,
                            ey,
                            &mut p1ptr,
                            &mut p2ptr,
                            iw,
                            ih,
                        );
                        if cy < ey {
                            if match_3rd_pair(
                                *p1ptr,
                                *p2ptr,
                                possible.as_mut_ptr(),
                                &mut nposs,
                            ) != 0
                            {
                                ret = process_vertical_scan_minutia_V2(
                                    minutiae,
                                    cx,
                                    cy,
                                    y2,
                                    possible[0 as libc::c_int as usize],
                                    bdata,
                                    iw,
                                    ih,
                                    pdirection_map,
                                    plow_flow_map,
                                    phigh_curve_map,
                                    lfsparms,
                                );
                                if ret != 0 {
                                    if ret < 0 as libc::c_int {
                                        return ret;
                                    }
                                }
                            }
                            if *p1ptr as libc::c_int != *p2ptr as libc::c_int {
                                cy -= 1;
                            }
                        }
                    }
                }
            } else {
                cy += 1;
            }
        }
        cx += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn process_horizontal_scan_minutia_V2(
    mut minutiae: *mut MINUTIAE,
    cx: libc::c_int,
    cy: libc::c_int,
    x2: libc::c_int,
    feature_id: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut pdirection_map: *mut libc::c_int,
    mut plow_flow_map: *mut libc::c_int,
    mut phigh_curve_map: *mut libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut minutia: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut x_loc: libc::c_int = 0;
    let mut y_loc: libc::c_int = 0;
    let mut x_edge: libc::c_int = 0;
    let mut y_edge: libc::c_int = 0;
    let mut idir: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut dmapval: libc::c_int = 0;
    let mut fmapval: libc::c_int = 0;
    let mut cmapval: libc::c_int = 0;
    let mut reliability: libc::c_double = 0.;
    x_loc = cx + x2 >> 1 as libc::c_int;
    x_edge = x_loc;
    if (*g_feature_patterns.as_mut_ptr().offset(feature_id as isize)).appearing != 0 {
        y_loc = cy + 1 as libc::c_int;
        y_edge = cy;
    } else {
        y_loc = cy;
        y_edge = cy + 1 as libc::c_int;
    }
    dmapval = *pdirection_map.offset((y_loc * iw) as isize).offset(x_loc as isize);
    fmapval = *plow_flow_map.offset((y_loc * iw) as isize).offset(x_loc as isize);
    cmapval = *phigh_curve_map.offset((y_loc * iw) as isize).offset(x_loc as isize);
    if dmapval == -(1 as libc::c_int) {
        return 2 as libc::c_int;
    }
    if cmapval != 0 {
        ret = adjust_high_curvature_minutia_V2(
            &mut idir,
            &mut x_loc,
            &mut y_loc,
            &mut x_edge,
            &mut y_edge,
            x_loc,
            y_loc,
            x_edge,
            y_edge,
            bdata,
            iw,
            ih,
            plow_flow_map,
            minutiae,
            lfsparms,
        );
        if ret != 0 {
            return ret;
        }
    } else {
        idir = get_low_curvature_direction(
            0 as libc::c_int,
            (*g_feature_patterns.as_mut_ptr().offset(feature_id as isize)).appearing,
            dmapval,
            (*lfsparms).num_directions,
        );
    }
    if fmapval != 0 {
        reliability = 0.50f64;
    } else {
        reliability = 0.99f64;
    }
    ret = create_minutia(
        &mut minutia,
        x_loc,
        y_loc,
        x_edge,
        y_edge,
        idir,
        reliability,
        (*g_feature_patterns.as_mut_ptr().offset(feature_id as isize)).type_0,
        (*g_feature_patterns.as_mut_ptr().offset(feature_id as isize)).appearing,
        feature_id,
    );
    if ret != 0 {
        return ret;
    }
    ret = update_minutiae_V2(
        minutiae,
        minutia,
        0 as libc::c_int,
        dmapval,
        bdata,
        iw,
        ih,
        lfsparms,
    );
    if ret != 0 as libc::c_int {
        free_minutia(minutia);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn process_vertical_scan_minutia_V2(
    mut minutiae: *mut MINUTIAE,
    cx: libc::c_int,
    cy: libc::c_int,
    y2: libc::c_int,
    feature_id: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut pdirection_map: *mut libc::c_int,
    mut plow_flow_map: *mut libc::c_int,
    mut phigh_curve_map: *mut libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut minutia: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut x_loc: libc::c_int = 0;
    let mut y_loc: libc::c_int = 0;
    let mut x_edge: libc::c_int = 0;
    let mut y_edge: libc::c_int = 0;
    let mut idir: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut dmapval: libc::c_int = 0;
    let mut fmapval: libc::c_int = 0;
    let mut cmapval: libc::c_int = 0;
    let mut reliability: libc::c_double = 0.;
    if (*g_feature_patterns.as_mut_ptr().offset(feature_id as isize)).appearing != 0 {
        x_loc = cx + 1 as libc::c_int;
        x_edge = cx;
    } else {
        x_loc = cx;
        x_edge = cx + 1 as libc::c_int;
    }
    y_loc = cy + y2 >> 1 as libc::c_int;
    y_edge = y_loc;
    dmapval = *pdirection_map.offset((y_loc * iw) as isize).offset(x_loc as isize);
    fmapval = *plow_flow_map.offset((y_loc * iw) as isize).offset(x_loc as isize);
    cmapval = *phigh_curve_map.offset((y_loc * iw) as isize).offset(x_loc as isize);
    if dmapval == -(1 as libc::c_int) {
        return 2 as libc::c_int;
    }
    if cmapval != 0 {
        ret = adjust_high_curvature_minutia_V2(
            &mut idir,
            &mut x_loc,
            &mut y_loc,
            &mut x_edge,
            &mut y_edge,
            x_loc,
            y_loc,
            x_edge,
            y_edge,
            bdata,
            iw,
            ih,
            plow_flow_map,
            minutiae,
            lfsparms,
        );
        if ret != 0 {
            return ret;
        }
    } else {
        idir = get_low_curvature_direction(
            1 as libc::c_int,
            (*g_feature_patterns.as_mut_ptr().offset(feature_id as isize)).appearing,
            dmapval,
            (*lfsparms).num_directions,
        );
    }
    if fmapval != 0 {
        reliability = 0.50f64;
    } else {
        reliability = 0.99f64;
    }
    ret = create_minutia(
        &mut minutia,
        x_loc,
        y_loc,
        x_edge,
        y_edge,
        idir,
        reliability,
        (*g_feature_patterns.as_mut_ptr().offset(feature_id as isize)).type_0,
        (*g_feature_patterns.as_mut_ptr().offset(feature_id as isize)).appearing,
        feature_id,
    );
    if ret != 0 {
        return ret;
    }
    ret = update_minutiae_V2(
        minutiae,
        minutia,
        1 as libc::c_int,
        dmapval,
        bdata,
        iw,
        ih,
        lfsparms,
    );
    if ret != 0 as libc::c_int {
        free_minutia(minutia);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn adjust_high_curvature_minutia_V2(
    mut oidir: *mut libc::c_int,
    mut ox_loc: *mut libc::c_int,
    mut oy_loc: *mut libc::c_int,
    mut ox_edge: *mut libc::c_int,
    mut oy_edge: *mut libc::c_int,
    x_loc: libc::c_int,
    y_loc: libc::c_int,
    x_edge: libc::c_int,
    y_edge: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut plow_flow_map: *mut libc::c_int,
    mut minutiae: *mut MINUTIAE,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut contour_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ncontour: libc::c_int = 0;
    let mut min_i: libc::c_int = 0;
    let mut min_theta: libc::c_double = 0.;
    let mut feature_pix: libc::c_int = 0;
    let mut mid_x: libc::c_int = 0;
    let mut mid_y: libc::c_int = 0;
    let mut mid_pix: libc::c_int = 0;
    let mut idir: libc::c_int = 0;
    let mut half_contour: libc::c_int = 0;
    let mut angle_edge: libc::c_int = 0;
    half_contour = (*lfsparms).high_curve_half_contour;
    angle_edge = half_contour >> 1 as libc::c_int;
    feature_pix = *bdata.offset((y_loc * iw) as isize).offset(x_loc as isize)
        as libc::c_int;
    ret = get_high_curvature_contour(
        &mut contour_x,
        &mut contour_y,
        &mut contour_ex,
        &mut contour_ey,
        &mut ncontour,
        half_contour,
        x_loc,
        y_loc,
        x_edge,
        y_edge,
        bdata,
        iw,
        ih,
    );
    if ret != 0 {
        if ret == 1 as libc::c_int {
            ret = is_loop_clockwise(
                contour_x,
                contour_y,
                ncontour,
                (0 as libc::c_int == 0) as libc::c_int,
            );
            if ret != 0 {
                free_contour(contour_x, contour_y, contour_ex, contour_ey);
                if ret < 0 as libc::c_int {
                    return ret;
                }
                return 2 as libc::c_int;
            }
            ret = process_loop_V2(
                minutiae,
                contour_x,
                contour_y,
                contour_ex,
                contour_ey,
                ncontour,
                bdata,
                iw,
                ih,
                plow_flow_map,
                lfsparms,
            );
            free_contour(contour_x, contour_y, contour_ex, contour_ey);
            if ret == 0 as libc::c_int {
                return 2 as libc::c_int;
            }
            return ret;
        }
        return ret;
    }
    if ncontour == 0 as libc::c_int {
        return 2 as libc::c_int;
    }
    ret = min_contour_theta(
        &mut min_i,
        &mut min_theta,
        angle_edge,
        contour_x,
        contour_y,
        ncontour,
    );
    if ret != 0 {
        free_contour(contour_x, contour_y, contour_ex, contour_ey);
        return ret;
    }
    if min_theta >= (*lfsparms).max_high_curve_theta {
        free_contour(contour_x, contour_y, contour_ex, contour_ey);
        return 2 as libc::c_int;
    }
    mid_x = *contour_x.offset((min_i - angle_edge) as isize)
        + *contour_x.offset((min_i + angle_edge) as isize) >> 1 as libc::c_int;
    mid_y = *contour_y.offset((min_i - angle_edge) as isize)
        + *contour_y.offset((min_i + angle_edge) as isize) >> 1 as libc::c_int;
    mid_pix = *bdata.offset((mid_y * iw) as isize).offset(mid_x as isize) as libc::c_int;
    if mid_pix != feature_pix {
        free_contour(contour_x, contour_y, contour_ex, contour_ey);
        return 2 as libc::c_int;
    }
    idir = line2direction(
        *contour_x.offset(min_i as isize),
        *contour_y.offset(min_i as isize),
        mid_x,
        mid_y,
        (*lfsparms).num_directions,
    );
    *oidir = idir;
    *ox_loc = *contour_x.offset(min_i as isize);
    *oy_loc = *contour_y.offset(min_i as isize);
    *ox_edge = *contour_ex.offset(min_i as isize);
    *oy_edge = *contour_ey.offset(min_i as isize);
    free_contour(contour_x, contour_y, contour_ex, contour_ey);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_low_curvature_direction(
    scan_dir: libc::c_int,
    appearing: libc::c_int,
    imapval: libc::c_int,
    ndirs: libc::c_int,
) -> libc::c_int {
    let mut idir: libc::c_int = 0;
    idir = imapval;
    if imapval <= ndirs >> 1 as libc::c_int {
        if scan_dir == 0 as libc::c_int {
            if appearing != 0 {
                idir += ndirs;
            }
        } else if appearing == 0 {
            idir += ndirs;
        }
    } else if scan_dir == 0 as libc::c_int {
        if appearing == 0 {
            idir += ndirs;
        }
    } else if appearing == 0 {
        idir += ndirs;
    }
    return idir;
}
