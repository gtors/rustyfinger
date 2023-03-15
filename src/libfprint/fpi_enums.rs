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
pub const FPI_DEVICE_UDEV_SUBTYPE_HIDRAW: C2RustUnnamed_0 = 2;
pub const FPI_DEVICE_UDEV_SUBTYPE_SPIDEV: C2RustUnnamed_0 = 1;
pub const FPI_DEVICE_ACTION_CLEAR_STORAGE: C2RustUnnamed_3 = 10;
pub const FPI_DEVICE_ACTION_DELETE: C2RustUnnamed_3 = 9;
pub const FPI_DEVICE_ACTION_LIST: C2RustUnnamed_3 = 8;
pub const FPI_DEVICE_ACTION_CAPTURE: C2RustUnnamed_3 = 7;
pub const FPI_DEVICE_ACTION_IDENTIFY: C2RustUnnamed_3 = 6;
pub const FPI_DEVICE_ACTION_VERIFY: C2RustUnnamed_3 = 5;
pub const FPI_DEVICE_ACTION_ENROLL: C2RustUnnamed_3 = 4;
pub const FPI_DEVICE_ACTION_CLOSE: C2RustUnnamed_3 = 3;
pub const FPI_DEVICE_ACTION_OPEN: C2RustUnnamed_3 = 2;
pub const FPI_DEVICE_ACTION_PROBE: C2RustUnnamed_3 = 1;
pub const FPI_DEVICE_ACTION_NONE: C2RustUnnamed_3 = 0;
pub const FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_OFF: C2RustUnnamed_4 = 6;
pub const FPI_IMAGE_DEVICE_STATE_CAPTURE: C2RustUnnamed_4 = 5;
pub const FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_ON: C2RustUnnamed_4 = 4;
pub const FPI_IMAGE_DEVICE_STATE_IDLE: C2RustUnnamed_4 = 3;
pub const FPI_IMAGE_DEVICE_STATE_DEACTIVATING: C2RustUnnamed_4 = 2;
pub const FPI_IMAGE_DEVICE_STATE_ACTIVATING: C2RustUnnamed_4 = 1;
pub const FPI_IMAGE_DEVICE_STATE_INACTIVE: C2RustUnnamed_4 = 0;
pub const FPI_IMAGE_PARTIAL: C2RustUnnamed = 8;
pub const FPI_IMAGE_COLORS_INVERTED: C2RustUnnamed = 4;
pub const FPI_IMAGE_H_FLIPPED: C2RustUnnamed = 2;
pub const FPI_IMAGE_V_FLIPPED: C2RustUnnamed = 1;
pub const FPI_PRINT_NBIS: C2RustUnnamed_1 = 2;
pub const FPI_PRINT_RAW: C2RustUnnamed_1 = 1;
pub const FPI_PRINT_UNDEFINED: C2RustUnnamed_1 = 0;
pub const FPI_MATCH_SUCCESS: C2RustUnnamed_2 = 1;
pub const FPI_MATCH_FAIL: C2RustUnnamed_2 = 0;
pub const FPI_MATCH_ERROR: C2RustUnnamed_2 = -1;
pub const FP_TRANSFER_INTERRUPT: C2RustUnnamed_5 = 3;
pub const FP_TRANSFER_BULK: C2RustUnnamed_5 = 2;
pub const FP_TRANSFER_CONTROL: C2RustUnnamed_5 = 0;
pub const FP_TRANSFER_NONE: C2RustUnnamed_5 = -1;
pub type C2RustUnnamed = libc::c_uint;
pub type C2RustUnnamed_0 = libc::c_uint;
pub type C2RustUnnamed_1 = libc::c_uint;
pub type C2RustUnnamed_2 = libc::c_int;
pub type C2RustUnnamed_3 = libc::c_uint;
pub type C2RustUnnamed_4 = libc::c_uint;
pub type C2RustUnnamed_5 = libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn fpi_device_udev_subtype_flags_get_type() -> GType {
    static mut gtype_id: gsize = 0 as libc::c_int as gsize;
    static mut values: [GFlagsValue; 3] = [
        {
            let mut init = _GFlagsValue {
                value: FPI_DEVICE_UDEV_SUBTYPE_SPIDEV as libc::c_int as guint,
                value_name: b"FPI_DEVICE_UDEV_SUBTYPE_SPIDEV\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"spidev\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: FPI_DEVICE_UDEV_SUBTYPE_HIDRAW as libc::c_int as guint,
                value_name: b"FPI_DEVICE_UDEV_SUBTYPE_HIDRAW\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"hidraw\0" as *const u8 as *const libc::c_char,
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
                b"FpiDeviceUdevSubtypeFlags\0" as *const u8 as *const libc::c_char,
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
pub unsafe extern "C" fn fpi_device_action_get_type() -> GType {
    static mut gtype_id: gsize = 0 as libc::c_int as gsize;
    static mut values: [GEnumValue; 12] = [
        {
            let mut init = _GEnumValue {
                value: FPI_DEVICE_ACTION_NONE as libc::c_int,
                value_name: b"FPI_DEVICE_ACTION_NONE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"none\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_DEVICE_ACTION_PROBE as libc::c_int,
                value_name: b"FPI_DEVICE_ACTION_PROBE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"probe\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_DEVICE_ACTION_OPEN as libc::c_int,
                value_name: b"FPI_DEVICE_ACTION_OPEN\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"open\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_DEVICE_ACTION_CLOSE as libc::c_int,
                value_name: b"FPI_DEVICE_ACTION_CLOSE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"close\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_DEVICE_ACTION_ENROLL as libc::c_int,
                value_name: b"FPI_DEVICE_ACTION_ENROLL\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"enroll\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_DEVICE_ACTION_VERIFY as libc::c_int,
                value_name: b"FPI_DEVICE_ACTION_VERIFY\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"verify\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_DEVICE_ACTION_IDENTIFY as libc::c_int,
                value_name: b"FPI_DEVICE_ACTION_IDENTIFY\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"identify\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_DEVICE_ACTION_CAPTURE as libc::c_int,
                value_name: b"FPI_DEVICE_ACTION_CAPTURE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"capture\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_DEVICE_ACTION_LIST as libc::c_int,
                value_name: b"FPI_DEVICE_ACTION_LIST\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"list\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_DEVICE_ACTION_DELETE as libc::c_int,
                value_name: b"FPI_DEVICE_ACTION_DELETE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"delete\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_DEVICE_ACTION_CLEAR_STORAGE as libc::c_int,
                value_name: b"FPI_DEVICE_ACTION_CLEAR_STORAGE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"clear-storage\0" as *const u8 as *const libc::c_char,
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
                b"FpiDeviceAction\0" as *const u8 as *const libc::c_char,
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
pub unsafe extern "C" fn fpi_image_device_state_get_type() -> GType {
    static mut gtype_id: gsize = 0 as libc::c_int as gsize;
    static mut values: [GEnumValue; 8] = [
        {
            let mut init = _GEnumValue {
                value: FPI_IMAGE_DEVICE_STATE_INACTIVE as libc::c_int,
                value_name: b"FPI_IMAGE_DEVICE_STATE_INACTIVE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"inactive\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_IMAGE_DEVICE_STATE_ACTIVATING as libc::c_int,
                value_name: b"FPI_IMAGE_DEVICE_STATE_ACTIVATING\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"activating\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_IMAGE_DEVICE_STATE_DEACTIVATING as libc::c_int,
                value_name: b"FPI_IMAGE_DEVICE_STATE_DEACTIVATING\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"deactivating\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_IMAGE_DEVICE_STATE_IDLE as libc::c_int,
                value_name: b"FPI_IMAGE_DEVICE_STATE_IDLE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"idle\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_ON as libc::c_int,
                value_name: b"FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_ON\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"await-finger-on\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_IMAGE_DEVICE_STATE_CAPTURE as libc::c_int,
                value_name: b"FPI_IMAGE_DEVICE_STATE_CAPTURE\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"capture\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_OFF as libc::c_int,
                value_name: b"FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_OFF\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"await-finger-off\0" as *const u8 as *const libc::c_char,
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
                b"FpiImageDeviceState\0" as *const u8 as *const libc::c_char,
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
pub unsafe extern "C" fn fpi_image_flags_get_type() -> GType {
    static mut gtype_id: gsize = 0 as libc::c_int as gsize;
    static mut values: [GFlagsValue; 5] = [
        {
            let mut init = _GFlagsValue {
                value: FPI_IMAGE_V_FLIPPED as libc::c_int as guint,
                value_name: b"FPI_IMAGE_V_FLIPPED\0" as *const u8 as *const libc::c_char,
                value_nick: b"v-flipped\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: FPI_IMAGE_H_FLIPPED as libc::c_int as guint,
                value_name: b"FPI_IMAGE_H_FLIPPED\0" as *const u8 as *const libc::c_char,
                value_nick: b"h-flipped\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: FPI_IMAGE_COLORS_INVERTED as libc::c_int as guint,
                value_name: b"FPI_IMAGE_COLORS_INVERTED\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"colors-inverted\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GFlagsValue {
                value: FPI_IMAGE_PARTIAL as libc::c_int as guint,
                value_name: b"FPI_IMAGE_PARTIAL\0" as *const u8 as *const libc::c_char,
                value_nick: b"partial\0" as *const u8 as *const libc::c_char,
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
                b"FpiImageFlags\0" as *const u8 as *const libc::c_char,
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
pub unsafe extern "C" fn fpi_print_type_get_type() -> GType {
    static mut gtype_id: gsize = 0 as libc::c_int as gsize;
    static mut values: [GEnumValue; 4] = [
        {
            let mut init = _GEnumValue {
                value: FPI_PRINT_UNDEFINED as libc::c_int,
                value_name: b"FPI_PRINT_UNDEFINED\0" as *const u8 as *const libc::c_char,
                value_nick: b"undefined\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_PRINT_RAW as libc::c_int,
                value_name: b"FPI_PRINT_RAW\0" as *const u8 as *const libc::c_char,
                value_nick: b"raw\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_PRINT_NBIS as libc::c_int,
                value_name: b"FPI_PRINT_NBIS\0" as *const u8 as *const libc::c_char,
                value_nick: b"nbis\0" as *const u8 as *const libc::c_char,
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
                b"FpiPrintType\0" as *const u8 as *const libc::c_char,
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
pub unsafe extern "C" fn fpi_match_result_get_type() -> GType {
    static mut gtype_id: gsize = 0 as libc::c_int as gsize;
    static mut values: [GEnumValue; 4] = [
        {
            let mut init = _GEnumValue {
                value: FPI_MATCH_ERROR as libc::c_int,
                value_name: b"FPI_MATCH_ERROR\0" as *const u8 as *const libc::c_char,
                value_nick: b"error\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_MATCH_FAIL as libc::c_int,
                value_name: b"FPI_MATCH_FAIL\0" as *const u8 as *const libc::c_char,
                value_nick: b"fail\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FPI_MATCH_SUCCESS as libc::c_int,
                value_name: b"FPI_MATCH_SUCCESS\0" as *const u8 as *const libc::c_char,
                value_nick: b"success\0" as *const u8 as *const libc::c_char,
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
                b"FpiMatchResult\0" as *const u8 as *const libc::c_char,
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
pub unsafe extern "C" fn fpi_transfer_type_get_type() -> GType {
    static mut gtype_id: gsize = 0 as libc::c_int as gsize;
    static mut values: [GEnumValue; 5] = [
        {
            let mut init = _GEnumValue {
                value: FP_TRANSFER_NONE as libc::c_int,
                value_name: b"FP_TRANSFER_NONE\0" as *const u8 as *const libc::c_char,
                value_nick: b"none\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_TRANSFER_CONTROL as libc::c_int,
                value_name: b"FP_TRANSFER_CONTROL\0" as *const u8 as *const libc::c_char,
                value_nick: b"control\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_TRANSFER_BULK as libc::c_int,
                value_name: b"FP_TRANSFER_BULK\0" as *const u8 as *const libc::c_char,
                value_nick: b"bulk\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = _GEnumValue {
                value: FP_TRANSFER_INTERRUPT as libc::c_int,
                value_name: b"FP_TRANSFER_INTERRUPT\0" as *const u8
                    as *const libc::c_char,
                value_nick: b"interrupt\0" as *const u8 as *const libc::c_char,
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
                b"FpiTransferType\0" as *const u8 as *const libc::c_char,
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
