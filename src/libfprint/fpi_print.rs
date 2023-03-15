use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GVariant;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn g_ptr_array_new_with_free_func(
        element_free_func: GDestroyNotify,
    ) -> *mut GPtrArray;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_set_error(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        format: *const gchar,
        _: ...
    );
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn g_date_new() -> *mut GDate;
    fn g_date_new_dmy(day: GDateDay, month: GDateMonth, year: GDateYear) -> *mut GDate;
    fn g_date_valid(date: *const GDate) -> gboolean;
    fn g_date_get_month(date: *const GDate) -> GDateMonth;
    fn g_date_get_year(date: *const GDate) -> GDateYear;
    fn g_date_get_day(date: *const GDate) -> GDateDay;
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
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
    fn g_random_int() -> guint32;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_ascii_strtod(nptr: *const gchar, endptr: *mut *mut gchar) -> gdouble;
    fn g_ascii_strtoll(
        nptr: *const gchar,
        endptr: *mut *mut gchar,
        base: guint,
    ) -> gint64;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_strcmp0(str1: *const libc::c_char, str2: *const libc::c_char) -> libc::c_int;
    fn g_assertion_message(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        message: *const libc::c_char,
    );
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
    fn g_type_check_instance_cast(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> *mut GTypeInstance;
    fn g_type_check_instance_is_a(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> gboolean;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_io_error_quark() -> GQuark;
    fn fp_image_get_minutiae(self_0: *mut FpImage) -> *mut GPtrArray;
    fn fp_print_get_type() -> GType;
    fn fp_print_get_finger(print: *mut FpPrint) -> FpFinger;
    fn fp_print_get_username(print: *mut FpPrint) -> *const gchar;
    fn fp_print_get_enroll_date(print: *mut FpPrint) -> *const GDate;
    fn fp_print_set_finger(print: *mut FpPrint, finger: FpFinger);
    fn fp_print_set_username(print: *mut FpPrint, username: *const gchar);
    fn fp_print_set_enroll_date(print: *mut FpPrint, enroll_date: *const GDate);
    fn sort_x_y(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int;
    fn lfs2nist_minutia_XYT(
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *const MINUTIA,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn bozorth_to_gallery(
        _: libc::c_int,
        _: *mut xyt_struct,
        _: *mut xyt_struct,
    ) -> libc::c_int;
    fn bozorth_probe_init(_: *mut xyt_struct) -> libc::c_int;
    fn fpi_device_error_new_msg(
        error: FpDeviceError,
        msg: *const gchar,
        _: ...
    ) -> *mut GError;
}
pub type size_t = libc::c_ulong;
pub type guint8 = libc::c_uchar;
pub type guint16 = libc::c_ushort;
pub type gint32 = libc::c_int;
pub type guint32 = libc::c_uint;
pub type gint64 = libc::c_long;
pub type gsize = libc::c_ulong;
pub type gchar = libc::c_char;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type guint = libc::c_uint;
pub type gdouble = libc::c_double;
pub type gpointer = *mut libc::c_void;
pub type gconstpointer = *const libc::c_void;
pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type GData = _GData;
pub type GDateYear = guint16;
pub type GDateDay = guint8;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GDate {
    #[bitfield(name = "julian_days", ty = "guint", bits = "0..=31")]
    #[bitfield(name = "julian", ty = "guint", bits = "32..=32")]
    #[bitfield(name = "dmy", ty = "guint", bits = "33..=33")]
    #[bitfield(name = "day", ty = "guint", bits = "34..=39")]
    #[bitfield(name = "month", ty = "guint", bits = "40..=43")]
    #[bitfield(name = "year", ty = "guint", bits = "44..=59")]
    pub julian_days_julian_dmy_day_month_year: [u8; 8],
}
pub type GDate = _GDate;
pub type GDateMonth = libc::c_uint;
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
pub type GDate_autoptr = *mut GDate;
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
pub type GInitiallyUnowned = _GObject;
pub type C2RustUnnamed = libc::c_uint;
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
pub struct _FpPrint {
    pub parent_instance: GInitiallyUnowned,
    pub type_0: FpiPrintType,
    pub driver: *mut gchar,
    pub device_id: *mut gchar,
    pub device_stored: gboolean,
    pub image: *mut FpImage,
    pub finger: FpFinger,
    pub username: *mut gchar,
    pub description: *mut gchar,
    pub enroll_date: *mut GDate,
    pub data: *mut GVariant,
    pub prints: *mut GPtrArray,
}
pub type FpFinger = libc::c_uint;
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
pub type FpiPrintType = libc::c_uint;
pub const FPI_PRINT_NBIS: FpiPrintType = 2;
pub const FPI_PRINT_RAW: FpiPrintType = 1;
pub const FPI_PRINT_UNDEFINED: FpiPrintType = 0;
pub type FpPrint = _FpPrint;
pub type FpDeviceError = libc::c_uint;
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
pub type FpiMatchResult = libc::c_int;
pub const FPI_MATCH_SUCCESS: FpiMatchResult = 1;
pub const FPI_MATCH_FAIL: FpiMatchResult = 0;
pub const FPI_MATCH_ERROR: FpiMatchResult = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xyt_struct {
    pub nrows: libc::c_int,
    pub xcol: [libc::c_int; 200],
    pub ycol: [libc::c_int; 200],
    pub thetacol: [libc::c_int; 200],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fp_minutiae {
    pub alloc: libc::c_int,
    pub num: libc::c_int,
    pub list: *mut *mut fp_minutia,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct minutiae_struct {
    pub col: [libc::c_int; 4],
}
pub type MINUTIA = fp_minutia;
#[inline]
unsafe extern "C" fn FP_IS_PRINT(mut ptr: gpointer) -> gboolean {
    return ({
        let mut __inst: *mut GTypeInstance = ptr as *mut GTypeInstance;
        let mut __t: GType = fp_print_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = 0 as libc::c_int;
        } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
            __r = (0 as libc::c_int == 0) as libc::c_int;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    });
}
#[no_mangle]
pub unsafe extern "C" fn fpi_print_add_print(
    mut print: *mut FpPrint,
    mut add: *mut FpPrint,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*print).type_0 as libc::c_uint
            == FPI_PRINT_NBIS as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fpi_print_add_print\0"))
                .as_ptr(),
            b"print->type == FPI_PRINT_NBIS\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*add).type_0 as libc::c_uint == FPI_PRINT_NBIS as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fpi_print_add_print\0"))
                .as_ptr(),
            b"add->type == FPI_PRINT_NBIS\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*(*add).prints).len == 1 as libc::c_int as libc::c_uint {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-print.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fpi_print_add_print\0"))
                .as_ptr(),
            b"add->prints->len == 1\0" as *const u8 as *const libc::c_char,
        );
    }
    g_ptr_array_add(
        (*print).prints,
        ({
            g_memdup2(
                *((*(*add).prints).pdata).offset(0 as libc::c_int as isize)
                    as gconstpointer,
                ::core::mem::size_of::<xyt_struct>() as libc::c_ulong,
            )
        }),
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_print_set_type(
    mut print: *mut FpPrint,
    mut type_0: FpiPrintType,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(print as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"fpi_print_set_type\0"))
                .as_ptr(),
            b"FP_IS_PRINT (print)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*print).type_0 as libc::c_uint
            == FPI_PRINT_UNDEFINED as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"fpi_print_set_type\0"))
                .as_ptr(),
            b"print->type == FPI_PRINT_UNDEFINED\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*print).type_0 = type_0;
    if (*print).type_0 as libc::c_uint == FPI_PRINT_NBIS as libc::c_int as libc::c_uint {
        if !(({
            let mut _g_boolean_var_: libc::c_int = 0;
            if ((*print).prints).is_null() {
                _g_boolean_var_ = 1 as libc::c_int;
            } else {
                _g_boolean_var_ = 0 as libc::c_int;
            }
            _g_boolean_var_
        }) as libc::c_long != 0)
        {
            g_assertion_message(
                b"libfprint-print\0" as *const u8 as *const libc::c_char,
                b"../libfprint/fpi-print.c\0" as *const u8 as *const libc::c_char,
                76 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 19],
                    &[libc::c_char; 19],
                >(b"fpi_print_set_type\0"))
                    .as_ptr(),
                b"'print->prints' should be NULL\0" as *const u8 as *const libc::c_char,
            );
        }
        (*print)
            .prints = g_ptr_array_new_with_free_func(
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    g_object_notify(
        g_type_check_instance_cast(
            print as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"fpi-type\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_print_set_device_stored(
    mut print: *mut FpPrint,
    mut device_stored: gboolean,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(print as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fpi_print_set_device_stored\0"))
                .as_ptr(),
            b"FP_IS_PRINT (print)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*print).device_stored = device_stored;
    g_object_notify(
        g_type_check_instance_cast(
            print as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"device-stored\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn minutiae_to_xyt(
    mut minutiae: *mut fp_minutiae,
    mut bwidth: libc::c_int,
    mut bheight: libc::c_int,
    mut xyt: *mut xyt_struct,
) {
    let mut i: libc::c_int = 0;
    let mut minutia: *mut fp_minutia = 0 as *mut fp_minutia;
    let mut c: [minutiae_struct; 1000] = [minutiae_struct { col: [0; 4] }; 1000];
    let mut nmin: libc::c_int = if (*minutiae).num < 200 as libc::c_int {
        (*minutiae).num
    } else {
        200 as libc::c_int
    };
    i = 0 as libc::c_int;
    while i < nmin {
        minutia = *((*minutiae).list).offset(i as isize);
        lfs2nist_minutia_XYT(
            &mut *((*c.as_mut_ptr().offset(i as isize)).col)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize),
            &mut *((*c.as_mut_ptr().offset(i as isize)).col)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize),
            &mut *((*c.as_mut_ptr().offset(i as isize)).col)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize),
            minutia,
            bwidth,
            bheight,
        );
        c[i as usize]
            .col[3 as libc::c_int
            as usize] = (if (*minutia).reliability * 100.0f64
            < 0 as libc::c_int as libc::c_double
        {
            (*minutia).reliability * 100.0f64 - 0.5f64
        } else {
            (*minutia).reliability * 100.0f64 + 0.5f64
        }) as libc::c_int;
        if c[i as usize].col[2 as libc::c_int as usize] > 180 as libc::c_int {
            c[i as usize].col[2 as libc::c_int as usize] -= 360 as libc::c_int;
        }
        i += 1;
    }
    qsort(
        &mut c as *mut [minutiae_struct; 1000] as *mut libc::c_void,
        nmin as size_t,
        ::core::mem::size_of::<minutiae_struct>() as libc::c_ulong,
        Some(
            sort_x_y
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < nmin {
        (*xyt).xcol[i as usize] = c[i as usize].col[0 as libc::c_int as usize];
        (*xyt).ycol[i as usize] = c[i as usize].col[1 as libc::c_int as usize];
        (*xyt).thetacol[i as usize] = c[i as usize].col[2 as libc::c_int as usize];
        i += 1;
    }
    (*xyt).nrows = nmin;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_print_add_from_image(
    mut print: *mut FpPrint,
    mut image: *mut FpImage,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut minutiae: *mut GPtrArray = 0 as *mut GPtrArray;
    let mut _minutiae: fp_minutiae = fp_minutiae {
        alloc: 0,
        num: 0,
        list: 0 as *mut *mut fp_minutia,
    };
    let mut xyt: *mut xyt_struct = 0 as *mut xyt_struct;
    if (*print).type_0 as libc::c_uint != FPI_PRINT_NBIS as libc::c_int as libc::c_uint
        || image.is_null()
    {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_DATA as libc::c_int,
            b"Cannot add print data from image!\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    minutiae = fp_image_get_minutiae(image);
    if minutiae.is_null() || (*minutiae).len == 0 as libc::c_int as libc::c_uint {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_DATA as libc::c_int,
            b"No minutiae found in image or not yet detected!\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    _minutiae.num = (*minutiae).len as libc::c_int;
    _minutiae.list = (*minutiae).pdata as *mut *mut fp_minutia;
    _minutiae.alloc = (*minutiae).len as libc::c_int;
    xyt = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<xyt_struct>() as libc::c_ulong;
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
    }) as *mut xyt_struct;
    minutiae_to_xyt(
        &mut _minutiae,
        (*image).width as libc::c_int,
        (*image).height as libc::c_int,
        xyt,
    );
    g_ptr_array_add((*print).prints, xyt as gpointer);
    let mut _pp: C2RustUnnamed_0 = C2RustUnnamed_0 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*print).image as *mut *mut FpImage as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    (*print).image = g_object_ref(image as gpointer) as *mut FpImage;
    g_object_notify(
        g_type_check_instance_cast(
            print as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"image\0" as *const u8 as *const libc::c_char,
    );
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_print_bz3_match(
    mut template: *mut FpPrint,
    mut print: *mut FpPrint,
    mut bz3_threshold: gint,
    mut error: *mut *mut GError,
) -> FpiMatchResult {
    let mut pstruct: *mut xyt_struct = 0 as *mut xyt_struct;
    let mut probe_len: gint = 0;
    let mut i: gint = 0;
    if (*template).type_0 as libc::c_uint
        != FPI_PRINT_NBIS as libc::c_int as libc::c_uint
        || (*print).type_0 as libc::c_uint
            != FPI_PRINT_NBIS as libc::c_int as libc::c_uint
    {
        *error = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_NOT_SUPPORTED,
            b"It is only possible to match NBIS type print data\0" as *const u8
                as *const libc::c_char,
        );
        return FPI_MATCH_ERROR;
    }
    if (*(*print).prints).len != 1 as libc::c_int as libc::c_uint {
        *error = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_GENERAL,
            b"New print contains more than one print!\0" as *const u8
                as *const libc::c_char,
        );
        return FPI_MATCH_ERROR;
    }
    pstruct = *((*(*print).prints).pdata).offset(0 as libc::c_int as isize)
        as *mut xyt_struct;
    probe_len = bozorth_probe_init(pstruct);
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*(*template).prints).len {
        let mut gstruct: *mut xyt_struct = 0 as *mut xyt_struct;
        let mut score: gint = 0;
        gstruct = *((*(*template).prints).pdata).offset(i as isize) as *mut xyt_struct;
        score = bozorth_to_gallery(probe_len, pstruct, gstruct);
        g_log(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"score %d/%d\0" as *const u8 as *const libc::c_char,
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
pub unsafe extern "C" fn fpi_print_generate_user_id(
    mut print: *mut FpPrint,
) -> *mut gchar {
    let mut username: *const gchar = 0 as *const gchar;
    let mut user_id: *mut gchar = 0 as *mut gchar;
    let mut date: *const GDate = 0 as *const GDate;
    let mut y: gint = 0 as libc::c_int;
    let mut m: gint = 0 as libc::c_int;
    let mut d: gint = 0 as libc::c_int;
    let mut rand_id: gint32 = 0 as libc::c_int;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !print.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-print.c\0" as *const u8 as *const libc::c_char,
            282 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_print_generate_user_id\0"))
                .as_ptr(),
            b"print\0" as *const u8 as *const libc::c_char,
        );
    }
    date = fp_print_get_enroll_date(print);
    if !date.is_null() && g_date_valid(date) != 0 {
        y = g_date_get_year(date) as gint;
        m = g_date_get_month(date) as gint;
        d = g_date_get_day(date) as gint;
    }
    username = fp_print_get_username(print);
    if username.is_null() {
        username = b"nobody\0" as *const u8 as *const libc::c_char;
    }
    if g_strcmp0(
        g_getenv(b"FP_DEVICE_EMULATION\0" as *const u8 as *const libc::c_char),
        b"1\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        rand_id = 0 as libc::c_int;
    } else {
        rand_id = g_random_int() as gint32;
    }
    user_id = g_strdup_printf(
        b"FP1-%04d%02d%02d-%X-%08X-%s\0" as *const u8 as *const libc::c_char,
        y,
        m,
        d,
        fp_print_get_finger(print) as libc::c_uint,
        rand_id,
        username,
    );
    return user_id;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_print_fill_from_user_id(
    mut print: *mut FpPrint,
    mut user_id: *const libc::c_char,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !user_id.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fpi_print_fill_from_user_id\0"))
                .as_ptr(),
            b"user_id\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if g_str_has_prefix(user_id, b"FP1-\0" as *const u8 as *const libc::c_char) != 0
        && strlen(user_id) >= 24 as libc::c_int as libc::c_ulong
        && *user_id.offset(12 as libc::c_int as isize) as libc::c_int == '-' as i32
        && *user_id.offset(14 as libc::c_int as isize) as libc::c_int == '-' as i32
        && *user_id.offset(23 as libc::c_int as isize) as libc::c_int == '-' as i32
    {
        let mut copy: *mut gchar = g_strdup(user_id);
        let mut date: GDate_autoptr = 0 as GDate_autoptr;
        let mut date_ymd: gint32 = 0;
        let mut finger: gint32 = 0;
        let mut username: *mut gchar = 0 as *mut gchar;
        *copy.offset(12 as libc::c_int as isize) = '\0' as i32 as gchar;
        date_ymd = g_ascii_strtod(
            copy.offset(4 as libc::c_int as isize),
            0 as *mut *mut gchar,
        ) as gint32;
        if date_ymd > 0 as libc::c_int {
            date = g_date_new_dmy(
                (date_ymd % 100 as libc::c_int) as GDateDay,
                (date_ymd / 100 as libc::c_int % 100 as libc::c_int) as GDateMonth,
                (date_ymd / 10000 as libc::c_int) as GDateYear,
            );
        } else {
            date = g_date_new();
        }
        fp_print_set_enroll_date(print, date as *const GDate);
        *copy.offset(14 as libc::c_int as isize) = '\0' as i32 as gchar;
        finger = g_ascii_strtoll(
            copy.offset(13 as libc::c_int as isize),
            0 as *mut *mut gchar,
            16 as libc::c_int as guint,
        ) as gint32;
        fp_print_set_finger(print, finger as FpFinger);
        username = copy.offset(24 as libc::c_int as isize);
        if strlen(username) > 0 as libc::c_int as libc::c_ulong
            && g_strcmp0(username, b"nobody\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
        {
            fp_print_set_username(print, username);
        }
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    return 0 as libc::c_int;
}
