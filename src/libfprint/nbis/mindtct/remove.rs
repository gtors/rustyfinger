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
    fn minmaxs(
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *const libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn closest_dir_dist(_: libc::c_int, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn line2direction(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn distance(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn squared_distance(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn remove_minutia(_: libc::c_int, _: *mut MINUTIAE) -> libc::c_int;
    fn sort_minutiae_y_x(
        _: *mut MINUTIAE,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn num_valid_8nbrs(
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn fill_loop(
        _: *const libc::c_int,
        _: *const libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn on_hook(
        _: *const MINUTIA,
        _: *const MINUTIA,
        _: libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn on_island_lake(
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut libc::c_int,
        _: *const MINUTIA,
        _: *const MINUTIA,
        _: libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn on_loop(
        _: *const MINUTIA,
        _: libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn line_points(
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn search_in_direction(
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn free_path(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: *const LFSPARMS,
    ) -> libc::c_int;
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
    fn get_centered_contour(
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
    fn free_contour(
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    );
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
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
pub unsafe extern "C" fn remove_false_minutia_V2(
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
    ret = sort_minutiae_y_x(minutiae, iw, ih);
    if ret != 0 {
        return ret;
    }
    ret = remove_islands_and_lakes(minutiae, bdata, iw, ih, lfsparms);
    if ret != 0 {
        return ret;
    }
    ret = remove_holes(minutiae, bdata, iw, ih, lfsparms);
    if ret != 0 {
        return ret;
    }
    ret = remove_pointing_invblock_V2(minutiae, direction_map, mw, mh, lfsparms);
    if ret != 0 {
        return ret;
    }
    ret = remove_near_invblock_V2(minutiae, direction_map, mw, mh, lfsparms);
    if ret != 0 {
        return ret;
    }
    ret = remove_or_adjust_side_minutiae_V2(
        minutiae,
        bdata,
        iw,
        ih,
        direction_map,
        mw,
        mh,
        lfsparms,
    );
    if ret != 0 {
        return ret;
    }
    ret = remove_hooks(minutiae, bdata, iw, ih, lfsparms);
    if ret != 0 {
        return ret;
    }
    ret = remove_overlaps(minutiae, bdata, iw, ih, lfsparms);
    if ret != 0 {
        return ret;
    }
    ret = remove_malformations(minutiae, bdata, iw, ih, low_flow_map, mw, mh, lfsparms);
    if ret != 0 {
        return ret;
    }
    ret = remove_pores_V2(
        minutiae,
        bdata,
        iw,
        ih,
        direction_map,
        low_flow_map,
        high_curve_map,
        mw,
        mh,
        lfsparms,
    );
    if ret != 0 {
        return ret;
    }
    ret = remove_perimeter_pts(minutiae, bdata, iw, ih, lfsparms);
    if ret != 0 {
        return ret;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn remove_holes(
    mut minutiae: *mut MINUTIAE,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut minutia: *mut MINUTIA = 0 as *mut MINUTIA;
    print2log(
        b"\nREMOVING HOLES:\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < (*minutiae).num {
        minutia = *((*minutiae).list).offset(i as isize);
        if (*minutia).type_0 == 0 as libc::c_int {
            ret = on_loop(minutia, (*lfsparms).small_loop_len, bdata, iw, ih);
            if ret == 1 as libc::c_int || ret == 2 as libc::c_int {
                print2log(
                    b"%d,%d RM\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*minutia).x,
                    (*minutia).y,
                );
                ret = remove_minutia(i, minutiae);
                if ret != 0 {
                    return ret;
                }
            } else if ret == 0 as libc::c_int {
                i += 1;
            } else {
                return ret
            }
        } else {
            i += 1;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn remove_hooks(
    mut minutiae: *mut MINUTIAE,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut to_remove: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut delta_y: libc::c_int = 0;
    let mut full_ndirs: libc::c_int = 0;
    let mut qtr_ndirs: libc::c_int = 0;
    let mut deltadir: libc::c_int = 0;
    let mut min_deltadir: libc::c_int = 0;
    let mut minutia1: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut minutia2: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut dist: libc::c_double = 0.;
    print2log(
        b"\nREMOVING HOOKS:\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    to_remove = calloc(
        (*minutiae).num as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if to_remove.is_null() {
        fprintf(
            stderr,
            b"ERROR : remove_hooks : calloc : to_remove\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(640 as libc::c_int);
    }
    full_ndirs = (*lfsparms).num_directions << 1 as libc::c_int;
    qtr_ndirs = (*lfsparms).num_directions >> 2 as libc::c_int;
    min_deltadir = 3 as libc::c_int * qtr_ndirs - 1 as libc::c_int;
    f = 0 as libc::c_int;
    while f < (*minutiae).num - 1 as libc::c_int {
        if *to_remove.offset(f as isize) == 0 {
            print2log(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            minutia1 = *((*minutiae).list).offset(f as isize);
            s = f + 1 as libc::c_int;
            while s < (*minutiae).num {
                minutia2 = *((*minutiae).list).offset(s as isize);
                print2log(
                    b"1:%d(%d,%d)%d 2:%d(%d,%d)%d \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    f,
                    (*minutia1).x,
                    (*minutia1).y,
                    (*minutia1).type_0,
                    s,
                    (*minutia2).x,
                    (*minutia2).y,
                    (*minutia2).type_0,
                );
                if *bdata
                    .offset(((*minutia1).y * iw) as isize)
                    .offset((*minutia1).x as isize) as libc::c_int != (*minutia1).type_0
                {
                    print2log(
                        b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    break;
                } else {
                    if *bdata
                        .offset(((*minutia2).y * iw) as isize)
                        .offset((*minutia2).x as isize) as libc::c_int
                        != (*minutia2).type_0
                    {
                        *to_remove
                            .offset(s as isize) = (0 as libc::c_int == 0) as libc::c_int;
                    }
                    if *to_remove.offset(s as isize) == 0 {
                        delta_y = (*minutia2).y - (*minutia1).y;
                        if delta_y <= (*lfsparms).max_rmtest_dist {
                            print2log(
                                b"1DY \0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            dist = distance(
                                (*minutia1).x,
                                (*minutia1).y,
                                (*minutia2).x,
                                (*minutia2).y,
                            );
                            if dist <= (*lfsparms).max_rmtest_dist as libc::c_double {
                                print2log(
                                    b"2DS \0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                                deltadir = closest_dir_dist(
                                    (*minutia1).direction,
                                    (*minutia2).direction,
                                    full_ndirs,
                                );
                                if deltadir == -(1 as libc::c_int) {
                                    g_free(to_remove as gpointer);
                                    fprintf(
                                        stderr,
                                        b"ERROR : remove_hooks : INVALID direction\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    return -(641 as libc::c_int);
                                }
                                if deltadir > min_deltadir {
                                    print2log(
                                        b"3DD \0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                    if (*minutia1).type_0 != (*minutia2).type_0 {
                                        ret = on_hook(
                                            minutia1,
                                            minutia2,
                                            (*lfsparms).max_hook_len,
                                            bdata,
                                            iw,
                                            ih,
                                        );
                                        if ret == 1 as libc::c_int {
                                            print2log(
                                                b"4HK RM\n\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                            );
                                            *to_remove
                                                .offset(
                                                    f as isize,
                                                ) = (0 as libc::c_int == 0) as libc::c_int;
                                            *to_remove
                                                .offset(
                                                    s as isize,
                                                ) = (0 as libc::c_int == 0) as libc::c_int;
                                        } else if ret == 2 as libc::c_int {
                                            print2log(
                                                b"RM\n\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                            );
                                            *to_remove
                                                .offset(
                                                    f as isize,
                                                ) = (0 as libc::c_int == 0) as libc::c_int;
                                            break;
                                        } else if ret < 0 as libc::c_int {
                                            g_free(to_remove as gpointer);
                                            return ret;
                                        } else {
                                            print2log(
                                                b"\n\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                            );
                                        }
                                    } else {
                                        print2log(
                                            b"\n\0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                        );
                                    }
                                } else {
                                    print2log(
                                        b"\n\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                }
                            } else {
                                print2log(
                                    b"\n\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                            }
                        } else {
                            print2log(
                                b"\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            break;
                        }
                    } else {
                        print2log(
                            b"\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                    s += 1;
                }
            }
        }
        f += 1;
    }
    i = (*minutiae).num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if *to_remove.offset(i as isize) != 0 {
            ret = remove_minutia(i, minutiae);
            if ret != 0 {
                g_free(to_remove as gpointer);
                return ret;
            }
        }
        i -= 1;
    }
    g_free(to_remove as gpointer);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn remove_islands_and_lakes(
    mut minutiae: *mut MINUTIAE,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut to_remove: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut delta_y: libc::c_int = 0;
    let mut full_ndirs: libc::c_int = 0;
    let mut qtr_ndirs: libc::c_int = 0;
    let mut deltadir: libc::c_int = 0;
    let mut min_deltadir: libc::c_int = 0;
    let mut loop_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut loop_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut loop_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut loop_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nloop: libc::c_int = 0;
    let mut minutia1: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut minutia2: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut dist: libc::c_double = 0.;
    let mut dist_thresh: libc::c_int = 0;
    let mut half_loop: libc::c_int = 0;
    print2log(
        b"\nREMOVING ISLANDS AND LAKES:\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    dist_thresh = (*lfsparms).max_rmtest_dist;
    half_loop = (*lfsparms).max_half_loop;
    to_remove = calloc(
        (*minutiae).num as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if to_remove.is_null() {
        fprintf(
            stderr,
            b"ERROR : remove_islands_and_lakes : calloc : to_remove\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(610 as libc::c_int);
    }
    full_ndirs = (*lfsparms).num_directions << 1 as libc::c_int;
    qtr_ndirs = (*lfsparms).num_directions >> 2 as libc::c_int;
    min_deltadir = 3 as libc::c_int * qtr_ndirs - 1 as libc::c_int;
    f = 0 as libc::c_int;
    while f < (*minutiae).num - 1 as libc::c_int {
        if *to_remove.offset(f as isize) == 0 {
            print2log(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            minutia1 = *((*minutiae).list).offset(f as isize);
            s = f + 1 as libc::c_int;
            while s < (*minutiae).num {
                minutia2 = *((*minutiae).list).offset(s as isize);
                if (*minutia2).type_0 == (*minutia1).type_0 {
                    print2log(
                        b"1:%d(%d,%d)%d 2:%d(%d,%d)%d \0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        f,
                        (*minutia1).x,
                        (*minutia1).y,
                        (*minutia1).type_0,
                        s,
                        (*minutia2).x,
                        (*minutia2).y,
                        (*minutia2).type_0,
                    );
                    if *bdata
                        .offset(((*minutia1).y * iw) as isize)
                        .offset((*minutia1).x as isize) as libc::c_int
                        != (*minutia1).type_0
                    {
                        print2log(
                            b"\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        break;
                    } else {
                        if *bdata
                            .offset(((*minutia2).y * iw) as isize)
                            .offset((*minutia2).x as isize) as libc::c_int
                            != (*minutia2).type_0
                        {
                            *to_remove
                                .offset(
                                    s as isize,
                                ) = (0 as libc::c_int == 0) as libc::c_int;
                        }
                        if *to_remove.offset(s as isize) == 0 {
                            delta_y = (*minutia2).y - (*minutia1).y;
                            if delta_y <= dist_thresh {
                                print2log(
                                    b"1DY \0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                                dist = distance(
                                    (*minutia1).x,
                                    (*minutia1).y,
                                    (*minutia2).x,
                                    (*minutia2).y,
                                );
                                if dist <= dist_thresh as libc::c_double {
                                    print2log(
                                        b"2DS \0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                    deltadir = closest_dir_dist(
                                        (*minutia1).direction,
                                        (*minutia2).direction,
                                        full_ndirs,
                                    );
                                    if deltadir == -(1 as libc::c_int) {
                                        g_free(to_remove as gpointer);
                                        fprintf(
                                            stderr,
                                            b"ERROR : remove_islands_and_lakes : INVALID direction\n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                        return -(611 as libc::c_int);
                                    }
                                    if deltadir > min_deltadir {
                                        print2log(
                                            b"3DD \0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                        );
                                        ret = on_island_lake(
                                            &mut loop_x,
                                            &mut loop_y,
                                            &mut loop_ex,
                                            &mut loop_ey,
                                            &mut nloop,
                                            minutia1,
                                            minutia2,
                                            half_loop,
                                            bdata,
                                            iw,
                                            ih,
                                        );
                                        if ret == 1 as libc::c_int {
                                            print2log(
                                                b"4IL RM\n\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                            );
                                            ret = fill_loop(loop_x, loop_y, nloop, bdata, iw, ih);
                                            if ret != 0 {
                                                free_contour(loop_x, loop_y, loop_ex, loop_ey);
                                                g_free(to_remove as gpointer);
                                                return ret;
                                            }
                                            *to_remove
                                                .offset(
                                                    f as isize,
                                                ) = (0 as libc::c_int == 0) as libc::c_int;
                                            *to_remove
                                                .offset(
                                                    s as isize,
                                                ) = (0 as libc::c_int == 0) as libc::c_int;
                                            free_contour(loop_x, loop_y, loop_ex, loop_ey);
                                        } else if ret == 2 as libc::c_int {
                                            print2log(
                                                b"RM\n\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                            );
                                            *to_remove
                                                .offset(
                                                    f as isize,
                                                ) = (0 as libc::c_int == 0) as libc::c_int;
                                            break;
                                        } else if ret < 0 as libc::c_int {
                                            g_free(to_remove as gpointer);
                                            return ret;
                                        } else {
                                            print2log(
                                                b"\n\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                            );
                                        }
                                    } else {
                                        print2log(
                                            b"\n\0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                        );
                                    }
                                } else {
                                    print2log(
                                        b"\n\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                }
                            } else {
                                print2log(
                                    b"\n\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                                break;
                            }
                        } else {
                            print2log(
                                b"\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                    }
                }
                s += 1;
            }
        }
        f += 1;
    }
    i = (*minutiae).num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if *to_remove.offset(i as isize) != 0 {
            ret = remove_minutia(i, minutiae);
            if ret != 0 {
                g_free(to_remove as gpointer);
                return ret;
            }
        }
        i -= 1;
    }
    g_free(to_remove as gpointer);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn remove_malformations(
    mut minutiae: *mut MINUTIAE,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut low_flow_map: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut minutia: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut contour_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ncontour: libc::c_int = 0;
    let mut ax1: libc::c_int = 0;
    let mut ay1: libc::c_int = 0;
    let mut bx1: libc::c_int = 0;
    let mut by1: libc::c_int = 0;
    let mut ax2: libc::c_int = 0;
    let mut ay2: libc::c_int = 0;
    let mut bx2: libc::c_int = 0;
    let mut by2: libc::c_int = 0;
    let mut x_list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut y_list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut num: libc::c_int = 0;
    let mut a_dist: libc::c_double = 0.;
    let mut b_dist: libc::c_double = 0.;
    let mut ratio: libc::c_double = 0.;
    let mut fmapval: libc::c_int = 0;
    let mut removed: libc::c_int = 0;
    let mut blk_x: libc::c_int = 0;
    let mut blk_y: libc::c_int = 0;
    print2log(
        b"\nREMOVING MALFORMATIONS:\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    i = (*minutiae).num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        minutia = *((*minutiae).list).offset(i as isize);
        ret = trace_contour(
            &mut contour_x,
            &mut contour_y,
            &mut contour_ex,
            &mut contour_ey,
            &mut ncontour,
            (*lfsparms).malformation_steps_2,
            (*minutia).x,
            (*minutia).y,
            (*minutia).x,
            (*minutia).y,
            (*minutia).ex,
            (*minutia).ey,
            1 as libc::c_int,
            bdata,
            iw,
            ih,
        );
        if ret < 0 as libc::c_int {
            return ret;
        }
        if ret == 2 as libc::c_int || ret == 1 as libc::c_int
            || ncontour < (*lfsparms).malformation_steps_2
        {
            if ret == 1 as libc::c_int || ncontour < (*lfsparms).malformation_steps_2 {
                free_contour(contour_x, contour_y, contour_ex, contour_ey);
            }
            print2log(
                b"%d,%d RMA\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*minutia).x,
                (*minutia).y,
            );
            ret = remove_minutia(i, minutiae);
            if ret != 0 {
                return ret;
            }
        } else {
            ax1 = *contour_x
                .offset(((*lfsparms).malformation_steps_1 - 1 as libc::c_int) as isize);
            ay1 = *contour_y
                .offset(((*lfsparms).malformation_steps_1 - 1 as libc::c_int) as isize);
            bx1 = *contour_x
                .offset(((*lfsparms).malformation_steps_2 - 1 as libc::c_int) as isize);
            by1 = *contour_y
                .offset(((*lfsparms).malformation_steps_2 - 1 as libc::c_int) as isize);
            free_contour(contour_x, contour_y, contour_ex, contour_ey);
            ret = trace_contour(
                &mut contour_x,
                &mut contour_y,
                &mut contour_ex,
                &mut contour_ey,
                &mut ncontour,
                (*lfsparms).malformation_steps_2,
                (*minutia).x,
                (*minutia).y,
                (*minutia).x,
                (*minutia).y,
                (*minutia).ex,
                (*minutia).ey,
                0 as libc::c_int,
                bdata,
                iw,
                ih,
            );
            if ret < 0 as libc::c_int {
                return ret;
            }
            if ret == 2 as libc::c_int || ret == 1 as libc::c_int
                || ncontour < (*lfsparms).malformation_steps_2
            {
                if ret == 1 as libc::c_int || ncontour < (*lfsparms).malformation_steps_2
                {
                    free_contour(contour_x, contour_y, contour_ex, contour_ey);
                }
                print2log(
                    b"%d,%d RMB\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*minutia).x,
                    (*minutia).y,
                );
                ret = remove_minutia(i, minutiae);
                if ret != 0 {
                    return ret;
                }
            } else {
                ax2 = *contour_x
                    .offset(
                        ((*lfsparms).malformation_steps_1 - 1 as libc::c_int) as isize,
                    );
                ay2 = *contour_y
                    .offset(
                        ((*lfsparms).malformation_steps_1 - 1 as libc::c_int) as isize,
                    );
                bx2 = *contour_x
                    .offset(
                        ((*lfsparms).malformation_steps_2 - 1 as libc::c_int) as isize,
                    );
                by2 = *contour_y
                    .offset(
                        ((*lfsparms).malformation_steps_2 - 1 as libc::c_int) as isize,
                    );
                free_contour(contour_x, contour_y, contour_ex, contour_ey);
                a_dist = distance(ax1, ay1, ax2, ay2);
                b_dist = distance(bx1, by1, bx2, by2);
                blk_x = (*minutia).x / (*lfsparms).blocksize;
                blk_y = (*minutia).y / (*lfsparms).blocksize;
                removed = 0 as libc::c_int;
                if a_dist == 0.0f64 || b_dist == 0.0f64 {
                    print2log(
                        b"%d,%d RMMAL1\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*minutia).x,
                        (*minutia).y,
                    );
                    ret = remove_minutia(i, minutiae);
                    if ret != 0 {
                        return ret;
                    }
                    removed = (0 as libc::c_int == 0) as libc::c_int;
                }
                if removed == 0 {
                    fmapval = *low_flow_map
                        .offset((blk_y * mw) as isize)
                        .offset(blk_x as isize);
                    if fmapval != 0 {
                        if b_dist > (*lfsparms).max_malformation_dist as libc::c_double {
                            print2log(
                                b"%d,%d RMMAL2\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                (*minutia).x,
                                (*minutia).y,
                            );
                            ret = remove_minutia(i, minutiae);
                            if ret != 0 {
                                return ret;
                            }
                            removed = (0 as libc::c_int == 0) as libc::c_int;
                        }
                    }
                }
                if removed == 0 {
                    ret = line_points(
                        &mut x_list,
                        &mut y_list,
                        &mut num,
                        bx1,
                        by1,
                        bx2,
                        by2,
                    );
                    if ret != 0 {
                        return ret;
                    }
                    j = 0 as libc::c_int;
                    while j < num {
                        if *bdata
                            .offset((*y_list.offset(j as isize) * iw) as isize)
                            .offset(*x_list.offset(j as isize) as isize) as libc::c_int
                            != (*minutia).type_0
                        {
                            ratio = b_dist / a_dist;
                            ratio = if ratio < 0.0f64 {
                                (ratio * 16384.0f64 - 0.5f64) as libc::c_int
                                    as libc::c_double / 16384.0f64
                            } else {
                                (ratio * 16384.0f64 + 0.5f64) as libc::c_int
                                    as libc::c_double / 16384.0f64
                            };
                            if ratio > (*lfsparms).min_malformation_ratio {
                                print2log(
                                    b"%d,%d RMMAL3 (%f)\n\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    (*minutia).x,
                                    (*minutia).y,
                                    ratio,
                                );
                                ret = remove_minutia(i, minutiae);
                                if ret != 0 {
                                    g_free(x_list as gpointer);
                                    g_free(y_list as gpointer);
                                    return ret;
                                }
                                break;
                            }
                        }
                        j += 1;
                    }
                    g_free(x_list as gpointer);
                    g_free(y_list as gpointer);
                }
            }
        }
        i -= 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn remove_near_invblock_V2(
    mut minutiae: *mut MINUTIAE,
    mut direction_map: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut ni: libc::c_int = 0;
    let mut nbx: libc::c_int = 0;
    let mut nby: libc::c_int = 0;
    let mut nvalid: libc::c_int = 0;
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut sbi: libc::c_int = 0;
    let mut ebi: libc::c_int = 0;
    let mut bx: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    let mut px: libc::c_int = 0;
    let mut py: libc::c_int = 0;
    let mut removed: libc::c_int = 0;
    let mut minutia: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut lo_margin: libc::c_int = 0;
    let mut hi_margin: libc::c_int = 0;
    static mut startblk: [libc::c_int; 9] = [
        6 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        6 as libc::c_int,
        -(1 as libc::c_int),
        2 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        2 as libc::c_int,
    ];
    static mut endblk: [libc::c_int; 9] = [
        8 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        6 as libc::c_int,
        -(1 as libc::c_int),
        2 as libc::c_int,
        6 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
    ];
    static mut blkdx: [libc::c_int; 9] = [
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        0 as libc::c_int,
    ];
    static mut blkdy: [libc::c_int; 9] = [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ];
    print2log(
        b"\nREMOVING MINUTIA NEAR INVALID BLOCKS:\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    if (*lfsparms).inv_block_margin > (*lfsparms).blocksize >> 1 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR : remove_near_invblock_V2 : margin too large for blocksize\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(620 as libc::c_int);
    }
    lo_margin = (*lfsparms).inv_block_margin;
    hi_margin = (*lfsparms).blocksize - (*lfsparms).inv_block_margin - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*minutiae).num {
        minutia = *((*minutiae).list).offset(i as isize);
        bx = (*minutia).x / (*lfsparms).blocksize;
        by = (*minutia).y / (*lfsparms).blocksize;
        px = (*minutia).x % (*lfsparms).blocksize;
        py = (*minutia).y % (*lfsparms).blocksize;
        if px < lo_margin {
            ix = 0 as libc::c_int;
        } else if px > hi_margin {
            ix = 2 as libc::c_int;
        } else {
            ix = 1 as libc::c_int;
        }
        if py < lo_margin {
            iy = 0 as libc::c_int;
        } else if py > hi_margin {
            iy = 2 as libc::c_int;
        } else {
            iy = 1 as libc::c_int;
        }
        removed = 0 as libc::c_int;
        if ix != 1 as libc::c_int || iy != 1 as libc::c_int {
            sbi = *startblk
                .as_mut_ptr()
                .offset((iy * 3 as libc::c_int) as isize)
                .offset(ix as isize);
            ebi = *endblk
                .as_mut_ptr()
                .offset((iy * 3 as libc::c_int) as isize)
                .offset(ix as isize);
            ni = sbi;
            while ni <= ebi {
                nbx = bx + blkdx[ni as usize];
                nby = by + blkdy[ni as usize];
                if nbx < 0 as libc::c_int || nbx >= mw || nby < 0 as libc::c_int
                    || nby >= mh
                {
                    print2log(
                        b"%d,%d RM1\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*minutia).x,
                        (*minutia).y,
                    );
                    ret = remove_minutia(i, minutiae);
                    if ret != 0 {
                        return ret;
                    }
                    removed = (0 as libc::c_int == 0) as libc::c_int;
                    break;
                } else {
                    if *direction_map.offset((nby * mw) as isize).offset(nbx as isize)
                        == -(1 as libc::c_int)
                    {
                        nvalid = num_valid_8nbrs(direction_map, nbx, nby, mw, mh);
                        if nvalid < (*lfsparms).rm_valid_nbr_min {
                            print2log(
                                b"%d,%d RM2\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                (*minutia).x,
                                (*minutia).y,
                            );
                            ret = remove_minutia(i, minutiae);
                            if ret != 0 {
                                return ret;
                            }
                            removed = (0 as libc::c_int == 0) as libc::c_int;
                            break;
                        }
                    }
                    ni += 1;
                }
            }
        }
        if removed == 0 {
            i += 1;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn remove_pointing_invblock_V2(
    mut minutiae: *mut MINUTIAE,
    mut direction_map: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut delta_x: libc::c_int = 0;
    let mut delta_y: libc::c_int = 0;
    let mut dmapval: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut ny: libc::c_int = 0;
    let mut bx: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    let mut minutia: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut pi_factor: libc::c_double = 0.;
    let mut theta: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    print2log(
        b"\nREMOVING MINUTIA POINTING TO INVALID BLOCKS:\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    pi_factor = 3.14159265358979323846f64 / (*lfsparms).num_directions as libc::c_double;
    i = 0 as libc::c_int;
    while i < (*minutiae).num {
        minutia = *((*minutiae).list).offset(i as isize);
        theta = (*minutia).direction as libc::c_double * pi_factor;
        dx = sin(theta) * (*lfsparms).trans_dir_pix as libc::c_double;
        dy = cos(theta) * (*lfsparms).trans_dir_pix as libc::c_double;
        dx = if dx < 0.0f64 {
            (dx * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
        } else {
            (dx * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
        };
        dy = if dy < 0.0f64 {
            (dy * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
        } else {
            (dy * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
        };
        delta_x = (if dx < 0 as libc::c_int as libc::c_double {
            dx - 0.5f64
        } else {
            dx + 0.5f64
        }) as libc::c_int;
        delta_y = (if dy < 0 as libc::c_int as libc::c_double {
            dy - 0.5f64
        } else {
            dy + 0.5f64
        }) as libc::c_int;
        nx = (*minutia).x - delta_x;
        ny = (*minutia).y + delta_y;
        bx = nx / (*lfsparms).blocksize;
        by = ny / (*lfsparms).blocksize;
        bx = if 0 as libc::c_int > bx { 0 as libc::c_int } else { bx };
        bx = if (mw - 1 as libc::c_int) < bx { mw - 1 as libc::c_int } else { bx };
        by = if 0 as libc::c_int > by { 0 as libc::c_int } else { by };
        by = if (mh - 1 as libc::c_int) < by { mh - 1 as libc::c_int } else { by };
        dmapval = *direction_map.offset((by * mw) as isize).offset(bx as isize);
        if dmapval == -(1 as libc::c_int) {
            print2log(
                b"%d,%d RM\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*minutia).x,
                (*minutia).y,
            );
            ret = remove_minutia(i, minutiae);
            if ret != 0 {
                return ret;
            }
        } else {
            i += 1;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mark_minutiae_in_range(
    mut minutiae: *mut MINUTIAE,
    mut to_remove: *mut libc::c_int,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) {
    let mut i: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*minutiae).num {
        if !(*to_remove.offset(i as isize) != 0) {
            dist = sqrt(
                ((x - (**((*minutiae).list).offset(i as isize)).x)
                    * (x - (**((*minutiae).list).offset(i as isize)).x)
                    + (y - (**((*minutiae).list).offset(i as isize)).y)
                        * (y - (**((*minutiae).list).offset(i as isize)).y))
                    as libc::c_double,
            ) as libc::c_int;
            if dist < (*lfsparms).min_pp_distance {
                *to_remove.offset(i as isize) = 1 as libc::c_int;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn remove_perimeter_pts(
    mut minutiae: *mut MINUTIAE,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut to_remove: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut left: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut left_up: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut left_down: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut right: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut right_up: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut right_down: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut removed: libc::c_int = 0 as libc::c_int;
    let mut left_min: libc::c_int = 0;
    let mut right_max: libc::c_int = 0;
    if (*lfsparms).remove_perimeter_pts == 0 {
        return 0 as libc::c_int;
    }
    to_remove = calloc(
        (*minutiae).num as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    left = calloc(
        ih as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    left_up = calloc(
        ih as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    left_down = calloc(
        ih as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    right = calloc(
        ih as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    right_up = calloc(
        ih as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    right_down = calloc(
        ih as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    left_min = iw - 1 as libc::c_int;
    right_max = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < ih {
        j = 0 as libc::c_int;
        while j < left_min {
            if *bdata.offset((i * iw + j) as isize) as libc::c_int != 0 as libc::c_int {
                left_min = j;
                break;
            } else {
                j += 1;
            }
        }
        if left_min == iw - 1 as libc::c_int {
            *left_down.offset(i as isize) = -(1 as libc::c_int);
        } else {
            *left_down.offset(i as isize) = left_min;
        }
        j = iw - 1 as libc::c_int;
        while j >= right_max {
            if *bdata.offset((i * iw + j) as isize) as libc::c_int != 0 as libc::c_int {
                right_max = j;
                break;
            } else {
                j -= 1;
            }
        }
        if right_max == 0 as libc::c_int {
            *right_down.offset(i as isize) = -(1 as libc::c_int);
        } else {
            *right_down.offset(i as isize) = right_max;
        }
        i += 1;
    }
    left_min = iw - 1 as libc::c_int;
    right_max = 0 as libc::c_int;
    i = ih - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        j = 0 as libc::c_int;
        while j < left_min {
            if *bdata.offset((i * iw + j) as isize) as libc::c_int != 0 as libc::c_int {
                left_min = j;
                break;
            } else {
                j += 1;
            }
        }
        if left_min == iw - 1 as libc::c_int {
            *left_up.offset(i as isize) = -(1 as libc::c_int);
        } else {
            *left_up.offset(i as isize) = left_min;
        }
        j = iw - 1 as libc::c_int;
        while j >= right_max {
            if *bdata.offset((i * iw + j) as isize) as libc::c_int != 0 as libc::c_int {
                right_max = j;
                break;
            } else {
                j -= 1;
            }
        }
        if right_max == 0 as libc::c_int {
            *right_up.offset(i as isize) = -(1 as libc::c_int);
        } else {
            *right_up.offset(i as isize) = right_max;
        }
        i -= 1;
    }
    left_min = *left_down.offset((ih - 1 as libc::c_int) as isize);
    right_max = *right_down.offset((ih - 1 as libc::c_int) as isize);
    i = 0 as libc::c_int;
    while i < ih {
        if *left_down.offset(i as isize) != left_min {
            *left.offset(i as isize) = *left_down.offset(i as isize);
        } else {
            *left.offset(i as isize) = *left_up.offset(i as isize);
        }
        if *right_down.offset(i as isize) != right_max {
            *right.offset(i as isize) = *right_down.offset(i as isize);
        } else {
            *right.offset(i as isize) = *right_up.offset(i as isize);
        }
        i += 1;
    }
    free(left_up as *mut libc::c_void);
    free(left_down as *mut libc::c_void);
    free(right_up as *mut libc::c_void);
    free(right_down as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < ih {
        if *left.offset(i as isize) != -(1 as libc::c_int) {
            mark_minutiae_in_range(
                minutiae,
                to_remove,
                *left.offset(i as isize),
                i,
                lfsparms,
            );
        }
        if *right.offset(i as isize) != -(1 as libc::c_int) {
            mark_minutiae_in_range(
                minutiae,
                to_remove,
                *right.offset(i as isize),
                i,
                lfsparms,
            );
        }
        i += 1;
    }
    free(left as *mut libc::c_void);
    free(right as *mut libc::c_void);
    i = (*minutiae).num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if *to_remove.offset(i as isize) != 0 {
            removed += 1;
            ret = remove_minutia(i, minutiae);
            if ret != 0 {
                free(to_remove as *mut libc::c_void);
                return ret;
            }
        }
        i -= 1;
    }
    free(to_remove as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn remove_overlaps(
    mut minutiae: *mut MINUTIAE,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut to_remove: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut delta_y: libc::c_int = 0;
    let mut full_ndirs: libc::c_int = 0;
    let mut qtr_ndirs: libc::c_int = 0;
    let mut deltadir: libc::c_int = 0;
    let mut min_deltadir: libc::c_int = 0;
    let mut minutia1: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut minutia2: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut dist: libc::c_double = 0.;
    let mut joindir: libc::c_int = 0;
    let mut opp1dir: libc::c_int = 0;
    let mut half_ndirs: libc::c_int = 0;
    print2log(
        b"\nREMOVING OVERLAPS:\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    to_remove = calloc(
        (*minutiae).num as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if to_remove.is_null() {
        fprintf(
            stderr,
            b"ERROR : remove_overlaps : calloc : to_remove\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(650 as libc::c_int);
    }
    full_ndirs = (*lfsparms).num_directions << 1 as libc::c_int;
    qtr_ndirs = (*lfsparms).num_directions >> 2 as libc::c_int;
    half_ndirs = (*lfsparms).num_directions >> 1 as libc::c_int;
    min_deltadir = 3 as libc::c_int * qtr_ndirs - 1 as libc::c_int;
    f = 0 as libc::c_int;
    while f < (*minutiae).num - 1 as libc::c_int {
        if *to_remove.offset(f as isize) == 0 {
            print2log(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            minutia1 = *((*minutiae).list).offset(f as isize);
            s = f + 1 as libc::c_int;
            while s < (*minutiae).num {
                minutia2 = *((*minutiae).list).offset(s as isize);
                print2log(
                    b"1:%d(%d,%d)%d 2:%d(%d,%d)%d \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    f,
                    (*minutia1).x,
                    (*minutia1).y,
                    (*minutia1).type_0,
                    s,
                    (*minutia2).x,
                    (*minutia2).y,
                    (*minutia2).type_0,
                );
                if *bdata
                    .offset(((*minutia1).y * iw) as isize)
                    .offset((*minutia1).x as isize) as libc::c_int != (*minutia1).type_0
                {
                    print2log(
                        b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    break;
                } else {
                    if *bdata
                        .offset(((*minutia2).y * iw) as isize)
                        .offset((*minutia2).x as isize) as libc::c_int
                        != (*minutia2).type_0
                    {
                        *to_remove
                            .offset(s as isize) = (0 as libc::c_int == 0) as libc::c_int;
                    }
                    if *to_remove.offset(s as isize) == 0 {
                        delta_y = (*minutia2).y - (*minutia1).y;
                        if delta_y <= (*lfsparms).max_overlap_dist {
                            print2log(
                                b"1DY \0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            dist = distance(
                                (*minutia1).x,
                                (*minutia1).y,
                                (*minutia2).x,
                                (*minutia2).y,
                            );
                            if dist <= (*lfsparms).max_overlap_dist as libc::c_double {
                                print2log(
                                    b"2DS \0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                                deltadir = closest_dir_dist(
                                    (*minutia1).direction,
                                    (*minutia2).direction,
                                    full_ndirs,
                                );
                                if deltadir == -(1 as libc::c_int) {
                                    g_free(to_remove as gpointer);
                                    fprintf(
                                        stderr,
                                        b"ERROR : remove_overlaps : INVALID direction\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    return -(651 as libc::c_int);
                                }
                                if deltadir > min_deltadir {
                                    print2log(
                                        b"3DD \0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                    if (*minutia1).type_0 == (*minutia2).type_0 {
                                        joindir = line2direction(
                                            (*minutia1).x,
                                            (*minutia1).y,
                                            (*minutia2).x,
                                            (*minutia2).y,
                                            (*lfsparms).num_directions,
                                        );
                                        opp1dir = ((*minutia1).direction
                                            + (*lfsparms).num_directions) % full_ndirs;
                                        joindir = abs(opp1dir - joindir);
                                        joindir = if joindir < full_ndirs - joindir {
                                            joindir
                                        } else {
                                            full_ndirs - joindir
                                        };
                                        print2log(
                                            b"joindir=%d dist=%f \0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                            joindir,
                                            dist,
                                        );
                                        if (joindir <= half_ndirs
                                            || dist
                                                <= (*lfsparms).max_overlap_join_dist as libc::c_double)
                                            && free_path(
                                                (*minutia1).x,
                                                (*minutia1).y,
                                                (*minutia2).x,
                                                (*minutia2).y,
                                                bdata,
                                                iw,
                                                ih,
                                                lfsparms,
                                            ) != 0
                                        {
                                            print2log(
                                                b"4OV RM\n\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                            );
                                            *to_remove
                                                .offset(
                                                    f as isize,
                                                ) = (0 as libc::c_int == 0) as libc::c_int;
                                            *to_remove
                                                .offset(
                                                    s as isize,
                                                ) = (0 as libc::c_int == 0) as libc::c_int;
                                        } else {
                                            print2log(
                                                b"\n\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                            );
                                        }
                                    } else {
                                        print2log(
                                            b"\n\0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                        );
                                    }
                                } else {
                                    print2log(
                                        b"\n\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                }
                            } else {
                                print2log(
                                    b"\n\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                            }
                        } else {
                            print2log(
                                b"\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            break;
                        }
                    } else {
                        print2log(
                            b"\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                    s += 1;
                }
            }
        }
        f += 1;
    }
    i = (*minutiae).num - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if *to_remove.offset(i as isize) != 0 {
            ret = remove_minutia(i, minutiae);
            if ret != 0 {
                g_free(to_remove as gpointer);
                return ret;
            }
        }
        i -= 1;
    }
    g_free(to_remove as gpointer);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn remove_pores_V2(
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
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut removed: libc::c_int = 0;
    let mut blk_x: libc::c_int = 0;
    let mut blk_y: libc::c_int = 0;
    let mut rx: libc::c_int = 0;
    let mut ry: libc::c_int = 0;
    let mut px: libc::c_int = 0;
    let mut py: libc::c_int = 0;
    let mut pex: libc::c_int = 0;
    let mut pey: libc::c_int = 0;
    let mut bx: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut qx: libc::c_int = 0;
    let mut qy: libc::c_int = 0;
    let mut qex: libc::c_int = 0;
    let mut qey: libc::c_int = 0;
    let mut ax: libc::c_int = 0;
    let mut ay: libc::c_int = 0;
    let mut cx: libc::c_int = 0;
    let mut cy: libc::c_int = 0;
    let mut minutia: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut pi_factor: libc::c_double = 0.;
    let mut theta: libc::c_double = 0.;
    let mut sin_theta: libc::c_double = 0.;
    let mut cos_theta: libc::c_double = 0.;
    let mut ab2: libc::c_double = 0.;
    let mut cd2: libc::c_double = 0.;
    let mut ratio: libc::c_double = 0.;
    let mut contour_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ncontour: libc::c_int = 0;
    let mut drx: libc::c_double = 0.;
    let mut dry: libc::c_double = 0.;
    print2log(
        b"\nREMOVING PORES:\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pi_factor = 3.14159265358979323846f64 / (*lfsparms).num_directions as libc::c_double;
    i = 0 as libc::c_int;
    while i < (*minutiae).num {
        minutia = *((*minutiae).list).offset(i as isize);
        removed = 0 as libc::c_int;
        blk_x = (*minutia).x / (*lfsparms).blocksize;
        blk_y = (*minutia).y / (*lfsparms).blocksize;
        if (*low_flow_map.offset((blk_y * mw) as isize).offset(blk_x as isize) != 0
            || *high_curve_map.offset((blk_y * mw) as isize).offset(blk_x as isize) != 0)
            && *direction_map.offset((blk_y * mw) as isize).offset(blk_x as isize)
                >= 0 as libc::c_int
        {
            theta = (*minutia).direction as libc::c_double * pi_factor;
            sin_theta = sin(theta);
            cos_theta = cos(theta);
            drx = (*minutia).x as libc::c_double
                - sin_theta * (*lfsparms).pores_trans_r as libc::c_double;
            dry = (*minutia).y as libc::c_double
                + cos_theta * (*lfsparms).pores_trans_r as libc::c_double;
            drx = if drx < 0.0f64 {
                (drx * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
            } else {
                (drx * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
            };
            dry = if dry < 0.0f64 {
                (dry * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
            } else {
                (dry * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
            };
            rx = (if drx < 0 as libc::c_int as libc::c_double {
                drx - 0.5f64
            } else {
                drx + 0.5f64
            }) as libc::c_int;
            ry = (if dry < 0 as libc::c_int as libc::c_double {
                dry - 0.5f64
            } else {
                dry + 0.5f64
            }) as libc::c_int;
            if *bdata.offset((ry * iw) as isize).offset(rx as isize) as libc::c_int
                != (*minutia).type_0
            {
                if search_in_direction(
                    &mut px,
                    &mut py,
                    &mut pex,
                    &mut pey,
                    (*minutia).type_0,
                    rx,
                    ry,
                    -cos_theta,
                    -sin_theta,
                    (*lfsparms).pores_perp_steps,
                    bdata,
                    iw,
                    ih,
                ) != 0
                {
                    ret = trace_contour(
                        &mut contour_x,
                        &mut contour_y,
                        &mut contour_ex,
                        &mut contour_ey,
                        &mut ncontour,
                        (*lfsparms).pores_steps_fwd,
                        px,
                        py,
                        px,
                        py,
                        pex,
                        pey,
                        1 as libc::c_int,
                        bdata,
                        iw,
                        ih,
                    );
                    if ret < 0 as libc::c_int {
                        return ret;
                    }
                    if ret == 2 as libc::c_int || ret == 1 as libc::c_int
                        || ncontour < (*lfsparms).pores_steps_fwd
                    {
                        if ret == 1 as libc::c_int
                            || ncontour < (*lfsparms).pores_steps_fwd
                        {
                            free_contour(contour_x, contour_y, contour_ex, contour_ey);
                        }
                        print2log(
                            b"%d,%d RMB\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            (*minutia).x,
                            (*minutia).y,
                        );
                        ret = remove_minutia(i, minutiae);
                        if ret != 0 {
                            return ret;
                        }
                        removed = (0 as libc::c_int == 0) as libc::c_int;
                    } else {
                        bx = *contour_x.offset((ncontour - 1 as libc::c_int) as isize);
                        by = *contour_y.offset((ncontour - 1 as libc::c_int) as isize);
                        free_contour(contour_x, contour_y, contour_ex, contour_ey);
                        ret = trace_contour(
                            &mut contour_x,
                            &mut contour_y,
                            &mut contour_ex,
                            &mut contour_ey,
                            &mut ncontour,
                            (*lfsparms).pores_steps_bwd,
                            px,
                            py,
                            px,
                            py,
                            pex,
                            pey,
                            0 as libc::c_int,
                            bdata,
                            iw,
                            ih,
                        );
                        if ret < 0 as libc::c_int {
                            return ret;
                        }
                        if ret == 2 as libc::c_int || ret == 1 as libc::c_int
                            || ncontour < (*lfsparms).pores_steps_bwd
                        {
                            if ret == 1 as libc::c_int
                                || ncontour < (*lfsparms).pores_steps_bwd
                            {
                                free_contour(contour_x, contour_y, contour_ex, contour_ey);
                            }
                            print2log(
                                b"%d,%d RMD\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                (*minutia).x,
                                (*minutia).y,
                            );
                            ret = remove_minutia(i, minutiae);
                            if ret != 0 {
                                return ret;
                            }
                            removed = (0 as libc::c_int == 0) as libc::c_int;
                        } else {
                            dx = *contour_x
                                .offset((ncontour - 1 as libc::c_int) as isize);
                            dy = *contour_y
                                .offset((ncontour - 1 as libc::c_int) as isize);
                            free_contour(contour_x, contour_y, contour_ex, contour_ey);
                            if search_in_direction(
                                &mut qx,
                                &mut qy,
                                &mut qex,
                                &mut qey,
                                (*minutia).type_0,
                                rx,
                                ry,
                                cos_theta,
                                sin_theta,
                                (*lfsparms).pores_perp_steps,
                                bdata,
                                iw,
                                ih,
                            ) != 0
                            {
                                ret = trace_contour(
                                    &mut contour_x,
                                    &mut contour_y,
                                    &mut contour_ex,
                                    &mut contour_ey,
                                    &mut ncontour,
                                    (*lfsparms).pores_steps_fwd,
                                    qx,
                                    qy,
                                    qx,
                                    qy,
                                    qex,
                                    qey,
                                    0 as libc::c_int,
                                    bdata,
                                    iw,
                                    ih,
                                );
                                if ret < 0 as libc::c_int {
                                    return ret;
                                }
                                if ret == 2 as libc::c_int || ret == 1 as libc::c_int
                                    || ncontour < (*lfsparms).pores_steps_fwd
                                {
                                    if ret == 1 as libc::c_int
                                        || ncontour < (*lfsparms).pores_steps_fwd
                                    {
                                        free_contour(contour_x, contour_y, contour_ex, contour_ey);
                                    }
                                    print2log(
                                        b"%d,%d RMA\n\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        (*minutia).x,
                                        (*minutia).y,
                                    );
                                    ret = remove_minutia(i, minutiae);
                                    if ret != 0 {
                                        return ret;
                                    }
                                    removed = (0 as libc::c_int == 0) as libc::c_int;
                                } else {
                                    ax = *contour_x
                                        .offset((ncontour - 1 as libc::c_int) as isize);
                                    ay = *contour_y
                                        .offset((ncontour - 1 as libc::c_int) as isize);
                                    free_contour(contour_x, contour_y, contour_ex, contour_ey);
                                    ret = trace_contour(
                                        &mut contour_x,
                                        &mut contour_y,
                                        &mut contour_ex,
                                        &mut contour_ey,
                                        &mut ncontour,
                                        (*lfsparms).pores_steps_bwd,
                                        qx,
                                        qy,
                                        qx,
                                        qy,
                                        qex,
                                        qey,
                                        1 as libc::c_int,
                                        bdata,
                                        iw,
                                        ih,
                                    );
                                    if ret < 0 as libc::c_int {
                                        return ret;
                                    }
                                    if ret == 2 as libc::c_int || ret == 1 as libc::c_int
                                        || ncontour < (*lfsparms).pores_steps_bwd
                                    {
                                        if ret == 1 as libc::c_int
                                            || ncontour < (*lfsparms).pores_steps_bwd
                                        {
                                            free_contour(contour_x, contour_y, contour_ex, contour_ey);
                                        }
                                        print2log(
                                            b"%d,%d RMC\n\0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                            (*minutia).x,
                                            (*minutia).y,
                                        );
                                        ret = remove_minutia(i, minutiae);
                                        if ret != 0 {
                                            return ret;
                                        }
                                        removed = (0 as libc::c_int == 0) as libc::c_int;
                                    } else {
                                        cx = *contour_x
                                            .offset((ncontour - 1 as libc::c_int) as isize);
                                        cy = *contour_y
                                            .offset((ncontour - 1 as libc::c_int) as isize);
                                        free_contour(contour_x, contour_y, contour_ex, contour_ey);
                                        ab2 = squared_distance(ax, ay, bx, by);
                                        cd2 = squared_distance(cx, cy, dx, dy);
                                        if cd2 > (*lfsparms).pores_min_dist2 {
                                            ratio = ab2 / cd2;
                                            if ratio <= (*lfsparms).pores_max_ratio {
                                                print2log(
                                                    b"%d,%d \0" as *const u8 as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    (*minutia).x,
                                                    (*minutia).y,
                                                );
                                                print2log(
                                                    b"R=%d,%d P=%d,%d B=%d,%d D=%d,%d Q=%d,%d A=%d,%d C=%d,%d \0"
                                                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                                                    rx,
                                                    ry,
                                                    px,
                                                    py,
                                                    bx,
                                                    by,
                                                    dx,
                                                    dy,
                                                    qx,
                                                    qy,
                                                    ax,
                                                    ay,
                                                    cx,
                                                    cy,
                                                );
                                                print2log(
                                                    b"RMRATIO %f\n\0" as *const u8 as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    ratio,
                                                );
                                                ret = remove_minutia(i, minutiae);
                                                if ret != 0 {
                                                    return ret;
                                                }
                                                removed = (0 as libc::c_int == 0) as libc::c_int;
                                            }
                                        }
                                    }
                                }
                            } else {
                                print2log(
                                    b"%d,%d RMQ\n\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    (*minutia).x,
                                    (*minutia).y,
                                );
                                ret = remove_minutia(i, minutiae);
                                if ret != 0 {
                                    return ret;
                                }
                                removed = (0 as libc::c_int == 0) as libc::c_int;
                            }
                        }
                    }
                } else {
                    print2log(
                        b"%d,%d RMP\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*minutia).x,
                        (*minutia).y,
                    );
                    ret = remove_minutia(i, minutiae);
                    if ret != 0 {
                        return ret;
                    }
                    removed = (0 as libc::c_int == 0) as libc::c_int;
                }
            }
        }
        if removed == 0 {
            i += 1;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn remove_or_adjust_side_minutiae_V2(
    mut minutiae: *mut MINUTIAE,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut direction_map: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut minutia: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut pi_factor: libc::c_double = 0.;
    let mut theta: libc::c_double = 0.;
    let mut sin_theta: libc::c_double = 0.;
    let mut cos_theta: libc::c_double = 0.;
    let mut contour_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ncontour: libc::c_int = 0;
    let mut rot_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut minloc: libc::c_int = 0;
    let mut minmax_val: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut minmax_i: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut minmax_type: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut minmax_alloc: libc::c_int = 0;
    let mut minmax_num: libc::c_int = 0;
    let mut drot_y: libc::c_double = 0.;
    let mut bx: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    print2log(
        b"\nADJUSTING SIDE MINUTIA:\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    rot_y = g_malloc(
        ((((*lfsparms).side_half_contour << 1 as libc::c_int) + 1 as libc::c_int)
            as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    pi_factor = 3.14159265358979323846f64 / (*lfsparms).num_directions as libc::c_double;
    i = 0 as libc::c_int;
    while i < (*minutiae).num {
        minutia = *((*minutiae).list).offset(i as isize);
        ret = get_centered_contour(
            &mut contour_x,
            &mut contour_y,
            &mut contour_ex,
            &mut contour_ey,
            &mut ncontour,
            (*lfsparms).side_half_contour,
            (*minutia).x,
            (*minutia).y,
            (*minutia).ex,
            (*minutia).ey,
            bdata,
            iw,
            ih,
        );
        if ret < 0 as libc::c_int {
            g_free(rot_y as gpointer);
            return ret;
        }
        if ret == 1 as libc::c_int || ret == 2 as libc::c_int || ret == 3 as libc::c_int
        {
            print2log(
                b"%d,%d RM1\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*minutia).x,
                (*minutia).y,
            );
            ret = remove_minutia(i, minutiae);
            if ret != 0 {
                g_free(rot_y as gpointer);
                return ret;
            }
        } else {
            theta = (*minutia).direction as libc::c_double * pi_factor;
            sin_theta = sin(theta);
            cos_theta = cos(theta);
            j = 0 as libc::c_int;
            while j < ncontour {
                drot_y = *contour_x.offset(j as isize) as libc::c_double * sin_theta
                    - *contour_y.offset(j as isize) as libc::c_double * cos_theta;
                drot_y = if drot_y < 0.0f64 {
                    (drot_y * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double
                        / 16384.0f64
                } else {
                    (drot_y * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double
                        / 16384.0f64
                };
                *rot_y
                    .offset(
                        j as isize,
                    ) = (if drot_y < 0 as libc::c_int as libc::c_double {
                    drot_y - 0.5f64
                } else {
                    drot_y + 0.5f64
                }) as libc::c_int;
                j += 1;
            }
            ret = minmaxs(
                &mut minmax_val,
                &mut minmax_type,
                &mut minmax_i,
                &mut minmax_alloc,
                &mut minmax_num,
                rot_y,
                ncontour,
            );
            if ret != 0 {
                g_free(rot_y as gpointer);
                free_contour(contour_x, contour_y, contour_ex, contour_ey);
                return ret;
            }
            if minmax_num == 1 as libc::c_int
                && *minmax_type.offset(0 as libc::c_int as isize) == -(1 as libc::c_int)
            {
                print2log(
                    b"%d,%d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*minutia).x,
                    (*minutia).y,
                );
                (*minutia)
                    .x = *contour_x
                    .offset(*minmax_i.offset(0 as libc::c_int as isize) as isize);
                (*minutia)
                    .y = *contour_y
                    .offset(*minmax_i.offset(0 as libc::c_int as isize) as isize);
                (*minutia)
                    .ex = *contour_ex
                    .offset(*minmax_i.offset(0 as libc::c_int as isize) as isize);
                (*minutia)
                    .ey = *contour_ey
                    .offset(*minmax_i.offset(0 as libc::c_int as isize) as isize);
                bx = (*minutia).x / (*lfsparms).blocksize;
                by = (*minutia).y / (*lfsparms).blocksize;
                if *direction_map.offset((by * mw) as isize).offset(bx as isize)
                    == -(1 as libc::c_int)
                {
                    ret = remove_minutia(i, minutiae);
                    if ret != 0 {
                        g_free(rot_y as gpointer);
                        free_contour(contour_x, contour_y, contour_ex, contour_ey);
                        if minmax_alloc > 0 as libc::c_int {
                            g_free(minmax_val as gpointer);
                            g_free(minmax_type as gpointer);
                            g_free(minmax_i as gpointer);
                        }
                        return ret;
                    }
                    print2log(
                        b"RM2\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                } else {
                    i += 1;
                    print2log(
                        b"AD1 %d,%d\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*minutia).x,
                        (*minutia).y,
                    );
                }
            } else if minmax_num == 3 as libc::c_int
                && *minmax_type.offset(0 as libc::c_int as isize) == -(1 as libc::c_int)
            {
                if *minmax_val.offset(0 as libc::c_int as isize)
                    < *minmax_val.offset(2 as libc::c_int as isize)
                {
                    minloc = *minmax_i.offset(0 as libc::c_int as isize);
                } else {
                    minloc = *minmax_i.offset(2 as libc::c_int as isize);
                }
                print2log(
                    b"%d,%d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*minutia).x,
                    (*minutia).y,
                );
                (*minutia).x = *contour_x.offset(minloc as isize);
                (*minutia).y = *contour_y.offset(minloc as isize);
                (*minutia).ex = *contour_ex.offset(minloc as isize);
                (*minutia).ey = *contour_ey.offset(minloc as isize);
                bx = (*minutia).x / (*lfsparms).blocksize;
                by = (*minutia).y / (*lfsparms).blocksize;
                if *direction_map.offset((by * mw) as isize).offset(bx as isize)
                    == -(1 as libc::c_int)
                {
                    ret = remove_minutia(i, minutiae);
                    if ret != 0 {
                        g_free(rot_y as gpointer);
                        free_contour(contour_x, contour_y, contour_ex, contour_ey);
                        if minmax_alloc > 0 as libc::c_int {
                            g_free(minmax_val as gpointer);
                            g_free(minmax_type as gpointer);
                            g_free(minmax_i as gpointer);
                        }
                        return ret;
                    }
                    print2log(
                        b"RM3\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                } else {
                    i += 1;
                    print2log(
                        b"AD2 %d,%d\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*minutia).x,
                        (*minutia).y,
                    );
                }
            } else {
                print2log(
                    b"%d,%d RM4\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*minutia).x,
                    (*minutia).y,
                );
                ret = remove_minutia(i, minutiae);
                if ret != 0 {
                    g_free(rot_y as gpointer);
                    free_contour(contour_x, contour_y, contour_ex, contour_ey);
                    if minmax_alloc > 0 as libc::c_int {
                        g_free(minmax_val as gpointer);
                        g_free(minmax_type as gpointer);
                        g_free(minmax_i as gpointer);
                    }
                    return ret;
                }
            }
            free_contour(contour_x, contour_y, contour_ex, contour_ey);
            if minmax_alloc > 0 as libc::c_int {
                g_free(minmax_val as gpointer);
                g_free(minmax_type as gpointer);
                g_free(minmax_i as gpointer);
            }
        }
    }
    g_free(rot_y as gpointer);
    return 0 as libc::c_int;
}
