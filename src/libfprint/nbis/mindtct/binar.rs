use ::libc;
extern "C" {
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn fill_holes(_: *mut libc::c_uchar, _: libc::c_int, _: libc::c_int);
}
pub type gsize = libc::c_ulong;
pub type gpointer = *mut libc::c_void;
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
pub unsafe extern "C" fn binarize_V2(
    mut odata: *mut *mut libc::c_uchar,
    mut ow: *mut libc::c_int,
    mut oh: *mut libc::c_int,
    mut pdata: *mut libc::c_uchar,
    pw: libc::c_int,
    ph: libc::c_int,
    mut direction_map: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    mut dirbingrids: *const ROTGRIDS,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut bdata: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut bw: libc::c_int = 0;
    let mut bh: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    ret = binarize_image_V2(
        &mut bdata,
        &mut bw,
        &mut bh,
        pdata,
        pw,
        ph,
        direction_map,
        mw,
        mh,
        (*lfsparms).blocksize,
        dirbingrids,
    );
    if ret != 0 {
        return ret;
    }
    i = 0 as libc::c_int;
    while i < (*lfsparms).num_fill_holes {
        fill_holes(bdata, bw, bh);
        i += 1;
    }
    *odata = bdata;
    *ow = bw;
    *oh = bh;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binarize_image_V2(
    mut odata: *mut *mut libc::c_uchar,
    mut ow: *mut libc::c_int,
    mut oh: *mut libc::c_int,
    mut pdata: *mut libc::c_uchar,
    pw: libc::c_int,
    ph: libc::c_int,
    mut direction_map: *const libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    blocksize: libc::c_int,
    mut dirbingrids: *const ROTGRIDS,
) -> libc::c_int {
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut bw: libc::c_int = 0;
    let mut bh: libc::c_int = 0;
    let mut bx: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    let mut mapval: libc::c_int = 0;
    let mut bdata: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut pptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut spptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    bw = pw - ((*dirbingrids).pad << 1 as libc::c_int);
    bh = ph - ((*dirbingrids).pad << 1 as libc::c_int);
    bdata = g_malloc(
        ((bw * bh) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    bptr = bdata;
    spptr = pdata
        .offset(((*dirbingrids).pad * pw) as isize)
        .offset((*dirbingrids).pad as isize);
    iy = 0 as libc::c_int;
    while iy < bh {
        pptr = spptr;
        ix = 0 as libc::c_int;
        while ix < bw {
            bx = ix / blocksize;
            by = iy / blocksize;
            mapval = *direction_map.offset((by * mw) as isize).offset(bx as isize);
            if mapval == -(1 as libc::c_int) {
                *bptr = 255 as libc::c_int as libc::c_uchar;
            } else {
                *bptr = dirbinarize(pptr, mapval, dirbingrids) as libc::c_uchar;
            }
            pptr = pptr.offset(1);
            bptr = bptr.offset(1);
            ix += 1;
        }
        spptr = spptr.offset(pw as isize);
        iy += 1;
    }
    *odata = bdata;
    *ow = bw;
    *oh = bh;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dirbinarize(
    mut pptr: *const libc::c_uchar,
    idir: libc::c_int,
    mut dirbingrids: *const ROTGRIDS,
) -> libc::c_int {
    let mut gx: libc::c_int = 0;
    let mut gy: libc::c_int = 0;
    let mut gi: libc::c_int = 0;
    let mut cy: libc::c_int = 0;
    let mut rsum: libc::c_int = 0;
    let mut gsum: libc::c_int = 0;
    let mut csum: libc::c_int = 0 as libc::c_int;
    let mut grid: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dcy: libc::c_double = 0.;
    grid = *((*dirbingrids).grids).offset(idir as isize);
    dcy = ((*dirbingrids).grid_h - 1 as libc::c_int) as libc::c_double / 2.0f64;
    dcy = if dcy < 0.0f64 {
        (dcy * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
    } else {
        (dcy * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
    };
    cy = (if dcy < 0 as libc::c_int as libc::c_double {
        dcy - 0.5f64
    } else {
        dcy + 0.5f64
    }) as libc::c_int;
    gi = 0 as libc::c_int;
    gsum = 0 as libc::c_int;
    gy = 0 as libc::c_int;
    while gy < (*dirbingrids).grid_h {
        rsum = 0 as libc::c_int;
        gx = 0 as libc::c_int;
        while gx < (*dirbingrids).grid_w {
            rsum += *pptr.offset(*grid.offset(gi as isize) as isize) as libc::c_int;
            gi += 1;
            gx += 1;
        }
        gsum += rsum;
        if gy == cy {
            csum = rsum;
        }
        gy += 1;
    }
    if csum * (*dirbingrids).grid_h < gsum {
        return 0 as libc::c_int
    } else {
        return 255 as libc::c_int
    };
}
