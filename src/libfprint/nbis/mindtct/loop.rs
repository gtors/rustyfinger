use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn squared_distance(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn line2direction(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn minutia_type(_: libc::c_int) -> libc::c_int;
    fn is_minutia_appearing(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn create_minutia(
        _: *mut *mut MINUTIA,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn update_minutiae(
        _: *mut MINUTIAE,
        _: *mut MINUTIA,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: *const LFSPARMS,
    ) -> libc::c_int;
    fn free_minutia(_: *mut MINUTIA);
    fn shape_from_contour(
        _: *mut *mut SHAPE,
        _: *const libc::c_int,
        _: *const libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn free_shape(_: *mut SHAPE);
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
    fn free_contour(
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    );
    fn allocate_contour(
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn is_chain_clockwise(
        _: *const libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn chain_code_loop(
        _: *mut *mut libc::c_int,
        _: *mut libc::c_int,
        _: *const libc::c_int,
        _: *const libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn g_free(mem: gpointer);
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
pub struct rows {
    pub y: libc::c_int,
    pub xs: *mut libc::c_int,
    pub alloc: libc::c_int,
    pub npts: libc::c_int,
}
pub type ROW = rows;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shape {
    pub ymin: libc::c_int,
    pub ymax: libc::c_int,
    pub rows: *mut *mut ROW,
    pub alloc: libc::c_int,
    pub nrows: libc::c_int,
}
pub type SHAPE = shape;
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
pub unsafe extern "C" fn on_loop(
    mut minutia: *const MINUTIA,
    max_loop_len: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut contour_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ncontour: libc::c_int = 0;
    ret = trace_contour(
        &mut contour_x,
        &mut contour_y,
        &mut contour_ex,
        &mut contour_ey,
        &mut ncontour,
        max_loop_len,
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
    if ret == 2 as libc::c_int {
        return ret;
    }
    if ret == 1 as libc::c_int {
        free_contour(contour_x, contour_y, contour_ex, contour_ey);
        return 1 as libc::c_int;
    }
    if ret == 0 as libc::c_int {
        free_contour(contour_x, contour_y, contour_ex, contour_ey);
        return 0 as libc::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn on_island_lake(
    mut ocontour_x: *mut *mut libc::c_int,
    mut ocontour_y: *mut *mut libc::c_int,
    mut ocontour_ex: *mut *mut libc::c_int,
    mut ocontour_ey: *mut *mut libc::c_int,
    mut oncontour: *mut libc::c_int,
    mut minutia1: *const MINUTIA,
    mut minutia2: *const MINUTIA,
    max_half_loop: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut contour1_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour1_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour1_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour1_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ncontour1: libc::c_int = 0;
    let mut contour2_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour2_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour2_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour2_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ncontour2: libc::c_int = 0;
    let mut loop_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut loop_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut loop_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut loop_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nloop: libc::c_int = 0;
    ret = trace_contour(
        &mut contour1_x,
        &mut contour1_y,
        &mut contour1_ex,
        &mut contour1_ey,
        &mut ncontour1,
        max_half_loop,
        (*minutia2).x,
        (*minutia2).y,
        (*minutia1).x,
        (*minutia1).y,
        (*minutia1).ex,
        (*minutia1).ey,
        0 as libc::c_int,
        bdata,
        iw,
        ih,
    );
    if ret == 2 as libc::c_int {
        return ret;
    }
    if ret == 1 as libc::c_int {
        ret = trace_contour(
            &mut contour2_x,
            &mut contour2_y,
            &mut contour2_ex,
            &mut contour2_ey,
            &mut ncontour2,
            max_half_loop,
            (*minutia1).x,
            (*minutia1).y,
            (*minutia2).x,
            (*minutia2).y,
            (*minutia2).ex,
            (*minutia2).ey,
            0 as libc::c_int,
            bdata,
            iw,
            ih,
        );
        if ret == 2 as libc::c_int {
            free_contour(contour1_x, contour1_y, contour1_ex, contour1_ey);
            return ret;
        }
        if ret == 1 as libc::c_int {
            nloop = ncontour1 + ncontour2 + 2 as libc::c_int;
            ret = allocate_contour(
                &mut loop_x,
                &mut loop_y,
                &mut loop_ex,
                &mut loop_ey,
                nloop,
            );
            if ret != 0 {
                free_contour(contour1_x, contour1_y, contour1_ex, contour1_ey);
                free_contour(contour2_x, contour2_y, contour2_ex, contour2_ey);
                return ret;
            }
            l = 0 as libc::c_int;
            *loop_x.offset(l as isize) = (*minutia1).x;
            *loop_y.offset(l as isize) = (*minutia1).y;
            *loop_ex.offset(l as isize) = (*minutia1).ex;
            let fresh0 = l;
            l = l + 1;
            *loop_ey.offset(fresh0 as isize) = (*minutia1).ey;
            i = 0 as libc::c_int;
            while i < ncontour1 {
                *loop_x.offset(l as isize) = *contour1_x.offset(i as isize);
                *loop_y.offset(l as isize) = *contour1_y.offset(i as isize);
                *loop_ex.offset(l as isize) = *contour1_ex.offset(i as isize);
                let fresh1 = l;
                l = l + 1;
                *loop_ey.offset(fresh1 as isize) = *contour1_ey.offset(i as isize);
                i += 1;
            }
            *loop_x.offset(l as isize) = (*minutia2).x;
            *loop_y.offset(l as isize) = (*minutia2).y;
            *loop_ex.offset(l as isize) = (*minutia2).ex;
            let fresh2 = l;
            l = l + 1;
            *loop_ey.offset(fresh2 as isize) = (*minutia2).ey;
            i = 0 as libc::c_int;
            while i < ncontour2 {
                *loop_x.offset(l as isize) = *contour2_x.offset(i as isize);
                *loop_y.offset(l as isize) = *contour2_y.offset(i as isize);
                *loop_ex.offset(l as isize) = *contour2_ex.offset(i as isize);
                let fresh3 = l;
                l = l + 1;
                *loop_ey.offset(fresh3 as isize) = *contour2_ey.offset(i as isize);
                i += 1;
            }
            free_contour(contour1_x, contour1_y, contour1_ex, contour1_ey);
            free_contour(contour2_x, contour2_y, contour2_ex, contour2_ey);
            *ocontour_x = loop_x;
            *ocontour_y = loop_y;
            *ocontour_ex = loop_ex;
            *ocontour_ey = loop_ey;
            *oncontour = nloop;
            return 1 as libc::c_int;
        }
        if ret == 0 as libc::c_int {
            free_contour(contour1_x, contour1_y, contour1_ex, contour1_ey);
            free_contour(contour2_x, contour2_y, contour2_ex, contour2_ey);
            return 0 as libc::c_int;
        }
        free_contour(contour1_x, contour1_y, contour1_ex, contour1_ey);
        return ret;
    }
    if ret == 0 as libc::c_int {
        free_contour(contour1_x, contour1_y, contour1_ex, contour1_ey);
        return 0 as libc::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn on_hook(
    mut minutia1: *const MINUTIA,
    mut minutia2: *const MINUTIA,
    max_hook_len: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut contour_x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_y: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut contour_ey: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ncontour: libc::c_int = 0;
    ret = trace_contour(
        &mut contour_x,
        &mut contour_y,
        &mut contour_ex,
        &mut contour_ey,
        &mut ncontour,
        max_hook_len,
        (*minutia2).x,
        (*minutia2).y,
        (*minutia1).ex,
        (*minutia1).ey,
        (*minutia1).x,
        (*minutia1).y,
        0 as libc::c_int,
        bdata,
        iw,
        ih,
    );
    if ret == 2 as libc::c_int {
        return ret;
    }
    if ret == 1 as libc::c_int {
        free_contour(contour_x, contour_y, contour_ex, contour_ey);
        return 1 as libc::c_int;
    }
    if ret != 0 as libc::c_int {
        return ret;
    }
    free_contour(contour_x, contour_y, contour_ex, contour_ey);
    ret = trace_contour(
        &mut contour_x,
        &mut contour_y,
        &mut contour_ex,
        &mut contour_ey,
        &mut ncontour,
        max_hook_len,
        (*minutia2).x,
        (*minutia2).y,
        (*minutia1).ex,
        (*minutia1).ey,
        (*minutia1).x,
        (*minutia1).y,
        1 as libc::c_int,
        bdata,
        iw,
        ih,
    );
    if ret == 2 as libc::c_int {
        return ret;
    }
    if ret == 1 as libc::c_int {
        free_contour(contour_x, contour_y, contour_ex, contour_ey);
        return 1 as libc::c_int;
    }
    if ret == 0 as libc::c_int {
        free_contour(contour_x, contour_y, contour_ex, contour_ey);
        return 0 as libc::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn is_loop_clockwise(
    mut contour_x: *const libc::c_int,
    mut contour_y: *const libc::c_int,
    ncontour: libc::c_int,
    default_ret: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut chain: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nchain: libc::c_int = 0;
    ret = chain_code_loop(&mut chain, &mut nchain, contour_x, contour_y, ncontour);
    if ret != 0 {
        return ret;
    }
    if nchain == 0 as libc::c_int {
        return default_ret;
    }
    ret = is_chain_clockwise(chain, nchain, default_ret);
    g_free(chain as gpointer);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn process_loop_V2(
    mut minutiae: *mut MINUTIAE,
    mut contour_x: *const libc::c_int,
    mut contour_y: *const libc::c_int,
    mut contour_ex: *const libc::c_int,
    mut contour_ey: *const libc::c_int,
    ncontour: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut plow_flow_map: *mut libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut idir: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut appearing: libc::c_int = 0;
    let mut min_dist: libc::c_double = 0.;
    let mut max_dist: libc::c_double = 0.;
    let mut min_fr: libc::c_int = 0;
    let mut max_fr: libc::c_int = 0;
    let mut min_to: libc::c_int = 0;
    let mut max_to: libc::c_int = 0;
    let mut mid_x: libc::c_int = 0;
    let mut mid_y: libc::c_int = 0;
    let mut mid_pix: libc::c_int = 0;
    let mut feature_pix: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut minutia: *mut MINUTIA = 0 as *mut MINUTIA;
    let mut fmapval: libc::c_int = 0;
    let mut reliability: libc::c_double = 0.;
    if ncontour <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if ncontour > (*lfsparms).min_loop_len {
        feature_pix = *bdata
            .offset((*contour_y.offset(0 as libc::c_int as isize) * iw) as isize)
            .offset(*contour_x.offset(0 as libc::c_int as isize) as isize)
            as libc::c_int;
        get_loop_aspect(
            &mut min_fr,
            &mut min_to,
            &mut min_dist,
            &mut max_fr,
            &mut max_to,
            &mut max_dist,
            contour_x,
            contour_y,
            ncontour,
        );
        if min_dist < (*lfsparms).min_loop_aspect_dist
            || max_dist / min_dist >= (*lfsparms).min_loop_aspect_ratio
        {
            mid_x = *contour_x.offset(max_fr as isize)
                + *contour_x.offset(max_to as isize) >> 1 as libc::c_int;
            mid_y = *contour_y.offset(max_fr as isize)
                + *contour_y.offset(max_to as isize) >> 1 as libc::c_int;
            mid_pix = *bdata.offset((mid_y * iw) as isize).offset(mid_x as isize)
                as libc::c_int;
            if mid_pix == feature_pix {
                idir = line2direction(
                    *contour_x.offset(max_fr as isize),
                    *contour_y.offset(max_fr as isize),
                    *contour_x.offset(max_to as isize),
                    *contour_y.offset(max_to as isize),
                    (*lfsparms).num_directions,
                );
                type_0 = minutia_type(feature_pix);
                appearing = is_minutia_appearing(
                    *contour_x.offset(max_fr as isize),
                    *contour_y.offset(max_fr as isize),
                    *contour_ex.offset(max_fr as isize),
                    *contour_ey.offset(max_fr as isize),
                );
                if appearing < 0 as libc::c_int {
                    return appearing;
                }
                fmapval = *plow_flow_map
                    .offset((*contour_y.offset(max_fr as isize) * iw) as isize)
                    .offset(*contour_x.offset(max_fr as isize) as isize);
                if fmapval != 0 {
                    reliability = 0.50f64;
                } else {
                    reliability = 0.99f64;
                }
                ret = create_minutia(
                    &mut minutia,
                    *contour_x.offset(max_fr as isize),
                    *contour_y.offset(max_fr as isize),
                    *contour_ex.offset(max_fr as isize),
                    *contour_ey.offset(max_fr as isize),
                    idir,
                    reliability,
                    type_0,
                    appearing,
                    10 as libc::c_int,
                );
                if ret != 0 {
                    return ret;
                }
                ret = update_minutiae(minutiae, minutia, bdata, iw, ih, lfsparms);
                if ret == 2 as libc::c_int {
                    free_minutia(minutia);
                }
                idir += (*lfsparms).num_directions;
                idir %= (*lfsparms).num_directions << 1 as libc::c_int;
                appearing = is_minutia_appearing(
                    *contour_x.offset(max_to as isize),
                    *contour_y.offset(max_to as isize),
                    *contour_ex.offset(max_to as isize),
                    *contour_ey.offset(max_to as isize),
                );
                if appearing < 0 as libc::c_int {
                    return appearing;
                }
                fmapval = *plow_flow_map
                    .offset((*contour_y.offset(max_to as isize) * iw) as isize)
                    .offset(*contour_x.offset(max_to as isize) as isize);
                if fmapval != 0 {
                    reliability = 0.50f64;
                } else {
                    reliability = 0.99f64;
                }
                ret = create_minutia(
                    &mut minutia,
                    *contour_x.offset(max_to as isize),
                    *contour_y.offset(max_to as isize),
                    *contour_ex.offset(max_to as isize),
                    *contour_ey.offset(max_to as isize),
                    idir,
                    reliability,
                    type_0,
                    appearing,
                    10 as libc::c_int,
                );
                if ret != 0 {
                    return ret;
                }
                ret = update_minutiae(minutiae, minutia, bdata, iw, ih, lfsparms);
                if ret == 2 as libc::c_int {
                    free_minutia(minutia);
                }
                return 0 as libc::c_int;
            }
        }
    }
    ret = fill_loop(contour_x, contour_y, ncontour, bdata, iw, ih);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn get_loop_aspect(
    mut omin_fr: *mut libc::c_int,
    mut omin_to: *mut libc::c_int,
    mut omin_dist: *mut libc::c_double,
    mut omax_fr: *mut libc::c_int,
    mut omax_to: *mut libc::c_int,
    mut omax_dist: *mut libc::c_double,
    mut contour_x: *const libc::c_int,
    mut contour_y: *const libc::c_int,
    ncontour: libc::c_int,
) {
    let mut halfway: libc::c_int = 0;
    let mut limit: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut dist: libc::c_double = 0.;
    let mut min_dist: libc::c_double = 0.;
    let mut max_dist: libc::c_double = 0.;
    let mut min_i: libc::c_int = 0;
    let mut max_i: libc::c_int = 0;
    let mut min_j: libc::c_int = 0;
    let mut max_j: libc::c_int = 0;
    halfway = ncontour >> 1 as libc::c_int;
    i = 0 as libc::c_int;
    j = halfway;
    dist = squared_distance(
        *contour_x.offset(i as isize),
        *contour_y.offset(i as isize),
        *contour_x.offset(j as isize),
        *contour_y.offset(j as isize),
    );
    min_dist = dist;
    min_i = i;
    min_j = j;
    max_dist = dist;
    max_i = i;
    max_j = j;
    i += 1;
    j += 1;
    j %= ncontour;
    if ncontour % 2 as libc::c_int != 0 {
        limit = ncontour;
    } else {
        limit = halfway;
    }
    while i < limit {
        dist = squared_distance(
            *contour_x.offset(i as isize),
            *contour_y.offset(i as isize),
            *contour_x.offset(j as isize),
            *contour_y.offset(j as isize),
        );
        if dist < min_dist {
            min_dist = dist;
            min_i = i;
            min_j = j;
        }
        if dist > max_dist {
            max_dist = dist;
            max_i = i;
            max_j = j;
        }
        i += 1;
        j += 1;
        j %= ncontour;
    }
    *omin_fr = min_i;
    *omin_to = min_j;
    *omin_dist = min_dist;
    *omax_fr = max_i;
    *omax_to = max_j;
    *omax_dist = max_dist;
}
#[no_mangle]
pub unsafe extern "C" fn fill_loop(
    mut contour_x: *const libc::c_int,
    mut contour_y: *const libc::c_int,
    ncontour: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) -> libc::c_int {
    let mut shape: *mut SHAPE = 0 as *mut SHAPE;
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut lastj: libc::c_int = 0;
    let mut next_pix: libc::c_int = 0;
    let mut feature_pix: libc::c_int = 0;
    let mut edge_pix: libc::c_int = 0;
    ret = shape_from_contour(&mut shape, contour_x, contour_y, ncontour);
    if ret != 0 {
        return ret;
    }
    feature_pix = *bdata
        .offset((*contour_y.offset(0 as libc::c_int as isize) * iw) as isize)
        .offset(*contour_x.offset(0 as libc::c_int as isize) as isize) as libc::c_int;
    if feature_pix != 0 {
        edge_pix = 0 as libc::c_int;
    } else {
        edge_pix = 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*shape).nrows {
        y = (**((*shape).rows).offset(i as isize)).y;
        if (**((*shape).rows).offset(i as isize)).npts < 1 as libc::c_int {
            free_shape(shape);
            fprintf(
                stderr,
                b"WARNING : fill_loop : unexpected shape, preempting loop fill\n\0"
                    as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
        j = 0 as libc::c_int;
        x = *((**((*shape).rows).offset(i as isize)).xs).offset(j as isize);
        *bdata.offset((y * iw) as isize).offset(x as isize) = edge_pix as libc::c_uchar;
        lastj = (**((*shape).rows).offset(i as isize)).npts - 1 as libc::c_int;
        while j < lastj {
            x += 1;
            next_pix = *bdata.offset((y * iw) as isize).offset(x as isize)
                as libc::c_int;
            if next_pix == edge_pix {
                j += 1;
                x = *((**((*shape).rows).offset(i as isize)).xs).offset(j as isize);
                *bdata
                    .offset((y * iw) as isize)
                    .offset(x as isize) = edge_pix as libc::c_uchar;
            } else {
                j += 1;
                nx = *((**((*shape).rows).offset(i as isize)).xs).offset(j as isize);
                fill_partial_row(edge_pix, x, nx, y, bdata, iw, ih);
            }
        }
        i += 1;
    }
    free_shape(shape);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fill_partial_row(
    fill_pix: libc::c_int,
    frx: libc::c_int,
    tox: libc::c_int,
    y: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let mut bptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    bptr = bdata.offset((y * iw) as isize).offset(frx as isize);
    x = frx;
    while x <= tox {
        *bptr = fill_pix as libc::c_uchar;
        bptr = bptr.offset(1);
        x += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn flood_fill4(
    fill_pix: libc::c_int,
    x: libc::c_int,
    y: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) {
    let mut pptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut y_north: libc::c_int = 0;
    let mut y_south: libc::c_int = 0;
    let mut x_east: libc::c_int = 0;
    let mut x_west: libc::c_int = 0;
    pptr = bdata.offset((y * iw) as isize).offset(x as isize);
    if *pptr as libc::c_int != fill_pix {
        *pptr = fill_pix as libc::c_uchar;
        y_north = y - 1 as libc::c_int;
        y_south = y + 1 as libc::c_int;
        x_west = x - 1 as libc::c_int;
        x_east = x + 1 as libc::c_int;
        if y_north >= 0 as libc::c_int {
            flood_fill4(fill_pix, x, y_north, bdata, iw, ih);
        }
        if x_east < iw {
            flood_fill4(fill_pix, x_east, y, bdata, iw, ih);
        }
        if y_south < ih {
            flood_fill4(fill_pix, x, y_south, bdata, iw, ih);
        }
        if x_west >= 0 as libc::c_int {
            flood_fill4(fill_pix, x_west, y, bdata, iw, ih);
        }
    }
}
