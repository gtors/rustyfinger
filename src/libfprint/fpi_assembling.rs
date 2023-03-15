use ::libc;
extern "C" {
    pub type _GData;
    pub type _GTimer;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_get_real_time() -> gint64;
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn g_return_if_fail_warning(
        log_domain: *const libc::c_char,
        pretty_function: *const libc::c_char,
        expression: *const libc::c_char,
    );
    fn g_qsort_with_data(
        pbase: gconstpointer,
        total_elems: gint,
        size: gsize,
        compare_func: GCompareDataFunc,
        user_data: gpointer,
    );
    fn g_timer_new() -> *mut GTimer;
    fn g_timer_destroy(timer: *mut GTimer);
    fn g_timer_stop(timer: *mut GTimer);
    fn g_timer_elapsed(timer: *mut GTimer, microseconds: *mut gulong) -> gdouble;
    fn fp_image_new(width: gint, height: gint) -> *mut FpImage;
}
pub type size_t = libc::c_ulong;
pub type guint8 = libc::c_uchar;
pub type gint32 = libc::c_int;
pub type gint64 = libc::c_long;
pub type gsize = libc::c_ulong;
pub type gchar = libc::c_char;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type gulong = libc::c_ulong;
pub type guint = libc::c_uint;
pub type gdouble = libc::c_double;
pub type gpointer = *mut libc::c_void;
pub type gconstpointer = *const libc::c_void;
pub type GCompareDataFunc = Option::<
    unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GLogLevelFlags = libc::c_int;
pub const G_LOG_LEVEL_MASK: GLogLevelFlags = -4;
pub const G_LOG_LEVEL_DEBUG: GLogLevelFlags = 128;
pub const G_LOG_LEVEL_INFO: GLogLevelFlags = 64;
pub const G_LOG_LEVEL_MESSAGE: GLogLevelFlags = 32;
pub const G_LOG_LEVEL_WARNING: GLogLevelFlags = 16;
pub const G_LOG_LEVEL_CRITICAL: GLogLevelFlags = 8;
pub const G_LOG_LEVEL_ERROR: GLogLevelFlags = 4;
pub const G_LOG_FLAG_FATAL: GLogLevelFlags = 2;
pub const G_LOG_FLAG_RECURSION: GLogLevelFlags = 1;
pub type GTimer = _GTimer;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fpi_frame {
    pub delta_x: libc::c_int,
    pub delta_y: libc::c_int,
    pub data: [libc::c_uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fpi_frame_asmbl_ctx {
    pub frame_width: libc::c_uint,
    pub frame_height: libc::c_uint,
    pub image_width: libc::c_uint,
    pub get_pixel: Option::<
        unsafe extern "C" fn(
            *mut fpi_frame_asmbl_ctx,
            *mut fpi_frame,
            libc::c_uint,
            libc::c_uint,
        ) -> libc::c_uchar,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fpi_line_asmbl_ctx {
    pub line_width: libc::c_uint,
    pub max_height: libc::c_uint,
    pub resolution: libc::c_uint,
    pub median_filter_size: libc::c_uint,
    pub max_search_offset: libc::c_uint,
    pub get_deviation: Option::<
        unsafe extern "C" fn(
            *mut fpi_line_asmbl_ctx,
            *mut GSList,
            *mut GSList,
        ) -> libc::c_int,
    >,
    pub get_pixel: Option::<
        unsafe extern "C" fn(
            *mut fpi_line_asmbl_ctx,
            *mut GSList,
            libc::c_uint,
        ) -> libc::c_uchar,
    >,
}
unsafe extern "C" fn calc_error(
    mut ctx: *mut fpi_frame_asmbl_ctx,
    mut first_frame: *mut fpi_frame,
    mut second_frame: *mut fpi_frame,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
) -> libc::c_uint {
    let mut width: libc::c_uint = 0;
    let mut height: libc::c_uint = 0;
    let mut x1: libc::c_uint = 0;
    let mut y1: libc::c_uint = 0;
    let mut x2: libc::c_uint = 0;
    let mut y2: libc::c_uint = 0;
    let mut err: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    width = ((*ctx).frame_width)
        .wrapping_sub((if dx > 0 as libc::c_int { dx } else { -dx }) as libc::c_uint);
    height = ((*ctx).frame_height).wrapping_sub(dy as libc::c_uint);
    if height == 0 as libc::c_int as libc::c_uint
        || width == 0 as libc::c_int as libc::c_uint
    {
        return 2147483647 as libc::c_int as libc::c_uint;
    }
    y1 = 0 as libc::c_int as libc::c_uint;
    y2 = dy as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    err = 0 as libc::c_int as libc::c_uint;
    loop {
        x1 = (if dx < 0 as libc::c_int { 0 as libc::c_int } else { dx }) as libc::c_uint;
        x2 = (if dx < 0 as libc::c_int { -dx } else { 0 as libc::c_int })
            as libc::c_uint;
        j = 0 as libc::c_int as libc::c_uint;
        loop {
            let mut v1: libc::c_uchar = 0;
            let mut v2: libc::c_uchar = 0;
            v1 = ((*ctx).get_pixel)
                .expect("non-null function pointer")(ctx, first_frame, x1, y1);
            v2 = ((*ctx).get_pixel)
                .expect("non-null function pointer")(ctx, second_frame, x2, y2);
            err = err
                .wrapping_add(
                    (if v1 as libc::c_int > v2 as libc::c_int {
                        v1 as libc::c_int - v2 as libc::c_int
                    } else {
                        v2 as libc::c_int - v1 as libc::c_int
                    }) as libc::c_uint,
                );
            j = j.wrapping_add(1);
            x1 = x1.wrapping_add(1);
            x2 = x2.wrapping_add(1);
            if !(j < width) {
                break;
            }
        }
        i = i.wrapping_add(1);
        y1 = y1.wrapping_add(1);
        y2 = y2.wrapping_add(1);
        if !(i < height) {
            break;
        }
    }
    err = err.wrapping_mul(((*ctx).frame_height).wrapping_mul((*ctx).frame_width));
    err = err.wrapping_div(height.wrapping_mul(width));
    return err;
}
unsafe extern "C" fn find_overlap(
    mut ctx: *mut fpi_frame_asmbl_ctx,
    mut first_frame: *mut fpi_frame,
    mut second_frame: *mut fpi_frame,
    mut dx_out: *mut libc::c_int,
    mut dy_out: *mut libc::c_int,
    mut min_error: *mut libc::c_uint,
) {
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut err: libc::c_uint = 0;
    *min_error = (255 as libc::c_int as libc::c_uint)
        .wrapping_mul((*ctx).frame_height)
        .wrapping_mul((*ctx).frame_width);
    dy = 2 as libc::c_int;
    while (dy as libc::c_uint) < (*ctx).frame_height {
        dx = -(8 as libc::c_int);
        while dx < 8 as libc::c_int {
            err = calc_error(ctx, first_frame, second_frame, dx, dy);
            if err < *min_error {
                *min_error = err;
                *dx_out = -dx;
                *dy_out = dy;
            }
            dx += 1;
        }
        dy += 1;
    }
}
unsafe extern "C" fn do_movement_estimation(
    mut ctx: *mut fpi_frame_asmbl_ctx,
    mut stripes: *mut GSList,
    mut reverse: gboolean,
) -> libc::c_uint {
    let mut l: *mut GSList = 0 as *mut GSList;
    let mut timer: *mut GTimer = 0 as *mut GTimer;
    let mut num_frames: guint = 1 as libc::c_int as guint;
    let mut prev_stripe: *mut fpi_frame = 0 as *mut fpi_frame;
    let mut min_error: libc::c_uint = 0;
    let mut total_error: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    timer = g_timer_new();
    prev_stripe = (*stripes).data as *mut fpi_frame;
    l = (*stripes).next;
    while !l.is_null() {
        let mut cur_stripe: *mut fpi_frame = (*l).data as *mut fpi_frame;
        if reverse != 0 {
            find_overlap(
                ctx,
                prev_stripe,
                cur_stripe,
                &mut (*cur_stripe).delta_x,
                &mut (*cur_stripe).delta_y,
                &mut min_error,
            );
            (*cur_stripe).delta_y = -(*cur_stripe).delta_y;
            (*cur_stripe).delta_x = -(*cur_stripe).delta_x;
        } else {
            find_overlap(
                ctx,
                cur_stripe,
                prev_stripe,
                &mut (*cur_stripe).delta_x,
                &mut (*cur_stripe).delta_y,
                &mut min_error,
            );
        }
        total_error = total_error.wrapping_add(min_error as libc::c_ulonglong);
        prev_stripe = cur_stripe;
        l = (*l).next;
        num_frames = num_frames.wrapping_add(1);
    }
    g_timer_stop(timer);
    g_log(
        b"libfprint-assembling\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"calc delta completed in %f secs\0" as *const u8 as *const libc::c_char,
        g_timer_elapsed(timer, 0 as *mut gulong),
    );
    g_timer_destroy(timer);
    return total_error.wrapping_div(num_frames as libc::c_ulonglong) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_do_movement_estimation(
    mut ctx: *mut fpi_frame_asmbl_ctx,
    mut stripes: *mut GSList,
) {
    let mut err: libc::c_int = 0;
    let mut rev_err: libc::c_int = 0;
    err = do_movement_estimation(ctx, stripes, 0 as libc::c_int) as libc::c_int;
    rev_err = do_movement_estimation(
        ctx,
        stripes,
        (0 as libc::c_int == 0) as libc::c_int,
    ) as libc::c_int;
    g_log(
        b"libfprint-assembling\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"errors: %d rev: %d\0" as *const u8 as *const libc::c_char,
        err,
        rev_err,
    );
    if err < rev_err {
        do_movement_estimation(ctx, stripes, 0 as libc::c_int);
    }
}
#[inline]
unsafe extern "C" fn aes_blit_stripe(
    mut ctx: *mut fpi_frame_asmbl_ctx,
    mut img: *mut FpImage,
    mut stripe: *mut fpi_frame,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    let mut ix1: libc::c_uint = 0;
    let mut iy1: libc::c_uint = 0;
    let mut fx1: libc::c_uint = 0;
    let mut fy1: libc::c_uint = 0;
    let mut fx: libc::c_uint = 0;
    let mut fy: libc::c_uint = 0;
    let mut ix: libc::c_uint = 0;
    let mut iy: libc::c_uint = 0;
    if x < 0 as libc::c_int {
        ix1 = 0 as libc::c_int as libc::c_uint;
        fx1 = -x as libc::c_uint;
    } else {
        ix1 = x as libc::c_uint;
        fx1 = 0 as libc::c_int as libc::c_uint;
    }
    if y < 0 as libc::c_int {
        iy1 = 0 as libc::c_int as libc::c_uint;
        fy1 = -y as libc::c_uint;
    } else {
        iy1 = y as libc::c_uint;
        fy1 = 0 as libc::c_int as libc::c_uint;
    }
    fy = fy1;
    iy = iy1;
    while fy < (*ctx).frame_height && iy < (*img).height {
        fx = fx1;
        ix = ix1;
        while fx < (*ctx).frame_width && ix < (*img).width {
            *((*img).data)
                .offset(
                    ix.wrapping_add(iy.wrapping_mul((*img).width)) as isize,
                ) = ((*ctx).get_pixel)
                .expect("non-null function pointer")(ctx, stripe, fx, fy);
            fx = fx.wrapping_add(1);
            ix = ix.wrapping_add(1);
        }
        fy = fy.wrapping_add(1);
        iy = iy.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fpi_assemble_frames(
    mut ctx: *mut fpi_frame_asmbl_ctx,
    mut stripes: *mut GSList,
) -> *mut FpImage {
    let mut l: *mut GSList = 0 as *mut GSList;
    let mut img: *mut FpImage = 0 as *mut FpImage;
    let mut height: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut reverse: gboolean = 0 as libc::c_int;
    let mut fpi_frame: *mut fpi_frame = 0 as *mut fpi_frame;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !stripes.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-assembling\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fpi_assemble_frames\0"))
                .as_ptr(),
            b"stripes != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FpImage;
    }
    fpi_frame = (*stripes).data as *mut fpi_frame;
    (*fpi_frame).delta_x = 0 as libc::c_int;
    (*fpi_frame).delta_y = 0 as libc::c_int;
    l = stripes;
    while !l.is_null() {
        fpi_frame = (*l).data as *mut fpi_frame;
        height += (*fpi_frame).delta_y;
        l = (*l).next;
    }
    g_log(
        b"libfprint-assembling\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"height is %d\0" as *const u8 as *const libc::c_char,
        height,
    );
    if height < 0 as libc::c_int {
        reverse = (0 as libc::c_int == 0) as libc::c_int;
        height = -height;
    }
    height = (height as libc::c_uint).wrapping_add((*ctx).frame_height) as libc::c_int
        as libc::c_int;
    img = fp_image_new((*ctx).image_width as gint, height);
    (*img).flags = FPI_IMAGE_COLORS_INVERTED;
    (*img)
        .flags = ::core::mem::transmute::<
        libc::c_uint,
        FpiImageFlags,
    >(
        (*img).flags as libc::c_uint
            | (if reverse != 0 {
                0 as libc::c_int
            } else {
                FPI_IMAGE_H_FLIPPED as libc::c_int | FPI_IMAGE_V_FLIPPED as libc::c_int
            }) as libc::c_uint,
    );
    (*img).width = (*ctx).image_width;
    (*img).height = height as guint;
    y = (if reverse != 0 {
        (height as libc::c_uint).wrapping_sub((*ctx).frame_height)
    } else {
        0 as libc::c_int as libc::c_uint
    }) as libc::c_int;
    x = ((*ctx).image_width as libc::c_int - (*ctx).frame_width as libc::c_int)
        / 2 as libc::c_int;
    l = stripes;
    while !l.is_null() {
        fpi_frame = (*l).data as *mut fpi_frame;
        y += (*fpi_frame).delta_y;
        x += (*fpi_frame).delta_x;
        aes_blit_stripe(ctx, img, fpi_frame, x, y);
        l = (*l).next;
    }
    return img;
}
unsafe extern "C" fn cmpint(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
    mut data: gpointer,
) -> libc::c_int {
    let mut a: libc::c_int = *(p1 as *mut libc::c_int);
    let mut b: libc::c_int = *(p2 as *mut libc::c_int);
    if a < b {
        return -(1 as libc::c_int)
    } else if a == b {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn median_filter(
    mut data: *mut libc::c_int,
    mut size: libc::c_int,
    mut filtersize: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut result: *mut libc::c_int = g_malloc0(
        (size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut sortbuf: *mut libc::c_int = g_malloc0(
        (filtersize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < size {
        let mut i1: libc::c_int = i - (filtersize - 1 as libc::c_int) / 2 as libc::c_int;
        let mut i2: libc::c_int = i + (filtersize - 1 as libc::c_int) / 2 as libc::c_int;
        if i1 < 0 as libc::c_int {
            i1 = 0 as libc::c_int;
        }
        if i2 >= size {
            i2 = size - 1 as libc::c_int;
        }
        memmove(
            sortbuf as *mut libc::c_void,
            data.offset(i1 as isize) as *const libc::c_void,
            ((i2 - i1 + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        g_qsort_with_data(
            sortbuf as gconstpointer,
            i2 - i1 + 1 as libc::c_int,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
            Some(
                cmpint
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        gpointer,
                    ) -> libc::c_int,
            ),
            0 as *mut libc::c_void,
        );
        *result
            .offset(
                i as isize,
            ) = *sortbuf
            .offset(((i2 - i1 + 1 as libc::c_int) / 2 as libc::c_int) as isize);
        i += 1;
    }
    memmove(
        data as *mut libc::c_void,
        result as *const libc::c_void,
        (size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    g_free(result as gpointer);
    g_free(sortbuf as gpointer);
}
unsafe extern "C" fn interpolate_lines(
    mut ctx: *mut fpi_line_asmbl_ctx,
    mut line1: *mut GSList,
    mut y1_f: gint32,
    mut line2: *mut GSList,
    mut y2_f: gint32,
    mut output: *mut libc::c_uchar,
    mut yi_f: gint32,
    mut size: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut p1: libc::c_uchar = 0;
    let mut p2: libc::c_uchar = 0;
    if line1.is_null() || line2.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < size {
        let mut unscaled: gint = 0;
        p1 = ((*ctx).get_pixel)
            .expect("non-null function pointer")(ctx, line1, i as libc::c_uint);
        p2 = ((*ctx).get_pixel)
            .expect("non-null function pointer")(ctx, line2, i as libc::c_uint);
        unscaled = (yi_f - y1_f) * p2 as libc::c_int + (y2_f - yi_f) * p1 as libc::c_int;
        *output.offset(i as isize) = (unscaled / (y2_f - y1_f)) as libc::c_uchar;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn fpi_assemble_lines(
    mut ctx: *mut fpi_line_asmbl_ctx,
    mut lines: *mut GSList,
    mut num_lines: size_t,
) -> *mut FpImage {
    let mut i: libc::c_int = 0;
    let mut row1: *mut GSList = 0 as *mut GSList;
    let mut row2: *mut GSList = 0 as *mut GSList;
    let mut y_f: gint32 = 0 as libc::c_int;
    let mut line_ind: libc::c_int = 0 as libc::c_int;
    let mut offsets: *mut libc::c_int = ({
        let mut __n: gsize = num_lines.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let mut __s: gsize = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong;
        let mut __p: gpointer = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = g_malloc0(__n);
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut libc::c_int;
    let mut output: *mut libc::c_uchar = g_malloc0(
        ((*ctx).line_width).wrapping_mul((*ctx).max_height) as gsize,
    ) as *mut libc::c_uchar;
    let mut img: *mut FpImage = 0 as *mut FpImage;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !lines.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-assembling\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"fpi_assemble_lines\0"))
                .as_ptr(),
            b"lines != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FpImage;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if num_lines >= 2 as libc::c_int as libc::c_ulong {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-assembling\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"fpi_assemble_lines\0"))
                .as_ptr(),
            b"num_lines >= 2\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FpImage;
    }
    g_log(
        b"libfprint-assembling\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%li\0" as *const u8 as *const libc::c_char,
        g_get_real_time(),
    );
    row1 = lines;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < num_lines.wrapping_sub(1 as libc::c_int as libc::c_ulong) && !row1.is_null()
    {
        let mut bestmatch: libc::c_int = i;
        let mut bestdiff: libc::c_int = 0 as libc::c_int;
        let mut j: libc::c_int = 0;
        let mut firstrow: libc::c_int = 0;
        let mut lastrow: libc::c_int = 0;
        firstrow = i + 1 as libc::c_int;
        lastrow = (if ((i as libc::c_uint).wrapping_add((*ctx).max_search_offset)
            as libc::c_ulong) < num_lines.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            (i as libc::c_uint).wrapping_add((*ctx).max_search_offset) as libc::c_ulong
        } else {
            num_lines.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        }) as libc::c_int;
        row2 = if !row1.is_null() { (*row1).next } else { 0 as *mut GSList };
        j = firstrow;
        while j <= lastrow {
            let mut diff: libc::c_int = ((*ctx).get_deviation)
                .expect("non-null function pointer")(ctx, row1, row2);
            if j == firstrow || diff < bestdiff {
                bestdiff = diff;
                bestmatch = j;
            }
            row2 = if !row2.is_null() { (*row2).next } else { 0 as *mut GSList };
            j += 1;
        }
        *offsets.offset((i / 2 as libc::c_int) as isize) = bestmatch - i;
        g_log(
            b"libfprint-assembling\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"%d\0" as *const u8 as *const libc::c_char,
            *offsets.offset((i / 2 as libc::c_int) as isize),
        );
        row1 = if !row1.is_null() { (*row1).next } else { 0 as *mut GSList };
        if !row1.is_null() {
            row1 = if !row1.is_null() { (*row1).next } else { 0 as *mut GSList };
        }
        i += 2 as libc::c_int;
    }
    median_filter(
        offsets,
        num_lines
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        (*ctx).median_filter_size as libc::c_int,
    );
    g_log(
        b"libfprint-assembling\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"offsets_filtered: %li\0" as *const u8 as *const libc::c_char,
        g_get_real_time(),
    );
    i = 0 as libc::c_int;
    while i as libc::c_ulong
        <= num_lines
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        g_log(
            b"libfprint-assembling\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"%d\0" as *const u8 as *const libc::c_char,
            *offsets.offset(i as isize),
        );
        i += 1;
    }
    row1 = lines;
    i = 0 as libc::c_int;
    's_183: while (i as libc::c_ulong)
        < num_lines.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        let mut offset: libc::c_int = *offsets.offset((i / 2 as libc::c_int) as isize);
        if offset > 0 as libc::c_int {
            let mut ynext_f: gint32 = (y_f as libc::c_uint)
                .wrapping_add(
                    ((*ctx).resolution << 16 as libc::c_int)
                        .wrapping_div(offset as libc::c_uint),
                ) as gint32;
            while (line_ind << 16 as libc::c_int) < ynext_f {
                if line_ind as libc::c_uint
                    > ((*ctx).max_height).wrapping_sub(1 as libc::c_int as libc::c_uint)
                {
                    break 's_183;
                }
                interpolate_lines(
                    ctx,
                    row1,
                    y_f,
                    if !row1.is_null() { (*row1).next } else { 0 as *mut GSList },
                    ynext_f,
                    output
                        .offset(
                            (line_ind as libc::c_uint).wrapping_mul((*ctx).line_width)
                                as isize,
                        ),
                    line_ind << 16 as libc::c_int,
                    (*ctx).line_width as libc::c_int,
                );
                line_ind += 1;
            }
            y_f = ynext_f;
        }
        i += 1;
        row1 = (if !row1.is_null() { (*row1).next } else { 0 as *mut GSList });
    }
    img = fp_image_new((*ctx).line_width as gint, line_ind);
    (*img).height = line_ind as guint;
    (*img).width = (*ctx).line_width;
    (*img).flags = FPI_IMAGE_V_FLIPPED;
    memmove(
        (*img).data as *mut libc::c_void,
        output as *const libc::c_void,
        ((*ctx).line_width).wrapping_mul(line_ind as libc::c_uint) as libc::c_ulong,
    );
    g_free(offsets as gpointer);
    g_free(output as gpointer);
    return img;
}
