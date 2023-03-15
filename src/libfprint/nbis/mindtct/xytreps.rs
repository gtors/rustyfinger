use ::libc;
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
pub type MINUTIA = fp_minutia;
#[no_mangle]
pub unsafe extern "C" fn lfs2nist_minutia_XYT(
    mut ox: *mut libc::c_int,
    mut oy: *mut libc::c_int,
    mut ot: *mut libc::c_int,
    mut minutia: *const MINUTIA,
    iw: libc::c_int,
    ih: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut degrees_per_unit: libc::c_float = 0.;
    x = (*minutia).x;
    y = ih - (*minutia).y;
    degrees_per_unit = 180 as libc::c_int as libc::c_float
        / 16 as libc::c_int as libc::c_float;
    t = (270 as libc::c_int
        - (if (*minutia).direction as libc::c_float * degrees_per_unit
            < 0 as libc::c_int as libc::c_float
        {
            ((*minutia).direction as libc::c_float * degrees_per_unit) as libc::c_double
                - 0.5f64
        } else {
            ((*minutia).direction as libc::c_float * degrees_per_unit) as libc::c_double
                + 0.5f64
        }) as libc::c_int) % 360 as libc::c_int;
    if t < 0 as libc::c_int {
        t += 360 as libc::c_int;
    }
    *ox = x;
    *oy = y;
    *ot = t;
}
