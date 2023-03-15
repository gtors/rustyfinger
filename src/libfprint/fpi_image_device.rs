use ::libc;
extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _FpImage;
    pub type _FpPrint;
    fn g_error_new(
        domain: GQuark,
        code: gint,
        format: *const gchar,
        _: ...
    ) -> *mut GError;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_clear_error(err: *mut *mut GError);
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
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
    fn g_type_class_peek_static(type_0: GType) -> gpointer;
    fn g_type_class_get_instance_private_offset(g_class: gpointer) -> gint;
    fn g_type_check_instance_cast(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> *mut GTypeInstance;
    fn g_type_check_instance_is_a(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> gboolean;
    fn g_signal_emit_by_name(instance: gpointer, detailed_signal: *const gchar, _: ...);
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_enum_to_string(g_enum_type: GType, value: gint) -> *mut gchar;
    fn g_cancellable_set_error_if_cancelled(
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_io_error_quark() -> GQuark;
    fn fp_image_get_type() -> GType;
    fn fp_image_detect_minutiae(
        self_0: *mut FpImage,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn fp_image_detect_minutiae_finish(
        self_0: *mut FpImage,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn fp_device_get_type() -> GType;
    fn fp_print_new(device: *mut FpDevice) -> *mut FpPrint;
    fn fp_device_retry_quark() -> GQuark;
    fn fp_device_error_quark() -> GQuark;
    fn fp_device_get_nr_enroll_stages(device: *mut FpDevice) -> gint;
    fn fpi_image_device_state_get_type() -> GType;
    fn fpi_print_add_print(print: *mut FpPrint, add: *mut FpPrint);
    fn fpi_print_set_type(print: *mut FpPrint, type_0: FpiPrintType);
    fn fpi_print_add_from_image(
        print: *mut FpPrint,
        image: *mut FpImage,
        error: *mut *mut GError,
    ) -> gboolean;
    fn fpi_print_bz3_match(
        temp: *mut FpPrint,
        print: *mut FpPrint,
        bz3_threshold: gint,
        error: *mut *mut GError,
    ) -> FpiMatchResult;
    fn fpi_device_get_current_action(device: *mut FpDevice) -> FpiDeviceAction;
    fn fpi_device_action_is_cancelled(device: *mut FpDevice) -> gboolean;
    fn fpi_device_retry_new(error: FpDeviceRetry) -> *mut GError;
    fn fpi_device_retry_new_msg(
        error: FpDeviceRetry,
        msg: *const gchar,
        _: ...
    ) -> *mut GError;
    fn fpi_device_get_enroll_data(device: *mut FpDevice, print: *mut *mut FpPrint);
    fn fpi_device_get_verify_data(device: *mut FpDevice, print: *mut *mut FpPrint);
    fn fpi_device_get_identify_data(device: *mut FpDevice, prints: *mut *mut GPtrArray);
    fn fpi_device_get_cancellable(device: *mut FpDevice) -> *mut GCancellable;
    fn fpi_device_action_error(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_open_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_close_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_enroll_complete(
        device: *mut FpDevice,
        print: *mut FpPrint,
        error: *mut GError,
    );
    fn fpi_device_verify_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_identify_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_capture_complete(
        device: *mut FpDevice,
        image: *mut FpImage,
        error: *mut GError,
    );
    fn fpi_device_enroll_progress(
        device: *mut FpDevice,
        completed_stages: gint,
        print: *mut FpPrint,
        error: *mut GError,
    );
    fn fpi_device_verify_report(
        device: *mut FpDevice,
        result: FpiMatchResult,
        print: *mut FpPrint,
        error: *mut GError,
    );
    fn fpi_device_identify_report(
        device: *mut FpDevice,
        match_0: *mut FpPrint,
        print: *mut FpPrint,
        error: *mut GError,
    );
    fn fpi_device_report_finger_status(
        device: *mut FpDevice,
        finger_status: FpFingerStatusFlags,
    ) -> gboolean;
    fn fpi_device_report_finger_status_changes(
        device: *mut FpDevice,
        added_status: FpFingerStatusFlags,
        removed_status: FpFingerStatusFlags,
    ) -> gboolean;
    fn fp_image_device_get_type() -> GType;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_0 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_0 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_0 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_0 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_0 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_0 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_0 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_0 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_0 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_0 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_0 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_0 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_0 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_0 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_0 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_0 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_0 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_0 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_0 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_0 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_0 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_0 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_0 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_0 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_0 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_0 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_0 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_0 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_0 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_0 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_0 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_0 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_0 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_0 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_0 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_0 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_0 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_0 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_0 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_0 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_0 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_0 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_0 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_0 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_0 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_0 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_0 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_0 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_0 = 0;
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GAsyncReadyCallback = Option::<
    unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUsbDevice {
    pub parent_instance: GObject,
}
pub type GUsbDevice = _GUsbDevice;
pub type FpImage = _FpImage;
pub type FpImage_autoptr = *mut FpImage;
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
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub driver_data: guint64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub virtual_envvar: *const gchar,
    pub c2rust_unnamed_0: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub udev_types: FpiDeviceUdevSubtypeFlags,
    pub spi_acpi_id: *const gchar,
    pub hid_id: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub pid: guint,
    pub vid: guint,
}
pub type FpiDeviceUdevSubtypeFlags = libc::c_uint;
pub const FPI_DEVICE_UDEV_SUBTYPE_HIDRAW: FpiDeviceUdevSubtypeFlags = 2;
pub const FPI_DEVICE_UDEV_SUBTYPE_SPIDEV: FpiDeviceUdevSubtypeFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub pid: guint,
    pub vid: guint,
}
pub type FpDeviceType = libc::c_uint;
pub const FP_DEVICE_TYPE_USB: FpDeviceType = 2;
pub const FP_DEVICE_TYPE_UDEV: FpDeviceType = 1;
pub const FP_DEVICE_TYPE_VIRTUAL: FpDeviceType = 0;
pub type FpDeviceClass = _FpDeviceClass;
pub type FpPrint = _FpPrint;
pub type FpPrint_autoptr = *mut FpPrint;
pub type FpFingerStatusFlags = libc::c_uint;
pub const FP_FINGER_STATUS_PRESENT: FpFingerStatusFlags = 2;
pub const FP_FINGER_STATUS_NEEDED: FpFingerStatusFlags = 1;
pub const FP_FINGER_STATUS_NONE: FpFingerStatusFlags = 0;
pub type FpDeviceRetry = libc::c_uint;
pub const FP_DEVICE_RETRY_REMOVE_FINGER: FpDeviceRetry = 3;
pub const FP_DEVICE_RETRY_CENTER_FINGER: FpDeviceRetry = 2;
pub const FP_DEVICE_RETRY_TOO_SHORT: FpDeviceRetry = 1;
pub const FP_DEVICE_RETRY_GENERAL: FpDeviceRetry = 0;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const FP_DEVICE_ERROR_TOO_HOT: C2RustUnnamed_5 = 257;
pub const FP_DEVICE_ERROR_REMOVED: C2RustUnnamed_5 = 256;
pub const FP_DEVICE_ERROR_DATA_DUPLICATE: C2RustUnnamed_5 = 9;
pub const FP_DEVICE_ERROR_DATA_FULL: C2RustUnnamed_5 = 8;
pub const FP_DEVICE_ERROR_DATA_NOT_FOUND: C2RustUnnamed_5 = 7;
pub const FP_DEVICE_ERROR_DATA_INVALID: C2RustUnnamed_5 = 6;
pub const FP_DEVICE_ERROR_PROTO: C2RustUnnamed_5 = 5;
pub const FP_DEVICE_ERROR_BUSY: C2RustUnnamed_5 = 4;
pub const FP_DEVICE_ERROR_ALREADY_OPEN: C2RustUnnamed_5 = 3;
pub const FP_DEVICE_ERROR_NOT_OPEN: C2RustUnnamed_5 = 2;
pub const FP_DEVICE_ERROR_NOT_SUPPORTED: C2RustUnnamed_5 = 1;
pub const FP_DEVICE_ERROR_GENERAL: C2RustUnnamed_5 = 0;
pub type FpiPrintType = libc::c_uint;
pub const FPI_PRINT_NBIS: FpiPrintType = 2;
pub const FPI_PRINT_RAW: FpiPrintType = 1;
pub const FPI_PRINT_UNDEFINED: FpiPrintType = 0;
pub type FpiMatchResult = libc::c_int;
pub const FPI_MATCH_SUCCESS: FpiMatchResult = 1;
pub const FPI_MATCH_FAIL: FpiMatchResult = 0;
pub const FPI_MATCH_ERROR: FpiMatchResult = -1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub from: FpiImageDeviceState,
    pub to: FpiImageDeviceState,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[inline]
unsafe extern "C" fn g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = 0 as *mut libc::c_void;
    ref_0 = *ptr;
    *ptr = 0 as *mut libc::c_void;
    return ref_0;
}
#[inline]
unsafe extern "C" fn FP_IMAGE(mut ptr: gpointer) -> *mut FpImage {
    return g_type_check_instance_cast(ptr as *mut GTypeInstance, fp_image_get_type())
        as *mut libc::c_void as *mut FpImage;
}
#[inline]
unsafe extern "C" fn FP_DEVICE(mut ptr: gpointer) -> *mut FpDevice {
    return g_type_check_instance_cast(ptr as *mut GTypeInstance, fp_device_get_type())
        as *mut libc::c_void as *mut FpDevice;
}
#[inline]
unsafe extern "C" fn FP_IMAGE_DEVICE(mut ptr: gpointer) -> *mut FpImageDevice {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fp_image_device_get_type(),
    ) as *mut libc::c_void as *mut FpImageDevice;
}
#[inline]
unsafe extern "C" fn FP_IS_IMAGE_DEVICE(mut ptr: gpointer) -> gboolean {
    return ({
        let mut __inst: *mut GTypeInstance = ptr as *mut GTypeInstance;
        let mut __t: GType = fp_image_device_get_type();
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
#[inline]
unsafe extern "C" fn FP_IMAGE_DEVICE_GET_CLASS(
    mut ptr: gpointer,
) -> *mut FpImageDeviceClass {
    return (*(ptr as *mut GTypeInstance)).g_class as *mut FpImageDeviceClass;
}
#[inline]
unsafe extern "C" fn fp_image_device_get_instance_private(
    mut self_0: *mut FpImageDevice,
) -> gpointer {
    let mut img_class: *mut FpImageDeviceClass = g_type_class_peek_static(
        fp_image_device_get_type(),
    ) as *mut FpImageDeviceClass;
    return (self_0 as *mut guint8)
        .offset(
            g_type_class_get_instance_private_offset(img_class as gpointer) as glong
                as isize,
        ) as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_image_device_activate(mut self_0: *mut FpImageDevice) {
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    let mut cls: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_GET_CLASS(self_0 as gpointer);
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).active == 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-image-device.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"fpi_image_device_activate\0"))
                .as_ptr(),
            b"!priv->active\0" as *const u8 as *const libc::c_char,
        );
    }
    g_log(
        b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Activating image device\0" as *const u8 as *const libc::c_char,
    );
    fp_image_device_change_state(self_0, FPI_IMAGE_DEVICE_STATE_ACTIVATING);
    ((*cls).activate).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_image_device_deactivate(
    mut self_0: *mut FpImageDevice,
    mut cancelling: gboolean,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    let mut cls: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_GET_CLASS(device as gpointer);
    if (*priv_0).active == 0
        || (*priv_0).state as libc::c_uint
            == FPI_IMAGE_DEVICE_STATE_DEACTIVATING as libc::c_int as libc::c_uint
    {
        g_log(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Already deactivated, ignoring request.\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if cancelling == 0
        && (*priv_0).state as libc::c_uint
            != FPI_IMAGE_DEVICE_STATE_IDLE as libc::c_int as libc::c_uint
    {
        g_log(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Deactivating image device while it is not idle, this should not happen.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    g_log(
        b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Deactivating image device\0" as *const u8 as *const libc::c_char,
    );
    fp_image_device_change_state(self_0, FPI_IMAGE_DEVICE_STATE_DEACTIVATING);
    ((*cls).deactivate).expect("non-null function pointer")(self_0);
}
unsafe extern "C" fn fp_image_device_change_state(
    mut self_0: *mut FpImageDevice,
    mut state: FpiImageDeviceState,
) {
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    let mut prev_state_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut state_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut transition_is_valid: gboolean = 0 as libc::c_int;
    let mut i: gint = 0;
    let mut valid_transitions: [C2RustUnnamed_6; 14] = [
        {
            let mut init = C2RustUnnamed_6 {
                from: FPI_IMAGE_DEVICE_STATE_INACTIVE,
                to: FPI_IMAGE_DEVICE_STATE_ACTIVATING,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                from: FPI_IMAGE_DEVICE_STATE_ACTIVATING,
                to: FPI_IMAGE_DEVICE_STATE_IDLE,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                from: FPI_IMAGE_DEVICE_STATE_ACTIVATING,
                to: FPI_IMAGE_DEVICE_STATE_INACTIVE,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                from: FPI_IMAGE_DEVICE_STATE_IDLE,
                to: FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_ON,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                from: FPI_IMAGE_DEVICE_STATE_IDLE,
                to: FPI_IMAGE_DEVICE_STATE_CAPTURE,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                from: FPI_IMAGE_DEVICE_STATE_IDLE,
                to: FPI_IMAGE_DEVICE_STATE_DEACTIVATING,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                from: FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_ON,
                to: FPI_IMAGE_DEVICE_STATE_CAPTURE,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                from: FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_ON,
                to: FPI_IMAGE_DEVICE_STATE_DEACTIVATING,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                from: FPI_IMAGE_DEVICE_STATE_CAPTURE,
                to: FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_OFF,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                from: FPI_IMAGE_DEVICE_STATE_CAPTURE,
                to: FPI_IMAGE_DEVICE_STATE_IDLE,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                from: FPI_IMAGE_DEVICE_STATE_CAPTURE,
                to: FPI_IMAGE_DEVICE_STATE_DEACTIVATING,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                from: FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_OFF,
                to: FPI_IMAGE_DEVICE_STATE_IDLE,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                from: FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_OFF,
                to: FPI_IMAGE_DEVICE_STATE_DEACTIVATING,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                from: FPI_IMAGE_DEVICE_STATE_DEACTIVATING,
                to: FPI_IMAGE_DEVICE_STATE_INACTIVE,
            };
            init
        },
    ];
    prev_state_str = g_enum_to_string(
        fpi_image_device_state_get_type(),
        (*priv_0).state as gint,
    );
    state_str = g_enum_to_string(fpi_image_device_state_get_type(), state as gint);
    g_log(
        b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Image device internal state change from %s to %s\0" as *const u8
            as *const libc::c_char,
        prev_state_str,
        state_str,
    );
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[C2RustUnnamed_6; 14]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong)
    {
        if valid_transitions[i as usize].from as libc::c_uint
            == (*priv_0).state as libc::c_uint
            && valid_transitions[i as usize].to as libc::c_uint == state as libc::c_uint
        {
            transition_is_valid = (0 as libc::c_int == 0) as libc::c_int;
            break;
        } else {
            i += 1;
        }
    }
    if transition_is_valid == 0 {
        g_log(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Internal state machine issue: transition from %s to %s should not happen!\0"
                as *const u8 as *const libc::c_char,
            prev_state_str,
            state_str,
        );
    }
    (*priv_0).state = state;
    g_object_notify(
        g_type_check_instance_cast(
            self_0 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"fpi-image-device-state\0" as *const u8 as *const libc::c_char,
    );
    g_signal_emit_by_name(
        self_0 as gpointer,
        b"fpi-image-device-state-changed\0" as *const u8 as *const libc::c_char,
        (*priv_0).state as libc::c_uint,
    );
    if state as libc::c_uint
        == FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_ON as libc::c_int as libc::c_uint
    {
        fpi_device_report_finger_status_changes(
            FP_DEVICE(self_0 as gpointer),
            FP_FINGER_STATUS_NEEDED,
            FP_FINGER_STATUS_NONE,
        );
    } else if state as libc::c_uint
        == FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_OFF as libc::c_int as libc::c_uint
    {
        fpi_device_report_finger_status_changes(
            FP_DEVICE(self_0 as gpointer),
            FP_FINGER_STATUS_NONE,
            FP_FINGER_STATUS_NEEDED,
        );
    }
}
unsafe extern "C" fn fp_image_device_enroll_maybe_await_finger_on(
    mut self_0: *mut FpImageDevice,
) {
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    if (*priv_0).minutiae_scan_active != 0 || (*priv_0).finger_present != 0 {
        return;
    }
    fp_image_device_change_state(self_0, FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_ON);
}
unsafe extern "C" fn fp_image_device_maybe_complete_action(
    mut self_0: *mut FpImageDevice,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    let mut action: FpiDeviceAction = FPI_DEVICE_ACTION_NONE;
    if !error.is_null() {
        if !((*priv_0).action_error).is_null()
            && !((*(*priv_0).action_error).domain == fp_device_retry_quark())
        {
            g_log(
                b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Will complete with first error, new error was: %s\0" as *const u8
                    as *const libc::c_char,
                (*error).message,
            );
            g_clear_error(&mut error);
        } else {
            g_clear_error(&mut (*priv_0).action_error);
            (*priv_0).action_error = error;
        }
    }
    if (*priv_0).active != 0 || (*priv_0).minutiae_scan_active != 0 {
        return;
    }
    if ((*priv_0).action_error).is_null() {
        g_cancellable_set_error_if_cancelled(
            fpi_device_get_cancellable(device),
            &mut (*priv_0).action_error,
        );
    }
    if !((*priv_0).action_error).is_null() {
        fpi_device_action_error(
            device,
            (if 0 as libc::c_int != 0 {
                (*priv_0).action_error as *mut libc::c_void
            } else {
                g_steal_pointer(
                    &mut (*priv_0).action_error as *mut *mut GError as gpointer,
                )
            }) as *mut GError,
        );
        let mut _pp: C2RustUnnamed_7 = C2RustUnnamed_7 {
            in_0: 0 as *mut libc::c_char,
        };
        let mut _p: gpointer = 0 as *mut libc::c_void;
        let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(gpointer) -> ()>,
            GDestroyNotify,
        >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
        _pp
            .in_0 = &mut (*priv_0).capture_image as *mut *mut FpImage
            as *mut libc::c_char;
        _p = *_pp.out;
        if !_p.is_null() {
            *_pp.out = 0 as *mut libc::c_void;
            _destroy.expect("non-null function pointer")(_p);
        }
        return;
    }
    action = fpi_device_get_current_action(FP_DEVICE(self_0 as gpointer));
    if action as libc::c_uint == FPI_DEVICE_ACTION_ENROLL as libc::c_int as libc::c_uint
    {
        let mut enroll_print: *mut FpPrint = 0 as *mut FpPrint;
        fpi_device_get_enroll_data(device, &mut enroll_print);
        fpi_device_enroll_complete(
            device,
            g_object_ref(enroll_print as gpointer) as *mut FpPrint,
            0 as *mut GError,
        );
    } else if action as libc::c_uint
        == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
    {
        fpi_device_verify_complete(device, 0 as *mut GError);
    } else if action as libc::c_uint
        == FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
    {
        fpi_device_identify_complete(device, 0 as *mut GError);
    } else if action as libc::c_uint
        == FPI_DEVICE_ACTION_CAPTURE as libc::c_int as libc::c_uint
    {
        fpi_device_capture_complete(
            device,
            (if 0 as libc::c_int != 0 {
                (*priv_0).capture_image as *mut libc::c_void
            } else {
                g_steal_pointer(
                    &mut (*priv_0).capture_image as *mut *mut FpImage as gpointer,
                )
            }) as *mut FpImage,
            0 as *mut GError,
        );
    } else {
        g_assertion_message_expr(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-image-device.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"fp_image_device_maybe_complete_action\0"))
                .as_ptr(),
            0 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn fpi_image_device_minutiae_detected(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut image: FpImage_autoptr = FP_IMAGE(source_object as gpointer);
    let mut print: FpPrint_autoptr = 0 as FpPrint_autoptr;
    let mut error: *mut GError = 0 as *mut GError;
    let mut self_0: *mut FpImageDevice = FP_IMAGE_DEVICE(user_data);
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    let mut priv_0: *mut FpImageDevicePrivate = 0 as *mut FpImageDevicePrivate;
    let mut action: FpiDeviceAction = FPI_DEVICE_ACTION_NONE;
    priv_0 = fp_image_device_get_instance_private(FP_IMAGE_DEVICE(device as gpointer))
        as *mut FpImageDevicePrivate;
    (*priv_0).minutiae_scan_active = 0 as libc::c_int;
    if fp_image_detect_minutiae_finish(image, res, &mut error) == 0 {
        if g_error_matches(
            error,
            g_io_error_quark(),
            G_IO_ERROR_CANCELLED as libc::c_int,
        ) != 0
        {
            fp_image_device_maybe_complete_action(
                self_0,
                (if 0 as libc::c_int != 0 {
                    error as *mut libc::c_void
                } else {
                    g_steal_pointer(&mut error as *mut *mut GError as gpointer)
                }) as *mut GError,
            );
            fpi_image_device_deactivate(self_0, (0 as libc::c_int == 0) as libc::c_int);
            return;
        }
        g_log(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Failed to detect minutiae: %s\0" as *const u8 as *const libc::c_char,
            (*error).message,
        );
        let mut _pp: C2RustUnnamed_9 = C2RustUnnamed_9 {
            in_0: 0 as *mut libc::c_char,
        };
        let mut _p: gpointer = 0 as *mut libc::c_void;
        let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GError) -> ()>,
            GDestroyNotify,
        >(Some(g_error_free as unsafe extern "C" fn(*mut GError) -> ()));
        _pp.in_0 = &mut error as *mut *mut GError as *mut libc::c_char;
        _p = *_pp.out;
        if !_p.is_null() {
            *_pp.out = 0 as *mut libc::c_void;
            _destroy.expect("non-null function pointer")(_p);
        }
        error = fpi_device_retry_new_msg(
            FP_DEVICE_RETRY_GENERAL,
            b"Minutiae detection failed, please retry\0" as *const u8
                as *const libc::c_char,
        );
    }
    action = fpi_device_get_current_action(device);
    if action as libc::c_uint == FPI_DEVICE_ACTION_CAPTURE as libc::c_int as libc::c_uint
    {
        (*priv_0)
            .capture_image = (if 0 as libc::c_int != 0 {
            image as *mut libc::c_void
        } else {
            g_steal_pointer(&mut image as *mut FpImage_autoptr as gpointer)
        }) as *mut FpImage;
        fp_image_device_maybe_complete_action(
            self_0,
            (if 0 as libc::c_int != 0 {
                error as *mut libc::c_void
            } else {
                g_steal_pointer(&mut error as *mut *mut GError as gpointer)
            }) as *mut GError,
        );
        return;
    }
    if error.is_null() {
        print = fp_print_new(device);
        fpi_print_set_type(print, FPI_PRINT_NBIS);
        if fpi_print_add_from_image(print, image, &mut error) == 0 {
            let mut _pp_0: C2RustUnnamed_8 = C2RustUnnamed_8 {
                in_0: 0 as *mut libc::c_char,
            };
            let mut _p_0: gpointer = 0 as *mut libc::c_void;
            let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                GDestroyNotify,
            >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
            _pp_0.in_0 = &mut print as *mut FpPrint_autoptr as *mut libc::c_char;
            _p_0 = *_pp_0.out;
            if !_p_0.is_null() {
                *_pp_0.out = 0 as *mut libc::c_void;
                _destroy_0.expect("non-null function pointer")(_p_0);
            }
            if (*error).domain != fp_device_retry_quark() {
                fp_image_device_maybe_complete_action(
                    self_0,
                    (if 0 as libc::c_int != 0 {
                        error as *mut libc::c_void
                    } else {
                        g_steal_pointer(&mut error as *mut *mut GError as gpointer)
                    }) as *mut GError,
                );
                fpi_image_device_deactivate(
                    self_0,
                    (0 as libc::c_int == 0) as libc::c_int,
                );
                return;
            }
        }
    }
    if action as libc::c_uint == FPI_DEVICE_ACTION_ENROLL as libc::c_int as libc::c_uint
    {
        let mut enroll_print: *mut FpPrint = 0 as *mut FpPrint;
        fpi_device_get_enroll_data(device, &mut enroll_print);
        if !print.is_null() {
            fpi_print_add_print(enroll_print, print);
            (*priv_0).enroll_stage += 1 as libc::c_int;
        }
        fpi_device_enroll_progress(
            device,
            (*priv_0).enroll_stage,
            (if 0 as libc::c_int != 0 {
                print as *mut libc::c_void
            } else {
                g_steal_pointer(&mut print as *mut FpPrint_autoptr as gpointer)
            }) as *mut FpPrint,
            error,
        );
        if (*priv_0).enroll_stage == fp_device_get_nr_enroll_stages(device) {
            fp_image_device_maybe_complete_action(
                self_0,
                (if 0 as libc::c_int != 0 {
                    error as *mut libc::c_void
                } else {
                    g_steal_pointer(&mut error as *mut *mut GError as gpointer)
                }) as *mut GError,
            );
            fpi_image_device_deactivate(self_0, 0 as libc::c_int);
        } else {
            fp_image_device_enroll_maybe_await_finger_on(
                FP_IMAGE_DEVICE(device as gpointer),
            );
        }
    } else if action as libc::c_uint
        == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
    {
        let mut template: *mut FpPrint = 0 as *mut FpPrint;
        let mut result: FpiMatchResult = FPI_MATCH_FAIL;
        fpi_device_get_verify_data(device, &mut template);
        if !print.is_null() {
            result = fpi_print_bz3_match(
                template,
                print,
                (*priv_0).bz3_threshold,
                &mut error,
            );
        } else {
            result = FPI_MATCH_ERROR;
        }
        if error.is_null() || (*error).domain == fp_device_retry_quark() {
            fpi_device_verify_report(
                device,
                result,
                (if 0 as libc::c_int != 0 {
                    print as *mut libc::c_void
                } else {
                    g_steal_pointer(&mut print as *mut FpPrint_autoptr as gpointer)
                }) as *mut FpPrint,
                (if 0 as libc::c_int != 0 {
                    error as *mut libc::c_void
                } else {
                    g_steal_pointer(&mut error as *mut *mut GError as gpointer)
                }) as *mut GError,
            );
        }
        fp_image_device_maybe_complete_action(
            self_0,
            (if 0 as libc::c_int != 0 {
                error as *mut libc::c_void
            } else {
                g_steal_pointer(&mut error as *mut *mut GError as gpointer)
            }) as *mut GError,
        );
    } else if action as libc::c_uint
        == FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
    {
        let mut i: gint = 0;
        let mut templates: *mut GPtrArray = 0 as *mut GPtrArray;
        let mut result_0: *mut FpPrint = 0 as *mut FpPrint;
        fpi_device_get_identify_data(device, &mut templates);
        i = 0 as libc::c_int;
        while error.is_null() && (i as libc::c_uint) < (*templates).len {
            let mut template_0: *mut FpPrint = *((*templates).pdata).offset(i as isize)
                as *mut FpPrint;
            if fpi_print_bz3_match(
                template_0,
                print,
                (*priv_0).bz3_threshold,
                &mut error,
            ) as libc::c_int == FPI_MATCH_SUCCESS as libc::c_int
            {
                result_0 = template_0;
                break;
            } else {
                i += 1;
            }
        }
        if error.is_null() || (*error).domain == fp_device_retry_quark() {
            fpi_device_identify_report(
                device,
                result_0,
                (if 0 as libc::c_int != 0 {
                    print as *mut libc::c_void
                } else {
                    g_steal_pointer(&mut print as *mut FpPrint_autoptr as gpointer)
                }) as *mut FpPrint,
                (if 0 as libc::c_int != 0 {
                    error as *mut libc::c_void
                } else {
                    g_steal_pointer(&mut error as *mut *mut GError as gpointer)
                }) as *mut GError,
            );
        }
        fp_image_device_maybe_complete_action(
            self_0,
            (if 0 as libc::c_int != 0 {
                error as *mut libc::c_void
            } else {
                g_steal_pointer(&mut error as *mut *mut GError as gpointer)
            }) as *mut GError,
        );
    } else {
        g_assertion_message_expr(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-image-device.c\0" as *const u8 as *const libc::c_char,
            366 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"fpi_image_device_minutiae_detected\0"))
                .as_ptr(),
            0 as *const libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_image_device_set_bz3_threshold(
    mut self_0: *mut FpImageDevice,
    mut bz3_threshold: gint,
) {
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_IMAGE_DEVICE(self_0 as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"fpi_image_device_set_bz3_threshold\0"))
                .as_ptr(),
            b"FP_IS_IMAGE_DEVICE (self)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if bz3_threshold > 0 as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"fpi_image_device_set_bz3_threshold\0"))
                .as_ptr(),
            b"bz3_threshold > 0\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*priv_0).bz3_threshold = bz3_threshold;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_image_device_report_finger_status(
    mut self_0: *mut FpImageDevice,
    mut present: gboolean,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    let mut action: FpiDeviceAction = FPI_DEVICE_ACTION_NONE;
    if present != 0 {
        fpi_device_report_finger_status_changes(
            device,
            FP_FINGER_STATUS_PRESENT,
            FP_FINGER_STATUS_NONE,
        );
    } else {
        fpi_device_report_finger_status_changes(
            device,
            FP_FINGER_STATUS_NONE,
            FP_FINGER_STATUS_PRESENT,
        );
    }
    if (*priv_0).state as libc::c_uint
        == FPI_IMAGE_DEVICE_STATE_INACTIVE as libc::c_int as libc::c_uint
    {
        g_log(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Ignoring finger presence report as the device is not active!\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    action = fpi_device_get_current_action(device);
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if action as libc::c_uint
            != FPI_DEVICE_ACTION_OPEN as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-image-device.c\0" as *const u8 as *const libc::c_char,
            437 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"fpi_image_device_report_finger_status\0"))
                .as_ptr(),
            b"action != FPI_DEVICE_ACTION_OPEN\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if action as libc::c_uint
            != FPI_DEVICE_ACTION_CLOSE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-image-device.c\0" as *const u8 as *const libc::c_char,
            438 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"fpi_image_device_report_finger_status\0"))
                .as_ptr(),
            b"action != FPI_DEVICE_ACTION_CLOSE\0" as *const u8 as *const libc::c_char,
        );
    }
    g_log(
        b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Image device reported finger status: %s\0" as *const u8 as *const libc::c_char,
        if present != 0 {
            b"on\0" as *const u8 as *const libc::c_char
        } else {
            b"off\0" as *const u8 as *const libc::c_char
        },
    );
    (*priv_0).finger_present = present;
    if present != 0
        && (*priv_0).state as libc::c_uint
            == FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_ON as libc::c_int as libc::c_uint
    {
        fp_image_device_change_state(self_0, FPI_IMAGE_DEVICE_STATE_CAPTURE);
    } else if present == 0
        && (*priv_0).state as libc::c_uint
            == FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_OFF as libc::c_int as libc::c_uint
    {
        fp_image_device_change_state(self_0, FPI_IMAGE_DEVICE_STATE_IDLE);
        if action as libc::c_uint
            != FPI_DEVICE_ACTION_ENROLL as libc::c_int as libc::c_uint
        {
            fpi_image_device_deactivate(self_0, 0 as libc::c_int);
        } else {
            fp_image_device_enroll_maybe_await_finger_on(self_0);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn fpi_image_device_image_captured(
    mut self_0: *mut FpImageDevice,
    mut image: *mut FpImage,
) {
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    let mut action: FpiDeviceAction = FPI_DEVICE_ACTION_NONE;
    action = fpi_device_get_current_action(FP_DEVICE(self_0 as gpointer));
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !image.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"fpi_image_device_image_captured\0"))
                .as_ptr(),
            b"image != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).state as libc::c_uint
            == FPI_IMAGE_DEVICE_STATE_CAPTURE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"fpi_image_device_image_captured\0"))
                .as_ptr(),
            b"priv->state == FPI_IMAGE_DEVICE_STATE_CAPTURE\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if action as libc::c_uint
            == FPI_DEVICE_ACTION_ENROLL as libc::c_int as libc::c_uint
            || action as libc::c_uint
                == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
            || action as libc::c_uint
                == FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
            || action as libc::c_uint
                == FPI_DEVICE_ACTION_CAPTURE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"fpi_image_device_image_captured\0"))
                .as_ptr(),
            b"action == FPI_DEVICE_ACTION_ENROLL || action == FPI_DEVICE_ACTION_VERIFY || action == FPI_DEVICE_ACTION_IDENTIFY || action == FPI_DEVICE_ACTION_CAPTURE\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    g_log(
        b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Image device captured an image\0" as *const u8 as *const libc::c_char,
    );
    (*priv_0).minutiae_scan_active = (0 as libc::c_int == 0) as libc::c_int;
    fp_image_detect_minutiae(
        image,
        fpi_device_get_cancellable(FP_DEVICE(self_0 as gpointer)),
        Some(
            fpi_image_device_minutiae_detected
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        self_0 as gpointer,
    );
    fp_image_device_change_state(self_0, FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_OFF);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_image_device_retry_scan(
    mut self_0: *mut FpImageDevice,
    mut retry: FpDeviceRetry,
) {
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    let mut action: FpiDeviceAction = FPI_DEVICE_ACTION_NONE;
    let mut error: *mut GError = 0 as *mut GError;
    action = fpi_device_get_current_action(FP_DEVICE(self_0 as gpointer));
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).state as libc::c_uint
            != FPI_IMAGE_DEVICE_STATE_INACTIVE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fpi_image_device_retry_scan\0"))
                .as_ptr(),
            b"priv->state != FPI_IMAGE_DEVICE_STATE_INACTIVE\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if action as libc::c_uint
            == FPI_DEVICE_ACTION_ENROLL as libc::c_int as libc::c_uint
            || action as libc::c_uint
                == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
            || action as libc::c_uint
                == FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
            || action as libc::c_uint
                == FPI_DEVICE_ACTION_CAPTURE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fpi_image_device_retry_scan\0"))
                .as_ptr(),
            b"action == FPI_DEVICE_ACTION_ENROLL || action == FPI_DEVICE_ACTION_VERIFY || action == FPI_DEVICE_ACTION_IDENTIFY || action == FPI_DEVICE_ACTION_CAPTURE\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    error = fpi_device_retry_new(retry);
    if action as libc::c_uint == FPI_DEVICE_ACTION_ENROLL as libc::c_int as libc::c_uint
    {
        g_log(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Reporting retry during enroll\0" as *const u8 as *const libc::c_char,
        );
        fpi_device_enroll_progress(
            FP_DEVICE(self_0 as gpointer),
            (*priv_0).enroll_stage,
            0 as *mut FpPrint,
            error,
        );
        fp_image_device_change_state(self_0, FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_OFF);
    } else if action as libc::c_uint
        == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
    {
        fpi_device_verify_report(
            FP_DEVICE(self_0 as gpointer),
            FPI_MATCH_ERROR,
            0 as *mut FpPrint,
            error,
        );
        fp_image_device_maybe_complete_action(self_0, 0 as *mut GError);
        fpi_image_device_deactivate(self_0, (0 as libc::c_int == 0) as libc::c_int);
    } else if action as libc::c_uint
        == FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
    {
        fpi_device_identify_report(
            FP_DEVICE(self_0 as gpointer),
            0 as *mut FpPrint,
            0 as *mut FpPrint,
            error,
        );
        fp_image_device_maybe_complete_action(self_0, 0 as *mut GError);
        fpi_image_device_deactivate(self_0, (0 as libc::c_int == 0) as libc::c_int);
    } else {
        g_log(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Abort current operation due to retry (no early-reporting)\0" as *const u8
                as *const libc::c_char,
        );
        fp_image_device_maybe_complete_action(self_0, error);
        fpi_image_device_deactivate(self_0, (0 as libc::c_int == 0) as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_image_device_session_error(
    mut self_0: *mut FpImageDevice,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !self_0.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"fpi_image_device_session_error\0"))
                .as_ptr(),
            b"self\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if error.is_null() {
        g_log(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Driver did not provide an error, generating a generic one\0" as *const u8
                as *const libc::c_char,
        );
        error = g_error_new(
            fp_device_error_quark(),
            FP_DEVICE_ERROR_GENERAL as libc::c_int,
            b"Driver reported session error without an error\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (*priv_0).active == 0 {
        let mut action: FpiDeviceAction = fpi_device_get_current_action(
            FP_DEVICE(self_0 as gpointer),
        );
        g_log(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Driver reported session error, but device is inactive.\0" as *const u8
                as *const libc::c_char,
        );
        if action as libc::c_uint
            != FPI_DEVICE_ACTION_NONE as libc::c_int as libc::c_uint
        {
            g_log(
                b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Translating to activation failure!\0" as *const u8
                    as *const libc::c_char,
            );
            fpi_image_device_activate_complete(self_0, error);
            return;
        }
    } else if g_error_matches(
        error,
        g_io_error_quark(),
        G_IO_ERROR_CANCELLED as libc::c_int,
    ) != 0 && fpi_device_action_is_cancelled(FP_DEVICE(self_0 as gpointer)) != 0
    {
        g_log(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Driver reported a cancellation error, this is expected but not required. Ignoring.\0"
                as *const u8 as *const libc::c_char,
        );
        g_clear_error(&mut error);
        return;
    } else {
        if (*priv_0).state as libc::c_uint
            == FPI_IMAGE_DEVICE_STATE_INACTIVE as libc::c_int as libc::c_uint
        {
            g_log(
                b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Driver reported session error while deactivating already, ignoring. This indicates a driver bug.\0"
                    as *const u8 as *const libc::c_char,
            );
            g_clear_error(&mut error);
            return;
        }
    }
    if (*error).domain == fp_device_retry_quark() {
        g_log(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Driver should report retries using fpi_image_device_retry_scan!\0"
                as *const u8 as *const libc::c_char,
        );
    }
    fp_image_device_maybe_complete_action(self_0, error);
    fpi_image_device_deactivate(self_0, (0 as libc::c_int == 0) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_image_device_activate_complete(
    mut self_0: *mut FpImageDevice,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    let mut action: FpiDeviceAction = FPI_DEVICE_ACTION_NONE;
    action = fpi_device_get_current_action(FP_DEVICE(self_0 as gpointer));
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
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"fpi_image_device_activate_complete\0"))
                .as_ptr(),
            b"priv->active == FALSE\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).state as libc::c_uint
            == FPI_IMAGE_DEVICE_STATE_ACTIVATING as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"fpi_image_device_activate_complete\0"))
                .as_ptr(),
            b"priv->state == FPI_IMAGE_DEVICE_STATE_ACTIVATING\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if action as libc::c_uint
            == FPI_DEVICE_ACTION_ENROLL as libc::c_int as libc::c_uint
            || action as libc::c_uint
                == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
            || action as libc::c_uint
                == FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
            || action as libc::c_uint
                == FPI_DEVICE_ACTION_CAPTURE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"fpi_image_device_activate_complete\0"))
                .as_ptr(),
            b"action == FPI_DEVICE_ACTION_ENROLL || action == FPI_DEVICE_ACTION_VERIFY || action == FPI_DEVICE_ACTION_IDENTIFY || action == FPI_DEVICE_ACTION_CAPTURE\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    if !error.is_null() {
        g_log(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Image device activation failed\0" as *const u8 as *const libc::c_char,
        );
        fpi_device_action_error(FP_DEVICE(self_0 as gpointer), error);
        return;
    }
    g_log(
        b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Image device activation completed\0" as *const u8 as *const libc::c_char,
    );
    (*priv_0).active = (0 as libc::c_int == 0) as libc::c_int;
    fp_image_device_change_state(self_0, FPI_IMAGE_DEVICE_STATE_IDLE);
    fp_image_device_change_state(self_0, FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_ON);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_image_device_deactivate_complete(
    mut self_0: *mut FpImageDevice,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).active == (0 as libc::c_int == 0) as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fpi_image_device_deactivate_complete\0"))
                .as_ptr(),
            b"priv->active == TRUE\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).state as libc::c_uint
            == FPI_IMAGE_DEVICE_STATE_DEACTIVATING as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fpi_image_device_deactivate_complete\0"))
                .as_ptr(),
            b"priv->state == FPI_IMAGE_DEVICE_STATE_DEACTIVATING\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    g_log(
        b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Image device deactivation completed\0" as *const u8 as *const libc::c_char,
    );
    (*priv_0).active = 0 as libc::c_int;
    (*priv_0).finger_present = 0 as libc::c_int;
    fp_image_device_change_state(self_0, FPI_IMAGE_DEVICE_STATE_INACTIVE);
    fp_image_device_maybe_complete_action(self_0, error);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_image_device_open_complete(
    mut self_0: *mut FpImageDevice,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    let mut action: FpiDeviceAction = FPI_DEVICE_ACTION_NONE;
    action = fpi_device_get_current_action(FP_DEVICE(self_0 as gpointer));
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
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"fpi_image_device_open_complete\0"))
                .as_ptr(),
            b"priv->active == FALSE\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if action as libc::c_uint
            == FPI_DEVICE_ACTION_OPEN as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"fpi_image_device_open_complete\0"))
                .as_ptr(),
            b"action == FPI_DEVICE_ACTION_OPEN\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    g_log(
        b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Image device open completed\0" as *const u8 as *const libc::c_char,
    );
    (*priv_0).state = FPI_IMAGE_DEVICE_STATE_INACTIVE;
    g_object_notify(
        g_type_check_instance_cast(
            self_0 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"fpi-image-device-state\0" as *const u8 as *const libc::c_char,
    );
    fpi_device_report_finger_status(
        FP_DEVICE(self_0 as gpointer),
        FP_FINGER_STATUS_NONE,
    );
    fpi_device_open_complete(FP_DEVICE(self_0 as gpointer), error);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_image_device_close_complete(
    mut self_0: *mut FpImageDevice,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpImageDevicePrivate = fp_image_device_get_instance_private(
        self_0,
    ) as *mut FpImageDevicePrivate;
    let mut action: FpiDeviceAction = FPI_DEVICE_ACTION_NONE;
    action = fpi_device_get_current_action(FP_DEVICE(self_0 as gpointer));
    g_log(
        b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Image device close completed\0" as *const u8 as *const libc::c_char,
    );
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
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"fpi_image_device_close_complete\0"))
                .as_ptr(),
            b"priv->active == FALSE\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if action as libc::c_uint
            == FPI_DEVICE_ACTION_CLOSE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-image_device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"fpi_image_device_close_complete\0"))
                .as_ptr(),
            b"action == FPI_DEVICE_ACTION_CLOSE\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*priv_0).state = FPI_IMAGE_DEVICE_STATE_INACTIVE;
    g_object_notify(
        g_type_check_instance_cast(
            self_0 as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"fpi-image-device-state\0" as *const u8 as *const libc::c_char,
    );
    fpi_device_close_complete(FP_DEVICE(self_0 as gpointer), error);
}
