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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
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
    fn abs(_: libc::c_int) -> libc::c_int;
    fn labs(_: libc::c_long) -> libc::c_long;
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_get_monotonic_time() -> gint64;
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn g_log_structured(log_domain: *const gchar, log_level: GLogLevelFlags, _: ...);
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
    fn fpi_ssm_get_cur_state(machine: *mut FpiSsm) -> libc::c_int;
    fn fpi_ssm_mark_failed(machine: *mut FpiSsm, error: *mut GError);
    fn fpi_ssm_mark_completed(machine: *mut FpiSsm);
    fn fpi_ssm_next_state_delayed(machine: *mut FpiSsm, delay: libc::c_int);
    fn fpi_ssm_jump_to_state(machine: *mut FpiSsm, state: libc::c_int);
    fn fpi_ssm_next_state(machine: *mut FpiSsm);
    fn fpi_ssm_start_subsm(parent: *mut FpiSsm, child: *mut FpiSsm);
    fn fpi_ssm_start(ssm: *mut FpiSsm, callback: FpiSsmCompletedCallback);
    fn fpi_ssm_new_full(
        dev: *mut FpDevice,
        handler: FpiSsmHandlerCallback,
        nr_states: libc::c_int,
        start_cleanup: libc::c_int,
        machine_name: *const libc::c_char,
    ) -> *mut FpiSsm;
    fn fpi_usb_transfer_submit(
        transfer: *mut FpiUsbTransfer,
        timeout_ms: guint,
        cancellable: *mut GCancellable,
        callback: FpiUsbTransferCallback,
        user_data: gpointer,
    );
    fn fpi_usb_transfer_fill_bulk_full(
        transfer: *mut FpiUsbTransfer,
        endpoint: guint8,
        buffer: *mut guint8,
        length: gsize,
        free_func: GDestroyNotify,
    );
    fn fpi_usb_transfer_new(device: *mut FpDevice) -> *mut FpiUsbTransfer;
    fn fpi_image_device_retry_scan(self_0: *mut FpImageDevice, retry: FpDeviceRetry);
    fn fpi_image_device_image_captured(self_0: *mut FpImageDevice, image: *mut FpImage);
    fn fpi_image_device_report_finger_status(
        self_0: *mut FpImageDevice,
        present: gboolean,
    );
    fn fpi_image_device_deactivate_complete(
        self_0: *mut FpImageDevice,
        error: *mut GError,
    );
    fn fpi_image_device_activate_complete(
        self_0: *mut FpImageDevice,
        error: *mut GError,
    );
    fn fpi_image_device_close_complete(self_0: *mut FpImageDevice, error: *mut GError);
    fn fpi_image_device_open_complete(self_0: *mut FpImageDevice, error: *mut GError);
    fn fp_image_device_get_type() -> GType;
    fn fpi_device_error_new(error: FpDeviceError) -> *mut GError;
    fn fpi_device_retry_new(error: FpDeviceRetry) -> *mut GError;
    fn fpi_device_get_usb_device(device: *mut FpDevice) -> *mut GUsbDevice;
    fn g_io_error_quark() -> GQuark;
    fn fp_image_new(width: gint, height: gint) -> *mut FpImage;
    fn g_usb_device_claim_interface(
        self_0: *mut GUsbDevice,
        interface: gint,
        flags: GUsbDeviceClaimInterfaceFlags,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_usb_device_release_interface(
        self_0: *mut GUsbDevice,
        interface: gint,
        flags: GUsbDeviceClaimInterfaceFlags,
        error: *mut *mut GError,
    ) -> gboolean;
    fn fp_device_get_type() -> GType;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUsbDevice {
    pub parent_instance: GObject,
}
pub type GUsbDevice = _GUsbDevice;
pub type GUsbDeviceDirection = libc::c_uint;
pub const G_USB_DEVICE_DIRECTION_HOST_TO_DEVICE: GUsbDeviceDirection = 1;
pub const G_USB_DEVICE_DIRECTION_DEVICE_TO_HOST: GUsbDeviceDirection = 0;
pub type GUsbDeviceRequestType = libc::c_uint;
pub const G_USB_DEVICE_REQUEST_TYPE_RESERVED: GUsbDeviceRequestType = 3;
pub const G_USB_DEVICE_REQUEST_TYPE_VENDOR: GUsbDeviceRequestType = 2;
pub const G_USB_DEVICE_REQUEST_TYPE_CLASS: GUsbDeviceRequestType = 1;
pub const G_USB_DEVICE_REQUEST_TYPE_STANDARD: GUsbDeviceRequestType = 0;
pub type GUsbDeviceRecipient = libc::c_uint;
pub const G_USB_DEVICE_RECIPIENT_OTHER: GUsbDeviceRecipient = 3;
pub const G_USB_DEVICE_RECIPIENT_ENDPOINT: GUsbDeviceRecipient = 2;
pub const G_USB_DEVICE_RECIPIENT_INTERFACE: GUsbDeviceRecipient = 1;
pub const G_USB_DEVICE_RECIPIENT_DEVICE: GUsbDeviceRecipient = 0;
pub type GUsbDeviceClaimInterfaceFlags = libc::c_uint;
pub const G_USB_DEVICE_CLAIM_INTERFACE_BIND_KERNEL_DRIVER: GUsbDeviceClaimInterfaceFlags = 1;
pub const G_USB_DEVICE_CLAIM_INTERFACE_NONE: GUsbDeviceClaimInterfaceFlags = 0;
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
pub struct _FpiUsbTransfer {
    pub device: *mut FpDevice,
    pub ssm: *mut FpiSsm,
    pub length: gssize,
    pub actual_length: gssize,
    pub buffer: *mut guchar,
    pub ref_count: guint,
    pub type_0: FpiTransferType,
    pub endpoint: guint8,
    pub direction: GUsbDeviceDirection,
    pub request_type: GUsbDeviceRequestType,
    pub recipient: GUsbDeviceRecipient,
    pub request: guint8,
    pub value: guint16,
    pub idx: guint16,
    pub short_is_error: gboolean,
    pub user_data: gpointer,
    pub callback: FpiUsbTransferCallback,
    pub free_buffer: GDestroyNotify,
}
pub type FpiUsbTransferCallback = Option::<
    unsafe extern "C" fn(*mut FpiUsbTransfer, *mut FpDevice, gpointer, *mut GError) -> (),
>;
pub type FpiUsbTransfer = _FpiUsbTransfer;
pub type FpiTransferType = libc::c_int;
pub const FP_TRANSFER_INTERRUPT: FpiTransferType = 3;
pub const FP_TRANSFER_BULK: FpiTransferType = 2;
pub const FP_TRANSFER_CONTROL: FpiTransferType = 0;
pub const FP_TRANSFER_NONE: FpiTransferType = -1;
pub type FpiSsm = _FpiSsm;
pub type FpiSsmCompletedCallback = Option::<
    unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
>;
pub type FpiSsmHandlerCallback = Option::<
    unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpDeviceVfs101 {
    pub parent: FpImageDevice,
    pub active: gboolean,
    pub deactivate: gboolean,
    pub seqnum: libc::c_uint,
    pub buffer: *mut libc::c_uchar,
    pub length: libc::c_uint,
    pub ignore_error: libc::c_int,
    pub counter: libc::c_int,
    pub contrast: libc::c_int,
    pub best_contrast: libc::c_int,
    pub best_clevel: libc::c_int,
    pub bottom: libc::c_int,
    pub height: libc::c_int,
}
pub type FpDeviceVfs101 = _FpDeviceVfs101;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpDeviceVfs101Class {
    pub parent_class: FpImageDeviceClass,
}
pub const M_LOOP_NUM_STATES: C2RustUnnamed_10 = 18;
pub const M_LOOP_3_GET_PRINT: C2RustUnnamed_10 = 14;
pub const M_LOOP_3_LOOP: C2RustUnnamed_10 = 17;
pub const M_LOOP_0_GET_PRINT: C2RustUnnamed_10 = 0;
pub const M_LOOP_3_CHECK_IMAGE: C2RustUnnamed_10 = 16;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub const M_LOOP_3_LOAD_IMAGE: C2RustUnnamed_10 = 15;
pub const M_SWAP_NUM_STATES: C2RustUnnamed_8 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub const M_SWAP_RECV: C2RustUnnamed_8 = 1;
pub const M_SWAP_SEND: C2RustUnnamed_8 = 0;
pub const M_LOOP_2_LOAD_IMAGE: C2RustUnnamed_10 = 13;
pub const M_LOOP_2_ABORT_PRINT: C2RustUnnamed_10 = 12;
pub const M_LOOP_1_SLEEP: C2RustUnnamed_10 = 11;
pub const M_LOOP_1_GET_STATE: C2RustUnnamed_10 = 6;
pub const M_LOOP_1_LOOP: C2RustUnnamed_10 = 10;
pub const M_LOOP_1_LOAD_IMAGE: C2RustUnnamed_10 = 9;
pub const M_LOOP_1_GET_PRINT: C2RustUnnamed_10 = 8;
pub const VFS_FINGER_PRESENT: C2RustUnnamed_9 = 1;
pub const VFS_FINGER_UNKNOWN: C2RustUnnamed_9 = 2;
pub const VFS_FINGER_EMPTY: C2RustUnnamed_9 = 0;
pub const M_LOOP_1_CHECK_STATE: C2RustUnnamed_10 = 7;
pub const M_LOOP_0_CHECK_ACTION: C2RustUnnamed_10 = 5;
pub const M_LOOP_0_EXTRACT_IMAGE: C2RustUnnamed_10 = 4;
pub const M_LOOP_0_SLEEP: C2RustUnnamed_10 = 1;
pub const M_LOOP_0_LOAD_IMAGE: C2RustUnnamed_10 = 3;
pub const M_LOOP_0_GET_STATE: C2RustUnnamed_10 = 2;
pub const M_INIT_NUM_STATES: C2RustUnnamed_11 = 30;
pub const M_INIT_5_SET_INFO_RATE: C2RustUnnamed_11 = 29;
pub const M_INIT_5_SET_INFO_CONTRAST: C2RustUnnamed_11 = 28;
pub const M_INIT_5_SET_CONTRAST: C2RustUnnamed_11 = 27;
pub const M_INIT_5_SET_EXPOSURE: C2RustUnnamed_11 = 26;
pub const M_INIT_4_SET_CONTRAST: C2RustUnnamed_11 = 22;
pub const M_INIT_4_CHECK_CONTRAST: C2RustUnnamed_11 = 25;
pub const M_INIT_4_LOAD_IMAGE: C2RustUnnamed_11 = 24;
pub const M_INIT_4_GET_PRINT: C2RustUnnamed_11 = 23;
pub const M_INIT_4_SET_EXPOSURE: C2RustUnnamed_11 = 21;
pub const M_INIT_3_SET_INFO_RATE: C2RustUnnamed_11 = 20;
pub const M_INIT_3_SET_INFO_CONTRAST: C2RustUnnamed_11 = 19;
pub const M_INIT_3_SET_STATE5_COUNT: C2RustUnnamed_11 = 18;
pub const M_INIT_3_SET_STATE3_COUNT: C2RustUnnamed_11 = 17;
pub const M_INIT_3_SET_THRESHOLD: C2RustUnnamed_11 = 16;
pub const M_INIT_3_SET_0078: C2RustUnnamed_11 = 15;
pub const M_INIT_3_SET_0076: C2RustUnnamed_11 = 14;
pub const M_INIT_3_SET_0011: C2RustUnnamed_11 = 13;
pub const M_INIT_3_SET_000E: C2RustUnnamed_11 = 12;
pub const M_INIT_2_GET_STATE: C2RustUnnamed_11 = 7;
pub const M_INIT_2_LOOP: C2RustUnnamed_11 = 11;
pub const M_INIT_2_LOAD_IMAGE: C2RustUnnamed_11 = 10;
pub const M_INIT_2_GET_PRINT: C2RustUnnamed_11 = 9;
pub const M_INIT_0_ABORT_PRINT: C2RustUnnamed_11 = 1;
pub const M_INIT_2_CHECK_STATE: C2RustUnnamed_11 = 8;
pub const M_INIT_1_GET_PRINT: C2RustUnnamed_11 = 3;
pub const M_INIT_1_LOOP: C2RustUnnamed_11 = 6;
pub const M_INIT_1_CHECK_IMAGE: C2RustUnnamed_11 = 5;
pub const M_INIT_1_LOAD_IMAGE: C2RustUnnamed_11 = 4;
pub const M_INIT_0_LOAD_IMAGE: C2RustUnnamed_11 = 2;
pub const M_INIT_0_RECV_DIRTY: C2RustUnnamed_11 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub type C2RustUnnamed_8 = libc::c_uint;
pub type C2RustUnnamed_9 = libc::c_uint;
pub type C2RustUnnamed_10 = libc::c_uint;
pub type C2RustUnnamed_11 = libc::c_uint;
#[inline]
unsafe extern "C" fn FP_IMAGE_DEVICE_CLASS(
    mut ptr: gpointer,
) -> *mut FpImageDeviceClass {
    return g_type_check_class_cast(ptr as *mut GTypeClass, fp_image_device_get_type())
        as *mut libc::c_void as *mut FpImageDeviceClass;
}
#[inline]
unsafe extern "C" fn FP_IMAGE_DEVICE(mut ptr: gpointer) -> *mut FpImageDevice {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fp_image_device_get_type(),
    ) as *mut libc::c_void as *mut FpImageDevice;
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
unsafe extern "C" fn FPI_DEVICE_VFS101(mut ptr: gpointer) -> *mut FpDeviceVfs101 {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_vfs101_get_type(),
    ) as *mut libc::c_void as *mut FpDeviceVfs101;
}
unsafe extern "C" fn fpi_device_vfs101_class_intern_init(mut klass: gpointer) {
    fpi_device_vfs101_parent_class = g_type_class_peek_parent(klass);
    if FpDeviceVfs101_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpDeviceVfs101_private_offset);
    }
    fpi_device_vfs101_class_init(klass as *mut FpDeviceVfs101Class);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_vfs101_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_vfs101_get_type_once();
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
unsafe extern "C" fn fpi_device_vfs101_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_image_device_get_type(),
        g_intern_static_string(b"FpDeviceVfs101\0" as *const u8 as *const libc::c_char),
        ::core::mem::size_of::<FpDeviceVfs101Class>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_vfs101_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpDeviceVfs101>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpDeviceVfs101) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_vfs101_init
                        as unsafe extern "C" fn(*mut FpDeviceVfs101) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut fpi_device_vfs101_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut FpDeviceVfs101_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn byte(
    mut position: libc::c_int,
    mut value: libc::c_int,
) -> libc::c_uchar {
    return (value >> position * 8 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn get_seqnum(
    mut h: libc::c_int,
    mut l: libc::c_int,
) -> libc::c_ushort {
    return (h << 8 as libc::c_int | l) as libc::c_ushort;
}
#[inline]
unsafe extern "C" fn check_seqnum(mut vdev: *mut FpDeviceVfs101) -> libc::c_int {
    if byte(0 as libc::c_int, (*vdev).seqnum as libc::c_int) as libc::c_int
        == *((*vdev).buffer).offset(0 as libc::c_int as isize) as libc::c_int
        && byte(1 as libc::c_int, (*vdev).seqnum as libc::c_int) as libc::c_int
            == *((*vdev).buffer).offset(1 as libc::c_int as isize) as libc::c_int
    {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn async_send_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(device as gpointer);
    let mut self_0: *mut FpDeviceVfs101 = FPI_DEVICE_VFS101(dev as gpointer);
    if !error.is_null() {
        if (*self_0).ignore_error == 0 {
            fpi_ssm_mark_failed((*transfer).ssm, error);
            return;
        } else {
            g_error_free(error);
            g_log(
                b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Ignoring send error: %s\0" as *const u8 as *const libc::c_char,
                (*error).message,
            );
        }
    }
    (*self_0).ignore_error = 0 as libc::c_int;
    g_log(
        b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%02x %02x %02x %02x %02x %02x %02x %02x\0" as *const u8 as *const libc::c_char,
        *((*self_0).buffer).offset(6 as libc::c_int as isize) as libc::c_int,
        *((*self_0).buffer).offset(7 as libc::c_int as isize) as libc::c_int,
        *((*self_0).buffer).offset(8 as libc::c_int as isize) as libc::c_int,
        *((*self_0).buffer).offset(9 as libc::c_int as isize) as libc::c_int,
        *((*self_0).buffer).offset(10 as libc::c_int as isize) as libc::c_int,
        *((*self_0).buffer).offset(11 as libc::c_int as isize) as libc::c_int,
        *((*self_0).buffer).offset(12 as libc::c_int as isize) as libc::c_int,
        *((*self_0).buffer).offset(13 as libc::c_int as isize) as libc::c_int,
    );
    fpi_ssm_next_state((*transfer).ssm);
}
unsafe extern "C" fn async_send(mut ssm: *mut FpiSsm, mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpDeviceVfs101 = FPI_DEVICE_VFS101(dev as gpointer);
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    transfer = fpi_usb_transfer_new(FP_DEVICE(dev as gpointer));
    (*self_0).seqnum = ((*self_0).seqnum).wrapping_add(1);
    *((*self_0).buffer)
        .offset(
            0 as libc::c_int as isize,
        ) = byte(0 as libc::c_int, (*self_0).seqnum as libc::c_int);
    *((*self_0).buffer)
        .offset(
            1 as libc::c_int as isize,
        ) = byte(1 as libc::c_int, (*self_0).seqnum as libc::c_int);
    fpi_usb_transfer_fill_bulk_full(
        transfer,
        (1 as libc::c_int | 0 as libc::c_int) as guint8,
        (*self_0).buffer,
        (*self_0).length as gsize,
        None,
    );
    (*transfer).ssm = ssm;
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_submit(
        transfer,
        100 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            async_send_cb
                as unsafe extern "C" fn(
                    *mut FpiUsbTransfer,
                    *mut FpDevice,
                    gpointer,
                    *mut GError,
                ) -> (),
        ),
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn async_recv_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(device as gpointer);
    let mut self_0: *mut FpDeviceVfs101 = FPI_DEVICE_VFS101(dev as gpointer);
    if (*self_0).ignore_error == 0 {
        if !error.is_null() {
            fpi_ssm_mark_failed((*transfer).ssm, error);
            return;
        }
        if check_seqnum(self_0) != 0 {
            g_log(
                b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"seqnum mismatch, got %04x, expected %04x\0" as *const u8
                    as *const libc::c_char,
                get_seqnum(
                    *((*self_0).buffer).offset(1 as libc::c_int as isize) as libc::c_int,
                    *((*self_0).buffer).offset(0 as libc::c_int as isize) as libc::c_int,
                ) as libc::c_int,
                (*self_0).seqnum,
            );
            fpi_ssm_mark_failed(
                (*transfer).ssm,
                fpi_device_error_new(FP_DEVICE_ERROR_PROTO),
            );
            return;
        }
    }
    let mut _pp: C2RustUnnamed_6 = C2RustUnnamed_6 {
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
    (*self_0).ignore_error = 0 as libc::c_int;
    g_log(
        b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%02x %02x %02x %02x %02x %02x %02x %02x\0" as *const u8 as *const libc::c_char,
        *((*self_0).buffer).offset(6 as libc::c_int as isize) as libc::c_int,
        *((*self_0).buffer).offset(7 as libc::c_int as isize) as libc::c_int,
        *((*self_0).buffer).offset(8 as libc::c_int as isize) as libc::c_int,
        *((*self_0).buffer).offset(9 as libc::c_int as isize) as libc::c_int,
        *((*self_0).buffer).offset(10 as libc::c_int as isize) as libc::c_int,
        *((*self_0).buffer).offset(11 as libc::c_int as isize) as libc::c_int,
        *((*self_0).buffer).offset(12 as libc::c_int as isize) as libc::c_int,
        *((*self_0).buffer).offset(13 as libc::c_int as isize) as libc::c_int,
    );
    (*self_0).length = (*transfer).actual_length as libc::c_uint;
    fpi_ssm_next_state((*transfer).ssm);
}
unsafe extern "C" fn async_recv(mut ssm: *mut FpiSsm, mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpDeviceVfs101 = FPI_DEVICE_VFS101(dev as gpointer);
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    transfer = fpi_usb_transfer_new(FP_DEVICE(dev as gpointer));
    fpi_usb_transfer_fill_bulk_full(
        transfer,
        (1 as libc::c_int | 0x80 as libc::c_int) as guint8,
        (*self_0).buffer,
        0xf as libc::c_int as gsize,
        None,
    );
    (*transfer).ssm = ssm;
    fpi_usb_transfer_submit(
        transfer,
        100 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            async_recv_cb
                as unsafe extern "C" fn(
                    *mut FpiUsbTransfer,
                    *mut FpDevice,
                    gpointer,
                    *mut GError,
                ) -> (),
        ),
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn async_load_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(device as gpointer);
    let mut self_0: *mut FpDeviceVfs101 = FPI_DEVICE_VFS101(dev as gpointer);
    if (*self_0).ignore_error == 0 {
        if !error.is_null() {
            fpi_ssm_mark_failed((*transfer).ssm, error);
            return;
        }
        if (*transfer).actual_length % 292 as libc::c_int as libc::c_long != 0 {
            g_log(
                b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"received incomplete frame\0" as *const u8 as *const libc::c_char,
            );
            fpi_ssm_mark_failed(
                (*transfer).ssm,
                fpi_device_error_new(FP_DEVICE_ERROR_PROTO),
            );
            return;
        }
    }
    let mut _pp: C2RustUnnamed_5 = C2RustUnnamed_5 {
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
    (*self_0)
        .length = ((*self_0).length as libc::c_long + (*transfer).actual_length)
        as libc::c_uint;
    if (*transfer).actual_length
        == (16 as libc::c_int * 292 as libc::c_int) as libc::c_long
    {
        if ((5000 as libc::c_int * 292 as libc::c_int) as libc::c_uint)
            .wrapping_sub((*self_0).length)
            < (16 as libc::c_int * 292 as libc::c_int) as libc::c_uint
        {
            g_log(
                b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"buffer full, image too large\0" as *const u8 as *const libc::c_char,
            );
            fpi_ssm_mark_failed(
                (*transfer).ssm,
                fpi_device_error_new(FP_DEVICE_ERROR_PROTO),
            );
            return;
        } else {
            async_load((*transfer).ssm, dev);
        }
    } else {
        (*self_0).ignore_error = 0 as libc::c_int;
        (*self_0)
            .height = ((*self_0).length).wrapping_div(292 as libc::c_int as libc::c_uint)
            as libc::c_int;
        g_log(
            b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"image loaded, height = %d\0" as *const u8 as *const libc::c_char,
            (*self_0).height,
        );
        fpi_ssm_next_state((*transfer).ssm);
    };
}
unsafe extern "C" fn async_load(mut ssm: *mut FpiSsm, mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpDeviceVfs101 = FPI_DEVICE_VFS101(dev as gpointer);
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    let mut buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    transfer = fpi_usb_transfer_new(FP_DEVICE(dev as gpointer));
    buffer = ((*self_0).buffer).offset((*self_0).length as isize);
    fpi_usb_transfer_fill_bulk_full(
        transfer,
        (2 as libc::c_int | 0x80 as libc::c_int) as guint8,
        buffer,
        (16 as libc::c_int * 292 as libc::c_int) as gsize,
        None,
    );
    (*transfer).ssm = ssm;
    fpi_usb_transfer_submit(
        transfer,
        100 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            async_load_cb
                as unsafe extern "C" fn(
                    *mut FpiUsbTransfer,
                    *mut FpDevice,
                    gpointer,
                    *mut GError,
                ) -> (),
        ),
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn m_swap_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            async_send(ssm, FP_IMAGE_DEVICE(dev as gpointer));
        }
        1 => {
            async_recv(ssm, FP_IMAGE_DEVICE(dev as gpointer));
        }
        _ => {}
    };
}
unsafe extern "C" fn m_swap(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
    mut data: *mut libc::c_uchar,
    mut length: size_t,
) {
    let mut self_0: *mut FpDeviceVfs101 = FPI_DEVICE_VFS101(dev as gpointer);
    let mut subsm: *mut FpiSsm = 0 as *mut FpiSsm;
    memcpy((*self_0).buffer as *mut libc::c_void, data as *const libc::c_void, length);
    memset(
        ((*self_0).buffer).offset(length as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (16 as libc::c_int as libc::c_ulong).wrapping_sub(length),
    );
    (*self_0).length = length as libc::c_uint;
    subsm = fpi_ssm_new_full(
        FP_DEVICE(dev as gpointer),
        Some(m_swap_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
        M_SWAP_NUM_STATES as libc::c_int,
        M_SWAP_NUM_STATES as libc::c_int,
        b"M_SWAP_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start_subsm(ssm, subsm);
}
unsafe extern "C" fn vfs_get_print(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
    mut param: libc::c_uint,
    mut type_0: libc::c_int,
) {
    let mut data: [[libc::c_uchar; 14]; 2] = [
        [
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0x3 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0x1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0x1 as libc::c_int as libc::c_uchar,
        ],
        [
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0x3 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0x1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0x1 as libc::c_int as libc::c_uchar,
            0x1 as libc::c_int as libc::c_uchar,
        ],
    ];
    g_log(
        b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"param = %04x, type = %d\0" as *const u8 as *const libc::c_char,
        param,
        type_0,
    );
    data[type_0
        as usize][6 as libc::c_int
        as usize] = byte(0 as libc::c_int, param as libc::c_int);
    data[type_0
        as usize][7 as libc::c_int
        as usize] = byte(1 as libc::c_int, param as libc::c_int);
    m_swap(ssm, dev, (data[type_0 as usize]).as_mut_ptr(), 0xe as libc::c_int as size_t);
}
unsafe extern "C" fn vfs_set_param(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
    mut param: libc::c_uint,
    mut value: libc::c_uint,
) {
    let mut data: [libc::c_uchar; 10] = [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x5 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];
    g_log(
        b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"param = %04x, value = %04x\0" as *const u8 as *const libc::c_char,
        param,
        value,
    );
    data[6 as libc::c_int as usize] = byte(0 as libc::c_int, param as libc::c_int);
    data[7 as libc::c_int as usize] = byte(1 as libc::c_int, param as libc::c_int);
    data[8 as libc::c_int as usize] = byte(0 as libc::c_int, value as libc::c_int);
    data[9 as libc::c_int as usize] = byte(1 as libc::c_int, value as libc::c_int);
    m_swap(ssm, dev, data.as_mut_ptr(), 0xa as libc::c_int as size_t);
}
unsafe extern "C" fn vfs_abort_print(mut ssm: *mut FpiSsm, mut dev: *mut FpImageDevice) {
    let mut data: [libc::c_uchar; 6] = [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xe as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];
    g_log_structured(
        b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/vfs101.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"469\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"vfs_abort_print\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/vfs101.c:469\0" as *const u8 as *const libc::c_char,
    );
    m_swap(ssm, dev, data.as_mut_ptr(), 0x6 as libc::c_int as size_t);
}
unsafe extern "C" fn vfs_poke(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
    mut addr: libc::c_uint,
    mut value: libc::c_uint,
    mut size: libc::c_uint,
) {
    let mut data: [libc::c_uchar; 15] = [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x13 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];
    g_log(
        b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"addr = %04x, value = %04x\0" as *const u8 as *const libc::c_char,
        addr,
        value,
    );
    data[6 as libc::c_int as usize] = byte(0 as libc::c_int, addr as libc::c_int);
    data[7 as libc::c_int as usize] = byte(1 as libc::c_int, addr as libc::c_int);
    data[8 as libc::c_int as usize] = byte(2 as libc::c_int, addr as libc::c_int);
    data[9 as libc::c_int as usize] = byte(3 as libc::c_int, addr as libc::c_int);
    data[10 as libc::c_int as usize] = byte(0 as libc::c_int, value as libc::c_int);
    data[11 as libc::c_int as usize] = byte(1 as libc::c_int, value as libc::c_int);
    data[12 as libc::c_int as usize] = byte(2 as libc::c_int, value as libc::c_int);
    data[13 as libc::c_int as usize] = byte(3 as libc::c_int, value as libc::c_int);
    data[14 as libc::c_int as usize] = byte(0 as libc::c_int, size as libc::c_int);
    m_swap(ssm, dev, data.as_mut_ptr(), 0xf as libc::c_int as size_t);
}
unsafe extern "C" fn vfs_get_finger_state(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
) {
    let mut data: [libc::c_uchar; 6] = [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x16 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];
    g_log_structured(
        b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/vfs101.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"509\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 21],
            &[libc::c_char; 21],
        >(b"vfs_get_finger_state\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/vfs101.c:509\0" as *const u8 as *const libc::c_char,
    );
    m_swap(ssm, dev, data.as_mut_ptr(), 0x6 as libc::c_int as size_t);
}
unsafe extern "C" fn vfs_img_load(mut ssm: *mut FpiSsm, mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpDeviceVfs101 = FPI_DEVICE_VFS101(dev as gpointer);
    g_log_structured(
        b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/vfs101.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"522\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"vfs_img_load\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/vfs101.c:522\0" as *const u8 as *const libc::c_char,
    );
    (*self_0).length = 0 as libc::c_int as libc::c_uint;
    (*self_0).bottom = 0 as libc::c_int;
    (*self_0).height = -(1 as libc::c_int);
    async_load(ssm, dev);
}
unsafe extern "C" fn img_screen(mut vdev: *mut FpDeviceVfs101) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut top: libc::c_int = 0;
    let mut level: libc::c_long = 0;
    let mut last_line: libc::c_int = (*vdev).height - 1 as libc::c_int;
    g_log(
        b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"image height before screen = %d\0" as *const u8 as *const libc::c_char,
        (*vdev).height,
    );
    count = 0 as libc::c_int;
    y = last_line;
    top = last_line;
    while y >= 0 as libc::c_int {
        level = (*((*vdev).buffer)
            .offset((283 as libc::c_int + y * 292 as libc::c_int) as isize)
            as libc::c_int * 256 as libc::c_int
            + *((*vdev).buffer)
                .offset((282 as libc::c_int + y * 292 as libc::c_int) as isize)
                as libc::c_int) as libc::c_long;
        g_log(
            b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"line = %d, scan level = %ld\0" as *const u8 as *const libc::c_char,
            y,
            level,
        );
        if level >= 768 as libc::c_int as libc::c_long && top == last_line {
            if count < 4 as libc::c_int {
                count += 1;
            } else {
                top = y + 4 as libc::c_int;
                count = 0 as libc::c_int;
            }
        } else if (level < 64 as libc::c_int as libc::c_long
            || level >= 65535 as libc::c_int as libc::c_long) && top != last_line
        {
            if count < 4 as libc::c_int {
                count += 1;
            } else {
                (*vdev).bottom = y + 4 as libc::c_int + 1 as libc::c_int;
                break;
            }
        } else {
            count = 0 as libc::c_int;
        }
        y -= 1;
    }
    (*vdev).height = top - (*vdev).bottom + 1 as libc::c_int;
    if (*vdev).height > 1023 as libc::c_int {
        (*vdev).height = 1023 as libc::c_int;
    }
    g_log(
        b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"image height after screen = %d\0" as *const u8 as *const libc::c_char,
        (*vdev).height,
    );
    y = (*vdev).bottom;
    while y <= top {
        x = 6 as libc::c_int;
        while x < 200 as libc::c_int + 6 as libc::c_int {
            if *((*vdev).buffer).offset((x + y * 292 as libc::c_int) as isize)
                as libc::c_int > 144 as libc::c_int
            {
                *((*vdev).buffer)
                    .offset(
                        (x + y * 292 as libc::c_int) as isize,
                    ) = 255 as libc::c_int as libc::c_uchar;
            }
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C" fn img_copy(mut self_0: *mut FpDeviceVfs101, mut img: *mut FpImage) {
    let mut line: libc::c_uint = 0;
    let mut img_buffer: *mut libc::c_uchar = (*img).data;
    let mut vdev_buffer: *mut libc::c_uchar = ((*self_0).buffer)
        .offset(((*self_0).bottom * 292 as libc::c_int) as isize)
        .offset(6 as libc::c_int as isize);
    line = 0 as libc::c_int as libc::c_uint;
    while line < (*img).height {
        memcpy(
            img_buffer as *mut libc::c_void,
            vdev_buffer as *const libc::c_void,
            200 as libc::c_int as libc::c_ulong,
        );
        vdev_buffer = vdev_buffer.offset(292 as libc::c_int as isize);
        img_buffer = img_buffer.offset(200 as libc::c_int as isize);
        line = line.wrapping_add(1);
    }
}
unsafe extern "C" fn img_extract(mut ssm: *mut FpiSsm, mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpDeviceVfs101 = FPI_DEVICE_VFS101(dev as gpointer);
    let mut img: *mut FpImage = 0 as *mut FpImage;
    img_screen(self_0);
    if (*self_0).height < 200 as libc::c_int {
        (*self_0).height = 0 as libc::c_int;
        fpi_image_device_retry_scan(dev, FP_DEVICE_RETRY_TOO_SHORT);
        return;
    }
    img = fp_image_new((*self_0).height, 200 as libc::c_int);
    (*img).width = 200 as libc::c_int as guint;
    (*img).height = (*self_0).height as guint;
    (*img).flags = FPI_IMAGE_V_FLIPPED;
    img_copy(self_0, img);
    fpi_image_device_image_captured(dev, img);
}
#[inline]
unsafe extern "C" fn vfs_finger_state(mut vdev: *mut FpDeviceVfs101) -> libc::c_int {
    match *((*vdev).buffer).offset(0xa as libc::c_int as isize) as libc::c_int {
        0 | 1 => return VFS_FINGER_EMPTY as libc::c_int,
        2 | 3 | 4 | 5 | 6 => return VFS_FINGER_PRESENT as libc::c_int,
        _ => return VFS_FINGER_UNKNOWN as libc::c_int,
    };
}
unsafe extern "C" fn vfs_check_contrast(mut vdev: *mut FpDeviceVfs101) {
    let mut y: libc::c_int = 0;
    let mut count: libc::c_long = 0 as libc::c_int as libc::c_long;
    y = 0 as libc::c_int;
    while y < (*vdev).height {
        count = count
            + *((*vdev).buffer)
                .offset((5 as libc::c_int + y * 292 as libc::c_int) as isize)
                as libc::c_long
            - *((*vdev).buffer)
                .offset((4 as libc::c_int + y * 292 as libc::c_int) as isize)
                as libc::c_long;
        y += 1;
    }
    count = count / (*vdev).height as libc::c_long;
    if count < 16 as libc::c_int as libc::c_long {
        (*vdev).contrast += 1;
        return;
    }
    g_log(
        b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"contrast = %d, level = %ld\0" as *const u8 as *const libc::c_char,
        (*vdev).contrast,
        count,
    );
    if labs(count - 128 as libc::c_int as libc::c_long)
        < abs((*vdev).best_clevel - 128 as libc::c_int) as libc::c_long
    {
        (*vdev).best_contrast = (*vdev).contrast;
        (*vdev).best_clevel = count as libc::c_int;
    }
}
unsafe extern "C" fn m_loop_state(mut ssm: *mut FpiSsm, mut _dev: *mut FpDevice) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpDeviceVfs101 = FPI_DEVICE_VFS101(_dev as gpointer);
    if (*self_0).deactivate != 0 {
        fpi_ssm_mark_completed(ssm);
        return;
    }
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            vfs_get_print(
                ssm,
                dev,
                5000 as libc::c_int as libc::c_uint,
                1 as libc::c_int,
            );
        }
        1 => {
            fpi_ssm_next_state_delayed(ssm, 50 as libc::c_int);
        }
        2 => {
            vfs_get_finger_state(ssm, dev);
        }
        3 => {
            match vfs_finger_state(self_0) {
                0 => {
                    fpi_image_device_report_finger_status(dev, 0 as libc::c_int);
                    fpi_ssm_jump_to_state(ssm, M_LOOP_0_SLEEP as libc::c_int);
                }
                1 => {
                    fpi_image_device_report_finger_status(
                        dev,
                        (0 as libc::c_int == 0) as libc::c_int,
                    );
                    (*self_0).ignore_error = (0 as libc::c_int == 0) as libc::c_int;
                    vfs_img_load(ssm, dev);
                }
                _ => {
                    fpi_image_device_report_finger_status(dev, 0 as libc::c_int);
                    g_log(
                        b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_CRITICAL,
                        b"unknown device state 0x%02x\0" as *const u8
                            as *const libc::c_char,
                        *((*self_0).buffer).offset(0xa as libc::c_int as isize)
                            as libc::c_int,
                    );
                    fpi_ssm_mark_failed(
                        ssm,
                        fpi_device_error_new(FP_DEVICE_ERROR_PROTO),
                    );
                }
            }
        }
        4 => {
            img_extract(ssm, dev);
            fpi_ssm_next_state_delayed(ssm, 10 as libc::c_int);
        }
        5 => {
            if (*self_0).height > 0 as libc::c_int {
                fpi_ssm_jump_to_state(ssm, M_LOOP_2_ABORT_PRINT as libc::c_int);
            } else {
                fpi_ssm_next_state(ssm);
            }
        }
        6 => {
            vfs_get_finger_state(ssm, dev);
        }
        7 => {
            if vfs_finger_state(self_0) == VFS_FINGER_PRESENT as libc::c_int {
                fpi_image_device_report_finger_status(
                    dev,
                    (0 as libc::c_int == 0) as libc::c_int,
                );
                fpi_ssm_next_state_delayed(ssm, 250 as libc::c_int);
            } else {
                fpi_image_device_report_finger_status(dev, 0 as libc::c_int);
                fpi_ssm_jump_to_state(ssm, M_LOOP_1_SLEEP as libc::c_int);
            }
        }
        8 => {
            vfs_get_print(
                ssm,
                dev,
                5000 as libc::c_int as libc::c_uint,
                1 as libc::c_int,
            );
        }
        9 => {
            (*self_0).ignore_error = (0 as libc::c_int == 0) as libc::c_int;
            vfs_img_load(ssm, dev);
        }
        10 => {
            fpi_ssm_jump_to_state(ssm, M_LOOP_1_GET_STATE as libc::c_int);
        }
        11 => {
            fpi_ssm_next_state_delayed(ssm, 10 as libc::c_int);
        }
        12 => {
            vfs_abort_print(ssm, dev);
        }
        13 => {
            (*self_0).ignore_error = (0 as libc::c_int == 0) as libc::c_int;
            vfs_img_load(ssm, dev);
        }
        14 => {
            vfs_get_print(
                ssm,
                dev,
                0xa as libc::c_int as libc::c_uint,
                0 as libc::c_int,
            );
        }
        15 => {
            (*self_0).ignore_error = (0 as libc::c_int == 0) as libc::c_int;
            vfs_img_load(ssm, dev);
        }
        16 => {
            if (*self_0).height == 10 as libc::c_int {
                (*self_0).counter = 0 as libc::c_int;
                fpi_ssm_jump_to_state(ssm, M_LOOP_0_GET_PRINT as libc::c_int);
            } else if (*self_0).counter < 10 as libc::c_int {
                (*self_0).counter += 1;
                fpi_ssm_next_state_delayed(ssm, 100 as libc::c_int);
            } else {
                g_log(
                    b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_CRITICAL,
                    b"waiting abort reach max loop counter\0" as *const u8
                        as *const libc::c_char,
                );
                fpi_ssm_mark_failed(ssm, fpi_device_error_new(FP_DEVICE_ERROR_PROTO));
            }
        }
        17 => {
            fpi_ssm_jump_to_state(ssm, M_LOOP_3_GET_PRINT as libc::c_int);
        }
        _ => {}
    };
}
unsafe extern "C" fn m_loop_complete(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpDeviceVfs101 = FPI_DEVICE_VFS101(dev as gpointer);
    if (*self_0).active != 0 {
        fpi_image_device_deactivate_complete(FP_IMAGE_DEVICE(dev as gpointer), error);
    }
    (*self_0).active = 0 as libc::c_int;
}
unsafe extern "C" fn m_init_state(mut ssm: *mut FpiSsm, mut _dev: *mut FpDevice) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpDeviceVfs101 = FPI_DEVICE_VFS101(_dev as gpointer);
    if (*self_0).deactivate != 0 {
        fpi_ssm_mark_failed(
            ssm,
            g_error_new(
                g_io_error_quark(),
                G_IO_ERROR_CANCELLED as libc::c_int,
                b"Initialisation was cancelled\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            (*self_0).ignore_error = (0 as libc::c_int == 0) as libc::c_int;
            async_recv(ssm, dev);
        }
        1 => {
            vfs_abort_print(ssm, dev);
        }
        2 => {
            (*self_0).ignore_error = (0 as libc::c_int == 0) as libc::c_int;
            vfs_img_load(ssm, dev);
        }
        3 => {
            vfs_get_print(
                ssm,
                dev,
                0xa as libc::c_int as libc::c_uint,
                0 as libc::c_int,
            );
        }
        4 => {
            (*self_0).ignore_error = (0 as libc::c_int == 0) as libc::c_int;
            vfs_img_load(ssm, dev);
        }
        5 => {
            if (*self_0).height == 10 as libc::c_int {
                (*self_0).counter = 0 as libc::c_int;
                fpi_ssm_jump_to_state(ssm, M_INIT_2_GET_STATE as libc::c_int);
            } else if (*self_0).counter < 10 as libc::c_int {
                (*self_0).counter += 1;
                fpi_ssm_next_state_delayed(ssm, 100 as libc::c_int);
            } else {
                g_log(
                    b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_CRITICAL,
                    b"waiting abort reach max loop counter\0" as *const u8
                        as *const libc::c_char,
                );
                fpi_ssm_mark_failed(ssm, fpi_device_error_new(FP_DEVICE_ERROR_PROTO));
            }
        }
        6 => {
            fpi_ssm_jump_to_state(ssm, M_INIT_1_GET_PRINT as libc::c_int);
        }
        7 => {
            vfs_get_finger_state(ssm, dev);
        }
        8 => {
            if vfs_finger_state(self_0) == VFS_FINGER_PRESENT as libc::c_int {
                if (*self_0).counter < 2 as libc::c_int {
                    (*self_0).counter += 1;
                    fpi_ssm_next_state_delayed(ssm, 250 as libc::c_int);
                } else {
                    g_log(
                        b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_WARNING,
                        b"unexpected finger find, remove finger from the scanner\0"
                            as *const u8 as *const libc::c_char,
                    );
                    fpi_ssm_mark_failed(
                        ssm,
                        fpi_device_retry_new(FP_DEVICE_RETRY_REMOVE_FINGER),
                    );
                }
            } else if (*self_0).counter == 0 as libc::c_int {
                fpi_ssm_jump_to_state(ssm, M_INIT_3_SET_000E as libc::c_int);
            } else {
                (*self_0).counter = 0 as libc::c_int;
                fpi_ssm_jump_to_state(ssm, M_INIT_0_ABORT_PRINT as libc::c_int);
            }
        }
        9 => {
            vfs_get_print(
                ssm,
                dev,
                5000 as libc::c_int as libc::c_uint,
                1 as libc::c_int,
            );
        }
        10 => {
            (*self_0).ignore_error = (0 as libc::c_int == 0) as libc::c_int;
            vfs_img_load(ssm, dev);
        }
        11 => {
            fpi_ssm_jump_to_state(ssm, M_INIT_2_GET_STATE as libc::c_int);
        }
        12 => {
            vfs_set_param(
                ssm,
                dev,
                0xe as libc::c_int as libc::c_uint,
                0x1 as libc::c_int as libc::c_uint,
            );
        }
        13 => {
            vfs_set_param(
                ssm,
                dev,
                0x11 as libc::c_int as libc::c_uint,
                0x8 as libc::c_int as libc::c_uint,
            );
        }
        14 => {
            vfs_set_param(
                ssm,
                dev,
                0x76 as libc::c_int as libc::c_uint,
                0x12 as libc::c_int as libc::c_uint,
            );
        }
        15 => {
            vfs_set_param(
                ssm,
                dev,
                0x78 as libc::c_int as libc::c_uint,
                0x2230 as libc::c_int as libc::c_uint,
            );
        }
        16 => {
            vfs_set_param(
                ssm,
                dev,
                0x57 as libc::c_int as libc::c_uint,
                0x96 as libc::c_int as libc::c_uint,
            );
        }
        17 => {
            vfs_set_param(
                ssm,
                dev,
                0x5e as libc::c_int as libc::c_uint,
                0x64 as libc::c_int as libc::c_uint,
            );
        }
        18 => {
            vfs_set_param(
                ssm,
                dev,
                0x5f as libc::c_int as libc::c_uint,
                0xc8 as libc::c_int as libc::c_uint,
            );
        }
        19 => {
            vfs_set_param(
                ssm,
                dev,
                0x77 as libc::c_int as libc::c_uint,
                10 as libc::c_int as libc::c_uint,
            );
        }
        20 => {
            vfs_set_param(
                ssm,
                dev,
                0x62 as libc::c_int as libc::c_uint,
                32 as libc::c_int as libc::c_uint,
            );
        }
        21 => {
            vfs_poke(
                ssm,
                dev,
                0xff500e as libc::c_int as libc::c_uint,
                0x4000 as libc::c_int as libc::c_uint,
                0x2 as libc::c_int as libc::c_uint,
            );
            (*self_0).counter = 1 as libc::c_int;
        }
        22 => {
            vfs_poke(
                ssm,
                dev,
                0xff5038 as libc::c_int as libc::c_uint,
                (*self_0).contrast as libc::c_uint,
                0x1 as libc::c_int as libc::c_uint,
            );
        }
        23 => {
            vfs_get_print(
                ssm,
                dev,
                0xa as libc::c_int as libc::c_uint,
                0 as libc::c_int,
            );
        }
        24 => {
            vfs_img_load(ssm, dev);
        }
        25 => {
            vfs_check_contrast(self_0);
            if (*self_0).contrast <= 6 as libc::c_int
                || (*self_0).counter >= 12 as libc::c_int
            {
                (*self_0).contrast = (*self_0).best_contrast;
                (*self_0).counter = 0 as libc::c_int;
                g_log(
                    b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"use contrast value = %d\0" as *const u8 as *const libc::c_char,
                    (*self_0).contrast,
                );
                fpi_ssm_next_state(ssm);
            } else {
                (*self_0).contrast -= 1;
                (*self_0).counter += 1;
                fpi_ssm_jump_to_state(ssm, M_INIT_4_SET_CONTRAST as libc::c_int);
            }
        }
        26 => {
            vfs_poke(
                ssm,
                dev,
                0xff500e as libc::c_int as libc::c_uint,
                0x21c0 as libc::c_int as libc::c_uint,
                0x2 as libc::c_int as libc::c_uint,
            );
        }
        27 => {
            vfs_poke(
                ssm,
                dev,
                0xff5038 as libc::c_int as libc::c_uint,
                (*self_0).contrast as libc::c_uint,
                0x1 as libc::c_int as libc::c_uint,
            );
        }
        28 => {
            vfs_set_param(
                ssm,
                dev,
                0x77 as libc::c_int as libc::c_uint,
                (*self_0).contrast as libc::c_uint,
            );
        }
        29 => {
            vfs_set_param(
                ssm,
                dev,
                0x62 as libc::c_int as libc::c_uint,
                0x1 as libc::c_int as libc::c_uint,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn m_init_complete(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    fpi_image_device_activate_complete(dev, error);
    if error.is_null() {
        let mut ssm_loop: *mut FpiSsm = 0 as *mut FpiSsm;
        ssm_loop = fpi_ssm_new_full(
            FP_DEVICE(dev as gpointer),
            Some(m_loop_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
            M_LOOP_NUM_STATES as libc::c_int,
            M_LOOP_NUM_STATES as libc::c_int,
            b"M_LOOP_NUM_STATES\0" as *const u8 as *const libc::c_char,
        );
        fpi_ssm_start(
            ssm_loop,
            Some(
                m_loop_complete
                    as unsafe extern "C" fn(
                        *mut FpiSsm,
                        *mut FpDevice,
                        *mut GError,
                    ) -> (),
            ),
        );
    }
}
unsafe extern "C" fn dev_activate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpDeviceVfs101 = FPI_DEVICE_VFS101(dev as gpointer);
    let mut ssm: *mut FpiSsm = 0 as *mut FpiSsm;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*self_0).active == 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-vfs101\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/vfs101.c\0" as *const u8 as *const libc::c_char,
            1267 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"dev_activate\0"))
                .as_ptr(),
            b"!self->active\0" as *const u8 as *const libc::c_char,
        );
    }
    (*self_0).active = (0 as libc::c_int == 0) as libc::c_int;
    (*self_0).deactivate = 0 as libc::c_int;
    (*self_0).contrast = 15 as libc::c_int;
    (*self_0).best_clevel = -(1 as libc::c_int);
    (*self_0).counter = 0 as libc::c_int;
    ssm = fpi_ssm_new_full(
        FP_DEVICE(dev as gpointer),
        Some(m_init_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
        M_INIT_NUM_STATES as libc::c_int,
        M_INIT_NUM_STATES as libc::c_int,
        b"M_INIT_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            m_init_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn dev_deactivate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpDeviceVfs101 = FPI_DEVICE_VFS101(dev as gpointer);
    if (*self_0).active == 0 {
        fpi_image_device_deactivate_complete(dev, 0 as *mut GError);
        return;
    }
    (*self_0).deactivate = (0 as libc::c_int == 0) as libc::c_int;
}
unsafe extern "C" fn dev_open(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpDeviceVfs101 = FPI_DEVICE_VFS101(dev as gpointer);
    let mut error: *mut GError = 0 as *mut GError;
    g_usb_device_claim_interface(
        fpi_device_get_usb_device(FP_DEVICE(dev as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    );
    (*self_0).seqnum = -(1 as libc::c_int) as libc::c_uint;
    (*self_0)
        .buffer = g_malloc0((5000 as libc::c_int * 292 as libc::c_int) as gsize)
        as *mut libc::c_uchar;
    fpi_image_device_open_complete(dev, error);
}
unsafe extern "C" fn dev_close(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpDeviceVfs101 = FPI_DEVICE_VFS101(dev as gpointer);
    let mut error: *mut GError = 0 as *mut GError;
    g_usb_device_release_interface(
        fpi_device_get_usb_device(FP_DEVICE(dev as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    );
    let mut _pp: C2RustUnnamed_7 = C2RustUnnamed_7 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*self_0).buffer as *mut *mut libc::c_uchar as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    fpi_image_device_close_complete(dev, error);
}
static mut id_table: [FpIdEntry; 2] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0x1 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
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
unsafe extern "C" fn fpi_device_vfs101_init(mut self_0: *mut FpDeviceVfs101) {}
unsafe extern "C" fn fpi_device_vfs101_class_init(mut klass: *mut FpDeviceVfs101Class) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    let mut img_class: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_CLASS(
        klass as gpointer,
    );
    (*dev_class).id = b"vfs101\0" as *const u8 as *const libc::c_char;
    (*dev_class).full_name = b"Validity VFS101\0" as *const u8 as *const libc::c_char;
    (*dev_class).type_0 = FP_DEVICE_TYPE_USB;
    (*dev_class).id_table = id_table.as_ptr();
    (*dev_class).scan_type = FP_SCAN_TYPE_SWIPE;
    (*img_class)
        .img_open = Some(dev_open as unsafe extern "C" fn(*mut FpImageDevice) -> ());
    (*img_class)
        .img_close = Some(dev_close as unsafe extern "C" fn(*mut FpImageDevice) -> ());
    (*img_class)
        .activate = Some(dev_activate as unsafe extern "C" fn(*mut FpImageDevice) -> ());
    (*img_class)
        .deactivate = Some(
        dev_deactivate as unsafe extern "C" fn(*mut FpImageDevice) -> (),
    );
    (*img_class).bz3_threshold = 24 as libc::c_int;
    (*img_class).img_width = 200 as libc::c_int;
    (*img_class).img_height = -(1 as libc::c_int);
}
