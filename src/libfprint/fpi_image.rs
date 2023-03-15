use ::libc;
extern "C" {
    pub type _GData;
    pub type pixman_image;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fp_image_new(width: gint, height: gint) -> *mut FpImage;
    fn pixman_transform_init_identity(matrix: *mut pixman_transform);
    fn pixman_transform_scale(
        forward: *mut pixman_transform,
        reverse: *mut pixman_transform,
        sx: pixman_fixed_t,
        sy: pixman_fixed_t,
    ) -> pixman_bool_t;
    fn pixman_image_create_bits(
        format: pixman_format_code_t,
        width: libc::c_int,
        height: libc::c_int,
        bits: *mut uint32_t,
        rowstride_bytes: libc::c_int,
    ) -> *mut pixman_image_t;
    fn pixman_image_unref(image: *mut pixman_image_t) -> pixman_bool_t;
    fn pixman_image_set_transform(
        image: *mut pixman_image_t,
        transform: *const pixman_transform_t,
    ) -> pixman_bool_t;
    fn pixman_image_set_filter(
        image: *mut pixman_image_t,
        filter: pixman_filter_t,
        filter_params: *const pixman_fixed_t,
        n_filter_params: libc::c_int,
    ) -> pixman_bool_t;
    fn pixman_image_get_data(image: *mut pixman_image_t) -> *mut uint32_t;
    fn pixman_image_composite32(
        op: pixman_op_t,
        src: *mut pixman_image_t,
        mask: *mut pixman_image_t,
        dest: *mut pixman_image_t,
        src_x: int32_t,
        src_y: int32_t,
        mask_x: int32_t,
        mask_y: int32_t,
        dest_x: int32_t,
        dest_y: int32_t,
        width: int32_t,
        height: int32_t,
    );
}
pub type guint8 = libc::c_uchar;
pub type guint64 = libc::c_ulong;
pub type gsize = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type gint = libc::c_int;
pub type guint = libc::c_uint;
pub type gdouble = libc::c_double;
pub type gpointer = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
pub type int32_t = __int32_t;
pub type GData = _GData;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeClass {
    pub g_type: GType,
}
pub type GTypeClass = _GTypeClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpImage {
    pub parent: GObject,
    pub width: guint,
    pub height: guint,
    pub ppmm: gdouble,
    pub flags: FpiImageFlags,
    pub data: *mut guint8,
    pub binarized: *mut guint8,
    pub minutiae: *mut GPtrArray,
    pub ref_count: guint,
}
pub type FpiImageFlags = libc::c_uint;
pub const FPI_IMAGE_PARTIAL: FpiImageFlags = 8;
pub const FPI_IMAGE_COLORS_INVERTED: FpiImageFlags = 4;
pub const FPI_IMAGE_H_FLIPPED: FpiImageFlags = 2;
pub const FPI_IMAGE_V_FLIPPED: FpiImageFlags = 1;
pub type FpImage = _FpImage;
pub type pixman_bool_t = libc::c_int;
pub type pixman_image_t = pixman_image;
pub type uint32_t = __uint32_t;
pub type pixman_op_t = libc::c_uint;
pub const PIXMAN_OP_HSL_LUMINOSITY: pixman_op_t = 62;
pub const PIXMAN_OP_HSL_COLOR: pixman_op_t = 61;
pub const PIXMAN_OP_HSL_SATURATION: pixman_op_t = 60;
pub const PIXMAN_OP_HSL_HUE: pixman_op_t = 59;
pub const PIXMAN_OP_EXCLUSION: pixman_op_t = 58;
pub const PIXMAN_OP_DIFFERENCE: pixman_op_t = 57;
pub const PIXMAN_OP_SOFT_LIGHT: pixman_op_t = 56;
pub const PIXMAN_OP_HARD_LIGHT: pixman_op_t = 55;
pub const PIXMAN_OP_COLOR_BURN: pixman_op_t = 54;
pub const PIXMAN_OP_COLOR_DODGE: pixman_op_t = 53;
pub const PIXMAN_OP_LIGHTEN: pixman_op_t = 52;
pub const PIXMAN_OP_DARKEN: pixman_op_t = 51;
pub const PIXMAN_OP_OVERLAY: pixman_op_t = 50;
pub const PIXMAN_OP_SCREEN: pixman_op_t = 49;
pub const PIXMAN_OP_MULTIPLY: pixman_op_t = 48;
pub const PIXMAN_OP_CONJOINT_XOR: pixman_op_t = 43;
pub const PIXMAN_OP_CONJOINT_ATOP_REVERSE: pixman_op_t = 42;
pub const PIXMAN_OP_CONJOINT_ATOP: pixman_op_t = 41;
pub const PIXMAN_OP_CONJOINT_OUT_REVERSE: pixman_op_t = 40;
pub const PIXMAN_OP_CONJOINT_OUT: pixman_op_t = 39;
pub const PIXMAN_OP_CONJOINT_IN_REVERSE: pixman_op_t = 38;
pub const PIXMAN_OP_CONJOINT_IN: pixman_op_t = 37;
pub const PIXMAN_OP_CONJOINT_OVER_REVERSE: pixman_op_t = 36;
pub const PIXMAN_OP_CONJOINT_OVER: pixman_op_t = 35;
pub const PIXMAN_OP_CONJOINT_DST: pixman_op_t = 34;
pub const PIXMAN_OP_CONJOINT_SRC: pixman_op_t = 33;
pub const PIXMAN_OP_CONJOINT_CLEAR: pixman_op_t = 32;
pub const PIXMAN_OP_DISJOINT_XOR: pixman_op_t = 27;
pub const PIXMAN_OP_DISJOINT_ATOP_REVERSE: pixman_op_t = 26;
pub const PIXMAN_OP_DISJOINT_ATOP: pixman_op_t = 25;
pub const PIXMAN_OP_DISJOINT_OUT_REVERSE: pixman_op_t = 24;
pub const PIXMAN_OP_DISJOINT_OUT: pixman_op_t = 23;
pub const PIXMAN_OP_DISJOINT_IN_REVERSE: pixman_op_t = 22;
pub const PIXMAN_OP_DISJOINT_IN: pixman_op_t = 21;
pub const PIXMAN_OP_DISJOINT_OVER_REVERSE: pixman_op_t = 20;
pub const PIXMAN_OP_DISJOINT_OVER: pixman_op_t = 19;
pub const PIXMAN_OP_DISJOINT_DST: pixman_op_t = 18;
pub const PIXMAN_OP_DISJOINT_SRC: pixman_op_t = 17;
pub const PIXMAN_OP_DISJOINT_CLEAR: pixman_op_t = 16;
pub const PIXMAN_OP_SATURATE: pixman_op_t = 13;
pub const PIXMAN_OP_ADD: pixman_op_t = 12;
pub const PIXMAN_OP_XOR: pixman_op_t = 11;
pub const PIXMAN_OP_ATOP_REVERSE: pixman_op_t = 10;
pub const PIXMAN_OP_ATOP: pixman_op_t = 9;
pub const PIXMAN_OP_OUT_REVERSE: pixman_op_t = 8;
pub const PIXMAN_OP_OUT: pixman_op_t = 7;
pub const PIXMAN_OP_IN_REVERSE: pixman_op_t = 6;
pub const PIXMAN_OP_IN: pixman_op_t = 5;
pub const PIXMAN_OP_OVER_REVERSE: pixman_op_t = 4;
pub const PIXMAN_OP_OVER: pixman_op_t = 3;
pub const PIXMAN_OP_DST: pixman_op_t = 2;
pub const PIXMAN_OP_SRC: pixman_op_t = 1;
pub const PIXMAN_OP_CLEAR: pixman_op_t = 0;
pub type pixman_fixed_t = pixman_fixed_16_16_t;
pub type pixman_fixed_16_16_t = int32_t;
pub type pixman_filter_t = libc::c_uint;
pub const PIXMAN_FILTER_SEPARABLE_CONVOLUTION: pixman_filter_t = 6;
pub const PIXMAN_FILTER_CONVOLUTION: pixman_filter_t = 5;
pub const PIXMAN_FILTER_BILINEAR: pixman_filter_t = 4;
pub const PIXMAN_FILTER_NEAREST: pixman_filter_t = 3;
pub const PIXMAN_FILTER_BEST: pixman_filter_t = 2;
pub const PIXMAN_FILTER_GOOD: pixman_filter_t = 1;
pub const PIXMAN_FILTER_FAST: pixman_filter_t = 0;
pub type pixman_transform_t = pixman_transform;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pixman_transform {
    pub matrix: [[pixman_fixed_t; 3]; 3],
}
pub type pixman_format_code_t = libc::c_uint;
pub const PIXMAN_yv12: pixman_format_code_t = 201785344;
pub const PIXMAN_yuy2: pixman_format_code_t = 268828672;
pub const PIXMAN_g1: pixman_format_code_t = 17104896;
pub const PIXMAN_a1: pixman_format_code_t = 16846848;
pub const PIXMAN_g4: pixman_format_code_t = 67436544;
pub const PIXMAN_c4: pixman_format_code_t = 67371008;
pub const PIXMAN_a1b1g1r1: pixman_format_code_t = 67309841;
pub const PIXMAN_a1r1g1b1: pixman_format_code_t = 67244305;
pub const PIXMAN_b1g2r1: pixman_format_code_t = 67305761;
pub const PIXMAN_r1g2b1: pixman_format_code_t = 67240225;
pub const PIXMAN_a4: pixman_format_code_t = 67190784;
pub const PIXMAN_x4g4: pixman_format_code_t = 134545408;
pub const PIXMAN_x4c4: pixman_format_code_t = 134479872;
pub const PIXMAN_x4a4: pixman_format_code_t = 134299648;
pub const PIXMAN_g8: pixman_format_code_t = 134545408;
pub const PIXMAN_c8: pixman_format_code_t = 134479872;
pub const PIXMAN_a2b2g2r2: pixman_format_code_t = 134423074;
pub const PIXMAN_a2r2g2b2: pixman_format_code_t = 134357538;
pub const PIXMAN_b2g3r3: pixman_format_code_t = 134415154;
pub const PIXMAN_r3g3b2: pixman_format_code_t = 134349618;
pub const PIXMAN_a8: pixman_format_code_t = 134316032;
pub const PIXMAN_x4b4g4r4: pixman_format_code_t = 268633156;
pub const PIXMAN_a4b4g4r4: pixman_format_code_t = 268649540;
pub const PIXMAN_x4r4g4b4: pixman_format_code_t = 268567620;
pub const PIXMAN_a4r4g4b4: pixman_format_code_t = 268584004;
pub const PIXMAN_x1b5g5r5: pixman_format_code_t = 268633429;
pub const PIXMAN_a1b5g5r5: pixman_format_code_t = 268637525;
pub const PIXMAN_x1r5g5b5: pixman_format_code_t = 268567893;
pub const PIXMAN_a1r5g5b5: pixman_format_code_t = 268571989;
pub const PIXMAN_b5g6r5: pixman_format_code_t = 268633445;
pub const PIXMAN_r5g6b5: pixman_format_code_t = 268567909;
pub const PIXMAN_b8g8r8: pixman_format_code_t = 402851976;
pub const PIXMAN_r8g8b8: pixman_format_code_t = 402786440;
pub const PIXMAN_a8r8g8b8_sRGB: pixman_format_code_t = 537561224;
pub const PIXMAN_a2b10g10r10: pixman_format_code_t = 537078442;
pub const PIXMAN_x2b10g10r10: pixman_format_code_t = 537070250;
pub const PIXMAN_a2r10g10b10: pixman_format_code_t = 537012906;
pub const PIXMAN_x2r10g10b10: pixman_format_code_t = 537004714;
pub const PIXMAN_x14r6g6b6: pixman_format_code_t = 537003622;
pub const PIXMAN_r8g8b8x8: pixman_format_code_t = 537462920;
pub const PIXMAN_r8g8b8a8: pixman_format_code_t = 537495688;
pub const PIXMAN_b8g8r8x8: pixman_format_code_t = 537397384;
pub const PIXMAN_b8g8r8a8: pixman_format_code_t = 537430152;
pub const PIXMAN_x8b8g8r8: pixman_format_code_t = 537069704;
pub const PIXMAN_a8b8g8r8: pixman_format_code_t = 537102472;
pub const PIXMAN_x8r8g8b8: pixman_format_code_t = 537004168;
pub const PIXMAN_a8r8g8b8: pixman_format_code_t = 537036936;
pub const PIXMAN_rgb_float: pixman_format_code_t = 214631492;
pub const PIXMAN_rgba_float: pixman_format_code_t = 281756740;
#[no_mangle]
pub unsafe extern "C" fn fpi_std_sq_dev(mut buf: *const guint8, mut size: gint) -> gint {
    let mut res: guint64 = 0 as libc::c_int as guint64;
    let mut mean: guint64 = 0 as libc::c_int as guint64;
    let mut i: gint = 0;
    i = 0 as libc::c_int;
    while i < size {
        mean = (mean as libc::c_ulong)
            .wrapping_add(*buf.offset(i as isize) as libc::c_ulong) as guint64
            as guint64;
        i += 1;
    }
    mean = (mean as libc::c_ulong).wrapping_div(size as libc::c_ulong) as guint64
        as guint64;
    i = 0 as libc::c_int;
    while i < size {
        let mut dev: libc::c_int = (*buf.offset(i as isize) as libc::c_int
            as libc::c_ulong)
            .wrapping_sub(mean) as libc::c_int;
        res = (res as libc::c_ulong).wrapping_add((dev * dev) as libc::c_ulong)
            as guint64 as guint64;
        i += 1;
    }
    return res.wrapping_div(size as libc::c_ulong) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_mean_sq_diff_norm(
    mut buf1: *const guint8,
    mut buf2: *const guint8,
    mut size: gint,
) -> gint {
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        let mut dev: libc::c_int = *buf1.offset(i as isize) as libc::c_int
            - *buf2.offset(i as isize) as libc::c_int;
        res += dev * dev;
        i += 1;
    }
    return res / size;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_image_resize(
    mut orig_img: *mut FpImage,
    mut w_factor: guint,
    mut h_factor: guint,
) -> *mut FpImage {
    let mut new_width: libc::c_int = ((*orig_img).width).wrapping_mul(w_factor)
        as libc::c_int;
    let mut new_height: libc::c_int = ((*orig_img).height).wrapping_mul(h_factor)
        as libc::c_int;
    let mut orig: *mut pixman_image_t = 0 as *mut pixman_image_t;
    let mut resized: *mut pixman_image_t = 0 as *mut pixman_image_t;
    let mut transform: pixman_transform_t = pixman_transform_t {
        matrix: [[0; 3]; 3],
    };
    let mut newimg: *mut FpImage = 0 as *mut FpImage;
    orig = pixman_image_create_bits(
        PIXMAN_a8,
        (*orig_img).width as libc::c_int,
        (*orig_img).height as libc::c_int,
        (*orig_img).data as *mut uint32_t,
        (*orig_img).width as libc::c_int,
    );
    resized = pixman_image_create_bits(
        PIXMAN_a8,
        new_width,
        new_height,
        0 as *mut uint32_t,
        new_width,
    );
    pixman_transform_init_identity(&mut transform);
    pixman_transform_scale(
        0 as *mut pixman_transform,
        &mut transform,
        (w_factor << 16 as libc::c_int) as pixman_fixed_t,
        (h_factor << 16 as libc::c_int) as pixman_fixed_t,
    );
    pixman_image_set_transform(orig, &mut transform);
    pixman_image_set_filter(
        orig,
        PIXMAN_FILTER_BILINEAR,
        0 as *const pixman_fixed_t,
        0 as libc::c_int,
    );
    pixman_image_composite32(
        PIXMAN_OP_SRC,
        orig,
        0 as *mut pixman_image_t,
        resized,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        new_width,
        new_height,
    );
    newimg = fp_image_new(new_width, new_height);
    (*newimg).flags = (*orig_img).flags;
    memcpy(
        (*newimg).data as *mut libc::c_void,
        pixman_image_get_data(resized) as *const libc::c_void,
        (new_width * new_height) as libc::c_ulong,
    );
    pixman_image_unref(orig);
    pixman_image_unref(resized);
    return newimg;
}
