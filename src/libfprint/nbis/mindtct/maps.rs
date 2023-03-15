use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
    fn block_offsets(
        _: *mut *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn low_contrast_block(
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: *const LFSPARMS,
    ) -> libc::c_int;
    fn find_valid_block(
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn set_margin_blocks(
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn dft_dir_powers(
        _: *mut *mut libc::c_double,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *const DFTWAVES,
        _: *const ROTGRIDS,
    ) -> libc::c_int;
    fn dft_power_stats(
        _: *mut libc::c_int,
        _: *mut libc::c_double,
        _: *mut libc::c_int,
        _: *mut libc::c_double,
        _: *mut *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn free_dir_powers(_: *mut *mut libc::c_double, _: libc::c_int);
    fn alloc_dir_powers(
        _: *mut *mut *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn alloc_power_stats(
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_double,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_double,
        _: libc::c_int,
    ) -> libc::c_int;
    fn closest_dir_dist(_: libc::c_int, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn erode_charimage_2(
        _: *mut libc::c_uchar,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn dilate_charimage_2(
        _: *mut libc::c_uchar,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
    );
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
pub unsafe extern "C" fn gen_image_maps(
    mut odmap: *mut *mut libc::c_int,
    mut olcmap: *mut *mut libc::c_int,
    mut olfmap: *mut *mut libc::c_int,
    mut ohcmap: *mut *mut libc::c_int,
    mut omw: *mut libc::c_int,
    mut omh: *mut libc::c_int,
    mut pdata: *mut libc::c_uchar,
    pw: libc::c_int,
    ph: libc::c_int,
    mut dir2rad: *const DIR2RAD,
    mut dftwaves: *const DFTWAVES,
    mut dftgrids: *const ROTGRIDS,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut direction_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut low_contrast_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut low_flow_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut high_curve_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut mw: libc::c_int = 0;
    let mut mh: libc::c_int = 0;
    let mut iw: libc::c_int = 0;
    let mut ih: libc::c_int = 0;
    let mut blkoffs: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ret: libc::c_int = 0;
    if (*dftgrids).grid_w != (*dftgrids).grid_h {
        fprintf(
            stderr,
            b"ERROR : gen_image_maps : DFT grids must be square\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(540 as libc::c_int);
    }
    iw = pw - ((*dftgrids).pad << 1 as libc::c_int);
    ih = ph - ((*dftgrids).pad << 1 as libc::c_int);
    ret = block_offsets(
        &mut blkoffs,
        &mut mw,
        &mut mh,
        iw,
        ih,
        (*dftgrids).pad,
        (*lfsparms).blocksize,
    );
    if ret != 0 {
        return ret;
    }
    ret = gen_initial_maps(
        &mut direction_map,
        &mut low_contrast_map,
        &mut low_flow_map,
        blkoffs,
        mw,
        mh,
        pdata,
        pw,
        ph,
        dftwaves,
        dftgrids,
        lfsparms,
    );
    if ret != 0 {
        g_free(blkoffs as gpointer);
        return ret;
    }
    ret = morph_TF_map(low_flow_map, mw, mh, lfsparms);
    if ret != 0 {
        g_free(direction_map as gpointer);
        g_free(low_contrast_map as gpointer);
        g_free(low_flow_map as gpointer);
        return ret;
    }
    remove_incon_dirs(direction_map, mw, mh, dir2rad, lfsparms);
    smooth_direction_map(direction_map, low_contrast_map, mw, mh, dir2rad, lfsparms);
    ret = interpolate_direction_map(direction_map, low_contrast_map, mw, mh, lfsparms);
    if ret != 0 {
        g_free(direction_map as gpointer);
        g_free(low_contrast_map as gpointer);
        g_free(low_flow_map as gpointer);
        return ret;
    }
    remove_incon_dirs(direction_map, mw, mh, dir2rad, lfsparms);
    smooth_direction_map(direction_map, low_contrast_map, mw, mh, dir2rad, lfsparms);
    set_margin_blocks(direction_map, mw, mh, -(1 as libc::c_int));
    ret = gen_high_curve_map(&mut high_curve_map, direction_map, mw, mh, lfsparms);
    if ret != 0 {
        g_free(direction_map as gpointer);
        g_free(low_contrast_map as gpointer);
        g_free(low_flow_map as gpointer);
        return ret;
    }
    g_free(blkoffs as gpointer);
    *odmap = direction_map;
    *olcmap = low_contrast_map;
    *olfmap = low_flow_map;
    *ohcmap = high_curve_map;
    *omw = mw;
    *omh = mh;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gen_initial_maps(
    mut odmap: *mut *mut libc::c_int,
    mut olcmap: *mut *mut libc::c_int,
    mut olfmap: *mut *mut libc::c_int,
    mut blkoffs: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut pdata: *mut libc::c_uchar,
    pw: libc::c_int,
    ph: libc::c_int,
    mut dftwaves: *const DFTWAVES,
    mut dftgrids: *const ROTGRIDS,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut direction_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut low_contrast_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut low_flow_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut bi: libc::c_int = 0;
    let mut bsize: libc::c_int = 0;
    let mut blkdir: libc::c_int = 0;
    let mut wis: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut powmax_dirs: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut powers: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut powmaxs: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pownorms: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut nstats: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut dft_offset: libc::c_int = 0;
    let mut xminlimit: libc::c_int = 0;
    let mut xmaxlimit: libc::c_int = 0;
    let mut yminlimit: libc::c_int = 0;
    let mut ymaxlimit: libc::c_int = 0;
    let mut win_x: libc::c_int = 0;
    let mut win_y: libc::c_int = 0;
    let mut low_contrast_offset: libc::c_int = 0;
    print2log(
        b"INITIAL MAP\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut dest: gsize = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        let (fresh0, fresh1) = mw.overflowing_mul(mh);
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
            b"../libfprint/nbis/mindtct/maps.c\0" as *const u8 as *const libc::c_char,
            275 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"gen_initial_maps\0"))
                .as_ptr(),
            b"g_size_checked_mul(&dest, mw, mh)\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if dest < 2147483647 as libc::c_int as libc::c_ulong {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/maps.c\0" as *const u8 as *const libc::c_char,
            275 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"gen_initial_maps\0"))
                .as_ptr(),
            b"dest < G_MAXINT\0" as *const u8 as *const libc::c_char,
        );
    }
    bsize = mw * mh;
    direction_map = g_malloc(
        (bsize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    memset(
        direction_map as *mut libc::c_void,
        -(1 as libc::c_int),
        (bsize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    low_contrast_map = g_malloc(
        (bsize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    memset(
        low_contrast_map as *mut libc::c_void,
        0 as libc::c_int,
        (bsize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    low_flow_map = g_malloc(
        (bsize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    memset(
        low_flow_map as *mut libc::c_void,
        0 as libc::c_int,
        (bsize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    ret = alloc_dir_powers(&mut powers, (*dftwaves).nwaves, (*dftgrids).ngrids);
    if ret != 0 {
        g_free(direction_map as gpointer);
        g_free(low_contrast_map as gpointer);
        g_free(low_flow_map as gpointer);
        return ret;
    }
    nstats = (*dftwaves).nwaves - 1 as libc::c_int;
    ret = alloc_power_stats(
        &mut wis,
        &mut powmaxs,
        &mut powmax_dirs,
        &mut pownorms,
        nstats,
    );
    if ret != 0 {
        g_free(direction_map as gpointer);
        g_free(low_contrast_map as gpointer);
        g_free(low_flow_map as gpointer);
        free_dir_powers(powers, (*dftwaves).nwaves);
        return ret;
    }
    xminlimit = (*dftgrids).pad;
    yminlimit = (*dftgrids).pad;
    xmaxlimit = pw - (*dftgrids).pad - (*lfsparms).windowsize - 1 as libc::c_int;
    ymaxlimit = ph - (*dftgrids).pad - (*lfsparms).windowsize - 1 as libc::c_int;
    bi = 0 as libc::c_int;
    while bi < bsize {
        dft_offset = *blkoffs.offset(bi as isize) - (*lfsparms).windowoffset * pw
            - (*lfsparms).windowoffset;
        win_x = dft_offset % pw;
        win_y = dft_offset / pw;
        win_x = if xminlimit > win_x { xminlimit } else { win_x };
        win_x = if xmaxlimit < win_x { xmaxlimit } else { win_x };
        win_y = if yminlimit > win_y { yminlimit } else { win_y };
        win_y = if ymaxlimit < win_y { ymaxlimit } else { win_y };
        low_contrast_offset = win_y * pw + win_x;
        print2log(
            b"   BLOCK %2d (%2d, %2d) \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            bi,
            bi % mw,
            bi / mw,
        );
        ret = low_contrast_block(
            low_contrast_offset,
            (*lfsparms).windowsize,
            pdata,
            pw,
            ph,
            lfsparms,
        );
        if ret != 0 {
            if ret < 0 as libc::c_int {
                g_free(direction_map as gpointer);
                g_free(low_contrast_map as gpointer);
                g_free(low_flow_map as gpointer);
                free_dir_powers(powers, (*dftwaves).nwaves);
                g_free(wis as gpointer);
                g_free(powmaxs as gpointer);
                g_free(powmax_dirs as gpointer);
                g_free(pownorms as gpointer);
                return ret;
            }
            print2log(
                b"LOW CONTRAST\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            *low_contrast_map
                .offset(bi as isize) = (0 as libc::c_int == 0) as libc::c_int;
        } else {
            print2log(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            ret = dft_dir_powers(
                powers,
                pdata,
                low_contrast_offset,
                pw,
                ph,
                dftwaves,
                dftgrids,
            );
            if ret != 0 {
                g_free(direction_map as gpointer);
                g_free(low_contrast_map as gpointer);
                g_free(low_flow_map as gpointer);
                free_dir_powers(powers, (*dftwaves).nwaves);
                g_free(wis as gpointer);
                g_free(powmaxs as gpointer);
                g_free(powmax_dirs as gpointer);
                g_free(pownorms as gpointer);
                return ret;
            }
            ret = dft_power_stats(
                wis,
                powmaxs,
                powmax_dirs,
                pownorms,
                powers,
                1 as libc::c_int,
                (*dftwaves).nwaves,
                (*dftgrids).ngrids,
            );
            if ret != 0 {
                g_free(direction_map as gpointer);
                g_free(low_contrast_map as gpointer);
                g_free(low_flow_map as gpointer);
                free_dir_powers(powers, (*dftwaves).nwaves);
                g_free(wis as gpointer);
                g_free(powmaxs as gpointer);
                g_free(powmax_dirs as gpointer);
                g_free(pownorms as gpointer);
                return ret;
            }
            blkdir = primary_dir_test(
                powers,
                wis,
                powmaxs,
                powmax_dirs,
                pownorms,
                nstats,
                lfsparms,
            );
            if blkdir != -(1 as libc::c_int) {
                *direction_map.offset(bi as isize) = blkdir;
            } else {
                blkdir = secondary_fork_test(
                    powers,
                    wis,
                    powmaxs,
                    powmax_dirs,
                    pownorms,
                    nstats,
                    lfsparms,
                );
                if blkdir != -(1 as libc::c_int) {
                    *direction_map.offset(bi as isize) = blkdir;
                } else {
                    *low_flow_map
                        .offset(bi as isize) = (0 as libc::c_int == 0) as libc::c_int;
                }
            }
        }
        bi += 1;
    }
    free_dir_powers(powers, (*dftwaves).nwaves);
    g_free(wis as gpointer);
    g_free(powmaxs as gpointer);
    g_free(powmax_dirs as gpointer);
    g_free(pownorms as gpointer);
    *odmap = direction_map;
    *olcmap = low_contrast_map;
    *olfmap = low_flow_map;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn interpolate_direction_map(
    mut direction_map: *mut libc::c_int,
    mut low_contrast_map: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut new_dir: libc::c_int = 0;
    let mut n_dir: libc::c_int = 0;
    let mut e_dir: libc::c_int = 0;
    let mut s_dir: libc::c_int = 0;
    let mut w_dir: libc::c_int = 0;
    let mut n_dist: libc::c_int = 0 as libc::c_int;
    let mut e_dist: libc::c_int = 0 as libc::c_int;
    let mut s_dist: libc::c_int = 0 as libc::c_int;
    let mut w_dist: libc::c_int = 0 as libc::c_int;
    let mut total_dist: libc::c_int = 0;
    let mut n_found: libc::c_int = 0;
    let mut e_found: libc::c_int = 0;
    let mut s_found: libc::c_int = 0;
    let mut w_found: libc::c_int = 0;
    let mut total_found: libc::c_int = 0;
    let mut n_delta: libc::c_int = 0 as libc::c_int;
    let mut e_delta: libc::c_int = 0 as libc::c_int;
    let mut s_delta: libc::c_int = 0 as libc::c_int;
    let mut w_delta: libc::c_int = 0 as libc::c_int;
    let mut total_delta: libc::c_int = 0;
    let mut nbr_x: libc::c_int = 0;
    let mut nbr_y: libc::c_int = 0;
    let mut omap: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut optr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut avr_dir: libc::c_double = 0.;
    print2log(
        b"INTERPOLATE DIRECTION MAP\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    let mut dest: gsize = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        let (fresh2, fresh3) = mw.overflowing_mul(mh);
        *(&mut dest as *mut gsize) = fresh2;
        if !fresh3 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/maps.c\0" as *const u8 as *const libc::c_char,
            488 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"interpolate_direction_map\0"))
                .as_ptr(),
            b"g_size_checked_mul(&dest, mw, mh)\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut dest_0: gsize = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        let (fresh4, fresh5) = (mw * mh)
            .overflowing_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
        *(&mut dest_0 as *mut gsize) = fresh4;
        if !fresh5 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/maps.c\0" as *const u8 as *const libc::c_char,
            489 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"interpolate_direction_map\0"))
                .as_ptr(),
            b"g_size_checked_mul(&dest, mw * mh, sizeof(int))\0" as *const u8
                as *const libc::c_char,
        );
    }
    omap = g_malloc(
        ((mw * mh) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    dptr = direction_map;
    cptr = low_contrast_map;
    optr = omap;
    y = 0 as libc::c_int;
    while y < mh {
        x = 0 as libc::c_int;
        while x < mw {
            if *cptr == 0 && *dptr == -(1 as libc::c_int) {
                total_found = 0 as libc::c_int;
                total_dist = 0 as libc::c_int;
                n_found = find_valid_block(
                    &mut n_dir,
                    &mut nbr_x,
                    &mut nbr_y,
                    direction_map,
                    low_contrast_map,
                    x,
                    y,
                    mw,
                    mh,
                    0 as libc::c_int,
                    -(1 as libc::c_int),
                );
                if n_found == (0 as libc::c_int == 0) as libc::c_int {
                    n_dist = y - nbr_y;
                    total_dist += n_dist;
                    total_found += 1;
                }
                e_found = find_valid_block(
                    &mut e_dir,
                    &mut nbr_x,
                    &mut nbr_y,
                    direction_map,
                    low_contrast_map,
                    x,
                    y,
                    mw,
                    mh,
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
                if e_found == (0 as libc::c_int == 0) as libc::c_int {
                    e_dist = nbr_x - x;
                    total_dist += e_dist;
                    total_found += 1;
                }
                s_found = find_valid_block(
                    &mut s_dir,
                    &mut nbr_x,
                    &mut nbr_y,
                    direction_map,
                    low_contrast_map,
                    x,
                    y,
                    mw,
                    mh,
                    0 as libc::c_int,
                    1 as libc::c_int,
                );
                if s_found == (0 as libc::c_int == 0) as libc::c_int {
                    s_dist = nbr_y - y;
                    total_dist += s_dist;
                    total_found += 1;
                }
                w_found = find_valid_block(
                    &mut w_dir,
                    &mut nbr_x,
                    &mut nbr_y,
                    direction_map,
                    low_contrast_map,
                    x,
                    y,
                    mw,
                    mh,
                    -(1 as libc::c_int),
                    0 as libc::c_int,
                );
                if w_found == (0 as libc::c_int == 0) as libc::c_int {
                    w_dist = x - nbr_x;
                    total_dist += w_dist;
                    total_found += 1;
                }
                if total_found >= (*lfsparms).min_interpolate_nbrs {
                    total_delta = 0.0f64 as libc::c_int;
                    if n_found != 0 {
                        n_delta = total_dist - n_dist;
                        total_delta += n_delta;
                    }
                    if e_found != 0 {
                        e_delta = total_dist - e_dist;
                        total_delta += e_delta;
                    }
                    if s_found != 0 {
                        s_delta = total_dist - s_dist;
                        total_delta += s_delta;
                    }
                    if w_found != 0 {
                        w_delta = total_dist - w_dist;
                        total_delta += w_delta;
                    }
                    avr_dir = 0.0f64;
                    if n_found != 0 {
                        avr_dir
                            += n_dir as libc::c_double
                                * (n_delta as libc::c_double
                                    / total_delta as libc::c_double);
                    }
                    if e_found != 0 {
                        avr_dir
                            += e_dir as libc::c_double
                                * (e_delta as libc::c_double
                                    / total_delta as libc::c_double);
                    }
                    if s_found != 0 {
                        avr_dir
                            += s_dir as libc::c_double
                                * (s_delta as libc::c_double
                                    / total_delta as libc::c_double);
                    }
                    if w_found != 0 {
                        avr_dir
                            += w_dir as libc::c_double
                                * (w_delta as libc::c_double
                                    / total_delta as libc::c_double);
                    }
                    avr_dir = if avr_dir < 0.0f64 {
                        (avr_dir * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double
                            / 16384.0f64
                    } else {
                        (avr_dir * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double
                            / 16384.0f64
                    };
                    new_dir = (if avr_dir < 0 as libc::c_int as libc::c_double {
                        avr_dir - 0.5f64
                    } else {
                        avr_dir + 0.5f64
                    }) as libc::c_int;
                    print2log(
                        b"   Block %d,%d INTERP numnbs=%d newdir=%d\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        x,
                        y,
                        total_found,
                        new_dir,
                    );
                    *optr = new_dir;
                } else {
                    *optr = *dptr;
                }
            } else {
                *optr = *dptr;
            }
            dptr = dptr.offset(1);
            cptr = cptr.offset(1);
            optr = optr.offset(1);
            x += 1;
        }
        y += 1;
    }
    memcpy(
        direction_map as *mut libc::c_void,
        omap as *const libc::c_void,
        ((mw * mh) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    g_free(omap as gpointer);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn morph_TF_map(
    mut tfmap: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut cimage: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut mimage: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut cptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut mptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut dest: gsize = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        let (fresh6, fresh7) = mw.overflowing_mul(mh);
        *(&mut dest as *mut gsize) = fresh6;
        if !fresh7 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/maps.c\0" as *const u8 as *const libc::c_char,
            657 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"morph_TF_map\0"))
                .as_ptr(),
            b"g_size_checked_mul(&dest, mw, mh)\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if dest < 2147483647 as libc::c_int as libc::c_ulong {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/maps.c\0" as *const u8 as *const libc::c_char,
            657 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"morph_TF_map\0"))
                .as_ptr(),
            b"dest < G_MAXINT\0" as *const u8 as *const libc::c_char,
        );
    }
    cimage = g_malloc((mw * mh) as gsize) as *mut libc::c_uchar;
    mimage = g_malloc((mw * mh) as gsize) as *mut libc::c_uchar;
    cptr = cimage;
    mptr = tfmap;
    i = 0 as libc::c_int;
    while i < mw * mh {
        let fresh8 = mptr;
        mptr = mptr.offset(1);
        let fresh9 = cptr;
        cptr = cptr.offset(1);
        *fresh9 = *fresh8 as libc::c_uchar;
        i += 1;
    }
    dilate_charimage_2(cimage, mimage, mw, mh);
    dilate_charimage_2(mimage, cimage, mw, mh);
    erode_charimage_2(cimage, mimage, mw, mh);
    erode_charimage_2(mimage, cimage, mw, mh);
    cptr = cimage;
    mptr = tfmap;
    i = 0 as libc::c_int;
    while i < mw * mh {
        let fresh10 = cptr;
        cptr = cptr.offset(1);
        let fresh11 = mptr;
        mptr = mptr.offset(1);
        *fresh11 = *fresh10 as libc::c_int;
        i += 1;
    }
    g_free(cimage as gpointer);
    g_free(mimage as gpointer);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pixelize_map(
    mut omap: *mut *mut libc::c_int,
    iw: libc::c_int,
    ih: libc::c_int,
    mut imap: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    blocksize: libc::c_int,
) -> libc::c_int {
    let mut pmap: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut blkoffs: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut bw: libc::c_int = 0;
    let mut bh: libc::c_int = 0;
    let mut bi: libc::c_int = 0;
    let mut spptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dest: gsize = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        let (fresh12, fresh13) = iw.overflowing_mul(ih);
        *(&mut dest as *mut gsize) = fresh12;
        if !fresh13 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/maps.c\0" as *const u8 as *const libc::c_char,
            714 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"pixelize_map\0"))
                .as_ptr(),
            b"g_size_checked_mul(&dest, iw, ih)\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut dest_0: gsize = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        let (fresh14, fresh15) = (iw * ih)
            .overflowing_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
        *(&mut dest_0 as *mut gsize) = fresh14;
        if !fresh15 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/maps.c\0" as *const u8 as *const libc::c_char,
            715 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"pixelize_map\0"))
                .as_ptr(),
            b"g_size_checked_mul(&dest, iw * ih, sizeof(int))\0" as *const u8
                as *const libc::c_char,
        );
    }
    pmap = g_malloc(
        ((iw * ih) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    ret = block_offsets(
        &mut blkoffs,
        &mut bw,
        &mut bh,
        iw,
        ih,
        0 as libc::c_int,
        blocksize,
    );
    if ret != 0 {
        g_free(pmap as gpointer);
        return ret;
    }
    if bw != mw || bh != mh {
        g_free(blkoffs as gpointer);
        g_free(pmap as gpointer);
        fprintf(
            stderr,
            b"ERROR : pixelize_map : block dimensions do not match\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(591 as libc::c_int);
    }
    bi = 0 as libc::c_int;
    while bi < mw * mh {
        spptr = pmap.offset(*blkoffs.offset(bi as isize) as isize);
        y = 0 as libc::c_int;
        while y < blocksize {
            pptr = spptr;
            x = 0 as libc::c_int;
            while x < blocksize {
                let fresh16 = pptr;
                pptr = pptr.offset(1);
                *fresh16 = *imap.offset(bi as isize);
                x += 1;
            }
            spptr = spptr.offset(iw as isize);
            y += 1;
        }
        bi += 1;
    }
    g_free(blkoffs as gpointer);
    *omap = pmap;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn smooth_direction_map(
    mut direction_map: *mut libc::c_int,
    mut low_contrast_map: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut dir2rad: *const DIR2RAD,
    mut lfsparms: *const LFSPARMS,
) {
    let mut mx: libc::c_int = 0;
    let mut my: libc::c_int = 0;
    let mut dptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut avrdir: libc::c_int = 0;
    let mut nvalid: libc::c_int = 0;
    let mut dir_strength: libc::c_double = 0.;
    print2log(
        b"SMOOTH DIRECTION MAP\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    dptr = direction_map;
    cptr = low_contrast_map;
    my = 0 as libc::c_int;
    while my < mh {
        mx = 0 as libc::c_int;
        while mx < mw {
            if *cptr == 0 {
                average_8nbr_dir(
                    &mut avrdir,
                    &mut dir_strength,
                    &mut nvalid,
                    direction_map,
                    mx,
                    my,
                    mw,
                    mh,
                    dir2rad,
                );
                if dir_strength >= (*lfsparms).dir_strength_min {
                    if *dptr != -(1 as libc::c_int) {
                        if nvalid >= (*lfsparms).rmv_valid_nbr_min {
                            *dptr = avrdir;
                        }
                    } else if nvalid >= (*lfsparms).smth_valid_nbr_min {
                        *dptr = avrdir;
                    }
                }
            }
            dptr = dptr.offset(1);
            cptr = cptr.offset(1);
            mx += 1;
        }
        my += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gen_high_curve_map(
    mut ohcmap: *mut *mut libc::c_int,
    mut direction_map: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut high_curve_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut mapsize: libc::c_int = 0;
    let mut hptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut bx: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    let mut nvalid: libc::c_int = 0;
    let mut cmeasure: libc::c_int = 0;
    let mut vmeasure: libc::c_int = 0;
    let mut dest: gsize = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        let (fresh17, fresh18) = mw.overflowing_mul(mh);
        *(&mut dest as *mut gsize) = fresh17;
        if !fresh18 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/maps.c\0" as *const u8 as *const libc::c_char,
            872 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"gen_high_curve_map\0"))
                .as_ptr(),
            b"g_size_checked_mul(&dest, mw, mh)\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if dest < 2147483647 as libc::c_int as libc::c_ulong {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/maps.c\0" as *const u8 as *const libc::c_char,
            872 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"gen_high_curve_map\0"))
                .as_ptr(),
            b"dest < G_MAXINT\0" as *const u8 as *const libc::c_char,
        );
    }
    mapsize = mw * mh;
    let mut dest_0: gsize = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        let (fresh19, fresh20) = mapsize
            .overflowing_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
        *(&mut dest_0 as *mut gsize) = fresh19;
        if !fresh20 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/nbis/mindtct/maps.c\0" as *const u8 as *const libc::c_char,
            876 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"gen_high_curve_map\0"))
                .as_ptr(),
            b"g_size_checked_mul(&dest, mapsize, sizeof(int))\0" as *const u8
                as *const libc::c_char,
        );
    }
    high_curve_map = g_malloc(
        (mapsize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    memset(
        high_curve_map as *mut libc::c_void,
        0 as libc::c_int,
        (mapsize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    hptr = high_curve_map;
    dptr = direction_map;
    by = 0 as libc::c_int;
    while by < mh {
        bx = 0 as libc::c_int;
        while bx < mw {
            nvalid = num_valid_8nbrs(direction_map, bx, by, mw, mh);
            if nvalid > 0 as libc::c_int {
                if *dptr == -(1 as libc::c_int) {
                    if nvalid >= (*lfsparms).vort_valid_nbr_min {
                        vmeasure = vorticity(
                            direction_map,
                            bx,
                            by,
                            mw,
                            mh,
                            (*lfsparms).num_directions,
                        );
                        if vmeasure >= (*lfsparms).highcurv_vorticity_min {
                            *hptr = (0 as libc::c_int == 0) as libc::c_int;
                        }
                    }
                } else {
                    cmeasure = curvature(
                        direction_map,
                        bx,
                        by,
                        mw,
                        mh,
                        (*lfsparms).num_directions,
                    );
                    if cmeasure >= (*lfsparms).highcurv_curvature_min {
                        *hptr = (0 as libc::c_int == 0) as libc::c_int;
                    }
                }
            }
            dptr = dptr.offset(1);
            hptr = hptr.offset(1);
            bx += 1;
        }
        by += 1;
    }
    *ohcmap = high_curve_map;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn primary_dir_test(
    mut powers: *mut *mut libc::c_double,
    mut wis: *const libc::c_int,
    mut powmaxs: *const libc::c_double,
    mut powmax_dirs: *const libc::c_int,
    mut pownorms: *const libc::c_double,
    nstats: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    print2log(
        b"      Primary\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    w = 0 as libc::c_int;
    while w < nstats {
        if *powmaxs.offset(*wis.offset(w as isize) as isize) > (*lfsparms).powmax_min
            && *pownorms.offset(*wis.offset(w as isize) as isize)
                > (*lfsparms).pownorm_min
            && *(*powers.offset(0 as libc::c_int as isize))
                .offset(*powmax_dirs.offset(*wis.offset(w as isize) as isize) as isize)
                <= (*lfsparms).powmax_max
        {
            return *powmax_dirs.offset(*wis.offset(w as isize) as isize);
        }
        w += 1;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn secondary_fork_test(
    mut powers: *mut *mut libc::c_double,
    mut wis: *const libc::c_int,
    mut powmaxs: *const libc::c_double,
    mut powmax_dirs: *const libc::c_int,
    mut pownorms: *const libc::c_double,
    nstats: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut ldir: libc::c_int = 0;
    let mut rdir: libc::c_int = 0;
    let mut fork_pownorm_min: libc::c_double = 0.;
    let mut fork_pow_thresh: libc::c_double = 0.;
    fork_pownorm_min = (*lfsparms).fork_pct_pownorm * (*lfsparms).pownorm_min;
    if *powmaxs.offset(*wis.offset(0 as libc::c_int as isize) as isize)
        > (*lfsparms).powmax_min
        && *pownorms.offset(*wis.offset(0 as libc::c_int as isize) as isize)
            >= fork_pownorm_min
        && *(*powers.offset(0 as libc::c_int as isize))
            .offset(
                *powmax_dirs.offset(*wis.offset(0 as libc::c_int as isize) as isize)
                    as isize,
            ) <= (*lfsparms).powmax_max
    {
        rdir = (*powmax_dirs.offset(*wis.offset(0 as libc::c_int as isize) as isize)
            + (*lfsparms).fork_interval) % (*lfsparms).num_directions;
        ldir = (*powmax_dirs.offset(*wis.offset(0 as libc::c_int as isize) as isize)
            + (*lfsparms).num_directions - (*lfsparms).fork_interval)
            % (*lfsparms).num_directions;
        print2log(
            b"         Left = %d, Current = %d, Right = %d\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ldir,
            *powmax_dirs.offset(*wis.offset(0 as libc::c_int as isize) as isize),
            rdir,
        );
        fork_pow_thresh = *powmaxs
            .offset(*wis.offset(0 as libc::c_int as isize) as isize)
            * (*lfsparms).fork_pct_powmax;
        if (*(*powers
            .offset(
                (*wis.offset(0 as libc::c_int as isize) + 1 as libc::c_int) as isize,
            ))
            .offset(ldir as isize) <= fork_pow_thresh
            || *(*powers
                .offset(
                    (*wis.offset(0 as libc::c_int as isize) + 1 as libc::c_int) as isize,
                ))
                .offset(rdir as isize) <= fork_pow_thresh)
            && (*(*powers
                .offset(
                    (*wis.offset(0 as libc::c_int as isize) + 1 as libc::c_int) as isize,
                ))
                .offset(ldir as isize) > fork_pow_thresh
                || *(*powers
                    .offset(
                        (*wis.offset(0 as libc::c_int as isize) + 1 as libc::c_int)
                            as isize,
                    ))
                    .offset(rdir as isize) > fork_pow_thresh)
        {
            return *powmax_dirs.offset(*wis.offset(0 as libc::c_int as isize) as isize);
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn remove_incon_dirs(
    mut imap: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut dir2rad: *const DIR2RAD,
    mut lfsparms: *const LFSPARMS,
) {
    let mut cx: libc::c_int = 0;
    let mut cy: libc::c_int = 0;
    let mut iptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nremoved: libc::c_int = 0;
    let mut lbox: libc::c_int = 0;
    let mut rbox: libc::c_int = 0;
    let mut tbox: libc::c_int = 0;
    let mut bbox: libc::c_int = 0;
    cx = mw >> 1 as libc::c_int;
    cy = mh >> 1 as libc::c_int;
    loop {
        nremoved = 0 as libc::c_int;
        iptr = imap.offset((cy * mw) as isize).offset(cx as isize);
        if *iptr != -(1 as libc::c_int)
            && remove_dir(imap, cx, cy, mw, mh, dir2rad, lfsparms) != 0
        {
            *iptr = -(1 as libc::c_int);
            nremoved += 1;
        }
        lbox = cx - 1 as libc::c_int;
        tbox = cy - 1 as libc::c_int;
        rbox = cx + 1 as libc::c_int;
        bbox = cy + 1 as libc::c_int;
        while lbox >= 0 as libc::c_int || rbox < mw || tbox >= 0 as libc::c_int
            || bbox < mh
        {
            if tbox >= 0 as libc::c_int {
                nremoved
                    += test_top_edge(
                        lbox,
                        tbox,
                        rbox,
                        bbox,
                        imap,
                        mw,
                        mh,
                        dir2rad,
                        lfsparms,
                    );
            }
            if rbox < mw {
                nremoved
                    += test_right_edge(
                        lbox,
                        tbox,
                        rbox,
                        bbox,
                        imap,
                        mw,
                        mh,
                        dir2rad,
                        lfsparms,
                    );
            }
            if bbox < mh {
                nremoved
                    += test_bottom_edge(
                        lbox,
                        tbox,
                        rbox,
                        bbox,
                        imap,
                        mw,
                        mh,
                        dir2rad,
                        lfsparms,
                    );
            }
            if lbox >= 0 as libc::c_int {
                nremoved
                    += test_left_edge(
                        lbox,
                        tbox,
                        rbox,
                        bbox,
                        imap,
                        mw,
                        mh,
                        dir2rad,
                        lfsparms,
                    );
            }
            lbox -= 1;
            tbox -= 1;
            rbox += 1;
            bbox += 1;
        }
        if !(nremoved != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_top_edge(
    lbox: libc::c_int,
    tbox: libc::c_int,
    rbox: libc::c_int,
    bbox: libc::c_int,
    mut imap: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut dir2rad: *const DIR2RAD,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut bx: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    let mut sx: libc::c_int = 0;
    let mut ex: libc::c_int = 0;
    let mut iptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut sptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut eptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nremoved: libc::c_int = 0;
    nremoved = 0 as libc::c_int;
    sx = if lbox > 0 as libc::c_int { lbox } else { 0 as libc::c_int };
    sptr = imap.offset((tbox * mw) as isize).offset(sx as isize);
    ex = if (rbox - 1 as libc::c_int) < mw - 1 as libc::c_int {
        rbox - 1 as libc::c_int
    } else {
        mw - 1 as libc::c_int
    };
    eptr = imap.offset((tbox * mw) as isize).offset(ex as isize);
    iptr = sptr;
    bx = sx;
    by = tbox;
    while iptr <= eptr {
        if *iptr != -(1 as libc::c_int)
            && remove_dir(imap, bx, by, mw, mh, dir2rad, lfsparms) != 0
        {
            *iptr = -(1 as libc::c_int);
            nremoved += 1;
        }
        iptr = iptr.offset(1);
        bx += 1;
    }
    return nremoved;
}
#[no_mangle]
pub unsafe extern "C" fn test_right_edge(
    lbox: libc::c_int,
    tbox: libc::c_int,
    rbox: libc::c_int,
    bbox: libc::c_int,
    mut imap: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut dir2rad: *const DIR2RAD,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut bx: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    let mut sy: libc::c_int = 0;
    let mut ey: libc::c_int = 0;
    let mut iptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut sptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut eptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nremoved: libc::c_int = 0;
    nremoved = 0 as libc::c_int;
    sy = if tbox > 0 as libc::c_int { tbox } else { 0 as libc::c_int };
    sptr = imap.offset((sy * mw) as isize).offset(rbox as isize);
    ey = if (bbox - 1 as libc::c_int) < mh - 1 as libc::c_int {
        bbox - 1 as libc::c_int
    } else {
        mh - 1 as libc::c_int
    };
    eptr = imap.offset((ey * mw) as isize).offset(rbox as isize);
    iptr = sptr;
    bx = rbox;
    by = sy;
    while iptr <= eptr {
        if *iptr != -(1 as libc::c_int)
            && remove_dir(imap, bx, by, mw, mh, dir2rad, lfsparms) != 0
        {
            *iptr = -(1 as libc::c_int);
            nremoved += 1;
        }
        iptr = iptr.offset(mw as isize);
        by += 1;
    }
    return nremoved;
}
#[no_mangle]
pub unsafe extern "C" fn test_bottom_edge(
    lbox: libc::c_int,
    tbox: libc::c_int,
    rbox: libc::c_int,
    bbox: libc::c_int,
    mut imap: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut dir2rad: *const DIR2RAD,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut bx: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    let mut sx: libc::c_int = 0;
    let mut ex: libc::c_int = 0;
    let mut iptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut sptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut eptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nremoved: libc::c_int = 0;
    nremoved = 0 as libc::c_int;
    sx = if rbox < mw - 1 as libc::c_int { rbox } else { mw - 1 as libc::c_int };
    sptr = imap.offset((bbox * mw) as isize).offset(sx as isize);
    ex = if lbox - 1 as libc::c_int > 0 as libc::c_int {
        lbox - 1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    eptr = imap.offset((bbox * mw) as isize).offset(ex as isize);
    iptr = sptr;
    bx = sx;
    by = bbox;
    while iptr >= eptr {
        if *iptr != -(1 as libc::c_int)
            && remove_dir(imap, bx, by, mw, mh, dir2rad, lfsparms) != 0
        {
            *iptr = -(1 as libc::c_int);
            nremoved += 1;
        }
        iptr = iptr.offset(-1);
        bx -= 1;
    }
    return nremoved;
}
#[no_mangle]
pub unsafe extern "C" fn test_left_edge(
    lbox: libc::c_int,
    tbox: libc::c_int,
    rbox: libc::c_int,
    bbox: libc::c_int,
    mut imap: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut dir2rad: *const DIR2RAD,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut bx: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    let mut sy: libc::c_int = 0;
    let mut ey: libc::c_int = 0;
    let mut iptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut sptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut eptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nremoved: libc::c_int = 0;
    nremoved = 0 as libc::c_int;
    sy = if bbox < mh - 1 as libc::c_int { bbox } else { mh - 1 as libc::c_int };
    sptr = imap.offset((sy * mw) as isize).offset(lbox as isize);
    ey = if tbox - 1 as libc::c_int > 0 as libc::c_int {
        tbox - 1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    eptr = imap.offset((ey * mw) as isize).offset(lbox as isize);
    iptr = sptr;
    bx = lbox;
    by = sy;
    while iptr >= eptr {
        if *iptr != -(1 as libc::c_int)
            && remove_dir(imap, bx, by, mw, mh, dir2rad, lfsparms) != 0
        {
            *iptr = -(1 as libc::c_int);
            nremoved += 1;
        }
        iptr = iptr.offset(-(mw as isize));
        by -= 1;
    }
    return nremoved;
}
#[no_mangle]
pub unsafe extern "C" fn remove_dir(
    mut imap: *mut libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut dir2rad: *const DIR2RAD,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut avrdir: libc::c_int = 0;
    let mut nvalid: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    let mut dir_strength: libc::c_double = 0.;
    average_8nbr_dir(
        &mut avrdir,
        &mut dir_strength,
        &mut nvalid,
        imap,
        mx,
        my,
        mw,
        mh,
        dir2rad,
    );
    if nvalid < (*lfsparms).rmv_valid_nbr_min {
        return 1 as libc::c_int;
    }
    if dir_strength >= (*lfsparms).dir_strength_min {
        dist = abs(avrdir - *imap.offset((my * mw) as isize).offset(mx as isize));
        dist = if dist < (*dir2rad).ndirs - dist {
            dist
        } else {
            (*dir2rad).ndirs - dist
        };
        if dist > (*lfsparms).dir_distance_max {
            return 2 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn average_8nbr_dir(
    mut avrdir: *mut libc::c_int,
    mut dir_strength: *mut libc::c_double,
    mut nvalid: *mut libc::c_int,
    mut imap: *mut libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut dir2rad: *const DIR2RAD,
) {
    let mut iptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut e: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut cospart: libc::c_double = 0.;
    let mut sinpart: libc::c_double = 0.;
    let mut pi2: libc::c_double = 0.;
    let mut pi_factor: libc::c_double = 0.;
    let mut theta: libc::c_double = 0.;
    let mut avr: libc::c_double = 0.;
    e = mx + 1 as libc::c_int;
    w = mx - 1 as libc::c_int;
    n = my - 1 as libc::c_int;
    s = my + 1 as libc::c_int;
    *nvalid = 0 as libc::c_int;
    cospart = 0.0f64;
    sinpart = 0.0f64;
    if w >= 0 as libc::c_int && n >= 0 as libc::c_int {
        iptr = imap.offset((n * mw) as isize).offset(w as isize);
        if *iptr != -(1 as libc::c_int) {
            cospart += *((*dir2rad).cos).offset(*iptr as isize);
            sinpart += *((*dir2rad).sin).offset(*iptr as isize);
            *nvalid += 1;
        }
    }
    if n >= 0 as libc::c_int {
        iptr = imap.offset((n * mw) as isize).offset(mx as isize);
        if *iptr != -(1 as libc::c_int) {
            cospart += *((*dir2rad).cos).offset(*iptr as isize);
            sinpart += *((*dir2rad).sin).offset(*iptr as isize);
            *nvalid += 1;
        }
    }
    if e < mw && n >= 0 as libc::c_int {
        iptr = imap.offset((n * mw) as isize).offset(e as isize);
        if *iptr != -(1 as libc::c_int) {
            cospart += *((*dir2rad).cos).offset(*iptr as isize);
            sinpart += *((*dir2rad).sin).offset(*iptr as isize);
            *nvalid += 1;
        }
    }
    if e < mw {
        iptr = imap.offset((my * mw) as isize).offset(e as isize);
        if *iptr != -(1 as libc::c_int) {
            cospart += *((*dir2rad).cos).offset(*iptr as isize);
            sinpart += *((*dir2rad).sin).offset(*iptr as isize);
            *nvalid += 1;
        }
    }
    if e < mw && s < mh {
        iptr = imap.offset((s * mw) as isize).offset(e as isize);
        if *iptr != -(1 as libc::c_int) {
            cospart += *((*dir2rad).cos).offset(*iptr as isize);
            sinpart += *((*dir2rad).sin).offset(*iptr as isize);
            *nvalid += 1;
        }
    }
    if s < mh {
        iptr = imap.offset((s * mw) as isize).offset(mx as isize);
        if *iptr != -(1 as libc::c_int) {
            cospart += *((*dir2rad).cos).offset(*iptr as isize);
            sinpart += *((*dir2rad).sin).offset(*iptr as isize);
            *nvalid += 1;
        }
    }
    if w >= 0 as libc::c_int && s < mh {
        iptr = imap.offset((s * mw) as isize).offset(w as isize);
        if *iptr != -(1 as libc::c_int) {
            cospart += *((*dir2rad).cos).offset(*iptr as isize);
            sinpart += *((*dir2rad).sin).offset(*iptr as isize);
            *nvalid += 1;
        }
    }
    if w >= 0 as libc::c_int {
        iptr = imap.offset((my * mw) as isize).offset(w as isize);
        if *iptr != -(1 as libc::c_int) {
            cospart += *((*dir2rad).cos).offset(*iptr as isize);
            sinpart += *((*dir2rad).sin).offset(*iptr as isize);
            *nvalid += 1;
        }
    }
    if *nvalid == 0 as libc::c_int {
        *dir_strength = 0 as libc::c_int as libc::c_double;
        *avrdir = -(1 as libc::c_int);
        return;
    }
    cospart /= *nvalid as libc::c_double;
    sinpart /= *nvalid as libc::c_double;
    *dir_strength = cospart * cospart + sinpart * sinpart;
    *dir_strength = if *dir_strength < 0.0f64 {
        (*dir_strength * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double
            / 16384.0f64
    } else {
        (*dir_strength * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double
            / 16384.0f64
    };
    if *dir_strength < 0.2f64 {
        *dir_strength = 0 as libc::c_int as libc::c_double;
        *avrdir = -(1 as libc::c_int);
        return;
    }
    theta = atan2(sinpart, cospart);
    pi2 = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64;
    theta += pi2;
    theta = fmod(theta, pi2);
    pi_factor = pi2 / (*dir2rad).ndirs as libc::c_double;
    avr = theta / pi_factor;
    avr = if avr < 0.0f64 {
        (avr * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
    } else {
        (avr * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
    };
    *avrdir = (if avr < 0 as libc::c_int as libc::c_double {
        avr - 0.5f64
    } else {
        avr + 0.5f64
    }) as libc::c_int;
    *avrdir %= (*dir2rad).ndirs;
}
#[no_mangle]
pub unsafe extern "C" fn num_valid_8nbrs(
    mut imap: *mut libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
) -> libc::c_int {
    let mut e_ind: libc::c_int = 0;
    let mut w_ind: libc::c_int = 0;
    let mut n_ind: libc::c_int = 0;
    let mut s_ind: libc::c_int = 0;
    let mut nvalid: libc::c_int = 0;
    nvalid = 0 as libc::c_int;
    e_ind = mx + 1 as libc::c_int;
    w_ind = mx - 1 as libc::c_int;
    n_ind = my - 1 as libc::c_int;
    s_ind = my + 1 as libc::c_int;
    if w_ind >= 0 as libc::c_int && n_ind >= 0 as libc::c_int
        && *imap.offset((n_ind * mw) as isize).offset(w_ind as isize) >= 0 as libc::c_int
    {
        nvalid += 1;
    }
    if n_ind >= 0 as libc::c_int
        && *imap.offset((n_ind * mw) as isize).offset(mx as isize) >= 0 as libc::c_int
    {
        nvalid += 1;
    }
    if n_ind >= 0 as libc::c_int && e_ind < mw
        && *imap.offset((n_ind * mw) as isize).offset(e_ind as isize) >= 0 as libc::c_int
    {
        nvalid += 1;
    }
    if e_ind < mw
        && *imap.offset((my * mw) as isize).offset(e_ind as isize) >= 0 as libc::c_int
    {
        nvalid += 1;
    }
    if e_ind < mw && s_ind < mh
        && *imap.offset((s_ind * mw) as isize).offset(e_ind as isize) >= 0 as libc::c_int
    {
        nvalid += 1;
    }
    if s_ind < mh
        && *imap.offset((s_ind * mw) as isize).offset(mx as isize) >= 0 as libc::c_int
    {
        nvalid += 1;
    }
    if w_ind >= 0 as libc::c_int && s_ind < mh
        && *imap.offset((s_ind * mw) as isize).offset(w_ind as isize) >= 0 as libc::c_int
    {
        nvalid += 1;
    }
    if w_ind >= 0 as libc::c_int
        && *imap.offset((my * mw) as isize).offset(w_ind as isize) >= 0 as libc::c_int
    {
        nvalid += 1;
    }
    return nvalid;
}
#[no_mangle]
pub unsafe extern "C" fn vorticity(
    mut imap: *mut libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    ndirs: libc::c_int,
) -> libc::c_int {
    let mut e_ind: libc::c_int = 0;
    let mut w_ind: libc::c_int = 0;
    let mut n_ind: libc::c_int = 0;
    let mut s_ind: libc::c_int = 0;
    let mut nw_val: libc::c_int = 0;
    let mut n_val: libc::c_int = 0;
    let mut ne_val: libc::c_int = 0;
    let mut e_val: libc::c_int = 0;
    let mut se_val: libc::c_int = 0;
    let mut s_val: libc::c_int = 0;
    let mut sw_val: libc::c_int = 0;
    let mut w_val: libc::c_int = 0;
    let mut vmeasure: libc::c_int = 0;
    e_ind = mx + 1 as libc::c_int;
    w_ind = mx - 1 as libc::c_int;
    n_ind = my - 1 as libc::c_int;
    s_ind = my + 1 as libc::c_int;
    if w_ind >= 0 as libc::c_int && n_ind >= 0 as libc::c_int {
        nw_val = *imap.offset((n_ind * mw) as isize).offset(w_ind as isize);
    } else {
        nw_val = -(1 as libc::c_int);
    }
    if n_ind >= 0 as libc::c_int {
        n_val = *imap.offset((n_ind * mw) as isize).offset(mx as isize);
    } else {
        n_val = -(1 as libc::c_int);
    }
    if n_ind >= 0 as libc::c_int && e_ind < mw {
        ne_val = *imap.offset((n_ind * mw) as isize).offset(e_ind as isize);
    } else {
        ne_val = -(1 as libc::c_int);
    }
    if e_ind < mw {
        e_val = *imap.offset((my * mw) as isize).offset(e_ind as isize);
    } else {
        e_val = -(1 as libc::c_int);
    }
    if e_ind < mw && s_ind < mh {
        se_val = *imap.offset((s_ind * mw) as isize).offset(e_ind as isize);
    } else {
        se_val = -(1 as libc::c_int);
    }
    if s_ind < mh {
        s_val = *imap.offset((s_ind * mw) as isize).offset(mx as isize);
    } else {
        s_val = -(1 as libc::c_int);
    }
    if w_ind >= 0 as libc::c_int && s_ind < mh {
        sw_val = *imap.offset((s_ind * mw) as isize).offset(w_ind as isize);
    } else {
        sw_val = -(1 as libc::c_int);
    }
    if w_ind >= 0 as libc::c_int {
        w_val = *imap.offset((my * mw) as isize).offset(w_ind as isize);
    } else {
        w_val = -(1 as libc::c_int);
    }
    vmeasure = 0 as libc::c_int;
    accum_nbr_vorticity(&mut vmeasure, nw_val, n_val, ndirs);
    accum_nbr_vorticity(&mut vmeasure, n_val, ne_val, ndirs);
    accum_nbr_vorticity(&mut vmeasure, ne_val, e_val, ndirs);
    accum_nbr_vorticity(&mut vmeasure, e_val, se_val, ndirs);
    accum_nbr_vorticity(&mut vmeasure, se_val, s_val, ndirs);
    accum_nbr_vorticity(&mut vmeasure, s_val, sw_val, ndirs);
    accum_nbr_vorticity(&mut vmeasure, sw_val, w_val, ndirs);
    accum_nbr_vorticity(&mut vmeasure, w_val, nw_val, ndirs);
    return vmeasure;
}
#[no_mangle]
pub unsafe extern "C" fn accum_nbr_vorticity(
    mut vmeasure: *mut libc::c_int,
    dir1: libc::c_int,
    dir2: libc::c_int,
    ndirs: libc::c_int,
) {
    let mut dist: libc::c_int = 0;
    if dir1 != dir2 && dir1 >= 0 as libc::c_int && dir2 >= 0 as libc::c_int {
        dist = dir2 - dir1;
        if dist < 0 as libc::c_int {
            dist += ndirs;
        }
        if dist > ndirs >> 1 as libc::c_int {
            *vmeasure -= 1;
        } else {
            *vmeasure += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn curvature(
    mut imap: *mut libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    ndirs: libc::c_int,
) -> libc::c_int {
    let mut iptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut e_ind: libc::c_int = 0;
    let mut w_ind: libc::c_int = 0;
    let mut n_ind: libc::c_int = 0;
    let mut s_ind: libc::c_int = 0;
    let mut nw_val: libc::c_int = 0;
    let mut n_val: libc::c_int = 0;
    let mut ne_val: libc::c_int = 0;
    let mut e_val: libc::c_int = 0;
    let mut se_val: libc::c_int = 0;
    let mut s_val: libc::c_int = 0;
    let mut sw_val: libc::c_int = 0;
    let mut w_val: libc::c_int = 0;
    let mut cmeasure: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    e_ind = mx + 1 as libc::c_int;
    w_ind = mx - 1 as libc::c_int;
    n_ind = my - 1 as libc::c_int;
    s_ind = my + 1 as libc::c_int;
    if w_ind >= 0 as libc::c_int && n_ind >= 0 as libc::c_int {
        nw_val = *imap.offset((n_ind * mw) as isize).offset(w_ind as isize);
    } else {
        nw_val = -(1 as libc::c_int);
    }
    if n_ind >= 0 as libc::c_int {
        n_val = *imap.offset((n_ind * mw) as isize).offset(mx as isize);
    } else {
        n_val = -(1 as libc::c_int);
    }
    if n_ind >= 0 as libc::c_int && e_ind < mw {
        ne_val = *imap.offset((n_ind * mw) as isize).offset(e_ind as isize);
    } else {
        ne_val = -(1 as libc::c_int);
    }
    if e_ind < mw {
        e_val = *imap.offset((my * mw) as isize).offset(e_ind as isize);
    } else {
        e_val = -(1 as libc::c_int);
    }
    if e_ind < mw && s_ind < mh {
        se_val = *imap.offset((s_ind * mw) as isize).offset(e_ind as isize);
    } else {
        se_val = -(1 as libc::c_int);
    }
    if s_ind < mh {
        s_val = *imap.offset((s_ind * mw) as isize).offset(mx as isize);
    } else {
        s_val = -(1 as libc::c_int);
    }
    if w_ind >= 0 as libc::c_int && s_ind < mh {
        sw_val = *imap.offset((s_ind * mw) as isize).offset(w_ind as isize);
    } else {
        sw_val = -(1 as libc::c_int);
    }
    if w_ind >= 0 as libc::c_int {
        w_val = *imap.offset((my * mw) as isize).offset(w_ind as isize);
    } else {
        w_val = -(1 as libc::c_int);
    }
    iptr = imap.offset((my * mw) as isize).offset(mx as isize);
    cmeasure = -(1 as libc::c_int);
    dist = closest_dir_dist(*iptr, nw_val, ndirs);
    if dist > cmeasure {
        cmeasure = dist;
    }
    dist = closest_dir_dist(*iptr, n_val, ndirs);
    if dist > cmeasure {
        cmeasure = dist;
    }
    dist = closest_dir_dist(*iptr, ne_val, ndirs);
    if dist > cmeasure {
        cmeasure = dist;
    }
    dist = closest_dir_dist(*iptr, e_val, ndirs);
    if dist > cmeasure {
        cmeasure = dist;
    }
    dist = closest_dir_dist(*iptr, se_val, ndirs);
    if dist > cmeasure {
        cmeasure = dist;
    }
    dist = closest_dir_dist(*iptr, s_val, ndirs);
    if dist > cmeasure {
        cmeasure = dist;
    }
    dist = closest_dir_dist(*iptr, sw_val, ndirs);
    if dist > cmeasure {
        cmeasure = dist;
    }
    dist = closest_dir_dist(*iptr, w_val, ndirs);
    if dist > cmeasure {
        cmeasure = dist;
    }
    return cmeasure;
}
