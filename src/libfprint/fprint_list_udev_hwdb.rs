use ::libc;
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    fn g_array_sort(array: *mut GArray, compare_func: GCompareFunc);
    fn g_path_get_basename(file_name: *const gchar) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_insert(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn g_print(format: *const gchar, _: ...);
    fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    fn g_strcmp0(str1: *const libc::c_char, str2: *const libc::c_char) -> libc::c_int;
    fn g_type_class_ref(type_0: GType) -> gpointer;
    fn fpi_get_driver_types() -> *mut GArray;
}
pub type gint32 = libc::c_int;
pub type gint64 = libc::c_long;
pub type guint64 = libc::c_ulong;
pub type gsize = libc::c_ulong;
pub type gchar = libc::c_char;
pub type glong = libc::c_long;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type gulong = libc::c_ulong;
pub type guint = libc::c_uint;
pub type gfloat = libc::c_float;
pub type gdouble = libc::c_double;
pub type gpointer = *mut libc::c_void;
pub type gconstpointer = *const libc::c_void;
pub type GCompareFunc = Option::<
    unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
>;
pub type GEqualFunc = Option::<
    unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
>;
pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option::<unsafe extern "C" fn(gconstpointer) -> guint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
pub type GData = _GData;
pub type GHashTable = _GHashTable;
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
pub type GArray_autoptr = *mut GArray;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub v_int: gint,
    pub v_uint: guint,
    pub v_long: glong,
    pub v_ulong: gulong,
    pub v_int64: gint64,
    pub v_uint64: guint64,
    pub v_float: gfloat,
    pub v_double: gdouble,
    pub v_pointer: gpointer,
}
pub type GValue = _GValue;
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
pub type GParamFlags = libc::c_int;
pub const G_PARAM_DEPRECATED: GParamFlags = -2147483648;
pub const G_PARAM_EXPLICIT_NOTIFY: GParamFlags = 1073741824;
pub const G_PARAM_STATIC_BLURB: GParamFlags = 128;
pub const G_PARAM_STATIC_NICK: GParamFlags = 64;
pub const G_PARAM_PRIVATE: GParamFlags = 32;
pub const G_PARAM_STATIC_NAME: GParamFlags = 32;
pub const G_PARAM_LAX_VALIDATION: GParamFlags = 16;
pub const G_PARAM_CONSTRUCT_ONLY: GParamFlags = 8;
pub const G_PARAM_CONSTRUCT: GParamFlags = 4;
pub const G_PARAM_READWRITE: GParamFlags = 3;
pub const G_PARAM_WRITABLE: GParamFlags = 2;
pub const G_PARAM_READABLE: GParamFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpec {
    pub g_type_instance: GTypeInstance,
    pub name: *const gchar,
    pub flags: GParamFlags,
    pub value_type: GType,
    pub owner_type: GType,
    pub _nick: *mut gchar,
    pub _blurb: *mut gchar,
    pub qdata: *mut GData,
    pub ref_count: guint,
    pub param_id: guint,
}
pub type GParamSpec = _GParamSpec;
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
pub struct _GObjectClass {
    pub g_type_class: GTypeClass,
    pub construct_properties: *mut GSList,
    pub constructor: Option::<
        unsafe extern "C" fn(GType, guint, *mut GObjectConstructParam) -> *mut GObject,
    >,
    pub set_property: Option::<
        unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    >,
    pub get_property: Option::<
        unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    >,
    pub dispose: Option::<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub finalize: Option::<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub dispatch_properties_changed: Option::<
        unsafe extern "C" fn(*mut GObject, guint, *mut *mut GParamSpec) -> (),
    >,
    pub notify: Option::<unsafe extern "C" fn(*mut GObject, *mut GParamSpec) -> ()>,
    pub constructed: Option::<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub flags: gsize,
    pub n_construct_properties: gsize,
    pub pspecs: gpointer,
    pub n_pspecs: gsize,
    pub pdummy: [gpointer; 3],
}
pub type GObjectConstructParam = _GObjectConstructParam;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObjectConstructParam {
    pub pspec: *mut GParamSpec,
    pub value: *mut GValue,
}
pub type GObjectClass = _GObjectClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUsbDevice {
    pub parent_instance: GObject,
}
pub type GUsbDevice = _GUsbDevice;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpDevice {
    pub parent_instance: GObject,
}
pub type FpDevice = _FpDevice;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpDeviceClass {
    pub parent_class: GObjectClass,
    pub id: *const gchar,
    pub full_name: *const gchar,
    pub type_0: FpDeviceType,
    pub id_table: *const FpIdEntry,
    pub features: FpDeviceFeature,
    pub nr_enroll_stages: gint,
    pub scan_type: FpScanType,
    pub temp_hot_seconds: gint32,
    pub temp_cold_seconds: gint32,
    pub usb_discover: Option::<unsafe extern "C" fn(*mut GUsbDevice) -> gint>,
    pub probe: Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
    pub open: Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
    pub close: Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
    pub enroll: Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
    pub verify: Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
    pub identify: Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
    pub capture: Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
    pub list: Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
    pub delete: Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
    pub clear_storage: Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
    pub cancel: Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
    pub suspend: Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
    pub resume: Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
}
pub type FpScanType = libc::c_uint;
pub const FP_SCAN_TYPE_PRESS: FpScanType = 1;
pub const FP_SCAN_TYPE_SWIPE: FpScanType = 0;
pub type FpDeviceFeature = libc::c_uint;
pub const FP_DEVICE_FEATURE_UPDATE_PRINT: FpDeviceFeature = 512;
pub const FP_DEVICE_FEATURE_ALWAYS_ON: FpDeviceFeature = 256;
pub const FP_DEVICE_FEATURE_DUPLICATES_CHECK: FpDeviceFeature = 128;
pub const FP_DEVICE_FEATURE_STORAGE_CLEAR: FpDeviceFeature = 64;
pub const FP_DEVICE_FEATURE_STORAGE_DELETE: FpDeviceFeature = 32;
pub const FP_DEVICE_FEATURE_STORAGE_LIST: FpDeviceFeature = 16;
pub const FP_DEVICE_FEATURE_STORAGE: FpDeviceFeature = 8;
pub const FP_DEVICE_FEATURE_VERIFY: FpDeviceFeature = 4;
pub const FP_DEVICE_FEATURE_IDENTIFY: FpDeviceFeature = 2;
pub const FP_DEVICE_FEATURE_CAPTURE: FpDeviceFeature = 1;
pub const FP_DEVICE_FEATURE_NONE: FpDeviceFeature = 0;
pub type FpIdEntry = _FpIdEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpIdEntry {
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub driver_data: guint64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub virtual_envvar: *const gchar,
    pub c2rust_unnamed_0: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub udev_types: FpiDeviceUdevSubtypeFlags,
    pub spi_acpi_id: *const gchar,
    pub hid_id: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub pid: guint,
    pub vid: guint,
}
pub type FpiDeviceUdevSubtypeFlags = libc::c_uint;
pub const FPI_DEVICE_UDEV_SUBTYPE_HIDRAW: FpiDeviceUdevSubtypeFlags = 2;
pub const FPI_DEVICE_UDEV_SUBTYPE_SPIDEV: FpiDeviceUdevSubtypeFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub pid: guint,
    pub vid: guint,
}
pub type FpDeviceType = libc::c_uint;
pub const FP_DEVICE_TYPE_USB: FpDeviceType = 2;
pub const FP_DEVICE_TYPE_UDEV: FpDeviceType = 1;
pub const FP_DEVICE_TYPE_VIRTUAL: FpDeviceType = 0;
pub type FpDeviceClass = _FpDeviceClass;
pub type FpDeviceClass_autoptr = *mut FpDeviceClass;
static mut whitelist_id_table: [FpIdEntry; 97] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x730b as libc::c_int as guint,
                        vid: 0x4e8 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x36b as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xc00 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xc4c as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xc57 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xc5e as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xc5a as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xc70 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xc72 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x2706 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x3057 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x3104 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x310d as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x81 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x88 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x8a as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x9a as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x9b as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xa2 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xa8 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xb7 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xbb as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xbe as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xc4 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xcb as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xc9 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xd8 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xda as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xdc as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xe4 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xe7 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xe9 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xfd as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5801 as libc::c_int as guint,
                        vid: 0xa5c as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5805 as libc::c_int as guint,
                        vid: 0xa5c as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5834 as libc::c_int as guint,
                        vid: 0xa5c as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5840 as libc::c_int as guint,
                        vid: 0xa5c as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5841 as libc::c_int as guint,
                        vid: 0xa5c as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5842 as libc::c_int as guint,
                        vid: 0xa5c as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5843 as libc::c_int as guint,
                        vid: 0xa5c as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5844 as libc::c_int as guint,
                        vid: 0xa5c as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5845 as libc::c_int as guint,
                        vid: 0xa5c as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5812 as libc::c_int as guint,
                        vid: 0xbda as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x7 as libc::c_int as guint,
                        vid: 0x10a5 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x9200 as libc::c_int as guint,
                        vid: 0x10a5 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x9800 as libc::c_int as guint,
                        vid: 0x10a5 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x9545 as libc::c_int as guint,
                        vid: 0x1188 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x7 as libc::c_int as guint,
                        vid: 0x138a as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x3a as libc::c_int as guint,
                        vid: 0x138a as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x3c as libc::c_int as guint,
                        vid: 0x138a as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x3d as libc::c_int as guint,
                        vid: 0x138a as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x3f as libc::c_int as guint,
                        vid: 0x138a as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x90 as libc::c_int as guint,
                        vid: 0x138a as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x92 as libc::c_int as guint,
                        vid: 0x138a as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x94 as libc::c_int as guint,
                        vid: 0x138a as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x97 as libc::c_int as guint,
                        vid: 0x138a as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x9d as libc::c_int as guint,
                        vid: 0x138a as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xab as libc::c_int as guint,
                        vid: 0x138a as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x1002 as libc::c_int as guint,
                        vid: 0x147e as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x88 as libc::c_int as guint,
                        vid: 0x1491 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x1027 as libc::c_int as guint,
                        vid: 0x16d1 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x300 as libc::c_int as guint,
                        vid: 0x1c7a as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x575 as libc::c_int as guint,
                        vid: 0x1c7a as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x576 as libc::c_int as guint,
                        vid: 0x1c7a as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5042 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5110 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5117 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5120 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5125 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5201 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x521d as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5301 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x530c as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x532d as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5335 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x533c as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5381 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5385 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x538c as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x538d as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5395 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5503 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x550a as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x550c as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5584 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x55a2 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x55a4 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x55b4 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5740 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5e0a as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x581a as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x9338 as libc::c_int as guint,
                        vid: 0x2808 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x93a9 as libc::c_int as guint,
                        vid: 0x2808 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x2020 as libc::c_int as guint,
                        vid: 0x298d as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x2033 as libc::c_int as guint,
                        vid: 0x298d as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x930 as libc::c_int as guint,
                        vid: 0x3538 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0,
                        vid: 0 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
];
static mut blacklist_id_table: [FpIdEntry; 3] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x2016 as libc::c_int as guint,
                        vid: 0x483 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xbb as libc::c_int as guint,
                        vid: 0x45e as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0,
                        vid: 0 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
];
static mut whitelist: FpDeviceClass = unsafe {
    {
        let mut init = _FpDeviceClass {
            parent_class: GObjectClass {
                g_type_class: GTypeClass { g_type: 0 },
                construct_properties: 0 as *const GSList as *mut GSList,
                constructor: None,
                set_property: None,
                get_property: None,
                dispose: None,
                finalize: None,
                dispatch_properties_changed: None,
                notify: None,
                constructed: None,
                flags: 0,
                n_construct_properties: 0,
                pspecs: 0 as *const libc::c_void as *mut libc::c_void,
                n_pspecs: 0,
                pdummy: [0 as *const libc::c_void as *mut libc::c_void; 3],
            },
            id: b"whitelist\0" as *const u8 as *const libc::c_char,
            full_name: b"Hardcoded whitelist\0" as *const u8 as *const libc::c_char,
            type_0: FP_DEVICE_TYPE_USB,
            id_table: whitelist_id_table.as_ptr(),
            features: FP_DEVICE_FEATURE_NONE,
            nr_enroll_stages: 0,
            scan_type: FP_SCAN_TYPE_SWIPE,
            temp_hot_seconds: 0,
            temp_cold_seconds: 0,
            usb_discover: None,
            probe: None,
            open: None,
            close: None,
            enroll: None,
            verify: None,
            identify: None,
            capture: None,
            list: None,
            delete: None,
            clear_storage: None,
            cancel: None,
            suspend: None,
            resume: None,
        };
        init
    }
};
#[no_mangle]
pub static mut printed: *mut GHashTable = 0 as *const GHashTable as *mut GHashTable;
unsafe extern "C" fn print_driver(mut cls: *const FpDeviceClass) {
    let mut entry: *const FpIdEntry = 0 as *const FpIdEntry;
    let mut num_printed: gint = 0 as libc::c_int;
    if (*cls).type_0 as libc::c_uint != FP_DEVICE_TYPE_USB as libc::c_int as libc::c_uint
    {
        return;
    }
    entry = (*cls).id_table;
    while (*entry).c2rust_unnamed.c2rust_unnamed.vid != 0 as libc::c_int as libc::c_uint
    {
        let mut bl_entry: *const FpIdEntry = 0 as *const FpIdEntry;
        let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
        bl_entry = blacklist_id_table.as_ptr();
        while (*bl_entry).c2rust_unnamed.c2rust_unnamed.vid
            != 0 as libc::c_int as libc::c_uint
        {
            if (*entry).c2rust_unnamed.c2rust_unnamed.vid
                == (*bl_entry).c2rust_unnamed.c2rust_unnamed.vid
                && (*entry).c2rust_unnamed.c2rust_unnamed.pid
                    == (*bl_entry).c2rust_unnamed.c2rust_unnamed.pid
            {
                break;
            }
            bl_entry = bl_entry.offset(1);
        }
        if !((*bl_entry).c2rust_unnamed.c2rust_unnamed.vid
            != 0 as libc::c_int as libc::c_uint)
        {
            key = g_strdup_printf(
                b"%04x:%04x\0" as *const u8 as *const libc::c_char,
                (*entry).c2rust_unnamed.c2rust_unnamed.vid,
                (*entry).c2rust_unnamed.c2rust_unnamed.pid,
            );
            if !(g_hash_table_lookup(printed, key as gconstpointer)).is_null() {
                if cls == &whitelist as *const FpDeviceClass {
                    g_log(
                        b"libfprint\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_WARNING,
                        b"%s implemented by driver %s\0" as *const u8
                            as *const libc::c_char,
                        key,
                        g_hash_table_lookup(printed, key as gconstpointer)
                            as *const libc::c_char,
                    );
                }
                g_free(key as gpointer);
            } else {
                g_hash_table_insert(
                    printed,
                    key as gpointer,
                    (*cls).id as *mut libc::c_void,
                );
                if num_printed == 0 as libc::c_int {
                    if cls != &whitelist as *const FpDeviceClass {
                        g_print(
                            b"\n# Supported by libfprint driver %s\n\0" as *const u8
                                as *const libc::c_char,
                            (*cls).id,
                        );
                    } else {
                        g_print(
                            b"\n# Known unsupported devices\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
                g_print(
                    b"usb:v%04Xp%04X*\n\0" as *const u8 as *const libc::c_char,
                    (*entry).c2rust_unnamed.c2rust_unnamed.vid,
                    (*entry).c2rust_unnamed.c2rust_unnamed.pid,
                );
                num_printed += 1;
            }
        }
        entry = entry.offset(1);
    }
    if num_printed > 0 as libc::c_int {
        g_print(b" ID_AUTOSUSPEND=1\n\0" as *const u8 as *const libc::c_char);
        g_print(b" ID_PERSIST=0\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn driver_compare(
    mut p1: gconstpointer,
    mut p2: gconstpointer,
) -> libc::c_int {
    let mut cls1: FpDeviceClass_autoptr = g_type_class_ref(*(p1 as *mut GType))
        as FpDeviceClass_autoptr;
    let mut cls2: FpDeviceClass_autoptr = g_type_class_ref(*(p2 as *mut GType))
        as FpDeviceClass_autoptr;
    return g_strcmp0((*cls1).id, (*cls2).id);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut drivers: GArray_autoptr = fpi_get_driver_types();
    let mut program_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: guint = 0;
    program_name = g_path_get_basename(*argv.offset(0 as libc::c_int as isize));
    g_print(
        b"# SPDX-License-Identifier: LGPL-2.1-or-later\n\0" as *const u8
            as *const libc::c_char,
    );
    g_print(
        b"# This file has been generated using %s with all drivers enabled\n\0"
            as *const u8 as *const libc::c_char,
        program_name,
    );
    printed = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(
            g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
        ),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        None,
    );
    g_array_sort(
        drivers,
        Some(
            driver_compare
                as unsafe extern "C" fn(gconstpointer, gconstpointer) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int as guint;
    while i < (*drivers).len {
        let mut driver: GType = *((*drivers).data as *mut libc::c_void as *mut GType)
            .offset(i as isize);
        let mut cls: FpDeviceClass_autoptr = g_type_class_ref(driver)
            as FpDeviceClass_autoptr;
        if !((*cls).type_0 as libc::c_uint
            != FP_DEVICE_TYPE_USB as libc::c_int as libc::c_uint)
        {
            print_driver(cls as *const FpDeviceClass);
        }
        i = i.wrapping_add(1);
    }
    print_driver(&whitelist);
    g_hash_table_destroy(printed);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
