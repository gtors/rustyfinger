use ::libc;
extern "C" {
    pub type _GData;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const gchar,
        class_size: guint,
        class_init: GClassInitFunc,
        instance_size: guint,
        instance_init: GInstanceInitFunc,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_class_adjust_private_offset(
        g_class: gpointer,
        private_size_or_offset: *mut gint,
    );
    fn g_type_check_class_cast(
        g_class: *mut GTypeClass,
        is_a_type: GType,
    ) -> *mut GTypeClass;
    fn fpi_device_aes3k_get_type() -> GType;
    fn fp_device_get_type() -> GType;
    fn fp_image_device_get_type() -> GType;
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
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
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
pub type GInstanceInitFunc = Option::<
    unsafe extern "C" fn(*mut GTypeInstance, gpointer) -> (),
>;
pub type GClassInitFunc = Option::<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GTypeFlags = libc::c_uint;
pub const G_TYPE_FLAG_FINAL: GTypeFlags = 64;
pub const G_TYPE_FLAG_VALUE_ABSTRACT: GTypeFlags = 32;
pub const G_TYPE_FLAG_ABSTRACT: GTypeFlags = 16;
pub const G_TYPE_FLAG_NONE: GTypeFlags = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpImageDevice {
    pub parent_instance: FpDevice,
}
pub type FpImageDevice = _FpImageDevice;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpImageDeviceClass {
    pub parent_class: FpDeviceClass,
    pub bz3_threshold: gint,
    pub img_width: gint,
    pub img_height: gint,
    pub img_open: Option::<unsafe extern "C" fn(*mut FpImageDevice) -> ()>,
    pub img_close: Option::<unsafe extern "C" fn(*mut FpImageDevice) -> ()>,
    pub activate: Option::<unsafe extern "C" fn(*mut FpImageDevice) -> ()>,
    pub change_state: Option::<
        unsafe extern "C" fn(*mut FpImageDevice, FpiImageDeviceState) -> (),
    >,
    pub deactivate: Option::<unsafe extern "C" fn(*mut FpImageDevice) -> ()>,
}
pub type FpiImageDeviceState = libc::c_uint;
pub const FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_OFF: FpiImageDeviceState = 6;
pub const FPI_IMAGE_DEVICE_STATE_CAPTURE: FpiImageDeviceState = 5;
pub const FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_ON: FpiImageDeviceState = 4;
pub const FPI_IMAGE_DEVICE_STATE_IDLE: FpiImageDeviceState = 3;
pub const FPI_IMAGE_DEVICE_STATE_DEACTIVATING: FpiImageDeviceState = 2;
pub const FPI_IMAGE_DEVICE_STATE_ACTIVATING: FpiImageDeviceState = 1;
pub const FPI_IMAGE_DEVICE_STATE_INACTIVE: FpiImageDeviceState = 0;
pub type FpImageDeviceClass = _FpImageDeviceClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes_regwrite {
    pub reg: libc::c_uchar,
    pub value: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceAes3k {
    pub parent_instance: FpImageDevice,
}
pub type FpiDeviceAes3k = _FpiDeviceAes3k;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceAes3kClass {
    pub parent: FpImageDeviceClass,
    pub frame_width: gsize,
    pub frame_size: gsize,
    pub frame_number: gsize,
    pub enlarge_factor: gsize,
    pub data_buflen: gsize,
    pub init_reqs: *mut aes_regwrite,
    pub init_reqs_len: gsize,
    pub capture_reqs: *mut aes_regwrite,
    pub capture_reqs_len: gsize,
}
pub type FpiDeviceAes3kClass = _FpiDeviceAes3kClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceAes4000 {
    pub parent: FpiDeviceAes3k,
}
pub type FpiDeviceAes4000 = _FpiDeviceAes4000;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiDeviceAes4000Class {
    pub parent_class: FpiDeviceAes3kClass,
}
#[inline]
unsafe extern "C" fn FPI_DEVICE_AES3K_CLASS(
    mut ptr: gpointer,
) -> *mut FpiDeviceAes3kClass {
    return g_type_check_class_cast(ptr as *mut GTypeClass, fpi_device_aes3k_get_type())
        as *mut libc::c_void as *mut FpiDeviceAes3kClass;
}
#[inline]
unsafe extern "C" fn FP_DEVICE_CLASS(mut ptr: gpointer) -> *mut FpDeviceClass {
    return g_type_check_class_cast(ptr as *mut GTypeClass, fp_device_get_type())
        as *mut libc::c_void as *mut FpDeviceClass;
}
#[inline]
unsafe extern "C" fn FP_IMAGE_DEVICE_CLASS(
    mut ptr: gpointer,
) -> *mut FpImageDeviceClass {
    return g_type_check_class_cast(ptr as *mut GTypeClass, fp_image_device_get_type())
        as *mut libc::c_void as *mut FpImageDeviceClass;
}
static mut init_reqs: [aes_regwrite; 50] = [
    {
        let mut init = aes_regwrite {
            reg: 0x80 as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x80 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x81 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x80 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x80 as libc::c_int as libc::c_uchar,
            value: 0x2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x80 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x80 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x80 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x81 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x80 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x81 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x82 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x83 as libc::c_int as libc::c_uchar,
            value: 0x13 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x84 as libc::c_int as libc::c_uchar,
            value: 0x7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x85 as libc::c_int as libc::c_uchar,
            value: 0x3d as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x86 as libc::c_int as libc::c_uchar,
            value: 0x3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x87 as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x88 as libc::c_int as libc::c_uchar,
            value: 0x2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x89 as libc::c_int as libc::c_uchar,
            value: 0x2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x8a as libc::c_int as libc::c_uchar,
            value: 0x33 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x8b as libc::c_int as libc::c_uchar,
            value: 0x33 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x8c as libc::c_int as libc::c_uchar,
            value: 0xf as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x8d as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x8e as libc::c_int as libc::c_uchar,
            value: 0x23 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x8f as libc::c_int as libc::c_uchar,
            value: 0x7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x90 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x91 as libc::c_int as libc::c_uchar,
            value: 0x1c as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x92 as libc::c_int as libc::c_uchar,
            value: 0x8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x93 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x94 as libc::c_int as libc::c_uchar,
            value: 0x5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x95 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x96 as libc::c_int as libc::c_uchar,
            value: 0x18 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x97 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x98 as libc::c_int as libc::c_uchar,
            value: 0x28 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x99 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x9a as libc::c_int as libc::c_uchar,
            value: 0xb as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x9b as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x9c as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x9d as libc::c_int as libc::c_uchar,
            value: 0x9 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x9e as libc::c_int as libc::c_uchar,
            value: 0x53 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x9f as libc::c_int as libc::c_uchar,
            value: 0x6b as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut capture_reqs: [aes_regwrite; 6] = [
    {
        let mut init = aes_regwrite {
            reg: 0x80 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x81 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x81 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x81 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn fpi_device_aes4000_get_type() -> GType {
    static mut static_g_define_type_id: gsize = 0 as libc::c_int as gsize;
    if ({
        if 0 as libc::c_int != 0 {} else {};
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &mut static_g_define_type_id
                as *mut gsize;
            *(&mut gapg_temp_newval
                as *mut gsize) = ::core::intrinsics::atomic_load_seqcst(
                gapg_temp_atomic,
            );
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(
                &mut static_g_define_type_id as *mut gsize as *mut libc::c_void,
            ) != 0) as libc::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = fpi_device_aes4000_get_type_once();
        if 0 as libc::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {};
        g_once_init_leave(
            &mut static_g_define_type_id as *mut gsize as *mut libc::c_void,
            g_define_type_id,
        );
    }
    return static_g_define_type_id;
}
static mut fpi_device_aes4000_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut FpiDeviceAes4000_private_offset: gint = 0;
unsafe extern "C" fn fpi_device_aes4000_class_intern_init(mut klass: gpointer) {
    fpi_device_aes4000_parent_class = g_type_class_peek_parent(klass);
    if FpiDeviceAes4000_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpiDeviceAes4000_private_offset);
    }
    fpi_device_aes4000_class_init(klass as *mut FpiDeviceAes4000Class);
}
#[inline(never)]
unsafe extern "C" fn fpi_device_aes4000_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fpi_device_aes3k_get_type(),
        g_intern_static_string(
            b"FpiDeviceAes4000\0" as *const u8 as *const libc::c_char,
        ),
        ::core::mem::size_of::<FpiDeviceAes4000Class>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_aes4000_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpiDeviceAes4000>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpiDeviceAes4000) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_aes4000_init
                        as unsafe extern "C" fn(*mut FpiDeviceAes4000) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut id_table: [FpIdEntry; 2] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x8ff as libc::c_int as guint,
                        vid: 0x5501 as libc::c_int as guint,
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
                        pid: 0 as libc::c_int as guint,
                        vid: 0 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: 0 as libc::c_int as guint64,
        };
        init
    },
];
unsafe extern "C" fn fpi_device_aes4000_init(mut self_0: *mut FpiDeviceAes4000) {}
unsafe extern "C" fn fpi_device_aes4000_class_init(
    mut klass: *mut FpiDeviceAes4000Class,
) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    let mut img_class: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_CLASS(
        klass as gpointer,
    );
    let mut aes_class: *mut FpiDeviceAes3kClass = FPI_DEVICE_AES3K_CLASS(
        klass as gpointer,
    );
    (*dev_class).id = b"aes4000\0" as *const u8 as *const libc::c_char;
    (*dev_class).full_name = b"AuthenTec AES4000\0" as *const u8 as *const libc::c_char;
    (*dev_class).id_table = id_table.as_ptr();
    (*img_class).img_height = 96 as libc::c_int * 3 as libc::c_int;
    (*img_class).img_width = 96 as libc::c_int * 3 as libc::c_int;
    (*aes_class).data_buflen = 0x1259 as libc::c_int as gsize;
    (*aes_class).frame_width = 96 as libc::c_int as gsize;
    (*aes_class)
        .frame_size = (96 as libc::c_int * 16 as libc::c_int / 2 as libc::c_int)
        as gsize;
    (*aes_class).frame_number = (96 as libc::c_int / 16 as libc::c_int) as gsize;
    (*aes_class).enlarge_factor = 3 as libc::c_int as gsize;
    (*aes_class).init_reqs = init_reqs.as_mut_ptr();
    (*aes_class)
        .init_reqs_len = (::core::mem::size_of::<[aes_regwrite; 50]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<aes_regwrite>() as libc::c_ulong);
    (*aes_class).capture_reqs = capture_reqs.as_mut_ptr();
    (*aes_class)
        .capture_reqs_len = (::core::mem::size_of::<[aes_regwrite; 6]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<aes_regwrite>() as libc::c_ulong);
}
