use ::libc;
extern "C" {
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
    fn line_points(
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
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
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
}
pub type gsize = libc::c_ulong;
pub type gpointer = *mut libc::c_void;
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
pub unsafe extern "C" fn bits_8to6(
    mut idata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut isize: libc::c_int = 0;
    let mut iptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    isize = iw * ih;
    iptr = idata;
    i = 0 as libc::c_int;
    while i < isize {
        let fresh0 = iptr;
        iptr = iptr.offset(1);
        *fresh0 = (*fresh0 as libc::c_int >> 2 as libc::c_int) as libc::c_uchar;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gray2bin(
    thresh: libc::c_int,
    less_pix: libc::c_int,
    greater_pix: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < iw * ih {
        if *bdata.offset(i as isize) as libc::c_int >= thresh {
            *bdata.offset(i as isize) = greater_pix as libc::c_uchar;
        } else {
            *bdata.offset(i as isize) = less_pix as libc::c_uchar;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn pad_uchar_image(
    mut optr: *mut *mut libc::c_uchar,
    mut ow: *mut libc::c_int,
    mut oh: *mut libc::c_int,
    mut idata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    pad: libc::c_int,
    pad_value: libc::c_int,
) -> libc::c_int {
    let mut pdata: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut pptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut iptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut pw: libc::c_int = 0;
    let mut ph: libc::c_int = 0;
    let mut pad2: libc::c_int = 0;
    let mut psize: libc::c_int = 0;
    pad2 = pad << 1 as libc::c_int;
    pw = iw + pad2;
    ph = ih + pad2;
    psize = pw * ph;
    pdata = g_malloc(
        (psize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    memset(pdata as *mut libc::c_void, pad_value, psize as libc::c_ulong);
    iptr = idata;
    pptr = pdata.offset((pad * pw) as isize).offset(pad as isize);
    i = 0 as libc::c_int;
    while i < ih {
        memcpy(
            pptr as *mut libc::c_void,
            iptr as *const libc::c_void,
            iw as libc::c_ulong,
        );
        iptr = iptr.offset(iw as isize);
        pptr = pptr.offset(pw as isize);
        i += 1;
    }
    *optr = pdata;
    *ow = pw;
    *oh = ph;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fill_holes(
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) {
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut iw2: libc::c_int = 0;
    let mut lptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut mptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut rptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    sptr = bdata.offset(1 as libc::c_int as isize);
    iy = 0 as libc::c_int;
    while iy < ih {
        lptr = sptr.offset(-(1 as libc::c_int as isize));
        mptr = sptr;
        rptr = sptr.offset(1 as libc::c_int as isize);
        ix = 1 as libc::c_int;
        while ix < iw - 1 as libc::c_int {
            if *lptr as libc::c_int != *mptr as libc::c_int
                && *lptr as libc::c_int == *rptr as libc::c_int
            {
                *mptr = *lptr;
                lptr = lptr.offset(2 as libc::c_int as isize);
                mptr = mptr.offset(2 as libc::c_int as isize);
                rptr = rptr.offset(2 as libc::c_int as isize);
                ix += 1;
            } else {
                lptr = lptr.offset(1);
                mptr = mptr.offset(1);
                rptr = rptr.offset(1);
            }
            ix += 1;
        }
        sptr = sptr.offset(iw as isize);
        iy += 1;
    }
    iw2 = iw << 1 as libc::c_int;
    sptr = bdata.offset(iw as isize);
    ix = 0 as libc::c_int;
    while ix < iw {
        tptr = sptr.offset(-(iw as isize));
        mptr = sptr;
        bptr = sptr.offset(iw as isize);
        iy = 1 as libc::c_int;
        while iy < ih - 1 as libc::c_int {
            if *tptr as libc::c_int != *mptr as libc::c_int
                && *tptr as libc::c_int == *bptr as libc::c_int
            {
                *mptr = *tptr;
                tptr = tptr.offset(iw2 as isize);
                mptr = mptr.offset(iw2 as isize);
                bptr = bptr.offset(iw2 as isize);
                iy += 1;
            } else {
                tptr = tptr.offset(iw as isize);
                mptr = mptr.offset(iw as isize);
                bptr = bptr.offset(iw as isize);
            }
            iy += 1;
        }
        sptr = sptr.offset(1);
        ix += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn free_path(
    x1: libc::c_int,
    y1: libc::c_int,
    x2: libc::c_int,
    y2: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut x_list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut y_list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut num: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut trans: libc::c_int = 0;
    let mut preval: libc::c_int = 0;
    let mut nextval: libc::c_int = 0;
    ret = line_points(&mut x_list, &mut y_list, &mut num, x1, y1, x2, y2);
    if ret != 0 {
        return ret;
    }
    trans = 0 as libc::c_int;
    preval = *bdata.offset((y1 * iw) as isize).offset(x1 as isize) as libc::c_int;
    i = 1 as libc::c_int;
    while i < num {
        nextval = *bdata
            .offset((*y_list.offset(i as isize) * iw) as isize)
            .offset(*x_list.offset(i as isize) as isize) as libc::c_int;
        if nextval != preval {
            trans += 1;
            if trans > (*lfsparms).maxtrans {
                g_free(x_list as gpointer);
                g_free(y_list as gpointer);
                return 0 as libc::c_int;
            }
            preval = nextval;
        }
        i += 1;
    }
    g_free(x_list as gpointer);
    g_free(y_list as gpointer);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn search_in_direction(
    mut ox: *mut libc::c_int,
    mut oy: *mut libc::c_int,
    mut oex: *mut libc::c_int,
    mut oey: *mut libc::c_int,
    pix: libc::c_int,
    strt_x: libc::c_int,
    strt_y: libc::c_int,
    delta_x: libc::c_double,
    delta_y: libc::c_double,
    maxsteps: libc::c_int,
    mut bdata: *mut libc::c_uchar,
    iw: libc::c_int,
    ih: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut px: libc::c_int = 0;
    let mut py: libc::c_int = 0;
    let mut fx: libc::c_double = 0.;
    let mut fy: libc::c_double = 0.;
    px = strt_x;
    py = strt_y;
    fx = strt_x as libc::c_double;
    fy = strt_y as libc::c_double;
    i = 0 as libc::c_int;
    while i < maxsteps {
        fx += delta_x;
        fy += delta_y;
        x = (if fx < 0 as libc::c_int as libc::c_double {
            fx - 0.5f64
        } else {
            fx + 0.5f64
        }) as libc::c_int;
        y = (if fy < 0 as libc::c_int as libc::c_double {
            fy - 0.5f64
        } else {
            fy + 0.5f64
        }) as libc::c_int;
        if x < 0 as libc::c_int || x >= iw || y < 0 as libc::c_int || y >= ih {
            *ox = -(1 as libc::c_int);
            *oy = -(1 as libc::c_int);
            *oex = -(1 as libc::c_int);
            *oey = -(1 as libc::c_int);
            return 0 as libc::c_int;
        }
        if *bdata.offset((y * iw) as isize).offset(x as isize) as libc::c_int == pix {
            fix_edge_pixel_pair(&mut x, &mut y, &mut px, &mut py, bdata, iw, ih);
            *ox = x;
            *oy = y;
            *oex = px;
            *oey = py;
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        px = x;
        py = y;
        i += 1;
    }
    *ox = -(1 as libc::c_int);
    *oy = -(1 as libc::c_int);
    *oex = -(1 as libc::c_int);
    *oey = -(1 as libc::c_int);
    return 0 as libc::c_int;
}
