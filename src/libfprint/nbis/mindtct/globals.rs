use ::libc;
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
pub static mut g_dft_coefs: [libc::c_double; 4] = [
    1 as libc::c_int as libc::c_double,
    2 as libc::c_int as libc::c_double,
    3 as libc::c_int as libc::c_double,
    4 as libc::c_int as libc::c_double,
];
#[no_mangle]
pub static mut g_lfsparms: LFSPARMS = {
    let mut init = g_lfsparms {
        pad_value: 128 as libc::c_int,
        join_line_radius: 1 as libc::c_int,
        blocksize: 24 as libc::c_int,
        windowsize: 0 as libc::c_int,
        windowoffset: 0 as libc::c_int,
        num_directions: 16 as libc::c_int,
        start_dir_angle: 3.14159265358979323846f64 / 2.0f64,
        rmv_valid_nbr_min: 3 as libc::c_int,
        dir_strength_min: 0.2f64,
        dir_distance_max: 3 as libc::c_int,
        smth_valid_nbr_min: 7 as libc::c_int,
        vort_valid_nbr_min: 7 as libc::c_int,
        highcurv_vorticity_min: 5 as libc::c_int,
        highcurv_curvature_min: 5 as libc::c_int,
        min_interpolate_nbrs: 0 as libc::c_int,
        percentile_min_max: 0 as libc::c_int,
        min_contrast_delta: 0 as libc::c_int,
        num_dft_waves: 4 as libc::c_int,
        powmax_min: 100000.0f64,
        pownorm_min: 3.8f64,
        powmax_max: 50000000.0f64,
        fork_interval: 2 as libc::c_int,
        fork_pct_powmax: 0.7f64,
        fork_pct_pownorm: 0.75f64,
        dirbin_grid_w: 7 as libc::c_int,
        dirbin_grid_h: 9 as libc::c_int,
        isobin_grid_dim: 11 as libc::c_int,
        num_fill_holes: 3 as libc::c_int,
        max_minutia_delta: 10 as libc::c_int,
        max_high_curve_theta: 3.14159265358979323846f64 / 3.0f64,
        high_curve_half_contour: 14 as libc::c_int,
        min_loop_len: 20 as libc::c_int,
        min_loop_aspect_dist: 1.0f64,
        min_loop_aspect_ratio: 2.25f64,
        link_table_dim: 20 as libc::c_int,
        max_link_dist: 20 as libc::c_int,
        min_theta_dist: 5 as libc::c_int,
        maxtrans: 2 as libc::c_int,
        score_theta_norm: 15.0f64,
        score_dist_norm: 10.0f64,
        score_dist_weight: 4.0f64,
        score_numerator: 32000.0f64,
        max_rmtest_dist: 8 as libc::c_int,
        max_hook_len: 15 as libc::c_int,
        max_half_loop: 15 as libc::c_int,
        trans_dir_pix: 6 as libc::c_int,
        small_loop_len: 15 as libc::c_int,
        side_half_contour: 7 as libc::c_int,
        inv_block_margin: 6 as libc::c_int,
        rm_valid_nbr_min: 7 as libc::c_int,
        max_overlap_dist: 0 as libc::c_int,
        max_overlap_join_dist: 0 as libc::c_int,
        malformation_steps_1: 0 as libc::c_int,
        malformation_steps_2: 0 as libc::c_int,
        min_malformation_ratio: 0.0f64,
        max_malformation_dist: 0 as libc::c_int,
        pores_trans_r: 3 as libc::c_int,
        pores_perp_steps: 12 as libc::c_int,
        pores_steps_fwd: 10 as libc::c_int,
        pores_steps_bwd: 8 as libc::c_int,
        pores_min_dist2: 0.5f64,
        pores_max_ratio: 2.25f64,
        remove_perimeter_pts: 0 as libc::c_int,
        min_pp_distance: 10 as libc::c_int,
        max_nbrs: 5 as libc::c_int,
        max_ridge_steps: 10 as libc::c_int,
    };
    init
};
#[no_mangle]
pub static mut g_lfsparms_V2: LFSPARMS = {
    let mut init = g_lfsparms {
        pad_value: 128 as libc::c_int,
        join_line_radius: 1 as libc::c_int,
        blocksize: 8 as libc::c_int,
        windowsize: 24 as libc::c_int,
        windowoffset: 8 as libc::c_int,
        num_directions: 16 as libc::c_int,
        start_dir_angle: 3.14159265358979323846f64 / 2.0f64,
        rmv_valid_nbr_min: 3 as libc::c_int,
        dir_strength_min: 0.2f64,
        dir_distance_max: 3 as libc::c_int,
        smth_valid_nbr_min: 7 as libc::c_int,
        vort_valid_nbr_min: 7 as libc::c_int,
        highcurv_vorticity_min: 5 as libc::c_int,
        highcurv_curvature_min: 5 as libc::c_int,
        min_interpolate_nbrs: 2 as libc::c_int,
        percentile_min_max: 10 as libc::c_int,
        min_contrast_delta: 5 as libc::c_int,
        num_dft_waves: 4 as libc::c_int,
        powmax_min: 100000.0f64,
        pownorm_min: 3.8f64,
        powmax_max: 50000000.0f64,
        fork_interval: 2 as libc::c_int,
        fork_pct_powmax: 0.7f64,
        fork_pct_pownorm: 0.75f64,
        dirbin_grid_w: 7 as libc::c_int,
        dirbin_grid_h: 9 as libc::c_int,
        isobin_grid_dim: 0 as libc::c_int,
        num_fill_holes: 3 as libc::c_int,
        max_minutia_delta: 10 as libc::c_int,
        max_high_curve_theta: 3.14159265358979323846f64 / 3.0f64,
        high_curve_half_contour: 14 as libc::c_int,
        min_loop_len: 20 as libc::c_int,
        min_loop_aspect_dist: 1.0f64,
        min_loop_aspect_ratio: 2.25f64,
        link_table_dim: 0 as libc::c_int,
        max_link_dist: 0 as libc::c_int,
        min_theta_dist: 0 as libc::c_int,
        maxtrans: 2 as libc::c_int,
        score_theta_norm: 0.0f64,
        score_dist_norm: 0.0f64,
        score_dist_weight: 0.0f64,
        score_numerator: 0.0f64,
        max_rmtest_dist: 16 as libc::c_int,
        max_hook_len: 30 as libc::c_int,
        max_half_loop: 30 as libc::c_int,
        trans_dir_pix: 4 as libc::c_int,
        small_loop_len: 15 as libc::c_int,
        side_half_contour: 7 as libc::c_int,
        inv_block_margin: 4 as libc::c_int,
        rm_valid_nbr_min: 7 as libc::c_int,
        max_overlap_dist: 8 as libc::c_int,
        max_overlap_join_dist: 6 as libc::c_int,
        malformation_steps_1: 10 as libc::c_int,
        malformation_steps_2: 20 as libc::c_int,
        min_malformation_ratio: 2.0f64,
        max_malformation_dist: 20 as libc::c_int,
        pores_trans_r: 3 as libc::c_int,
        pores_perp_steps: 12 as libc::c_int,
        pores_steps_fwd: 10 as libc::c_int,
        pores_steps_bwd: 8 as libc::c_int,
        pores_min_dist2: 0.5f64,
        pores_max_ratio: 2.25f64,
        remove_perimeter_pts: 0 as libc::c_int,
        min_pp_distance: 10 as libc::c_int,
        max_nbrs: 5 as libc::c_int,
        max_ridge_steps: 10 as libc::c_int,
    };
    init
};
#[no_mangle]
pub static mut g_nbr8_dx: [libc::c_int; 8] = [
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    0 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
];
#[no_mangle]
pub static mut g_nbr8_dy: [libc::c_int; 8] = [
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    0 as libc::c_int,
    -(1 as libc::c_int),
];
#[no_mangle]
pub static mut g_chaincodes_nbr8: [libc::c_int; 9] = [
    3 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
    4 as libc::c_int,
    -(1 as libc::c_int),
    0 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
];
#[no_mangle]
pub static mut g_feature_patterns: [FEATURE_PATTERN; 10] = [
    {
        let mut init = feature_pattern {
            type_0: 1 as libc::c_int,
            appearing: 1 as libc::c_int,
            first: [0 as libc::c_int, 0 as libc::c_int],
            second: [0 as libc::c_int, 1 as libc::c_int],
            third: [0 as libc::c_int, 0 as libc::c_int],
        };
        init
    },
    {
        let mut init = feature_pattern {
            type_0: 1 as libc::c_int,
            appearing: 0 as libc::c_int,
            first: [0 as libc::c_int, 0 as libc::c_int],
            second: [1 as libc::c_int, 0 as libc::c_int],
            third: [0 as libc::c_int, 0 as libc::c_int],
        };
        init
    },
    {
        let mut init = feature_pattern {
            type_0: 0 as libc::c_int,
            appearing: 0 as libc::c_int,
            first: [1 as libc::c_int, 1 as libc::c_int],
            second: [0 as libc::c_int, 1 as libc::c_int],
            third: [1 as libc::c_int, 1 as libc::c_int],
        };
        init
    },
    {
        let mut init = feature_pattern {
            type_0: 0 as libc::c_int,
            appearing: 1 as libc::c_int,
            first: [1 as libc::c_int, 1 as libc::c_int],
            second: [1 as libc::c_int, 0 as libc::c_int],
            third: [1 as libc::c_int, 1 as libc::c_int],
        };
        init
    },
    {
        let mut init = feature_pattern {
            type_0: 0 as libc::c_int,
            appearing: 0 as libc::c_int,
            first: [1 as libc::c_int, 0 as libc::c_int],
            second: [0 as libc::c_int, 1 as libc::c_int],
            third: [1 as libc::c_int, 1 as libc::c_int],
        };
        init
    },
    {
        let mut init = feature_pattern {
            type_0: 0 as libc::c_int,
            appearing: 0 as libc::c_int,
            first: [1 as libc::c_int, 1 as libc::c_int],
            second: [0 as libc::c_int, 1 as libc::c_int],
            third: [1 as libc::c_int, 0 as libc::c_int],
        };
        init
    },
    {
        let mut init = feature_pattern {
            type_0: 0 as libc::c_int,
            appearing: 1 as libc::c_int,
            first: [1 as libc::c_int, 1 as libc::c_int],
            second: [1 as libc::c_int, 0 as libc::c_int],
            third: [0 as libc::c_int, 1 as libc::c_int],
        };
        init
    },
    {
        let mut init = feature_pattern {
            type_0: 0 as libc::c_int,
            appearing: 1 as libc::c_int,
            first: [0 as libc::c_int, 1 as libc::c_int],
            second: [1 as libc::c_int, 0 as libc::c_int],
            third: [1 as libc::c_int, 1 as libc::c_int],
        };
        init
    },
    {
        let mut init = feature_pattern {
            type_0: 0 as libc::c_int,
            appearing: 0 as libc::c_int,
            first: [1 as libc::c_int, 0 as libc::c_int],
            second: [0 as libc::c_int, 1 as libc::c_int],
            third: [1 as libc::c_int, 0 as libc::c_int],
        };
        init
    },
    {
        let mut init = feature_pattern {
            type_0: 0 as libc::c_int,
            appearing: 1 as libc::c_int,
            first: [0 as libc::c_int, 1 as libc::c_int],
            second: [1 as libc::c_int, 0 as libc::c_int],
            third: [0 as libc::c_int, 1 as libc::c_int],
        };
        init
    },
];
