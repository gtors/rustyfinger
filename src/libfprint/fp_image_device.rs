use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _FpImage;
    pub type _FpPrint;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
    fn g_signal_new(
        signal_name: *const gchar,
        itype: GType,
        signal_flags: GSignalFlags,
        class_offset: guint,
        accumulator: GSignalAccumulator,
        accu_data: gpointer,
        c_marshaller: GSignalCMarshaller,
        return_type: GType,
        n_params: guint,
        _: ...
    ) -> guint;
    fn g_object_class_install_properties(
        oclass: *mut GObjectClass,
        n_pspecs: guint,
        pspecs: *mut *mut GParamSpec,
    );
    fn g_object_get(object: gpointer, first_property_name: *const gchar, _: ...);
    fn g_value_set_enum(value: *mut GValue, v_enum: gint);
    fn g_param_spec_enum(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        enum_type: GType,
        default_value: gint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_type_check_class_cast(
        g_class: *mut GTypeClass,
        is_a_type: GType,
    ) -> *mut GTypeClass;
    fn g_type_check_instance_cast(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> *mut GTypeInstance;
    fn g_type_class_adjust_private_offset(
        g_class: gpointer,
        private_size_or_offset: *mut gint,
    );
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const gchar,
        class_size: guint,
        class_init: GClassInitFunc,
        instance_size: guint,
        instance_init: GInstanceInitFunc,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
    fn g_type_name(type_0: GType) -> *const gchar;
    fn fp_device_get_type() -> GType;
    fn fpi_image_device_state_get_type() -> GType;
    fn fpi_print_set_type(print: *mut FpPrint, type_0: FpiPrintType);
    fn fpi_device_class_auto_initialize_features(device_class: *mut FpDeviceClass);
    fn fpi_device_get_current_action(device: *mut FpDevice) -> FpiDeviceAction;
    fn fpi_device_error_new(error: FpDeviceError) -> *mut GError;
    fn fpi_device_get_enroll_data(device: *mut FpDevice, print: *mut *mut FpPrint);
    fn fpi_device_get_capture_data(
        device: *mut FpDevice,
        wait_for_finger: *mut gboolean,
    );
    fn fpi_device_action_error(device: *mut FpDevice, error: *mut GError);
    fn fpi_image_device_deactivate_complete(
        self_0: *mut FpImageDevice,
        error: *mut GError,
    );
    fn fpi_image_device_activate_complete(
        self_0: *mut FpImageDevice,
        error: *mut GError,
    );
    fn fpi_image_device_deactivate(
        image_device: *mut FpImageDevice,
        cancelling: gboolean,
    );
    fn fpi_image_device_activate(image_device: *mut FpImageDevice);
}
pub type guint8 = libc::c_uchar;
pub type gint32 = libc::c_int;
pub type guint32 = libc::c_uint;
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
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GClosure {
    #[bitfield(name = "ref_count", ty = "guint", bits = "0..=14")]
    #[bitfield(name = "meta_marshal_nouse", ty = "guint", bits = "15..=15")]
    #[bitfield(name = "n_guards", ty = "guint", bits = "16..=16")]
    #[bitfield(name = "n_fnotifiers", ty = "guint", bits = "17..=18")]
    #[bitfield(name = "n_inotifiers", ty = "guint", bits = "19..=26")]
    #[bitfield(name = "in_inotify", ty = "guint", bits = "27..=27")]
    #[bitfield(name = "floating", ty = "guint", bits = "28..=28")]
    #[bitfield(name = "derivative_flag", ty = "guint", bits = "29..=29")]
    #[bitfield(name = "in_marshal", ty = "guint", bits = "30..=30")]
    #[bitfield(name = "is_invalid", ty = "guint", bits = "31..=31")]
    pub ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub marshal: Option::<
        unsafe extern "C" fn(
            *mut GClosure,
            *mut GValue,
            guint,
            *const GValue,
            gpointer,
            gpointer,
        ) -> (),
    >,
    pub data: gpointer,
    pub notifiers: *mut GClosureNotifyData,
}
pub type GClosureNotifyData = _GClosureNotifyData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GClosureNotifyData {
    pub data: gpointer,
    pub notify: GClosureNotify,
}
pub type GClosureNotify = Option::<unsafe extern "C" fn(gpointer, *mut GClosure) -> ()>;
pub type GClosure = _GClosure;
pub type GClosureMarshal = Option::<
    unsafe extern "C" fn(
        *mut GClosure,
        *mut GValue,
        guint,
        *const GValue,
        gpointer,
        gpointer,
    ) -> (),
>;
pub type GSignalFlags = libc::c_uint;
pub const G_SIGNAL_ACCUMULATOR_FIRST_RUN: GSignalFlags = 131072;
pub const G_SIGNAL_DEPRECATED: GSignalFlags = 256;
pub const G_SIGNAL_MUST_COLLECT: GSignalFlags = 128;
pub const G_SIGNAL_NO_HOOKS: GSignalFlags = 64;
pub const G_SIGNAL_ACTION: GSignalFlags = 32;
pub const G_SIGNAL_DETAILED: GSignalFlags = 16;
pub const G_SIGNAL_NO_RECURSE: GSignalFlags = 8;
pub const G_SIGNAL_RUN_CLEANUP: GSignalFlags = 4;
pub const G_SIGNAL_RUN_LAST: GSignalFlags = 2;
pub const G_SIGNAL_RUN_FIRST: GSignalFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSignalInvocationHint {
    pub signal_id: guint,
    pub detail: GQuark,
    pub run_type: GSignalFlags,
}
pub type GSignalInvocationHint = _GSignalInvocationHint;
pub type GSignalCMarshaller = GClosureMarshal;
pub type GSignalAccumulator = Option::<
    unsafe extern "C" fn(
        *mut GSignalInvocationHint,
        *mut GValue,
        *const GValue,
        gpointer,
    ) -> gboolean,
>;
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
pub type FpImage = _FpImage;
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
pub type FpiPrintType = libc::c_uint;
pub const FPI_PRINT_NBIS: FpiPrintType = 2;
pub const FPI_PRINT_RAW: FpiPrintType = 1;
pub const FPI_PRINT_UNDEFINED: FpiPrintType = 0;
pub type FpiDeviceAction = libc::c_uint;
pub const FPI_DEVICE_ACTION_CLEAR_STORAGE: FpiDeviceAction = 10;
pub const FPI_DEVICE_ACTION_DELETE: FpiDeviceAction = 9;
pub const FPI_DEVICE_ACTION_LIST: FpiDeviceAction = 8;
pub const FPI_DEVICE_ACTION_CAPTURE: FpiDeviceAction = 7;
pub const FPI_DEVICE_ACTION_IDENTIFY: FpiDeviceAction = 6;
pub const FPI_DEVICE_ACTION_VERIFY: FpiDeviceAction = 5;
pub const FPI_DEVICE_ACTION_ENROLL: FpiDeviceAction = 4;
pub const FPI_DEVICE_ACTION_CLOSE: FpiDeviceAction = 3;
pub const FPI_DEVICE_ACTION_OPEN: FpiDeviceAction = 2;
pub const FPI_DEVICE_ACTION_PROBE: FpiDeviceAction = 1;
pub const FPI_DEVICE_ACTION_NONE: FpiDeviceAction = 0;
pub type FpImageDevice = _FpImageDevice;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpImageDevice {
    pub parent_instance: FpDevice,
}
pub type FpImageDeviceClass = _FpImageDeviceClass;
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
pub const N_PROPS: C2RustUnnamed_4 = 2;
pub const FPI_STATE_CHANGED: C2RustUnnamed_5 = 0;
pub const PROP_FPI_STATE: C2RustUnnamed_4 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpImageDevicePrivate {
    pub state: FpiImageDeviceState,
    pub active: gboolean,
    pub finger_present: gboolean,
    pub enroll_stage: gint,
    pub minutiae_scan_active: gboolean,
    pub action_error: *mut GError,
    pub capture_image: *mut FpImage,
    pub bz3_threshold: gint,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const PROP_0: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_5 = 1;
#[inline]
unsafe extern "C" fn FP_DEVICE_CLASS(mut ptr: gpointer) -> *mut FpDeviceClass {
    return g_type_check_class_cast(ptr as *mut GTypeClass, fp_device_get_type())
        as *mut libc::c_void as *mut FpDeviceClass;
}
#[inline]
unsafe extern "C" fn FP_IMAGE_DEVICE(mut ptr: gpointer) -> *mut FpImageDevice {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fp_image_device_get_type(),
    ) as *mut libc::c_void as *mut FpImageDevice;
}
#[inline]
unsafe extern "C" fn FP_IMAGE_DEVICE_GET_CLASS(
    mut ptr: gpointer,
) -> *mut FpImageDeviceClass {
    return (*(ptr as *mut GTypeInstance)).g_class as *mut FpImageDeviceClass;
}
static mut FpImageDevice_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn fp_image_device_get_instance_private(
    mut self_0: *mut FpImageDevice,
) -> gpointer {
    return (self_0 as *mut guint8).offset(FpImageDevice_private_offset as glong as isize)
        as gpointer;
}
static mut fp_image_device_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn fp_image_device_get_type() -> GType {
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
        let mut g_define_type_id: GType = fp_image_device_get_type_once();
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
#[inline(never)]
unsafe extern "C" fn fp_image_device_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_device_get_type(),
        g_intern_static_string(b"FpImageDevice\0" as *const u8 as *const libc::c_char),
        ::core::mem::size_of::<FpImageDeviceClass>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fp_image_device_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpImageDevice>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpImageDevice) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fp_image_device_init
                        as unsafe extern "C" fn(*mut FpImageDevice) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    FpImageDevice_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<FpImageDevicePrivate>() as libc::c_ulong,
    );
    return g_define_type_id;
}
unsafe extern "C" fn fp_image_device_class_intern_init(mut klass: gpointer) {
    fp_image_device_parent_class = g_type_class_peek_parent(klass);
    if FpImageDevice_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpImageDevice_private_offset);
    }
    fp_image_device_class_init(klass as *mut FpImageDeviceClass);
}
static mut properties: [*mut GParamSpec; 2] = [0 as *const GParamSpec
    as *mut GParamSpec; 2];
static mut signals: [guint; 1] = [0 as libc::c_int as guint];
unsafe extern "C" fn fp_image_device_open(mut device: *mut FpDevice) {
    let mut cls: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_GET_CLASS(device as gpointer);
    ((*cls).img_open)
        .expect("non-null function pointer")(FP_IMAGE_DEVICE(device as gpointer));
}
unsafe extern "C" fn fp_image_device_close(mut device: *mut FpDevice) {
    let mut self_0: *mut FpImageDevice = FP_IMAGE_DEVICE(device as gpointer);
    let mut cls: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_GET_CLASS(self_0 as gpointer);
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).active == 0 as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-image-device.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fp_image_device_close\0"))
                .as_ptr(),
            b"priv->active == FALSE\0" as *const u8 as *const libc::c_char,
        );
    }
    ((*cls).img_close).expect("non-null function pointer")(self_0);
}
unsafe extern "C" fn fp_image_device_cancel_action(mut device: *mut FpDevice) {
    let mut self_0: *mut FpImageDevice = FP_IMAGE_DEVICE(device as gpointer);
    let mut action: FpiDeviceAction = FPI_DEVICE_ACTION_NONE;
    action = fpi_device_get_current_action(device);
    if action as libc::c_uint == FPI_DEVICE_ACTION_ENROLL as libc::c_int as libc::c_uint
        || action as libc::c_uint
            == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
        || action as libc::c_uint
            == FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
        || action as libc::c_uint
            == FPI_DEVICE_ACTION_CAPTURE as libc::c_int as libc::c_uint
    {
        fpi_image_device_deactivate(self_0, (0 as libc::c_int == 0) as libc::c_int);
    }
}
unsafe extern "C" fn fp_image_device_start_capture_action(mut device: *mut FpDevice) {
    let mut self_0: *mut FpImageDevice = FP_IMAGE_DEVICE(device as gpointer);
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    let mut action: FpiDeviceAction = FPI_DEVICE_ACTION_NONE;
    let mut print_type: FpiPrintType = FPI_PRINT_UNDEFINED;
    action = fpi_device_get_current_action(device);
    if action as libc::c_uint == FPI_DEVICE_ACTION_CAPTURE as libc::c_int as libc::c_uint
    {
        let mut wait_for_finger: gboolean = 0;
        fpi_device_get_capture_data(device, &mut wait_for_finger);
        if wait_for_finger == 0 {
            fpi_device_action_error(
                device,
                fpi_device_error_new(FP_DEVICE_ERROR_NOT_SUPPORTED),
            );
            return;
        }
    } else if action as libc::c_uint
        == FPI_DEVICE_ACTION_ENROLL as libc::c_int as libc::c_uint
    {
        let mut enroll_print: *mut FpPrint = 0 as *mut FpPrint;
        fpi_device_get_enroll_data(device, &mut enroll_print);
        g_object_get(
            enroll_print as gpointer,
            b"fpi-type\0" as *const u8 as *const libc::c_char,
            &mut print_type as *mut FpiPrintType,
            0 as *mut libc::c_void,
        );
        if print_type as libc::c_uint != FPI_PRINT_NBIS as libc::c_int as libc::c_uint {
            fpi_print_set_type(enroll_print, FPI_PRINT_NBIS);
        }
    }
    (*priv_0).enroll_stage = 0 as libc::c_int;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).finger_present == 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-image-device.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fp_image_device_start_capture_action\0"))
                .as_ptr(),
            b"!priv->finger_present\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).minutiae_scan_active == 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-image-device.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fp_image_device_start_capture_action\0"))
                .as_ptr(),
            b"!priv->minutiae_scan_active\0" as *const u8 as *const libc::c_char,
        );
    }
    fpi_image_device_activate(self_0);
}
unsafe extern "C" fn fp_image_device_finalize(mut object: *mut GObject) {
    let mut self_0: *mut FpImageDevice = object as *mut FpImageDevice;
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).active == 0 as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-image-device.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"fp_image_device_finalize\0"))
                .as_ptr(),
            b"priv->active == FALSE\0" as *const u8 as *const libc::c_char,
        );
    }
    ((*(g_type_check_class_cast(
        fp_image_device_parent_class as *mut GTypeClass,
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
    ) as *mut libc::c_void as *mut GObjectClass))
        .finalize)
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn fp_image_device_default_activate(mut self_0: *mut FpImageDevice) {
    fpi_image_device_activate_complete(self_0, 0 as *mut GError);
}
unsafe extern "C" fn fp_image_device_default_deactivate(mut self_0: *mut FpImageDevice) {
    fpi_image_device_deactivate_complete(self_0, 0 as *mut GError);
}
unsafe extern "C" fn fp_image_device_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut FpImageDevice = FP_IMAGE_DEVICE(object as gpointer);
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    match prop_id {
        1 => {
            g_value_set_enum(value, (*priv_0).state as gint);
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const libc::c_char,
                b"../libfprint/fp-image-device.c\0" as *const u8 as *const libc::c_char,
                185 as libc::c_int,
                b"property\0" as *const u8 as *const libc::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn fp_image_device_constructed(mut obj: *mut GObject) {
    let mut self_0: *mut FpImageDevice = FP_IMAGE_DEVICE(obj as gpointer);
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    let mut cls: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_GET_CLASS(self_0 as gpointer);
    (*priv_0).bz3_threshold = 40 as libc::c_int;
    if (*cls).bz3_threshold > 0 as libc::c_int {
        (*priv_0).bz3_threshold = (*cls).bz3_threshold;
    }
    ((*(g_type_check_class_cast(
        fp_image_device_parent_class as *mut GTypeClass,
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
    ) as *mut libc::c_void as *mut GObjectClass))
        .constructed)
        .expect("non-null function pointer")(obj);
}
unsafe extern "C" fn fp_image_device_class_init(mut klass: *mut FpImageDeviceClass) {
    let mut object_class: *mut GObjectClass = g_type_check_class_cast(
        klass as *mut GTypeClass,
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
    ) as *mut libc::c_void as *mut GObjectClass;
    let mut fp_device_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    (*object_class)
        .finalize = Some(
        fp_image_device_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    );
    (*object_class)
        .get_property = Some(
        fp_image_device_get_property
            as unsafe extern "C" fn(
                *mut GObject,
                guint,
                *mut GValue,
                *mut GParamSpec,
            ) -> (),
    );
    (*object_class)
        .constructed = Some(
        fp_image_device_constructed as unsafe extern "C" fn(*mut GObject) -> (),
    );
    (*fp_device_class).nr_enroll_stages = 5 as libc::c_int;
    (*fp_device_class)
        .open = Some(fp_image_device_open as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*fp_device_class)
        .close = Some(
        fp_image_device_close as unsafe extern "C" fn(*mut FpDevice) -> (),
    );
    (*fp_device_class)
        .enroll = Some(
        fp_image_device_start_capture_action as unsafe extern "C" fn(*mut FpDevice) -> (),
    );
    (*fp_device_class)
        .verify = Some(
        fp_image_device_start_capture_action as unsafe extern "C" fn(*mut FpDevice) -> (),
    );
    (*fp_device_class)
        .identify = Some(
        fp_image_device_start_capture_action as unsafe extern "C" fn(*mut FpDevice) -> (),
    );
    (*fp_device_class)
        .capture = Some(
        fp_image_device_start_capture_action as unsafe extern "C" fn(*mut FpDevice) -> (),
    );
    (*fp_device_class)
        .cancel = Some(
        fp_image_device_cancel_action as unsafe extern "C" fn(*mut FpDevice) -> (),
    );
    fpi_device_class_auto_initialize_features(fp_device_class);
    (*fp_device_class)
        .features = ::core::mem::transmute::<
        libc::c_uint,
        FpDeviceFeature,
    >(
        (*fp_device_class).features as libc::c_uint
            | FP_DEVICE_FEATURE_UPDATE_PRINT as libc::c_int as libc::c_uint,
    );
    (*klass)
        .activate = Some(
        fp_image_device_default_activate
            as unsafe extern "C" fn(*mut FpImageDevice) -> (),
    );
    (*klass)
        .deactivate = Some(
        fp_image_device_default_deactivate
            as unsafe extern "C" fn(*mut FpImageDevice) -> (),
    );
    properties[PROP_FPI_STATE as libc::c_int
        as usize] = g_param_spec_enum(
        b"fpi-image-device-state\0" as *const u8 as *const libc::c_char,
        b"Image Device State\0" as *const u8 as *const libc::c_char,
        b"Private: The state of the image device\0" as *const u8 as *const libc::c_char,
        fpi_image_device_state_get_type(),
        FPI_IMAGE_DEVICE_STATE_INACTIVE as libc::c_int,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READABLE as libc::c_int)
            as GParamFlags,
    );
    signals[FPI_STATE_CHANGED as libc::c_int
        as usize] = g_signal_new(
        b"fpi-image-device-state-changed\0" as *const u8 as *const libc::c_char,
        (*(object_class as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_FIRST,
        344 as libc::c_ulong as glong as guint,
        None,
        0 as *mut libc::c_void,
        None,
        ((1 as libc::c_int) << 2 as libc::c_int) as GType,
        1 as libc::c_int as guint,
        fpi_image_device_state_get_type(),
    );
    g_object_class_install_properties(
        object_class,
        N_PROPS as libc::c_int as guint,
        properties.as_mut_ptr(),
    );
}
unsafe extern "C" fn fp_image_device_init(mut self_0: *mut FpImageDevice) {}
