use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn gray2bin(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn binarize_V2(
        _: *mut *mut libc::c_uchar,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *const ROTGRIDS,
        _: *const LFSPARMS,
    ) -> libc::c_int;
    fn alloc_minutiae(_: *mut *mut MINUTIAE, _: libc::c_int) -> libc::c_int;
    fn free_rotgrids(_: *mut ROTGRIDS);
    fn init_rotgrids(
        _: *mut *mut ROTGRIDS,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn free_dftwaves(_: *mut DFTWAVES);
    fn free_dir2rad(_: *mut DIR2RAD);
    fn gen_image_maps(
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: *const DIR2RAD,
        _: *const DFTWAVES,
        _: *const ROTGRIDS,
        _: *const LFSPARMS,
    ) -> libc::c_int;
    fn bits_8to6(_: *mut libc::c_uchar, _: libc::c_int, _: libc::c_int);
    fn pad_uchar_image(
        _: *mut *mut libc::c_uchar,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn init_dftwaves(
        _: *mut *mut DFTWAVES,
        _: *const libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn init_dir2rad(_: *mut *mut DIR2RAD, _: libc::c_int) -> libc::c_int;
    fn get_max_padding_V2(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn detect_minutiae_V2(
        _: *mut MINUTIAE,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *const LFSPARMS,
    ) -> libc::c_int;
    fn free_minutiae(_: *mut MINUTIAE);
    fn count_minutiae_ridges(
        _: *mut MINUTIAE,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: *const LFSPARMS,
    ) -> libc::c_int;
    fn remove_false_minutia_V2(
        _: *mut MINUTIAE,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *const LFSPARMS,
    ) -> libc::c_int;
    static mut g_dft_coefs: [libc::c_double; 0];
    fn open_logfile() -> libc::c_int;
    fn close_logfile() -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dir2rad {
    pub ndirs: libc::c_int,
    pub cos: *mut libc::c_double,
    pub sin: *mut libc::c_double,
}
pub type DIR2RAD = dir2rad;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dftwave {
    pub cos: *mut libc::c_double,
    pub sin: *mut libc::c_double,
}
pub type DFTWAVE = dftwave;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dftwaves {
    pub nwaves: libc::c_int,
    pub wavelen: libc::c_int,
    pub waves: *mut *mut DFTWAVE,
}
pub type DFTWAVES = dftwaves;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rotgrids {
    pub pad: libc::c_int,
    pub relative2: libc::c_int,
    pub start_angle: libc::c_double,
    pub ngrids: libc::c_int,
    pub grid_w: libc::c_int,
    pub grid_h: libc::c_int,
    pub grids: *mut *mut libc::c_int,
}
pub type ROTGRIDS = rotgrids;
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
pub unsafe extern "C" fn lfs_detect_minutiae_V2(
    mut ominutiae: *mut *mut MINUTIAE,
    mut odmap: *mut *mut libc::c_int,
    mut olcmap: *mut *mut libc::c_int,
    mut olfmap: *mut *mut libc::c_int,
    mut ohcmap: *mut *mut libc::c_int,
    mut omw: *mut libc::c_int,
    mut omh: *mut libc::c_int,
    mut obdata: *mut *mut libc::c_uchar,
    mut obw: *mut libc::c_int,
    mut obh: *mut libc::c_int,
    mut idata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut pdata: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bdata: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut pw: libc::c_int = 0;
    let mut ph: libc::c_int = 0;
    let mut bw: libc::c_int = 0;
    let mut bh: libc::c_int = 0;
    let mut dir2rad: *mut DIR2RAD = 0 as *mut DIR2RAD;
    let mut dftwaves: *mut DFTWAVES = 0 as *mut DFTWAVES;
    let mut dftgrids: *mut ROTGRIDS = 0 as *mut ROTGRIDS;
    let mut dirbingrids: *mut ROTGRIDS = 0 as *mut ROTGRIDS;
    let mut direction_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut low_contrast_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut low_flow_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut high_curve_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut mw: libc::c_int = 0;
    let mut mh: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut maxpad: libc::c_int = 0;
    let mut minutiae: *mut MINUTIAE = 0 as *mut MINUTIAE;
    ret = open_logfile();
    if ret != 0 {
        return ret;
    }
    maxpad = get_max_padding_V2(
        (*lfsparms).windowsize,
        (*lfsparms).windowoffset,
        (*lfsparms).dirbin_grid_w,
        (*lfsparms).dirbin_grid_h,
    );
    ret = init_dir2rad(&mut dir2rad, (*lfsparms).num_directions);
    if ret != 0 {
        return ret;
    }
    ret = init_dftwaves(
        &mut dftwaves,
        g_dft_coefs.as_mut_ptr(),
        (*lfsparms).num_dft_waves,
        (*lfsparms).windowsize,
    );
    if ret != 0 {
        free_dir2rad(dir2rad);
        return ret;
    }
    ret = init_rotgrids(
        &mut dftgrids,
        iw,
        ih,
        maxpad,
        (*lfsparms).start_dir_angle,
        (*lfsparms).num_directions,
        (*lfsparms).windowsize,
        (*lfsparms).windowsize,
        1 as libc::c_int,
    );
    if ret != 0 {
        free_dir2rad(dir2rad);
        free_dftwaves(dftwaves);
        return ret;
    }
    if maxpad > 0 as libc::c_int {
        ret = pad_uchar_image(
            &mut pdata,
            &mut pw,
            &mut ph,
            idata,
            iw,
            ih,
            maxpad,
            (*lfsparms).pad_value,
        );
        if ret != 0 {
            free_dir2rad(dir2rad);
            free_dftwaves(dftwaves);
            free_rotgrids(dftgrids);
            return ret;
        }
    } else {
        pdata = g_malloc((iw * ih) as gsize) as *mut libc::c_uchar;
        memcpy(
            pdata as *mut libc::c_void,
            idata as *const libc::c_void,
            (iw * ih) as libc::c_ulong,
        );
        pw = iw;
        ph = ih;
    }
    bits_8to6(pdata, pw, ph);
    print2log(
        b"\nINITIALIZATION AND PADDING DONE\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    ret = gen_image_maps(
        &mut direction_map,
        &mut low_contrast_map,
        &mut low_flow_map,
        &mut high_curve_map,
        &mut mw,
        &mut mh,
        pdata,
        pw,
        ph,
        dir2rad,
        dftwaves,
        dftgrids,
        lfsparms,
    );
    if ret != 0 {
        free_dir2rad(dir2rad);
        free_dftwaves(dftwaves);
        free_rotgrids(dftgrids);
        g_free(pdata as gpointer);
        return ret;
    }
    free_dir2rad(dir2rad);
    free_dftwaves(dftwaves);
    free_rotgrids(dftgrids);
    print2log(
        b"\nMAPS DONE\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    ret = init_rotgrids(
        &mut dirbingrids,
        iw,
        ih,
        maxpad,
        (*lfsparms).start_dir_angle,
        (*lfsparms).num_directions,
        (*lfsparms).dirbin_grid_w,
        (*lfsparms).dirbin_grid_h,
        0 as libc::c_int,
    );
    if ret != 0 {
        g_free(pdata as gpointer);
        g_free(direction_map as gpointer);
        g_free(low_contrast_map as gpointer);
        g_free(low_flow_map as gpointer);
        g_free(high_curve_map as gpointer);
        return ret;
    }
    ret = binarize_V2(
        &mut bdata,
        &mut bw,
        &mut bh,
        pdata,
        pw,
        ph,
        direction_map,
        mw,
        mh,
        dirbingrids,
        lfsparms,
    );
    if ret != 0 {
        g_free(pdata as gpointer);
        g_free(direction_map as gpointer);
        g_free(low_contrast_map as gpointer);
        g_free(low_flow_map as gpointer);
        g_free(high_curve_map as gpointer);
        free_rotgrids(dirbingrids);
        return ret;
    }
    free_rotgrids(dirbingrids);
    if iw != bw || ih != bh {
        g_free(pdata as gpointer);
        g_free(direction_map as gpointer);
        g_free(low_contrast_map as gpointer);
        g_free(low_flow_map as gpointer);
        g_free(high_curve_map as gpointer);
        g_free(bdata as gpointer);
        fprintf(
            stderr,
            b"ERROR : lfs_detect_minutiae_V2 :\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"binary image has bad dimensions : %d, %d\n\0" as *const u8
                as *const libc::c_char,
            bw,
            bh,
        );
        return -(581 as libc::c_int);
    }
    print2log(
        b"\nBINARIZATION DONE\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    gray2bin(1 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int, bdata, iw, ih);
    ret = alloc_minutiae(&mut minutiae, 1000 as libc::c_int);
    if ret != 0 {
        return ret;
    }
    ret = detect_minutiae_V2(
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
        g_free(pdata as gpointer);
        g_free(direction_map as gpointer);
        g_free(low_contrast_map as gpointer);
        g_free(low_flow_map as gpointer);
        g_free(high_curve_map as gpointer);
        g_free(bdata as gpointer);
        return ret;
    }
    ret = remove_false_minutia_V2(
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
        g_free(pdata as gpointer);
        g_free(direction_map as gpointer);
        g_free(low_contrast_map as gpointer);
        g_free(low_flow_map as gpointer);
        g_free(high_curve_map as gpointer);
        g_free(bdata as gpointer);
        free_minutiae(minutiae);
        return ret;
    }
    print2log(
        b"\nMINUTIA DETECTION DONE\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    ret = count_minutiae_ridges(minutiae, bdata, iw, ih, lfsparms);
    if ret != 0 {
        g_free(pdata as gpointer);
        g_free(direction_map as gpointer);
        g_free(low_contrast_map as gpointer);
        g_free(low_flow_map as gpointer);
        g_free(high_curve_map as gpointer);
        free_minutiae(minutiae);
        return ret;
    }
    print2log(
        b"\nNEIGHBOR RIDGE COUNT DONE\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    gray2bin(1 as libc::c_int, 255 as libc::c_int, 0 as libc::c_int, bdata, iw, ih);
    g_free(pdata as gpointer);
    *odmap = direction_map;
    *olcmap = low_contrast_map;
    *olfmap = low_flow_map;
    *ohcmap = high_curve_map;
    *omw = mw;
    *omh = mh;
    *obdata = bdata;
    *obw = bw;
    *obh = bh;
    *ominutiae = minutiae;
    ret = close_logfile();
    if ret != 0 {
        return ret;
    }
    return 0 as libc::c_int;
}
