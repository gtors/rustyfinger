use ::c2rust_bitfields;
use ::libc;
use chrono::prelude::*;
use rand::prelude::*;
use std::env;
use xrono;

extern "C" {
    pub type _GData;
    pub type _GVariant;
    fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: i32, format: *const u8, _: ...);
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
    fn g_date_new() -> *mut GDate;
    fn g_date_new_dmy(day: GDateDay, month: GDateMonth, year: GDateYear) -> *mut GDate;
    fn g_date_valid(date: *const GDate) -> bool;
    fn g_date_get_month(date: *const GDate) -> GDateMonth;
    fn g_date_get_year(date: *const GDate) -> GDateYear;
    fn g_date_get_day(date: *const GDate) -> GDateDay;
    fn g_getenv(variable: *const u8) -> *const u8;
    fn g_log(log_domain: *const u8, log_level: GLogLevelFlags, format: *const u8, _: ...);
    fn g_return_if_fail_warning(
        log_domain: *const u8,
        pretty_function: *const u8,
        expression: *const u8,
    );
    fn g_ascii_strtod(nptr: *const u8, endptr: *mut *mut u8) -> f32;
    fn g_ascii_strtoll(nptr: *const u8, endptr: *mut *mut u8, base: u64) -> i64;
    fn g_strdup(str: *const u8) -> *mut u8;
    fn g_strdup_printf(format: *const u8, _: ...) -> *mut u8;
    fn g_memdup2(mem: gconstpointer, byte_size: u64) -> gpointer;
    fn g_assertion_message(
        domain: *const u8,
        file: *const u8,
        line: i32,
        func: *const u8,
        message: *const u8,
    );
    fn g_assertion_message_expr(
        domain: *const u8,
        file: *const u8,
        line: i32,
        func: *const u8,
        expr: *const u8,
    ) -> !;
    fn g_type_check_instance_cast(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> *mut GTypeInstance;
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> bool;
    fn g_object_notify(object: *mut GObject, property_name: *const u8);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_io_error_quark() -> GQuark;
    fn fp_image_get_minutiae(self_0: *mut FpImage) -> *mut GPtrArray;
    fn sort_x_y(_: *const libc::c_void, _: *const libc::c_void) -> i32;
    fn lfs2nist_minutia_XYT(
        _: *mut i32,
        _: *mut i32,
        _: *mut i32,
        _: *const MINUTIA,
        _: i32,
        _: i32,
    );
    fn bozorth_to_gallery(_: i32, _: *mut xyt_struct, _: *mut xyt_struct) -> i32;
    fn bozorth_probe_init(_: *mut xyt_struct) -> i32;
    fn fpi_device_error_new_msg(error: FpDeviceError, msg: *const u8, _: ...) -> *mut GError;
}
pub type gpointer = *mut libc::c_void;
pub type gconstpointer = *const libc::c_void;
//pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;

#[derive(Copy, Clone)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: u64,
}
pub type GPtrArray = _GPtrArray;
pub type GQuark = u32;
#[derive(Copy, Clone)]
pub struct _GError {
    pub domain: GQuark,
    pub code: i32,
    pub message: *mut u8,
}
pub type GError = _GError;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>;
pub type GData = _GData;
pub type GDateYear = u16;
pub type GDateDay = u8;
#[derive(Copy, Clone, BitfieldStruct)]
pub struct _GDate {
    #[bitfield(name = "julian_days", ty = "u64", bits = "0..=31")]
    #[bitfield(name = "julian", ty = "u64", bits = "32..=32")]
    #[bitfield(name = "dmy", ty = "u64", bits = "33..=33")]
    #[bitfield(name = "day", ty = "u64", bits = "34..=39")]
    #[bitfield(name = "month", ty = "u64", bits = "40..=43")]
    #[bitfield(name = "year", ty = "u64", bits = "44..=59")]
    pub julian_days_julian_dmy_day_month_year: [u8; 8],
}
pub type GDate = _GDate;
pub type GDateMonth = u64;
pub const G_DATE_DECEMBER: GDateMonth = 12;
pub const G_DATE_NOVEMBER: GDateMonth = 11;
pub const G_DATE_OCTOBER: GDateMonth = 10;
pub const G_DATE_SEPTEMBER: GDateMonth = 9;
pub const G_DATE_AUGUST: GDateMonth = 8;
pub const G_DATE_JULY: GDateMonth = 7;
pub const G_DATE_JUNE: GDateMonth = 6;
pub const G_DATE_MAY: GDateMonth = 5;
pub const G_DATE_APRIL: GDateMonth = 4;
pub const G_DATE_MARCH: GDateMonth = 3;
pub const G_DATE_FEBRUARY: GDateMonth = 2;
pub const G_DATE_JANUARY: GDateMonth = 1;
pub const G_DATE_BAD_MONTH: GDateMonth = 0;
pub type GVariant = _GVariant;
pub type GLogLevelFlags = i32;
pub const G_LOG_LEVEL_MASK: GLogLevelFlags = -4;
pub const G_LOG_LEVEL_DEBUG: GLogLevelFlags = 128;
pub const G_LOG_LEVEL_INFO: GLogLevelFlags = 64;
pub const G_LOG_LEVEL_MESSAGE: GLogLevelFlags = 32;
pub const G_LOG_LEVEL_WARNING: GLogLevelFlags = 16;
pub const G_LOG_LEVEL_CRITICAL: GLogLevelFlags = 8;
pub const G_LOG_LEVEL_ERROR: GLogLevelFlags = 4;
pub const G_LOG_FLAG_FATAL: GLogLevelFlags = 2;
pub const G_LOG_FLAG_RECURSION: GLogLevelFlags = 1;
pub type GDate_autoptr = *mut GDate;
pub type GType = u64;
#[derive(Copy, Clone)]
pub struct _GTypeClass {
    pub g_type: GType,
}
pub type GTypeClass = _GTypeClass;
#[derive(Copy, Clone)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
#[derive(Copy, Clone)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: u64,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
pub type GInitiallyUnowned = _GObject;
pub type C2RustUnnamed = u64;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed = 0;

#[derive(Copy, Clone)]
pub struct Minutia {
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

#[derive(Copy, Clone)]
pub struct _FpImage {
    pub parent: GObject,
    pub width: u64,
    pub height: u64,
    pub ppmm: f32,
    pub flags: FpiImageFlags,
    pub data: *mut u8,
    pub binarized: *mut u8,
    pub minutiae: *mut GPtrArray,
    pub ref_count: u64,
}
pub type FpiImageFlags = u64;
pub const FPI_IMAGE_PARTIAL: FpiImageFlags = 8;
pub const FPI_IMAGE_COLORS_INVERTED: FpiImageFlags = 4;
pub const FPI_IMAGE_H_FLIPPED: FpiImageFlags = 2;
pub const FPI_IMAGE_V_FLIPPED: FpiImageFlags = 1;
pub type FpImage = _FpImage;

#[derive(Copy, Clone)]
pub struct _FpPrint {
    pub parent_instance: GInitiallyUnowned,
    pub type_0: FpiPrintType,
    pub driver: *mut u8,
    pub device_id: *mut u8,
    pub device_stored: bool,
    pub image: *mut FpImage,
    pub finger: FpFinger,
    pub username: *mut u8,
    pub description: *mut u8,
    pub enroll_date: *mut GDate,
    pub data: *mut GVariant,
    pub prints: *mut GPtrArray,
}

pub type FpFinger = u64;
pub const FP_FINGER_LAST: FpFinger = 10;
pub const FP_FINGER_FIRST: FpFinger = 1;
pub const FP_FINGER_RIGHT_LITTLE: FpFinger = 10;
pub const FP_FINGER_RIGHT_RING: FpFinger = 9;
pub const FP_FINGER_RIGHT_MIDDLE: FpFinger = 8;
pub const FP_FINGER_RIGHT_INDEX: FpFinger = 7;
pub const FP_FINGER_RIGHT_THUMB: FpFinger = 6;
pub const FP_FINGER_LEFT_LITTLE: FpFinger = 5;
pub const FP_FINGER_LEFT_RING: FpFinger = 4;
pub const FP_FINGER_LEFT_MIDDLE: FpFinger = 3;
pub const FP_FINGER_LEFT_INDEX: FpFinger = 2;
pub const FP_FINGER_LEFT_THUMB: FpFinger = 1;
pub const FP_FINGER_UNKNOWN: FpFinger = 0;
pub type FpiPrintType = u64;
pub const FPI_PRINT_NBIS: FpiPrintType = 2;
pub const FPI_PRINT_RAW: FpiPrintType = 1;
pub const FPI_PRINT_UNDEFINED: FpiPrintType = 0;
pub type FpPrint = _FpPrint;
pub type FpDeviceError = u64;
pub const FP_DEVICE_ERROR_TOO_HOT: FpDeviceError = 257;
pub const FP_DEVICE_ERROR_REMOVED: FpDeviceError = 256;
pub const FP_DEVICE_ERROR_DATA_DUPLICATE: FpDeviceError = 9;
pub const FP_DEVICE_ERROR_DATA_FULL: FpDeviceError = 8;
pub const FP_DEVICE_ERROR_DATA_NOT_FOUND: FpDeviceError = 7;
pub const FP_DEVICE_ERROR_DATA_INVALID: FpDeviceError = 6;
pub const FP_DEVICE_ERROR_PROTO: FpDeviceError = 5;
pub const FP_DEVICE_ERROR_BUSY: FpDeviceError = 4;
pub const FP_DEVICE_ERROR_ALREADY_OPEN: FpDeviceError = 3;
pub const FP_DEVICE_ERROR_NOT_OPEN: FpDeviceError = 2;
pub const FP_DEVICE_ERROR_NOT_SUPPORTED: FpDeviceError = 1;
pub const FP_DEVICE_ERROR_GENERAL: FpDeviceError = 0;
pub type FpiMatchResult = i32;
pub const FPI_MATCH_SUCCESS: FpiMatchResult = 1;
pub const FPI_MATCH_FAIL: FpiMatchResult = 0;
pub const FPI_MATCH_ERROR: FpiMatchResult = -1;
#[derive(Copy, Clone)]
pub struct xyt_struct {
    pub nrows: i32,
    pub xcol: [i32; 200],
    pub ycol: [i32; 200],
    pub thetacol: [i32; 200],
}
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
    pub in_0: *mut u8,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
pub struct Minutiae {
    pub alloc: i32,
    pub num: i32,
    pub list: *mut *mut Minutia,
}
#[derive(Copy, Clone)]
pub struct minutiae_struct {
    pub col: [i32; 4],
}

pub type MINUTIA = Minutia;
#[inline]
unsafe extern "C" fn FP_IS_PRINT(mut ptr: gpointer) -> bool {
    return ({
        let mut __inst: *mut GTypeInstance = ptr as *mut GTypeInstance;
        let mut __t: GType = fp_print_get_type();
        let mut __r: bool = 0;
        if __inst.is_null() {
            __r = 0;
        } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
            __r = (0 == 0) as i32;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    });
}

#[no_mangle]
pub unsafe extern "C" fn fpi_print_add_print(mut print: *mut FpPrint, mut add: *mut FpPrint) {
    if ({
        let mut _g_boolean_var_: i32 = 0;
        if (*print).type_0 == FPI_PRINT_NBIS as i32 {
            _g_boolean_var_ = 1;
        } else {
            _g_boolean_var_ = 0;
        }
        _g_boolean_var_
    }) as i64
        != 0
    {
    } else {
        g_return_if_fail_warning(
            b"libfprint-print",
            (*::core::mem::transmute::<&[u8; 20], &[u8; 20]>(b"fpi_print_add_print\0")).as_ptr(),
            b"print->type == FPI_PRINT_NBIS",
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: i32 = 0;
        if (*add).type_0 == FPI_PRINT_NBIS as i32 {
            _g_boolean_var_ = 1;
        } else {
            _g_boolean_var_ = 0;
        }
        _g_boolean_var_
    }) as i64
        != 0
    {
    } else {
        g_return_if_fail_warning(
            b"libfprint-print",
            (*::core::mem::transmute::<&[u8; 20], &[u8; 20]>(b"fpi_print_add_print\0")).as_ptr(),
            b"add->type == FPI_PRINT_NBIS",
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: i32 = 0;
        if (*(*add).prints).len == 1 as u64 {
            _g_boolean_var_ = 1;
        } else {
            _g_boolean_var_ = 0;
        }
        _g_boolean_var_
    }) as i64
        != 0
    {
    } else {
        g_assertion_message_expr(
            b"libfprint-print",
            b"../libfprint/fpi-print.c",
            52,
            (*::core::mem::transmute::<&[u8; 20], &[u8; 20]>(b"fpi_print_add_print\0")).as_ptr(),
            b"add->prints->len == 1",
        );
    }
    g_ptr_array_add(
        (*print).prints,
        ({
            g_memdup2(
                *((*(*add).prints).pdata).offset(0) as gconstpointer,
                ::core::mem::size_of::<xyt_struct>() as u64,
            )
        }),
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_print_set_type(mut print: *mut FpPrint, mut type_0: FpiPrintType) {
    if ({
        let mut _g_boolean_var_: i32 = 0;
        if FP_IS_PRINT(print as gpointer) != 0 {
            _g_boolean_var_ = 1;
        } else {
            _g_boolean_var_ = 0;
        }
        _g_boolean_var_
    }) as i64
        != 0
    {
    } else {
        g_return_if_fail_warning(
            b"libfprint-print",
            (*::core::mem::transmute::<&[u8; 19], &[u8; 19]>(b"fpi_print_set_type\0")).as_ptr(),
            b"FP_IS_PRINT (print)",
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: i32 = 0;
        if (*print).type_0 == FPI_PRINT_UNDEFINED as i32 {
            _g_boolean_var_ = 1;
        } else {
            _g_boolean_var_ = 0;
        }
        _g_boolean_var_
    }) as i64
        != 0
    {
    } else {
        g_return_if_fail_warning(
            b"libfprint-print",
            (*::core::mem::transmute::<&[u8; 19], &[u8; 19]>(b"fpi_print_set_type\0")).as_ptr(),
            b"print->type == FPI_PRINT_UNDEFINED",
        );
        return;
    }
    (*print).type_0 = type_0;
    if (*print).type_0 == FPI_PRINT_NBIS as i32 {
        if !(({
            let mut _g_boolean_var_: i32 = 0;
            if ((*print).prints).is_null() {
                _g_boolean_var_ = 1;
            } else {
                _g_boolean_var_ = 0;
            }
            _g_boolean_var_
        }) as i64
            != 0)
        {
            g_assertion_message(
                b"libfprint-print",
                b"../libfprint/fpi-print.c",
                76,
                (*::core::mem::transmute::<&[u8; 19], &[u8; 19]>(b"fpi_print_set_type\0")).as_ptr(),
                b"'print->prints' should be NULL",
            );
        }
        (*print).prints =
            g_ptr_array_new_with_free_func(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    }
    g_object_notify(
        g_type_check_instance_cast(print as *mut GTypeInstance, ((20) << 2) as GType)
            as *mut libc::c_void as *mut GObject,
        b"fpi-type",
    );
}

#[no_mangle]
pub unsafe extern "C" fn fpi_print_set_device_stored(
    mut print: *mut FpPrint,
    mut device_stored: bool,
) {
    if ({
        let mut _g_boolean_var_: i32 = 0;
        if FP_IS_PRINT(print as gpointer) != 0 {
            _g_boolean_var_ = 1;
        } else {
            _g_boolean_var_ = 0;
        }
        _g_boolean_var_
    }) as i64
        != 0
    {
    } else {
        g_return_if_fail_warning(
            b"libfprint-print",
            (*::core::mem::transmute::<&[u8; 28], &[u8; 28]>(b"fpi_print_set_device_stored\0"))
                .as_ptr(),
            b"FP_IS_PRINT (print)",
        );
        return;
    }
    (*print).device_stored = device_stored;
    g_object_notify(
        g_type_check_instance_cast(print as *mut GTypeInstance, ((20) << 2) as GType)
            as *mut libc::c_void as *mut GObject,
        b"device-stored",
    );
}

unsafe extern "C" fn minutiae_to_xyt(
    mut minutiae: *mut Minutiae,
    mut bwidth: i32,
    mut bheight: i32,
    mut xyt: *mut xyt_struct,
) {
    let mut i: i32 = 0;
    let mut minutia: *mut Minutia = 0 as *mut Minutia;
    let mut c: [minutiae_struct; 1000] = [minutiae_struct { col: [0; 4] }; 1000];
    let mut nmin: i32 = if (*minutiae).num < 200 {
        (*minutiae).num
    } else {
        200
    };
    i = 0;
    while i < nmin {
        minutia = *((*minutiae).list).offset(i as isize);
        lfs2nist_minutia_XYT(
            &mut *((*c.as_mut_ptr().offset(i as isize)).col)
                .as_mut_ptr()
                .offset(0),
            &mut *((*c.as_mut_ptr().offset(i as isize)).col)
                .as_mut_ptr()
                .offset(1),
            &mut *((*c.as_mut_ptr().offset(i as isize)).col)
                .as_mut_ptr()
                .offset(2),
            minutia,
            bwidth,
            bheight,
        );
        c[i as usize].col[3] = (if (*minutia).reliability * 100.0f64 < 0 as f32 {
            (*minutia).reliability * 100.0f64 - 0.5f64
        } else {
            (*minutia).reliability * 100.0f64 + 0.5f64
        }) as i32;
        if c[i as usize].col[2] > 180 {
            c[i as usize].col[2] -= 360;
        }
        i += 1;
    }
    qsort(
        &mut c as *mut [minutiae_struct; 1000] as *mut libc::c_void,
        nmin as u64,
        ::core::mem::size_of::<minutiae_struct>() as u64,
        Some(sort_x_y as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32),
    );
    i = 0;
    while i < nmin {
        (*xyt).xcol[i as usize] = c[i as usize].col[0];
        (*xyt).ycol[i as usize] = c[i as usize].col[1];
        (*xyt).thetacol[i as usize] = c[i as usize].col[2];
        i += 1;
    }
    (*xyt).nrows = nmin;
}

#[no_mangle]
pub unsafe extern "C" fn fpi_print_add_from_image(
    mut print: *mut FpPrint,
    mut image: *mut FpImage,
    mut error: *mut *mut GError,
) -> bool {
    let mut minutiae: *mut GPtrArray = 0 as *mut GPtrArray;
    let mut _minutiae: Minutiae = Minutiae {
        alloc: 0,
        num: 0,
        list: 0 as *mut *mut Minutia,
    };
    let mut xyt: *mut xyt_struct = 0 as *mut xyt_struct;
    if (*print).type_0 != FPI_PRINT_NBIS as i32 || image.is_null() {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_DATA as i32,
            b"Cannot add print data from image!",
        );
        return 0;
    }
    minutiae = fp_image_get_minutiae(image);
    if minutiae.is_null() || (*minutiae).len == 0 as u64 {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_DATA as i32,
            b"No minutiae found in image or not yet detected!",
        );
        return 0;
    }
    _minutiae.num = (*minutiae).len as i32;
    _minutiae.list = (*minutiae).pdata as *mut *mut Minutia;
    _minutiae.alloc = (*minutiae).len as i32;
    xyt = ({
        let mut __n: u64 = 1 as u64;
        let mut __s: u64 = ::core::mem::size_of::<xyt_struct>() as u64;
        let mut __p: gpointer = 0 as *mut libc::c_void;
        if __s == 1 as u64 {
            __p = g_malloc0(__n);
        } else if 0 != 0
            && (__s == 0 as u64
                || __n
                    <= (9223372036854775807 as u64)
                        .wrapping_mul(2)
                        .wrapping_add(1)
                        .wrapping_div(__s))
        {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut xyt_struct;
    minutiae_to_xyt(
        &mut _minutiae,
        (*image).width as i32,
        (*image).height as i32,
        xyt,
    );
    g_ptr_array_add((*print).prints, xyt as gpointer);
    let mut _pp: C2RustUnnamed_0 = C2RustUnnamed_0 { in_0: 0 as *mut u8 };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify =
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    _pp.in_0 = &mut (*print).image as *mut *mut FpImage as *mut u8;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    (*print).image = g_object_ref(image as gpointer) as *mut FpImage;
    g_object_notify(
        g_type_check_instance_cast(print as *mut GTypeInstance, ((20) << 2) as GType)
            as *mut libc::c_void as *mut GObject,
        b"image",
    );
    return (0 == 0) as i32;
}

#[no_mangle]
pub unsafe extern "C" fn fpi_print_bz3_match(
    mut template: *mut FpPrint,
    mut print: *mut FpPrint,
    mut bz3_threshold: i32,
    mut error: *mut *mut GError,
) -> FpiMatchResult {
    let mut pstruct: *mut xyt_struct = 0 as *mut xyt_struct;
    let mut probe_len: i32 = 0;
    let mut i: i32 = 0;
    if (*template).type_0 != FPI_PRINT_NBIS as i32 || (*print).type_0 != FPI_PRINT_NBIS as i32 {
        *error = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_NOT_SUPPORTED,
            b"It is only possible to match NBIS type print data",
        );
        return FPI_MATCH_ERROR;
    }
    if (*(*print).prints).len != 1 as u64 {
        *error = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_GENERAL,
            b"New print contains more than one print!",
        );
        return FPI_MATCH_ERROR;
    }
    pstruct = *((*(*print).prints).pdata).offset(0) as *mut xyt_struct;
    probe_len = bozorth_probe_init(pstruct);
    i = 0;
    while (i as u64) < (*(*template).prints).len {
        let mut gstruct: *mut xyt_struct = 0 as *mut xyt_struct;
        let mut score: i32 = 0;
        gstruct = *((*(*template).prints).pdata).offset(i as isize) as *mut xyt_struct;
        score = bozorth_to_gallery(probe_len, pstruct, gstruct);
        g_log(
            b"libfprint-print",
            G_LOG_LEVEL_DEBUG,
            b"score %d/%d",
            score,
            bz3_threshold,
        );
        if score >= bz3_threshold {
            return FPI_MATCH_SUCCESS;
        }
        i += 1;
    }
    return FPI_MATCH_FAIL;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_print_generate_user_id(mut print: *mut FpPrint) -> *mut u8 {
    let mut username: *const u8 = 0 as *const u8;
    let mut user_id: *mut u8 = 0 as *mut u8;
    let mut date: *const GDate = 0 as *const GDate;
    let mut y: i32 = 0;
    let mut m: i32 = 0;
    let mut d: i32 = 0;
    let mut rand_id: i32 = 0;

    if ({
        let mut _g_boolean_var_: i32 = 0;
        if !print.is_null() {
            _g_boolean_var_ = 1;
        } else {
            _g_boolean_var_ = 0;
        }
        _g_boolean_var_
    }) as i64
        != 0
    {
    } else {
        g_assertion_message_expr(
            b"libfprint-print",
            b"../libfprint/fpi-print.c",
            282,
            (*::core::mem::transmute::<&[u8; 27], &[u8; 27]>(b"fpi_print_generate_user_id\0"))
                .as_ptr(),
            b"print",
        );
    }
    date = fp_print_get_enroll_date(print);
    if !date.is_null() && g_date_valid(date) != 0 {
        y = g_date_get_year(date) as i32;
        m = g_date_get_month(date) as i32;
        d = g_date_get_day(date) as i32;
    }
    username = fp_print_get_username(print);
    if username.is_null() {
        username = b"nobody";
    }

    if Ok("1") == env::var("FP_DEVICE_EMULATION") {
        rand_id = 0;
    } else {
        rand_id = rand::random();
    }

    user_id = g_strdup_printf(
        b"FP1-%04d%02d%02d-%X-%08X-%s",
        y,
        m,
        d,
        fp_print_get_finger(print) as u64,
        rand_id,
        username,
    );
    return user_id;
}

impl FpPrint {
    /// Parses info from FP1-{date}-{finger}-{username} ID format
    pub fn fill_from_user_id(mut self, user_id: Into<String>) -> bool {
        let user_id: String = user_id.into();

        if !(user_id.starts_with("FP1-") && user_id.len() >= 24) {
            return false;
        }

        let split = user_id.split("-");

        split
            .nth(1)
            .map(|encoded_date| encoded_date.parse::<u32>())
            .map(|encoded_date| {
                if encoded_date > 0 {
                    let d = encoded_date % 100;
                    let m = encoded_date / 100 % 100;
                    let y = encoded_date / 10000;
                    NaiveDate::from_ymd(y, m, d)
                } else {
                    Utc::now().date_naive()
                }
            })
            .map(self.set_enroll_date);

        split
            .nth(2)
            .map(|finger| finger.parse::<u32>())
            .map(FpFinger::try_from)
            .map(self.set_finger);

        split
            .nth(3)
            .filter(|username| username.len() > 0 && username != "nobody")
            .map(self.set_username);

        return true;
    }
}
