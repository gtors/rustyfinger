use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn gen_quality_map(
        _: *mut *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn combined_minutia_quality(
        _: *mut MINUTIAE,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_double,
    ) -> libc::c_int;
    fn free_minutiae(_: *mut MINUTIAE);
    fn lfs_detect_minutiae_V2(
        _: *mut *mut MINUTIAE,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut *mut libc::c_uchar,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: *const LFSPARMS,
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
pub unsafe extern "C" fn get_minutiae(
    mut ominutiae: *mut *mut MINUTIAE,
    mut oquality_map: *mut *mut libc::c_int,
    mut odirection_map: *mut *mut libc::c_int,
    mut olow_contrast_map: *mut *mut libc::c_int,
    mut olow_flow_map: *mut *mut libc::c_int,
    mut ohigh_curve_map: *mut *mut libc::c_int,
    mut omap_w: *mut libc::c_int,
    mut omap_h: *mut libc::c_int,
    mut obdata: *mut *mut libc::c_uchar,
    mut obw: *mut libc::c_int,
    mut obh: *mut libc::c_int,
    mut obd: *mut libc::c_int,
    mut idata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    id: libc::c_int,
    ppmm: libc::c_double,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut minutiae: *mut MINUTIAE = 0 as *mut MINUTIAE;
    let mut direction_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut low_contrast_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut low_flow_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut high_curve_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut quality_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut map_w: libc::c_int = 0;
    let mut map_h: libc::c_int = 0;
    let mut bdata: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bw: libc::c_int = 0;
    let mut bh: libc::c_int = 0;
    if id != 8 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR : get_minutiae : input image pixel \0" as *const u8
                as *const libc::c_char,
        );
        fprintf(stderr, b"depth = %d != 8.\n\0" as *const u8 as *const libc::c_char, id);
        return -(2 as libc::c_int);
    }
    ret = lfs_detect_minutiae_V2(
        &mut minutiae,
        &mut direction_map,
        &mut low_contrast_map,
        &mut low_flow_map,
        &mut high_curve_map,
        &mut map_w,
        &mut map_h,
        &mut bdata,
        &mut bw,
        &mut bh,
        idata,
        iw,
        ih,
        lfsparms,
    );
    if ret != 0 {
        return ret;
    }
    ret = gen_quality_map(
        &mut quality_map,
        direction_map,
        low_contrast_map,
        low_flow_map,
        high_curve_map,
        map_w,
        map_h,
    );
    if ret != 0 {
        free_minutiae(minutiae);
        g_free(direction_map as gpointer);
        g_free(low_contrast_map as gpointer);
        g_free(low_flow_map as gpointer);
        g_free(high_curve_map as gpointer);
        g_free(bdata as gpointer);
        return ret;
    }
    ret = combined_minutia_quality(
        minutiae,
        quality_map,
        map_w,
        map_h,
        (*lfsparms).blocksize,
        idata,
        iw,
        ih,
        id,
        ppmm,
    );
    if ret != 0 {
        free_minutiae(minutiae);
        g_free(direction_map as gpointer);
        g_free(low_contrast_map as gpointer);
        g_free(low_flow_map as gpointer);
        g_free(high_curve_map as gpointer);
        g_free(quality_map as gpointer);
        g_free(bdata as gpointer);
        return ret;
    }
    *ominutiae = minutiae;
    *oquality_map = quality_map;
    *odirection_map = direction_map;
    *olow_contrast_map = low_contrast_map;
    *olow_flow_map = low_flow_map;
    *ohigh_curve_map = high_curve_map;
    *omap_w = map_w;
    *omap_h = map_h;
    *obdata = bdata;
    *obw = bw;
    *obh = bh;
    *obd = id;
    return 0 as libc::c_int;
}
