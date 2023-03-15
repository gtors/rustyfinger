use ::libc;
extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GTask;
    pub type _FpImage;
    pub type _FpPrint;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_find(
        haystack: *mut GPtrArray,
        needle: gconstpointer,
        index_: *mut guint,
    ) -> gboolean;
    fn g_error_new_literal(
        domain: GQuark,
        code: gint,
        message: *const gchar,
    ) -> *mut GError;
    fn g_error_new_valist(
        domain: GQuark,
        code: gint,
        format: *const gchar,
        args: ::core::ffi::VaList,
    ) -> *mut GError;
    fn g_clear_error(err: *mut *mut GError);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_remove(list: *mut GSList, data: gconstpointer) -> *mut GSList;
    fn g_main_context_get_thread_default() -> *mut GMainContext;
    fn g_source_new(source_funcs: *mut GSourceFuncs, struct_size: guint) -> *mut GSource;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_destroy(source: *mut GSource);
    fn g_source_set_priority(source: *mut GSource, priority: gint);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_source_set_name(source: *mut GSource, name: *const libc::c_char);
    fn g_source_set_ready_time(source: *mut GSource, ready_time: gint64);
    fn g_source_get_time(source: *mut GSource) -> gint64;
    fn g_idle_source_new() -> *mut GSource;
    fn g_get_monotonic_time() -> gint64;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_set_size(string: *mut GString, len: gsize) -> *mut GString;
    fn g_string_prepend(string: *mut GString, val: *const gchar) -> *mut GString;
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
    fn g_strchomp(string: *mut gchar) -> *mut gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    fn g_strcmp0(str1: *const libc::c_char, str2: *const libc::c_char) -> libc::c_int;
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
    fn g_type_class_get_instance_private_offset(g_class: gpointer) -> gint;
    fn g_type_check_instance_cast(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> *mut GTypeInstance;
    fn g_type_check_instance_is_a(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> gboolean;
    fn g_type_check_class_is_a(g_class: *mut GTypeClass, is_a_type: GType) -> gboolean;
    fn g_signal_emit_by_name(instance: gpointer, detailed_signal: *const gchar, _: ...);
    fn g_object_get(object: gpointer, first_property_name: *const gchar, _: ...);
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref_sink(object: gpointer) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_signal_connect_object(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        gobject: gpointer,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_enum_to_string(g_enum_type: GType, value: gint) -> *mut gchar;
    fn g_flags_to_string(flags_type: GType, value: guint) -> *mut gchar;
    fn g_type_class_peek_static(type_0: GType) -> gpointer;
    fn g_cancellable_is_cancelled(cancellable: *mut GCancellable) -> gboolean;
    fn g_cancellable_disconnect(cancellable: *mut GCancellable, handler_id: gulong);
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
    fn g_task_set_task_data(
        task: *mut GTask,
        task_data: gpointer,
        task_data_destroy: GDestroyNotify,
    );
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_priority(task: *mut GTask) -> gint;
    fn g_task_get_context(task: *mut GTask) -> *mut GMainContext;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_return_pointer(
        task: *mut GTask,
        result: gpointer,
        result_destroy: GDestroyNotify,
    );
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_int(task: *mut GTask, result: gssize);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_get_completed(task: *mut GTask) -> gboolean;
    fn g_usb_device_get_parent(self_0: *mut GUsbDevice) -> *mut GUsbDevice;
    fn g_usb_device_get_bus(self_0: *mut GUsbDevice) -> guint8;
    fn g_usb_device_get_port_number(self_0: *mut GUsbDevice) -> guint8;
    fn g_usb_device_close(self_0: *mut GUsbDevice, error: *mut *mut GError) -> gboolean;
    fn fp_device_get_type() -> GType;
    fn fp_temperature_get_type() -> GType;
    fn fp_finger_get_type() -> GType;
    fn fp_finger_status_flags_get_type() -> GType;
    fn fp_print_get_type() -> GType;
    fn fp_print_get_finger(print: *mut FpPrint) -> FpFinger;
    fn fp_device_retry_quark() -> GQuark;
    fn fp_device_error_quark() -> GQuark;
    fn fpi_device_action_get_type() -> GType;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type guint8 = libc::c_uchar;
pub type gint32 = libc::c_int;
pub type guint32 = libc::c_uint;
pub type gint64 = libc::c_long;
pub type guint64 = libc::c_ulong;
pub type gssize = libc::c_long;
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
pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
pub type va_list = __builtin_va_list;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type ssize_t = __ssize_t;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GMainContext = _GMainContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSource {
    pub callback_data: gpointer,
    pub callback_funcs: *mut GSourceCallbackFuncs,
    pub source_funcs: *const GSourceFuncs,
    pub ref_count: guint,
    pub context: *mut GMainContext,
    pub priority: gint,
    pub flags: guint,
    pub source_id: guint,
    pub poll_fds: *mut GSList,
    pub prev: *mut GSource,
    pub next: *mut GSource,
    pub name: *mut libc::c_char,
    pub priv_0: *mut GSourcePrivate,
}
pub type GSourcePrivate = _GSourcePrivate;
pub type GSource = _GSource;
pub type GSourceFuncs = _GSourceFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceFuncs {
    pub prepare: Option::<unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean>,
    pub check: Option::<unsafe extern "C" fn(*mut GSource) -> gboolean>,
    pub dispatch: Option::<
        unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
    >,
    pub finalize: Option::<unsafe extern "C" fn(*mut GSource) -> ()>,
    pub closure_callback: GSourceFunc,
    pub closure_marshal: GSourceDummyMarshal,
}
pub type GSourceDummyMarshal = Option::<unsafe extern "C" fn() -> ()>;
pub type GSourceFunc = Option::<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type GSourceCallbackFuncs = _GSourceCallbackFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceCallbackFuncs {
    pub ref_0: Option::<unsafe extern "C" fn(gpointer) -> ()>,
    pub unref: Option::<unsafe extern "C" fn(gpointer) -> ()>,
    pub get: Option::<
        unsafe extern "C" fn(
            gpointer,
            *mut GSource,
            *mut GSourceFunc,
            *mut gpointer,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
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
pub type GError_autoptr = *mut GError;
pub type GString_autoptr = *mut GString;
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
pub type GCallback = Option::<unsafe extern "C" fn() -> ()>;
pub type GConnectFlags = libc::c_uint;
pub const G_CONNECT_SWAPPED: GConnectFlags = 2;
pub const G_CONNECT_AFTER: GConnectFlags = 1;
pub const G_CONNECT_DEFAULT: GConnectFlags = 0;
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
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GTask = _GTask;
pub type GAsyncReadyCallback = Option::<
    unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
>;
pub type GTask_autoptr = *mut GTask;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUsbDevice {
    pub parent_instance: GObject,
}
pub type GUsbDevice = _GUsbDevice;
pub type GUsbDevice_autoptr = *mut GUsbDevice;
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
pub type FpFingerStatusFlags = libc::c_uint;
pub const FP_FINGER_STATUS_PRESENT: FpFingerStatusFlags = 2;
pub const FP_FINGER_STATUS_NEEDED: FpFingerStatusFlags = 1;
pub const FP_FINGER_STATUS_NONE: FpFingerStatusFlags = 0;
pub type FpTemperature = libc::c_uint;
pub const FP_TEMPERATURE_HOT: FpTemperature = 2;
pub const FP_TEMPERATURE_WARM: FpTemperature = 1;
pub const FP_TEMPERATURE_COLD: FpTemperature = 0;
pub type FpDeviceRetry = libc::c_uint;
pub const FP_DEVICE_RETRY_REMOVE_FINGER: FpDeviceRetry = 3;
pub const FP_DEVICE_RETRY_CENTER_FINGER: FpDeviceRetry = 2;
pub const FP_DEVICE_RETRY_TOO_SHORT: FpDeviceRetry = 1;
pub const FP_DEVICE_RETRY_GENERAL: FpDeviceRetry = 0;
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
pub type FpEnrollProgress = Option::<
    unsafe extern "C" fn(*mut FpDevice, gint, *mut FpPrint, gpointer, *mut GError) -> (),
>;
pub type FpMatchCb = Option::<
    unsafe extern "C" fn(
        *mut FpDevice,
        *mut FpPrint,
        *mut FpPrint,
        gpointer,
        *mut GError,
    ) -> (),
>;
pub type FpiPrintType = libc::c_uint;
pub const FPI_PRINT_NBIS: FpiPrintType = 2;
pub const FPI_PRINT_RAW: FpiPrintType = 1;
pub const FPI_PRINT_UNDEFINED: FpiPrintType = 0;
pub type FpiMatchResult = libc::c_int;
pub const FPI_MATCH_SUCCESS: FpiMatchResult = 1;
pub const FPI_MATCH_FAIL: FpiMatchResult = 0;
pub const FPI_MATCH_ERROR: FpiMatchResult = -1;
pub type FpTimeoutFunc = Option::<unsafe extern "C" fn(*mut FpDevice, gpointer) -> ()>;
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
pub struct FpDevicePrivate {
    pub type_0: FpDeviceType,
    pub usb_device: *mut GUsbDevice,
    pub virtual_env: *const gchar,
    pub udev_data: C2RustUnnamed_4,
    pub is_removed: gboolean,
    pub is_open: gboolean,
    pub is_suspended: gboolean,
    pub device_id: *mut gchar,
    pub device_name: *mut gchar,
    pub scan_type: FpScanType,
    pub features: FpDeviceFeature,
    pub driver_data: guint64,
    pub nr_enroll_stages: gint,
    pub sources: *mut GSList,
    pub current_action: FpiDeviceAction,
    pub current_task: *mut GTask,
    pub current_cancellation_reason: *mut GError,
    pub current_user_cb: GAsyncReadyCallback,
    pub current_cancellable: *mut GCancellable,
    pub current_cancellable_id: gulong,
    pub current_task_cancellable_id: gulong,
    pub current_idle_cancel_source: *mut GSource,
    pub current_task_idle_return_source: *mut GSource,
    pub wait_for_finger: gboolean,
    pub finger_status: FpFingerStatusFlags,
    pub critical_section: guint,
    pub critical_section_flush_source: *mut GSource,
    pub cancel_queued: gboolean,
    pub suspend_queued: gboolean,
    pub resume_queued: gboolean,
    pub suspend_resume_task: *mut GTask,
    pub suspend_error: *mut GError,
    pub temp_timeout: *mut GSource,
    pub temp_current: FpTemperature,
    pub temp_hot_seconds: gint32,
    pub temp_cold_seconds: gint32,
    pub temp_last_update: gint64,
    pub temp_last_active: gboolean,
    pub temp_current_ratio: gdouble,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub spidev_path: *mut gchar,
    pub hidraw_path: *mut gchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpEnrollData {
    pub print: *mut FpPrint,
    pub enroll_progress_cb: FpEnrollProgress,
    pub enroll_progress_data: gpointer,
    pub enroll_progress_destroy: GDestroyNotify,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpMatchData {
    pub enrolled_print: *mut FpPrint,
    pub gallery: *mut GPtrArray,
    pub result_reported: gboolean,
    pub match_0: *mut FpPrint,
    pub print: *mut FpPrint,
    pub error: *mut GError,
    pub match_cb: FpMatchCb,
    pub match_data: gpointer,
    pub match_destroy: GDestroyNotify,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpDeviceTimeoutSource {
    pub source: GSource,
    pub device: *mut FpDevice,
}
pub type FpDeviceTaskReturnType = _FpDeviceTaskReturnType;
pub type _FpDeviceTaskReturnType = libc::c_uint;
pub const FP_DEVICE_TASK_RETURN_ERROR: _FpDeviceTaskReturnType = 4;
pub const FP_DEVICE_TASK_RETURN_PTR_ARRAY: _FpDeviceTaskReturnType = 3;
pub const FP_DEVICE_TASK_RETURN_OBJECT: _FpDeviceTaskReturnType = 2;
pub const FP_DEVICE_TASK_RETURN_BOOL: _FpDeviceTaskReturnType = 1;
pub const FP_DEVICE_TASK_RETURN_INT: _FpDeviceTaskReturnType = 0;
pub type FpDeviceTaskReturnData = _FpDeviceTaskReturnData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpDeviceTaskReturnData {
    pub device: *mut FpDevice,
    pub type_0: FpDeviceTaskReturnType,
    pub result: gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub in_0: *mut libc::c_char,
    pub out: *mut *mut GObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub in_0: *mut libc::c_char,
    pub out: *mut *mut GObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_19 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_20 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_21 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_22 {
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
unsafe extern "C" fn g_set_object(
    mut object_ptr: *mut *mut GObject,
    mut new_object: *mut GObject,
) -> gboolean {
    let mut old_object: *mut GObject = *object_ptr;
    if old_object == new_object {
        return 0 as libc::c_int;
    }
    if !new_object.is_null() {
        g_object_ref(new_object as gpointer);
    }
    *object_ptr = new_object;
    if !old_object.is_null() {
        g_object_unref(old_object as gpointer);
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn FP_IS_DEVICE(mut ptr: gpointer) -> gboolean {
    return ({
        let mut __inst: *mut GTypeInstance = ptr as *mut GTypeInstance;
        let mut __t: GType = fp_device_get_type();
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
unsafe extern "C" fn FP_IS_DEVICE_CLASS(mut ptr: gpointer) -> gboolean {
    return ({
        let mut __class: *mut GTypeClass = ptr as *mut GTypeClass;
        let mut __t: GType = fp_device_get_type();
        let mut __r: gboolean = 0;
        if __class.is_null() {
            __r = 0 as libc::c_int;
        } else if (*__class).g_type == __t {
            __r = (0 as libc::c_int == 0) as libc::c_int;
        } else {
            __r = g_type_check_class_is_a(__class, __t);
        }
        __r
    });
}
#[inline]
unsafe extern "C" fn FP_DEVICE_GET_CLASS(mut ptr: gpointer) -> *mut FpDeviceClass {
    return (*(ptr as *mut GTypeInstance)).g_class as *mut FpDeviceClass;
}
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
#[inline]
unsafe extern "C" fn fp_device_get_instance_private(
    mut self_0: *mut FpDevice,
) -> gpointer {
    let mut dev_class: *mut FpDeviceClass = g_type_class_peek_static(
        fp_device_get_type(),
    ) as *mut FpDeviceClass;
    return (self_0 as *mut guint8)
        .offset(
            g_type_class_get_instance_private_offset(dev_class as gpointer) as glong
                as isize,
        ) as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_class_auto_initialize_features(
    mut device_class: *mut FpDeviceClass,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE_CLASS(device_class as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"fpi_device_class_auto_initialize_features\0"))
                .as_ptr(),
            b"FP_IS_DEVICE_CLASS (device_class)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ((*device_class).capture).is_some() {
        (*device_class)
            .features = ::core::mem::transmute::<
            libc::c_uint,
            FpDeviceFeature,
        >(
            (*device_class).features as libc::c_uint
                | FP_DEVICE_FEATURE_CAPTURE as libc::c_int as libc::c_uint,
        );
    }
    if ((*device_class).verify).is_some() {
        (*device_class)
            .features = ::core::mem::transmute::<
            libc::c_uint,
            FpDeviceFeature,
        >(
            (*device_class).features as libc::c_uint
                | FP_DEVICE_FEATURE_VERIFY as libc::c_int as libc::c_uint,
        );
    }
    if ((*device_class).identify).is_some() {
        (*device_class)
            .features = ::core::mem::transmute::<
            libc::c_uint,
            FpDeviceFeature,
        >(
            (*device_class).features as libc::c_uint
                | FP_DEVICE_FEATURE_IDENTIFY as libc::c_int as libc::c_uint,
        );
    }
    if ((*device_class).list).is_some() {
        (*device_class)
            .features = ::core::mem::transmute::<
            libc::c_uint,
            FpDeviceFeature,
        >(
            (*device_class).features as libc::c_uint
                | FP_DEVICE_FEATURE_STORAGE_LIST as libc::c_int as libc::c_uint,
        );
    }
    if ((*device_class).delete).is_some() {
        (*device_class)
            .features = ::core::mem::transmute::<
            libc::c_uint,
            FpDeviceFeature,
        >(
            (*device_class).features as libc::c_uint
                | FP_DEVICE_FEATURE_STORAGE_DELETE as libc::c_int as libc::c_uint,
        );
    }
    if ((*device_class).clear_storage).is_some() {
        (*device_class)
            .features = ::core::mem::transmute::<
            libc::c_uint,
            FpDeviceFeature,
        >(
            (*device_class).features as libc::c_uint
                | FP_DEVICE_FEATURE_STORAGE_CLEAR as libc::c_int as libc::c_uint,
        );
    }
    if ((*device_class).delete).is_some()
        && (((*device_class).list).is_some()
            || ((*device_class).clear_storage).is_some())
    {
        (*device_class)
            .features = ::core::mem::transmute::<
            libc::c_uint,
            FpDeviceFeature,
        >(
            (*device_class).features as libc::c_uint
                | FP_DEVICE_FEATURE_STORAGE as libc::c_int as libc::c_uint,
        );
    }
    if (*device_class).temp_hot_seconds < 0 as libc::c_int {
        (*device_class)
            .features = ::core::mem::transmute::<
            libc::c_uint,
            FpDeviceFeature,
        >(
            (*device_class).features as libc::c_uint
                | FP_DEVICE_FEATURE_ALWAYS_ON as libc::c_int as libc::c_uint,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_retry_new(mut error: FpDeviceRetry) -> *mut GError {
    let mut msg: *const gchar = 0 as *const gchar;
    match error as libc::c_uint {
        0 => {
            msg = b"Please try again.\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            msg = b"The swipe was too short, please try again.\0" as *const u8
                as *const libc::c_char;
        }
        2 => {
            msg = b"The finger was not centered properly, please try again.\0"
                as *const u8 as *const libc::c_char;
        }
        3 => {
            msg = b"Please try again after removing the finger first.\0" as *const u8
                as *const libc::c_char;
        }
        _ => {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Unsupported error, returning general error instead!\0" as *const u8
                    as *const libc::c_char,
            );
            error = FP_DEVICE_RETRY_GENERAL;
            msg = b"Please try again.\0" as *const u8 as *const libc::c_char;
        }
    }
    return g_error_new_literal(fp_device_retry_quark(), error as gint, msg);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_error_new(mut error: FpDeviceError) -> *mut GError {
    let mut msg: *const gchar = 0 as *const gchar;
    match error as libc::c_uint {
        0 => {
            msg = b"An unspecified error occurred!\0" as *const u8
                as *const libc::c_char;
        }
        1 => {
            msg = b"The operation is not supported on this device!\0" as *const u8
                as *const libc::c_char;
        }
        2 => {
            msg = b"The device needs to be opened first!\0" as *const u8
                as *const libc::c_char;
        }
        3 => {
            msg = b"The device has already been opened!\0" as *const u8
                as *const libc::c_char;
        }
        4 => {
            msg = b"The device is still busy with another operation, please try again later.\0"
                as *const u8 as *const libc::c_char;
        }
        5 => {
            msg = b"The driver encountered a protocol error with the device.\0"
                as *const u8 as *const libc::c_char;
        }
        6 => {
            msg = b"Passed (print) data is not valid.\0" as *const u8
                as *const libc::c_char;
        }
        8 => {
            msg = b"On device storage space is full.\0" as *const u8
                as *const libc::c_char;
        }
        7 => {
            msg = b"Print was not found on the devices storage.\0" as *const u8
                as *const libc::c_char;
        }
        9 => {
            msg = b"This finger has already enrolled, please try a different finger\0"
                as *const u8 as *const libc::c_char;
        }
        256 => {
            msg = b"This device has been removed from the system.\0" as *const u8
                as *const libc::c_char;
        }
        257 => {
            msg = b"Device disabled to prevent overheating.\0" as *const u8
                as *const libc::c_char;
        }
        _ => {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Unsupported error, returning general error instead!\0" as *const u8
                    as *const libc::c_char,
            );
            error = FP_DEVICE_ERROR_GENERAL;
            msg = b"An unspecified error occurred!\0" as *const u8
                as *const libc::c_char;
        }
    }
    return g_error_new_literal(fp_device_error_quark(), error as gint, msg);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_retry_new_msg(
    mut device_error: FpDeviceRetry,
    mut msg: *const gchar,
    mut args: ...
) -> *mut GError {
    let mut error: *mut GError = 0 as *mut GError;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    error = g_error_new_valist(
        fp_device_retry_quark(),
        device_error as gint,
        msg,
        args_0.as_va_list(),
    );
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_error_new_msg(
    mut device_error: FpDeviceError,
    mut msg: *const gchar,
    mut args: ...
) -> *mut GError {
    let mut error: *mut GError = 0 as *mut GError;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    error = g_error_new_valist(
        fp_device_error_quark(),
        device_error as gint,
        msg,
        args_0.as_va_list(),
    );
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_set_nr_enroll_stages(
    mut device: *mut FpDevice,
    mut enroll_stages: gint,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"fpi_device_set_nr_enroll_stages\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if enroll_stages > 0 as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"fpi_device_set_nr_enroll_stages\0"))
                .as_ptr(),
            b"enroll_stages > 0\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*priv_0).nr_enroll_stages = enroll_stages;
    g_object_notify(
        g_type_check_instance_cast(
            device as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"nr-enroll-stages\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_set_scan_type(
    mut device: *mut FpDevice,
    mut scan_type: FpScanType,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"fpi_device_set_scan_type\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*priv_0).scan_type = scan_type;
    g_object_notify(
        g_type_check_instance_cast(
            device as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"scan-type\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_update_features(
    mut device: *mut FpDevice,
    mut update: FpDeviceFeature,
    mut value: FpDeviceFeature,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_update_features\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_PROBE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_update_features\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_PROBE\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if value as libc::c_uint & update as libc::c_uint == value as libc::c_uint {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_update_features\0"))
                .as_ptr(),
            b"(value & update) == value\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*priv_0)
        .features = ((*priv_0).features as libc::c_uint & !(update as libc::c_uint)
        | value as libc::c_uint & update as libc::c_uint) as FpDeviceFeature;
}
unsafe extern "C" fn timeout_finalize(mut source: *mut GSource) {
    let mut timeout_source: *mut FpDeviceTimeoutSource = source
        as *mut FpDeviceTimeoutSource;
    let mut priv_0: *mut FpDevicePrivate = 0 as *mut FpDevicePrivate;
    priv_0 = fp_device_get_instance_private((*timeout_source).device)
        as *mut FpDevicePrivate;
    (*priv_0).sources = g_slist_remove((*priv_0).sources, source as gconstpointer);
}
unsafe extern "C" fn timeout_dispatch(
    mut source: *mut GSource,
    mut gsource_func: GSourceFunc,
    mut user_data: gpointer,
) -> gboolean {
    let mut timeout_source: *mut FpDeviceTimeoutSource = source
        as *mut FpDeviceTimeoutSource;
    let mut callback: FpTimeoutFunc = ::core::mem::transmute::<
        GSourceFunc,
        FpTimeoutFunc,
    >(gsource_func);
    callback.expect("non-null function pointer")((*timeout_source).device, user_data);
    return 0 as libc::c_int;
}
static mut timeout_funcs: GSourceFuncs = unsafe {
    {
        let mut init = _GSourceFuncs {
            prepare: None,
            check: None,
            dispatch: Some(
                timeout_dispatch
                    as unsafe extern "C" fn(
                        *mut GSource,
                        GSourceFunc,
                        gpointer,
                    ) -> gboolean,
            ),
            finalize: Some(timeout_finalize as unsafe extern "C" fn(*mut GSource) -> ()),
            closure_callback: None,
            closure_marshal: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn fpi_device_add_timeout(
    mut device: *mut FpDevice,
    mut interval: gint,
    mut func: FpTimeoutFunc,
    mut user_data: gpointer,
    mut destroy_notify: GDestroyNotify,
) -> *mut GSource {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut source: *mut FpDeviceTimeoutSource = 0 as *mut FpDeviceTimeoutSource;
    let mut context: *mut GMainContext = 0 as *mut GMainContext;
    source = g_source_new(
        &mut timeout_funcs,
        ::core::mem::size_of::<FpDeviceTimeoutSource>() as libc::c_ulong as guint,
    ) as *mut FpDeviceTimeoutSource;
    (*source).device = device;
    if !((*priv_0).current_task).is_null() {
        context = g_task_get_context((*priv_0).current_task);
    } else {
        context = g_main_context_get_thread_default();
    }
    g_source_attach(&mut (*source).source, context);
    g_source_set_callback(
        &mut (*source).source,
        ::core::mem::transmute::<FpTimeoutFunc, GSourceFunc>(func),
        user_data,
        destroy_notify,
    );
    g_source_set_ready_time(
        &mut (*source).source,
        (g_source_get_time(&mut (*source).source) as libc::c_ulong)
            .wrapping_add(
                (interval as libc::c_ulong).wrapping_mul(1000 as libc::c_int as guint64),
            ) as gint64,
    );
    (*priv_0).sources = g_slist_prepend((*priv_0).sources, source as gpointer);
    g_source_unref(&mut (*source).source);
    return &mut (*source).source;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_get_usb_device(
    mut device: *mut FpDevice,
) -> *mut GUsbDevice {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"fpi_device_get_usb_device\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut GUsbDevice;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).type_0 as libc::c_uint
            == FP_DEVICE_TYPE_USB as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"fpi_device_get_usb_device\0"))
                .as_ptr(),
            b"priv->type == FP_DEVICE_TYPE_USB\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut GUsbDevice;
    }
    return (*priv_0).usb_device;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_get_udev_data(
    mut device: *mut FpDevice,
    mut subtype: FpiDeviceUdevSubtypeFlags,
) -> gpointer {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"fpi_device_get_udev_data\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_void;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).type_0 as libc::c_uint
            == FP_DEVICE_TYPE_UDEV as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"fpi_device_get_udev_data\0"))
                .as_ptr(),
            b"priv->type == FP_DEVICE_TYPE_UDEV\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_void;
    }
    match subtype as libc::c_uint {
        2 => return (*priv_0).udev_data.hidraw_path as gpointer,
        1 => return (*priv_0).udev_data.spidev_path as gpointer,
        _ => {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"file %s: line %d (%s): should not be reached\0" as *const u8
                    as *const libc::c_char,
                b"../libfprint/fpi-device.c\0" as *const u8 as *const libc::c_char,
                439 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"fpi_device_get_udev_data\0"))
                    .as_ptr(),
            );
            return 0 as *mut libc::c_void;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_get_virtual_env(
    mut device: *mut FpDevice,
) -> *const gchar {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_get_virtual_env\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *const gchar;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).type_0 as libc::c_uint
            == FP_DEVICE_TYPE_VIRTUAL as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_get_virtual_env\0"))
                .as_ptr(),
            b"priv->type == FP_DEVICE_TYPE_VIRTUAL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *const gchar;
    }
    return (*priv_0).virtual_env;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_get_current_action(
    mut device: *mut FpDevice,
) -> FpiDeviceAction {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"fpi_device_get_current_action\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return FPI_DEVICE_ACTION_NONE;
    }
    return (*priv_0).current_action;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_action_is_cancelled(
    mut device: *mut FpDevice,
) -> gboolean {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"fpi_device_action_is_cancelled\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            != FPI_DEVICE_ACTION_NONE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"fpi_device_action_is_cancelled\0"))
                .as_ptr(),
            b"priv->current_action != FPI_DEVICE_ACTION_NONE\0" as *const u8
                as *const libc::c_char,
        );
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    return g_cancellable_is_cancelled((*priv_0).current_cancellable);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_get_driver_data(
    mut device: *mut FpDevice,
) -> guint64 {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_get_driver_data\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int as guint64;
    }
    return (*priv_0).driver_data;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_get_enroll_data(
    mut device: *mut FpDevice,
    mut print: *mut *mut FpPrint,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut data: *mut FpEnrollData = 0 as *mut FpEnrollData;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_get_enroll_data\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_ENROLL as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_get_enroll_data\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_ENROLL\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    data = g_task_get_task_data((*priv_0).current_task) as *mut FpEnrollData;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !data.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-device.c\0" as *const u8 as *const libc::c_char,
            543 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_get_enroll_data\0"))
                .as_ptr(),
            b"data\0" as *const u8 as *const libc::c_char,
        );
    }
    if !print.is_null() {
        *print = (*data).print;
    }
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_get_capture_data(
    mut device: *mut FpDevice,
    mut wait_for_finger: *mut gboolean,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fpi_device_get_capture_data\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
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
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fpi_device_get_capture_data\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_CAPTURE\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if !wait_for_finger.is_null() {
        *wait_for_finger = (*priv_0).wait_for_finger;
    }
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_get_verify_data(
    mut device: *mut FpDevice,
    mut print: *mut *mut FpPrint,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut data: *mut FpMatchData = 0 as *mut FpMatchData;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_get_verify_data\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_get_verify_data\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_VERIFY\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    data = g_task_get_task_data((*priv_0).current_task) as *mut FpMatchData;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !data.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-device.c\0" as *const u8 as *const libc::c_char,
            587 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_get_verify_data\0"))
                .as_ptr(),
            b"data\0" as *const u8 as *const libc::c_char,
        );
    }
    if !print.is_null() {
        *print = (*data).enrolled_print;
    }
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_get_identify_data(
    mut device: *mut FpDevice,
    mut prints: *mut *mut GPtrArray,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut data: *mut FpMatchData = 0 as *mut FpMatchData;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"fpi_device_get_identify_data\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"fpi_device_get_identify_data\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_IDENTIFY\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    data = g_task_get_task_data((*priv_0).current_task) as *mut FpMatchData;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !data.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-device.c\0" as *const u8 as *const libc::c_char,
            618 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"fpi_device_get_identify_data\0"))
                .as_ptr(),
            b"data\0" as *const u8 as *const libc::c_char,
        );
    }
    if !prints.is_null() {
        *prints = (*data).gallery;
    }
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_get_delete_data(
    mut device: *mut FpDevice,
    mut print: *mut *mut FpPrint,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_get_delete_data\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_DELETE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_get_delete_data\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_DELETE\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if !print.is_null() {
        *print = g_task_get_task_data((*priv_0).current_task) as *mut FpPrint;
    }
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_get_cancellable(
    mut device: *mut FpDevice,
) -> *mut GCancellable {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_get_cancellable\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut GCancellable;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            != FPI_DEVICE_ACTION_NONE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_get_cancellable\0"))
                .as_ptr(),
            b"priv->current_action != FPI_DEVICE_ACTION_NONE\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut GCancellable;
    }
    return (*priv_0).current_cancellable;
}
unsafe extern "C" fn emit_removed_on_task_completed(mut device: *mut FpDevice) {
    g_signal_emit_by_name(
        device as gpointer,
        b"removed\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_remove(mut device: *mut FpDevice) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"fpi_device_remove\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).is_removed == 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"fpi_device_remove\0"))
                .as_ptr(),
            b"!priv->is_removed\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*priv_0).is_removed = (0 as libc::c_int == 0) as libc::c_int;
    g_object_notify(
        g_type_check_instance_cast(
            device as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"removed\0" as *const u8 as *const libc::c_char,
    );
    if !((*priv_0).current_task).is_null() {
        g_signal_connect_object(
            (*priv_0).current_task as gpointer,
            b"notify::completed\0" as *const u8 as *const libc::c_char,
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
                GCallback,
            >(
                Some(
                    emit_removed_on_task_completed
                        as unsafe extern "C" fn(*mut FpDevice) -> (),
                ),
            ),
            device as gpointer,
            G_CONNECT_SWAPPED,
        );
    } else {
        g_signal_emit_by_name(
            device as gpointer,
            b"removed\0" as *const u8 as *const libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_action_error(
    mut device: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"fpi_device_action_error\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            != FPI_DEVICE_ACTION_NONE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"fpi_device_action_error\0"))
                .as_ptr(),
            b"priv->current_action != FPI_DEVICE_ACTION_NONE\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if !error.is_null() {
        let mut action_str: *mut libc::c_char = 0 as *mut libc::c_char;
        action_str = g_enum_to_string(
            fpi_device_action_get_type(),
            (*priv_0).current_action as gint,
        );
        g_log(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Device reported generic error (%s) during action; action was: %s\0"
                as *const u8 as *const libc::c_char,
            (*error).message,
            action_str,
        );
    } else {
        g_log(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Device failed to pass an error to generic action error function\0"
                as *const u8 as *const libc::c_char,
        );
        error = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_GENERAL,
            b"Device reported error but did not provide an error condition\0"
                as *const u8 as *const libc::c_char,
        );
    }
    match (*priv_0).current_action as libc::c_uint {
        1 => {
            fpi_device_probe_complete(
                device,
                0 as *const gchar,
                0 as *const gchar,
                error,
            );
        }
        2 => {
            fpi_device_open_complete(device, error);
        }
        3 => {
            fpi_device_close_complete(device, error);
        }
        4 => {
            fpi_device_enroll_complete(device, 0 as *mut FpPrint, error);
        }
        5 => {
            fpi_device_verify_complete(device, error);
        }
        6 => {
            fpi_device_identify_complete(device, error);
        }
        7 => {
            fpi_device_capture_complete(device, 0 as *mut FpImage, error);
        }
        9 => {
            fpi_device_delete_complete(device, error);
        }
        8 => {
            fpi_device_list_complete(device, 0 as *mut GPtrArray, error);
        }
        10 => {
            fpi_device_clear_storage_complete(device, error);
        }
        0 | _ => {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"file %s: line %d (%s): should not be reached\0" as *const u8
                    as *const libc::c_char,
                b"../libfprint/fpi-device.c\0" as *const u8 as *const libc::c_char,
                788 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"fpi_device_action_error\0"))
                    .as_ptr(),
            );
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_critical_enter(mut device: *mut FpDevice) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            != FPI_DEVICE_ACTION_NONE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"fpi_device_critical_enter\0"))
                .as_ptr(),
            b"priv->current_action != FPI_DEVICE_ACTION_NONE\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    (*priv_0)
        .critical_section = ((*priv_0).critical_section as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint) as guint as guint;
    if !((*priv_0).critical_section_flush_source).is_null() {
        g_source_destroy((*priv_0).critical_section_flush_source);
    }
    (*priv_0).critical_section_flush_source = 0 as *mut GSource;
}
unsafe extern "C" fn fpi_device_critical_section_flush_idle_cb(
    mut device: *mut FpDevice,
) -> gboolean {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut cls: *mut FpDeviceClass = FP_DEVICE_GET_CLASS(device as gpointer);
    if (*priv_0).cancel_queued != 0 {
        if (*priv_0).current_action as libc::c_uint
            != FPI_DEVICE_ACTION_NONE as libc::c_int as libc::c_uint
            && ((*priv_0).current_task_idle_return_source).is_null()
        {
            ((*cls).cancel).expect("non-null function pointer")(device);
        }
        (*priv_0).cancel_queued = 0 as libc::c_int;
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    if (*priv_0).suspend_queued != 0 {
        (*priv_0).suspend_queued = 0 as libc::c_int;
        fpi_device_suspend(device);
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    if (*priv_0).resume_queued != 0 {
        (*priv_0).resume_queued = 0 as libc::c_int;
        fpi_device_resume(device);
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    (*priv_0).critical_section_flush_source = 0 as *mut GSource;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_critical_leave(mut device: *mut FpDevice) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            != FPI_DEVICE_ACTION_NONE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"fpi_device_critical_leave\0"))
                .as_ptr(),
            b"priv->current_action != FPI_DEVICE_ACTION_NONE\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).critical_section != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"fpi_device_critical_leave\0"))
                .as_ptr(),
            b"priv->critical_section\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*priv_0)
        .critical_section = ((*priv_0).critical_section as libc::c_uint)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as guint as guint;
    if (*priv_0).critical_section != 0 {
        return;
    }
    if !((*priv_0).critical_section_flush_source).is_null() {
        return;
    }
    (*priv_0).critical_section_flush_source = g_idle_source_new();
    g_source_set_priority(
        (*priv_0).critical_section_flush_source,
        -(100 as libc::c_int),
    );
    g_source_set_callback(
        (*priv_0).critical_section_flush_source,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FpDevice) -> gboolean>,
            GSourceFunc,
        >(
            Some(
                fpi_device_critical_section_flush_idle_cb
                    as unsafe extern "C" fn(*mut FpDevice) -> gboolean,
            ),
        ),
        device as gpointer,
        None,
    );
    g_source_set_name(
        (*priv_0).critical_section_flush_source,
        b"Flush libfprint driver critical section\0" as *const u8 as *const libc::c_char,
    );
    g_source_attach(
        (*priv_0).critical_section_flush_source,
        g_task_get_context((*priv_0).current_task),
    );
    g_source_unref((*priv_0).critical_section_flush_source);
}
unsafe extern "C" fn clear_device_cancel_action(mut device: *mut FpDevice) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut _pp: C2RustUnnamed_9 = C2RustUnnamed_9 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GSource) -> ()>,
        GDestroyNotify,
    >(Some(g_source_destroy as unsafe extern "C" fn(*mut GSource) -> ()));
    _pp
        .in_0 = &mut (*priv_0).current_idle_cancel_source as *mut *mut GSource
        as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    if (*priv_0).current_cancellable_id != 0 {
        g_cancellable_disconnect(
            (*priv_0).current_cancellable,
            (*priv_0).current_cancellable_id,
        );
        (*priv_0).current_cancellable_id = 0 as libc::c_int as gulong;
    }
    if (*priv_0).current_task_cancellable_id != 0 {
        g_cancellable_disconnect(
            g_task_get_cancellable((*priv_0).current_task),
            (*priv_0).current_task_cancellable_id,
        );
        (*priv_0).current_task_cancellable_id = 0 as libc::c_int as gulong;
    }
}
unsafe extern "C" fn fp_device_task_return_in_idle_cb(
    mut user_data: gpointer,
) -> gboolean {
    let mut data: *mut FpDeviceTaskReturnData = user_data as *mut FpDeviceTaskReturnData;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private((*data).device)
        as *mut FpDevicePrivate;
    let mut action_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut action: FpiDeviceAction = FPI_DEVICE_ACTION_NONE;
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut cancellation_reason: GError_autoptr = 0 as GError_autoptr;
    action_str = g_enum_to_string(
        fpi_device_action_get_type(),
        (*priv_0).current_action as gint,
    );
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Completing action %s in idle!\0" as *const u8 as *const libc::c_char,
        action_str,
    );
    task = (if 0 as libc::c_int != 0 {
        (*priv_0).current_task as *mut libc::c_void
    } else {
        g_steal_pointer(&mut (*priv_0).current_task as *mut *mut GTask as gpointer)
    }) as GTask_autoptr;
    action = (*priv_0).current_action;
    (*priv_0).current_action = FPI_DEVICE_ACTION_NONE;
    (*priv_0).current_task_idle_return_source = 0 as *mut GSource;
    let mut _pp: C2RustUnnamed_8 = C2RustUnnamed_8 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp
        .in_0 = &mut (*priv_0).current_cancellable as *mut *mut GCancellable
        as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    cancellation_reason = (if 0 as libc::c_int != 0 {
        (*priv_0).current_cancellation_reason as *mut libc::c_void
    } else {
        g_steal_pointer(
            &mut (*priv_0).current_cancellation_reason as *mut *mut GError as gpointer,
        )
    }) as GError_autoptr;
    fpi_device_update_temp((*data).device, 0 as libc::c_int);
    if action as libc::c_uint == FPI_DEVICE_ACTION_OPEN as libc::c_int as libc::c_uint
        && (*data).type_0 as libc::c_uint
            != FP_DEVICE_TASK_RETURN_ERROR as libc::c_int as libc::c_uint
    {
        (*priv_0).is_open = (0 as libc::c_int == 0) as libc::c_int;
        g_object_notify(
            g_type_check_instance_cast(
                (*data).device as *mut GTypeInstance,
                ((20 as libc::c_int) << 2 as libc::c_int) as GType,
            ) as *mut libc::c_void as *mut GObject,
            b"open\0" as *const u8 as *const libc::c_char,
        );
    } else if action as libc::c_uint
        == FPI_DEVICE_ACTION_CLOSE as libc::c_int as libc::c_uint
    {
        (*priv_0).is_open = 0 as libc::c_int;
        g_object_notify(
            g_type_check_instance_cast(
                (*data).device as *mut GTypeInstance,
                ((20 as libc::c_int) << 2 as libc::c_int) as GType,
            ) as *mut libc::c_void as *mut GObject,
            b"open\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*priv_0).is_removed != 0
        && (action as libc::c_uint
            != FPI_DEVICE_ACTION_OPEN as libc::c_int as libc::c_uint
            || action as libc::c_uint
                == FPI_DEVICE_ACTION_OPEN as libc::c_int as libc::c_uint
                && (*data).type_0 as libc::c_uint
                    == FP_DEVICE_TASK_RETURN_ERROR as libc::c_int as libc::c_uint)
    {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_REMOVED));
        return 0 as libc::c_int;
    }
    match (*data).type_0 as libc::c_uint {
        0 => {
            g_task_return_int(task, (*data).result as glong as gint as gssize);
        }
        1 => {
            g_task_return_boolean(task, (*data).result as gulong as guint as gboolean);
        }
        2 => {
            g_task_return_pointer(
                task,
                if 0 as libc::c_int != 0 {
                    (*data).result
                } else {
                    g_steal_pointer(&mut (*data).result as *mut gpointer as gpointer)
                },
                Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
            );
        }
        3 => {
            g_task_return_pointer(
                task,
                if 0 as libc::c_int != 0 {
                    (*data).result
                } else {
                    g_steal_pointer(&mut (*data).result as *mut gpointer as gpointer)
                },
                ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut GPtrArray) -> ()>,
                    GDestroyNotify,
                >(Some(g_ptr_array_unref as unsafe extern "C" fn(*mut GPtrArray) -> ())),
            );
        }
        4 => {
            if !cancellation_reason.is_null() {
                g_task_set_task_data(task, 0 as *mut libc::c_void, None);
                g_task_return_error(
                    task,
                    (if 0 as libc::c_int != 0 {
                        cancellation_reason as *mut libc::c_void
                    } else {
                        g_steal_pointer(
                            &mut cancellation_reason as *mut GError_autoptr as gpointer,
                        )
                    }) as *mut GError,
                );
            } else {
                g_task_return_error(
                    task,
                    (if 0 as libc::c_int != 0 {
                        (*data).result
                    } else {
                        g_steal_pointer(&mut (*data).result as *mut gpointer as gpointer)
                    }) as *mut GError,
                );
            }
        }
        _ => {
            g_assertion_message_expr(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                b"../libfprint/fpi-device.c\0" as *const u8 as *const libc::c_char,
                1036 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"fp_device_task_return_in_idle_cb\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fpi_device_task_return_data_free(
    mut data: *mut FpDeviceTaskReturnData,
) {
    if !((*data).result).is_null() {
        match (*data).type_0 as libc::c_uint {
            0 | 1 => {}
            2 => {
                let mut _pp: C2RustUnnamed_6 = C2RustUnnamed_6 {
                    in_0: 0 as *mut libc::c_char,
                };
                let mut _p: gpointer = 0 as *mut libc::c_void;
                let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(gpointer) -> ()>,
                    GDestroyNotify,
                >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
                _pp
                    .in_0 = &mut (*data).result as *mut gpointer as *mut *mut GObject
                    as *mut libc::c_char;
                _p = *_pp.out;
                if !_p.is_null() {
                    *_pp.out = 0 as *mut libc::c_void;
                    _destroy.expect("non-null function pointer")(_p);
                }
            }
            3 => {
                let mut _pp_0: C2RustUnnamed_5 = C2RustUnnamed_5 {
                    in_0: 0 as *mut libc::c_char,
                };
                let mut _p_0: gpointer = 0 as *mut libc::c_void;
                let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut GPtrArray) -> ()>,
                    GDestroyNotify,
                >(Some(g_ptr_array_unref as unsafe extern "C" fn(*mut GPtrArray) -> ()));
                _pp_0
                    .in_0 = &mut (*data).result as *mut gpointer as *mut *mut GPtrArray
                    as *mut libc::c_char;
                _p_0 = *_pp_0.out;
                if !_p_0.is_null() {
                    *_pp_0.out = 0 as *mut libc::c_void;
                    _destroy_0.expect("non-null function pointer")(_p_0);
                }
            }
            4 => {
                g_clear_error(&mut (*data).result as *mut gpointer as *mut *mut GError);
            }
            _ => {
                g_assertion_message_expr(
                    b"libfprint-device\0" as *const u8 as *const libc::c_char,
                    b"../libfprint/fpi-device.c\0" as *const u8 as *const libc::c_char,
                    1066 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 33],
                        &[libc::c_char; 33],
                    >(b"fpi_device_task_return_data_free\0"))
                        .as_ptr(),
                    0 as *const libc::c_char,
                );
            }
        }
    }
    g_object_unref((*data).device as gpointer);
    g_free(data as gpointer);
}
unsafe extern "C" fn fpi_device_return_task_in_idle(
    mut device: *mut FpDevice,
    mut return_type: FpDeviceTaskReturnType,
    mut return_data: gpointer,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut data: *mut FpDeviceTaskReturnData = 0 as *mut FpDeviceTaskReturnData;
    data = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<FpDeviceTaskReturnData>()
            as libc::c_ulong;
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
    }) as *mut FpDeviceTaskReturnData;
    (*data).device = g_object_ref(device as gpointer) as *mut FpDevice;
    (*data).type_0 = return_type;
    (*data).result = return_data;
    (*priv_0).current_task_idle_return_source = g_idle_source_new();
    g_source_set_priority(
        (*priv_0).current_task_idle_return_source,
        g_task_get_priority((*priv_0).current_task),
    );
    g_source_set_callback(
        (*priv_0).current_task_idle_return_source,
        Some(
            fp_device_task_return_in_idle_cb
                as unsafe extern "C" fn(gpointer) -> gboolean,
        ),
        data as gpointer,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FpDeviceTaskReturnData) -> ()>,
            GDestroyNotify,
        >(
            Some(
                fpi_device_task_return_data_free
                    as unsafe extern "C" fn(*mut FpDeviceTaskReturnData) -> (),
            ),
        ),
    );
    g_source_attach(
        (*priv_0).current_task_idle_return_source,
        g_task_get_context((*priv_0).current_task),
    );
    g_source_unref((*priv_0).current_task_idle_return_source);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_probe_complete(
    mut device: *mut FpDevice,
    mut device_id: *const gchar,
    mut device_name: *const gchar,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"fpi_device_probe_complete\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_PROBE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"fpi_device_probe_complete\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_PROBE\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Device reported probe completion\0" as *const u8 as *const libc::c_char,
    );
    clear_device_cancel_action(device);
    fpi_device_report_finger_status(device, FP_FINGER_STATUS_NONE);
    if error.is_null() {
        if !device_id.is_null() {
            let mut _pp: C2RustUnnamed_14 = C2RustUnnamed_14 {
                in_0: 0 as *mut libc::c_char,
            };
            let mut _p: gpointer = 0 as *mut libc::c_void;
            let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                GDestroyNotify,
            >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
            _pp.in_0 = &mut (*priv_0).device_id as *mut *mut gchar as *mut libc::c_char;
            _p = *_pp.out;
            if !_p.is_null() {
                *_pp.out = 0 as *mut libc::c_void;
                _destroy.expect("non-null function pointer")(_p);
            }
            (*priv_0).device_id = g_strdup(device_id);
            g_object_notify(
                g_type_check_instance_cast(
                    device as *mut GTypeInstance,
                    ((20 as libc::c_int) << 2 as libc::c_int) as GType,
                ) as *mut libc::c_void as *mut GObject,
                b"device-id\0" as *const u8 as *const libc::c_char,
            );
        }
        if !device_name.is_null() {
            let mut _pp_0: C2RustUnnamed_13 = C2RustUnnamed_13 {
                in_0: 0 as *mut libc::c_char,
            };
            let mut _p_0: gpointer = 0 as *mut libc::c_void;
            let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                GDestroyNotify,
            >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
            _pp_0
                .in_0 = &mut (*priv_0).device_name as *mut *mut gchar
                as *mut libc::c_char;
            _p_0 = *_pp_0.out;
            if !_p_0.is_null() {
                *_pp_0.out = 0 as *mut libc::c_void;
                _destroy_0.expect("non-null function pointer")(_p_0);
            }
            (*priv_0).device_name = g_strdup(device_name);
            g_object_notify(
                g_type_check_instance_cast(
                    device as *mut GTypeInstance,
                    ((20 as libc::c_int) << 2 as libc::c_int) as GType,
                ) as *mut libc::c_void as *mut GObject,
                b"name\0" as *const u8 as *const libc::c_char,
            );
        }
        fpi_device_return_task_in_idle(
            device,
            FP_DEVICE_TASK_RETURN_BOOL,
            (0 as libc::c_int == 0) as libc::c_int as gulong as gpointer,
        );
    } else {
        fpi_device_return_task_in_idle(
            device,
            FP_DEVICE_TASK_RETURN_ERROR,
            error as gpointer,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_open_complete(
    mut device: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"fpi_device_open_complete\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
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
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"fpi_device_open_complete\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_OPEN\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Device reported open completion\0" as *const u8 as *const libc::c_char,
    );
    clear_device_cancel_action(device);
    fpi_device_report_finger_status(device, FP_FINGER_STATUS_NONE);
    if error.is_null() {
        fpi_device_return_task_in_idle(
            device,
            FP_DEVICE_TASK_RETURN_BOOL,
            (0 as libc::c_int == 0) as libc::c_int as gulong as gpointer,
        );
    } else {
        fpi_device_return_task_in_idle(
            device,
            FP_DEVICE_TASK_RETURN_ERROR,
            error as gpointer,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_close_complete(
    mut device: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut nested_error: *mut GError = 0 as *mut GError;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"fpi_device_close_complete\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
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
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"fpi_device_close_complete\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_CLOSE\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Device reported close completion\0" as *const u8 as *const libc::c_char,
    );
    clear_device_cancel_action(device);
    fpi_device_report_finger_status(device, FP_FINGER_STATUS_NONE);
    match (*priv_0).type_0 as libc::c_uint {
        2 => {
            if g_usb_device_close((*priv_0).usb_device, &mut nested_error) == 0 {
                if error.is_null() {
                    error = nested_error;
                }
                fpi_device_return_task_in_idle(
                    device,
                    FP_DEVICE_TASK_RETURN_ERROR,
                    error as gpointer,
                );
                return;
            }
        }
        0 | 1 => {}
        _ => {
            g_assertion_message_expr(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                b"../libfprint/fpi-device.c\0" as *const u8 as *const libc::c_char,
                1212 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"fpi_device_close_complete\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    }
    if error.is_null() {
        fpi_device_return_task_in_idle(
            device,
            FP_DEVICE_TASK_RETURN_BOOL,
            (0 as libc::c_int == 0) as libc::c_int as gulong as gpointer,
        );
    } else {
        fpi_device_return_task_in_idle(
            device,
            FP_DEVICE_TASK_RETURN_ERROR,
            error as gpointer,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_enroll_complete(
    mut device: *mut FpDevice,
    mut print: *mut FpPrint,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_enroll_complete\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_ENROLL as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_enroll_complete\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_ENROLL\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Device reported enroll completion\0" as *const u8 as *const libc::c_char,
    );
    clear_device_cancel_action(device);
    fpi_device_report_finger_status(device, FP_FINGER_STATUS_NONE);
    if error.is_null() {
        if FP_IS_PRINT(print as gpointer) != 0 {
            let mut print_type: FpiPrintType = FPI_PRINT_UNDEFINED;
            let mut finger_str: *mut libc::c_char = 0 as *mut libc::c_char;
            g_object_get(
                print as gpointer,
                b"fpi-type\0" as *const u8 as *const libc::c_char,
                &mut print_type as *mut FpiPrintType,
                0 as *mut libc::c_void,
            );
            if print_type as libc::c_uint
                == FPI_PRINT_UNDEFINED as libc::c_int as libc::c_uint
            {
                g_log(
                    b"libfprint-device\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_WARNING,
                    b"Driver did not set the type on the returned print!\0" as *const u8
                        as *const libc::c_char,
                );
                let mut _pp: C2RustUnnamed_12 = C2RustUnnamed_12 {
                    in_0: 0 as *mut libc::c_char,
                };
                let mut _p: gpointer = 0 as *mut libc::c_void;
                let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(gpointer) -> ()>,
                    GDestroyNotify,
                >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
                _pp.in_0 = &mut print as *mut *mut FpPrint as *mut libc::c_char;
                _p = *_pp.out;
                if !_p.is_null() {
                    *_pp.out = 0 as *mut libc::c_void;
                    _destroy.expect("non-null function pointer")(_p);
                }
                error = fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_GENERAL,
                    b"Driver provided incorrect print data!\0" as *const u8
                        as *const libc::c_char,
                );
                fpi_device_return_task_in_idle(
                    device,
                    FP_DEVICE_TASK_RETURN_ERROR,
                    error as gpointer,
                );
                return;
            }
            finger_str = g_enum_to_string(
                fp_finger_get_type(),
                fp_print_get_finger(print) as gint,
            );
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Print for finger %s enrolled\0" as *const u8 as *const libc::c_char,
                finger_str,
            );
            fpi_device_return_task_in_idle(
                device,
                FP_DEVICE_TASK_RETURN_OBJECT,
                print as gpointer,
            );
        } else {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Driver did not provide a valid print and failed to provide an error!\0"
                    as *const u8 as *const libc::c_char,
            );
            error = fpi_device_error_new_msg(
                FP_DEVICE_ERROR_GENERAL,
                b"Driver failed to provide print data!\0" as *const u8
                    as *const libc::c_char,
            );
            fpi_device_return_task_in_idle(
                device,
                FP_DEVICE_TASK_RETURN_ERROR,
                error as gpointer,
            );
        }
    } else {
        fpi_device_return_task_in_idle(
            device,
            FP_DEVICE_TASK_RETURN_ERROR,
            error as gpointer,
        );
        if FP_IS_PRINT(print as gpointer) != 0 {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Driver passed an error but also provided a print, returning error!\0"
                    as *const u8 as *const libc::c_char,
            );
            g_object_unref(print as gpointer);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_verify_complete(
    mut device: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut data: *mut FpMatchData = 0 as *mut FpMatchData;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_verify_complete\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_verify_complete\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_VERIFY\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Device reported verify completion\0" as *const u8 as *const libc::c_char,
    );
    data = g_task_get_task_data((*priv_0).current_task) as *mut FpMatchData;
    clear_device_cancel_action(device);
    fpi_device_report_finger_status(device, FP_FINGER_STATUS_NONE);
    if error.is_null() {
        if (*data).result_reported == 0 {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Driver reported successful verify complete but did not report the result earlier. Reporting error instead\0"
                    as *const u8 as *const libc::c_char,
            );
            fpi_device_return_task_in_idle(
                device,
                FP_DEVICE_TASK_RETURN_ERROR,
                fpi_device_error_new(FP_DEVICE_ERROR_GENERAL) as gpointer,
            );
        } else if !((*data).error).is_null() {
            fpi_device_return_task_in_idle(
                device,
                FP_DEVICE_TASK_RETURN_ERROR,
                if 0 as libc::c_int != 0 {
                    (*data).error as *mut libc::c_void
                } else {
                    g_steal_pointer(&mut (*data).error as *mut *mut GError as gpointer)
                },
            );
        } else {
            fpi_device_return_task_in_idle(
                device,
                FP_DEVICE_TASK_RETURN_INT,
                (if !((*data).match_0).is_null() {
                    FPI_MATCH_SUCCESS as libc::c_int
                } else {
                    FPI_MATCH_FAIL as libc::c_int
                }) as glong as gpointer,
            );
        }
    } else {
        if (*error).domain == fp_device_retry_quark() {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Driver reported a retry error to fpi_device_verify_complete. This is not permissible and needs to be reported using fpi_device_verify_report, reporting general verification failure instead.\0"
                    as *const u8 as *const libc::c_char,
            );
            g_clear_error(&mut error);
            error = fpi_device_error_new(FP_DEVICE_ERROR_GENERAL);
        }
        fpi_device_return_task_in_idle(
            device,
            FP_DEVICE_TASK_RETURN_ERROR,
            error as gpointer,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_identify_complete(
    mut device: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut data: *mut FpMatchData = 0 as *mut FpMatchData;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"fpi_device_identify_complete\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"fpi_device_identify_complete\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_IDENTIFY\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Device reported identify completion\0" as *const u8 as *const libc::c_char,
    );
    data = g_task_get_task_data((*priv_0).current_task) as *mut FpMatchData;
    clear_device_cancel_action(device);
    fpi_device_report_finger_status(device, FP_FINGER_STATUS_NONE);
    if error.is_null() {
        if (*data).result_reported == 0 {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Driver reported successful identify complete but did not report the result earlier. Reporting error instead\0"
                    as *const u8 as *const libc::c_char,
            );
            fpi_device_return_task_in_idle(
                device,
                FP_DEVICE_TASK_RETURN_ERROR,
                fpi_device_error_new(FP_DEVICE_ERROR_GENERAL) as gpointer,
            );
        } else if !((*data).error).is_null() {
            fpi_device_return_task_in_idle(
                device,
                FP_DEVICE_TASK_RETURN_ERROR,
                if 0 as libc::c_int != 0 {
                    (*data).error as *mut libc::c_void
                } else {
                    g_steal_pointer(&mut (*data).error as *mut *mut GError as gpointer)
                },
            );
        } else {
            fpi_device_return_task_in_idle(
                device,
                FP_DEVICE_TASK_RETURN_BOOL,
                (0 as libc::c_int == 0) as libc::c_int as gulong as gpointer,
            );
        }
    } else {
        if (*error).domain == fp_device_retry_quark() {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Driver reported a retry error to fpi_device_identify_complete. This is not permissible and needs to be reported using fpi_device_identify_report, reporting general identification failure instead.\0"
                    as *const u8 as *const libc::c_char,
            );
            g_clear_error(&mut error);
            error = fpi_device_error_new(FP_DEVICE_ERROR_GENERAL);
        }
        fpi_device_return_task_in_idle(
            device,
            FP_DEVICE_TASK_RETURN_ERROR,
            error as gpointer,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_capture_complete(
    mut device: *mut FpDevice,
    mut image: *mut FpImage,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fpi_device_capture_complete\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
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
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fpi_device_capture_complete\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_CAPTURE\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Device reported capture completion\0" as *const u8 as *const libc::c_char,
    );
    clear_device_cancel_action(device);
    fpi_device_report_finger_status(device, FP_FINGER_STATUS_NONE);
    if error.is_null() {
        if !image.is_null() {
            fpi_device_return_task_in_idle(
                device,
                FP_DEVICE_TASK_RETURN_OBJECT,
                image as gpointer,
            );
        } else {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Driver did not provide an error for a failed capture operation!\0"
                    as *const u8 as *const libc::c_char,
            );
            error = fpi_device_error_new_msg(
                FP_DEVICE_ERROR_GENERAL,
                b"Driver failed to provide an error!\0" as *const u8
                    as *const libc::c_char,
            );
            fpi_device_return_task_in_idle(
                device,
                FP_DEVICE_TASK_RETURN_ERROR,
                error as gpointer,
            );
        }
    } else {
        fpi_device_return_task_in_idle(
            device,
            FP_DEVICE_TASK_RETURN_ERROR,
            error as gpointer,
        );
        if !image.is_null() {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Driver passed an error but also provided an image, returning error!\0"
                    as *const u8 as *const libc::c_char,
            );
            let mut _pp: C2RustUnnamed_11 = C2RustUnnamed_11 {
                in_0: 0 as *mut libc::c_char,
            };
            let mut _p: gpointer = 0 as *mut libc::c_void;
            let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                GDestroyNotify,
            >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
            _pp.in_0 = &mut image as *mut *mut FpImage as *mut libc::c_char;
            _p = *_pp.out;
            if !_p.is_null() {
                *_pp.out = 0 as *mut libc::c_void;
                _destroy.expect("non-null function pointer")(_p);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_delete_complete(
    mut device: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_delete_complete\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_DELETE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_delete_complete\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_DELETE\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Device reported deletion completion\0" as *const u8 as *const libc::c_char,
    );
    clear_device_cancel_action(device);
    fpi_device_report_finger_status(device, FP_FINGER_STATUS_NONE);
    if error.is_null() {
        fpi_device_return_task_in_idle(
            device,
            FP_DEVICE_TASK_RETURN_BOOL,
            (0 as libc::c_int == 0) as libc::c_int as gulong as gpointer,
        );
    } else {
        fpi_device_return_task_in_idle(
            device,
            FP_DEVICE_TASK_RETURN_ERROR,
            error as gpointer,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_list_complete(
    mut device: *mut FpDevice,
    mut prints: *mut GPtrArray,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"fpi_device_list_complete\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_LIST as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"fpi_device_list_complete\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_LIST\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Device reported listing completion\0" as *const u8 as *const libc::c_char,
    );
    clear_device_cancel_action(device);
    fpi_device_report_finger_status(device, FP_FINGER_STATUS_NONE);
    if !prints.is_null() && !error.is_null() {
        g_log(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Driver reported back prints and error, ignoring prints\0" as *const u8
                as *const libc::c_char,
        );
        let mut _pp: C2RustUnnamed_10 = C2RustUnnamed_10 {
            in_0: 0 as *mut libc::c_char,
        };
        let mut _p: gpointer = 0 as *mut libc::c_void;
        let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GPtrArray) -> ()>,
            GDestroyNotify,
        >(Some(g_ptr_array_unref as unsafe extern "C" fn(*mut GPtrArray) -> ()));
        _pp.in_0 = &mut prints as *mut *mut GPtrArray as *mut libc::c_char;
        _p = *_pp.out;
        if !_p.is_null() {
            *_pp.out = 0 as *mut libc::c_void;
            _destroy.expect("non-null function pointer")(_p);
        }
    } else if prints.is_null() && error.is_null() {
        g_log(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Driver did not pass array but failed to provide an error\0" as *const u8
                as *const libc::c_char,
        );
        error = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_GENERAL,
            b"Driver failed to provide a list of prints\0" as *const u8
                as *const libc::c_char,
        );
    }
    if error.is_null() {
        fpi_device_return_task_in_idle(
            device,
            FP_DEVICE_TASK_RETURN_PTR_ARRAY,
            prints as gpointer,
        );
    } else {
        fpi_device_return_task_in_idle(
            device,
            FP_DEVICE_TASK_RETURN_ERROR,
            error as gpointer,
        );
    };
}
unsafe extern "C" fn update_attr(
    mut attr: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut r: gssize = 0;
    let mut buf: [libc::c_char; 50] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    fd = open(attr, 0 as libc::c_int);
    err = -*__errno_location();
    if fd < 0 as libc::c_int {
        return -err;
    }
    r = read(
        fd,
        buf.as_mut_ptr() as *mut libc::c_void,
        (::core::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    err = *__errno_location();
    close(fd);
    if r < 0 as libc::c_int as libc::c_long {
        return -err;
    }
    g_strchomp(buf.as_mut_ptr());
    if g_strcmp0(buf.as_mut_ptr(), value) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    fd = open(attr, 0o1 as libc::c_int | 0o1000 as libc::c_int);
    err = *__errno_location();
    if fd < 0 as libc::c_int {
        return -err;
    }
    r = write(fd, value as *const libc::c_void, strlen(value));
    err = -*__errno_location();
    close(fd);
    if r < 0 as libc::c_int as libc::c_long {
        g_log(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Could not write %s to %s\0" as *const u8 as *const libc::c_char,
            value,
            attr,
        );
        return -err;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn complete_suspend_resume_task(mut device: *mut FpDevice) {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !((*priv_0).suspend_resume_task).is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-device.c\0" as *const u8 as *const libc::c_char,
            1589 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"complete_suspend_resume_task\0"))
                .as_ptr(),
            b"priv->suspend_resume_task\0" as *const u8 as *const libc::c_char,
        );
    }
    task = (if 0 as libc::c_int != 0 {
        (*priv_0).suspend_resume_task as *mut libc::c_void
    } else {
        g_steal_pointer(
            &mut (*priv_0).suspend_resume_task as *mut *mut GTask as gpointer,
        )
    }) as GTask_autoptr;
    g_task_return_boolean(task, (0 as libc::c_int == 0) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_suspend(mut device: *mut FpDevice) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    match (*priv_0).current_action as libc::c_uint {
        0 => {
            fpi_device_suspend_complete(device, 0 as *mut GError);
        }
        4 | 5 | 6 | 7 => {
            if ((*FP_DEVICE_GET_CLASS(device as gpointer)).suspend).is_some() {
                if (*priv_0).critical_section != 0 {
                    (*priv_0).suspend_queued = (0 as libc::c_int == 0) as libc::c_int;
                } else {
                    ((*FP_DEVICE_GET_CLASS(device as gpointer)).suspend)
                        .expect("non-null function pointer")(device);
                }
            } else {
                fpi_device_suspend_complete(
                    device,
                    fpi_device_error_new(FP_DEVICE_ERROR_NOT_SUPPORTED),
                );
            }
        }
        1 | 2 | 3 | 9 | 8 | 10 | _ => {
            g_signal_connect_object(
                (*priv_0).current_task as gpointer,
                b"notify::completed\0" as *const u8 as *const libc::c_char,
                ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
                    GCallback,
                >(
                    Some(
                        complete_suspend_resume_task
                            as unsafe extern "C" fn(*mut FpDevice) -> (),
                    ),
                ),
                device as gpointer,
                G_CONNECT_SWAPPED,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_resume(mut device: *mut FpDevice) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    match (*priv_0).current_action as libc::c_uint {
        0 => {
            fpi_device_resume_complete(device, 0 as *mut GError);
        }
        4 | 5 | 6 | 7 => {
            if ((*FP_DEVICE_GET_CLASS(device as gpointer)).resume).is_some() {
                if (*priv_0).critical_section != 0 {
                    (*priv_0).resume_queued = (0 as libc::c_int == 0) as libc::c_int;
                } else {
                    ((*FP_DEVICE_GET_CLASS(device as gpointer)).resume)
                        .expect("non-null function pointer")(device);
                }
            } else {
                fpi_device_resume_complete(
                    device,
                    fpi_device_error_new(FP_DEVICE_ERROR_NOT_SUPPORTED),
                );
            }
        }
        1 | 2 | 3 | 9 | 8 | 10 | _ => {
            g_assertion_message_expr(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                b"../libfprint/fpi-device.c\0" as *const u8 as *const libc::c_char,
                1680 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 18],
                    &[libc::c_char; 18],
                >(b"fpi_device_resume\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_configure_wakeup(
    mut device: *mut FpDevice,
    mut enabled: gboolean,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    match (*priv_0).type_0 as libc::c_uint {
        2 => {
            let mut ports: GString_autoptr = 0 as GString_autoptr;
            let mut dev: GUsbDevice_autoptr = 0 as GUsbDevice_autoptr;
            let mut wakeup_command: *const libc::c_char = if enabled != 0 {
                b"enabled\0" as *const u8 as *const libc::c_char
            } else {
                b"disabled\0" as *const u8 as *const libc::c_char
            };
            let mut bus: guint8 = 0;
            let mut sysfs_wakeup: *mut gchar = 0 as *mut gchar;
            let mut sysfs_persist: *mut gchar = 0 as *mut gchar;
            let mut res: libc::c_int = 0;
            ports = g_string_new(0 as *const gchar);
            bus = g_usb_device_get_bus((*priv_0).usb_device);
            let mut _object_ptr: C2RustUnnamed_16 = C2RustUnnamed_16 {
                in_0: 0 as *mut libc::c_char,
            };
            _object_ptr.in_0 = &mut dev as *mut GUsbDevice_autoptr as *mut libc::c_char;
            if 0 as libc::c_int != 0 {
                dev = (*priv_0).usb_device;
            } else {};
            g_set_object(_object_ptr.out, (*priv_0).usb_device as *mut GObject);
            while 0 as libc::c_int == 0 {
                let mut parent: GUsbDevice_autoptr = g_usb_device_get_parent(dev);
                let mut port_str: *mut gchar = 0 as *mut gchar;
                let mut port: guint8 = 0;
                if parent.is_null() {
                    break;
                }
                port = g_usb_device_get_port_number(dev);
                port_str = g_strdup_printf(
                    b"%d.\0" as *const u8 as *const libc::c_char,
                    port as libc::c_int,
                );
                g_string_prepend(ports, port_str);
                let mut _object_ptr_0: C2RustUnnamed_15 = C2RustUnnamed_15 {
                    in_0: 0 as *mut libc::c_char,
                };
                _object_ptr_0
                    .in_0 = &mut dev as *mut GUsbDevice_autoptr as *mut libc::c_char;
                if 0 as libc::c_int != 0 {
                    dev = parent;
                } else {};
                g_set_object(_object_ptr_0.out, parent as *mut GObject);
            }
            g_string_set_size(
                ports,
                ((*ports).len).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            sysfs_wakeup = g_strdup_printf(
                b"/sys/bus/usb/devices/%d-%s/power/wakeup\0" as *const u8
                    as *const libc::c_char,
                bus as libc::c_int,
                (*ports).str_0,
            );
            res = update_attr(sysfs_wakeup, wakeup_command);
            if res < 0 as libc::c_int {
                g_log(
                    b"libfprint-device\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"Failed to set %s to %s\0" as *const u8 as *const libc::c_char,
                    sysfs_wakeup,
                    wakeup_command,
                );
            }
            sysfs_persist = g_strdup_printf(
                b"/sys/bus/usb/devices/%d-%s/power/persist\0" as *const u8
                    as *const libc::c_char,
                bus as libc::c_int,
                (*ports).str_0,
            );
            res = update_attr(sysfs_persist, b"0\0" as *const u8 as *const libc::c_char);
            if res < 0 as libc::c_int {
                g_log(
                    b"libfprint-device\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_WARNING,
                    b"Failed to disable USB persist by writing to %s\0" as *const u8
                        as *const libc::c_char,
                    sysfs_persist,
                );
            }
        }
        0 | 1 => {}
        _ => {
            g_assertion_message_expr(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                b"../libfprint/fpi-device.c\0" as *const u8 as *const libc::c_char,
                1747 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"fpi_device_configure_wakeup\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn fpi_device_suspend_completed(mut device: *mut FpDevice) {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if (*priv_0).current_action as libc::c_uint
        != FPI_DEVICE_ACTION_NONE as libc::c_int as libc::c_uint
    {
        fpi_device_configure_wakeup(device, (0 as libc::c_int == 0) as libc::c_int);
    }
    if (*priv_0).critical_section != 0 {
        g_log(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Driver was in a critical section at suspend time. It likely deadlocked!\0"
                as *const u8 as *const libc::c_char,
        );
    }
    task = (if 0 as libc::c_int != 0 {
        (*priv_0).suspend_resume_task as *mut libc::c_void
    } else {
        g_steal_pointer(
            &mut (*priv_0).suspend_resume_task as *mut *mut GTask as gpointer,
        )
    }) as GTask_autoptr;
    if !((*priv_0).suspend_error).is_null() {
        g_task_return_error(
            task,
            (if 0 as libc::c_int != 0 {
                (*priv_0).suspend_error as *mut libc::c_void
            } else {
                g_steal_pointer(
                    &mut (*priv_0).suspend_error as *mut *mut GError as gpointer,
                )
            }) as *mut GError,
        );
    } else {
        g_task_return_boolean(task, (0 as libc::c_int == 0) as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_suspend_complete(
    mut device: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fpi_device_suspend_complete\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !((*priv_0).suspend_resume_task).is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fpi_device_suspend_complete\0"))
                .as_ptr(),
            b"priv->suspend_resume_task\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if ((*priv_0).suspend_error).is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fpi_device_suspend_complete\0"))
                .as_ptr(),
            b"priv->suspend_error == NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*priv_0)
        .suspend_error = (if 0 as libc::c_int != 0 {
        error as *mut libc::c_void
    } else {
        g_steal_pointer(&mut error as *mut *mut GError as gpointer)
    }) as *mut GError;
    (*priv_0).is_suspended = (0 as libc::c_int == 0) as libc::c_int;
    if ((*priv_0).suspend_error).is_null() || ((*priv_0).current_task).is_null()
        || g_task_get_completed((*priv_0).current_task) != 0
    {
        fpi_device_suspend_completed(device);
        return;
    }
    g_signal_connect_object(
        (*priv_0).current_task as gpointer,
        b"notify::completed\0" as *const u8 as *const libc::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
            GCallback,
        >(
            Some(
                fpi_device_suspend_completed as unsafe extern "C" fn(*mut FpDevice) -> (),
            ),
        ),
        device as gpointer,
        G_CONNECT_SWAPPED,
    );
    if ((*priv_0).current_cancellation_reason).is_null() {
        (*priv_0)
            .current_cancellation_reason = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_BUSY,
            b"Cannot run while suspended.\0" as *const u8 as *const libc::c_char,
        );
    }
    g_cancellable_cancel((*priv_0).current_cancellable);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_resume_complete(
    mut device: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_resume_complete\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !((*priv_0).suspend_resume_task).is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_resume_complete\0"))
                .as_ptr(),
            b"priv->suspend_resume_task\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*priv_0).is_suspended = 0 as libc::c_int;
    fpi_device_configure_wakeup(device, 0 as libc::c_int);
    task = (if 0 as libc::c_int != 0 {
        (*priv_0).suspend_resume_task as *mut libc::c_void
    } else {
        g_steal_pointer(
            &mut (*priv_0).suspend_resume_task as *mut *mut GTask as gpointer,
        )
    }) as GTask_autoptr;
    if !error.is_null() {
        g_task_return_error(task, error);
    } else {
        g_task_return_boolean(task, (0 as libc::c_int == 0) as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_clear_storage_complete(
    mut device: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"fpi_device_clear_storage_complete\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_CLEAR_STORAGE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"fpi_device_clear_storage_complete\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_CLEAR_STORAGE\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Device reported deletion completion\0" as *const u8 as *const libc::c_char,
    );
    clear_device_cancel_action(device);
    fpi_device_report_finger_status(device, FP_FINGER_STATUS_NONE);
    if error.is_null() {
        fpi_device_return_task_in_idle(
            device,
            FP_DEVICE_TASK_RETURN_BOOL,
            (0 as libc::c_int == 0) as libc::c_int as gulong as gpointer,
        );
    } else {
        fpi_device_return_task_in_idle(
            device,
            FP_DEVICE_TASK_RETURN_ERROR,
            error as gpointer,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_enroll_progress(
    mut device: *mut FpDevice,
    mut completed_stages: gint,
    mut print: *mut FpPrint,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut data: *mut FpEnrollData = 0 as *mut FpEnrollData;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_enroll_progress\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_ENROLL as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_enroll_progress\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_ENROLL\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if error.is_null() || (*error).domain == fp_device_retry_quark() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_enroll_progress\0"))
                .as_ptr(),
            b"error == NULL || error->domain == FP_DEVICE_RETRY\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Device reported enroll progress, reported %i of %i have been completed\0"
            as *const u8 as *const libc::c_char,
        completed_stages,
        (*priv_0).nr_enroll_stages,
    );
    if !print.is_null() {
        g_object_ref_sink(print as gpointer);
    }
    if !error.is_null() && !print.is_null() {
        g_log(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Driver passed an error and also provided a print, returning error!\0"
                as *const u8 as *const libc::c_char,
        );
        let mut _pp: C2RustUnnamed_18 = C2RustUnnamed_18 {
            in_0: 0 as *mut libc::c_char,
        };
        let mut _p: gpointer = 0 as *mut libc::c_void;
        let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(gpointer) -> ()>,
            GDestroyNotify,
        >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
        _pp.in_0 = &mut print as *mut *mut FpPrint as *mut libc::c_char;
        _p = *_pp.out;
        if !_p.is_null() {
            *_pp.out = 0 as *mut libc::c_void;
            _destroy.expect("non-null function pointer")(_p);
        }
    }
    data = g_task_get_task_data((*priv_0).current_task) as *mut FpEnrollData;
    if ((*data).enroll_progress_cb).is_some() {
        ((*data).enroll_progress_cb)
            .expect(
                "non-null function pointer",
            )(device, completed_stages, print, (*data).enroll_progress_data, error);
    }
    g_clear_error(&mut error);
    let mut _pp_0: C2RustUnnamed_17 = C2RustUnnamed_17 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_0: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp_0.in_0 = &mut print as *mut *mut FpPrint as *mut libc::c_char;
    _p_0 = *_pp_0.out;
    if !_p_0.is_null() {
        *_pp_0.out = 0 as *mut libc::c_void;
        _destroy_0.expect("non-null function pointer")(_p_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_verify_report(
    mut device: *mut FpDevice,
    mut result: FpiMatchResult,
    mut print: *mut FpPrint,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut data: *mut FpMatchData = g_task_get_task_data((*priv_0).current_task)
        as *mut FpMatchData;
    let mut call_cb: gboolean = (0 as libc::c_int == 0) as libc::c_int;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"fpi_device_verify_report\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"fpi_device_verify_report\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_VERIFY\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*data).result_reported == 0 as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"fpi_device_verify_report\0"))
                .as_ptr(),
            b"data->result_reported == FALSE\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*data).result_reported = (0 as libc::c_int == 0) as libc::c_int;
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Device reported verify result\0" as *const u8 as *const libc::c_char,
    );
    if !print.is_null() {
        print = g_object_ref_sink(print as gpointer) as *mut FpPrint;
    }
    if !error.is_null() || result as libc::c_int == FPI_MATCH_ERROR as libc::c_int {
        if result as libc::c_int != FPI_MATCH_ERROR as libc::c_int {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Driver reported an error code without setting match result to error!\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if error.is_null() {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Driver reported an error without specifying a retry code, assuming general retry error!\0"
                    as *const u8 as *const libc::c_char,
            );
            error = fpi_device_retry_new(FP_DEVICE_RETRY_GENERAL);
        }
        if !print.is_null() {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Driver reported a print together with an error!\0" as *const u8
                    as *const libc::c_char,
            );
            let mut _pp: C2RustUnnamed_19 = C2RustUnnamed_19 {
                in_0: 0 as *mut libc::c_char,
            };
            let mut _p: gpointer = 0 as *mut libc::c_void;
            let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                GDestroyNotify,
            >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
            _pp.in_0 = &mut print as *mut *mut FpPrint as *mut libc::c_char;
            _p = *_pp.out;
            if !_p.is_null() {
                *_pp.out = 0 as *mut libc::c_void;
                _destroy.expect("non-null function pointer")(_p);
            }
        }
        (*data).error = error;
        if (*error).domain != fp_device_retry_quark() {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Driver reported a verify error that was not in the retry domain, delaying report!\0"
                    as *const u8 as *const libc::c_char,
            );
            call_cb = 0 as libc::c_int;
        }
    } else {
        if result as libc::c_int == FPI_MATCH_SUCCESS as libc::c_int {
            fpi_device_get_verify_data(device, &mut (*data).match_0);
            g_object_ref((*data).match_0 as gpointer);
        }
        (*data)
            .print = (if 0 as libc::c_int != 0 {
            print as *mut libc::c_void
        } else {
            g_steal_pointer(&mut print as *mut *mut FpPrint as gpointer)
        }) as *mut FpPrint;
    }
    if call_cb != 0 && ((*data).match_cb).is_some() {
        ((*data).match_cb)
            .expect(
                "non-null function pointer",
            )(device, (*data).match_0, (*data).print, (*data).match_data, (*data).error);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_identify_report(
    mut device: *mut FpDevice,
    mut match_0: *mut FpPrint,
    mut print: *mut FpPrint,
    mut error: *mut GError,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut data: *mut FpMatchData = g_task_get_task_data((*priv_0).current_task)
        as *mut FpMatchData;
    let mut call_cb: gboolean = (0 as libc::c_int == 0) as libc::c_int;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_identify_report\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_identify_report\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_IDENTIFY\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*data).result_reported == 0 as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_device_identify_report\0"))
                .as_ptr(),
            b"data->result_reported == FALSE\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*data).result_reported = (0 as libc::c_int == 0) as libc::c_int;
    if !match_0.is_null() {
        g_object_ref(match_0 as gpointer);
    }
    if !print.is_null() {
        print = g_object_ref_sink(print as gpointer) as *mut FpPrint;
    }
    if !match_0.is_null()
        && g_ptr_array_find((*data).gallery, match_0 as gconstpointer, 0 as *mut guint)
            == 0
    {
        g_log(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Driver reported a match to a print that was not in the gallery, ignoring match.\0"
                as *const u8 as *const libc::c_char,
        );
        let mut _pp: C2RustUnnamed_22 = C2RustUnnamed_22 {
            in_0: 0 as *mut libc::c_char,
        };
        let mut _p: gpointer = 0 as *mut libc::c_void;
        let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(gpointer) -> ()>,
            GDestroyNotify,
        >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
        _pp.in_0 = &mut match_0 as *mut *mut FpPrint as *mut libc::c_char;
        _p = *_pp.out;
        if !_p.is_null() {
            *_pp.out = 0 as *mut libc::c_void;
            _destroy.expect("non-null function pointer")(_p);
        }
    }
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Device reported identify result\0" as *const u8 as *const libc::c_char,
    );
    if !error.is_null() {
        if !match_0.is_null() {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Driver reported an error code but also provided a match!\0"
                    as *const u8 as *const libc::c_char,
            );
            let mut _pp_0: C2RustUnnamed_21 = C2RustUnnamed_21 {
                in_0: 0 as *mut libc::c_char,
            };
            let mut _p_0: gpointer = 0 as *mut libc::c_void;
            let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                GDestroyNotify,
            >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
            _pp_0.in_0 = &mut match_0 as *mut *mut FpPrint as *mut libc::c_char;
            _p_0 = *_pp_0.out;
            if !_p_0.is_null() {
                *_pp_0.out = 0 as *mut libc::c_void;
                _destroy_0.expect("non-null function pointer")(_p_0);
            }
        }
        if !print.is_null() {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Driver reported a print together with an error!\0" as *const u8
                    as *const libc::c_char,
            );
            let mut _pp_1: C2RustUnnamed_20 = C2RustUnnamed_20 {
                in_0: 0 as *mut libc::c_char,
            };
            let mut _p_1: gpointer = 0 as *mut libc::c_void;
            let mut _destroy_1: GDestroyNotify = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                GDestroyNotify,
            >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
            _pp_1.in_0 = &mut print as *mut *mut FpPrint as *mut libc::c_char;
            _p_1 = *_pp_1.out;
            if !_p_1.is_null() {
                *_pp_1.out = 0 as *mut libc::c_void;
                _destroy_1.expect("non-null function pointer")(_p_1);
            }
        }
        (*data).error = error;
        if (*error).domain != fp_device_retry_quark() {
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Driver reported a verify error that was not in the retry domain, delaying report!\0"
                    as *const u8 as *const libc::c_char,
            );
            call_cb = 0 as libc::c_int;
        }
    } else {
        if !match_0.is_null() {
            (*data)
                .match_0 = (if 0 as libc::c_int != 0 {
                match_0 as *mut libc::c_void
            } else {
                g_steal_pointer(&mut match_0 as *mut *mut FpPrint as gpointer)
            }) as *mut FpPrint;
        }
        if !print.is_null() {
            (*data)
                .print = (if 0 as libc::c_int != 0 {
                print as *mut libc::c_void
            } else {
                g_steal_pointer(&mut print as *mut *mut FpPrint as gpointer)
            }) as *mut FpPrint;
        }
    }
    if call_cb != 0 && ((*data).match_cb).is_some() {
        ((*data).match_cb)
            .expect(
                "non-null function pointer",
            )(device, (*data).match_0, (*data).print, (*data).match_data, (*data).error);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_report_finger_status(
    mut device: *mut FpDevice,
    mut finger_status: FpFingerStatusFlags,
) -> gboolean {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut status_string: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*priv_0).finger_status as libc::c_uint == finger_status as libc::c_uint {
        return 0 as libc::c_int;
    }
    status_string = g_flags_to_string(
        fp_finger_status_flags_get_type(),
        finger_status as guint,
    );
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Device reported finger status change: %s\0" as *const u8
            as *const libc::c_char,
        status_string,
    );
    (*priv_0).finger_status = finger_status;
    g_object_notify(
        g_type_check_instance_cast(
            device as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        b"finger-status\0" as *const u8 as *const libc::c_char,
    );
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_report_finger_status_changes(
    mut device: *mut FpDevice,
    mut added_status: FpFingerStatusFlags,
    mut removed_status: FpFingerStatusFlags,
) -> gboolean {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut finger_status: FpFingerStatusFlags = (*priv_0).finger_status;
    finger_status = ::core::mem::transmute::<
        libc::c_uint,
        FpFingerStatusFlags,
    >(finger_status as libc::c_uint | added_status as libc::c_uint);
    finger_status = ::core::mem::transmute::<
        libc::c_uint,
        FpFingerStatusFlags,
    >(finger_status as libc::c_uint & !(removed_status as libc::c_uint));
    return fpi_device_report_finger_status(device, finger_status);
}
unsafe extern "C" fn update_temp_timeout(
    mut device: *mut FpDevice,
    mut user_data: gpointer,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    fpi_device_update_temp(device, (*priv_0).temp_last_active);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_update_temp(
    mut device: *mut FpDevice,
    mut is_active: gboolean,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut now: gint64 = g_get_monotonic_time();
    let mut passed_seconds: gdouble = 0.;
    let mut alpha: gdouble = 0.;
    let mut next_threshold: gdouble = 0.;
    let mut old_ratio: gdouble = 0.;
    let mut old_temp: FpTemperature = FP_TEMPERATURE_COLD;
    let mut old_temp_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_temp_str: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*priv_0).temp_hot_seconds < 0 as libc::c_int {
        g_log(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Not updating temperature model, device can run continuously!\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    passed_seconds = (now - (*priv_0).temp_last_update) as libc::c_double / 1e6f64;
    old_ratio = (*priv_0).temp_current_ratio;
    if (*priv_0).temp_last_active != 0 {
        alpha = exp(-passed_seconds / (*priv_0).temp_hot_seconds as libc::c_double);
        (*priv_0)
            .temp_current_ratio = alpha * (*priv_0).temp_current_ratio
            + 1 as libc::c_int as libc::c_double - alpha;
    } else {
        alpha = exp(-passed_seconds / (*priv_0).temp_cold_seconds as libc::c_double);
        (*priv_0).temp_current_ratio = alpha * (*priv_0).temp_current_ratio;
    }
    (*priv_0).temp_last_active = is_active;
    (*priv_0).temp_last_update = now;
    old_temp = (*priv_0).temp_current;
    if (*priv_0).temp_current_ratio < 0.26894142136999512075f64 {
        (*priv_0).temp_current = FP_TEMPERATURE_COLD;
        next_threshold = if is_active != 0 {
            0.26894142136999512075f64
        } else {
            -1.0f64
        };
    } else if (*priv_0).temp_current_ratio < 0.5f64 {
        (*priv_0).temp_current = FP_TEMPERATURE_WARM;
        next_threshold = if is_active != 0 {
            1.0f64 - 0.26894142136999512075f64
        } else {
            0.26894142136999512075f64
        };
    } else if (*priv_0).temp_current_ratio < 1.0f64 - 0.26894142136999512075f64 {
        if (*priv_0).temp_current as libc::c_uint
            != FP_TEMPERATURE_HOT as libc::c_int as libc::c_uint
        {
            (*priv_0).temp_current = FP_TEMPERATURE_WARM;
        }
        next_threshold = if is_active != 0 {
            1.0f64 - 0.26894142136999512075f64
        } else {
            0.5f64
        };
    } else {
        (*priv_0).temp_current = FP_TEMPERATURE_HOT;
        next_threshold = if is_active != 0 { -1.0f64 } else { 0.5f64 };
    }
    old_temp_str = g_enum_to_string(fp_temperature_get_type(), old_temp as gint);
    new_temp_str = g_enum_to_string(
        fp_temperature_get_type(),
        (*priv_0).temp_current as gint,
    );
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Updated temperature model after %0.2f seconds, ratio %0.2f -> %0.2f, active %d -> %d, %s -> %s\0"
            as *const u8 as *const libc::c_char,
        passed_seconds,
        old_ratio,
        (*priv_0).temp_current_ratio,
        (*priv_0).temp_last_active,
        is_active,
        old_temp_str,
        new_temp_str,
    );
    if (*priv_0).temp_current as libc::c_uint != old_temp as libc::c_uint {
        g_object_notify(
            g_type_check_instance_cast(
                device as *mut GTypeInstance,
                ((20 as libc::c_int) << 2 as libc::c_int) as GType,
            ) as *mut libc::c_void as *mut GObject,
            b"temperature\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*priv_0).temp_current as libc::c_uint
        == FP_TEMPERATURE_HOT as libc::c_int as libc::c_uint
    {
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_ENROLL as libc::c_int as libc::c_uint
            || (*priv_0).current_action as libc::c_uint
                == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
            || (*priv_0).current_action as libc::c_uint
                == FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
            || (*priv_0).current_action as libc::c_uint
                == FPI_DEVICE_ACTION_CAPTURE as libc::c_int as libc::c_uint
        {
            if ((*priv_0).current_cancellation_reason).is_null() {
                (*priv_0)
                    .current_cancellation_reason = fpi_device_error_new(
                    FP_DEVICE_ERROR_TOO_HOT,
                );
            }
            g_cancellable_cancel((*priv_0).current_cancellable);
        }
    }
    let mut _pp: C2RustUnnamed_7 = C2RustUnnamed_7 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GSource) -> ()>,
        GDestroyNotify,
    >(Some(g_source_destroy as unsafe extern "C" fn(*mut GSource) -> ()));
    _pp.in_0 = &mut (*priv_0).temp_timeout as *mut *mut GSource as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    if next_threshold < 0 as libc::c_int as libc::c_double {
        return;
    }
    if is_active != 0 {
        passed_seconds = -(*priv_0).temp_hot_seconds as libc::c_double
            * log((next_threshold - 1.0f64) / ((*priv_0).temp_current_ratio - 1.0f64));
    } else {
        passed_seconds = -(*priv_0).temp_cold_seconds as libc::c_double
            * log(next_threshold / (*priv_0).temp_current_ratio);
    }
    passed_seconds += 0.1f64;
    (*priv_0)
        .temp_timeout = fpi_device_add_timeout(
        device,
        (passed_seconds * 1000 as libc::c_int as libc::c_double) as gint,
        Some(update_temp_timeout as unsafe extern "C" fn(*mut FpDevice, gpointer) -> ()),
        0 as *mut libc::c_void,
        None,
    );
}
