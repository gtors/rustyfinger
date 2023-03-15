use ::libc;
extern "C" {
    pub type _GData;
    pub type _GCancellablePrivate;
    pub type _FpiSsm;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_clear_error(err: *mut *mut GError);
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_slist_free(list: *mut GSList);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn g_strcmp0(str1: *const libc::c_char, str2: *const libc::c_char) -> libc::c_int;
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
    fn g_io_error_quark() -> GQuark;
    fn fpi_assemble_lines(
        ctx: *mut fpi_line_asmbl_ctx,
        lines: *mut GSList,
        num_lines: size_t,
    ) -> *mut FpImage;
    fn g_usb_device_error_quark() -> GQuark;
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
    fn fpi_device_get_usb_device(device: *mut FpDevice) -> *mut GUsbDevice;
    fn fpi_device_error_new(error: FpDeviceError) -> *mut GError;
    fn fpi_device_get_cancellable(device: *mut FpDevice) -> *mut GCancellable;
    fn fp_image_device_get_type() -> GType;
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
    fn fpi_usb_transfer_new(device: *mut FpDevice) -> *mut FpiUsbTransfer;
    fn fpi_usb_transfer_fill_bulk(
        transfer: *mut FpiUsbTransfer,
        endpoint: guint8,
        length: gsize,
    );
    fn fpi_usb_transfer_fill_bulk_full(
        transfer: *mut FpiUsbTransfer,
        endpoint: guint8,
        buffer: *mut guint8,
        length: gsize,
        free_func: GDestroyNotify,
    );
    fn fpi_usb_transfer_fill_interrupt(
        transfer: *mut FpiUsbTransfer,
        endpoint: guint8,
        length: gsize,
    );
    fn fpi_usb_transfer_fill_interrupt_full(
        transfer: *mut FpiUsbTransfer,
        endpoint: guint8,
        buffer: *mut guint8,
        length: gsize,
        free_func: GDestroyNotify,
    );
    fn fpi_usb_transfer_submit(
        transfer: *mut FpiUsbTransfer,
        timeout_ms: guint,
        cancellable: *mut GCancellable,
        callback: FpiUsbTransferCallback,
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
    fn fpi_ssm_mark_completed(machine: *mut FpiSsm);
    fn fpi_ssm_mark_failed(machine: *mut FpiSsm, error: *mut GError);
    fn fpi_ssm_get_cur_state(machine: *mut FpiSsm) -> libc::c_int;
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
pub struct fpi_line_asmbl_ctx {
    pub line_width: libc::c_uint,
    pub max_height: libc::c_uint,
    pub resolution: libc::c_uint,
    pub median_filter_size: libc::c_uint,
    pub max_search_offset: libc::c_uint,
    pub get_deviation: Option::<
        unsafe extern "C" fn(
            *mut fpi_line_asmbl_ctx,
            *mut GSList,
            *mut GSList,
        ) -> libc::c_int,
    >,
    pub get_pixel: Option::<
        unsafe extern "C" fn(
            *mut fpi_line_asmbl_ctx,
            *mut GSList,
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const G_USB_DEVICE_ERROR_LAST: C2RustUnnamed_1 = 10;
pub const G_USB_DEVICE_ERROR_PERMISSION_DENIED: C2RustUnnamed_1 = 9;
pub const G_USB_DEVICE_ERROR_FAILED: C2RustUnnamed_1 = 8;
pub const G_USB_DEVICE_ERROR_CANCELLED: C2RustUnnamed_1 = 7;
pub const G_USB_DEVICE_ERROR_ALREADY_OPEN: C2RustUnnamed_1 = 6;
pub const G_USB_DEVICE_ERROR_NOT_OPEN: C2RustUnnamed_1 = 5;
pub const G_USB_DEVICE_ERROR_NO_DEVICE: C2RustUnnamed_1 = 4;
pub const G_USB_DEVICE_ERROR_NOT_SUPPORTED: C2RustUnnamed_1 = 3;
pub const G_USB_DEVICE_ERROR_TIMED_OUT: C2RustUnnamed_1 = 2;
pub const G_USB_DEVICE_ERROR_IO: C2RustUnnamed_1 = 1;
pub const G_USB_DEVICE_ERROR_INTERNAL: C2RustUnnamed_1 = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub driver_data: guint64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub virtual_envvar: *const gchar,
    pub c2rust_unnamed_0: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub udev_types: FpiDeviceUdevSubtypeFlags,
    pub spi_acpi_id: *const gchar,
    pub hid_id: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub pid: guint,
    pub vid: guint,
}
pub type FpiDeviceUdevSubtypeFlags = libc::c_uint;
pub const FPI_DEVICE_UDEV_SUBTYPE_HIDRAW: FpiDeviceUdevSubtypeFlags = 2;
pub const FPI_DEVICE_UDEV_SUBTYPE_SPIDEV: FpiDeviceUdevSubtypeFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
#[repr(C, packed)]
pub struct vfs_line {
    pub _0x01: libc::c_uchar,
    pub _0xfe: libc::c_uchar,
    pub id: libc::c_ushort,
    pub noise_hash_1: libc::c_uchar,
    pub noise_hash_2: libc::c_uchar,
    pub _somedata: libc::c_ushort,
    pub data: [libc::c_uchar; 100],
    pub next_line_part: [libc::c_uchar; 32],
    pub scan_data: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpDeviceVfs0050 {
    pub parent: FpImageDevice,
    pub active: libc::c_char,
    pub control_packet: *mut libc::c_uchar,
    pub ssm_active: libc::c_char,
    pub need_report: libc::c_char,
    pub lines_buffer: *mut vfs_line,
    pub bytes: libc::c_int,
    pub memory: libc::c_int,
    pub usb_buffer: *mut libc::c_char,
    pub interrupt: [libc::c_uchar; 8],
}
pub type FpDeviceVfs0050 = _FpDeviceVfs0050;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpDeviceVfs0050Class {
    pub parent_class: FpImageDeviceClass,
}
pub const SSM_STATES: SSM_STATE = 12;
pub const SSM_TURN_ON: SSM_STATE = 5;
pub const SSM_WAIT_ANOTHER_SCAN: SSM_STATE = 11;
pub const SUBSM2_STATES: SUBSM2 = 7;
pub const SUBSM1_STATES: SUBSM1 = 3;
pub const SUBSM1_ABORT_2: SUBSM1 = 2;
pub const SUBSM1_RETURN_CODE: SUBSM1 = 1;
pub const SUBSM1_COMMAND_04: SUBSM1 = 0;
pub const SUBSM2_CLEAR_EP2: SUBSM2 = 6;
pub const SUBSM2_ABORT_3: SUBSM2 = 5;
pub const SUBSM2_READ_EMPTY_INTERRUPT: SUBSM2 = 4;
pub const SUBSM2_COMMIT_RESPONSE: SUBSM2 = 3;
pub const SUBSM2_SEND_CONTROL: SUBSM2 = 0;
pub const SUBSM2_SEND_COMMIT: SUBSM2 = 2;
pub const SUBSM2_RETURN_CODE: SUBSM2 = 1;
pub const SSM_CLEAR_EP2: SSM_STATE = 3;
pub const SSM_NEXT_RECEIVE: SSM_STATE = 10;
pub const SSM_SUBMIT_IMAGE: SSM_STATE = 9;
pub const SSM_RECEIVE_FINGER: SSM_STATE = 8;
pub const SSM_WAIT_INTERRUPT: SSM_STATE = 7;
pub const SSM_ASK_INTERRUPT: SSM_STATE = 6;
pub const SSM_TURN_OFF: SSM_STATE = 4;
pub const SSM_INITIAL_ABORT_3: SSM_STATE = 2;
pub const SSM_INITIAL_ABORT_2: SSM_STATE = 1;
pub const SSM_INITIAL_ABORT_1: SSM_STATE = 0;
pub type SUBSM1 = libc::c_uint;
pub type SUBSM2 = libc::c_uint;
pub type SSM_STATE = libc::c_uint;
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
#[inline]
unsafe extern "C" fn FPI_DEVICE_VFS0050(mut ptr: gpointer) -> *mut FpDeviceVfs0050 {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_vfs0050_get_type(),
    ) as *mut libc::c_void as *mut FpDeviceVfs0050;
}
static mut turn_on: [libc::c_uchar; 125] = [
    0x39 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0xbf as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
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
    0xf4 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
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
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
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
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
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
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
static mut turn_off: [libc::c_uchar; 125] = [
    0x39 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
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
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
static mut next_receive_1: [libc::c_uchar; 125] = [
    0x39 as libc::c_int as libc::c_uchar,
    0xb8 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xb8 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
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
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
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
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xb8 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
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
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
static mut next_receive_2: [libc::c_uchar; 125] = [
    0x39 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
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
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
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
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
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
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
static mut commit_out: [libc::c_uchar; 985] = [
    0x2 as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x78 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8c as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x98 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xa8 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xb0 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xb8 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xbc as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
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
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0xd3 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0xbb as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xcb as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xcb as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xcb as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xc4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x6 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xfc as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x78 as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x78 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x87 as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x26 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x40 as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
static mut empty_interrupt: [libc::c_uchar; 5] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
static mut interrupt1: [libc::c_uchar; 5] = [
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
];
static mut interrupt2: [libc::c_uchar; 5] = [
    0x2 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
];
static mut interrupt3: [libc::c_uchar; 5] = [
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
];
static mut fpi_device_vfs0050_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut FpDeviceVfs0050_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn fpi_device_vfs0050_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_image_device_get_type(),
        g_intern_static_string(b"FpDeviceVfs0050\0" as *const u8 as *const libc::c_char),
        ::core::mem::size_of::<FpDeviceVfs0050Class>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_vfs0050_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpDeviceVfs0050>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpDeviceVfs0050) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_vfs0050_init
                        as unsafe extern "C" fn(*mut FpDeviceVfs0050) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn fpi_device_vfs0050_class_intern_init(mut klass: gpointer) {
    fpi_device_vfs0050_parent_class = g_type_class_peek_parent(klass);
    if FpDeviceVfs0050_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpDeviceVfs0050_private_offset);
    }
    fpi_device_vfs0050_class_init(klass as *mut FpDeviceVfs0050Class);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_vfs0050_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_vfs0050_get_type_once();
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
unsafe extern "C" fn async_write_callback(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    if !error.is_null() {
        g_log(
            b"libfprint-vfs0050\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"USB write transfer: %s\0" as *const u8 as *const libc::c_char,
            (*error).message,
        );
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    fpi_ssm_next_state((*transfer).ssm);
}
unsafe extern "C" fn async_write(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut data: *mut libc::c_void,
    mut len: libc::c_int,
) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    transfer = fpi_usb_transfer_new(FP_DEVICE(dev as gpointer));
    fpi_usb_transfer_fill_bulk_full(
        transfer,
        0x1 as libc::c_int as guint8,
        data as *mut guint8,
        len as gsize,
        None,
    );
    (*transfer).ssm = ssm;
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_submit(
        transfer,
        100 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            async_write_callback
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
unsafe extern "C" fn async_read_callback(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut ep: libc::c_int = (*transfer).endpoint as libc::c_int;
    if !error.is_null() {
        g_log(
            b"libfprint-vfs0050\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"USB read transfer on endpoint %d: %s\0" as *const u8
                as *const libc::c_char,
            ep - 0x80 as libc::c_int,
            (*error).message,
        );
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    fpi_ssm_next_state((*transfer).ssm);
}
unsafe extern "C" fn async_read(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut ep: libc::c_int,
    mut data: *mut libc::c_void,
    mut len: libc::c_int,
) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    let mut free_func: GDestroyNotify = None;
    ep |= 0x80 as libc::c_int;
    transfer = fpi_usb_transfer_new(FP_DEVICE(dev as gpointer));
    (*transfer).ssm = ssm;
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    if data.is_null() {
        data = g_malloc0(len as gsize);
        free_func = Some(g_free as unsafe extern "C" fn(gpointer) -> ());
    }
    if ep == 0x83 as libc::c_int {
        fpi_usb_transfer_fill_interrupt_full(
            transfer,
            ep as guint8,
            data as *mut guint8,
            len as gsize,
            free_func,
        );
    } else {
        fpi_usb_transfer_fill_bulk_full(
            transfer,
            ep as guint8,
            data as *mut guint8,
            len as gsize,
            free_func,
        );
    }
    fpi_usb_transfer_submit(
        transfer,
        100 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            async_read_callback
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
unsafe extern "C" fn async_abort_callback(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut ep: libc::c_int = (*transfer).endpoint as libc::c_int;
    if g_error_matches(
        error,
        g_usb_device_error_quark(),
        G_USB_DEVICE_ERROR_TIMED_OUT as libc::c_int,
    ) != 0
        || g_strcmp0(
            g_getenv(b"FP_DEVICE_EMULATION\0" as *const u8 as *const libc::c_char),
            b"1\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && (*transfer).actual_length == 0 as libc::c_int as libc::c_long
    {
        g_clear_error(&mut error);
        fpi_ssm_next_state((*transfer).ssm);
        return;
    }
    if !error.is_null() {
        g_log(
            b"libfprint-vfs0050\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"USB write transfer: %s\0" as *const u8 as *const libc::c_char,
            (*error).message,
        );
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    g_log(
        b"libfprint-vfs0050\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_WARNING,
        b"Endpoint %d had extra %zd bytes readable\0" as *const u8
            as *const libc::c_char,
        ep - 0x80 as libc::c_int,
        (*transfer).actual_length,
    );
    fpi_ssm_jump_to_state((*transfer).ssm, fpi_ssm_get_cur_state((*transfer).ssm));
}
unsafe extern "C" fn async_abort(
    mut dev: *mut FpDevice,
    mut ssm: *mut FpiSsm,
    mut ep: libc::c_int,
) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    ep |= 0x80 as libc::c_int;
    transfer = fpi_usb_transfer_new(dev);
    if ep == 0x83 as libc::c_int {
        fpi_usb_transfer_fill_interrupt(
            transfer,
            ep as guint8,
            65536 as libc::c_int as gsize,
        );
    } else {
        fpi_usb_transfer_fill_bulk(
            transfer,
            ep as guint8,
            65536 as libc::c_int as gsize,
        );
    }
    (*transfer).ssm = ssm;
    fpi_usb_transfer_submit(
        transfer,
        20 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            async_abort_callback
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
unsafe extern "C" fn vfs0050_get_pixel(
    mut ctx: *mut fpi_line_asmbl_ctx,
    mut line: *mut GSList,
    mut x: libc::c_uint,
) -> libc::c_uchar {
    return (*((*line).data as *mut vfs_line)).data[x as usize];
}
unsafe extern "C" fn vfs0050_get_difference(
    mut ctx: *mut fpi_line_asmbl_ctx,
    mut line_list_1: *mut GSList,
    mut line_list_2: *mut GSList,
) -> libc::c_int {
    let mut line1: *mut vfs_line = (*line_list_1).data as *mut vfs_line;
    let mut line2: *mut vfs_line = (*line_list_2).data as *mut vfs_line;
    let shift: libc::c_int = (100 as libc::c_int - 32 as libc::c_int) / 2 as libc::c_int
        - 1 as libc::c_int;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let mut x: libc::c_int = (*line1).next_line_part[i as usize] as libc::c_int
            - (*line2).data[(shift + i) as usize] as libc::c_int;
        res += x * x;
        i += 1;
    }
    return res;
}
unsafe extern "C" fn is_noise(mut line: *mut vfs_line) -> libc::c_char {
    let mut val1: libc::c_int = (*line).noise_hash_1 as libc::c_int;
    let mut val2: libc::c_int = (*line).noise_hash_2 as libc::c_int;
    if val1 > 40 as libc::c_int && val1 < 256 as libc::c_int - 40 as libc::c_int
        && val2 > 40 as libc::c_int && val2 < 256 as libc::c_int - 40 as libc::c_int
    {
        return 1 as libc::c_int as libc::c_char;
    }
    return 0 as libc::c_int as libc::c_char;
}
static mut assembling_ctx: fpi_line_asmbl_ctx = unsafe {
    {
        let mut init = fpi_line_asmbl_ctx {
            line_width: 100 as libc::c_int as libc::c_uint,
            max_height: 3000 as libc::c_int as libc::c_uint,
            resolution: 10 as libc::c_int as libc::c_uint,
            median_filter_size: 25 as libc::c_int as libc::c_uint,
            max_search_offset: 100 as libc::c_int as libc::c_uint,
            get_deviation: Some(
                vfs0050_get_difference
                    as unsafe extern "C" fn(
                        *mut fpi_line_asmbl_ctx,
                        *mut GSList,
                        *mut GSList,
                    ) -> libc::c_int,
            ),
            get_pixel: Some(
                vfs0050_get_pixel
                    as unsafe extern "C" fn(
                        *mut fpi_line_asmbl_ctx,
                        *mut GSList,
                        libc::c_uint,
                    ) -> libc::c_uchar,
            ),
        };
        init
    }
};
unsafe extern "C" fn prepare_image(mut vdev: *mut FpDeviceVfs0050) -> *mut FpImage {
    let mut height: libc::c_int = (*vdev).bytes / 148 as libc::c_int;
    while height > 0 as libc::c_int {
        if is_noise(
            ((*vdev).lines_buffer)
                .offset(height as isize)
                .offset(-(1 as libc::c_int as isize)),
        ) == 0
        {
            break;
        }
        height -= 1;
    }
    if height > 3000 as libc::c_int {
        height = 3000 as libc::c_int;
    }
    if height < 100 as libc::c_int {
        return 0 as *mut FpImage;
    }
    let mut lines: *mut GSList = 0 as *mut GSList;
    let mut i: libc::c_int = height - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        lines = g_slist_prepend(
            lines,
            ((*vdev).lines_buffer).offset(i as isize) as gpointer,
        );
        i -= 1;
    }
    let mut img: *mut FpImage = fpi_assemble_lines(
        &mut assembling_ctx,
        lines,
        height as size_t,
    );
    g_slist_free(lines);
    return img;
}
unsafe extern "C" fn submit_image(mut self_0: *mut FpDeviceVfs0050) {
    let mut idev: *mut FpImageDevice = FP_IMAGE_DEVICE(self_0 as gpointer);
    if (*self_0).active == 0 {
        return;
    }
    let mut img: *mut FpImage = prepare_image(self_0);
    if img.is_null() {
        fpi_image_device_retry_scan(idev, FP_DEVICE_RETRY_TOO_SHORT);
    } else {
        fpi_image_device_image_captured(idev, img);
    }
    fpi_image_device_report_finger_status(idev, 0 as libc::c_int);
}
unsafe extern "C" fn clear_ep2_ssm(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut command04: libc::c_char = 0x4 as libc::c_int as libc::c_char;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            async_write(
                ssm,
                dev,
                &mut command04 as *mut libc::c_char as *mut libc::c_void,
                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
            );
        }
        1 => {
            async_read(
                ssm,
                dev,
                1 as libc::c_int,
                0 as *mut libc::c_void,
                2 as libc::c_int,
            );
        }
        2 => {
            async_abort(dev, ssm, 2 as libc::c_int);
        }
        _ => {
            g_log(
                b"libfprint-vfs0050\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"Unknown SUBSM1 state\0" as *const u8 as *const libc::c_char,
            );
            fpi_ssm_mark_failed(ssm, fpi_device_error_new(FP_DEVICE_ERROR_PROTO));
        }
    };
}
unsafe extern "C" fn clear_ep2(mut dev: *mut FpDevice, mut ssm: *mut FpiSsm) {
    let mut subsm: *mut FpiSsm = fpi_ssm_new_full(
        dev,
        Some(clear_ep2_ssm as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
        SUBSM1_STATES as libc::c_int,
        SUBSM1_STATES as libc::c_int,
        b"SUBSM1_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start_subsm(ssm, subsm);
}
unsafe extern "C" fn send_control_packet_ssm(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
) {
    let mut self_0: *mut FpDeviceVfs0050 = FPI_DEVICE_VFS0050(dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            async_write(
                ssm,
                dev,
                (*self_0).control_packet as *mut libc::c_void,
                125 as libc::c_int,
            );
        }
        1 => {
            async_read(
                ssm,
                dev,
                1 as libc::c_int,
                0 as *mut libc::c_void,
                2 as libc::c_int,
            );
        }
        2 => {
            if (*self_0).control_packet == next_receive_1.as_mut_ptr() {
                (*self_0).control_packet = next_receive_2.as_mut_ptr();
                fpi_ssm_jump_to_state(ssm, SUBSM2_SEND_CONTROL as libc::c_int);
            } else {
                async_write(
                    ssm,
                    dev,
                    commit_out.as_mut_ptr() as *mut libc::c_void,
                    ::core::mem::size_of::<[libc::c_uchar; 985]>() as libc::c_ulong
                        as libc::c_int,
                );
            }
        }
        3 => {
            async_read(
                ssm,
                dev,
                1 as libc::c_int,
                0 as *mut libc::c_void,
                1106 as libc::c_int,
            );
        }
        4 => {
            async_read(
                ssm,
                dev,
                3 as libc::c_int,
                ((*self_0).interrupt).as_mut_ptr() as *mut libc::c_void,
                5 as libc::c_int,
            );
        }
        5 => {
            if memcmp(
                ((*self_0).interrupt).as_mut_ptr() as *const libc::c_void,
                empty_interrupt.as_mut_ptr() as *const libc::c_void,
                5 as libc::c_int as libc::c_ulong,
            ) != 0
            {
                g_log(
                    b"libfprint-vfs0050\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_CRITICAL,
                    b"Unknown SUBSM2 state\0" as *const u8 as *const libc::c_char,
                );
                fpi_ssm_mark_failed(ssm, fpi_device_error_new(FP_DEVICE_ERROR_PROTO));
            } else {
                async_abort(dev, ssm, 3 as libc::c_int);
            }
        }
        6 => {
            if (*self_0).control_packet != turn_on.as_mut_ptr() {
                clear_ep2(dev, ssm);
            } else {
                fpi_ssm_next_state(ssm);
            }
        }
        _ => {
            g_log(
                b"libfprint-vfs0050\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"Unknown SUBSM2 state\0" as *const u8 as *const libc::c_char,
            );
            fpi_ssm_mark_failed(ssm, fpi_device_error_new(FP_DEVICE_ERROR_PROTO));
        }
    };
}
unsafe extern "C" fn send_control_packet(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut subsm: *mut FpiSsm = fpi_ssm_new_full(
        dev,
        Some(
            send_control_packet_ssm
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        SUBSM2_STATES as libc::c_int,
        SUBSM2_STATES as libc::c_int,
        b"SUBSM2_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start_subsm(ssm, subsm);
}
unsafe extern "C" fn clear_data(mut vdev: *mut FpDeviceVfs0050) {
    g_free((*vdev).lines_buffer as gpointer);
    (*vdev).lines_buffer = 0 as *mut vfs_line;
    (*vdev).bytes = 0 as libc::c_int;
    (*vdev).memory = (*vdev).bytes;
}
unsafe extern "C" fn interrupt_callback(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpDeviceVfs0050 = FPI_DEVICE_VFS0050(device as gpointer);
    let mut interrupt: *mut libc::c_uchar = (*transfer).buffer;
    if (*self_0).active == 0
        && g_error_matches(
            error,
            g_io_error_quark(),
            G_IO_ERROR_CANCELLED as libc::c_int,
        ) != 0
    {
        g_error_free(error);
        fpi_ssm_jump_to_state((*transfer).ssm, SSM_CLEAR_EP2 as libc::c_int);
        return;
    }
    if !error.is_null() {
        g_log(
            b"libfprint-vfs0050\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"USB read interrupt transfer: %s\0" as *const u8 as *const libc::c_char,
            (*error).message,
        );
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    if memcmp(
        interrupt as *const libc::c_void,
        interrupt1.as_mut_ptr() as *const libc::c_void,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        || memcmp(
            interrupt as *const libc::c_void,
            interrupt2.as_mut_ptr() as *const libc::c_void,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        || memcmp(
            interrupt as *const libc::c_void,
            interrupt3.as_mut_ptr() as *const libc::c_void,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        fpi_ssm_next_state((*transfer).ssm);
        return;
    }
    if *interrupt.offset(0 as libc::c_int as isize) as libc::c_int == 0x1 as libc::c_int
    {
        g_log(
            b"libfprint-vfs0050\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Finger is already on the scanner\0" as *const u8 as *const libc::c_char,
        );
        fpi_ssm_next_state((*transfer).ssm);
        return;
    }
    g_log(
        b"libfprint-vfs0050\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_CRITICAL,
        b"Unknown interrupt '%02x:%02x:%02x:%02x:%02x'!\0" as *const u8
            as *const libc::c_char,
        *interrupt.offset(0 as libc::c_int as isize) as libc::c_int
            & 0xff as libc::c_int,
        *interrupt.offset(1 as libc::c_int as isize) as libc::c_int
            & 0xff as libc::c_int,
        *interrupt.offset(2 as libc::c_int as isize) as libc::c_int
            & 0xff as libc::c_int,
        *interrupt.offset(3 as libc::c_int as isize) as libc::c_int
            & 0xff as libc::c_int,
        *interrupt.offset(4 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int,
    );
    fpi_ssm_mark_failed((*transfer).ssm, fpi_device_error_new(FP_DEVICE_ERROR_PROTO));
}
unsafe extern "C" fn receive_callback(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpDeviceVfs0050 = FPI_DEVICE_VFS0050(device as gpointer);
    if !error.is_null()
        && g_error_matches(
            error,
            g_usb_device_error_quark(),
            G_USB_DEVICE_ERROR_TIMED_OUT as libc::c_int,
        ) == 0
    {
        g_log(
            b"libfprint-vfs0050\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"USB read transfer: %s\0" as *const u8 as *const libc::c_char,
            (*error).message,
        );
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    if !error.is_null() {
        g_error_free(error);
    }
    if (*transfer).actual_length <= 0 as libc::c_int as libc::c_long {
        fpi_ssm_next_state((*transfer).ssm);
    } else {
        (*self_0)
            .bytes = ((*self_0).bytes as libc::c_long + (*transfer).actual_length)
            as libc::c_int;
        fpi_ssm_jump_to_state((*transfer).ssm, fpi_ssm_get_cur_state((*transfer).ssm));
    };
}
unsafe extern "C" fn activate_ssm(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut idev: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    let mut self_0: *mut FpDeviceVfs0050 = FPI_DEVICE_VFS0050(dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            async_abort(dev, ssm, 1 as libc::c_int);
        }
        1 => {
            async_abort(dev, ssm, 2 as libc::c_int);
        }
        2 => {
            async_abort(dev, ssm, 3 as libc::c_int);
        }
        3 => {
            clear_ep2(dev, ssm);
        }
        4 => {
            (*self_0).control_packet = turn_off.as_mut_ptr();
            send_control_packet(ssm, dev);
        }
        5 => {
            if (*self_0).active == 0 {
                fpi_ssm_mark_completed(ssm);
                if (*self_0).need_report != 0 {
                    fpi_image_device_deactivate_complete(idev, 0 as *mut GError);
                    (*self_0).need_report = 0 as libc::c_int as libc::c_char;
                }
            } else {
                (*self_0).control_packet = turn_on.as_mut_ptr();
                send_control_packet(ssm, dev);
            }
        }
        6 => {
            let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
            if (*self_0).need_report != 0 {
                fpi_image_device_activate_complete(idev, 0 as *mut GError);
                (*self_0).need_report = 0 as libc::c_int as libc::c_char;
            }
            transfer = fpi_usb_transfer_new(dev);
            (*transfer).ssm = ssm;
            (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
            fpi_usb_transfer_fill_interrupt(
                transfer,
                0x83 as libc::c_int as guint8,
                5 as libc::c_int as gsize,
            );
            fpi_usb_transfer_submit(
                transfer,
                0 as libc::c_int as guint,
                fpi_device_get_cancellable(dev),
                Some(
                    interrupt_callback
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
            clear_data(self_0);
            fpi_ssm_next_state(ssm);
        }
        7 => {}
        8 => {
            let mut transfer_0: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
            if (*self_0).memory == 0 as libc::c_int {
                g_free((*self_0).lines_buffer as gpointer);
                (*self_0).memory = 65536 as libc::c_int;
                (*self_0)
                    .lines_buffer = g_malloc0((*self_0).memory as gsize)
                    as *mut vfs_line;
                (*self_0).bytes = 0 as libc::c_int;
                fpi_image_device_report_finger_status(
                    idev,
                    (0 as libc::c_int == 0) as libc::c_int,
                );
            }
            while (*self_0).memory < (*self_0).bytes + 65536 as libc::c_int {
                let mut pre_memory: libc::c_int = (*self_0).memory;
                (*self_0).memory += 65536 as libc::c_int;
                (*self_0)
                    .lines_buffer = g_realloc(
                    (*self_0).lines_buffer as gpointer,
                    (*self_0).memory as gsize,
                ) as *mut vfs_line;
                memset(
                    ((*self_0).lines_buffer as *mut guint8).offset(pre_memory as isize)
                        as *mut libc::c_void,
                    0 as libc::c_int,
                    65536 as libc::c_int as libc::c_ulong,
                );
            }
            transfer_0 = fpi_usb_transfer_new(dev);
            fpi_usb_transfer_fill_bulk_full(
                transfer_0,
                0x82 as libc::c_int as guint8,
                ((*self_0).lines_buffer as *mut guint8).offset((*self_0).bytes as isize),
                65536 as libc::c_int as gsize,
                None,
            );
            (*transfer_0).ssm = ssm;
            fpi_usb_transfer_submit(
                transfer_0,
                100 as libc::c_int as guint,
                0 as *mut GCancellable,
                Some(
                    receive_callback
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
        9 => {
            submit_image(self_0);
            clear_data(self_0);
            fpi_ssm_next_state_delayed(ssm, 100 as libc::c_int);
        }
        10 => {
            if (*self_0).active == 0 {
                fpi_ssm_jump_to_state(ssm, SSM_CLEAR_EP2 as libc::c_int);
            } else {
                (*self_0).control_packet = next_receive_1.as_mut_ptr();
                send_control_packet(ssm, dev);
            }
        }
        11 => {
            fpi_ssm_jump_to_state_delayed(
                ssm,
                SSM_TURN_ON as libc::c_int,
                400 as libc::c_int,
            );
        }
        _ => {
            g_log(
                b"libfprint-vfs0050\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"Unknown state\0" as *const u8 as *const libc::c_char,
            );
            fpi_ssm_mark_failed(ssm, fpi_device_error_new(FP_DEVICE_ERROR_PROTO));
        }
    };
}
unsafe extern "C" fn dev_activate_callback(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpDeviceVfs0050 = FPI_DEVICE_VFS0050(dev as gpointer);
    (*self_0).ssm_active = 0 as libc::c_int as libc::c_char;
    if !error.is_null() {
        g_log(
            b"libfprint-vfs0050\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Unhandled device activation error: %s\0" as *const u8
                as *const libc::c_char,
            (*error).message,
        );
        g_error_free(error);
    }
}
unsafe extern "C" fn dev_activate(mut idev: *mut FpImageDevice) {
    let mut self_0: *mut FpDeviceVfs0050 = FPI_DEVICE_VFS0050(idev as gpointer);
    (*self_0).active = 1 as libc::c_int as libc::c_char;
    (*self_0).need_report = 1 as libc::c_int as libc::c_char;
    (*self_0).ssm_active = 1 as libc::c_int as libc::c_char;
    let mut ssm: *mut FpiSsm = fpi_ssm_new_full(
        FP_DEVICE(idev as gpointer),
        Some(activate_ssm as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
        SSM_STATES as libc::c_int,
        SSM_STATES as libc::c_int,
        b"SSM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            dev_activate_callback
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn dev_deactivate(mut idev: *mut FpImageDevice) {
    let mut self_0: *mut FpDeviceVfs0050 = FPI_DEVICE_VFS0050(idev as gpointer);
    if (*self_0).ssm_active == 0 {
        fpi_image_device_deactivate_complete(idev, 0 as *mut GError);
        return;
    }
    (*self_0).active = 0 as libc::c_int as libc::c_char;
    (*self_0).need_report = 1 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn dev_open_callback(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    fpi_image_device_open_complete(FP_IMAGE_DEVICE(dev as gpointer), error);
}
unsafe extern "C" fn dev_open(mut idev: *mut FpImageDevice) {
    let mut error: *mut GError = 0 as *mut GError;
    if g_usb_device_claim_interface(
        fpi_device_get_usb_device(FP_DEVICE(idev as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    ) == 0
    {
        fpi_image_device_open_complete(idev, error);
        return;
    }
    let mut ssm: *mut FpiSsm = fpi_ssm_new_full(
        FP_DEVICE(idev as gpointer),
        Some(activate_ssm as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
        SSM_STATES as libc::c_int,
        SSM_STATES as libc::c_int,
        b"SSM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            dev_open_callback
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn dev_close(mut idev: *mut FpImageDevice) {
    let mut error: *mut GError = 0 as *mut GError;
    let mut self_0: *mut FpDeviceVfs0050 = FPI_DEVICE_VFS0050(idev as gpointer);
    clear_data(self_0);
    g_usb_device_release_interface(
        fpi_device_get_usb_device(FP_DEVICE(idev as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    );
    fpi_image_device_close_complete(idev, error);
}
static mut id_table: [FpIdEntry; 2] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_2 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_5 {
                        pid: 0x50 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_2 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_5 {
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
unsafe extern "C" fn fpi_device_vfs0050_init(mut self_0: *mut FpDeviceVfs0050) {}
unsafe extern "C" fn fpi_device_vfs0050_class_init(
    mut klass: *mut FpDeviceVfs0050Class,
) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    let mut img_class: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_CLASS(
        klass as gpointer,
    );
    (*dev_class).id = b"vfs0050\0" as *const u8 as *const libc::c_char;
    (*dev_class).full_name = b"Validity VFS0050\0" as *const u8 as *const libc::c_char;
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
    (*img_class).img_width = 100 as libc::c_int;
    (*img_class).img_height = -(1 as libc::c_int);
}
