use ::libc;
extern "C" {
    pub type _GData;
    pub type _GCancellablePrivate;
    pub type _FpiSsm;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_new(
        domain: GQuark,
        code: gint,
        format: *const gchar,
        _: ...
    ) -> *mut GError;
    fn g_error_free(error: *mut GError);
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
    fn abs(_: libc::c_int) -> libc::c_int;
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_nth(list: *mut GSList, n: guint) -> *mut GSList;
    fn g_slist_length(list: *mut GSList) -> guint;
    fn g_get_monotonic_time() -> gint64;
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn g_log_structured(log_domain: *const gchar, log_level: GLogLevelFlags, _: ...);
    fn __errno_location() -> *mut libc::c_int;
    fn g_strcmp0(str1: *const libc::c_char, str2: *const libc::c_char) -> libc::c_int;
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
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
    fn g_type_check_instance_cast(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> *mut GTypeInstance;
    fn g_type_check_class_cast(
        g_class: *mut GTypeClass,
        is_a_type: GType,
    ) -> *mut GTypeClass;
    fn g_cancellable_set_error_if_cancelled(
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn fpi_ssm_spi_transfer_cb(
        transfer: *mut FpiSpiTransfer,
        device: *mut FpDevice,
        unused_data: gpointer,
        error: *mut GError,
    );
    fn fpi_ssm_silence_debug(machine: *mut FpiSsm);
    fn fpi_ssm_get_cur_state(machine: *mut FpiSsm) -> libc::c_int;
    fn fpi_ssm_get_data(machine: *mut FpiSsm) -> gpointer;
    fn fpi_ssm_set_data(
        machine: *mut FpiSsm,
        ssm_data: gpointer,
        ssm_data_destroy: GDestroyNotify,
    );
    fn fpi_ssm_mark_failed(machine: *mut FpiSsm, error: *mut GError);
    fn fpi_ssm_mark_completed(machine: *mut FpiSsm);
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn fpi_do_movement_estimation(ctx: *mut fpi_frame_asmbl_ctx, stripes: *mut GSList);
    fn fpi_assemble_frames(
        ctx: *mut fpi_frame_asmbl_ctx,
        stripes: *mut GSList,
    ) -> *mut FpImage;
    fn fp_device_get_type() -> GType;
    fn fpi_device_get_udev_data(
        device: *mut FpDevice,
        subtype: FpiDeviceUdevSubtypeFlags,
    ) -> gpointer;
    fn fpi_device_get_current_action(device: *mut FpDevice) -> FpiDeviceAction;
    fn fpi_device_action_is_cancelled(device: *mut FpDevice) -> gboolean;
    fn fpi_device_retry_new_msg(
        error: FpDeviceRetry,
        msg: *const gchar,
        _: ...
    ) -> *mut GError;
    fn fpi_device_error_new_msg(
        error: FpDeviceError,
        msg: *const gchar,
        _: ...
    ) -> *mut GError;
    fn fpi_device_get_driver_data(device: *mut FpDevice) -> guint64;
    fn fpi_device_get_cancellable(device: *mut FpDevice) -> *mut GCancellable;
    fn fp_image_device_get_type() -> GType;
    fn fpi_image_device_session_error(self_0: *mut FpImageDevice, error: *mut GError);
    fn fpi_image_device_open_complete(self_0: *mut FpImageDevice, error: *mut GError);
    fn fpi_image_device_close_complete(self_0: *mut FpImageDevice, error: *mut GError);
    fn fpi_image_device_activate_complete(
        self_0: *mut FpImageDevice,
        error: *mut GError,
    );
    fn fpi_image_device_deactivate_complete(
        self_0: *mut FpImageDevice,
        error: *mut GError,
    );
    fn fpi_image_device_report_finger_status(
        self_0: *mut FpImageDevice,
        present: gboolean,
    );
    fn fpi_image_device_image_captured(self_0: *mut FpImageDevice, image: *mut FpImage);
    fn fpi_image_device_retry_scan(self_0: *mut FpImageDevice, retry: FpDeviceRetry);
    fn fpi_image_resize(
        orig: *mut FpImage,
        w_factor: guint,
        h_factor: guint,
    ) -> *mut FpImage;
    fn fpi_spi_transfer_new(
        device: *mut FpDevice,
        spidev_fd: libc::c_int,
    ) -> *mut FpiSpiTransfer;
    fn fpi_spi_transfer_write(transfer: *mut FpiSpiTransfer, length: gsize);
    fn fpi_spi_transfer_read(transfer: *mut FpiSpiTransfer, length: gsize);
    fn fpi_spi_transfer_read_full(
        transfer: *mut FpiSpiTransfer,
        buffer: *mut guint8,
        length: gsize,
        free_func: GDestroyNotify,
    );
    fn fpi_spi_transfer_submit(
        transfer: *mut FpiSpiTransfer,
        cancellable: *mut GCancellable,
        callback: FpiSpiTransferCallback,
        user_data: gpointer,
    );
    fn fpi_ssm_new_full(
        dev: *mut FpDevice,
        handler: FpiSsmHandlerCallback,
        nr_states: libc::c_int,
        start_cleanup: libc::c_int,
        machine_name: *const libc::c_char,
    ) -> *mut FpiSsm;
    fn fpi_ssm_start(ssm: *mut FpiSsm, callback: FpiSsmCompletedCallback);
    fn fpi_ssm_start_subsm(parent: *mut FpiSsm, child: *mut FpiSsm);
    fn fpi_ssm_next_state(machine: *mut FpiSsm);
    fn fpi_ssm_jump_to_state(machine: *mut FpiSsm, state: libc::c_int);
    fn fpi_ssm_next_state_delayed(machine: *mut FpiSsm, delay: libc::c_int);
    fn fpi_ssm_jump_to_state_delayed(
        machine: *mut FpiSsm,
        state: libc::c_int,
        delay: libc::c_int,
    );
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type guint8 = libc::c_uchar;
pub type guint16 = libc::c_ushort;
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
pub type guchar = libc::c_uchar;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub type GIOErrorEnum = libc::c_uint;
pub const G_IO_ERROR_NO_SUCH_DEVICE: GIOErrorEnum = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: GIOErrorEnum = 46;
pub const G_IO_ERROR_NOT_CONNECTED: GIOErrorEnum = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: GIOErrorEnum = 44;
pub const G_IO_ERROR_BROKEN_PIPE: GIOErrorEnum = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: GIOErrorEnum = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: GIOErrorEnum = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: GIOErrorEnum = 41;
pub const G_IO_ERROR_PROXY_FAILED: GIOErrorEnum = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: GIOErrorEnum = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: GIOErrorEnum = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: GIOErrorEnum = 37;
pub const G_IO_ERROR_DBUS_ERROR: GIOErrorEnum = 36;
pub const G_IO_ERROR_INVALID_DATA: GIOErrorEnum = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: GIOErrorEnum = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: GIOErrorEnum = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: GIOErrorEnum = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: GIOErrorEnum = 31;
pub const G_IO_ERROR_FAILED_HANDLED: GIOErrorEnum = 30;
pub const G_IO_ERROR_WOULD_MERGE: GIOErrorEnum = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: GIOErrorEnum = 28;
pub const G_IO_ERROR_WOULD_BLOCK: GIOErrorEnum = 27;
pub const G_IO_ERROR_BUSY: GIOErrorEnum = 26;
pub const G_IO_ERROR_WOULD_RECURSE: GIOErrorEnum = 25;
pub const G_IO_ERROR_TIMED_OUT: GIOErrorEnum = 24;
pub const G_IO_ERROR_WRONG_ETAG: GIOErrorEnum = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: GIOErrorEnum = 22;
pub const G_IO_ERROR_READ_ONLY: GIOErrorEnum = 21;
pub const G_IO_ERROR_PENDING: GIOErrorEnum = 20;
pub const G_IO_ERROR_CANCELLED: GIOErrorEnum = 19;
pub const G_IO_ERROR_CLOSED: GIOErrorEnum = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: GIOErrorEnum = 17;
pub const G_IO_ERROR_NOT_MOUNTED: GIOErrorEnum = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: GIOErrorEnum = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: GIOErrorEnum = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: GIOErrorEnum = 13;
pub const G_IO_ERROR_NO_SPACE: GIOErrorEnum = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: GIOErrorEnum = 11;
pub const G_IO_ERROR_INVALID_FILENAME: GIOErrorEnum = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: GIOErrorEnum = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: GIOErrorEnum = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: GIOErrorEnum = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: GIOErrorEnum = 6;
pub const G_IO_ERROR_NOT_EMPTY: GIOErrorEnum = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: GIOErrorEnum = 4;
pub const G_IO_ERROR_IS_DIRECTORY: GIOErrorEnum = 3;
pub const G_IO_ERROR_EXISTS: GIOErrorEnum = 2;
pub const G_IO_ERROR_NOT_FOUND: GIOErrorEnum = 1;
pub const G_IO_ERROR_FAILED: GIOErrorEnum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
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
pub type FpImage_autoptr = *mut FpImage;
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
pub type FpiSsm = _FpiSsm;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiSpiTransfer {
    pub device: *mut FpDevice,
    pub ssm: *mut FpiSsm,
    pub length_wr: gssize,
    pub length_rd: gssize,
    pub buffer_wr: *mut guchar,
    pub buffer_rd: *mut guchar,
    pub ref_count: guint,
    pub spidev_fd: libc::c_int,
    pub user_data: gpointer,
    pub callback: FpiSpiTransferCallback,
    pub free_buffer_wr: GDestroyNotify,
    pub free_buffer_rd: GDestroyNotify,
}
pub type FpiSpiTransferCallback = Option::<
    unsafe extern "C" fn(*mut FpiSpiTransfer, *mut FpDevice, gpointer, *mut GError) -> (),
>;
pub type FpiSpiTransfer = _FpiSpiTransfer;
pub type FpiSsmCompletedCallback = Option::<
    unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
>;
pub type FpiSsmHandlerCallback = Option::<
    unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elanspi_sensor_entry {
    pub sensor_id: libc::c_uchar,
    pub height: libc::c_uchar,
    pub width: libc::c_uchar,
    pub ic_version: libc::c_uchar,
    pub is_otp_model: gboolean,
    pub name: *const gchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elanspi_reg_entry {
    pub addr: libc::c_uchar,
    pub value: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elanspi_regtable {
    pub other: *const elanspi_reg_entry,
    pub entries: [C2RustUnnamed_4; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub sid: libc::c_uchar,
    pub table: *const elanspi_reg_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceElanSpi {
    pub parent: FpImageDevice,
    pub sensor_width: guint8,
    pub sensor_height: guint8,
    pub sensor_ic_version: guint8,
    pub sensor_id: guint8,
    pub sensor_otp: gboolean,
    pub sensor_vcm_mode: guint8,
    pub frame_width: guint8,
    pub frame_height: guint8,
    pub sensor_raw_version: guint8,
    pub sensor_reg_17: guint8,
    pub sensor_reg_vref1: guint8,
    pub sensor_reg_28: guint8,
    pub sensor_reg_27: guint8,
    pub sensor_reg_dac2: guint8,
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub sensor_status: guint8,
    pub capture_timeout: gint64,
    pub bg_image: *mut guint16,
    pub last_image: *mut guint16,
    pub prev_frame_image: *mut guint16,
    pub fp_empty_counter: gint,
    pub fp_frame_list: *mut GSList,
    pub finger_wait_debounce: gint,
    pub deactivating: gboolean,
    pub capturing: gboolean,
    pub spi_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub old_data: C2RustUnnamed_7,
    pub hv_data: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub gdac_value: guint16,
    pub gdac_step: guint16,
    pub best_gdac: guint16,
    pub best_meandiff: guint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub dac_value: guint8,
    pub line_ptr: guint8,
    pub dacfine_retry: guint8,
    pub otp_timeout: gint64,
}
pub type FpiDeviceElanSpi = _FpiDeviceElanSpi;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiDeviceElanSpiClass {
    pub parent_class: FpImageDeviceClass,
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
pub const ELANSPI_FPCAPT_NSTATES: elanspi_fp_capture_state = 7;
pub const ELANSPI_FPCAPT_WAITUP_CAPTURE: elanspi_fp_capture_state = 5;
pub type elanspi_guess_result = libc::c_uint;
pub const ELANSPI_GUESS_UNKNOWN: elanspi_guess_result = 2;
pub const ELANSPI_GUESS_EMPTY: elanspi_guess_result = 1;
pub const ELANSPI_GUESS_FINGERPRINT: elanspi_guess_result = 0;
pub const ELANSPI_FPCAPT_WAITUP_PROCESS: elanspi_fp_capture_state = 6;
pub const ELANSPI_FPCAPT_FP_CAPTURE: elanspi_fp_capture_state = 3;
pub const ELANSPI_FPCAPT_FP_PROCESS: elanspi_fp_capture_state = 4;
pub const ELANSPI_FPCAPT_WAITDOWN_CAPTURE: elanspi_fp_capture_state = 1;
pub const ELANSPI_FPCAPT_WAITDOWN_PROCESS: elanspi_fp_capture_state = 2;
pub const ELANSPI_CAPTOLD_NSTATES: elanspi_capture_old_state = 3;
pub const ELANSPI_CAPTOLD_CHECK_LINEREADY: elanspi_capture_old_state = 1;
pub const ELANSPI_CAPTOLD_RECV_LINE: elanspi_capture_old_state = 2;
pub const ELANSPI_CAPTOLD_WRITE_CAPTURE: elanspi_capture_old_state = 0;
pub const ELANSPI_CAPTHV_NSTATES: elanspi_capture_hv_state = 3;
pub const ELANSPI_CAPTHV_CHECK_READY: elanspi_capture_hv_state = 1;
pub const ELANSPI_CAPTHV_RECV_IMAGE: elanspi_capture_hv_state = 2;
pub const ELANSPI_CAPTHV_WRITE_CAPTURE: elanspi_capture_hv_state = 0;
pub const ELANSPI_FPCAPT_INIT: elanspi_fp_capture_state = 0;
pub const ELANSPI_INIT_NSTATES: elanspi_init_state = 22;
pub const ELANSPI_INIT_BG_SAVE: elanspi_init_state = 21;
pub const ELANSPI_INIT_BG_CAPTURE: elanspi_init_state = 20;
pub const ELANSPI_CALIBOLD_PROTECT: elanspi_calibrate_old_state = 11;
pub const ELANSPI_CALIBOLD_NSTATES: elanspi_calibrate_old_state = 12;
pub const ELANSPI_CALIBOLD_DACFINE_CAPTURE: elanspi_calibrate_old_state = 8;
pub const ELANSPI_CALIBOLD_DACFINE_LOOP: elanspi_calibrate_old_state = 10;
pub const ELANSPI_CALIBOLD_DACFINE_WRITE_DAC1: elanspi_calibrate_old_state = 9;
pub const ELANSPI_CALIBOLD_WRITE_GAIN: elanspi_calibrate_old_state = 7;
pub const ELANSPI_CALIBOLD_DACBASE_WRITE_DAC1: elanspi_calibrate_old_state = 5;
pub const ELANSPI_CALIBOLD_CHECKFIN_CAPTURE: elanspi_calibrate_old_state = 6;
pub const ELANSPI_CALIBOLD_DACBASE_CAPTURE: elanspi_calibrate_old_state = 4;
pub const ELANSPI_WRTABLE_NSTATES: elanspi_write_regtable_state = 2;
pub const ELANSPI_WRTABLE_WRITE: elanspi_write_regtable_state = 0;
pub const ELANSPI_WRTABLE_ITERATE: elanspi_write_regtable_state = 1;
pub const ELANSPI_CALIBOLD_SEND_REGTABLE: elanspi_calibrate_old_state = 3;
pub const ELANSPI_CALIBOLD_STARTCALIBDELAY: elanspi_calibrate_old_state = 2;
pub const ELANSPI_CALIBOLD_WRITE_STARTCALIB: elanspi_calibrate_old_state = 1;
pub const ELANSPI_CALIBOLD_UNPROTECT: elanspi_calibrate_old_state = 0;
pub const ELANSPI_CALIBHV_PROTECT: elanspi_calibrate_hv_state = 13;
pub const ELANSPI_CALIBHV_NSTATES: elanspi_calibrate_hv_state = 14;
pub const ELANSPI_CALIBHV_WRITE_GDAC_H: elanspi_calibrate_hv_state = 7;
pub const ELANSPI_CALIBHV_WRITE_BEST_GDAC_H: elanspi_calibrate_hv_state = 11;
pub const ELANSPI_CALIBHV_PROCESS: elanspi_calibrate_hv_state = 10;
pub const ELANSPI_CALIBHV_CAPTURE: elanspi_calibrate_hv_state = 9;
pub const ELANSPI_CALIBHV_WRITE_BEST_GDAC_L: elanspi_calibrate_hv_state = 12;
pub const ELANSPI_CALIBHV_WRITE_GDAC_L: elanspi_calibrate_hv_state = 8;
pub const ELANSPI_CALIBHV_SEND_REGTABLE1: elanspi_calibrate_hv_state = 5;
pub const ELANSPI_CALIBHV_SELECT_PAGE1: elanspi_calibrate_hv_state = 4;
pub const ELANSPI_CALIBHV_SEND_REGTABLE0: elanspi_calibrate_hv_state = 3;
pub const ELANSPI_CALIBHV_UNPROTECT: elanspi_calibrate_hv_state = 2;
pub const ELANSPI_CALIBHV_WRITE_STARTCALIB: elanspi_calibrate_hv_state = 1;
pub const ELANSPI_CALIBHV_SELECT_PAGE0_1: elanspi_calibrate_hv_state = 6;
pub const ELANSPI_CALIBHV_SELECT_PAGE0_0: elanspi_calibrate_hv_state = 0;
pub const ELANSPI_INIT_CALIBRATE: elanspi_init_state = 19;
pub const ELANSPI_INIT_OTP_WRITE_0xc: elanspi_init_state = 18;
pub const ELANSPI_INIT_OTP_WRITE_0xb: elanspi_init_state = 17;
pub const ELANSPI_INIT_OTP_LOOP_UPDATEDAC_WRITE_10: elanspi_init_state = 16;
pub const ELANSPI_INIT_OTP_LOOP_UPDATEDAC_WRITE_DAC2: elanspi_init_state = 15;
pub const ELANSPI_INIT_OTP_LOOP_READ_0x28: elanspi_init_state = 12;
pub const ELANSPI_INIT_OTP_LOOP_UPDATEDAC_READ_DAC2: elanspi_init_state = 14;
pub const ELANSPI_INIT_OTP_LOOP_READ_0x27: elanspi_init_state = 13;
pub const ELANSPI_INIT_OTP_WRITE_0x28: elanspi_init_state = 11;
pub const ELANSPI_INIT_OTP_WRITE_VREF1: elanspi_init_state = 10;
pub const ELANSPI_INIT_OTP_READ_VREF1: elanspi_init_state = 9;
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
pub const ELANSPI_INIT_SWRESET2: elanspi_init_state = 7;
pub const ELANSPI_INIT_READ_VERSION: elanspi_init_state = 6;
pub const ELANSPI_INIT_READ_REG17: elanspi_init_state = 5;
pub const ELANSPI_INIT_READ_WIDTH: elanspi_init_state = 4;
pub const ELANSPI_INIT_READ_HEIGHT: elanspi_init_state = 3;
pub const ELANSPI_INIT_SWRESETDELAY2: elanspi_init_state = 8;
pub const ELANSPI_INIT_SWRESETDELAY1: elanspi_init_state = 2;
pub const ELANSPI_INIT_HWSWRESET: elanspi_init_state = 1;
pub const ELANSPI_INIT_READ_STATUS1: elanspi_init_state = 0;
pub type elanspi_init_state = libc::c_uint;
pub type elanspi_calibrate_old_state = libc::c_uint;
pub type elanspi_capture_old_state = libc::c_uint;
pub type elanspi_calibrate_hv_state = libc::c_uint;
pub type elanspi_capture_hv_state = libc::c_uint;
pub type elanspi_write_regtable_state = libc::c_uint;
pub type elanspi_fp_capture_state = libc::c_uint;
#[inline]
unsafe extern "C" fn g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = 0 as *mut libc::c_void;
    ref_0 = *ptr;
    *ptr = 0 as *mut libc::c_void;
    return ref_0;
}
#[inline]
unsafe extern "C" fn FP_DEVICE(mut ptr: gpointer) -> *mut FpDevice {
    return g_type_check_instance_cast(ptr as *mut GTypeInstance, fp_device_get_type())
        as *mut libc::c_void as *mut FpDevice;
}
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
unsafe extern "C" fn FP_IMAGE_DEVICE_CLASS(
    mut ptr: gpointer,
) -> *mut FpImageDeviceClass {
    return g_type_check_class_cast(ptr as *mut GTypeClass, fp_image_device_get_type())
        as *mut libc::c_void as *mut FpImageDeviceClass;
}
static mut elanspi_sensor_table: [elanspi_sensor_entry; 16] = [
    {
        let mut init = elanspi_sensor_entry {
            sensor_id: 0 as libc::c_int as libc::c_uchar,
            height: 0x78 as libc::c_int as libc::c_uchar,
            width: 0x78 as libc::c_int as libc::c_uchar,
            ic_version: 0 as libc::c_int as libc::c_uchar,
            is_otp_model: 0 as libc::c_int,
            name: b"eFSA120S\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = elanspi_sensor_entry {
            sensor_id: 0x1 as libc::c_int as libc::c_uchar,
            height: 0x78 as libc::c_int as libc::c_uchar,
            width: 0x78 as libc::c_int as libc::c_uchar,
            ic_version: 0x1 as libc::c_int as libc::c_uchar,
            is_otp_model: 0x1 as libc::c_int,
            name: b"eFSA120SA\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = elanspi_sensor_entry {
            sensor_id: 0x2 as libc::c_int as libc::c_uchar,
            height: 0xa0 as libc::c_int as libc::c_uchar,
            width: 0xa0 as libc::c_int as libc::c_uchar,
            ic_version: 0 as libc::c_int as libc::c_uchar,
            is_otp_model: 0 as libc::c_int,
            name: b"eFSA160S\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = elanspi_sensor_entry {
            sensor_id: 0x3 as libc::c_int as libc::c_uchar,
            height: 0xd0 as libc::c_int as libc::c_uchar,
            width: 0x50 as libc::c_int as libc::c_uchar,
            ic_version: 0 as libc::c_int as libc::c_uchar,
            is_otp_model: 0 as libc::c_int,
            name: b"eFSA820R\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = elanspi_sensor_entry {
            sensor_id: 0x4 as libc::c_int as libc::c_uchar,
            height: 0xc0 as libc::c_int as libc::c_uchar,
            width: 0x38 as libc::c_int as libc::c_uchar,
            ic_version: 0 as libc::c_int as libc::c_uchar,
            is_otp_model: 0 as libc::c_int,
            name: b"eFSA519R\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = elanspi_sensor_entry {
            sensor_id: 0x5 as libc::c_int as libc::c_uchar,
            height: 0x60 as libc::c_int as libc::c_uchar,
            width: 0x60 as libc::c_int as libc::c_uchar,
            ic_version: 0 as libc::c_int as libc::c_uchar,
            is_otp_model: 0 as libc::c_int,
            name: b"eFSA96S\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = elanspi_sensor_entry {
            sensor_id: 0x6 as libc::c_int as libc::c_uchar,
            height: 0x60 as libc::c_int as libc::c_uchar,
            width: 0x60 as libc::c_int as libc::c_uchar,
            ic_version: 0x1 as libc::c_int as libc::c_uchar,
            is_otp_model: 0x1 as libc::c_int,
            name: b"eFSA96SA\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = elanspi_sensor_entry {
            sensor_id: 0x7 as libc::c_int as libc::c_uchar,
            height: 0x60 as libc::c_int as libc::c_uchar,
            width: 0x60 as libc::c_int as libc::c_uchar,
            ic_version: 0x2 as libc::c_int as libc::c_uchar,
            is_otp_model: 0x1 as libc::c_int,
            name: b"eFSA96SB\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = elanspi_sensor_entry {
            sensor_id: 0x8 as libc::c_int as libc::c_uchar,
            height: 0xa0 as libc::c_int as libc::c_uchar,
            width: 0x50 as libc::c_int as libc::c_uchar,
            ic_version: 0x1 as libc::c_int as libc::c_uchar,
            is_otp_model: 0x1 as libc::c_int,
            name: b"eFSA816RA\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = elanspi_sensor_entry {
            sensor_id: 0x9 as libc::c_int as libc::c_uchar,
            height: 0x90 as libc::c_int as libc::c_uchar,
            width: 0x40 as libc::c_int as libc::c_uchar,
            ic_version: 0x1 as libc::c_int as libc::c_uchar,
            is_otp_model: 0x1 as libc::c_int,
            name: b"eFSA614RA\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = elanspi_sensor_entry {
            sensor_id: 0xa as libc::c_int as libc::c_uchar,
            height: 0x90 as libc::c_int as libc::c_uchar,
            width: 0x40 as libc::c_int as libc::c_uchar,
            ic_version: 0x2 as libc::c_int as libc::c_uchar,
            is_otp_model: 0x1 as libc::c_int,
            name: b"eFSA614RB\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = elanspi_sensor_entry {
            sensor_id: 0xb as libc::c_int as libc::c_uchar,
            height: 0x40 as libc::c_int as libc::c_uchar,
            width: 0x58 as libc::c_int as libc::c_uchar,
            ic_version: 0x1 as libc::c_int as libc::c_uchar,
            is_otp_model: 0x1 as libc::c_int,
            name: b"eFSA688RA\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = elanspi_sensor_entry {
            sensor_id: 0xc as libc::c_int as libc::c_uchar,
            height: 0x50 as libc::c_int as libc::c_uchar,
            width: 0x50 as libc::c_int as libc::c_uchar,
            ic_version: 0x1 as libc::c_int as libc::c_uchar,
            is_otp_model: 0 as libc::c_int,
            name: b"eFSA80SA\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = elanspi_sensor_entry {
            sensor_id: 0xd as libc::c_int as libc::c_uchar,
            height: 0x47 as libc::c_int as libc::c_uchar,
            width: 0x80 as libc::c_int as libc::c_uchar,
            ic_version: 0x1 as libc::c_int as libc::c_uchar,
            is_otp_model: 0x1 as libc::c_int,
            name: b"eFSA712RA\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = elanspi_sensor_entry {
            sensor_id: 0xe as libc::c_int as libc::c_uchar,
            height: 0x50 as libc::c_int as libc::c_uchar,
            width: 0x50 as libc::c_int as libc::c_uchar,
            ic_version: 0x2 as libc::c_int as libc::c_uchar,
            is_otp_model: 0 as libc::c_int,
            name: b"eFSA80SC\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = elanspi_sensor_entry {
            sensor_id: 0 as libc::c_int as libc::c_uchar,
            height: 0 as libc::c_int as libc::c_uchar,
            width: 0 as libc::c_int as libc::c_uchar,
            ic_version: 0 as libc::c_int as libc::c_uchar,
            is_otp_model: 0 as libc::c_int,
            name: 0 as *const gchar,
        };
        init
    },
];
static mut elanspi_calibration_table_default: [elanspi_reg_entry; 18] = [
    {
        let mut init = elanspi_reg_entry {
            addr: 0x5 as libc::c_int as libc::c_uchar,
            value: 0x60 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x6 as libc::c_int as libc::c_uchar,
            value: 0xc0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x7 as libc::c_int as libc::c_uchar,
            value: 0x80 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x8 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xa as libc::c_int as libc::c_uchar,
            value: 0x97 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xb as libc::c_int as libc::c_uchar,
            value: 0x72 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xc as libc::c_int as libc::c_uchar,
            value: 0x69 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xf as libc::c_int as libc::c_uchar,
            value: 0x2a as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x11 as libc::c_int as libc::c_uchar,
            value: 0x2a as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x13 as libc::c_int as libc::c_uchar,
            value: 0x27 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x15 as libc::c_int as libc::c_uchar,
            value: 0x67 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x18 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x21 as libc::c_int as libc::c_uchar,
            value: 0x20 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x22 as libc::c_int as libc::c_uchar,
            value: 0x36 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2a as libc::c_int as libc::c_uchar,
            value: 0x5f as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2b as libc::c_int as libc::c_uchar,
            value: 0xc0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2e as libc::c_int as libc::c_uchar,
            value: 0xff as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xff as libc::c_int as libc::c_uchar,
            value: 0xff as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut elanspi_calibration_table_id6: [elanspi_reg_entry; 26] = [
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2a as libc::c_int as libc::c_uchar,
            value: 0x7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x1 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2 as libc::c_int as libc::c_uchar,
            value: 0x5f as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x3 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x4 as libc::c_int as libc::c_uchar,
            value: 0x5f as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x5 as libc::c_int as libc::c_uchar,
            value: 0x60 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x6 as libc::c_int as libc::c_uchar,
            value: 0xc0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x7 as libc::c_int as libc::c_uchar,
            value: 0x80 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x8 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xa as libc::c_int as libc::c_uchar,
            value: 0x97 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xb as libc::c_int as libc::c_uchar,
            value: 0x72 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xc as libc::c_int as libc::c_uchar,
            value: 0x69 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xf as libc::c_int as libc::c_uchar,
            value: 0x2a as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x11 as libc::c_int as libc::c_uchar,
            value: 0x2a as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x13 as libc::c_int as libc::c_uchar,
            value: 0x27 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x15 as libc::c_int as libc::c_uchar,
            value: 0x67 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x18 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x21 as libc::c_int as libc::c_uchar,
            value: 0x20 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x22 as libc::c_int as libc::c_uchar,
            value: 0x36 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x29 as libc::c_int as libc::c_uchar,
            value: 0x2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2a as libc::c_int as libc::c_uchar,
            value: 0x3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2a as libc::c_int as libc::c_uchar,
            value: 0x5f as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2b as libc::c_int as libc::c_uchar,
            value: 0xc0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2c as libc::c_int as libc::c_uchar,
            value: 0x10 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2e as libc::c_int as libc::c_uchar,
            value: 0xff as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xff as libc::c_int as libc::c_uchar,
            value: 0xff as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut elanspi_calibration_table_id57: [elanspi_reg_entry; 19] = [
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2a as libc::c_int as libc::c_uchar,
            value: 0x7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x5 as libc::c_int as libc::c_uchar,
            value: 0x60 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x6 as libc::c_int as libc::c_uchar,
            value: 0xc0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x7 as libc::c_int as libc::c_uchar,
            value: 0x80 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x8 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xa as libc::c_int as libc::c_uchar,
            value: 0x97 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xb as libc::c_int as libc::c_uchar,
            value: 0x72 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xc as libc::c_int as libc::c_uchar,
            value: 0x69 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xf as libc::c_int as libc::c_uchar,
            value: 0x2a as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x11 as libc::c_int as libc::c_uchar,
            value: 0x2a as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x13 as libc::c_int as libc::c_uchar,
            value: 0x27 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x15 as libc::c_int as libc::c_uchar,
            value: 0x67 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x18 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x21 as libc::c_int as libc::c_uchar,
            value: 0x20 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x22 as libc::c_int as libc::c_uchar,
            value: 0x36 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2a as libc::c_int as libc::c_uchar,
            value: 0x5f as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2b as libc::c_int as libc::c_uchar,
            value: 0xc0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2e as libc::c_int as libc::c_uchar,
            value: 0xff as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xff as libc::c_int as libc::c_uchar,
            value: 0xff as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut elanspi_calibration_table_id0: [elanspi_reg_entry; 14] = [
    {
        let mut init = elanspi_reg_entry {
            addr: 0x5 as libc::c_int as libc::c_uchar,
            value: 0x60 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x6 as libc::c_int as libc::c_uchar,
            value: 0xc0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x8 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xa as libc::c_int as libc::c_uchar,
            value: 0x97 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xb as libc::c_int as libc::c_uchar,
            value: 0x72 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xc as libc::c_int as libc::c_uchar,
            value: 0x69 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xf as libc::c_int as libc::c_uchar,
            value: 0x2b as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x11 as libc::c_int as libc::c_uchar,
            value: 0x2b as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x13 as libc::c_int as libc::c_uchar,
            value: 0x28 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x15 as libc::c_int as libc::c_uchar,
            value: 0x28 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x18 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x21 as libc::c_int as libc::c_uchar,
            value: 0x20 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2a as libc::c_int as libc::c_uchar,
            value: 0x4b as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xff as libc::c_int as libc::c_uchar,
            value: 0xff as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut elanspi_calibration_table_old: elanspi_regtable = unsafe {
    {
        let mut init = elanspi_regtable {
            other: elanspi_calibration_table_default.as_ptr(),
            entries: [
                {
                    let mut init = C2RustUnnamed_4 {
                        sid: 0 as libc::c_int as libc::c_uchar,
                        table: elanspi_calibration_table_id0.as_ptr(),
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_4 {
                        sid: 0x5 as libc::c_int as libc::c_uchar,
                        table: elanspi_calibration_table_id57.as_ptr(),
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_4 {
                        sid: 0x6 as libc::c_int as libc::c_uchar,
                        table: elanspi_calibration_table_id6.as_ptr(),
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_4 {
                        sid: 0x7 as libc::c_int as libc::c_uchar,
                        table: elanspi_calibration_table_id57.as_ptr(),
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_4 {
                        sid: 0 as libc::c_int as libc::c_uchar,
                        table: 0 as *const elanspi_reg_entry,
                    };
                    init
                },
            ],
        };
        init
    }
};
static mut elanspi_calibration_table_page0_id14: [elanspi_reg_entry; 60] = [
    {
        let mut init = elanspi_reg_entry {
            addr: 0 as libc::c_int as libc::c_uchar,
            value: 0x5a as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x1 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2 as libc::c_int as libc::c_uchar,
            value: 0x4f as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x3 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x4 as libc::c_int as libc::c_uchar,
            value: 0x4f as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x5 as libc::c_int as libc::c_uchar,
            value: 0xa0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x6 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x7 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x8 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x9 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xa as libc::c_int as libc::c_uchar,
            value: 0x74 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xb as libc::c_int as libc::c_uchar,
            value: 0x5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xc as libc::c_int as libc::c_uchar,
            value: 0x8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xd as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xe as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xf as libc::c_int as libc::c_uchar,
            value: 0x14 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x10 as libc::c_int as libc::c_uchar,
            value: 0x3c as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x11 as libc::c_int as libc::c_uchar,
            value: 0x41 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x12 as libc::c_int as libc::c_uchar,
            value: 0xc as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x13 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x14 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x15 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x16 as libc::c_int as libc::c_uchar,
            value: 0x2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x17 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x18 as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x19 as libc::c_int as libc::c_uchar,
            value: 0xf4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x1a as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x1b as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x1c as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x1d as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x1e as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x1f as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x20 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x21 as libc::c_int as libc::c_uchar,
            value: 0x80 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x22 as libc::c_int as libc::c_uchar,
            value: 0x6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x23 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x24 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x25 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x26 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x27 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x28 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x29 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2a as libc::c_int as libc::c_uchar,
            value: 0x5f as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2b as libc::c_int as libc::c_uchar,
            value: 0xe2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2c as libc::c_int as libc::c_uchar,
            value: 0xa0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2d as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2e as libc::c_int as libc::c_uchar,
            value: 0xff as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2f as libc::c_int as libc::c_uchar,
            value: 0x40 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x30 as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x31 as libc::c_int as libc::c_uchar,
            value: 0x38 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x32 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x33 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x34 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x35 as libc::c_int as libc::c_uchar,
            value: 0x1f as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x36 as libc::c_int as libc::c_uchar,
            value: 0xff as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x37 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x38 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x39 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x3a as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xff as libc::c_int as libc::c_uchar,
            value: 0xff as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut elanspi_calibration_table_page1_id14: [elanspi_reg_entry; 65] = [
    {
        let mut init = elanspi_reg_entry {
            addr: 0 as libc::c_int as libc::c_uchar,
            value: 0x7b as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x1 as libc::c_int as libc::c_uchar,
            value: 0x7f as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2 as libc::c_int as libc::c_uchar,
            value: 0x77 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x3 as libc::c_int as libc::c_uchar,
            value: 0xd4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x4 as libc::c_int as libc::c_uchar,
            value: 0x7d as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x5 as libc::c_int as libc::c_uchar,
            value: 0x19 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x6 as libc::c_int as libc::c_uchar,
            value: 0x80 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x7 as libc::c_int as libc::c_uchar,
            value: 0x40 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x8 as libc::c_int as libc::c_uchar,
            value: 0x11 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x9 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xa as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xb as libc::c_int as libc::c_uchar,
            value: 0x14 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xc as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xd as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xe as libc::c_int as libc::c_uchar,
            value: 0x32 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xf as libc::c_int as libc::c_uchar,
            value: 0x2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x10 as libc::c_int as libc::c_uchar,
            value: 0x8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x11 as libc::c_int as libc::c_uchar,
            value: 0x6c as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x12 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x13 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x14 as libc::c_int as libc::c_uchar,
            value: 0x32 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x15 as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x16 as libc::c_int as libc::c_uchar,
            value: 0x16 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x17 as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x18 as libc::c_int as libc::c_uchar,
            value: 0x14 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x19 as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x1a as libc::c_int as libc::c_uchar,
            value: 0x16 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x1b as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x17 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x1e as libc::c_int as libc::c_uchar,
            value: 0xa as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x1f as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x20 as libc::c_int as libc::c_uchar,
            value: 0xa as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x21 as libc::c_int as libc::c_uchar,
            value: 0x2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x22 as libc::c_int as libc::c_uchar,
            value: 0x8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x23 as libc::c_int as libc::c_uchar,
            value: 0x29 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x24 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x25 as libc::c_int as libc::c_uchar,
            value: 0xc as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x26 as libc::c_int as libc::c_uchar,
            value: 0x1a as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x27 as libc::c_int as libc::c_uchar,
            value: 0x30 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x28 as libc::c_int as libc::c_uchar,
            value: 0x1a as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x29 as libc::c_int as libc::c_uchar,
            value: 0x30 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2a as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2b as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2c as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2d as libc::c_int as libc::c_uchar,
            value: 0x16 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2e as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x2f as libc::c_int as libc::c_uchar,
            value: 0x17 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x30 as libc::c_int as libc::c_uchar,
            value: 0x3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x31 as libc::c_int as libc::c_uchar,
            value: 0x2d as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x32 as libc::c_int as libc::c_uchar,
            value: 0x3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x33 as libc::c_int as libc::c_uchar,
            value: 0x2d as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x34 as libc::c_int as libc::c_uchar,
            value: 0x14 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x35 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x36 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x37 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x38 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x39 as libc::c_int as libc::c_uchar,
            value: 0x3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x3a as libc::c_int as libc::c_uchar,
            value: 0xfe as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x3b as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x3c as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x3d as libc::c_int as libc::c_uchar,
            value: 0x2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x3e as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0x3f as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = elanspi_reg_entry {
            addr: 0xff as libc::c_int as libc::c_uchar,
            value: 0xff as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut elanspi_calibration_table_new_page0: elanspi_regtable = unsafe {
    {
        let mut init = elanspi_regtable {
            other: 0 as *const elanspi_reg_entry,
            entries: [
                {
                    let mut init = C2RustUnnamed_4 {
                        sid: 0xe as libc::c_int as libc::c_uchar,
                        table: elanspi_calibration_table_page0_id14.as_ptr(),
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_4 {
                        sid: 0 as libc::c_int as libc::c_uchar,
                        table: 0 as *const elanspi_reg_entry,
                    };
                    init
                },
            ],
        };
        init
    }
};
static mut elanspi_calibration_table_new_page1: elanspi_regtable = unsafe {
    {
        let mut init = elanspi_regtable {
            other: 0 as *const elanspi_reg_entry,
            entries: [
                {
                    let mut init = C2RustUnnamed_4 {
                        sid: 0xe as libc::c_int as libc::c_uchar,
                        table: elanspi_calibration_table_page1_id14.as_ptr(),
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_4 {
                        sid: 0 as libc::c_int as libc::c_uchar,
                        table: 0 as *const elanspi_reg_entry,
                    };
                    init
                },
            ],
        };
        init
    }
};
static mut elanspi_id_table: [FpIdEntry; 10] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed_0: {
                    let mut init = C2RustUnnamed_1 {
                        udev_types: (FPI_DEVICE_UDEV_SUBTYPE_SPIDEV as libc::c_int
                            | FPI_DEVICE_UDEV_SUBTYPE_HIDRAW as libc::c_int)
                            as FpiDeviceUdevSubtypeFlags,
                        spi_acpi_id: b"ELAN7001\0" as *const u8 as *const libc::c_char,
                        hid_id: {
                            let mut init = C2RustUnnamed_2 {
                                pid: 0x3057 as libc::c_int as guint,
                                vid: 0x4f3 as libc::c_int as guint,
                            };
                            init
                        },
                    };
                    init
                },
            },
            driver_data: 2 as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed_0: {
                    let mut init = C2RustUnnamed_1 {
                        udev_types: (FPI_DEVICE_UDEV_SUBTYPE_SPIDEV as libc::c_int
                            | FPI_DEVICE_UDEV_SUBTYPE_HIDRAW as libc::c_int)
                            as FpiDeviceUdevSubtypeFlags,
                        spi_acpi_id: b"ELAN7001\0" as *const u8 as *const libc::c_char,
                        hid_id: {
                            let mut init = C2RustUnnamed_2 {
                                pid: 0x3087 as libc::c_int as guint,
                                vid: 0x4f3 as libc::c_int as guint,
                            };
                            init
                        },
                    };
                    init
                },
            },
            driver_data: 2 as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed_0: {
                    let mut init = C2RustUnnamed_1 {
                        udev_types: (FPI_DEVICE_UDEV_SUBTYPE_SPIDEV as libc::c_int
                            | FPI_DEVICE_UDEV_SUBTYPE_HIDRAW as libc::c_int)
                            as FpiDeviceUdevSubtypeFlags,
                        spi_acpi_id: b"ELAN7001\0" as *const u8 as *const libc::c_char,
                        hid_id: {
                            let mut init = C2RustUnnamed_2 {
                                pid: 0x30c6 as libc::c_int as guint,
                                vid: 0x4f3 as libc::c_int as guint,
                            };
                            init
                        },
                    };
                    init
                },
            },
            driver_data: 2 as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed_0: {
                    let mut init = C2RustUnnamed_1 {
                        udev_types: (FPI_DEVICE_UDEV_SUBTYPE_SPIDEV as libc::c_int
                            | FPI_DEVICE_UDEV_SUBTYPE_HIDRAW as libc::c_int)
                            as FpiDeviceUdevSubtypeFlags,
                        spi_acpi_id: b"ELAN70A1\0" as *const u8 as *const libc::c_char,
                        hid_id: {
                            let mut init = C2RustUnnamed_2 {
                                pid: 0x3134 as libc::c_int as guint,
                                vid: 0x4f3 as libc::c_int as guint,
                            };
                            init
                        },
                    };
                    init
                },
            },
            driver_data: 1 as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed_0: {
                    let mut init = C2RustUnnamed_1 {
                        udev_types: (FPI_DEVICE_UDEV_SUBTYPE_SPIDEV as libc::c_int
                            | FPI_DEVICE_UDEV_SUBTYPE_HIDRAW as libc::c_int)
                            as FpiDeviceUdevSubtypeFlags,
                        spi_acpi_id: b"ELAN7001\0" as *const u8 as *const libc::c_char,
                        hid_id: {
                            let mut init = C2RustUnnamed_2 {
                                pid: 0x3148 as libc::c_int as guint,
                                vid: 0x4f3 as libc::c_int as guint,
                            };
                            init
                        },
                    };
                    init
                },
            },
            driver_data: 2 as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed_0: {
                    let mut init = C2RustUnnamed_1 {
                        udev_types: (FPI_DEVICE_UDEV_SUBTYPE_SPIDEV as libc::c_int
                            | FPI_DEVICE_UDEV_SUBTYPE_HIDRAW as libc::c_int)
                            as FpiDeviceUdevSubtypeFlags,
                        spi_acpi_id: b"ELAN7001\0" as *const u8 as *const libc::c_char,
                        hid_id: {
                            let mut init = C2RustUnnamed_2 {
                                pid: 0x30b2 as libc::c_int as guint,
                                vid: 0x4f3 as libc::c_int as guint,
                            };
                            init
                        },
                    };
                    init
                },
            },
            driver_data: 0 as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed_0: {
                    let mut init = C2RustUnnamed_1 {
                        udev_types: (FPI_DEVICE_UDEV_SUBTYPE_SPIDEV as libc::c_int
                            | FPI_DEVICE_UDEV_SUBTYPE_HIDRAW as libc::c_int)
                            as FpiDeviceUdevSubtypeFlags,
                        spi_acpi_id: b"ELAN70A1\0" as *const u8 as *const libc::c_char,
                        hid_id: {
                            let mut init = C2RustUnnamed_2 {
                                pid: 0x30b2 as libc::c_int as guint,
                                vid: 0x4f3 as libc::c_int as guint,
                            };
                            init
                        },
                    };
                    init
                },
            },
            driver_data: 0 as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed_0: {
                    let mut init = C2RustUnnamed_1 {
                        udev_types: (FPI_DEVICE_UDEV_SUBTYPE_SPIDEV as libc::c_int
                            | FPI_DEVICE_UDEV_SUBTYPE_HIDRAW as libc::c_int)
                            as FpiDeviceUdevSubtypeFlags,
                        spi_acpi_id: b"ELAN7001\0" as *const u8 as *const libc::c_char,
                        hid_id: {
                            let mut init = C2RustUnnamed_2 {
                                pid: 0x309f as libc::c_int as guint,
                                vid: 0x4f3 as libc::c_int as guint,
                            };
                            init
                        },
                    };
                    init
                },
            },
            driver_data: 2 as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed_0: {
                    let mut init = C2RustUnnamed_1 {
                        udev_types: (FPI_DEVICE_UDEV_SUBTYPE_SPIDEV as libc::c_int
                            | FPI_DEVICE_UDEV_SUBTYPE_HIDRAW as libc::c_int)
                            as FpiDeviceUdevSubtypeFlags,
                        spi_acpi_id: b"ELAN7001\0" as *const u8 as *const libc::c_char,
                        hid_id: {
                            let mut init = C2RustUnnamed_2 {
                                pid: 0x241f as libc::c_int as guint,
                                vid: 0x4f3 as libc::c_int as guint,
                            };
                            init
                        },
                    };
                    init
                },
            },
            driver_data: 0 as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed_0: {
                    let mut init = C2RustUnnamed_1 {
                        udev_types: 0 as FpiDeviceUdevSubtypeFlags,
                        spi_acpi_id: 0 as *const gchar,
                        hid_id: C2RustUnnamed_2 { pid: 0, vid: 0 },
                    };
                    init
                },
            },
            driver_data: 0,
        };
        init
    },
];
#[inline]
unsafe extern "C" fn FPI_DEVICE_ELANSPI(mut ptr: gpointer) -> *mut FpiDeviceElanSpi {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_elanspi_get_type(),
    ) as *mut libc::c_void as *mut FpiDeviceElanSpi;
}
static mut FpiDeviceElanSpi_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn fpi_device_elanspi_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_image_device_get_type(),
        g_intern_static_string(
            b"FpiDeviceElanSpi\0" as *const u8 as *const libc::c_char,
        ),
        ::core::mem::size_of::<FpiDeviceElanSpiClass>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_elanspi_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpiDeviceElanSpi>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpiDeviceElanSpi) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_elanspi_init
                        as unsafe extern "C" fn(*mut FpiDeviceElanSpi) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut fpi_device_elanspi_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn fpi_device_elanspi_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_elanspi_get_type_once();
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
unsafe extern "C" fn fpi_device_elanspi_class_intern_init(mut klass: gpointer) {
    fpi_device_elanspi_parent_class = g_type_class_peek_parent(klass);
    if FpiDeviceElanSpi_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpiDeviceElanSpi_private_offset);
    }
    fpi_device_elanspi_class_init(klass as *mut FpiDeviceElanSpiClass);
}
unsafe extern "C" fn elanspi_do_hwreset(
    mut self_0: *mut FpiDeviceElanSpi,
    mut err: *mut *mut GError,
) {
    if g_strcmp0(
        g_getenv(b"FP_DEVICE_EMULATION\0" as *const u8 as *const libc::c_char),
        b"1\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        return;
    }
    let mut fd: libc::c_int = open(
        fpi_device_get_udev_data(
            FP_DEVICE(self_0 as gpointer),
            FPI_DEVICE_UDEV_SUBTYPE_HIDRAW,
        ) as *mut libc::c_char,
        0o2 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        g_set_error(
            err,
            g_io_error_quark(),
            g_io_error_from_errno(*__errno_location()) as gint,
            b"unable to open hid\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut buf: [guint8; 5] = [
        0xe as libc::c_int as guint8,
        0 as libc::c_int as guint8,
        0 as libc::c_int as guint8,
        0 as libc::c_int as guint8,
        0 as libc::c_int as guint8,
    ];
    if ioctl(
        fd,
        ((1 as libc::c_uint | 2 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('H' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((0x6 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            | ((5 as libc::c_int)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint) as libc::c_ulong,
        &mut buf as *mut [guint8; 5],
    ) != 5 as libc::c_int
    {
        g_set_error(
            err,
            g_io_error_quark(),
            g_io_error_from_errno(*__errno_location()) as gint,
            b"unable to reset via hid\0" as *const u8 as *const libc::c_char,
        );
    }
    close(fd);
}
unsafe extern "C" fn elanspi_do_swreset(
    mut self_0: *mut FpiDeviceElanSpi,
) -> *mut FpiSpiTransfer {
    let mut xfer: *mut FpiSpiTransfer = fpi_spi_transfer_new(
        FP_DEVICE(self_0 as gpointer),
        (*self_0).spi_fd,
    );
    fpi_spi_transfer_write(xfer, 1 as libc::c_int as gsize);
    *((*xfer).buffer_wr)
        .offset(0 as libc::c_int as isize) = 0x31 as libc::c_int as guchar;
    return xfer;
}
unsafe extern "C" fn elanspi_do_startcalib(
    mut self_0: *mut FpiDeviceElanSpi,
) -> *mut FpiSpiTransfer {
    let mut xfer: *mut FpiSpiTransfer = fpi_spi_transfer_new(
        FP_DEVICE(self_0 as gpointer),
        (*self_0).spi_fd,
    );
    fpi_spi_transfer_write(xfer, 1 as libc::c_int as gsize);
    *((*xfer).buffer_wr)
        .offset(0 as libc::c_int as isize) = 0x4 as libc::c_int as guchar;
    return xfer;
}
unsafe extern "C" fn elanspi_do_capture(
    mut self_0: *mut FpiDeviceElanSpi,
) -> *mut FpiSpiTransfer {
    let mut xfer: *mut FpiSpiTransfer = fpi_spi_transfer_new(
        FP_DEVICE(self_0 as gpointer),
        (*self_0).spi_fd,
    );
    fpi_spi_transfer_write(xfer, 1 as libc::c_int as gsize);
    *((*xfer).buffer_wr)
        .offset(0 as libc::c_int as isize) = 0x1 as libc::c_int as guchar;
    return xfer;
}
unsafe extern "C" fn elanspi_do_selectpage(
    mut self_0: *mut FpiDeviceElanSpi,
    mut page: guint8,
) -> *mut FpiSpiTransfer {
    let mut xfer: *mut FpiSpiTransfer = fpi_spi_transfer_new(
        FP_DEVICE(self_0 as gpointer),
        (*self_0).spi_fd,
    );
    fpi_spi_transfer_write(xfer, 2 as libc::c_int as gsize);
    *((*xfer).buffer_wr)
        .offset(0 as libc::c_int as isize) = 0x7 as libc::c_int as guchar;
    *((*xfer).buffer_wr).offset(1 as libc::c_int as isize) = page;
    return xfer;
}
unsafe extern "C" fn elanspi_single_read_cmd(
    mut self_0: *mut FpiDeviceElanSpi,
    mut cmd_id: guint8,
    mut data_out: *mut guint8,
) -> *mut FpiSpiTransfer {
    let mut xfer: *mut FpiSpiTransfer = fpi_spi_transfer_new(
        FP_DEVICE(self_0 as gpointer),
        (*self_0).spi_fd,
    );
    fpi_spi_transfer_write(xfer, 2 as libc::c_int as gsize);
    *((*xfer).buffer_wr).offset(0 as libc::c_int as isize) = cmd_id;
    *((*xfer).buffer_wr)
        .offset(1 as libc::c_int as isize) = 0xff as libc::c_int as guchar;
    fpi_spi_transfer_read_full(xfer, data_out, 1 as libc::c_int as gsize, None);
    return xfer;
}
unsafe extern "C" fn elanspi_read_status(
    mut self_0: *mut FpiDeviceElanSpi,
    mut data_out: *mut guint8,
) -> *mut FpiSpiTransfer {
    return elanspi_single_read_cmd(self_0, 0x3 as libc::c_int as guint8, data_out);
}
unsafe extern "C" fn elanspi_read_width(
    mut self_0: *mut FpiDeviceElanSpi,
    mut data_out: *mut guint8,
) -> *mut FpiSpiTransfer {
    return elanspi_single_read_cmd(self_0, 0x9 as libc::c_int as guint8, data_out);
}
unsafe extern "C" fn elanspi_read_height(
    mut self_0: *mut FpiDeviceElanSpi,
    mut data_out: *mut guint8,
) -> *mut FpiSpiTransfer {
    return elanspi_single_read_cmd(self_0, 0x8 as libc::c_int as guint8, data_out);
}
unsafe extern "C" fn elanspi_read_version(
    mut self_0: *mut FpiDeviceElanSpi,
    mut data_out: *mut guint8,
) -> *mut FpiSpiTransfer {
    return elanspi_single_read_cmd(self_0, 0xa as libc::c_int as guint8, data_out);
}
unsafe extern "C" fn elanspi_read_register(
    mut self_0: *mut FpiDeviceElanSpi,
    mut reg_id: guint8,
    mut data_out: *mut guint8,
) -> *mut FpiSpiTransfer {
    let mut xfer: *mut FpiSpiTransfer = fpi_spi_transfer_new(
        FP_DEVICE(self_0 as gpointer),
        (*self_0).spi_fd,
    );
    fpi_spi_transfer_write(xfer, 1 as libc::c_int as gsize);
    *((*xfer).buffer_wr)
        .offset(
            0 as libc::c_int as isize,
        ) = (reg_id as libc::c_int | 0x40 as libc::c_int) as guchar;
    fpi_spi_transfer_read_full(xfer, data_out, 1 as libc::c_int as gsize, None);
    return xfer;
}
unsafe extern "C" fn elanspi_write_register(
    mut self_0: *mut FpiDeviceElanSpi,
    mut reg_id: guint8,
    mut data_in: guint8,
) -> *mut FpiSpiTransfer {
    let mut xfer: *mut FpiSpiTransfer = fpi_spi_transfer_new(
        FP_DEVICE(self_0 as gpointer),
        (*self_0).spi_fd,
    );
    fpi_spi_transfer_write(xfer, 2 as libc::c_int as gsize);
    *((*xfer).buffer_wr)
        .offset(
            0 as libc::c_int as isize,
        ) = (reg_id as libc::c_int | 0x80 as libc::c_int) as guchar;
    *((*xfer).buffer_wr).offset(1 as libc::c_int as isize) = data_in;
    return xfer;
}
unsafe extern "C" fn elanspi_determine_sensor(
    mut self_0: *mut FpiDeviceElanSpi,
    mut err: *mut *mut GError,
) {
    let mut raw_height: guint8 = (*self_0).sensor_height;
    let mut raw_width: guint8 = (*self_0).sensor_width;
    if raw_height as libc::c_int == 0xa1 as libc::c_int
        && raw_width as libc::c_int == 0xa1 as libc::c_int
        || raw_height as libc::c_int == 0xd1 as libc::c_int
            && raw_width as libc::c_int == 0x51 as libc::c_int
        || raw_height as libc::c_int == 0xc1 as libc::c_int
            && raw_width as libc::c_int == 0x39 as libc::c_int
    {
        (*self_0).sensor_ic_version = 0 as libc::c_int as guint8;
        (*self_0).sensor_width = (raw_width as libc::c_int - 1 as libc::c_int) as guint8;
        (*self_0)
            .sensor_height = (raw_height as libc::c_int - 1 as libc::c_int) as guint8;
    } else if raw_width as libc::c_int == 0x60 as libc::c_int
        && raw_height as libc::c_int == 0x60 as libc::c_int
    {
        (*self_0)
            .sensor_ic_version = (if (*self_0).sensor_reg_17 as libc::c_int
            & 0x80 as libc::c_int != 0
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as guint8;
    } else if (raw_height as libc::c_int != 0xa0 as libc::c_int
        || raw_width as libc::c_int != 0x50 as libc::c_int)
        && (raw_height as libc::c_int != 0x90 as libc::c_int
            || raw_width as libc::c_int != 0x40 as libc::c_int)
        && (raw_height as libc::c_int != 0x78 as libc::c_int
            || raw_width as libc::c_int != 0x78 as libc::c_int)
    {
        if (raw_height as libc::c_int != 0x40 as libc::c_int
            || raw_width as libc::c_int != 0x58 as libc::c_int)
            && (raw_height as libc::c_int != 0x50 as libc::c_int
                || raw_width as libc::c_int != 0x50 as libc::c_int)
        {
            (*self_0).sensor_width = 0x78 as libc::c_int as guint8;
            (*self_0).sensor_height = 0x78 as libc::c_int as guint8;
            (*self_0).sensor_ic_version = 0 as libc::c_int as guint8;
        } else {
            (*self_0)
                .sensor_ic_version = (((*self_0).sensor_raw_version as libc::c_int
                & 0x70 as libc::c_int) >> 4 as libc::c_int) as guint8;
        }
    } else {
        (*self_0).sensor_ic_version = 1 as libc::c_int as guint8;
    }
    g_log(
        b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"<init/detect> after hardcoded lookup; %dx%d, version %d\0" as *const u8
            as *const libc::c_char,
        (*self_0).sensor_width as libc::c_int,
        (*self_0).sensor_height as libc::c_int,
        (*self_0).sensor_ic_version as libc::c_int,
    );
    let mut entry: *const elanspi_sensor_entry = elanspi_sensor_table.as_ptr();
    while !((*entry).name).is_null() {
        if (*entry).ic_version as libc::c_int
            == (*self_0).sensor_ic_version as libc::c_int
            && (*entry).width as libc::c_int == (*self_0).sensor_width as libc::c_int
            && (*entry).height as libc::c_int == (*self_0).sensor_height as libc::c_int
        {
            (*self_0).sensor_id = (*entry).sensor_id;
            (*self_0).sensor_otp = (*entry).is_otp_model;
            g_log(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"<init/detect> found sensor ID %d => [%s] (%d x %d)\0" as *const u8
                    as *const libc::c_char,
                (*self_0).sensor_id as libc::c_int,
                (*entry).name,
                (*self_0).sensor_width as libc::c_int,
                (*self_0).sensor_height as libc::c_int,
            );
            break;
        } else {
            entry = entry.offset(1 as libc::c_int as isize);
        }
    }
    if (*self_0).sensor_id as libc::c_int == 0xff as libc::c_int {
        *err = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_NOT_SUPPORTED,
            b"unknown sensor (%dx%d, v%d)\0" as *const u8 as *const libc::c_char,
            (*self_0).sensor_width as libc::c_int,
            (*self_0).sensor_height as libc::c_int,
            (*self_0).sensor_ic_version as libc::c_int,
        );
        return;
    }
    if fpi_device_get_driver_data(FP_DEVICE(self_0 as gpointer))
        & 1 as libc::c_int as libc::c_ulong != 0
    {
        (*self_0).frame_width = (*self_0).sensor_height;
        (*self_0)
            .frame_height = (if (*self_0).sensor_width as libc::c_int > 43 as libc::c_int
        {
            43 as libc::c_int
        } else {
            (*self_0).sensor_width as libc::c_int
        }) as guint8;
    } else {
        (*self_0).frame_width = (*self_0).sensor_width;
        (*self_0)
            .frame_height = (if (*self_0).sensor_height as libc::c_int
            > 43 as libc::c_int
        {
            43 as libc::c_int
        } else {
            (*self_0).sensor_height as libc::c_int
        }) as guint8;
    };
}
unsafe extern "C" fn elanspi_capture_old_line_handler(
    mut transfer: *mut FpiSpiTransfer,
    mut dev: *mut FpDevice,
    mut unused_data: gpointer,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceElanSpi = FPI_DEVICE_ELANSPI(dev as gpointer);
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    let mut col: libc::c_int = 0 as libc::c_int;
    while col < (*self_0).sensor_width as libc::c_int {
        let mut low: guint8 = *((*transfer).buffer_rd)
            .offset((col * 2 as libc::c_int + 1 as libc::c_int) as isize);
        let mut high: guint8 = *((*transfer).buffer_rd)
            .offset((col * 2 as libc::c_int) as isize);
        *((*self_0).last_image)
            .offset(
                ((*self_0).sensor_width as libc::c_int
                    * (*self_0).c2rust_unnamed.old_data.line_ptr as libc::c_int + col)
                    as isize,
            ) = (low as libc::c_int + high as libc::c_int * 0x100 as libc::c_int)
            as guint16;
        col += 1 as libc::c_int;
    }
    (*self_0)
        .c2rust_unnamed
        .old_data
        .line_ptr = ((*self_0).c2rust_unnamed.old_data.line_ptr as libc::c_int
        + 1 as libc::c_int) as guint8;
    if ((*self_0).c2rust_unnamed.old_data.line_ptr as libc::c_int)
        < (*self_0).sensor_height as libc::c_int
    {
        fpi_ssm_jump_to_state(
            (*transfer).ssm,
            ELANSPI_CAPTOLD_CHECK_LINEREADY as libc::c_int,
        );
    } else {
        if fpi_device_get_current_action(dev) as libc::c_uint
            == FPI_DEVICE_ACTION_NONE as libc::c_int as libc::c_uint
        {
            fpi_ssm_mark_completed((*transfer).ssm);
            return;
        }
        if fpi_device_action_is_cancelled(dev) != 0 {
            g_cancellable_set_error_if_cancelled(
                fpi_device_get_cancellable(dev),
                &mut error,
            );
            fpi_ssm_mark_failed((*transfer).ssm, error);
            return;
        }
        fpi_ssm_mark_completed((*transfer).ssm);
    };
}
unsafe extern "C" fn elanspi_capture_old_handler(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceElanSpi = FPI_DEVICE_ELANSPI(dev as gpointer);
    let mut xfer: *mut FpiSpiTransfer = 0 as *mut FpiSpiTransfer;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            (*self_0).c2rust_unnamed.old_data.line_ptr = 0 as libc::c_int as guint8;
            (*self_0)
                .capture_timeout = g_get_monotonic_time()
                + (100 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
            xfer = elanspi_do_capture(self_0);
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                0 as *mut GCancellable,
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        1 => {
            xfer = elanspi_read_status(self_0, &mut (*self_0).sensor_status);
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                0 as *mut GCancellable,
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        2 => {
            if (*self_0).sensor_status as libc::c_int & 4 as libc::c_int == 0 {
                if g_get_monotonic_time() > (*self_0).capture_timeout
                    && g_strcmp0(
                        g_getenv(
                            b"FP_DEVICE_EMULATION\0" as *const u8 as *const libc::c_char,
                        ),
                        b"1\0" as *const u8 as *const libc::c_char,
                    ) != 0 as libc::c_int
                {
                    fpi_ssm_mark_failed(
                        ssm,
                        g_error_new(
                            g_io_error_quark(),
                            G_IO_ERROR_TIMED_OUT as libc::c_int,
                            b"timed out waiting for new line\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    return;
                }
                fpi_ssm_jump_to_state(
                    ssm,
                    ELANSPI_CAPTOLD_CHECK_LINEREADY as libc::c_int,
                );
                return;
            }
            xfer = fpi_spi_transfer_new(dev, (*self_0).spi_fd);
            (*xfer).ssm = ssm;
            fpi_spi_transfer_write(xfer, 2 as libc::c_int as gsize);
            *((*xfer).buffer_wr)
                .offset(0 as libc::c_int as isize) = 0x10 as libc::c_int as guchar;
            fpi_spi_transfer_read(
                xfer,
                ((*self_0).sensor_width as libc::c_int * 2 as libc::c_int) as gsize,
            );
            fpi_spi_transfer_submit(
                xfer,
                0 as *mut GCancellable,
                Some(
                    elanspi_capture_old_line_handler
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        _ => {}
    };
}
unsafe extern "C" fn elanspi_send_regtable_handler(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceElanSpi = FPI_DEVICE_ELANSPI(dev as gpointer);
    let mut xfer: *mut FpiSpiTransfer = 0 as *mut FpiSpiTransfer;
    let mut entry: *const elanspi_reg_entry = fpi_ssm_get_data(ssm)
        as *const elanspi_reg_entry;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            xfer = elanspi_write_register(self_0, (*entry).addr, (*entry).value);
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        1 => {
            entry = entry.offset(1 as libc::c_int as isize);
            if (*entry).addr as libc::c_int != 0xff as libc::c_int {
                fpi_ssm_set_data(ssm, entry as gpointer, None);
                fpi_ssm_jump_to_state(ssm, ELANSPI_WRTABLE_WRITE as libc::c_int);
                return;
            }
            fpi_ssm_mark_completed(ssm);
            return;
        }
        _ => {}
    };
}
unsafe extern "C" fn elanspi_write_regtable(
    mut self_0: *mut FpiDeviceElanSpi,
    mut table: *const elanspi_regtable,
) -> *mut FpiSsm {
    let mut starting_entry: *const elanspi_reg_entry = (*table).other;
    let mut i: libc::c_int = 0 as libc::c_int;
    while !((*((*table).entries).as_ptr().offset(i as isize)).table).is_null() {
        if (*((*table).entries).as_ptr().offset(i as isize)).sid as libc::c_int
            == (*self_0).sensor_id as libc::c_int
        {
            starting_entry = (*((*table).entries).as_ptr().offset(i as isize)).table;
            break;
        } else {
            i += 1 as libc::c_int;
        }
    }
    if starting_entry.is_null() {
        g_log(
            b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"<regtable> unknown regtable for sensor %d\0" as *const u8
                as *const libc::c_char,
            (*self_0).sensor_id as libc::c_int,
        );
        return 0 as *mut FpiSsm;
    }
    let mut ssm: *mut FpiSsm = fpi_ssm_new_full(
        FP_DEVICE(self_0 as gpointer),
        Some(
            elanspi_send_regtable_handler
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        ELANSPI_WRTABLE_NSTATES as libc::c_int,
        ELANSPI_WRTABLE_NSTATES as libc::c_int,
        b"ELANSPI_WRTABLE_NSTATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_set_data(ssm, starting_entry as gpointer, None);
    return ssm;
}
unsafe extern "C" fn elanspi_mean_image(
    mut self_0: *mut FpiDeviceElanSpi,
    mut img: *const guint16,
) -> libc::c_int {
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i
        < (*self_0).sensor_width as libc::c_int * (*self_0).sensor_height as libc::c_int
    {
        total += *img.offset(i as isize) as libc::c_int;
        i += 1 as libc::c_int;
    }
    return total
        / ((*self_0).sensor_width as libc::c_int
            * (*self_0).sensor_height as libc::c_int);
}
unsafe extern "C" fn elanspi_calibrate_old_handler(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceElanSpi = FPI_DEVICE_ELANSPI(dev as gpointer);
    let mut xfer: *mut FpiSpiTransfer = 0 as *mut FpiSpiTransfer;
    let mut err: *mut GError = 0 as *mut GError;
    let mut chld: *mut FpiSsm = 0 as *mut FpiSsm;
    let mut mean_value: libc::c_int = 0 as libc::c_int;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            xfer = elanspi_write_register(
                self_0,
                0 as libc::c_int as guint8,
                0x5a as libc::c_int as guint8,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        1 => {
            xfer = elanspi_do_startcalib(self_0);
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        2 => {
            fpi_ssm_next_state_delayed(ssm, 1 as libc::c_int);
            return;
        }
        3 => {
            chld = elanspi_write_regtable(self_0, &elanspi_calibration_table_old);
            if chld.is_null() {
                err = fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_NOT_SUPPORTED,
                    b"unknown calibration table for sensor\0" as *const u8
                        as *const libc::c_char,
                );
                fpi_ssm_mark_failed(ssm, err);
                return;
            }
            fpi_ssm_start_subsm(ssm, chld);
            return;
        }
        4 | 6 | 8 => {
            chld = fpi_ssm_new_full(
                dev,
                Some(
                    elanspi_capture_old_handler
                        as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                ),
                ELANSPI_CAPTOLD_NSTATES as libc::c_int,
                ELANSPI_CAPTOLD_NSTATES as libc::c_int,
                b"ELANSPI_CAPTOLD_NSTATES\0" as *const u8 as *const libc::c_char,
            );
            fpi_ssm_silence_debug(chld);
            fpi_ssm_start_subsm(ssm, chld);
            return;
        }
        5 => {
            (*self_0)
                .c2rust_unnamed
                .old_data
                .dac_value = ((elanspi_mean_image(self_0, (*self_0).last_image)
                & 0xffff as libc::c_int) + 0x80 as libc::c_int >> 8 as libc::c_int)
                as guint8;
            if (0x3f as libc::c_int)
                < (*self_0).c2rust_unnamed.old_data.dac_value as libc::c_int
            {
                (*self_0)
                    .c2rust_unnamed
                    .old_data
                    .dac_value = 0x3f as libc::c_int as guint8;
            }
            g_log(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"<calibold> dac init is 0x%02x\0" as *const u8 as *const libc::c_char,
                (*self_0).c2rust_unnamed.old_data.dac_value as libc::c_int,
            );
            xfer = elanspi_write_register(
                self_0,
                0x6 as libc::c_int as guint8,
                ((*self_0).c2rust_unnamed.old_data.dac_value as libc::c_int
                    - 0x40 as libc::c_int) as guint8,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        7 => {
            if elanspi_mean_image(self_0, (*self_0).last_image) >= 1000 as libc::c_int {
                err = fpi_device_retry_new_msg(
                    FP_DEVICE_RETRY_REMOVE_FINGER,
                    b"finger on sensor during calibration\0" as *const u8
                        as *const libc::c_char,
                );
                fpi_ssm_mark_failed(ssm, err);
                return;
            }
            xfer = elanspi_write_register(
                self_0,
                0x5 as libc::c_int as guint8,
                0x6f as libc::c_int as guint8,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            (*self_0).c2rust_unnamed.old_data.dacfine_retry = 0 as libc::c_int as guint8;
            return;
        }
        9 => {
            mean_value = elanspi_mean_image(self_0, (*self_0).last_image);
            if mean_value >= 3000 as libc::c_int && mean_value <= 8000 as libc::c_int {
                fpi_ssm_jump_to_state(ssm, ELANSPI_CALIBOLD_PROTECT as libc::c_int);
                return;
            }
            if mean_value
                < 3000 as libc::c_int
                    + (8000 as libc::c_int - 3000 as libc::c_int) / 2 as libc::c_int
            {
                (*self_0)
                    .c2rust_unnamed
                    .old_data
                    .dac_value = ((*self_0).c2rust_unnamed.old_data.dac_value
                    as libc::c_int - 1 as libc::c_int) as guint8;
            } else {
                (*self_0)
                    .c2rust_unnamed
                    .old_data
                    .dac_value = ((*self_0).c2rust_unnamed.old_data.dac_value
                    as libc::c_int + 1 as libc::c_int) as guint8;
            }
            xfer = elanspi_write_register(
                self_0,
                0x6 as libc::c_int as guint8,
                ((*self_0).c2rust_unnamed.old_data.dac_value as libc::c_int
                    - 0x40 as libc::c_int) as guint8,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        10 => {
            (*self_0)
                .c2rust_unnamed
                .old_data
                .dacfine_retry = ((*self_0).c2rust_unnamed.old_data.dacfine_retry
                as libc::c_int + 1 as libc::c_int) as guint8;
            if (*self_0).c2rust_unnamed.old_data.dacfine_retry as libc::c_int
                >= 2 as libc::c_int
            {
                err = fpi_device_retry_new_msg(
                    FP_DEVICE_RETRY_REMOVE_FINGER,
                    b"finger on sensor during calibration\0" as *const u8
                        as *const libc::c_char,
                );
                fpi_ssm_mark_failed(ssm, err);
                return;
            }
            g_log(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"<calibold> repeating calibration for the %dth time\0" as *const u8
                    as *const libc::c_char,
                (*self_0).c2rust_unnamed.old_data.dacfine_retry as libc::c_int,
            );
            fpi_ssm_jump_to_state(ssm, ELANSPI_CALIBOLD_DACFINE_CAPTURE as libc::c_int);
            return;
        }
        11 => {
            g_log(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"<calibold> calibration ok, saving bg image\0" as *const u8
                    as *const libc::c_char,
            );
            xfer = elanspi_write_register(
                self_0,
                0 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        _ => {}
    };
}
unsafe extern "C" fn elanspi_capture_hv_image_handler(
    mut transfer: *mut FpiSpiTransfer,
    mut dev: *mut FpDevice,
    mut unused_data: gpointer,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceElanSpi = FPI_DEVICE_ELANSPI(dev as gpointer);
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    let mut i: libc::c_int = 0;
    let mut outptr: libc::c_int = 0;
    let mut value: guint16 = 0 as libc::c_int as guint16;
    i = 0 as libc::c_int;
    outptr = 0 as libc::c_int;
    while (i as libc::c_long) < (*transfer).length_rd
        && outptr
            < (*self_0).sensor_height as libc::c_int
                * (*self_0).sensor_width as libc::c_int * 2 as libc::c_int
    {
        if *((*transfer).buffer_rd).offset(i as isize) as libc::c_int
            != 0xff as libc::c_int
        {
            if outptr % 2 as libc::c_int != 0 {
                value = ((value as libc::c_int) << 8 as libc::c_int) as guint16;
                value = (value as libc::c_int
                    | *((*transfer).buffer_rd).offset(i as isize) as libc::c_int)
                    as guint16;
                *((*self_0).last_image)
                    .offset((outptr / 2 as libc::c_int) as isize) = value;
            } else {
                value = *((*transfer).buffer_rd).offset(i as isize) as guint16;
            }
            outptr += 1 as libc::c_int;
        }
        i += 1 as libc::c_int;
    }
    if outptr
        != (*self_0).sensor_height as libc::c_int * (*self_0).sensor_width as libc::c_int
            * 2 as libc::c_int
    {
        g_log(
            b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"<capture/hv> did not receive full image\0" as *const u8
                as *const libc::c_char,
        );
        error = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_PROTO,
            b"hv image receieve did not fill buffer\0" as *const u8
                as *const libc::c_char,
        );
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    fpi_ssm_mark_completed((*transfer).ssm);
}
unsafe extern "C" fn elanspi_capture_hv_handler(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceElanSpi = FPI_DEVICE_ELANSPI(dev as gpointer);
    let mut xfer: *mut FpiSpiTransfer = 0 as *mut FpiSpiTransfer;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            (*self_0).c2rust_unnamed.old_data.line_ptr = 0 as libc::c_int as guint8;
            (*self_0)
                .capture_timeout = g_get_monotonic_time()
                + (50 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
            xfer = elanspi_do_capture(self_0);
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        1 => {
            xfer = elanspi_read_status(self_0, &mut (*self_0).sensor_status);
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        2 => {
            if (*self_0).sensor_status as libc::c_int & 4 as libc::c_int == 0 {
                if g_get_monotonic_time() > (*self_0).capture_timeout {
                    fpi_ssm_mark_failed(
                        ssm,
                        g_error_new(
                            g_io_error_quark(),
                            G_IO_ERROR_TIMED_OUT as libc::c_int,
                            b"timed out waiting for image\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    return;
                }
                fpi_ssm_jump_to_state(ssm, ELANSPI_CAPTHV_CHECK_READY as libc::c_int);
                return;
            }
            xfer = fpi_spi_transfer_new(dev, (*self_0).spi_fd);
            (*xfer).ssm = ssm;
            fpi_spi_transfer_write(xfer, 2 as libc::c_int as gsize);
            *((*xfer).buffer_wr)
                .offset(0 as libc::c_int as isize) = 0x10 as libc::c_int as guchar;
            fpi_spi_transfer_read(
                xfer,
                ((*self_0).sensor_height as libc::c_int
                    * ((*self_0).sensor_width as libc::c_int * 2 as libc::c_int
                        + 48 as libc::c_int)) as gsize,
            );
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    elanspi_capture_hv_image_handler
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        _ => {}
    };
}
unsafe extern "C" fn elanspi_calibrate_hv_handler(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceElanSpi = FPI_DEVICE_ELANSPI(dev as gpointer);
    let mut xfer: *mut FpiSpiTransfer = 0 as *mut FpiSpiTransfer;
    let mut err: *mut GError = 0 as *mut GError;
    let mut chld: *mut FpiSsm = 0 as *mut FpiSsm;
    let mut mean_diff: libc::c_int = 0 as libc::c_int;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            (*self_0)
                .c2rust_unnamed
                .hv_data
                .gdac_value = 0x100 as libc::c_int as guint16;
            (*self_0).c2rust_unnamed.hv_data.gdac_step = 0x100 as libc::c_int as guint16;
            (*self_0).c2rust_unnamed.hv_data.best_gdac = 0 as libc::c_int as guint16;
            (*self_0)
                .c2rust_unnamed
                .hv_data
                .best_meandiff = 0xffff as libc::c_int as guint16;
        }
        6 => {}
        1 => {
            xfer = elanspi_do_startcalib(self_0);
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        2 => {
            xfer = elanspi_write_register(
                self_0,
                0 as libc::c_int as guint8,
                0x5a as libc::c_int as guint8,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        3 => {
            chld = elanspi_write_regtable(self_0, &elanspi_calibration_table_new_page0);
            if chld.is_null() {
                err = fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_NOT_SUPPORTED,
                    b"unknown calibration table for sensor\0" as *const u8
                        as *const libc::c_char,
                );
                fpi_ssm_mark_failed(ssm, err);
                return;
            }
            fpi_ssm_start_subsm(ssm, chld);
            return;
        }
        4 => {
            xfer = elanspi_do_selectpage(self_0, 1 as libc::c_int as guint8);
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        5 => {
            chld = elanspi_write_regtable(self_0, &elanspi_calibration_table_new_page1);
            if chld.is_null() {
                err = fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_NOT_SUPPORTED,
                    b"unknown calibration table for sensor\0" as *const u8
                        as *const libc::c_char,
                );
                fpi_ssm_mark_failed(ssm, err);
                return;
            }
            fpi_ssm_start_subsm(ssm, chld);
            return;
        }
        7 | 11 => {
            if fpi_ssm_get_cur_state(ssm)
                == ELANSPI_CALIBHV_WRITE_BEST_GDAC_H as libc::c_int
            {
                (*self_0)
                    .c2rust_unnamed
                    .hv_data
                    .gdac_value = (*self_0).c2rust_unnamed.hv_data.best_gdac;
            }
            xfer = elanspi_write_register(
                self_0,
                0x6 as libc::c_int as guint8,
                ((*self_0).c2rust_unnamed.hv_data.gdac_value as libc::c_int
                    >> 2 as libc::c_int & 0xff as libc::c_int) as guint8,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        8 | 12 => {
            xfer = elanspi_write_register(
                self_0,
                0x7 as libc::c_int as guint8,
                ((*self_0).c2rust_unnamed.hv_data.gdac_value as libc::c_int
                    & 3 as libc::c_int) as guint8,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        9 => {
            chld = fpi_ssm_new_full(
                dev,
                Some(
                    elanspi_capture_hv_handler
                        as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                ),
                ELANSPI_CAPTHV_NSTATES as libc::c_int,
                ELANSPI_CAPTHV_NSTATES as libc::c_int,
                b"ELANSPI_CAPTHV_NSTATES\0" as *const u8 as *const libc::c_char,
            );
            fpi_ssm_silence_debug(chld);
            fpi_ssm_start_subsm(ssm, chld);
            return;
        }
        10 => {
            mean_diff = abs(
                elanspi_mean_image(self_0, (*self_0).last_image) - 3000 as libc::c_int,
            );
            if mean_diff < 100 as libc::c_int {
                g_log(
                    b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"<calibhv> calibration ok (mdiff < 100 w/ gdac=%04x)\0" as *const u8
                        as *const libc::c_char,
                    (*self_0).c2rust_unnamed.hv_data.gdac_value as libc::c_int,
                );
                fpi_ssm_jump_to_state(ssm, ELANSPI_CALIBHV_PROTECT as libc::c_int);
                return;
            }
            if mean_diff < (*self_0).c2rust_unnamed.hv_data.best_meandiff as libc::c_int
            {
                (*self_0).c2rust_unnamed.hv_data.best_meandiff = mean_diff as guint16;
                (*self_0)
                    .c2rust_unnamed
                    .hv_data
                    .best_gdac = (*self_0).c2rust_unnamed.hv_data.gdac_value;
            }
            (*self_0)
                .c2rust_unnamed
                .hv_data
                .gdac_step = ((*self_0).c2rust_unnamed.hv_data.gdac_step as libc::c_int
                / 2 as libc::c_int) as guint16;
            if (*self_0).c2rust_unnamed.hv_data.gdac_step as libc::c_int
                == 0 as libc::c_int
            {
                g_log(
                    b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"<calibhv> calibration ok (step = 0 w/ best_gdac=%04x)\0"
                        as *const u8 as *const libc::c_char,
                    (*self_0).c2rust_unnamed.hv_data.best_gdac as libc::c_int,
                );
                fpi_ssm_jump_to_state(
                    ssm,
                    ELANSPI_CALIBHV_WRITE_BEST_GDAC_H as libc::c_int,
                );
                return;
            }
            if elanspi_mean_image(self_0, (*self_0).last_image) < 3000 as libc::c_int {
                (*self_0)
                    .c2rust_unnamed
                    .hv_data
                    .gdac_value = ((*self_0).c2rust_unnamed.hv_data.gdac_value
                    as libc::c_int
                    - (*self_0).c2rust_unnamed.hv_data.gdac_step as libc::c_int)
                    as guint16;
            } else {
                (*self_0)
                    .c2rust_unnamed
                    .hv_data
                    .gdac_value = ((*self_0).c2rust_unnamed.hv_data.gdac_value
                    as libc::c_int
                    + (*self_0).c2rust_unnamed.hv_data.gdac_step as libc::c_int)
                    as guint16;
            }
            fpi_ssm_jump_to_state(ssm, ELANSPI_CALIBHV_WRITE_GDAC_H as libc::c_int);
            return;
        }
        13 => {
            g_log(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"<calibhv> calibration ok, saving bg image\0" as *const u8
                    as *const libc::c_char,
            );
            xfer = elanspi_write_register(
                self_0,
                0 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        _ => {
            return;
        }
    }
    xfer = elanspi_do_selectpage(self_0, 0 as libc::c_int as guint8);
    (*xfer).ssm = ssm;
    fpi_spi_transfer_submit(
        xfer,
        fpi_device_get_cancellable(dev),
        Some(
            fpi_ssm_spi_transfer_cb
                as unsafe extern "C" fn(
                    *mut FpiSpiTransfer,
                    *mut FpDevice,
                    gpointer,
                    *mut GError,
                ) -> (),
        ),
        0 as *mut libc::c_void,
    );
    return;
}
unsafe extern "C" fn elanspi_init_ssm_handler(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceElanSpi = FPI_DEVICE_ELANSPI(dev as gpointer);
    let mut xfer: *mut FpiSpiTransfer = 0 as *mut FpiSpiTransfer;
    let mut err: *mut GError = 0 as *mut GError;
    let mut chld: *mut FpiSsm = 0 as *mut FpiSsm;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            xfer = elanspi_read_status(self_0, &mut (*self_0).sensor_status);
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        1 => {
            g_log(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"<init> got status %02x\0" as *const u8 as *const libc::c_char,
                (*self_0).sensor_status as libc::c_int,
            );
            elanspi_do_hwreset(self_0, &mut err);
            g_log(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"<init> sync hw reset\0" as *const u8 as *const libc::c_char,
            );
            if !err.is_null() {
                g_log(
                    b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_CRITICAL,
                    b"<init> sync hw reset failed\0" as *const u8 as *const libc::c_char,
                );
                fpi_ssm_mark_failed(ssm, err);
                return;
            }
        }
        2 | 8 => {
            fpi_ssm_next_state_delayed(ssm, 4 as libc::c_int);
            return;
        }
        3 => {
            g_log(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"<init> sw reset ok\0" as *const u8 as *const libc::c_char,
            );
            xfer = elanspi_read_height(self_0, &mut (*self_0).sensor_height);
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        4 => {
            (*self_0)
                .sensor_height = ((*self_0).sensor_height as libc::c_int
                + 1 as libc::c_int) as guint8;
            g_log(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"<init> raw height = %d\0" as *const u8 as *const libc::c_char,
                (*self_0).sensor_height as libc::c_int,
            );
            xfer = elanspi_read_width(self_0, &mut (*self_0).sensor_width);
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        5 => {
            (*self_0)
                .sensor_width = ((*self_0).sensor_width as libc::c_int
                + 1 as libc::c_int) as guint8;
            g_log(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"<init> raw width = %d\0" as *const u8 as *const libc::c_char,
                (*self_0).sensor_width as libc::c_int,
            );
            xfer = elanspi_read_register(
                self_0,
                0x17 as libc::c_int as guint8,
                &mut (*self_0).sensor_reg_17,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        6 => {
            g_log(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"<init> raw reg17 = %d\0" as *const u8 as *const libc::c_char,
                (*self_0).sensor_reg_17 as libc::c_int,
            );
            xfer = elanspi_read_version(self_0, &mut (*self_0).sensor_raw_version);
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        7 => {
            g_log(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"<init> raw version = %02x\0" as *const u8 as *const libc::c_char,
                (*self_0).sensor_raw_version as libc::c_int,
            );
            elanspi_determine_sensor(self_0, &mut err);
            if !err.is_null() {
                g_log(
                    b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_CRITICAL,
                    b"<init> sensor detection error\0" as *const u8
                        as *const libc::c_char,
                );
                fpi_ssm_mark_failed(ssm, err);
                return;
            }
            let mut _pp: C2RustUnnamed_13 = C2RustUnnamed_13 {
                in_0: 0 as *mut libc::c_char,
            };
            let mut _p: gpointer = 0 as *mut libc::c_void;
            let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                GDestroyNotify,
            >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
            _pp.in_0 = &mut (*self_0).bg_image as *mut *mut guint16 as *mut libc::c_char;
            _p = *_pp.out;
            if !_p.is_null() {
                *_pp.out = 0 as *mut libc::c_void;
                _destroy.expect("non-null function pointer")(_p);
            }
            let mut _pp_0: C2RustUnnamed_12 = C2RustUnnamed_12 {
                in_0: 0 as *mut libc::c_char,
            };
            let mut _p_0: gpointer = 0 as *mut libc::c_void;
            let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                GDestroyNotify,
            >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
            _pp_0
                .in_0 = &mut (*self_0).last_image as *mut *mut guint16
                as *mut libc::c_char;
            _p_0 = *_pp_0.out;
            if !_p_0.is_null() {
                *_pp_0.out = 0 as *mut libc::c_void;
                _destroy_0.expect("non-null function pointer")(_p_0);
            }
            let mut _pp_1: C2RustUnnamed_11 = C2RustUnnamed_11 {
                in_0: 0 as *mut libc::c_char,
            };
            let mut _p_1: gpointer = 0 as *mut libc::c_void;
            let mut _destroy_1: GDestroyNotify = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                GDestroyNotify,
            >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
            _pp_1
                .in_0 = &mut (*self_0).prev_frame_image as *mut *mut guint16
                as *mut libc::c_char;
            _p_1 = *_pp_1.out;
            if !_p_1.is_null() {
                *_pp_1.out = 0 as *mut libc::c_void;
                _destroy_1.expect("non-null function pointer")(_p_1);
            }
            (*self_0)
                .last_image = g_malloc0(
                ((*self_0).sensor_width as libc::c_int
                    * (*self_0).sensor_height as libc::c_int * 2 as libc::c_int) as gsize,
            ) as *mut guint16;
            (*self_0)
                .bg_image = g_malloc0(
                ((*self_0).sensor_width as libc::c_int
                    * (*self_0).sensor_height as libc::c_int * 2 as libc::c_int) as gsize,
            ) as *mut guint16;
            (*self_0)
                .prev_frame_image = g_malloc0(
                ((*self_0).sensor_width as libc::c_int
                    * (*self_0).sensor_height as libc::c_int * 2 as libc::c_int) as gsize,
            ) as *mut guint16;
        }
        9 => {
            if (*self_0).sensor_otp == 0 {
                fpi_ssm_jump_to_state(ssm, ELANSPI_INIT_CALIBRATE as libc::c_int);
                return;
            }
            (*self_0)
                .c2rust_unnamed
                .old_data
                .otp_timeout = g_get_monotonic_time()
                + (12 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
            xfer = elanspi_read_register(
                self_0,
                0x3d as libc::c_int as guint8,
                &mut (*self_0).sensor_reg_vref1,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        10 => {
            (*self_0)
                .sensor_reg_vref1 = ((*self_0).sensor_reg_vref1 as libc::c_int
                & 0x3f as libc::c_int) as guint8;
            xfer = elanspi_write_register(
                self_0,
                0x3d as libc::c_int as guint8,
                (*self_0).sensor_reg_vref1,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        11 => {
            xfer = elanspi_write_register(
                self_0,
                0x28 as libc::c_int as guint8,
                0x78 as libc::c_int as guint8,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        12 => {
            xfer = elanspi_read_register(
                self_0,
                0x28 as libc::c_int as guint8,
                &mut (*self_0).sensor_reg_28,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        13 => {
            if (*self_0).sensor_reg_28 as libc::c_int & 0x40 as libc::c_int != 0 {
                g_log(
                    b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"<init/otp> looping\0" as *const u8 as *const libc::c_char,
                );
                fpi_ssm_jump_to_state(
                    ssm,
                    ELANSPI_INIT_OTP_LOOP_READ_0x28 as libc::c_int,
                );
                return;
            }
            xfer = elanspi_read_register(
                self_0,
                0x27 as libc::c_int as guint8,
                &mut (*self_0).sensor_reg_27,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        14 => {
            if (*self_0).sensor_reg_27 as libc::c_int & 0x80 as libc::c_int != 0 {
                (*self_0).sensor_vcm_mode = 2 as libc::c_int as guint8;
                fpi_ssm_jump_to_state(ssm, ELANSPI_INIT_OTP_WRITE_0xb as libc::c_int);
                return;
            }
            if (*self_0).sensor_reg_27 as libc::c_int & 6 as libc::c_int
                != 6 as libc::c_int
            {
                if g_get_monotonic_time() > (*self_0).c2rust_unnamed.old_data.otp_timeout
                {
                    g_log(
                        b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_WARNING,
                        b"<init/otp> timed out waiting for vcom detection\0" as *const u8
                            as *const libc::c_char,
                    );
                    (*self_0).sensor_vcm_mode = 2 as libc::c_int as guint8;
                    fpi_ssm_jump_to_state(
                        ssm,
                        ELANSPI_INIT_OTP_WRITE_0xb as libc::c_int,
                    );
                    return;
                }
                g_log(
                    b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"<init/otp> looping\0" as *const u8 as *const libc::c_char,
                );
                fpi_ssm_jump_to_state(
                    ssm,
                    ELANSPI_INIT_OTP_LOOP_READ_0x28 as libc::c_int,
                );
                return;
            }
            (*self_0)
                .sensor_vcm_mode = (((*self_0).sensor_reg_27 as libc::c_int
                & 1 as libc::c_int) + 1 as libc::c_int) as guint8;
            xfer = elanspi_read_register(
                self_0,
                0x7 as libc::c_int as guint8,
                &mut (*self_0).sensor_reg_dac2,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        15 => {
            (*self_0)
                .sensor_reg_dac2 = ((*self_0).sensor_reg_dac2 as libc::c_int
                | 0x80 as libc::c_int) as guint8;
            xfer = elanspi_write_register(
                self_0,
                0x7 as libc::c_int as guint8,
                (*self_0).sensor_reg_dac2,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        16 => {
            xfer = elanspi_write_register(
                self_0,
                0xa as libc::c_int as guint8,
                0x97 as libc::c_int as guint8,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        17 => {
            g_log(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"<init/otp> got vcm mode = %d\0" as *const u8 as *const libc::c_char,
                (*self_0).sensor_vcm_mode as libc::c_int,
            );
            if (*self_0).sensor_vcm_mode as libc::c_int == 0 as libc::c_int {
                fpi_ssm_jump_to_state(ssm, ELANSPI_INIT_CALIBRATE as libc::c_int);
                return;
            }
            xfer = elanspi_write_register(
                self_0,
                0xb as libc::c_int as guint8,
                (if (*self_0).sensor_vcm_mode as libc::c_int == 2 as libc::c_int {
                    0x72 as libc::c_int
                } else {
                    0x71 as libc::c_int
                }) as guint8,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        18 => {
            xfer = elanspi_write_register(
                self_0,
                0xc as libc::c_int as guint8,
                (if (*self_0).sensor_vcm_mode as libc::c_int == 2 as libc::c_int {
                    0x62 as libc::c_int
                } else {
                    0x49 as libc::c_int
                }) as guint8,
            );
            (*xfer).ssm = ssm;
            fpi_spi_transfer_submit(
                xfer,
                fpi_device_get_cancellable(dev),
                Some(
                    fpi_ssm_spi_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiSpiTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            return;
        }
        19 => {
            g_log(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"<init/calibrate> starting calibrate\0" as *const u8
                    as *const libc::c_char,
            );
            if (*self_0).sensor_id as libc::c_int == 0xe as libc::c_int {
                chld = fpi_ssm_new_full(
                    dev,
                    Some(
                        elanspi_calibrate_hv_handler
                            as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                    ),
                    ELANSPI_CALIBHV_NSTATES as libc::c_int,
                    ELANSPI_CALIBHV_PROTECT as libc::c_int,
                    b"HV calibrate\0" as *const u8 as *const libc::c_char,
                );
            } else {
                chld = fpi_ssm_new_full(
                    dev,
                    Some(
                        elanspi_calibrate_old_handler
                            as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                    ),
                    ELANSPI_CALIBOLD_NSTATES as libc::c_int,
                    ELANSPI_CALIBOLD_PROTECT as libc::c_int,
                    b"old calibrate\0" as *const u8 as *const libc::c_char,
                );
            }
            fpi_ssm_silence_debug(chld);
            fpi_ssm_start_subsm(ssm, chld);
            return;
        }
        20 => {
            if (*self_0).sensor_id as libc::c_int == 0xe as libc::c_int {
                chld = fpi_ssm_new_full(
                    dev,
                    Some(
                        elanspi_capture_hv_handler
                            as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                    ),
                    ELANSPI_CAPTHV_NSTATES as libc::c_int,
                    ELANSPI_CAPTHV_NSTATES as libc::c_int,
                    b"ELANSPI_CAPTHV_NSTATES\0" as *const u8 as *const libc::c_char,
                );
            } else {
                chld = fpi_ssm_new_full(
                    dev,
                    Some(
                        elanspi_capture_old_handler
                            as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                    ),
                    ELANSPI_CAPTOLD_NSTATES as libc::c_int,
                    ELANSPI_CAPTOLD_NSTATES as libc::c_int,
                    b"ELANSPI_CAPTOLD_NSTATES\0" as *const u8 as *const libc::c_char,
                );
            }
            fpi_ssm_silence_debug(chld);
            fpi_ssm_start_subsm(ssm, chld);
            return;
        }
        21 => {
            memcpy(
                (*self_0).bg_image as *mut libc::c_void,
                (*self_0).last_image as *const libc::c_void,
                ((*self_0).sensor_height as libc::c_int
                    * (*self_0).sensor_width as libc::c_int * 2 as libc::c_int)
                    as libc::c_ulong,
            );
            fpi_ssm_mark_completed(ssm);
            return;
        }
        _ => {
            return;
        }
    }
    xfer = elanspi_do_swreset(self_0);
    (*xfer).ssm = ssm;
    fpi_spi_transfer_submit(
        xfer,
        fpi_device_get_cancellable(dev),
        Some(
            fpi_ssm_spi_transfer_cb
                as unsafe extern "C" fn(
                    *mut FpiSpiTransfer,
                    *mut FpDevice,
                    gpointer,
                    *mut GError,
                ) -> (),
        ),
        0 as *mut libc::c_void,
    );
    return;
}
unsafe extern "C" fn elanspi_correct_with_bg(
    mut self_0: *mut FpiDeviceElanSpi,
    mut raw_image: *mut guint16,
) -> gint {
    let mut count: gint = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i
        < (*self_0).sensor_width as libc::c_int * (*self_0).sensor_height as libc::c_int
    {
        if (*raw_image.offset(i as isize) as libc::c_int)
            < *((*self_0).bg_image).offset(i as isize) as libc::c_int
        {
            count += 1 as libc::c_int;
            *raw_image.offset(i as isize) = 0 as libc::c_int as guint16;
        } else {
            let ref mut fresh0 = *raw_image.offset(i as isize);
            *fresh0 = (*fresh0 as libc::c_int
                - *((*self_0).bg_image).offset(i as isize) as libc::c_int) as guint16;
        }
        i += 1 as libc::c_int;
    }
    return count;
}
unsafe extern "C" fn elanspi_lookup_pixel_with_rotation(
    mut self_0: *mut FpiDeviceElanSpi,
    mut data_in: *const guint16,
    mut y: libc::c_int,
    mut x: libc::c_int,
) -> guint16 {
    let mut rotation: libc::c_int = (fpi_device_get_driver_data(
        FP_DEVICE(self_0 as gpointer),
    ) & 3 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut x1: gint = x;
    let mut y1: gint = y;
    if rotation == 2 as libc::c_int {
        x1 = (*self_0).sensor_width as libc::c_int - x - 1 as libc::c_int;
        y1 = (*self_0).sensor_height as libc::c_int - y - 1 as libc::c_int;
    } else if rotation == 1 as libc::c_int {
        x1 = y;
        y1 = (*self_0).sensor_width as libc::c_int - x - 1 as libc::c_int;
    } else if rotation == 3 as libc::c_int {
        x1 = (*self_0).sensor_height as libc::c_int - y - 1 as libc::c_int;
        y1 = x;
    }
    return *data_in.offset((y1 * (*self_0).sensor_width as libc::c_int + x1) as isize);
}
unsafe extern "C" fn elanspi_guess_image(
    mut self_0: *mut FpiDeviceElanSpi,
    mut raw_image: *mut guint16,
) -> elanspi_guess_result {
    let mut image_copy: *mut guint16 = g_malloc0(
        ((*self_0).sensor_height as libc::c_int * (*self_0).sensor_width as libc::c_int
            * 2 as libc::c_int) as gsize,
    ) as *mut guint16;
    let mut frame_width: guint8 = 0;
    let mut frame_height: guint8 = 0;
    frame_width = (*self_0).frame_width;
    frame_height = (*self_0).frame_height;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if frame_width as libc::c_int != 0 && frame_height as libc::c_int != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/elanspi.c\0" as *const u8 as *const libc::c_char,
            1207 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"elanspi_guess_image\0"))
                .as_ptr(),
            b"frame_width && frame_height\0" as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        image_copy as *mut libc::c_void,
        raw_image as *const libc::c_void,
        ((*self_0).sensor_height as libc::c_int * (*self_0).sensor_width as libc::c_int
            * 2 as libc::c_int) as libc::c_ulong,
    );
    let mut invalid_percent: gint = 100 as libc::c_int
        * elanspi_correct_with_bg(self_0, image_copy)
        / ((*self_0).sensor_height as libc::c_int
            * (*self_0).sensor_width as libc::c_int);
    let mut is_fp: gint = 0 as libc::c_int;
    let mut is_empty: gint = 0 as libc::c_int;
    let mut mean: gint64 = 0 as libc::c_int as gint64;
    let mut sq_stddev: gint64 = 0 as libc::c_int as gint64;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < frame_height as libc::c_int {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < frame_width as libc::c_int {
            mean
                += elanspi_lookup_pixel_with_rotation(self_0, image_copy, j, i)
                    as gint64;
            i += 1 as libc::c_int;
        }
        j += 1 as libc::c_int;
    }
    mean /= (frame_width as libc::c_int * frame_height as libc::c_int) as libc::c_long;
    let mut j_0: libc::c_int = 0 as libc::c_int;
    while j_0 < frame_height as libc::c_int {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < frame_width as libc::c_int {
            let mut k: gint64 = elanspi_lookup_pixel_with_rotation(
                self_0,
                image_copy,
                j_0,
                i_0,
            ) as gint64 - mean;
            sq_stddev += k * k;
            i_0 += 1 as libc::c_int;
        }
        j_0 += 1 as libc::c_int;
    }
    sq_stddev
        /= (frame_width as libc::c_int * frame_height as libc::c_int) as libc::c_long;
    if invalid_percent < 3 as libc::c_int {
        is_fp += 1 as libc::c_int;
    }
    if invalid_percent > 6 as libc::c_int {
        is_empty += 1 as libc::c_int;
    }
    if sq_stddev > (592 as libc::c_int * 592 as libc::c_int) as libc::c_long {
        is_fp += 1 as libc::c_int;
    }
    if sq_stddev < (350 as libc::c_int * 350 as libc::c_int) as libc::c_long {
        is_empty += 1 as libc::c_int;
    }
    g_log(
        b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"<guess> stddev=%lud, ip=%d, is_fp=%d, is_empty=%d\0" as *const u8
            as *const libc::c_char,
        sq_stddev,
        invalid_percent,
        is_fp,
        is_empty,
    );
    if is_fp > is_empty {
        return ELANSPI_GUESS_FINGERPRINT
    } else if is_empty > is_fp {
        return ELANSPI_GUESS_EMPTY
    } else {
        return ELANSPI_GUESS_UNKNOWN
    };
}
unsafe extern "C" fn elanspi_check_waitupdown_done(
    mut self_0: *mut FpiDeviceElanSpi,
    mut target: elanspi_guess_result,
) -> gboolean {
    let mut guess: elanspi_guess_result = elanspi_guess_image(
        self_0,
        (*self_0).last_image,
    );
    if guess as libc::c_uint == ELANSPI_GUESS_UNKNOWN as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if guess as libc::c_uint == target as libc::c_uint {
        (*self_0).finger_wait_debounce += 1 as libc::c_int;
        return ((*self_0).finger_wait_debounce == 2 as libc::c_int) as libc::c_int;
    } else {
        (*self_0).finger_wait_debounce = 0 as libc::c_int;
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn cmp_u16(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return *(a as *mut guint16) as libc::c_int - *(b as *mut guint16) as libc::c_int;
}
unsafe extern "C" fn elanspi_process_frame(
    mut self_0: *mut FpiDeviceElanSpi,
    mut data_in: *const guint16,
    mut data_out: *mut guint8,
) {
    let mut frame_size: size_t = ((*self_0).frame_width as libc::c_int
        * (*self_0).frame_height as libc::c_int) as size_t;
    let vla = frame_size as usize;
    let mut data_in_sorted: Vec::<guint16> = ::std::vec::from_elem(0, vla);
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut offset: libc::c_int = 0 as libc::c_int;
    while i < (*self_0).frame_height as libc::c_int {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < (*self_0).frame_width as libc::c_int {
            let fresh1 = offset;
            offset = offset + 1;
            *data_in_sorted
                .as_mut_ptr()
                .offset(
                    fresh1 as isize,
                ) = elanspi_lookup_pixel_with_rotation(self_0, data_in, i, j);
            j += 1 as libc::c_int;
        }
        i += 1 as libc::c_int;
    }
    qsort(
        data_in_sorted.as_mut_ptr() as *mut libc::c_void,
        frame_size,
        2 as libc::c_int as size_t,
        Some(
            cmp_u16
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut lvl0: guint16 = *data_in_sorted
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize);
    let mut lvl1: guint16 = *data_in_sorted
        .as_mut_ptr()
        .offset(
            frame_size
                .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                .wrapping_div(10 as libc::c_int as libc::c_ulong) as isize,
        );
    let mut lvl2: guint16 = *data_in_sorted
        .as_mut_ptr()
        .offset(
            frame_size
                .wrapping_mul(65 as libc::c_int as libc::c_ulong)
                .wrapping_div(100 as libc::c_int as libc::c_ulong) as isize,
        );
    let mut lvl3: guint16 = *data_in_sorted
        .as_mut_ptr()
        .offset(frame_size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    lvl1 = (if lvl1 as libc::c_int > lvl0 as libc::c_int + 1 as libc::c_int {
        lvl1 as libc::c_int
    } else {
        lvl0 as libc::c_int + 1 as libc::c_int
    }) as guint16;
    lvl2 = (if lvl2 as libc::c_int > lvl1 as libc::c_int + 1 as libc::c_int {
        lvl2 as libc::c_int
    } else {
        lvl1 as libc::c_int + 1 as libc::c_int
    }) as guint16;
    lvl3 = (if lvl3 as libc::c_int > lvl2 as libc::c_int + 1 as libc::c_int {
        lvl3 as libc::c_int
    } else {
        lvl2 as libc::c_int + 1 as libc::c_int
    }) as guint16;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < (*self_0).frame_height as libc::c_int {
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < (*self_0).frame_width as libc::c_int {
            let mut px: guint16 = elanspi_lookup_pixel_with_rotation(
                self_0,
                data_in,
                i_0,
                j_0,
            );
            if (px as libc::c_int) < lvl0 as libc::c_int {
                px = 0 as libc::c_int as guint16;
            } else if px as libc::c_int > lvl3 as libc::c_int {
                px = 255 as libc::c_int as guint16;
            } else if lvl0 as libc::c_int <= px as libc::c_int
                && (px as libc::c_int) < lvl1 as libc::c_int
            {
                px = ((px as libc::c_int - lvl0 as libc::c_int) * 99 as libc::c_int
                    / (lvl1 as libc::c_int - lvl0 as libc::c_int)) as guint16;
            } else if lvl1 as libc::c_int <= px as libc::c_int
                && (px as libc::c_int) < lvl2 as libc::c_int
            {
                px = (99 as libc::c_int
                    + (px as libc::c_int - lvl1 as libc::c_int) * 56 as libc::c_int
                        / (lvl2 as libc::c_int - lvl1 as libc::c_int)) as guint16;
            } else {
                px = (155 as libc::c_int
                    + (px as libc::c_int - lvl2 as libc::c_int) * 100 as libc::c_int
                        / (lvl3 as libc::c_int - lvl2 as libc::c_int)) as guint16;
            }
            *data_out = px as guint8;
            data_out = data_out.offset(1 as libc::c_int as isize);
            j_0 += 1 as libc::c_int;
        }
        i_0 += 1 as libc::c_int;
    }
}
unsafe extern "C" fn elanspi_fp_assembling_get_pixel(
    mut ctx: *mut fpi_frame_asmbl_ctx,
    mut frame: *mut fpi_frame,
    mut x: libc::c_uint,
    mut y: libc::c_uint,
) -> libc::c_uchar {
    return *((*frame).data)
        .as_mut_ptr()
        .offset(y.wrapping_mul((*ctx).frame_width).wrapping_add(x) as isize);
}
unsafe extern "C" fn elanspi_fp_frame_stitch_and_submit(
    mut self_0: *mut FpiDeviceElanSpi,
) {
    let mut img: FpImage_autoptr = 0 as FpImage_autoptr;
    let mut scaled: FpImage_autoptr = 0 as FpImage_autoptr;
    let mut assembling_ctx: fpi_frame_asmbl_ctx = {
        let mut init = fpi_frame_asmbl_ctx {
            frame_width: (*self_0).frame_width as libc::c_uint,
            frame_height: (*self_0).frame_height as libc::c_uint,
            image_width: ((*self_0).frame_width as libc::c_int * 3 as libc::c_int
                / 2 as libc::c_int) as libc::c_uint,
            get_pixel: Some(
                elanspi_fp_assembling_get_pixel
                    as unsafe extern "C" fn(
                        *mut fpi_frame_asmbl_ctx,
                        *mut fpi_frame,
                        libc::c_uint,
                        libc::c_uint,
                    ) -> libc::c_uchar,
            ),
        };
        init
    };
    let mut frame_start: *mut GSList = g_slist_nth(
        (*self_0).fp_frame_list,
        1 as libc::c_int as guint,
    );
    fpi_do_movement_estimation(&mut assembling_ctx, frame_start);
    img = fpi_assemble_frames(&mut assembling_ctx, frame_start);
    scaled = fpi_image_resize(img, 2 as libc::c_int as guint, 2 as libc::c_int as guint);
    (*scaled)
        .flags = ::core::mem::transmute::<
        libc::c_uint,
        FpiImageFlags,
    >(
        (*scaled).flags as libc::c_uint
            | (FPI_IMAGE_PARTIAL as libc::c_int
                | FPI_IMAGE_COLORS_INVERTED as libc::c_int) as libc::c_uint,
    );
    fpi_image_device_image_captured(
        FP_IMAGE_DEVICE(self_0 as gpointer),
        (if 0 as libc::c_int != 0 {
            scaled as *mut libc::c_void
        } else {
            g_steal_pointer(&mut scaled as *mut FpImage_autoptr as gpointer)
        }) as *mut FpImage,
    );
    g_slist_free_full(
        (if 0 as libc::c_int != 0 {
            (*self_0).fp_frame_list as *mut libc::c_void
        } else {
            g_steal_pointer(&mut (*self_0).fp_frame_list as *mut *mut GSList as gpointer)
        }) as *mut GSList,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
unsafe extern "C" fn elanspi_get_frame_diff_stddev_sq(
    mut self_0: *mut FpiDeviceElanSpi,
    mut frame1: *mut guint16,
    mut frame2: *mut guint16,
) -> gint64 {
    let mut mean: gint64 = 0 as libc::c_int as gint64;
    let mut sq_stddev: gint64 = 0 as libc::c_int as gint64;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j
        < (*self_0).sensor_height as libc::c_int * (*self_0).sensor_width as libc::c_int
    {
        mean
            += abs(
                *frame1.offset(j as isize) as libc::c_int
                    - *frame2.offset(j as isize) as libc::c_int,
            ) as libc::c_long;
        j += 1 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*self_0).sensor_height as libc::c_int != 0
            && (*self_0).sensor_width as libc::c_int != 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/elanspi.c\0" as *const u8 as *const libc::c_char,
            1371 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"elanspi_get_frame_diff_stddev_sq\0"))
                .as_ptr(),
            b"self->sensor_height && self->sensor_width\0" as *const u8
                as *const libc::c_char,
        );
    }
    mean
        /= ((*self_0).sensor_height as libc::c_int
            * (*self_0).sensor_width as libc::c_int) as libc::c_long;
    let mut j_0: libc::c_int = 0 as libc::c_int;
    while j_0
        < (*self_0).sensor_height as libc::c_int * (*self_0).sensor_width as libc::c_int
    {
        let mut k: gint64 = abs(
            *frame1.offset(j_0 as isize) as libc::c_int
                - *frame2.offset(j_0 as isize) as libc::c_int,
        ) as libc::c_long - mean;
        sq_stddev += k * k;
        j_0 += 1 as libc::c_int;
    }
    sq_stddev
        /= ((*self_0).sensor_height as libc::c_int
            * (*self_0).sensor_width as libc::c_int) as libc::c_long;
    return sq_stddev;
}
unsafe extern "C" fn elanspi_fp_frame_handler(
    mut ssm: *mut FpiSsm,
    mut self_0: *mut FpiDeviceElanSpi,
) {
    let mut current_block: u64;
    let mut this_frame: *mut fpi_frame = 0 as *mut fpi_frame;
    match elanspi_guess_image(self_0, (*self_0).last_image) as libc::c_uint {
        2 => {
            g_log(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"<fp_frame> unknown, ignore...\0" as *const u8 as *const libc::c_char,
            );
            current_block = 14359455889292382949;
        }
        1 => {
            (*self_0).fp_empty_counter += 1 as libc::c_int;
            g_log(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"<fp_frame> got empty\0" as *const u8 as *const libc::c_char,
            );
            if (*self_0).fp_empty_counter > 1 as libc::c_int {
                g_log(
                    b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"<fp_frame> have enough debounce\0" as *const u8
                        as *const libc::c_char,
                );
                if g_slist_length((*self_0).fp_frame_list)
                    >= (7 as libc::c_int + 1 as libc::c_int) as libc::c_uint
                {
                    g_log(
                        b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"<fp_frame> have enough frames, submitting\0" as *const u8
                            as *const libc::c_char,
                    );
                    elanspi_fp_frame_stitch_and_submit(self_0);
                } else {
                    g_log(
                        b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"<fp_frame> not enough frames, reporting short swipe\0"
                            as *const u8 as *const libc::c_char,
                    );
                    fpi_image_device_retry_scan(
                        FP_IMAGE_DEVICE(self_0 as gpointer),
                        FP_DEVICE_RETRY_TOO_SHORT,
                    );
                }
                current_block = 2334949924075475812;
            } else {
                current_block = 14359455889292382949;
            }
        }
        0 => {
            if (*self_0).fp_empty_counter != 0 && !((*self_0).fp_frame_list).is_null() {
                if (*self_0).fp_empty_counter < 1 as libc::c_int {
                    g_log(
                        b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"<fp_frame> possible bounced fp\0" as *const u8
                            as *const libc::c_char,
                    );
                    current_block = 14359455889292382949;
                } else {
                    g_log(
                        b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"<fp_frame> too many empties, clearing list\0" as *const u8
                            as *const libc::c_char,
                    );
                    g_slist_free_full(
                        (if 0 as libc::c_int != 0 {
                            (*self_0).fp_frame_list as *mut libc::c_void
                        } else {
                            g_steal_pointer(
                                &mut (*self_0).fp_frame_list as *mut *mut GSList as gpointer,
                            )
                        }) as *mut GSList,
                        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
                    );
                    (*self_0).fp_empty_counter = 0 as libc::c_int;
                    current_block = 6009453772311597924;
                }
            } else {
                current_block = 6009453772311597924;
            }
            match current_block {
                14359455889292382949 => {}
                _ => {
                    if g_slist_length((*self_0).fp_frame_list)
                        > (20 as libc::c_int + 1 as libc::c_int) as libc::c_uint
                    {
                        g_log(
                            b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                            G_LOG_LEVEL_DEBUG,
                            b"<fp_frame> have enough frames, exiting now\0" as *const u8
                                as *const libc::c_char,
                        );
                        elanspi_fp_frame_stitch_and_submit(self_0);
                        current_block = 2334949924075475812;
                    } else {
                        this_frame = g_malloc0(
                            (((*self_0).sensor_height as libc::c_int
                                * (*self_0).sensor_width as libc::c_int) as libc::c_ulong)
                                .wrapping_add(
                                    ::core::mem::size_of::<fpi_frame>() as libc::c_ulong,
                                ),
                        ) as *mut fpi_frame;
                        elanspi_correct_with_bg(self_0, (*self_0).last_image);
                        elanspi_process_frame(
                            self_0,
                            (*self_0).last_image,
                            ((*this_frame).data).as_mut_ptr(),
                        );
                        if !((*self_0).fp_frame_list).is_null() {
                            let mut difference: gint = elanspi_get_frame_diff_stddev_sq(
                                self_0,
                                (*self_0).last_image,
                                (*self_0).prev_frame_image,
                            ) as gint;
                            g_log(
                                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                                G_LOG_LEVEL_DEBUG,
                                b"<fp_frame> diff = %d\0" as *const u8
                                    as *const libc::c_char,
                                difference,
                            );
                            if difference < 250 as libc::c_int * 250 as libc::c_int {
                                g_log(
                                    b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                                    G_LOG_LEVEL_DEBUG,
                                    b"<fp_frame> ignoring b.c. difference is too small\0"
                                        as *const u8 as *const libc::c_char,
                                );
                                current_block = 14359455889292382949;
                            } else {
                                current_block = 11057878835866523405;
                            }
                        } else {
                            current_block = 11057878835866523405;
                        }
                        match current_block {
                            14359455889292382949 => {}
                            _ => {
                                (*self_0)
                                    .fp_frame_list = g_slist_prepend(
                                    (*self_0).fp_frame_list,
                                    if 0 as libc::c_int != 0 {
                                        this_frame as *mut libc::c_void
                                    } else {
                                        g_steal_pointer(
                                            &mut this_frame as *mut *mut fpi_frame as gpointer,
                                        )
                                    },
                                );
                                memcpy(
                                    (*self_0).prev_frame_image as *mut libc::c_void,
                                    (*self_0).last_image as *const libc::c_void,
                                    ((*self_0).sensor_height as libc::c_int
                                        * (*self_0).sensor_width as libc::c_int * 2 as libc::c_int)
                                        as libc::c_ulong,
                                );
                                current_block = 14359455889292382949;
                            }
                        }
                    }
                }
            }
        }
        _ => {
            current_block = 14359455889292382949;
        }
    }
    match current_block {
        14359455889292382949 => {
            if (*self_0).sensor_id as libc::c_int == 0xe as libc::c_int {
                fpi_ssm_jump_to_state_delayed(
                    ssm,
                    ELANSPI_FPCAPT_FP_CAPTURE as libc::c_int,
                    23 as libc::c_int,
                );
            } else {
                fpi_ssm_jump_to_state(ssm, ELANSPI_FPCAPT_FP_CAPTURE as libc::c_int);
            }
            return;
        }
        _ => {
            (*self_0).finger_wait_debounce = 0 as libc::c_int;
            fpi_ssm_jump_to_state(ssm, ELANSPI_FPCAPT_WAITUP_CAPTURE as libc::c_int);
            return;
        }
    };
}
unsafe extern "C" fn elanspi_fp_capture_ssm_handler(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceElanSpi = FPI_DEVICE_ELANSPI(dev as gpointer);
    let mut chld: *mut FpiSsm = 0 as *mut FpiSsm;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            (*self_0).finger_wait_debounce = 0 as libc::c_int;
            fpi_ssm_next_state(ssm);
            return;
        }
        1 | 5 | 3 => {
            if (*self_0).deactivating != 0 {
                g_log(
                    b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"<capture> got deactivate; exiting\0" as *const u8
                        as *const libc::c_char,
                );
                (*self_0).deactivating = 0 as libc::c_int;
                fpi_ssm_mark_completed(ssm);
                fpi_image_device_deactivate_complete(
                    FP_IMAGE_DEVICE(dev as gpointer),
                    0 as *mut GError,
                );
                return;
            }
            if (*self_0).sensor_id as libc::c_int == 0xe as libc::c_int {
                chld = fpi_ssm_new_full(
                    dev,
                    Some(
                        elanspi_capture_hv_handler
                            as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                    ),
                    ELANSPI_CAPTHV_NSTATES as libc::c_int,
                    ELANSPI_CAPTHV_NSTATES as libc::c_int,
                    b"ELANSPI_CAPTHV_NSTATES\0" as *const u8 as *const libc::c_char,
                );
            } else {
                chld = fpi_ssm_new_full(
                    dev,
                    Some(
                        elanspi_capture_old_handler
                            as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                    ),
                    ELANSPI_CAPTOLD_NSTATES as libc::c_int,
                    ELANSPI_CAPTOLD_NSTATES as libc::c_int,
                    b"ELANSPI_CAPTOLD_NSTATES\0" as *const u8 as *const libc::c_char,
                );
            }
            fpi_ssm_silence_debug(chld);
            fpi_ssm_start_subsm(ssm, chld);
            return;
        }
        2 => {
            if elanspi_check_waitupdown_done(self_0, ELANSPI_GUESS_FINGERPRINT) == 0 {
                fpi_ssm_jump_to_state(
                    ssm,
                    ELANSPI_FPCAPT_WAITDOWN_CAPTURE as libc::c_int,
                );
                return;
            }
            (*self_0).finger_wait_debounce = 0 as libc::c_int;
            g_slist_free_full(
                (if 0 as libc::c_int != 0 {
                    (*self_0).fp_frame_list as *mut libc::c_void
                } else {
                    g_steal_pointer(
                        &mut (*self_0).fp_frame_list as *mut *mut GSList as gpointer,
                    )
                }) as *mut GSList,
                Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            );
            (*self_0).fp_empty_counter = 0 as libc::c_int;
            fpi_image_device_report_finger_status(
                FP_IMAGE_DEVICE(self_0 as gpointer),
                (0 as libc::c_int == 0) as libc::c_int,
            );
            fpi_ssm_jump_to_state(ssm, ELANSPI_FPCAPT_FP_CAPTURE as libc::c_int);
            return;
        }
        4 => {
            elanspi_fp_frame_handler(ssm, self_0);
            return;
        }
        6 => {
            if elanspi_check_waitupdown_done(self_0, ELANSPI_GUESS_EMPTY) == 0 {
                fpi_ssm_jump_to_state(ssm, ELANSPI_FPCAPT_WAITUP_CAPTURE as libc::c_int);
                return;
            }
            (*self_0).capturing = 0 as libc::c_int;
            fpi_image_device_report_finger_status(
                FP_IMAGE_DEVICE(self_0 as gpointer),
                0 as libc::c_int,
            );
            fpi_ssm_mark_completed(ssm);
            return;
        }
        _ => {}
    };
}
unsafe extern "C" fn elanspi_open(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceElanSpi = FPI_DEVICE_ELANSPI(dev as gpointer);
    let mut err: *mut GError = 0 as *mut GError;
    g_log_structured(
        b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elanspi.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"1562\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"elanspi_open\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elanspi.c:1562\0" as *const u8 as *const libc::c_char,
    );
    let mut spi_fd: libc::c_int = open(
        fpi_device_get_udev_data(
            FP_DEVICE(dev as gpointer),
            FPI_DEVICE_UDEV_SUBTYPE_SPIDEV,
        ) as *const libc::c_char,
        0o2 as libc::c_int,
    );
    if spi_fd < 0 as libc::c_int {
        g_set_error(
            &mut err as *mut *mut GError,
            g_io_error_quark(),
            g_io_error_from_errno(*__errno_location()) as gint,
            b"unable to open spi\0" as *const u8 as *const libc::c_char,
        );
        fpi_image_device_open_complete(dev, err);
        return;
    }
    (*self_0).spi_fd = spi_fd;
    fpi_image_device_open_complete(dev, 0 as *mut GError);
}
unsafe extern "C" fn elanspi_close(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceElanSpi = FPI_DEVICE_ELANSPI(dev as gpointer);
    if (*self_0).spi_fd >= 0 as libc::c_int {
        close((*self_0).spi_fd);
        (*self_0).spi_fd = -(1 as libc::c_int);
    }
    fpi_image_device_close_complete(dev, 0 as *mut GError);
}
unsafe extern "C" fn elanspi_init_finish(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut idev: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    g_log_structured(
        b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elanspi.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"1596\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"elanspi_init_finish\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elanspi.c:1596\0" as *const u8 as *const libc::c_char,
    );
    fpi_image_device_activate_complete(idev, error);
}
unsafe extern "C" fn elanspi_activate(mut dev: *mut FpImageDevice) {
    let mut ssm: *mut FpiSsm = fpi_ssm_new_full(
        FP_DEVICE(dev as gpointer),
        Some(
            elanspi_init_ssm_handler
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        ELANSPI_INIT_NSTATES as libc::c_int,
        ELANSPI_INIT_NSTATES as libc::c_int,
        b"ELANSPI_INIT_NSTATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            elanspi_init_finish
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn elanspi_deactivate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceElanSpi = FPI_DEVICE_ELANSPI(dev as gpointer);
    if (*self_0).capturing != 0 {
        (*self_0).deactivating = (0 as libc::c_int == 0) as libc::c_int;
        g_log(
            b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"<deactivate> waiting capture to stop\0" as *const u8 as *const libc::c_char,
        );
    } else {
        fpi_image_device_deactivate_complete(dev, 0 as *mut GError);
    };
}
unsafe extern "C" fn elanspi_fp_capture_finish(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut idev: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    let mut self_0: *mut FpiDeviceElanSpi = FPI_DEVICE_ELANSPI(dev as gpointer);
    (*self_0).capturing = 0 as libc::c_int;
    if (*self_0).deactivating != 0 {
        if !error.is_null() {
            g_error_free(error);
        }
        (*self_0).deactivating = 0 as libc::c_int;
        fpi_image_device_deactivate_complete(idev, 0 as *mut GError);
        return;
    }
    if !error.is_null() {
        fpi_image_device_session_error(idev, error);
    }
}
unsafe extern "C" fn elanspi_change_state(
    mut dev: *mut FpImageDevice,
    mut state: FpiImageDeviceState,
) {
    let mut self_0: *mut FpiDeviceElanSpi = FPI_DEVICE_ELANSPI(dev as gpointer);
    if state as libc::c_uint
        == FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_ON as libc::c_int as libc::c_uint
    {
        if ({
            let mut _g_boolean_var_: libc::c_int = 0;
            if (*self_0).capturing == 0 as libc::c_int {
                _g_boolean_var_ = 1 as libc::c_int;
            } else {
                _g_boolean_var_ = 0 as libc::c_int;
            }
            _g_boolean_var_
        }) as libc::c_long != 0
        {} else {
            g_assertion_message_expr(
                b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
                b"../libfprint/drivers/elanspi.c\0" as *const u8 as *const libc::c_char,
                1654 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"elanspi_change_state\0"))
                    .as_ptr(),
                b"self->capturing == FALSE\0" as *const u8 as *const libc::c_char,
            );
        }
        (*self_0).capturing = (0 as libc::c_int == 0) as libc::c_int;
        fpi_ssm_start(
            fpi_ssm_new_full(
                FP_DEVICE(dev as gpointer),
                Some(
                    elanspi_fp_capture_ssm_handler
                        as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                ),
                ELANSPI_FPCAPT_NSTATES as libc::c_int,
                ELANSPI_FPCAPT_NSTATES as libc::c_int,
                b"ELANSPI_FPCAPT_NSTATES\0" as *const u8 as *const libc::c_char,
            ),
            Some(
                elanspi_fp_capture_finish
                    as unsafe extern "C" fn(
                        *mut FpiSsm,
                        *mut FpDevice,
                        *mut GError,
                    ) -> (),
            ),
        );
        g_log(
            b"libfprint-elanspi\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"<change_state> started capturer\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn fpi_device_elanspi_init(mut self_0: *mut FpiDeviceElanSpi) {
    (*self_0).spi_fd = -(1 as libc::c_int);
    (*self_0).sensor_id = 0xff as libc::c_int as guint8;
}
unsafe extern "C" fn fpi_device_elanspi_finalize(mut this: *mut GObject) {
    let mut self_0: *mut FpiDeviceElanSpi = FPI_DEVICE_ELANSPI(this as gpointer);
    let mut _pp: C2RustUnnamed_10 = C2RustUnnamed_10 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*self_0).bg_image as *mut *mut guint16 as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    let mut _pp_0: C2RustUnnamed_9 = C2RustUnnamed_9 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_0: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp_0.in_0 = &mut (*self_0).last_image as *mut *mut guint16 as *mut libc::c_char;
    _p_0 = *_pp_0.out;
    if !_p_0.is_null() {
        *_pp_0.out = 0 as *mut libc::c_void;
        _destroy_0.expect("non-null function pointer")(_p_0);
    }
    let mut _pp_1: C2RustUnnamed_8 = C2RustUnnamed_8 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_1: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_1: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp_1
        .in_0 = &mut (*self_0).prev_frame_image as *mut *mut guint16
        as *mut libc::c_char;
    _p_1 = *_pp_1.out;
    if !_p_1.is_null() {
        *_pp_1.out = 0 as *mut libc::c_void;
        _destroy_1.expect("non-null function pointer")(_p_1);
    }
    g_slist_free_full(
        (if 0 as libc::c_int != 0 {
            (*self_0).fp_frame_list as *mut libc::c_void
        } else {
            g_steal_pointer(&mut (*self_0).fp_frame_list as *mut *mut GSList as gpointer)
        }) as *mut GSList,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    ((*(g_type_check_class_cast(
        fpi_device_elanspi_parent_class as *mut GTypeClass,
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
    ) as *mut libc::c_void as *mut GObjectClass))
        .finalize)
        .expect("non-null function pointer")(this);
}
unsafe extern "C" fn fpi_device_elanspi_class_init(
    mut klass: *mut FpiDeviceElanSpiClass,
) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    let mut img_class: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_CLASS(
        klass as gpointer,
    );
    (*dev_class).id = b"elanspi\0" as *const u8 as *const libc::c_char;
    (*dev_class)
        .full_name = b"ElanTech Embedded Fingerprint Sensor\0" as *const u8
        as *const libc::c_char;
    (*dev_class).type_0 = FP_DEVICE_TYPE_UDEV;
    (*dev_class).id_table = elanspi_id_table.as_ptr();
    (*dev_class).scan_type = FP_SCAN_TYPE_SWIPE;
    (*dev_class).nr_enroll_stages = 7 as libc::c_int;
    (*img_class).bz3_threshold = 24 as libc::c_int;
    (*img_class)
        .img_open = Some(elanspi_open as unsafe extern "C" fn(*mut FpImageDevice) -> ());
    (*img_class)
        .activate = Some(
        elanspi_activate as unsafe extern "C" fn(*mut FpImageDevice) -> (),
    );
    (*img_class)
        .deactivate = Some(
        elanspi_deactivate as unsafe extern "C" fn(*mut FpImageDevice) -> (),
    );
    (*img_class)
        .change_state = Some(
        elanspi_change_state
            as unsafe extern "C" fn(*mut FpImageDevice, FpiImageDeviceState) -> (),
    );
    (*img_class)
        .img_close = Some(
        elanspi_close as unsafe extern "C" fn(*mut FpImageDevice) -> (),
    );
    let ref mut fresh2 = (*(g_type_check_class_cast(
        klass as *mut GTypeClass,
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
    ) as *mut libc::c_void as *mut GObjectClass))
        .finalize;
    *fresh2 = Some(
        fpi_device_elanspi_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    );
}
