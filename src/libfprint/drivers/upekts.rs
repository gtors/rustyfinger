use ::libc;
extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GCancellablePrivate;
    pub type _FpPrint;
    pub type _FpiSsm;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_new_literal(
        domain: GQuark,
        code: gint,
        message: *const gchar,
    ) -> *mut GError;
    fn g_clear_error(err: *mut *mut GError);
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_variant_new_fixed_array(
        element_type: *const GVariantType,
        elements: gconstpointer,
        n_elements: gsize,
        element_size: gsize,
    ) -> *mut GVariant;
    fn g_variant_get_fixed_array(
        value: *mut GVariant,
        n_elements: *mut gsize,
        element_size: gsize,
    ) -> gconstpointer;
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
    fn g_object_set(object: gpointer, first_property_name: *const gchar, _: ...);
    fn g_object_get(object: gpointer, first_property_name: *const gchar, _: ...);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_io_error_quark() -> GQuark;
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
    fn fp_device_retry_quark() -> GQuark;
    fn fpi_print_set_type(print: *mut FpPrint, type_0: FpiPrintType);
    fn fpi_device_class_auto_initialize_features(device_class: *mut FpDeviceClass);
    fn fpi_device_get_usb_device(device: *mut FpDevice) -> *mut GUsbDevice;
    fn fpi_device_action_is_cancelled(device: *mut FpDevice) -> gboolean;
    fn fpi_device_retry_new(error: FpDeviceRetry) -> *mut GError;
    fn fpi_device_error_new(error: FpDeviceError) -> *mut GError;
    fn fpi_device_error_new_msg(
        error: FpDeviceError,
        msg: *const gchar,
        _: ...
    ) -> *mut GError;
    fn fpi_device_get_enroll_data(device: *mut FpDevice, print: *mut *mut FpPrint);
    fn fpi_device_get_verify_data(device: *mut FpDevice, print: *mut *mut FpPrint);
    fn fpi_device_open_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_close_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_enroll_complete(
        device: *mut FpDevice,
        print: *mut FpPrint,
        error: *mut GError,
    );
    fn fpi_device_verify_complete(device: *mut FpDevice, error: *mut GError);
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
    fn fpi_usb_transfer_fill_control(
        transfer: *mut FpiUsbTransfer,
        direction: GUsbDeviceDirection,
        request_type: GUsbDeviceRequestType,
        recipient: GUsbDeviceRecipient,
        request: guint8,
        value: guint16,
        idx: guint16,
        length: gsize,
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
    fn fpi_ssm_mark_failed(machine: *mut FpiSsm, error: *mut GError);
    fn fpi_ssm_set_data(
        machine: *mut FpiSsm,
        ssm_data: gpointer,
        ssm_data_destroy: GDestroyNotify,
    );
    fn fpi_ssm_get_data(machine: *mut FpiSsm) -> gpointer;
    fn fpi_ssm_get_cur_state(machine: *mut FpiSsm) -> libc::c_int;
    fn fpi_ssm_usb_transfer_cb(
        transfer: *mut FpiUsbTransfer,
        device: *mut FpDevice,
        unused_data: gpointer,
        error: *mut GError,
    );
    fn udf_crc(buffer: *mut libc::c_uchar, size: size_t) -> uint16_t;
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
pub type __uint16_t = libc::c_ushort;
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
pub type gconstpointer = *const libc::c_void;
pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
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
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
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
pub type GVariant_autoptr = *mut GVariant;
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
pub type FpPrint = _FpPrint;
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
pub type FpiPrintType = libc::c_uint;
pub const FPI_PRINT_NBIS: FpiPrintType = 2;
pub const FPI_PRINT_RAW: FpiPrintType = 1;
pub const FPI_PRINT_UNDEFINED: FpiPrintType = 0;
pub type FpiMatchResult = libc::c_int;
pub const FPI_MATCH_SUCCESS: FpiMatchResult = 1;
pub const FPI_MATCH_FAIL: FpiMatchResult = 0;
pub const FPI_MATCH_ERROR: FpiMatchResult = -1;
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
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceUpekts {
    pub parent: FpDevice,
    pub enroll_passed: gboolean,
    pub enroll_stage: gint,
    pub first_verify_iteration: gboolean,
    pub seq: guint8,
}
pub type FpiDeviceUpekts = _FpiDeviceUpekts;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiDeviceUpektsClass {
    pub parent_class: FpDeviceClass,
}
pub type read_msg_type = libc::c_uint;
pub const READ_MSG_RESPONSE: read_msg_type = 1;
pub const READ_MSG_CMD: read_msg_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnrollStopData {
    pub print: *mut FpPrint,
    pub error: *mut GError,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub const DEINITSM_NUM_STATES: deinitsm_states = 2;
pub type read_msg_cb_fn = Option::<
    unsafe extern "C" fn(
        *mut FpDevice,
        read_msg_type,
        guint8,
        libc::c_uchar,
        *mut libc::c_uchar,
        size_t,
        *mut libc::c_void,
        *mut GError,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct read_msg_data {
    pub buflen: gssize,
    pub buffer: *mut guint8,
    pub callback: read_msg_cb_fn,
    pub user_data: *mut libc::c_void,
}
pub const READ_MSG01: deinitsm_states = 1;
pub const SEND_RESP07: deinitsm_states = 0;
pub const ENROLL_START_NUM_STATES: enroll_start_sm_states = 3;
pub const READ_ENROLL_MSG28: enroll_start_sm_states = 2;
pub const ENROLL_INIT: enroll_start_sm_states = 1;
pub const INITSM_NUM_STATES: initsm_states = 14;
pub const READ28_0B: initsm_states = 13;
pub const SEND28_0B: initsm_states = 12;
pub const READ28_0C: initsm_states = 11;
pub const SEND28_0C: initsm_states = 10;
pub const READ28_08: initsm_states = 9;
pub const SEND28_08: initsm_states = 8;
pub const READ28_07: initsm_states = 7;
pub const SEND28_07: initsm_states = 6;
pub const READ28_06: initsm_states = 5;
pub const SEND28_06: initsm_states = 4;
pub const READ_MSG05: initsm_states = 3;
pub const SEND_RESP03: initsm_states = 2;
pub const READ_MSG03: initsm_states = 1;
pub const WRITE_CTRL400: initsm_states = 0;
pub const RUN_INITSM: enroll_start_sm_states = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VerifyStopData {
    pub error: *mut GError,
}
pub const VERIFY_NUM_STATES: C2RustUnnamed_6 = 2;
pub const VERIFY_INIT: C2RustUnnamed_6 = 1;
pub const VERIFY_RUN_INITSM: C2RustUnnamed_6 = 0;
pub type initsm_states = libc::c_uint;
pub type deinitsm_states = libc::c_uint;
pub type enroll_start_sm_states = libc::c_uint;
pub type C2RustUnnamed_6 = libc::c_uint;
#[inline]
unsafe extern "C" fn g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = 0 as *mut libc::c_void;
    ref_0 = *ptr;
    *ptr = 0 as *mut libc::c_void;
    return ref_0;
}
#[inline]
unsafe extern "C" fn FP_DEVICE_CLASS(mut ptr: gpointer) -> *mut FpDeviceClass {
    return g_type_check_class_cast(ptr as *mut GTypeClass, fp_device_get_type())
        as *mut libc::c_void as *mut FpDeviceClass;
}
#[inline]
unsafe extern "C" fn FPI_DEVICE_UPEKTS(mut ptr: gpointer) -> *mut FpiDeviceUpekts {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_upekts_get_type(),
    ) as *mut libc::c_void as *mut FpiDeviceUpekts;
}
static mut FpiDeviceUpekts_private_offset: gint = 0;
static mut fpi_device_upekts_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
unsafe extern "C" fn fpi_device_upekts_class_intern_init(mut klass: gpointer) {
    fpi_device_upekts_parent_class = g_type_class_peek_parent(klass);
    if FpiDeviceUpekts_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpiDeviceUpekts_private_offset);
    }
    fpi_device_upekts_class_init(klass as *mut FpiDeviceUpektsClass);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_upekts_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_upekts_get_type_once();
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
unsafe extern "C" fn fpi_device_upekts_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_device_get_type(),
        g_intern_static_string(b"FpiDeviceUpekts\0" as *const u8 as *const libc::c_char),
        ::core::mem::size_of::<FpiDeviceUpektsClass>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_upekts_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpiDeviceUpekts>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpiDeviceUpekts) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_upekts_init
                        as unsafe extern "C" fn(*mut FpiDeviceUpekts) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn alloc_send_cmd_transfer(
    mut dev: *mut FpDevice,
    mut seq_a: libc::c_uchar,
    mut seq_b: libc::c_uchar,
    mut data: *const libc::c_uchar,
    mut len: guint16,
) -> *mut FpiUsbTransfer {
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(dev);
    let mut crc: guint16 = 0;
    let mut ciao: *const libc::c_char = b"Ciao\0" as *const u8 as *const libc::c_char;
    let mut urblen: size_t = (len as libc::c_int + 9 as libc::c_int) as size_t;
    if data.is_null() && len as libc::c_int > 0 as libc::c_int {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"len>0 but no data?\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FpiUsbTransfer;
    }
    fpi_usb_transfer_fill_bulk(
        transfer,
        (2 as libc::c_int | 0 as libc::c_int) as guint8,
        urblen,
    );
    memcpy(
        (*transfer).buffer as *mut libc::c_void,
        ciao as *const libc::c_void,
        strlen(ciao),
    );
    *((*transfer).buffer).offset(4 as libc::c_int as isize) = seq_a;
    *((*transfer).buffer)
        .offset(
            5 as libc::c_int as isize,
        ) = (seq_b as libc::c_int
        | (len as libc::c_int & 0xf00 as libc::c_int) >> 8 as libc::c_int) as guchar;
    *((*transfer).buffer)
        .offset(
            6 as libc::c_int as isize,
        ) = (len as libc::c_int & 0xff as libc::c_int) as guchar;
    if !data.is_null() {
        memcpy(
            ((*transfer).buffer).offset(7 as libc::c_int as isize) as *mut libc::c_void,
            data as *const libc::c_void,
            len as libc::c_ulong,
        );
    }
    crc = udf_crc(
        ((*transfer).buffer).offset(4 as libc::c_int as isize),
        urblen.wrapping_sub(6 as libc::c_int as libc::c_ulong),
    );
    *((*transfer).buffer)
        .offset(
            urblen.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
        ) = (crc as libc::c_int & 0xff as libc::c_int) as guchar;
    *((*transfer).buffer)
        .offset(
            urblen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = (crc as libc::c_int >> 8 as libc::c_int) as guchar;
    return transfer;
}
unsafe extern "C" fn alloc_send_cmd28_transfer(
    mut dev: *mut FpDevice,
    mut subcmd: libc::c_uchar,
    mut data: *const libc::c_uchar,
    mut innerlen: guint16,
) -> *mut FpiUsbTransfer {
    let mut _innerlen: guint16 = innerlen;
    let mut len: size_t = (innerlen as libc::c_int + 6 as libc::c_int) as size_t;
    let mut buf: *mut libc::c_uchar = g_malloc0(len) as *mut libc::c_uchar;
    let mut upekdev: *mut FpiDeviceUpekts = FPI_DEVICE_UPEKTS(dev as gpointer);
    let mut seq: guint8 = ((*upekdev).seq as libc::c_int + 0x10 as libc::c_int)
        as guint8;
    let mut ret: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    g_log(
        b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"seq=%02x subcmd=%02x with %d bytes of data\0" as *const u8
            as *const libc::c_char,
        seq as libc::c_int,
        subcmd as libc::c_int,
        innerlen as libc::c_int,
    );
    _innerlen = (innerlen as libc::c_int + 3 as libc::c_int) as guint16;
    *buf.offset(0 as libc::c_int as isize) = 0x28 as libc::c_int as libc::c_uchar;
    *buf
        .offset(
            1 as libc::c_int as isize,
        ) = (_innerlen as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    *buf
        .offset(
            2 as libc::c_int as isize,
        ) = ((_innerlen as libc::c_int & 0xff00 as libc::c_int) >> 8 as libc::c_int)
        as libc::c_uchar;
    *buf.offset(5 as libc::c_int as isize) = subcmd;
    memcpy(
        buf.offset(6 as libc::c_int as isize) as *mut libc::c_void,
        data as *const libc::c_void,
        innerlen as libc::c_ulong,
    );
    ret = alloc_send_cmd_transfer(
        dev,
        0 as libc::c_int as libc::c_uchar,
        seq,
        buf,
        len as guint16,
    );
    (*upekdev).seq = seq;
    g_free(buf as gpointer);
    return ret;
}
unsafe extern "C" fn alloc_send_cmdresponse_transfer(
    mut dev: *mut FpDevice,
    mut seq: libc::c_uchar,
    mut data: *const libc::c_uchar,
    mut len: guint8,
) -> *mut FpiUsbTransfer {
    g_log(
        b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"seq=%02x len=%d\0" as *const u8 as *const libc::c_char,
        seq as libc::c_int,
        len as libc::c_int,
    );
    return alloc_send_cmd_transfer(
        dev,
        seq,
        0 as libc::c_int as libc::c_uchar,
        data,
        len as guint16,
    );
}
unsafe extern "C" fn busy_ack_sent_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut udata: *mut read_msg_data = user_data as *mut read_msg_data;
    if !error.is_null() {
        ((*udata).callback)
            .expect(
                "non-null function pointer",
            )(
            device,
            READ_MSG_CMD,
            0 as libc::c_int as guint8,
            0 as libc::c_int as libc::c_uchar,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
            (*udata).user_data,
            error,
        );
        g_free((*udata).buffer as gpointer);
        g_free(udata as gpointer);
    } else {
        __read_msg_async(device, udata);
    };
}
unsafe extern "C" fn busy_ack_retry_read(
    mut device: *mut FpDevice,
    mut udata: *mut read_msg_data,
) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    transfer = alloc_send_cmdresponse_transfer(
        device,
        0x9 as libc::c_int as libc::c_uchar,
        0 as *const libc::c_uchar,
        0 as libc::c_int as guint8,
    );
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_submit(
        transfer,
        5000 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            busy_ack_sent_cb
                as unsafe extern "C" fn(
                    *mut FpiUsbTransfer,
                    *mut FpDevice,
                    gpointer,
                    *mut GError,
                ) -> (),
        ),
        udata as gpointer,
    );
}
unsafe extern "C" fn __handle_incoming_msg(
    mut device: *mut FpDevice,
    mut udata: *mut read_msg_data,
) {
    let mut current_block: u64;
    let mut error: *mut GError = 0 as *mut GError;
    let mut buf: *mut guint8 = (*udata).buffer;
    let mut len: guint16 = 0;
    let mut computed_crc: guint16 = 0;
    let mut msg_crc: guint16 = 0;
    let mut code_a: libc::c_uchar = 0;
    let mut code_b: libc::c_uchar = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*udata).buflen >= 6 as libc::c_int as libc::c_long {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/upekts.c\0" as *const u8 as *const libc::c_char,
            244 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"__handle_incoming_msg\0"))
                .as_ptr(),
            b"udata->buflen >= 6\0" as *const u8 as *const libc::c_char,
        );
    }
    len = ((*buf.offset(5 as libc::c_int as isize) as libc::c_int & 0xf as libc::c_int)
        << 8 as libc::c_int | *buf.offset(6 as libc::c_int as isize) as libc::c_int)
        as guint16;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*udata).buflen >= (len as libc::c_int + 9 as libc::c_int) as libc::c_long {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/upekts.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"__handle_incoming_msg\0"))
                .as_ptr(),
            b"udata->buflen >= len + 9\0" as *const u8 as *const libc::c_char,
        );
    }
    computed_crc = udf_crc(
        buf.offset(4 as libc::c_int as isize),
        (len as libc::c_int + 3 as libc::c_int) as size_t,
    );
    msg_crc = ((*buf.offset((len as libc::c_int + 8 as libc::c_int) as isize)
        as libc::c_int) << 8 as libc::c_int
        | *buf.offset((len as libc::c_int + 7 as libc::c_int) as isize) as libc::c_int)
        as guint16;
    if computed_crc as libc::c_int != msg_crc as libc::c_int {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"CRC failed, got %04x expected %04x\0" as *const u8 as *const libc::c_char,
            msg_crc as libc::c_int,
            computed_crc as libc::c_int,
        );
        error = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_PROTO,
            b"CRC check on message failed\0" as *const u8 as *const libc::c_char,
        );
        current_block = 4740710386309217408;
    } else {
        code_a = *buf.offset(4 as libc::c_int as isize);
        code_b = (*buf.offset(5 as libc::c_int as isize) as libc::c_int
            & 0xf0 as libc::c_int) as libc::c_uchar;
        len = ((*buf.offset(5 as libc::c_int as isize) as libc::c_int
            & 0xf as libc::c_int) << 8 as libc::c_int
            | *buf.offset(6 as libc::c_int as isize) as libc::c_int) as guint16;
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"A=%02x B=%02x len=%d\0" as *const u8 as *const libc::c_char,
            code_a as libc::c_int,
            code_b as libc::c_int,
            len as libc::c_int,
        );
        if code_a as libc::c_int != 0 && code_b == 0 {
            g_log(
                b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"cmd %x from device to driver\0" as *const u8 as *const libc::c_char,
                code_a as libc::c_int,
            );
            if code_a as libc::c_int == 0x8 as libc::c_int {
                g_log(
                    b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"device busy, send busy-ack\0" as *const u8 as *const libc::c_char,
                );
                busy_ack_retry_read(device, udata);
                return;
            }
            ((*udata).callback)
                .expect(
                    "non-null function pointer",
                )(
                device,
                READ_MSG_CMD,
                code_a,
                0 as libc::c_int as libc::c_uchar,
                buf.offset(7 as libc::c_int as isize),
                len as size_t,
                (*udata).user_data,
                0 as *mut GError,
            );
            current_block = 12066412874945307152;
        } else if code_a == 0 {
            let mut innerbuf: *mut libc::c_uchar = buf.offset(7 as libc::c_int as isize);
            let mut _subcmd: libc::c_uchar = 0;
            let mut innerlen: guint16 = 0;
            if (len as libc::c_int) < 6 as libc::c_int {
                g_log(
                    b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_WARNING,
                    b"cmd response too short (%d)\0" as *const u8 as *const libc::c_char,
                    len as libc::c_int,
                );
                error = fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_PROTO,
                    b"CMD response too short (%d)\0" as *const u8 as *const libc::c_char,
                    len as libc::c_int,
                );
                current_block = 4740710386309217408;
            } else if *innerbuf.offset(0 as libc::c_int as isize) as libc::c_int
                != 0x28 as libc::c_int
            {
                g_log(
                    b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_WARNING,
                    b"cmd response without 28 byte?\0" as *const u8
                        as *const libc::c_char,
                );
                error = fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_PROTO,
                    b"CMD response without 0x28 byte\0" as *const u8
                        as *const libc::c_char,
                );
                current_block = 4740710386309217408;
            } else {
                if *innerbuf.offset(3 as libc::c_int as isize) as libc::c_int != 0
                    || *innerbuf.offset(4 as libc::c_int as isize) as libc::c_int != 0
                {
                    g_log(
                        b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"non-zero bytes in cmd response\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                innerlen = (*innerbuf.offset(1 as libc::c_int as isize) as libc::c_int
                    | (*innerbuf.offset(2 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int) as guint16;
                innerlen = (innerlen as libc::c_int - 3 as libc::c_int) as guint16;
                _subcmd = *innerbuf.offset(5 as libc::c_int as isize);
                g_log(
                    b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"device responds to subcmd %x with %d bytes\0" as *const u8
                        as *const libc::c_char,
                    _subcmd as libc::c_int,
                    innerlen as libc::c_int,
                );
                ((*udata).callback)
                    .expect(
                        "non-null function pointer",
                    )(
                    device,
                    READ_MSG_RESPONSE,
                    code_b,
                    _subcmd,
                    innerbuf.offset(6 as libc::c_int as isize),
                    innerlen as size_t,
                    (*udata).user_data,
                    0 as *mut GError,
                );
                current_block = 12066412874945307152;
            }
        } else {
            g_log(
                b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"don't know how to handle this message\0" as *const u8
                    as *const libc::c_char,
            );
            error = fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Message cannot be processed\0" as *const u8 as *const libc::c_char,
            );
            current_block = 4740710386309217408;
        }
    }
    match current_block {
        4740710386309217408 => {
            ((*udata).callback)
                .expect(
                    "non-null function pointer",
                )(
                device,
                READ_MSG_CMD,
                0 as libc::c_int as guint8,
                0 as libc::c_int as libc::c_uchar,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
                (*udata).user_data,
                error,
            );
        }
        _ => {}
    }
    g_free((*udata).buffer as gpointer);
    g_free(udata as gpointer);
}
unsafe extern "C" fn read_msg_extend_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut udata: *mut read_msg_data = user_data as *mut read_msg_data;
    if !error.is_null() {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"extended msg read failed: %s\0" as *const u8 as *const libc::c_char,
            (*error).message,
        );
        ((*udata).callback)
            .expect(
                "non-null function pointer",
            )(
            device,
            READ_MSG_CMD,
            0 as libc::c_int as guint8,
            0 as libc::c_int as libc::c_uchar,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
            (*udata).user_data,
            error,
        );
        g_free((*udata).buffer as gpointer);
        g_free(udata as gpointer);
        return;
    }
    __handle_incoming_msg(device, udata);
}
unsafe extern "C" fn read_msg_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut udata: *mut read_msg_data = user_data as *mut read_msg_data;
    let mut payload_len: guint16 = 0;
    let mut packet_len: gsize = 0;
    if !error.is_null() {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"async msg read failed: %s\0" as *const u8 as *const libc::c_char,
            (*error).message,
        );
    } else if (*transfer).actual_length < 9 as libc::c_int as libc::c_long {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"async msg read too short (%d)\0" as *const u8 as *const libc::c_char,
            (*transfer).actual_length as gint,
        );
        error = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_PROTO,
            b"Packet from device was too short (%li)\0" as *const u8
                as *const libc::c_char,
            (*transfer).actual_length,
        );
    } else if strncmp(
        (*udata).buffer as *mut libc::c_char,
        b"Ciao\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"no Ciao for you!!\0" as *const u8 as *const libc::c_char,
        );
        error = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_PROTO,
            b"Packet from device had incorrect header\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        payload_len = ((*((*udata).buffer).offset(5 as libc::c_int as isize)
            as libc::c_int & 0xf as libc::c_int) << 8 as libc::c_int
            | *((*udata).buffer).offset(6 as libc::c_int as isize) as libc::c_int)
            as guint16;
        packet_len = (payload_len as libc::c_int + 9 as libc::c_int) as gsize;
        if (*transfer).actual_length != 0x40 as libc::c_int as libc::c_long
            && packet_len > (*transfer).actual_length as libc::c_ulong
        {
            g_log(
                b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"msg didn't include enough data, expected=%d recv=%d\0" as *const u8
                    as *const libc::c_char,
                packet_len as gint,
                (*transfer).actual_length as gint,
            );
            error = fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Packet from device didn't include data\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            if packet_len > 0x40 as libc::c_int as libc::c_ulong {
                let mut needed: libc::c_int = packet_len
                    .wrapping_sub(0x40 as libc::c_int as libc::c_ulong) as libc::c_int;
                let mut etransfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(device);
                g_log(
                    b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"didn't fit in buffer, need to extend by %d bytes\0" as *const u8
                        as *const libc::c_char,
                    needed,
                );
                (*udata)
                    .buffer = g_realloc((*udata).buffer as gpointer, packet_len)
                    as *mut guint8;
                (*udata).buflen = packet_len as gssize;
                fpi_usb_transfer_fill_bulk_full(
                    etransfer,
                    (1 as libc::c_int | 0x80 as libc::c_int) as guint8,
                    ((*udata).buffer).offset(0x40 as libc::c_int as isize),
                    needed as gsize,
                    None,
                );
                (*etransfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
                fpi_usb_transfer_submit(
                    etransfer,
                    5000 as libc::c_int as guint,
                    0 as *mut GCancellable,
                    Some(
                        read_msg_extend_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                    udata as gpointer,
                );
                return;
            }
            __handle_incoming_msg(device, udata);
            return;
        }
    }
    ((*udata).callback)
        .expect(
            "non-null function pointer",
        )(
        device,
        READ_MSG_CMD,
        0 as libc::c_int as guint8,
        0 as libc::c_int as libc::c_uchar,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as size_t,
        (*udata).user_data,
        error,
    );
    g_free((*udata).buffer as gpointer);
    g_free(udata as gpointer);
}
unsafe extern "C" fn __read_msg_async(
    mut device: *mut FpDevice,
    mut udata: *mut read_msg_data,
) {
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(device);
    if (*udata).buflen != 0x40 as libc::c_int as libc::c_long {
        (*udata)
            .buffer = g_realloc(
            (*udata).buffer as gpointer,
            0x40 as libc::c_int as gsize,
        ) as *mut guint8;
        (*udata).buflen = 0x40 as libc::c_int as gssize;
    }
    fpi_usb_transfer_fill_bulk_full(
        transfer,
        (1 as libc::c_int | 0x80 as libc::c_int) as guint8,
        (*udata).buffer,
        (*udata).buflen as gsize,
        None,
    );
    fpi_usb_transfer_submit(
        transfer,
        5000 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            read_msg_cb
                as unsafe extern "C" fn(
                    *mut FpiUsbTransfer,
                    *mut FpDevice,
                    gpointer,
                    *mut GError,
                ) -> (),
        ),
        udata as gpointer,
    );
}
unsafe extern "C" fn read_msg_async(
    mut dev: *mut FpDevice,
    mut callback: read_msg_cb_fn,
    mut user_data: *mut libc::c_void,
) {
    let mut udata: *mut read_msg_data = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<read_msg_data>() as libc::c_ulong;
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
    }) as *mut read_msg_data;
    (*udata).buflen = 0 as libc::c_int as gssize;
    (*udata).buffer = 0 as *mut guint8;
    (*udata).callback = callback;
    (*udata).user_data = user_data;
    __read_msg_async(dev, udata);
}
static mut init_resp03: [libc::c_uchar; 8] = [
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
];
static mut init28_08: [libc::c_uchar; 25] = [
    0x4 as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0x97 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xa7 as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0xa0 as libc::c_int as libc::c_uchar,
    0x8a as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0xbf as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0xf3 as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0xc4 as libc::c_int as libc::c_uchar,
];
static mut init28_0c: [libc::c_uchar; 5] = [
    0x4 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
static mut init28_0b: [libc::c_uchar; 105] = [
    0x4 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
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
    0x64 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
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
    0x8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn initsm_read_msg_response_cb(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut type_0: read_msg_type,
    mut seq: guint8,
    mut expect_subcmd: libc::c_uchar,
    mut subcmd: libc::c_uchar,
    mut error: *mut GError,
) {
    let mut upekdev: *mut FpiDeviceUpekts = FPI_DEVICE_UPEKTS(dev as gpointer);
    if !error.is_null() {
        fpi_ssm_mark_failed(ssm, error);
    } else if type_0 as libc::c_uint != READ_MSG_RESPONSE as libc::c_int as libc::c_uint
    {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"expected response, got %d seq=%x in state %d\0" as *const u8
                as *const libc::c_char,
            type_0 as libc::c_uint,
            seq as libc::c_int,
            fpi_ssm_get_cur_state(ssm),
        );
        fpi_ssm_mark_failed(
            ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Unexpected message type\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else if seq as libc::c_int != (*upekdev).seq as libc::c_int {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"expected response to subcmd 0x%02x, got response to %02x in state %d\0"
                as *const u8 as *const libc::c_char,
            expect_subcmd as libc::c_int,
            subcmd as libc::c_int,
            fpi_ssm_get_cur_state(ssm),
        );
        fpi_ssm_mark_failed(
            ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Unexpected response subcommand\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else {
        fpi_ssm_next_state(ssm);
    };
}
unsafe extern "C" fn read28_0b_cb(
    mut dev: *mut FpDevice,
    mut type_0: read_msg_type,
    mut seq: guint8,
    mut subcmd: libc::c_uchar,
    mut data: *mut libc::c_uchar,
    mut data_len: size_t,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    initsm_read_msg_response_cb(
        user_data as *mut FpiSsm,
        dev,
        type_0,
        seq,
        0xb as libc::c_int as libc::c_uchar,
        subcmd,
        error,
    );
}
unsafe extern "C" fn read28_0c_cb(
    mut dev: *mut FpDevice,
    mut type_0: read_msg_type,
    mut seq: guint8,
    mut subcmd: libc::c_uchar,
    mut data: *mut libc::c_uchar,
    mut data_len: size_t,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    initsm_read_msg_response_cb(
        user_data as *mut FpiSsm,
        dev,
        type_0,
        seq,
        0xc as libc::c_int as libc::c_uchar,
        subcmd,
        error,
    );
}
unsafe extern "C" fn read28_08_cb(
    mut dev: *mut FpDevice,
    mut type_0: read_msg_type,
    mut seq: guint8,
    mut subcmd: libc::c_uchar,
    mut data: *mut libc::c_uchar,
    mut data_len: size_t,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    initsm_read_msg_response_cb(
        user_data as *mut FpiSsm,
        dev,
        type_0,
        seq,
        0x8 as libc::c_int as libc::c_uchar,
        subcmd,
        error,
    );
}
unsafe extern "C" fn read28_07_cb(
    mut dev: *mut FpDevice,
    mut type_0: read_msg_type,
    mut seq: guint8,
    mut subcmd: libc::c_uchar,
    mut data: *mut libc::c_uchar,
    mut data_len: size_t,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    initsm_read_msg_response_cb(
        user_data as *mut FpiSsm,
        dev,
        type_0,
        seq,
        0x7 as libc::c_int as libc::c_uchar,
        subcmd,
        error,
    );
}
unsafe extern "C" fn read28_06_cb(
    mut dev: *mut FpDevice,
    mut type_0: read_msg_type,
    mut seq: guint8,
    mut subcmd: libc::c_uchar,
    mut data: *mut libc::c_uchar,
    mut data_len: size_t,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    initsm_read_msg_response_cb(
        user_data as *mut FpiSsm,
        dev,
        type_0,
        seq,
        0x6 as libc::c_int as libc::c_uchar,
        subcmd,
        error,
    );
}
unsafe extern "C" fn initsm_read_msg_cmd_cb(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut type_0: read_msg_type,
    mut seq: guint8,
    mut expected_seq: guint8,
    mut error: *mut GError,
) {
    let mut upekdev: *mut FpiDeviceUpekts = FPI_DEVICE_UPEKTS(dev as gpointer);
    if !error.is_null() {
        fpi_ssm_mark_failed(ssm, error);
        return;
    } else {
        if type_0 as libc::c_uint != READ_MSG_CMD as libc::c_int as libc::c_uint {
            g_log(
                b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"expected command, got %d seq=%x in state %d\0" as *const u8
                    as *const libc::c_char,
                type_0 as libc::c_uint,
                seq as libc::c_int,
                fpi_ssm_get_cur_state(ssm),
            );
            fpi_ssm_mark_failed(
                ssm,
                fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_PROTO,
                    b"Expected command but got response\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            return;
        }
    }
    (*upekdev).seq = seq;
    if seq as libc::c_int != expected_seq as libc::c_int {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"expected seq=%x, got %x in state %d\0" as *const u8 as *const libc::c_char,
            expected_seq as libc::c_int,
            seq as libc::c_int,
            fpi_ssm_get_cur_state(ssm),
        );
        fpi_ssm_mark_failed(
            ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Got unexpected sequence number\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    fpi_ssm_next_state(ssm);
}
unsafe extern "C" fn read_msg05_cb(
    mut dev: *mut FpDevice,
    mut type_0: read_msg_type,
    mut seq: guint8,
    mut subcmd: libc::c_uchar,
    mut data: *mut libc::c_uchar,
    mut data_len: size_t,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    initsm_read_msg_cmd_cb(
        user_data as *mut FpiSsm,
        dev,
        type_0,
        5 as libc::c_int as guint8,
        seq,
        error,
    );
}
unsafe extern "C" fn read_msg03_cb(
    mut dev: *mut FpDevice,
    mut type_0: read_msg_type,
    mut seq: guint8,
    mut subcmd: libc::c_uchar,
    mut data: *mut libc::c_uchar,
    mut data_len: size_t,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    initsm_read_msg_cmd_cb(
        user_data as *mut FpiSsm,
        dev,
        type_0,
        3 as libc::c_int as guint8,
        seq,
        error,
    );
}
unsafe extern "C" fn initsm_read_msg_handler(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut callback: read_msg_cb_fn,
) {
    read_msg_async(dev, callback, ssm as *mut libc::c_void);
}
unsafe extern "C" fn initsm_send_msg28_handler(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut subcmd: libc::c_uchar,
    mut data: *const libc::c_uchar,
    mut innerlen: guint16,
) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    transfer = alloc_send_cmd28_transfer(dev, subcmd, data, innerlen);
    (*transfer).ssm = ssm;
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_submit(
        transfer,
        5000 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            fpi_ssm_usb_transfer_cb
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
unsafe extern "C" fn initsm_run_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut upekdev: *mut FpiDeviceUpekts = FPI_DEVICE_UPEKTS(dev as gpointer);
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    let mut dummy28_06: libc::c_uchar = 0;
    let mut dummy28_07: libc::c_uchar = 0;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            transfer = fpi_usb_transfer_new(dev);
            fpi_usb_transfer_fill_control(
                transfer,
                G_USB_DEVICE_DIRECTION_HOST_TO_DEVICE,
                G_USB_DEVICE_REQUEST_TYPE_VENDOR,
                G_USB_DEVICE_RECIPIENT_DEVICE,
                0xc as libc::c_int as guint8,
                0x100 as libc::c_int as guint16,
                0x400 as libc::c_int as guint16,
                1 as libc::c_int as gsize,
            );
            (*transfer).ssm = ssm;
            (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
            fpi_usb_transfer_submit(
                transfer,
                5000 as libc::c_int as guint,
                0 as *mut GCancellable,
                Some(
                    fpi_ssm_usb_transfer_cb
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
        1 => {
            initsm_read_msg_handler(
                ssm,
                dev,
                Some(
                    read_msg03_cb
                        as unsafe extern "C" fn(
                            *mut FpDevice,
                            read_msg_type,
                            guint8,
                            libc::c_uchar,
                            *mut libc::c_uchar,
                            size_t,
                            *mut libc::c_void,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        2 => {
            (*upekdev).seq = ((*upekdev).seq).wrapping_add(1);
            transfer = alloc_send_cmdresponse_transfer(
                dev,
                (*upekdev).seq,
                init_resp03.as_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong as guint8,
            );
            (*transfer).ssm = ssm;
            (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
            fpi_usb_transfer_submit(
                transfer,
                5000 as libc::c_int as guint,
                0 as *mut GCancellable,
                Some(
                    fpi_ssm_usb_transfer_cb
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
        3 => {
            initsm_read_msg_handler(
                ssm,
                dev,
                Some(
                    read_msg05_cb
                        as unsafe extern "C" fn(
                            *mut FpDevice,
                            read_msg_type,
                            guint8,
                            libc::c_uchar,
                            *mut libc::c_uchar,
                            size_t,
                            *mut libc::c_void,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        4 => {
            dummy28_06 = 0x4 as libc::c_int as libc::c_uchar;
            (*upekdev).seq = 0xf0 as libc::c_int as guint8;
            initsm_send_msg28_handler(
                ssm,
                dev,
                0x6 as libc::c_int as libc::c_uchar,
                &mut dummy28_06,
                1 as libc::c_int as guint16,
            );
        }
        5 => {
            initsm_read_msg_handler(
                ssm,
                dev,
                Some(
                    read28_06_cb
                        as unsafe extern "C" fn(
                            *mut FpDevice,
                            read_msg_type,
                            guint8,
                            libc::c_uchar,
                            *mut libc::c_uchar,
                            size_t,
                            *mut libc::c_void,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        6 => {
            dummy28_07 = 0x4 as libc::c_int as libc::c_uchar;
            initsm_send_msg28_handler(
                ssm,
                dev,
                0x7 as libc::c_int as libc::c_uchar,
                &mut dummy28_07,
                1 as libc::c_int as guint16,
            );
        }
        7 => {
            initsm_read_msg_handler(
                ssm,
                dev,
                Some(
                    read28_07_cb
                        as unsafe extern "C" fn(
                            *mut FpDevice,
                            read_msg_type,
                            guint8,
                            libc::c_uchar,
                            *mut libc::c_uchar,
                            size_t,
                            *mut libc::c_void,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        8 => {
            initsm_send_msg28_handler(
                ssm,
                dev,
                0x8 as libc::c_int as libc::c_uchar,
                init28_08.as_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 25]>() as libc::c_ulong as guint16,
            );
        }
        9 => {
            initsm_read_msg_handler(
                ssm,
                dev,
                Some(
                    read28_08_cb
                        as unsafe extern "C" fn(
                            *mut FpDevice,
                            read_msg_type,
                            guint8,
                            libc::c_uchar,
                            *mut libc::c_uchar,
                            size_t,
                            *mut libc::c_void,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        10 => {
            initsm_send_msg28_handler(
                ssm,
                dev,
                0xc as libc::c_int as libc::c_uchar,
                init28_0c.as_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 5]>() as libc::c_ulong as guint16,
            );
        }
        11 => {
            initsm_read_msg_handler(
                ssm,
                dev,
                Some(
                    read28_0c_cb
                        as unsafe extern "C" fn(
                            *mut FpDevice,
                            read_msg_type,
                            guint8,
                            libc::c_uchar,
                            *mut libc::c_uchar,
                            size_t,
                            *mut libc::c_void,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        12 => {
            initsm_send_msg28_handler(
                ssm,
                dev,
                0xb as libc::c_int as libc::c_uchar,
                init28_0b.as_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 105]>() as libc::c_ulong
                    as guint16,
            );
        }
        13 => {
            initsm_read_msg_handler(
                ssm,
                dev,
                Some(
                    read28_0b_cb
                        as unsafe extern "C" fn(
                            *mut FpDevice,
                            read_msg_type,
                            guint8,
                            libc::c_uchar,
                            *mut libc::c_uchar,
                            size_t,
                            *mut libc::c_void,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn initsm_new(mut dev: *mut FpDevice) -> *mut FpiSsm {
    return fpi_ssm_new_full(
        dev,
        Some(initsm_run_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
        INITSM_NUM_STATES as libc::c_int,
        INITSM_NUM_STATES as libc::c_int,
        b"INITSM_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn read_msg01_cb(
    mut dev: *mut FpDevice,
    mut type_0: read_msg_type,
    mut seq: guint8,
    mut subcmd: libc::c_uchar,
    mut data: *mut libc::c_uchar,
    mut data_len: size_t,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut ssm: *mut FpiSsm = user_data as *mut FpiSsm;
    let mut upekdev: *mut FpiDeviceUpekts = FPI_DEVICE_UPEKTS(dev as gpointer);
    if !error.is_null() {
        fpi_ssm_mark_failed(ssm, error);
        return;
    } else {
        if type_0 as libc::c_uint != READ_MSG_CMD as libc::c_int as libc::c_uint {
            g_log(
                b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"expected command, got %d seq=%x\0" as *const u8 as *const libc::c_char,
                type_0 as libc::c_uint,
                seq as libc::c_int,
            );
            fpi_ssm_mark_failed(
                ssm,
                fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_PROTO,
                    b"Expected command but got response\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            return;
        }
    }
    (*upekdev).seq = seq;
    if seq as libc::c_int != 1 as libc::c_int {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"expected seq=1, got %x\0" as *const u8 as *const libc::c_char,
            seq as libc::c_int,
        );
        fpi_ssm_mark_failed(
            ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Got wrong sequence number (%x)\0" as *const u8 as *const libc::c_char,
                seq as libc::c_int,
            ),
        );
        return;
    }
    fpi_ssm_next_state(ssm);
}
unsafe extern "C" fn deinitsm_state_handler(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    let mut dummy: libc::c_uchar = 0;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            transfer = 0 as *mut FpiUsbTransfer;
            dummy = 0 as libc::c_int as libc::c_uchar;
            transfer = alloc_send_cmdresponse_transfer(
                dev,
                0x7 as libc::c_int as libc::c_uchar,
                &mut dummy,
                1 as libc::c_int as guint8,
            );
            (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
            (*transfer).ssm = ssm;
            fpi_usb_transfer_submit(
                transfer,
                5000 as libc::c_int as guint,
                0 as *mut GCancellable,
                Some(
                    fpi_ssm_usb_transfer_cb
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
        1 => {
            read_msg_async(
                dev,
                Some(
                    read_msg01_cb
                        as unsafe extern "C" fn(
                            *mut FpDevice,
                            read_msg_type,
                            guint8,
                            libc::c_uchar,
                            *mut libc::c_uchar,
                            size_t,
                            *mut libc::c_void,
                            *mut GError,
                        ) -> (),
                ),
                ssm as *mut libc::c_void,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn initsm_done(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    if !error.is_null() {
        g_usb_device_release_interface(
            fpi_device_get_usb_device(dev),
            0 as libc::c_int,
            G_USB_DEVICE_CLAIM_INTERFACE_NONE,
            0 as *mut *mut GError,
        );
    }
    fpi_device_open_complete(dev, error);
}
unsafe extern "C" fn deinitsm_new(
    mut dev: *mut FpDevice,
    mut user_data: *mut libc::c_void,
) -> *mut FpiSsm {
    return fpi_ssm_new_full(
        dev,
        Some(
            deinitsm_state_handler
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        DEINITSM_NUM_STATES as libc::c_int,
        DEINITSM_NUM_STATES as libc::c_int,
        b"DEINITSM_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn dev_init(mut dev: *mut FpDevice) {
    let mut ssm: *mut FpiSsm = 0 as *mut FpiSsm;
    let mut error: *mut GError = 0 as *mut GError;
    let mut upekdev: *mut FpiDeviceUpekts = FPI_DEVICE_UPEKTS(dev as gpointer);
    if g_usb_device_claim_interface(
        fpi_device_get_usb_device(dev),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    ) == 0
    {
        fpi_device_open_complete(dev, error);
        return;
    }
    (*upekdev).seq = 0xf0 as libc::c_int as guint8;
    ssm = fpi_ssm_new_full(
        dev,
        Some(initsm_run_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
        INITSM_NUM_STATES as libc::c_int,
        INITSM_NUM_STATES as libc::c_int,
        b"INITSM_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            initsm_done
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn dev_exit(mut dev: *mut FpDevice) {
    let mut error: *mut GError = 0 as *mut GError;
    g_usb_device_release_interface(
        fpi_device_get_usb_device(dev),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    );
    fpi_device_close_complete(dev, error);
}
static mut enroll_init: [libc::c_uchar; 8] = [
    0x2 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
];
static mut scan_comp: [libc::c_uchar; 5] = [
    0x12 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
];
static mut poll_data: [libc::c_uchar; 2] = [
    0x30 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn enroll_start_sm_cb_msg28(
    mut dev: *mut FpDevice,
    mut type_0: read_msg_type,
    mut seq: guint8,
    mut subcmd: libc::c_uchar,
    mut data: *mut libc::c_uchar,
    mut data_len: size_t,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut upekdev: *mut FpiDeviceUpekts = FPI_DEVICE_UPEKTS(dev as gpointer);
    let mut ssm: *mut FpiSsm = user_data as *mut FpiSsm;
    if !error.is_null() {
        fpi_ssm_mark_failed(ssm, error);
    } else if type_0 as libc::c_uint != READ_MSG_RESPONSE as libc::c_int as libc::c_uint
    {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"expected response, got %d seq=%x\0" as *const u8 as *const libc::c_char,
            type_0 as libc::c_uint,
            seq as libc::c_int,
        );
        fpi_ssm_mark_failed(
            ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Unexpected response type\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else if subcmd as libc::c_int != 0 as libc::c_int {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"expected response to subcmd 0, got response to %02x\0" as *const u8
                as *const libc::c_char,
            subcmd as libc::c_int,
        );
        fpi_ssm_mark_failed(
            ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Got response to wrong subcommand\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else if seq as libc::c_int != (*upekdev).seq as libc::c_int {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"expected response to cmd seq=%02x, got response to %02x\0" as *const u8
                as *const libc::c_char,
            (*upekdev).seq as libc::c_int,
            seq as libc::c_int,
        );
        fpi_ssm_mark_failed(
            ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Got response with wrong sequence number\0" as *const u8
                    as *const libc::c_char,
            ),
        );
    } else {
        fpi_ssm_next_state(ssm);
    };
}
unsafe extern "C" fn enroll_start_sm_run_state(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
) {
    let mut initsm: *mut FpiSsm = 0 as *mut FpiSsm;
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            initsm = initsm_new(dev);
            fpi_ssm_start_subsm(ssm, initsm);
        }
        1 => {
            transfer = 0 as *mut FpiUsbTransfer;
            transfer = alloc_send_cmd28_transfer(
                dev,
                0x2 as libc::c_int as libc::c_uchar,
                enroll_init.as_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong as guint16,
            );
            (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
            (*transfer).ssm = ssm;
            fpi_usb_transfer_submit(
                transfer,
                5000 as libc::c_int as guint,
                0 as *mut GCancellable,
                Some(
                    fpi_ssm_usb_transfer_cb
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
        2 => {
            read_msg_async(
                dev,
                Some(
                    enroll_start_sm_cb_msg28
                        as unsafe extern "C" fn(
                            *mut FpDevice,
                            read_msg_type,
                            guint8,
                            libc::c_uchar,
                            *mut libc::c_uchar,
                            size_t,
                            *mut libc::c_void,
                            *mut GError,
                        ) -> (),
                ),
                ssm as *mut libc::c_void,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn enroll_stop_data_free(mut data: *mut EnrollStopData) {
    let mut _pp: C2RustUnnamed_5 = C2RustUnnamed_5 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*data).print as *mut *mut FpPrint as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    g_clear_error(&mut (*data).error);
    g_free(data as gpointer);
}
unsafe extern "C" fn enroll_stop_deinit_cb(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut data: *mut EnrollStopData = fpi_ssm_get_data(ssm) as *mut EnrollStopData;
    if !error.is_null() {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Error deinitializing: %s\0" as *const u8 as *const libc::c_char,
            (*error).message,
        );
    }
    fpi_device_enroll_complete(
        dev,
        (if 0 as libc::c_int != 0 {
            (*data).print as *mut libc::c_void
        } else {
            g_steal_pointer(&mut (*data).print as *mut *mut FpPrint as gpointer)
        }) as *mut FpPrint,
        (if 0 as libc::c_int != 0 {
            (*data).error as *mut libc::c_void
        } else {
            g_steal_pointer(&mut (*data).error as *mut *mut GError as gpointer)
        }) as *mut GError,
    );
}
unsafe extern "C" fn do_enroll_stop(
    mut dev: *mut FpDevice,
    mut print: *mut FpPrint,
    mut error: *mut GError,
) {
    let mut data: *mut EnrollStopData = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<EnrollStopData>() as libc::c_ulong;
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
    }) as *mut EnrollStopData;
    let mut ssm: *mut FpiSsm = deinitsm_new(dev, data as *mut libc::c_void);
    (*data).print = print;
    (*data).error = error;
    fpi_ssm_start(
        ssm,
        Some(
            enroll_stop_deinit_cb
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
    fpi_ssm_set_data(
        ssm,
        data as gpointer,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut EnrollStopData) -> ()>,
            GDestroyNotify,
        >(Some(enroll_stop_data_free as unsafe extern "C" fn(*mut EnrollStopData) -> ())),
    );
}
unsafe extern "C" fn e_handle_resp00(
    mut dev: *mut FpDevice,
    mut data: *mut libc::c_uchar,
    mut data_len: size_t,
) {
    let mut upekdev: *mut FpiDeviceUpekts = FPI_DEVICE_UPEKTS(dev as gpointer);
    let mut status: libc::c_uchar = 0;
    if data_len != 14 as libc::c_int as libc::c_ulong {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"received 3001 poll response of %lu bytes?\0" as *const u8
                as *const libc::c_char,
            data_len,
        );
        do_enroll_stop(
            dev,
            0 as *mut FpPrint,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"received 3001 response with wrong length\0" as *const u8
                    as *const libc::c_char,
            ),
        );
        return;
    }
    status = *data.offset(5 as libc::c_int as isize);
    g_log(
        b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"poll result = %02x\0" as *const u8 as *const libc::c_char,
        status as libc::c_int,
    );
    let mut current_block_18: u64;
    match status as libc::c_int {
        12 | 13 | 14 | 38 | 39 | 46 => {
            if (*upekdev).enroll_passed != 0 {
                (*upekdev).enroll_passed = 0 as libc::c_int;
                (*upekdev).enroll_stage += 1 as libc::c_int;
                fpi_device_enroll_progress(
                    dev,
                    (*upekdev).enroll_stage,
                    0 as *mut FpPrint,
                    0 as *mut GError,
                );
            }
            current_block_18 = 11042950489265723346;
        }
        28 => {
            current_block_18 = 2301847198389468932;
        }
        11 | 35 => {
            current_block_18 = 2301847198389468932;
        }
        15 => {
            fpi_device_enroll_progress(
                dev,
                (*upekdev).enroll_stage,
                0 as *mut FpPrint,
                fpi_device_retry_new(FP_DEVICE_RETRY_REMOVE_FINGER),
            );
            current_block_18 = 11042950489265723346;
        }
        30 => {
            fpi_device_enroll_progress(
                dev,
                (*upekdev).enroll_stage,
                0 as *mut FpPrint,
                fpi_device_retry_new(FP_DEVICE_RETRY_TOO_SHORT),
            );
            current_block_18 = 11042950489265723346;
        }
        36 => {
            fpi_device_enroll_progress(
                dev,
                (*upekdev).enroll_stage,
                0 as *mut FpPrint,
                fpi_device_retry_new(FP_DEVICE_RETRY_CENTER_FINGER),
            );
            current_block_18 = 11042950489265723346;
        }
        32 => {
            (*upekdev).enroll_passed = (0 as libc::c_int == 0) as libc::c_int;
            current_block_18 = 11042950489265723346;
        }
        0 => {
            current_block_18 = 11042950489265723346;
        }
        _ => {
            do_enroll_stop(
                dev,
                0 as *mut FpPrint,
                fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_PROTO,
                    b"Unrecognised scan status code\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            return;
        }
    }
    match current_block_18 {
        2301847198389468932 => {
            fpi_device_enroll_progress(
                dev,
                (*upekdev).enroll_stage,
                0 as *mut FpPrint,
                fpi_device_retry_new(FP_DEVICE_RETRY_GENERAL),
            );
        }
        _ => {}
    }
    enroll_iterate(dev);
}
unsafe extern "C" fn e_handle_resp02(
    mut dev: *mut FpDevice,
    mut data: *mut libc::c_uchar,
    mut data_len: size_t,
) {
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    let mut error: *mut GError = 0 as *mut GError;
    if data_len < ::core::mem::size_of::<[libc::c_uchar; 5]>() as libc::c_ulong {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"fingerprint data too short (%luu bytes)\0" as *const u8
                as *const libc::c_char,
            data_len,
        );
        error = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_PROTO,
            b"fingerprint data too short\0" as *const u8 as *const libc::c_char,
        );
    } else if memcmp(
        data as *const libc::c_void,
        scan_comp.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_uchar; 5]>() as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"unrecognised data prefix %x %x %x %x %x\0" as *const u8
                as *const libc::c_char,
            *data.offset(0 as libc::c_int as isize) as libc::c_int,
            *data.offset(1 as libc::c_int as isize) as libc::c_int,
            *data.offset(2 as libc::c_int as isize) as libc::c_int,
            *data.offset(3 as libc::c_int as isize) as libc::c_int,
            *data.offset(4 as libc::c_int as isize) as libc::c_int,
        );
        error = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_PROTO,
            b"fingerprint data has wrong prefix\0" as *const u8 as *const libc::c_char,
        );
    } else {
        let mut fp_data: *mut GVariant = 0 as *mut GVariant;
        fpi_device_get_enroll_data(dev, &mut print);
        fp_data = g_variant_new_fixed_array(
            b"y\0" as *const u8 as *const libc::c_char as *const GVariantType,
            data
                .offset(
                    ::core::mem::size_of::<[libc::c_uchar; 5]>() as libc::c_ulong
                        as isize,
                ) as gconstpointer,
            data_len
                .wrapping_sub(
                    ::core::mem::size_of::<[libc::c_uchar; 5]>() as libc::c_ulong,
                ),
            1 as libc::c_int as gsize,
        );
        fpi_print_set_type(print, FPI_PRINT_RAW);
        g_object_set(
            print as gpointer,
            b"fpi-data\0" as *const u8 as *const libc::c_char,
            fp_data,
            0 as *mut libc::c_void,
        );
        g_object_ref(print as gpointer);
    }
    do_enroll_stop(dev, print, error);
}
unsafe extern "C" fn enroll_iterate_msg_cb(
    mut dev: *mut FpDevice,
    mut msgtype: read_msg_type,
    mut seq: guint8,
    mut subcmd: libc::c_uchar,
    mut data: *mut libc::c_uchar,
    mut data_len: size_t,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    if !error.is_null() {
        do_enroll_stop(dev, 0 as *mut FpPrint, error);
        return;
    } else {
        if msgtype as libc::c_uint != READ_MSG_RESPONSE as libc::c_int as libc::c_uint {
            g_log(
                b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"expected response, got %d seq=%x\0" as *const u8
                    as *const libc::c_char,
                msgtype as libc::c_uint,
                seq as libc::c_int,
            );
            do_enroll_stop(
                dev,
                0 as *mut FpPrint,
                fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_PROTO,
                    b"Expected message response, not command\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            return;
        }
    }
    if subcmd as libc::c_int == 0 as libc::c_int {
        e_handle_resp00(dev, data, data_len);
    } else if subcmd as libc::c_int == 2 as libc::c_int {
        e_handle_resp02(dev, data, data_len);
    } else {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"unexpected subcmd %d\0" as *const u8 as *const libc::c_char,
            subcmd as libc::c_int,
        );
        do_enroll_stop(
            dev,
            0 as *mut FpPrint,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Unexpected subcommand\0" as *const u8 as *const libc::c_char,
            ),
        );
    };
}
unsafe extern "C" fn enroll_iterate_cmd_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    if !error.is_null() {
        do_enroll_stop(device, 0 as *mut FpPrint, error);
    } else {
        read_msg_async(
            device,
            Some(
                enroll_iterate_msg_cb
                    as unsafe extern "C" fn(
                        *mut FpDevice,
                        read_msg_type,
                        guint8,
                        libc::c_uchar,
                        *mut libc::c_uchar,
                        size_t,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
    };
}
unsafe extern "C" fn enroll_iterate(mut dev: *mut FpDevice) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    if fpi_device_action_is_cancelled(dev) != 0 {
        do_enroll_stop(
            dev,
            0 as *mut FpPrint,
            g_error_new_literal(
                g_io_error_quark(),
                G_IO_ERROR_CANCELLED as libc::c_int,
                b"Cancelled\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    transfer = alloc_send_cmd28_transfer(
        dev,
        0 as libc::c_int as libc::c_uchar,
        poll_data.as_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong as guint16,
    );
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_submit(
        transfer,
        5000 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            enroll_iterate_cmd_cb
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
unsafe extern "C" fn enroll_started(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    if !error.is_null() {
        do_enroll_stop(dev, 0 as *mut FpPrint, error);
    } else {
        enroll_iterate(dev);
    };
}
unsafe extern "C" fn enroll(mut dev: *mut FpDevice) {
    let mut upekdev: *mut FpiDeviceUpekts = FPI_DEVICE_UPEKTS(dev as gpointer);
    let mut ssm: *mut FpiSsm = fpi_ssm_new_full(
        dev,
        Some(
            enroll_start_sm_run_state
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        ENROLL_START_NUM_STATES as libc::c_int,
        ENROLL_START_NUM_STATES as libc::c_int,
        b"ENROLL_START_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    (*upekdev).enroll_passed = 0 as libc::c_int;
    (*upekdev).enroll_stage = 0 as libc::c_int;
    fpi_ssm_start(
        ssm,
        Some(
            enroll_started
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn verify_stop_data_free(mut data: *mut VerifyStopData) {
    g_clear_error(&mut (*data).error);
    g_free(data as gpointer);
}
unsafe extern "C" fn verify_stop_deinit_cb(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut data: *mut VerifyStopData = fpi_ssm_get_data(ssm) as *mut VerifyStopData;
    if !error.is_null() {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Error deinitializing: %s\0" as *const u8 as *const libc::c_char,
            (*error).message,
        );
    }
    if !((*data).error).is_null() {
        fpi_device_verify_complete(
            dev,
            (if 0 as libc::c_int != 0 {
                (*data).error as *mut libc::c_void
            } else {
                g_steal_pointer(&mut (*data).error as *mut *mut GError as gpointer)
            }) as *mut GError,
        );
    } else {
        fpi_device_verify_complete(
            dev,
            (if 0 as libc::c_int != 0 {
                error as *mut libc::c_void
            } else {
                g_steal_pointer(&mut error as *mut *mut GError as gpointer)
            }) as *mut GError,
        );
    }
    g_clear_error(&mut error);
}
unsafe extern "C" fn do_verify_stop(
    mut dev: *mut FpDevice,
    mut res: FpiMatchResult,
    mut error: *mut GError,
) {
    let mut data: *mut VerifyStopData = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<VerifyStopData>() as libc::c_ulong;
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
    }) as *mut VerifyStopData;
    let mut ssm: *mut FpiSsm = deinitsm_new(dev, data as *mut libc::c_void);
    if !error.is_null() && (*error).domain == fp_device_retry_quark() {
        fpi_device_verify_report(dev, res, 0 as *mut FpPrint, error);
    } else {
        (*data).error = error;
    }
    fpi_ssm_start(
        ssm,
        Some(
            verify_stop_deinit_cb
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
    fpi_ssm_set_data(
        ssm,
        data as gpointer,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut VerifyStopData) -> ()>,
            GDestroyNotify,
        >(Some(verify_stop_data_free as unsafe extern "C" fn(*mut VerifyStopData) -> ())),
    );
}
static mut verify_hdr: [libc::c_uchar; 25] = [
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn verify_start_sm_run_state(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
) {
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    let mut fp_data: GVariant_autoptr = 0 as GVariant_autoptr;
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    let mut data_len: gsize = 0;
    let mut data: *const guint8 = 0 as *const guint8;
    let mut msg: *mut guint8 = 0 as *mut guint8;
    let mut msg_len: gsize = 0;
    let mut initsm: *mut FpiSsm = 0 as *mut FpiSsm;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            initsm = initsm_new(dev);
            fpi_ssm_start_subsm(ssm, initsm);
        }
        1 => {
            fpi_device_get_verify_data(dev, &mut print);
            g_object_get(
                print as gpointer,
                b"fpi-data\0" as *const u8 as *const libc::c_char,
                &mut fp_data as *mut GVariant_autoptr,
                0 as *mut libc::c_void,
            );
            data = g_variant_get_fixed_array(
                fp_data,
                &mut data_len,
                1 as libc::c_int as gsize,
            ) as *const guint8;
            msg_len = (::core::mem::size_of::<[libc::c_uchar; 25]>() as libc::c_ulong)
                .wrapping_add(data_len);
            msg = g_malloc(msg_len) as *mut guint8;
            memcpy(
                msg as *mut libc::c_void,
                verify_hdr.as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[libc::c_uchar; 25]>() as libc::c_ulong,
            );
            memcpy(
                msg
                    .offset(
                        ::core::mem::size_of::<[libc::c_uchar; 25]>() as libc::c_ulong
                            as isize,
                    ) as *mut libc::c_void,
                data as *const libc::c_void,
                data_len,
            );
            transfer = alloc_send_cmd28_transfer(
                dev,
                0x3 as libc::c_int as libc::c_uchar,
                data,
                data_len as guint16,
            );
            g_free(msg as gpointer);
            (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
            (*transfer).ssm = ssm;
            fpi_usb_transfer_submit(
                transfer,
                5000 as libc::c_int as guint,
                0 as *mut GCancellable,
                Some(
                    fpi_ssm_usb_transfer_cb
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
        _ => {}
    };
}
unsafe extern "C" fn v_handle_resp00(
    mut dev: *mut FpDevice,
    mut data: *mut libc::c_uchar,
    mut data_len: size_t,
) {
    let mut status: libc::c_uchar = 0;
    let mut error: *mut GError = 0 as *mut GError;
    if data_len != 14 as libc::c_int as libc::c_ulong {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"received 3001 poll response of %luu bytes?\0" as *const u8
                as *const libc::c_char,
            data_len,
        );
        error = fpi_device_error_new(FP_DEVICE_ERROR_PROTO);
    } else {
        status = *data.offset(5 as libc::c_int as isize);
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"poll result = %02x\0" as *const u8 as *const libc::c_char,
            status as libc::c_int,
        );
        let mut current_block_11: u64;
        match status as libc::c_int {
            12 => {
                current_block_11 = 15976848397966268834;
            }
            32 => {
                g_log(
                    b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"processing scan for verification\0" as *const u8
                        as *const libc::c_char,
                );
                current_block_11 = 15976848397966268834;
            }
            0 => {
                g_log(
                    b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"good image\0" as *const u8 as *const libc::c_char,
                );
                current_block_11 = 15976848397966268834;
            }
            28 => {
                current_block_11 = 704407105786348947;
            }
            11 | 35 => {
                current_block_11 = 704407105786348947;
            }
            15 => {
                error = fpi_device_retry_new(FP_DEVICE_RETRY_REMOVE_FINGER);
                current_block_11 = 15976848397966268834;
            }
            30 => {
                error = fpi_device_retry_new(FP_DEVICE_RETRY_TOO_SHORT);
                current_block_11 = 15976848397966268834;
            }
            36 => {
                error = fpi_device_retry_new(FP_DEVICE_RETRY_CENTER_FINGER);
                current_block_11 = 15976848397966268834;
            }
            _ => {
                g_log(
                    b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_CRITICAL,
                    b"unrecognised verify status code %02x\0" as *const u8
                        as *const libc::c_char,
                    status as libc::c_int,
                );
                error = fpi_device_retry_new(FP_DEVICE_RETRY_GENERAL);
                current_block_11 = 15976848397966268834;
            }
        }
        match current_block_11 {
            704407105786348947 => {
                error = fpi_device_retry_new(FP_DEVICE_RETRY_GENERAL);
            }
            _ => {}
        }
    }
    if !error.is_null() {
        do_verify_stop(dev, FPI_MATCH_ERROR, error);
    } else {
        verify_iterate(dev);
    };
}
unsafe extern "C" fn v_handle_resp03(
    mut dev: *mut FpDevice,
    mut data: *mut libc::c_uchar,
    mut data_len: size_t,
) {
    let mut r: FpiMatchResult = FPI_MATCH_FAIL;
    let mut error: *mut GError = 0 as *mut GError;
    if data_len < 2 as libc::c_int as libc::c_ulong {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"verify result abnormally short!\0" as *const u8 as *const libc::c_char,
        );
        r = FPI_MATCH_ERROR;
        error = fpi_device_error_new(FP_DEVICE_ERROR_PROTO);
    } else if *data.offset(0 as libc::c_int as isize) as libc::c_int
        != 0x12 as libc::c_int
    {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"unexpected verify header byte %02x\0" as *const u8 as *const libc::c_char,
            *data.offset(0 as libc::c_int as isize) as libc::c_int,
        );
        r = FPI_MATCH_ERROR;
        error = fpi_device_error_new(FP_DEVICE_ERROR_PROTO);
    } else if *data.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        r = FPI_MATCH_FAIL;
    } else if *data.offset(1 as libc::c_int as isize) as libc::c_int
        == 0x1 as libc::c_int
    {
        r = FPI_MATCH_SUCCESS;
    } else {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"unrecognised verify result %02x\0" as *const u8 as *const libc::c_char,
            *data.offset(1 as libc::c_int as isize) as libc::c_int,
        );
        r = FPI_MATCH_ERROR;
        error = fpi_device_error_new(FP_DEVICE_ERROR_PROTO);
    }
    do_verify_stop(dev, r, error);
}
unsafe extern "C" fn verify_rd2800_cb(
    mut dev: *mut FpDevice,
    mut msgtype: read_msg_type,
    mut seq: guint8,
    mut subcmd: libc::c_uchar,
    mut data: *mut libc::c_uchar,
    mut data_len: size_t,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut upekdev: *mut FpiDeviceUpekts = FPI_DEVICE_UPEKTS(dev as gpointer);
    if !error.is_null() {
        do_verify_stop(dev, FPI_MATCH_ERROR, error);
        return;
    }
    if msgtype as libc::c_uint != READ_MSG_RESPONSE as libc::c_int as libc::c_uint {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"expected response, got %d seq=%x\0" as *const u8 as *const libc::c_char,
            msgtype as libc::c_uint,
            seq as libc::c_int,
        );
        do_verify_stop(
            dev,
            FPI_MATCH_ERROR,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Expected message response\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    if seq as libc::c_int != (*upekdev).seq as libc::c_int {
        g_log(
            b"libfprint-upekts\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"expected response to cmd seq=%02x, got response to %02x\0" as *const u8
                as *const libc::c_char,
            (*upekdev).seq as libc::c_int,
            seq as libc::c_int,
        );
        do_verify_stop(
            dev,
            FPI_MATCH_ERROR,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Response hat wrong command sequence\0" as *const u8
                    as *const libc::c_char,
            ),
        );
        return;
    }
    if subcmd as libc::c_int == 0 as libc::c_int {
        v_handle_resp00(dev, data, data_len);
    } else if subcmd as libc::c_int == 3 as libc::c_int {
        v_handle_resp03(dev, data, data_len);
    } else {
        do_verify_stop(
            dev,
            FPI_MATCH_ERROR,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Response had wrong subcommand type\0" as *const u8
                    as *const libc::c_char,
            ),
        );
    };
}
unsafe extern "C" fn verify_wr2800_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    if !error.is_null() {
        do_verify_stop(device, FPI_MATCH_ERROR, error);
    } else {
        read_msg_async(
            device,
            Some(
                verify_rd2800_cb
                    as unsafe extern "C" fn(
                        *mut FpDevice,
                        read_msg_type,
                        guint8,
                        libc::c_uchar,
                        *mut libc::c_uchar,
                        size_t,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
    };
}
unsafe extern "C" fn verify_iterate(mut dev: *mut FpDevice) {
    let mut upekdev: *mut FpiDeviceUpekts = FPI_DEVICE_UPEKTS(dev as gpointer);
    if fpi_device_action_is_cancelled(dev) != 0 {
        do_verify_stop(
            dev,
            FPI_MATCH_ERROR,
            g_error_new_literal(
                g_io_error_quark(),
                G_IO_ERROR_CANCELLED as libc::c_int,
                b"Cancelled\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    if (*upekdev).first_verify_iteration != 0 {
        read_msg_async(
            dev,
            Some(
                verify_rd2800_cb
                    as unsafe extern "C" fn(
                        *mut FpDevice,
                        read_msg_type,
                        guint8,
                        libc::c_uchar,
                        *mut libc::c_uchar,
                        size_t,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
        (*upekdev).first_verify_iteration = 0 as libc::c_int;
    } else {
        let mut transfer: *mut FpiUsbTransfer = alloc_send_cmd28_transfer(
            dev,
            0 as libc::c_int as libc::c_uchar,
            poll_data.as_ptr(),
            ::core::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong as guint16,
        );
        (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
        fpi_usb_transfer_submit(
            transfer,
            5000 as libc::c_int as guint,
            0 as *mut GCancellable,
            Some(
                verify_wr2800_cb
                    as unsafe extern "C" fn(
                        *mut FpiUsbTransfer,
                        *mut FpDevice,
                        gpointer,
                        *mut GError,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
    };
}
unsafe extern "C" fn verify_started(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut upekdev: *mut FpiDeviceUpekts = FPI_DEVICE_UPEKTS(dev as gpointer);
    if !error.is_null() {
        do_verify_stop(dev, FPI_MATCH_ERROR, error);
        return;
    }
    (*upekdev).first_verify_iteration = (0 as libc::c_int == 0) as libc::c_int;
    verify_iterate(dev);
}
unsafe extern "C" fn verify(mut dev: *mut FpDevice) {
    let mut ssm: *mut FpiSsm = fpi_ssm_new_full(
        dev,
        Some(
            verify_start_sm_run_state
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        VERIFY_NUM_STATES as libc::c_int,
        VERIFY_NUM_STATES as libc::c_int,
        b"VERIFY_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            verify_started
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
static mut id_table: [FpIdEntry; 2] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
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
unsafe extern "C" fn fpi_device_upekts_init(mut self_0: *mut FpiDeviceUpekts) {}
unsafe extern "C" fn fpi_device_upekts_class_init(mut klass: *mut FpiDeviceUpektsClass) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    (*dev_class).id = b"upekts\0" as *const u8 as *const libc::c_char;
    (*dev_class).full_name = b"UPEK TouchStrip\0" as *const u8 as *const libc::c_char;
    (*dev_class).type_0 = FP_DEVICE_TYPE_USB;
    (*dev_class).scan_type = FP_SCAN_TYPE_SWIPE;
    (*dev_class).id_table = id_table.as_ptr();
    (*dev_class).nr_enroll_stages = 3 as libc::c_int;
    (*dev_class).open = Some(dev_init as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class).close = Some(dev_exit as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class).verify = Some(verify as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class).enroll = Some(enroll as unsafe extern "C" fn(*mut FpDevice) -> ());
    fpi_device_class_auto_initialize_features(dev_class);
}
