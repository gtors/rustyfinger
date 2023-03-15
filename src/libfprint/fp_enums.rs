use ::libc;
extern "C" {
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_enum_register_static(
        name: *const gchar,
        const_static_values: *const GEnumValue,
    ) -> GType;
    fn g_flags_register_static(
        name: *const gchar,
        const_static_values: *const GFlagsValue,
    ) -> GType;
}
pub type gsize = libc::c_ulong;
pub type gchar = libc::c_char;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type guint = libc::c_uint;
pub type gpointer = *mut libc::c_void;
pub type GType = gsize;
pub type GEnumValue = _GEnumValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEnumValue {
    pub value: gint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
pub type GFlagsValue = _GFlagsValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFlagsValue {
    pub value: guint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
pub const FP_DEVICE_TYPE_USB: C2RustUnnamed_1 = 2;
pub const FP_DEVICE_TYPE_UDEV: C2RustUnnamed_1 = 1;
pub const FP_DEVICE_TYPE_VIRTUAL: C2RustUnnamed_1 = 0;
pub const FP_DEVICE_FEATURE_UPDATE_PRINT: C2RustUnnamed_2 = 512;
pub const FP_DEVICE_FEATURE_ALWAYS_ON: C2RustUnnamed_2 = 256;
pub const FP_DEVICE_FEATURE_DUPLICATES_CHECK: C2RustUnnamed_2 = 128;
pub const FP_DEVICE_FEATURE_STORAGE_CLEAR: C2RustUnnamed_2 = 64;
pub const FP_DEVICE_FEATURE_STORAGE_DELETE: C2RustUnnamed_2 = 32;
pub const FP_DEVICE_FEATURE_STORAGE_LIST: C2RustUnnamed_2 = 16;
pub const FP_DEVICE_FEATURE_STORAGE: C2RustUnnamed_2 = 8;
pub const FP_DEVICE_FEATURE_VERIFY: C2RustUnnamed_2 = 4;
pub const FP_DEVICE_FEATURE_IDENTIFY: C2RustUnnamed_2 = 2;
pub const FP_DEVICE_FEATURE_CAPTURE: C2RustUnnamed_2 = 1;
pub const FP_DEVICE_FEATURE_NONE: C2RustUnnamed_2 = 0;
pub const FP_SCAN_TYPE_PRESS: C2RustUnnamed_3 = 1;
pub const FP_SCAN_TYPE_SWIPE: C2RustUnnamed_3 = 0;
pub const FP_TEMPERATURE_HOT: C2RustUnnamed_4 = 2;
pub const FP_TEMPERATURE_WARM: C2RustUnnamed_4 = 1;
pub const FP_TEMPERATURE_COLD: C2RustUnnamed_4 = 0;
pub const FP_DEVICE_RETRY_REMOVE_FINGER: C2RustUnnamed_5 = 3;
pub const FP_DEVICE_RETRY_CENTER_FINGER: C2RustUnnamed_5 = 2;
pub const FP_DEVICE_RETRY_TOO_SHORT: C2RustUnnamed_5 = 1;
pub const FP_DEVICE_RETRY_GENERAL: C2RustUnnamed_5 = 0;
pub const FP_DEVICE_ERROR_TOO_HOT: C2RustUnnamed_6 = 257;
pub const FP_DEVICE_ERROR_REMOVED: C2RustUnnamed_6 = 256;
pub const FP_DEVICE_ERROR_DATA_DUPLICATE: C2RustUnnamed_6 = 9;
pub const FP_DEVICE_ERROR_DATA_FULL: C2RustUnnamed_6 = 8;
pub const FP_DEVICE_ERROR_DATA_NOT_FOUND: C2RustUnnamed_6 = 7;
pub const FP_DEVICE_ERROR_DATA_INVALID: C2RustUnnamed_6 = 6;
pub const FP_DEVICE_ERROR_PROTO: C2RustUnnamed_6 = 5;
pub const FP_DEVICE_ERROR_BUSY: C2RustUnnamed_6 = 4;
pub const FP_DEVICE_ERROR_ALREADY_OPEN: C2RustUnnamed_6 = 3;
pub const FP_DEVICE_ERROR_NOT_OPEN: C2RustUnnamed_6 = 2;
pub const FP_DEVICE_ERROR_NOT_SUPPORTED: C2RustUnnamed_6 = 1;
pub const FP_DEVICE_ERROR_GENERAL: C2RustUnnamed_6 = 0;
pub const FP_FINGER_LAST: C2RustUnnamed = 10;
pub const FP_FINGER_FIRST: C2RustUnnamed = 1;
pub const FP_FINGER_RIGHT_LITTLE: C2RustUnnamed = 10;
pub const FP_FINGER_RIGHT_RING: C2RustUnnamed = 9;
pub const FP_FINGER_RIGHT_MIDDLE: C2RustUnnamed = 8;
pub const FP_FINGER_RIGHT_INDEX: C2RustUnnamed = 7;
pub const FP_FINGER_RIGHT_THUMB: C2RustUnnamed = 6;
pub const FP_FINGER_LEFT_LITTLE: C2RustUnnamed = 5;
pub const FP_FINGER_LEFT_RING: C2RustUnnamed = 4;
pub const FP_FINGER_LEFT_MIDDLE: C2RustUnnamed = 3;
pub const FP_FINGER_LEFT_INDEX: C2RustUnnamed = 2;
pub const FP_FINGER_LEFT_THUMB: C2RustUnnamed = 1;
pub const FP_FINGER_UNKNOWN: C2RustUnnamed = 0;
pub const FP_FINGER_STATUS_PRESENT: C2RustUnnamed_0 = 2;
pub const FP_FINGER_STATUS_NEEDED: C2RustUnnamed_0 = 1;
pub const FP_FINGER_STATUS_NONE: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed = libc::c_uint;
pub type C2RustUnnamed_0 = libc::c_uint;
pub type C2RustUnnamed_1 = libc::c_uint;
pub type C2RustUnnamed_2 = libc::c_uint;
pub type C2RustUnnamed_3 = libc::c_uint;
pub type C2RustUnnamed_4 = libc::c_uint;
pub type C2RustUnnamed_5 = libc::c_uint;
pub type C2RustUnnamed_6 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn fp_device_type_get_type() -> GType {
    static mut gtype_id: gsize = 0 as libc::c_int as gsize;
    static mut values: [GEnumValue; 4] = [
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_TYPE_VIRTUAL as libc::c_int,
                value_name: b"FP_DEVICE_TYPE_VIRTUAL\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"virtual\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_TYPE_UDEV as libc::c_int,
                value_name: b"FP_DEVICE_TYPE_UDEV\0" as *const u8 as *const libc::c_char,
                value_nick: b"udev\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_TYPE_USB as libc::c_int,
                value_name: b"FP_DEVICE_TYPE_USB\0" as *const u8 as *const libc::c_char,
                value_nick: b"usb\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: 0 as libc::c_int,
                value_name: 0 as *const gchar,
                value_nick: 0 as *const gchar,
            };
            init
        },
    ];
    if ({
        if 0 as libc::c_int != 0 {} else {};
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &mut gtype_id as *mut gsize;
            *(&mut gapg_temp_newval
                as *mut gsize) = ::core::intrinsics::atomic_load_seqcst(
                gapg_temp_atomic,
            );
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&mut gtype_id as *mut gsize as *mut libc::c_void) != 0)
            as libc::c_int
    }) != 0
    {
        let mut new_type: GType = g_enum_register_static(
            g_intern_static_string(
                b"FpDeviceType\0" as *const u8 as *const libc::c_char,
            ),
            values.as_ptr(),
        );
        if 0 as libc::c_int != 0 {
            gtype_id = new_type;
        } else {};
        g_once_init_leave(&mut gtype_id as *mut gsize as *mut libc::c_void, new_type);
    }
    return gtype_id;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_feature_get_type() -> GType {
    static mut gtype_id: gsize = 0 as libc::c_int as gsize;
    static mut values: [GFlagsValue; 12] = [
        {
            let mut init = _GFlagsValue {
                value: FP_DEVICE_FEATURE_NONE as libc::c_int as guint,
                value_name: b"FP_DEVICE_FEATURE_NONE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"none\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: FP_DEVICE_FEATURE_CAPTURE as libc::c_int as guint,
                value_name: b"FP_DEVICE_FEATURE_CAPTURE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"capture\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: FP_DEVICE_FEATURE_IDENTIFY as libc::c_int as guint,
                value_name: b"FP_DEVICE_FEATURE_IDENTIFY\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"identify\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: FP_DEVICE_FEATURE_VERIFY as libc::c_int as guint,
                value_name: b"FP_DEVICE_FEATURE_VERIFY\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"verify\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: FP_DEVICE_FEATURE_STORAGE as libc::c_int as guint,
                value_name: b"FP_DEVICE_FEATURE_STORAGE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"storage\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: FP_DEVICE_FEATURE_STORAGE_LIST as libc::c_int as guint,
                value_name: b"FP_DEVICE_FEATURE_STORAGE_LIST\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"storage-list\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: FP_DEVICE_FEATURE_STORAGE_DELETE as libc::c_int as guint,
                value_name: b"FP_DEVICE_FEATURE_STORAGE_DELETE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"storage-delete\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: FP_DEVICE_FEATURE_STORAGE_CLEAR as libc::c_int as guint,
                value_name: b"FP_DEVICE_FEATURE_STORAGE_CLEAR\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"storage-clear\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: FP_DEVICE_FEATURE_DUPLICATES_CHECK as libc::c_int as guint,
                value_name: b"FP_DEVICE_FEATURE_DUPLICATES_CHECK\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"duplicates-check\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: FP_DEVICE_FEATURE_ALWAYS_ON as libc::c_int as guint,
                value_name: b"FP_DEVICE_FEATURE_ALWAYS_ON\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"always-on\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: FP_DEVICE_FEATURE_UPDATE_PRINT as libc::c_int as guint,
                value_name: b"FP_DEVICE_FEATURE_UPDATE_PRINT\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"update-print\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: 0 as libc::c_int as guint,
                value_name: 0 as *const gchar,
                value_nick: 0 as *const gchar,
            };
            init
        },
    ];
    if ({
        if 0 as libc::c_int != 0 {} else {};
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &mut gtype_id as *mut gsize;
            *(&mut gapg_temp_newval
                as *mut gsize) = ::core::intrinsics::atomic_load_seqcst(
                gapg_temp_atomic,
            );
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&mut gtype_id as *mut gsize as *mut libc::c_void) != 0)
            as libc::c_int
    }) != 0
    {
        let mut new_type: GType = g_flags_register_static(
            g_intern_static_string(
                b"FpDeviceFeature\0" as *const u8 as *const libc::c_char,
            ),
            values.as_ptr(),
        );
        if 0 as libc::c_int != 0 {
            gtype_id = new_type;
        } else {};
        g_once_init_leave(&mut gtype_id as *mut gsize as *mut libc::c_void, new_type);
    }
    return gtype_id;
}
#[no_mangle]
pub unsafe extern "C" fn fp_scan_type_get_type() -> GType {
    static mut gtype_id: gsize = 0 as libc::c_int as gsize;
    static mut values: [GEnumValue; 3] = [
        {
            let mut init = _GEnumValue {
                value: FP_SCAN_TYPE_SWIPE as libc::c_int,
                value_name: b"FP_SCAN_TYPE_SWIPE\0" as *const u8 as *const libc::c_char,
                value_nick: b"swipe\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_SCAN_TYPE_PRESS as libc::c_int,
                value_name: b"FP_SCAN_TYPE_PRESS\0" as *const u8 as *const libc::c_char,
                value_nick: b"press\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: 0 as libc::c_int,
                value_name: 0 as *const gchar,
                value_nick: 0 as *const gchar,
            };
            init
        },
    ];
    if ({
        if 0 as libc::c_int != 0 {} else {};
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &mut gtype_id as *mut gsize;
            *(&mut gapg_temp_newval
                as *mut gsize) = ::core::intrinsics::atomic_load_seqcst(
                gapg_temp_atomic,
            );
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&mut gtype_id as *mut gsize as *mut libc::c_void) != 0)
            as libc::c_int
    }) != 0
    {
        let mut new_type: GType = g_enum_register_static(
            g_intern_static_string(b"FpScanType\0" as *const u8 as *const libc::c_char),
            values.as_ptr(),
        );
        if 0 as libc::c_int != 0 {
            gtype_id = new_type;
        } else {};
        g_once_init_leave(&mut gtype_id as *mut gsize as *mut libc::c_void, new_type);
    }
    return gtype_id;
}
#[no_mangle]
pub unsafe extern "C" fn fp_temperature_get_type() -> GType {
    static mut gtype_id: gsize = 0 as libc::c_int as gsize;
    static mut values: [GEnumValue; 4] = [
        {
            let mut init = _GEnumValue {
                value: FP_TEMPERATURE_COLD as libc::c_int,
                value_name: b"FP_TEMPERATURE_COLD\0" as *const u8 as *const libc::c_char,
                value_nick: b"cold\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_TEMPERATURE_WARM as libc::c_int,
                value_name: b"FP_TEMPERATURE_WARM\0" as *const u8 as *const libc::c_char,
                value_nick: b"warm\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_TEMPERATURE_HOT as libc::c_int,
                value_name: b"FP_TEMPERATURE_HOT\0" as *const u8 as *const libc::c_char,
                value_nick: b"hot\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: 0 as libc::c_int,
                value_name: 0 as *const gchar,
                value_nick: 0 as *const gchar,
            };
            init
        },
    ];
    if ({
        if 0 as libc::c_int != 0 {} else {};
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &mut gtype_id as *mut gsize;
            *(&mut gapg_temp_newval
                as *mut gsize) = ::core::intrinsics::atomic_load_seqcst(
                gapg_temp_atomic,
            );
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&mut gtype_id as *mut gsize as *mut libc::c_void) != 0)
            as libc::c_int
    }) != 0
    {
        let mut new_type: GType = g_enum_register_static(
            g_intern_static_string(
                b"FpTemperature\0" as *const u8 as *const libc::c_char,
            ),
            values.as_ptr(),
        );
        if 0 as libc::c_int != 0 {
            gtype_id = new_type;
        } else {};
        g_once_init_leave(&mut gtype_id as *mut gsize as *mut libc::c_void, new_type);
    }
    return gtype_id;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_retry_get_type() -> GType {
    static mut gtype_id: gsize = 0 as libc::c_int as gsize;
    static mut values: [GEnumValue; 5] = [
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_RETRY_GENERAL as libc::c_int,
                value_name: b"FP_DEVICE_RETRY_GENERAL\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"general\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_RETRY_TOO_SHORT as libc::c_int,
                value_name: b"FP_DEVICE_RETRY_TOO_SHORT\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"too-short\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_RETRY_CENTER_FINGER as libc::c_int,
                value_name: b"FP_DEVICE_RETRY_CENTER_FINGER\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"center-finger\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_RETRY_REMOVE_FINGER as libc::c_int,
                value_name: b"FP_DEVICE_RETRY_REMOVE_FINGER\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"remove-finger\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: 0 as libc::c_int,
                value_name: 0 as *const gchar,
                value_nick: 0 as *const gchar,
            };
            init
        },
    ];
    if ({
        if 0 as libc::c_int != 0 {} else {};
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &mut gtype_id as *mut gsize;
            *(&mut gapg_temp_newval
                as *mut gsize) = ::core::intrinsics::atomic_load_seqcst(
                gapg_temp_atomic,
            );
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&mut gtype_id as *mut gsize as *mut libc::c_void) != 0)
            as libc::c_int
    }) != 0
    {
        let mut new_type: GType = g_enum_register_static(
            g_intern_static_string(
                b"FpDeviceRetry\0" as *const u8 as *const libc::c_char,
            ),
            values.as_ptr(),
        );
        if 0 as libc::c_int != 0 {
            gtype_id = new_type;
        } else {};
        g_once_init_leave(&mut gtype_id as *mut gsize as *mut libc::c_void, new_type);
    }
    return gtype_id;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_error_get_type() -> GType {
    static mut gtype_id: gsize = 0 as libc::c_int as gsize;
    static mut values: [GEnumValue; 13] = [
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_ERROR_GENERAL as libc::c_int,
                value_name: b"FP_DEVICE_ERROR_GENERAL\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"general\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_ERROR_NOT_SUPPORTED as libc::c_int,
                value_name: b"FP_DEVICE_ERROR_NOT_SUPPORTED\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"not-supported\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_ERROR_NOT_OPEN as libc::c_int,
                value_name: b"FP_DEVICE_ERROR_NOT_OPEN\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"not-open\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_ERROR_ALREADY_OPEN as libc::c_int,
                value_name: b"FP_DEVICE_ERROR_ALREADY_OPEN\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"already-open\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_ERROR_BUSY as libc::c_int,
                value_name: b"FP_DEVICE_ERROR_BUSY\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"busy\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_ERROR_PROTO as libc::c_int,
                value_name: b"FP_DEVICE_ERROR_PROTO\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"proto\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_ERROR_DATA_INVALID as libc::c_int,
                value_name: b"FP_DEVICE_ERROR_DATA_INVALID\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"data-invalid\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_ERROR_DATA_NOT_FOUND as libc::c_int,
                value_name: b"FP_DEVICE_ERROR_DATA_NOT_FOUND\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"data-not-found\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_ERROR_DATA_FULL as libc::c_int,
                value_name: b"FP_DEVICE_ERROR_DATA_FULL\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"data-full\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_ERROR_DATA_DUPLICATE as libc::c_int,
                value_name: b"FP_DEVICE_ERROR_DATA_DUPLICATE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"data-duplicate\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_ERROR_REMOVED as libc::c_int,
                value_name: b"FP_DEVICE_ERROR_REMOVED\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"removed\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_DEVICE_ERROR_TOO_HOT as libc::c_int,
                value_name: b"FP_DEVICE_ERROR_TOO_HOT\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"too-hot\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: 0 as libc::c_int,
                value_name: 0 as *const gchar,
                value_nick: 0 as *const gchar,
            };
            init
        },
    ];
    if ({
        if 0 as libc::c_int != 0 {} else {};
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &mut gtype_id as *mut gsize;
            *(&mut gapg_temp_newval
                as *mut gsize) = ::core::intrinsics::atomic_load_seqcst(
                gapg_temp_atomic,
            );
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&mut gtype_id as *mut gsize as *mut libc::c_void) != 0)
            as libc::c_int
    }) != 0
    {
        let mut new_type: GType = g_enum_register_static(
            g_intern_static_string(
                b"FpDeviceError\0" as *const u8 as *const libc::c_char,
            ),
            values.as_ptr(),
        );
        if 0 as libc::c_int != 0 {
            gtype_id = new_type;
        } else {};
        g_once_init_leave(&mut gtype_id as *mut gsize as *mut libc::c_void, new_type);
    }
    return gtype_id;
}
#[no_mangle]
pub unsafe extern "C" fn fp_finger_get_type() -> GType {
    static mut gtype_id: gsize = 0 as libc::c_int as gsize;
    static mut values: [GEnumValue; 14] = [
        {
            let mut init = _GEnumValue {
                value: FP_FINGER_UNKNOWN as libc::c_int,
                value_name: b"FP_FINGER_UNKNOWN\0" as *const u8 as *const libc::c_char,
                value_nick: b"unknown\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_FINGER_LEFT_THUMB as libc::c_int,
                value_name: b"FP_FINGER_LEFT_THUMB\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"left-thumb\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_FINGER_LEFT_INDEX as libc::c_int,
                value_name: b"FP_FINGER_LEFT_INDEX\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"left-index\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_FINGER_LEFT_MIDDLE as libc::c_int,
                value_name: b"FP_FINGER_LEFT_MIDDLE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"left-middle\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_FINGER_LEFT_RING as libc::c_int,
                value_name: b"FP_FINGER_LEFT_RING\0" as *const u8 as *const libc::c_char,
                value_nick: b"left-ring\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_FINGER_LEFT_LITTLE as libc::c_int,
                value_name: b"FP_FINGER_LEFT_LITTLE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"left-little\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_FINGER_RIGHT_THUMB as libc::c_int,
                value_name: b"FP_FINGER_RIGHT_THUMB\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"right-thumb\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_FINGER_RIGHT_INDEX as libc::c_int,
                value_name: b"FP_FINGER_RIGHT_INDEX\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"right-index\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_FINGER_RIGHT_MIDDLE as libc::c_int,
                value_name: b"FP_FINGER_RIGHT_MIDDLE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"right-middle\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_FINGER_RIGHT_RING as libc::c_int,
                value_name: b"FP_FINGER_RIGHT_RING\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"right-ring\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_FINGER_RIGHT_LITTLE as libc::c_int,
                value_name: b"FP_FINGER_RIGHT_LITTLE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"right-little\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_FINGER_FIRST as libc::c_int,
                value_name: b"FP_FINGER_FIRST\0" as *const u8 as *const libc::c_char,
                value_nick: b"first\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_FINGER_LAST as libc::c_int,
                value_name: b"FP_FINGER_LAST\0" as *const u8 as *const libc::c_char,
                value_nick: b"last\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: 0 as libc::c_int,
                value_name: 0 as *const gchar,
                value_nick: 0 as *const gchar,
            };
            init
        },
    ];
    if ({
        if 0 as libc::c_int != 0 {} else {};
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &mut gtype_id as *mut gsize;
            *(&mut gapg_temp_newval
                as *mut gsize) = ::core::intrinsics::atomic_load_seqcst(
                gapg_temp_atomic,
            );
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&mut gtype_id as *mut gsize as *mut libc::c_void) != 0)
            as libc::c_int
    }) != 0
    {
        let mut new_type: GType = g_enum_register_static(
            g_intern_static_string(b"FpFinger\0" as *const u8 as *const libc::c_char),
            values.as_ptr(),
        );
        if 0 as libc::c_int != 0 {
            gtype_id = new_type;
        } else {};
        g_once_init_leave(&mut gtype_id as *mut gsize as *mut libc::c_void, new_type);
    }
    return gtype_id;
}
#[no_mangle]
pub unsafe extern "C" fn fp_finger_status_flags_get_type() -> GType {
    static mut gtype_id: gsize = 0 as libc::c_int as gsize;
    static mut values: [GFlagsValue; 4] = [
        {
            let mut init = _GFlagsValue {
                value: FP_FINGER_STATUS_NONE as libc::c_int as guint,
                value_name: b"FP_FINGER_STATUS_NONE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"none\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: FP_FINGER_STATUS_NEEDED as libc::c_int as guint,
                value_name: b"FP_FINGER_STATUS_NEEDED\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"needed\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: FP_FINGER_STATUS_PRESENT as libc::c_int as guint,
                value_name: b"FP_FINGER_STATUS_PRESENT\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"present\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: 0 as libc::c_int as guint,
                value_name: 0 as *const gchar,
                value_nick: 0 as *const gchar,
            };
            init
        },
    ];
    if ({
        if 0 as libc::c_int != 0 {} else {};
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &mut gtype_id as *mut gsize;
            *(&mut gapg_temp_newval
                as *mut gsize) = ::core::intrinsics::atomic_load_seqcst(
                gapg_temp_atomic,
            );
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&mut gtype_id as *mut gsize as *mut libc::c_void) != 0)
            as libc::c_int
    }) != 0
    {
        let mut new_type: GType = g_flags_register_static(
            g_intern_static_string(
                b"FpFingerStatusFlags\0" as *const u8 as *const libc::c_char,
            ),
            values.as_ptr(),
        );
        if 0 as libc::c_int != 0 {
            gtype_id = new_type;
        } else {};
        g_once_init_leave(&mut gtype_id as *mut gsize as *mut libc::c_void, new_type);
    }
    return gtype_id;
}
