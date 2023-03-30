// This is the number of integer directions to be used in semicircle.
// CAUTION: If NUM_DIRECTIONS is changed, then the following will
// likely need to be changed:
// * HIGHCURV_VORTICITY_MIN,
// * HIGHCURV_CURVATURE_MIN,
// * FORK_INTERVAL
const NUM_DIRECTIONS: f32 = 16.0;

#[derive(Copy, Clone)]
pub struct FpMinutia {
    pub x: i32,
    pub y: i32,
    pub ex: i32,
    pub ey: i32,
    pub direction: i32,
    pub reliability: f32,
    pub type_0: i32,
    pub appearing: i32,
    pub feature_id: i32,
    pub nbrs: *mut i32,
    pub ridge_counts: *mut i32,
    pub num_nbrs: i32,
}
// REMOVE_ME:
//pub type MINUTIA = fp_minutia;

/// Converts XYT minutiae attributes in LFS native representation to NIST internal representation
///
/// # Arguments
///
/// * `minutia` - LFS minutia structure containing attributes to be converted
/// * `width`
/// * `height`
///
/// # Result
///
/// * ox - NIST internal based x-pixel coordinate
/// * oy - NIST internal based y-pixel coordinate
/// * ot - NIST internal based minutia direction/orientation
fn lfs2nist_minutia_XYT(mut minutia: &FpMinutia, _width: i32, height: i32) -> (i32, i32, i32) {
    // XYT's according to NIST internal rep:
    // 1. pixel coordinates with origin bottom-left
    // 2. orientation in degrees on range [0..360]
    //    with 0 pointing east and increasing counter
    //    clockwise (same as M1)
    // 3. direction pointing out and away from the
    //    ridge ending or bifurcation valley
    //    (opposite direction from M1)
    let x = minutia.x;
    let y = height - minutia.y;
    let degrees_per_unit: f32 = 180.0 / NUM_DIRECTIONS;

    let t = (270 - sround(minutia.direction * degrees_per_unit)) % 360;
    if t < 0 {
        t += 360;
    }

    (x, y, t)
}

fn sround(v: f32) -> i32 {
    if v < 0 {
        v - 0.5
    } else {
        v + 0.5
    }
}
