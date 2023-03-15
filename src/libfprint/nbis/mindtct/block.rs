use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn g_malloc(n_bytes: gsize) -> gpointer;
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
pub unsafe extern "C" fn block_offsets(
    mut optr: *mut *mut libc::c_int,
    mut ow: *mut libc::c_int,
    mut oh: *mut libc::c_int,
    iw: libc::c_int,
    ih: libc::c_int,
    pad: libc::c_int,
    blocksize: libc::c_int,
) -> libc::c_int {
    let mut blkoffs: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut bx: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    let mut bw: libc::c_int = 0;
    let mut bh: libc::c_int = 0;
    let mut bi: libc::c_int = 0;
    let mut bsize: libc::c_int = 0;
    let mut blkrow_start: libc::c_int = 0;
    let mut blkrow_size: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut lastbw: libc::c_int = 0;
    let mut lastbh: libc::c_int = 0;
    let mut pad2: libc::c_int = 0;
    let mut pw: libc::c_int = 0;
    if iw < blocksize || ih < blocksize {
        fprintf(
            stderr,
            b"ERROR : block_offsets : image must be at least %d by %d in size\n\0"
                as *const u8 as *const libc::c_char,
            blocksize,
            blocksize,
        );
        return -(80 as libc::c_int);
    }
    pad2 = pad << 1 as libc::c_int;
    pw = iw + pad2;
    bw = ceil(iw as libc::c_double / blocksize as libc::c_double) as libc::c_int;
    bh = ceil(ih as libc::c_double / blocksize as libc::c_double) as libc::c_int;
    bsize = bw * bh;
    lastbw = bw - 1 as libc::c_int;
    lastbh = bh - 1 as libc::c_int;
    blkoffs = g_malloc(
        (bsize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    bi = 0 as libc::c_int;
    blkrow_start = pad * pw + pad;
    blkrow_size = pw * blocksize;
    by = 0 as libc::c_int;
    while by < lastbh {
        offset = blkrow_start;
        bx = 0 as libc::c_int;
        while bx < lastbw {
            let fresh0 = bi;
            bi = bi + 1;
            *blkoffs.offset(fresh0 as isize) = offset;
            offset += blocksize;
            bx += 1;
        }
        let fresh1 = bi;
        bi = bi + 1;
        *blkoffs.offset(fresh1 as isize) = blkrow_start + iw - blocksize;
        blkrow_start += blkrow_size;
        by += 1;
    }
    blkrow_start = (pad + ih - blocksize) * pw + pad;
    offset = blkrow_start;
    bx = 0 as libc::c_int;
    while bx < lastbw {
        let fresh2 = bi;
        bi = bi + 1;
        *blkoffs.offset(fresh2 as isize) = offset;
        offset += blocksize;
        bx += 1;
    }
    let fresh3 = bi;
    bi = bi + 1;
    *blkoffs.offset(fresh3 as isize) = blkrow_start + iw - blocksize;
    *optr = blkoffs;
    *ow = bw;
    *oh = bh;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn low_contrast_block(
    blkoffset: libc::c_int,
    blocksize: libc::c_int,
    mut pdata: *mut libc::c_uchar,
    pw: libc::c_int,
    ph: libc::c_int,
    mut lfsparms: *const LFSPARMS,
) -> libc::c_int {
    let mut pixtable: [libc::c_int; 64] = [0; 64];
    let mut numpix: libc::c_int = 0;
    let mut px: libc::c_int = 0;
    let mut py: libc::c_int = 0;
    let mut pi: libc::c_int = 0;
    let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut pptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut delta: libc::c_int = 0;
    let mut tdbl: libc::c_double = 0.;
    let mut prctmin: libc::c_int = 0 as libc::c_int;
    let mut prctmax: libc::c_int = 0 as libc::c_int;
    let mut prctthresh: libc::c_int = 0;
    let mut pixsum: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    numpix = blocksize * blocksize;
    memset(
        pixtable.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (64 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    tdbl = (*lfsparms).percentile_min_max as libc::c_double / 100.0f64
        * (numpix - 1 as libc::c_int) as libc::c_double;
    tdbl = if tdbl < 0.0f64 {
        (tdbl * 16384.0f64 - 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
    } else {
        (tdbl * 16384.0f64 + 0.5f64) as libc::c_int as libc::c_double / 16384.0f64
    };
    prctthresh = (if tdbl < 0 as libc::c_int as libc::c_double {
        tdbl - 0.5f64
    } else {
        tdbl + 0.5f64
    }) as libc::c_int;
    sptr = pdata.offset(blkoffset as isize);
    py = 0 as libc::c_int;
    while py < blocksize {
        pptr = sptr;
        px = 0 as libc::c_int;
        while px < blocksize {
            pixtable[*pptr as usize] += 1;
            pptr = pptr.offset(1);
            px += 1;
        }
        sptr = sptr.offset(pw as isize);
        py += 1;
    }
    pi = 0 as libc::c_int;
    pixsum = 0 as libc::c_int;
    found = 0 as libc::c_int;
    while pi < 64 as libc::c_int {
        pixsum += pixtable[pi as usize];
        if pixsum >= prctthresh {
            prctmin = pi;
            found = (0 as libc::c_int == 0) as libc::c_int;
            break;
        } else {
            pi += 1;
        }
    }
    if found == 0 {
        fprintf(
            stderr,
            b"ERROR : low_contrast_block : min percentile pixel not found\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(510 as libc::c_int);
    }
    pi = 64 as libc::c_int - 1 as libc::c_int;
    pixsum = 0 as libc::c_int;
    found = 0 as libc::c_int;
    while pi >= 0 as libc::c_int {
        pixsum += pixtable[pi as usize];
        if pixsum >= prctthresh {
            prctmax = pi;
            found = (0 as libc::c_int == 0) as libc::c_int;
            break;
        } else {
            pi -= 1;
        }
    }
    if found == 0 {
        fprintf(
            stderr,
            b"ERROR : low_contrast_block : max percentile pixel not found\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(511 as libc::c_int);
    }
    delta = prctmax - prctmin;
    if delta < (*lfsparms).min_contrast_delta {
        return (0 as libc::c_int == 0) as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn find_valid_block(
    mut nbr_dir: *mut libc::c_int,
    mut nbr_x: *mut libc::c_int,
    mut nbr_y: *mut libc::c_int,
    mut direction_map: *mut libc::c_int,
    mut low_contrast_map: *mut libc::c_int,
    sx: libc::c_int,
    sy: libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    x_incr: libc::c_int,
    y_incr: libc::c_int,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    x = sx + x_incr;
    y = sy + y_incr;
    while x >= 0 as libc::c_int && x < mw && y >= 0 as libc::c_int && y < mh {
        if *low_contrast_map.offset((y * mw) as isize).offset(x as isize) != 0 {
            return 0 as libc::c_int;
        }
        dir = *direction_map.offset((y * mw) as isize).offset(x as isize);
        if dir >= 0 as libc::c_int {
            *nbr_dir = dir;
            *nbr_x = x;
            *nbr_y = y;
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        x += x_incr;
        y += y_incr;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn set_margin_blocks(
    mut map: *mut libc::c_int,
    mw: libc::c_int,
    mh: libc::c_int,
    margin_value: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut ptr1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ptr2: *mut libc::c_int = 0 as *mut libc::c_int;
    ptr1 = map;
    ptr2 = map.offset(((mh - 1 as libc::c_int) * mw) as isize);
    x = 0 as libc::c_int;
    while x < mw {
        let fresh4 = ptr1;
        ptr1 = ptr1.offset(1);
        *fresh4 = margin_value;
        let fresh5 = ptr2;
        ptr2 = ptr2.offset(1);
        *fresh5 = margin_value;
        x += 1;
    }
    ptr1 = map.offset(mw as isize);
    ptr2 = map
        .offset(mw as isize)
        .offset(mw as isize)
        .offset(-(1 as libc::c_int as isize));
    y = 1 as libc::c_int;
    while y < mh - 1 as libc::c_int {
        *ptr1 = margin_value;
        *ptr2 = margin_value;
        ptr1 = ptr1.offset(mw as isize);
        ptr2 = ptr2.offset(mw as isize);
        y += 1;
    }
}
