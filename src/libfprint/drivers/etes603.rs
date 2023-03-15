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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
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
    fn fpi_device_get_usb_device(device: *mut FpDevice) -> *mut GUsbDevice;
    fn fpi_device_error_new(error: FpDeviceError) -> *mut GError;
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
    fn fpi_usb_transfer_new(device: *mut FpDevice) -> *mut FpiUsbTransfer;
    fn fpi_usb_transfer_fill_bulk_full(
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
    fn fpi_ssm_next_state(machine: *mut FpiSsm);
    fn fpi_ssm_jump_to_state(machine: *mut FpiSsm, state: libc::c_int);
    fn fpi_ssm_mark_completed(machine: *mut FpiSsm);
    fn fpi_ssm_mark_failed(machine: *mut FpiSsm, error: *mut GError);
    fn fpi_ssm_get_cur_state(machine: *mut FpiSsm) -> libc::c_int;
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
pub struct egis_msg {
    pub magic: [guint8; 5],
    pub cmd: guint8,
    pub c2rust_unnamed: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub egis_readreg: C2RustUnnamed_11,
    pub sige_readreg: C2RustUnnamed_10,
    pub egis_writereg: C2RustUnnamed_8,
    pub egis_readf: C2RustUnnamed_7,
    pub egis_readfp: C2RustUnnamed_6,
    pub sige_misc: C2RustUnnamed_5,
    pub padding: [guint8; 58],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub val: [guint8; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub len: [guint8; 2],
    pub val: [guint8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub length_factor: guint8,
    pub length: guint8,
    pub use_gvv: guint8,
    pub gain: guint8,
    pub vrt: guint8,
    pub vrb: guint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub nb: guint8,
    pub regs: [C2RustUnnamed_9; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub reg: guint8,
    pub val: guint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub regs: [guint8; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub nb: guint8,
    pub regs: [guint8; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceEtes603 {
    pub parent: FpImageDevice,
    pub regs: [guint8; 256],
    pub req: *mut egis_msg,
    pub req_len: size_t,
    pub ans: *mut egis_msg,
    pub ans_len: size_t,
    pub fp: *mut guint8,
    pub fp_height: guint16,
    pub tunedc_min: guint8,
    pub tunedc_max: guint8,
    pub gain: guint8,
    pub dcoffset: guint8,
    pub vrt: guint8,
    pub vrb: guint8,
    pub is_active: libc::c_uint,
}
pub type FpiDeviceEtes603 = _FpiDeviceEtes603;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiDeviceEtes603Class {
    pub parent_class: FpImageDeviceClass,
}
pub const EXIT_NUM_STATES: C2RustUnnamed_17 = 2;
pub const EXIT_SET_REGS_ANS: C2RustUnnamed_17 = 1;
pub const EXIT_SET_REGS_REQ: C2RustUnnamed_17 = 0;
pub const FGR_NUM_STATES: C2RustUnnamed_15 = 14;
pub const FGR_FPA_GET_FRAME_REQ: C2RustUnnamed_15 = 12;
pub const FGR_FPA_GET_FRAME_ANS: C2RustUnnamed_15 = 13;
pub const FGR_FPA_INIT_SET_MODE_SENSOR_ANS: C2RustUnnamed_15 = 11;
pub const FGR_FPA_INIT_SET_MODE_SENSOR_REQ: C2RustUnnamed_15 = 10;
pub const FGR_FPA_INIT_SET_REG04_ANS: C2RustUnnamed_15 = 9;
pub const FGR_FPA_INIT_SET_REG04_REQ: C2RustUnnamed_15 = 8;
pub const FGR_FPA_INIT_SET_VCO_CONTROL_RT_ANS: C2RustUnnamed_15 = 7;
pub const FGR_FPA_INIT_SET_VCO_CONTROL_RT_REQ: C2RustUnnamed_15 = 6;
pub const FGR_FPA_INIT_SET_GAINVRTVRB_ANS: C2RustUnnamed_15 = 5;
pub const FGR_FPA_INIT_SET_GAINVRTVRB_REQ: C2RustUnnamed_15 = 4;
pub const FGR_FPA_INIT_SET_DCOFFSET_ANS: C2RustUnnamed_15 = 3;
pub const FGR_FPA_INIT_SET_DCOFFSET_REQ: C2RustUnnamed_15 = 2;
pub const FGR_FPA_INIT_SET_MODE_SLEEP_ANS: C2RustUnnamed_15 = 1;
pub const FGR_FPA_INIT_SET_MODE_SLEEP_REQ: C2RustUnnamed_15 = 0;
pub const CAP_NUM_STATES: C2RustUnnamed_16 = 6;
pub const CAP_FP_GET_FP_REQ: C2RustUnnamed_16 = 4;
pub const CAP_FP_GET_FP_ANS: C2RustUnnamed_16 = 5;
pub const CAP_FP_INIT_SET_MODE_FP_ANS: C2RustUnnamed_16 = 3;
pub const CAP_FP_INIT_SET_MODE_FP_REQ: C2RustUnnamed_16 = 2;
pub const CAP_FP_INIT_SET_REG10_ANS: C2RustUnnamed_16 = 1;
pub const CAP_FP_INIT_SET_REG10_REQ: C2RustUnnamed_16 = 0;
pub const TUNEVRB_NUM_STATES: C2RustUnnamed_14 = 17;
pub const TUNEVRB_FINAL_SET_MODE_SLEEP_ANS: C2RustUnnamed_14 = 16;
pub const TUNEVRB_FINAL_SET_MODE_SLEEP_REQ: C2RustUnnamed_14 = 15;
pub const TUNEVRB_FINAL_SET_GAINVRTVRB_ANS: C2RustUnnamed_14 = 14;
pub const TUNEVRB_FINAL_SET_GAINVRTVRB_REQ: C2RustUnnamed_14 = 13;
pub const TUNEVRB_FINAL_SET_REG2627_ANS: C2RustUnnamed_14 = 12;
pub const TUNEVRB_FINAL_SET_REG2627_REQ: C2RustUnnamed_14 = 11;
pub const TUNEVRB_FINAL_SET_DCOFFSET_ANS: C2RustUnnamed_14 = 10;
pub const TUNEVRB_FINAL_SET_DCOFFSET_REQ: C2RustUnnamed_14 = 9;
pub const TUNEVRB_FRAME_REQ: C2RustUnnamed_14 = 7;
pub const TUNEVRB_INIT: C2RustUnnamed_14 = 0;
pub const TUNEVRB_FRAME_ANS: C2RustUnnamed_14 = 8;
pub const TUNEVRB_SET_DCOFFSET_ANS: C2RustUnnamed_14 = 6;
pub const TUNEVRB_SET_DCOFFSET_REQ: C2RustUnnamed_14 = 5;
pub const TUNEVRB_GET_DCOFFSET_ANS: C2RustUnnamed_14 = 4;
pub const TUNEVRB_GET_DCOFFSET_REQ: C2RustUnnamed_14 = 3;
pub const TUNEVRB_GET_GAIN_ANS: C2RustUnnamed_14 = 2;
pub const TUNEVRB_GET_GAIN_REQ: C2RustUnnamed_14 = 1;
pub const TUNEDC_NUM_STATES: C2RustUnnamed_13 = 11;
pub const TUNEDC_FINAL_SET_DCOFFSET_ANS: C2RustUnnamed_13 = 10;
pub const TUNEDC_FINAL_SET_DCOFFSET_REQ: C2RustUnnamed_13 = 9;
pub const TUNEDC_FINAL_SET_GAIN_ANS: C2RustUnnamed_13 = 8;
pub const TUNEDC_FINAL_SET_GAIN_REQ: C2RustUnnamed_13 = 7;
pub const TUNEDC_FINAL_SET_REG2122_ANS: C2RustUnnamed_13 = 6;
pub const TUNEDC_FINAL_SET_REG2122_REQ: C2RustUnnamed_13 = 5;
pub const TUNEDC_SET_DCOFFSET_REQ: C2RustUnnamed_13 = 1;
pub const TUNEDC_GET_FRAME_ANS: C2RustUnnamed_13 = 4;
pub const TUNEDC_GET_FRAME_REQ: C2RustUnnamed_13 = 3;
pub const TUNEDC_SET_DCOFFSET_ANS: C2RustUnnamed_13 = 2;
pub const TUNEDC_INIT: C2RustUnnamed_13 = 0;
pub const INIT_NUM_STATES: C2RustUnnamed_12 = 12;
pub const INIT_REGS_ANS: C2RustUnnamed_12 = 11;
pub const INIT_REGS_REQ: C2RustUnnamed_12 = 10;
pub const INIT_ENC_ANS: C2RustUnnamed_12 = 9;
pub const INIT_ENC_REQ: C2RustUnnamed_12 = 8;
pub const INIT_SENSOR_ANS: C2RustUnnamed_12 = 7;
pub const INIT_SENSOR_REQ: C2RustUnnamed_12 = 6;
pub const INIT_CMD25_ANS: C2RustUnnamed_12 = 5;
pub const INIT_CMD25_REQ: C2RustUnnamed_12 = 4;
pub const INIT_CMD20_ANS: C2RustUnnamed_12 = 3;
pub const INIT_CMD20_REQ: C2RustUnnamed_12 = 2;
pub const INIT_CHECK_INFO_ANS: C2RustUnnamed_12 = 1;
pub const INIT_CHECK_INFO_REQ: C2RustUnnamed_12 = 0;
pub type C2RustUnnamed_12 = libc::c_uint;
pub type C2RustUnnamed_13 = libc::c_uint;
pub type C2RustUnnamed_14 = libc::c_uint;
pub type C2RustUnnamed_15 = libc::c_uint;
pub type C2RustUnnamed_16 = libc::c_uint;
pub type C2RustUnnamed_17 = libc::c_uint;
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
unsafe extern "C" fn FPI_DEVICE_ETES603(mut ptr: gpointer) -> *mut FpiDeviceEtes603 {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_etes603_get_type(),
    ) as *mut libc::c_void as *mut FpiDeviceEtes603;
}
unsafe extern "C" fn fpi_device_etes603_class_intern_init(mut klass: gpointer) {
    fpi_device_etes603_parent_class = g_type_class_peek_parent(klass);
    if FpiDeviceEtes603_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpiDeviceEtes603_private_offset);
    }
    fpi_device_etes603_class_init(klass as *mut FpiDeviceEtes603Class);
}
#[inline(never)]
unsafe extern "C" fn fpi_device_etes603_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_image_device_get_type(),
        g_intern_static_string(
            b"FpiDeviceEtes603\0" as *const u8 as *const libc::c_char,
        ),
        ::core::mem::size_of::<FpiDeviceEtes603Class>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_etes603_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpiDeviceEtes603>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpiDeviceEtes603) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_etes603_init
                        as unsafe extern "C" fn(*mut FpiDeviceEtes603) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_etes603_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_etes603_get_type_once();
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
static mut fpi_device_etes603_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut FpiDeviceEtes603_private_offset: gint = 0;
unsafe extern "C" fn msg_header_prepare(mut msg: *mut egis_msg) {
    (*msg).magic[0 as libc::c_int as usize] = 'E' as i32 as guint8;
    (*msg).magic[1 as libc::c_int as usize] = 'G' as i32 as guint8;
    (*msg).magic[2 as libc::c_int as usize] = 'I' as i32 as guint8;
    (*msg).magic[3 as libc::c_int as usize] = 'S' as i32 as guint8;
    (*msg).magic[4 as libc::c_int as usize] = 0x9 as libc::c_int as guint8;
}
unsafe extern "C" fn msg_header_check(mut msg: *mut egis_msg) -> libc::c_int {
    if (*msg).magic[0 as libc::c_int as usize] as libc::c_int == 'S' as i32
        && (*msg).magic[1 as libc::c_int as usize] as libc::c_int == 'I' as i32
        && (*msg).magic[2 as libc::c_int as usize] as libc::c_int == 'G' as i32
        && (*msg).magic[3 as libc::c_int as usize] as libc::c_int == 'E' as i32
        && (*msg).magic[4 as libc::c_int as usize] as libc::c_int == 0xa as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn msg_get_frame(
    mut self_0: *mut FpiDeviceEtes603,
    mut use_gvv: guint8,
    mut gain: guint8,
    mut vrt: guint8,
    mut vrb: guint8,
) {
    let mut msg: *mut egis_msg = (*self_0).req;
    msg_header_prepare(msg);
    (*msg).cmd = 0x3 as libc::c_int as guint8;
    (*msg).c2rust_unnamed.egis_readf.length_factor = 0x1 as libc::c_int as guint8;
    (*msg).c2rust_unnamed.egis_readf.length = 192 as libc::c_int as guint8;
    (*msg).c2rust_unnamed.egis_readf.use_gvv = use_gvv;
    (*msg).c2rust_unnamed.egis_readf.gain = gain;
    (*msg).c2rust_unnamed.egis_readf.vrt = vrt;
    (*msg).c2rust_unnamed.egis_readf.vrb = vrb;
    (*self_0).req_len = (6 as libc::c_int + 6 as libc::c_int) as size_t;
    (*self_0).ans_len = 384 as libc::c_int as size_t;
}
unsafe extern "C" fn msg_get_fp(
    mut self_0: *mut FpiDeviceEtes603,
    mut len0: guint8,
    mut len1: guint8,
    mut v2: guint8,
    mut v3: guint8,
    mut v4: guint8,
) {
    let mut msg: *mut egis_msg = (*self_0).req;
    msg_header_prepare(msg);
    (*msg).cmd = 0x6 as libc::c_int as guint8;
    (*msg).c2rust_unnamed.egis_readfp.len[0 as libc::c_int as usize] = len0;
    (*msg).c2rust_unnamed.egis_readfp.len[1 as libc::c_int as usize] = len1;
    (*msg).c2rust_unnamed.egis_readfp.val[0 as libc::c_int as usize] = v2;
    (*msg).c2rust_unnamed.egis_readfp.val[1 as libc::c_int as usize] = v3;
    (*msg).c2rust_unnamed.egis_readfp.val[2 as libc::c_int as usize] = v4;
    (*self_0).req_len = (6 as libc::c_int + 5 as libc::c_int) as size_t;
    (*self_0).ans_len = 64000 as libc::c_int as size_t;
}
unsafe extern "C" fn msg_get_regs(
    mut self_0: *mut FpiDeviceEtes603,
    mut n_args: libc::c_int,
    mut args: ...
) {
    let mut msg: *mut egis_msg = (*self_0).req;
    let mut ap: ::core::ffi::VaListImpl;
    let mut i: libc::c_int = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if n_args > 0 as libc::c_int && n_args <= 0x18 as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/etes603.c\0" as *const u8 as *const libc::c_char,
            317 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"msg_get_regs\0"))
                .as_ptr(),
            b"n_args > 0 && n_args <= REG_MAX\0" as *const u8 as *const libc::c_char,
        );
    }
    msg_header_prepare(msg);
    (*msg).cmd = 0x1 as libc::c_int as guint8;
    (*msg).c2rust_unnamed.egis_readreg.nb = n_args as guint8;
    ap = args.clone();
    i = 0 as libc::c_int;
    while i < n_args {
        (*msg)
            .c2rust_unnamed
            .egis_readreg
            .regs[i as usize] = ap.arg::<libc::c_int>() as guint8;
        i += 1;
    }
    (*self_0).req_len = (6 as libc::c_int + 1 as libc::c_int + n_args) as size_t;
    (*self_0).ans_len = (6 as libc::c_int + 1 as libc::c_int + n_args) as size_t;
}
unsafe extern "C" fn msg_parse_regs(mut dev: *mut FpiDeviceEtes603) -> libc::c_int {
    let mut i: size_t = 0;
    let mut n_args: size_t = 0;
    let mut msg_req: *mut egis_msg = (*dev).req;
    let mut msg_ans: *mut egis_msg = (*dev).ans;
    n_args = ((*dev).ans_len).wrapping_sub(6 as libc::c_int as libc::c_ulong);
    if msg_header_check(msg_ans) != 0 {
        return -(1 as libc::c_int);
    }
    if (*msg_ans).cmd as libc::c_int != 0x1 as libc::c_int {
        return -(2 as libc::c_int);
    }
    i = 0 as libc::c_int as size_t;
    while i < n_args {
        let mut reg: libc::c_int = (*msg_req)
            .c2rust_unnamed
            .egis_readreg
            .regs[i as usize] as libc::c_int;
        (*dev)
            .regs[reg
            as usize] = (*msg_ans).c2rust_unnamed.sige_readreg.regs[i as usize];
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn msg_set_regs(
    mut self_0: *mut FpiDeviceEtes603,
    mut n_args: libc::c_int,
    mut args: ...
) {
    let mut msg: *mut egis_msg = (*self_0).req;
    let mut ap: ::core::ffi::VaListImpl;
    let mut i: libc::c_int = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if n_args != 0 as libc::c_int && n_args % 2 as libc::c_int == 0 as libc::c_int
            && n_args <= 0x18 as libc::c_int * 2 as libc::c_int
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/etes603.c\0" as *const u8 as *const libc::c_char,
            367 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"msg_set_regs\0"))
                .as_ptr(),
            b"n_args != 0 && n_args % 2 == 0 && n_args <= REG_MAX * 2\0" as *const u8
                as *const libc::c_char,
        );
    }
    msg_header_prepare(msg);
    (*msg).cmd = 0x2 as libc::c_int as guint8;
    (*msg).c2rust_unnamed.egis_writereg.nb = (n_args / 2 as libc::c_int) as guint8;
    ap = args.clone();
    i = 0 as libc::c_int;
    while i < n_args / 2 as libc::c_int {
        (*msg)
            .c2rust_unnamed
            .egis_writereg
            .regs[i as usize]
            .reg = ap.arg::<libc::c_int>() as guint8;
        (*msg)
            .c2rust_unnamed
            .egis_writereg
            .regs[i as usize]
            .val = ap.arg::<libc::c_int>() as guint8;
        i += 1;
    }
    (*self_0).req_len = (6 as libc::c_int + 1 as libc::c_int + n_args) as size_t;
    (*self_0).ans_len = (6 as libc::c_int + 1 as libc::c_int) as size_t;
}
unsafe extern "C" fn msg_check_ok(mut dev: *mut FpiDeviceEtes603) -> libc::c_int {
    let mut msg: *mut egis_msg = (*dev).ans;
    if !(msg_header_check(msg) != 0) {
        if !((*msg).cmd as libc::c_int != 0x1 as libc::c_int) {
            return 0 as libc::c_int;
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn check_info(mut dev: *mut FpiDeviceEtes603) -> libc::c_int {
    if (*dev).regs[0x70 as libc::c_int as usize] as libc::c_int == 0x4a as libc::c_int
        && (*dev).regs[0x71 as libc::c_int as usize] as libc::c_int
            == 0x44 as libc::c_int
        && (*dev).regs[0x72 as libc::c_int as usize] as libc::c_int
            == 0x49 as libc::c_int
        && (*dev).regs[0x73 as libc::c_int as usize] as libc::c_int
            == 0x31 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    g_log(
        b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_CRITICAL,
        b"unknown device parameters (REG_70:%02X REG_71:%02X REG_FIRMWARE:%02X REG_VERSION:%02X)\0"
            as *const u8 as *const libc::c_char,
        (*dev).regs[0x70 as libc::c_int as usize] as libc::c_int,
        (*dev).regs[0x71 as libc::c_int as usize] as libc::c_int,
        (*dev).regs[0x72 as libc::c_int as usize] as libc::c_int,
        (*dev).regs[0x73 as libc::c_int as usize] as libc::c_int,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn msg_get_cmd20(mut dev: *mut FpiDeviceEtes603) {
    let mut msg: *mut egis_msg = (*dev).req;
    msg_header_prepare(msg);
    (*msg).cmd = 0x20 as libc::c_int as guint8;
    (*dev).req_len = 6 as libc::c_int as size_t;
}
unsafe extern "C" fn msg_check_cmd20(mut dev: *mut FpiDeviceEtes603) -> libc::c_int {
    let mut msg: *mut egis_msg = (*dev).ans;
    if msg_header_check(msg) != 0 {
        g_log(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"msg_header_check failed\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*msg).cmd as libc::c_int != 0x5 as libc::c_int
        || (*msg).c2rust_unnamed.sige_misc.val[0 as libc::c_int as usize] as libc::c_int
            != 0 as libc::c_int
        || (*msg).c2rust_unnamed.sige_misc.val[1 as libc::c_int as usize] as libc::c_int
            != 0 as libc::c_int
    {
        g_log(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"unexpected answer CMD_20 from device(%02X %02X %02X)\0" as *const u8
                as *const libc::c_char,
            (*msg).cmd as libc::c_int,
            (*msg).c2rust_unnamed.sige_misc.val[0 as libc::c_int as usize]
                as libc::c_int,
            (*msg).c2rust_unnamed.sige_misc.val[1 as libc::c_int as usize] as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn msg_get_cmd25(mut dev: *mut FpiDeviceEtes603) {
    let mut msg: *mut egis_msg = (*dev).req;
    msg_header_prepare(msg);
    (*msg).cmd = 0x25 as libc::c_int as guint8;
    (*dev).req_len = 6 as libc::c_int as size_t;
}
unsafe extern "C" fn msg_check_cmd25(mut dev: *mut FpiDeviceEtes603) -> libc::c_int {
    let mut msg: *mut egis_msg = (*dev).ans;
    if msg_header_check(msg) != 0 {
        g_log(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"msg_header_check failed\0" as *const u8 as *const libc::c_char,
        );
    } else if (*msg).cmd as libc::c_int != 0x1 as libc::c_int {
        g_log(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"CMD_OK failed\0" as *const u8 as *const libc::c_char,
        );
    } else {
        if (*msg).c2rust_unnamed.sige_misc.val[0 as libc::c_int as usize] as libc::c_int
            != 0 as libc::c_int
        {
            g_log(
                b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"unexpected answer for CMD_25 (%02X)\0" as *const u8
                    as *const libc::c_char,
                (*msg).c2rust_unnamed.sige_misc.val[0 as libc::c_int as usize]
                    as libc::c_int,
            );
        }
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn msg_set_mode_control(
    mut self_0: *mut FpiDeviceEtes603,
    mut mode: guint8,
) {
    msg_set_regs(self_0, 2 as libc::c_int, 0x2 as libc::c_int, mode as libc::c_int);
}
unsafe extern "C" fn process_get_brightness(
    mut f: *mut guint8,
    mut s: size_t,
) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    let mut sum: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < s {
        sum = sum
            .wrapping_add(
                (*f.offset(i as isize) as libc::c_int >> 4 as libc::c_int)
                    as libc::c_uint,
            );
        sum = sum
            .wrapping_add(
                (*f.offset(i as isize) as libc::c_int & 0xf as libc::c_int)
                    as libc::c_uint,
            );
        i = i.wrapping_add(1);
    }
    return sum;
}
unsafe extern "C" fn process_hist(
    mut f: *mut guint8,
    mut s: size_t,
    mut stat: *mut libc::c_float,
) {
    let mut hist: [libc::c_float; 16] = [0.; 16];
    let mut black_mean: libc::c_float = 0.;
    let mut white_mean: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        hist[i as usize] = 0.0f64 as libc::c_float;
        i += 1;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < s {
        hist[(*f.offset(i as isize) as libc::c_int >> 4 as libc::c_int) as usize] += 1.;
        hist[(*f.offset(i as isize) as libc::c_int & 0xf as libc::c_int) as usize] += 1.;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        hist[i
            as usize] = hist[i as usize]
            / s.wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_float;
        i += 1;
    }
    white_mean = 0.0f64 as libc::c_float;
    black_mean = white_mean;
    i = 1 as libc::c_int;
    while i < 8 as libc::c_int {
        black_mean += hist[i as usize];
        i += 1;
    }
    i = 8 as libc::c_int;
    while i < 15 as libc::c_int {
        white_mean += hist[i as usize];
        i += 1;
    }
    *stat.offset(0 as libc::c_int as isize) = hist[0 as libc::c_int as usize];
    *stat.offset(1 as libc::c_int as isize) = black_mean;
    *stat.offset(2 as libc::c_int as isize) = black_mean + white_mean;
    *stat.offset(3 as libc::c_int as isize) = white_mean;
    *stat.offset(4 as libc::c_int as isize) = hist[15 as libc::c_int as usize];
    g_log(
        b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"fullb=%6f black=%6f grey=%6f white=%6f fullw=%6f\0" as *const u8
            as *const libc::c_char,
        hist[0 as libc::c_int as usize] as libc::c_double,
        black_mean as libc::c_double,
        (black_mean + white_mean) as libc::c_double,
        white_mean as libc::c_double,
        hist[15 as libc::c_int as usize] as libc::c_double,
    );
}
unsafe extern "C" fn process_frame_empty(
    mut frame: *mut guint8,
    mut size: size_t,
) -> libc::c_int {
    let mut sum: libc::c_uint = process_get_brightness(frame, size);
    if (sum as libc::c_ulong) < size {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn process_4to8_bpp(
    mut input: *mut guint8,
    mut input_size: libc::c_uint,
    mut output: *mut guint8,
) {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < input_size {
        *output
            .offset(
                j as isize,
            ) = (*input.offset(i as isize) as libc::c_int & 0xf0 as libc::c_int)
            as guint8;
        *output
            .offset(
                j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) = ((*input.offset(i as isize) as libc::c_int) << 4 as libc::c_int)
            as guint8;
        i = i.wrapping_add(1);
        j = j.wrapping_add(2 as libc::c_int as libc::c_uint);
    }
}
unsafe extern "C" fn process_removefpi_end(mut dev: *mut FpiDeviceEtes603) {
    let mut i: libc::c_uint = 0;
    let mut pattern: *mut guint8 = ((*dev).fp)
        .offset(
            (((*dev).fp_height as libc::c_int - 2 as libc::c_int) * 256 as libc::c_int
                / 2 as libc::c_int) as isize,
        );
    i = 2 as libc::c_int as libc::c_uint;
    while i < (*dev).fp_height as libc::c_uint {
        if memcmp(
            pattern as *const libc::c_void,
            pattern
                .offset(
                    -(i
                        .wrapping_mul(256 as libc::c_int as libc::c_uint)
                        .wrapping_div(2 as libc::c_int as libc::c_uint) as isize),
                ) as *const libc::c_void,
            256 as libc::c_int as libc::c_ulong,
        ) != 0
        {
            break;
        }
        i = i.wrapping_add(2 as libc::c_int as libc::c_uint);
    }
    (*dev)
        .fp_height = ((*dev).fp_height as libc::c_uint).wrapping_sub(i) as guint16
        as guint16;
    g_log(
        b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Removing %d empty lines from image\0" as *const u8 as *const libc::c_char,
        i.wrapping_sub(2 as libc::c_int as libc::c_uint),
    );
}
unsafe extern "C" fn reset_param(mut dev: *mut FpiDeviceEtes603) {
    (*dev).dcoffset = 0 as libc::c_int as guint8;
    (*dev).vrt = 0 as libc::c_int as guint8;
    (*dev).vrb = 0 as libc::c_int as guint8;
    (*dev).gain = 0 as libc::c_int as guint8;
}
unsafe extern "C" fn async_tx(
    mut dev: *mut FpDevice,
    mut ep: libc::c_uint,
    mut cb: *mut libc::c_void,
    mut ssm: *mut FpiSsm,
) {
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(dev as gpointer);
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(dev);
    let mut buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut length: libc::c_int = 0;
    if ep == 0x2 as libc::c_int as libc::c_uint {
        buffer = (*self_0).req as *mut libc::c_uchar;
        length = (*self_0).req_len as libc::c_int;
    } else if ep == 0x81 as libc::c_int as libc::c_uint {
        buffer = (*self_0).ans as *mut libc::c_uchar;
        length = (*self_0).ans_len as libc::c_int;
    } else {
        g_assertion_message_expr(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/etes603.c\0" as *const u8 as *const libc::c_char,
            707 as libc::c_int,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"async_tx\0"))
                .as_ptr(),
            0 as *const libc::c_char,
        );
    }
    (*transfer).ssm = ssm;
    fpi_usb_transfer_fill_bulk_full(
        transfer,
        ep as guint8,
        buffer,
        length as gsize,
        None,
    );
    fpi_usb_transfer_submit(
        transfer,
        1000 as libc::c_int as guint,
        0 as *mut GCancellable,
        ::core::mem::transmute::<*mut libc::c_void, FpiUsbTransferCallback>(cb),
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn async_tx_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut idev: *mut FpImageDevice = FP_IMAGE_DEVICE(device as gpointer);
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(idev as gpointer);
    if !error.is_null() {
        g_log(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"transfer is not completed (result: %s)\0" as *const u8
                as *const libc::c_char,
            (*error).message,
        );
        fpi_ssm_mark_failed((*transfer).ssm, error);
    } else {
        let mut endpoint: libc::c_uchar = (*transfer).endpoint;
        let mut actual_length: libc::c_int = (*transfer).actual_length as libc::c_int;
        let mut length: libc::c_int = (*transfer).length as libc::c_int;
        if endpoint as libc::c_int == 0x2 as libc::c_int {
            if length != actual_length {
                g_log(
                    b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_WARNING,
                    b"length %d != actual_length %d\0" as *const u8
                        as *const libc::c_char,
                    length,
                    actual_length,
                );
            }
            async_tx(
                device,
                0x81 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                (*transfer).ssm,
            );
        } else if endpoint as libc::c_int == 0x81 as libc::c_int {
            (*self_0).ans_len = actual_length as size_t;
            fpi_ssm_next_state((*transfer).ssm);
        }
    };
}
unsafe extern "C" fn m_exit_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            msg_set_regs(
                self_0,
                4 as libc::c_int,
                0xe5 as libc::c_int,
                0x13 as libc::c_int,
                0x2 as libc::c_int,
                0x30 as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
        }
        1 => {
            if msg_check_ok(self_0) != 0 {
                fpi_ssm_mark_failed(ssm, fpi_device_error_new(FP_DEVICE_ERROR_PROTO));
                return;
            } else {
                fpi_ssm_mark_completed(ssm);
            }
        }
        _ => {
            g_assertion_message_expr(
                b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                b"../libfprint/drivers/etes603.c\0" as *const u8 as *const libc::c_char,
                771 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"m_exit_state\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn m_exit_complete(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut idev: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    if !error.is_null() {
        g_log(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"Error switching the device to idle state\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        g_log(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"The device is now in idle state\0" as *const u8 as *const libc::c_char,
        );
    }
    fpi_image_device_deactivate_complete(idev, error);
}
unsafe extern "C" fn m_exit_start(mut idev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(idev as gpointer);
    let mut ssm: *mut FpiSsm = fpi_ssm_new_full(
        FP_DEVICE(idev as gpointer),
        Some(m_exit_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
        EXIT_NUM_STATES as libc::c_int,
        EXIT_NUM_STATES as libc::c_int,
        b"EXIT_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    (*self_0).is_active = 0 as libc::c_int as libc::c_uint;
    g_log(
        b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Switching device to idle mode\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            m_exit_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn m_capture_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut current_block: u64;
    let mut idev: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(dev as gpointer);
    if (*self_0).is_active == 0 as libc::c_int as libc::c_uint {
        fpi_ssm_mark_completed(ssm);
        return;
    }
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            g_log(
                b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Capturing a fingerprint...\0" as *const u8 as *const libc::c_char,
            );
            memset(
                (*self_0).fp as *mut libc::c_void,
                0 as libc::c_int,
                (64000 as libc::c_int * 2 as libc::c_int) as libc::c_ulong,
            );
            (*self_0).fp_height = 0 as libc::c_int as guint16;
            msg_set_regs(
                self_0,
                2 as libc::c_int,
                0x10 as libc::c_int,
                0x92 as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 1836292691772056875;
        }
        1 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 1239119255275150084;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 1836292691772056875;
            }
        }
        2 => {
            msg_set_mode_control(self_0, 0x34 as libc::c_int as guint8);
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 1836292691772056875;
        }
        3 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 1239119255275150084;
            } else {
                g_log(
                    b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"Capturing a 1st frame...\0" as *const u8 as *const libc::c_char,
                );
                fpi_ssm_next_state(ssm);
                current_block = 1836292691772056875;
            }
        }
        4 => {
            msg_get_fp(
                self_0,
                0x1 as libc::c_int as guint8,
                0xf4 as libc::c_int as guint8,
                0x2 as libc::c_int as guint8,
                0x1 as libc::c_int as guint8,
                0x64 as libc::c_int as guint8,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 1836292691772056875;
        }
        5 => {
            memcpy(
                ((*self_0).fp)
                    .offset(
                        ((*self_0).fp_height as libc::c_int * 256 as libc::c_int
                            / 2 as libc::c_int) as isize,
                    ) as *mut libc::c_void,
                (*self_0).ans as *const libc::c_void,
                64000 as libc::c_int as libc::c_ulong,
            );
            (*self_0)
                .fp_height = ((*self_0).fp_height as libc::c_int + 500 as libc::c_int)
                as guint16;
            if (*self_0).fp_height as libc::c_int <= 500 as libc::c_int {
                (*self_0)
                    .fp_height = ((*self_0).fp_height as libc::c_int - 2 as libc::c_int)
                    as guint16;
                g_log(
                    b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"Capturing a 2nd frame...\0" as *const u8 as *const libc::c_char,
                );
                fpi_ssm_jump_to_state(ssm, CAP_FP_GET_FP_REQ as libc::c_int);
            } else {
                process_removefpi_end(self_0);
                process_removefpi_end(self_0);
                if (*self_0).fp_height as libc::c_int >= 256 as libc::c_int {
                    let mut img: *mut FpImage = fp_image_new(
                        256 as libc::c_int,
                        (*self_0).fp_height as gint,
                    );
                    let mut img_size: libc::c_uint = ((*self_0).fp_height as libc::c_int
                        * 256 as libc::c_int) as libc::c_uint;
                    (*img)
                        .flags = (FPI_IMAGE_COLORS_INVERTED as libc::c_int
                        | FPI_IMAGE_V_FLIPPED as libc::c_int) as FpiImageFlags;
                    (*img).height = (*self_0).fp_height as guint;
                    process_4to8_bpp(
                        (*self_0).fp,
                        img_size.wrapping_div(2 as libc::c_int as libc::c_uint),
                        (*img).data,
                    );
                    g_log(
                        b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"Sending the raw fingerprint image (%dx%d)\0" as *const u8
                            as *const libc::c_char,
                        (*img).width,
                        (*img).height,
                    );
                    fpi_image_device_image_captured(idev, img);
                } else {
                    fpi_image_device_retry_scan(idev, FP_DEVICE_RETRY_TOO_SHORT);
                }
                fpi_image_device_report_finger_status(idev, 0 as libc::c_int);
                fpi_ssm_mark_completed(ssm);
            }
            current_block = 1836292691772056875;
        }
        _ => {
            g_assertion_message_expr(
                b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                b"../libfprint/drivers/etes603.c\0" as *const u8 as *const libc::c_char,
                891 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"m_capture_state\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    }
    match current_block {
        1836292691772056875 => return,
        _ => {
            fpi_ssm_mark_failed(ssm, fpi_device_error_new(FP_DEVICE_ERROR_PROTO));
            return;
        }
    };
}
unsafe extern "C" fn m_capture_complete(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut idev: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(dev as gpointer);
    if !error.is_null() {
        if (*self_0).is_active != 0 {
            g_log(
                b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"Error while capturing fingerprint (%s)\0" as *const u8
                    as *const libc::c_char,
                (*error).message,
            );
            fpi_image_device_session_error(idev, error);
        } else {
            g_error_free(error);
        }
    }
    if (*self_0).is_active == (0 as libc::c_int == 0) as libc::c_int as libc::c_uint {
        g_log(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Device is still active, restarting finger detection\0" as *const u8
                as *const libc::c_char,
        );
        m_start_fingerdetect(idev);
    } else {
        g_log(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"And it's over.\0" as *const u8 as *const libc::c_char,
        );
        m_exit_start(idev);
    };
}
unsafe extern "C" fn m_finger_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut current_block: u64;
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(dev as gpointer);
    if (*self_0).is_active == 0 as libc::c_int as libc::c_uint {
        fpi_ssm_mark_completed(ssm);
        return;
    }
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            msg_set_mode_control(self_0, 0x30 as libc::c_int as guint8);
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 7226443171521532240;
        }
        1 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 6306441780062123745;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 7226443171521532240;
            }
        }
        2 => {
            msg_set_regs(
                self_0,
                2 as libc::c_int,
                0xe6 as libc::c_int,
                (*self_0).dcoffset as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 7226443171521532240;
        }
        3 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 6306441780062123745;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 7226443171521532240;
            }
        }
        4 => {
            msg_set_regs(
                self_0,
                6 as libc::c_int,
                0xe0 as libc::c_int,
                (*self_0).gain as libc::c_int,
                0xe1 as libc::c_int,
                (*self_0).vrt as libc::c_int,
                0xe2 as libc::c_int,
                (*self_0).vrb as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 7226443171521532240;
        }
        5 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 6306441780062123745;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 7226443171521532240;
            }
        }
        6 => {
            msg_set_regs(
                self_0,
                2 as libc::c_int,
                0xe5 as libc::c_int,
                0x14 as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 7226443171521532240;
        }
        7 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 6306441780062123745;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 7226443171521532240;
            }
        }
        8 => {
            msg_set_regs(self_0, 2 as libc::c_int, 0x4 as libc::c_int, 0 as libc::c_int);
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 7226443171521532240;
        }
        9 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 6306441780062123745;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 7226443171521532240;
            }
        }
        10 => {
            msg_set_mode_control(self_0, 0x33 as libc::c_int as guint8);
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 7226443171521532240;
        }
        11 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 6306441780062123745;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 7226443171521532240;
            }
        }
        12 => {
            msg_get_frame(
                self_0,
                0 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 7226443171521532240;
        }
        13 => {
            if process_frame_empty(
                (*self_0).ans as *mut guint8,
                384 as libc::c_int as size_t,
            ) != 0
            {
                fpi_ssm_jump_to_state(ssm, FGR_FPA_GET_FRAME_REQ as libc::c_int);
            } else {
                fpi_image_device_report_finger_status(
                    FP_IMAGE_DEVICE(dev as gpointer),
                    (0 as libc::c_int == 0) as libc::c_int,
                );
                fpi_ssm_mark_completed(ssm);
            }
            current_block = 7226443171521532240;
        }
        _ => {
            g_assertion_message_expr(
                b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                b"../libfprint/drivers/etes603.c\0" as *const u8 as *const libc::c_char,
                1031 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 15],
                    &[libc::c_char; 15],
                >(b"m_finger_state\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    }
    match current_block {
        7226443171521532240 => return,
        _ => {
            fpi_ssm_mark_failed(ssm, fpi_device_error_new(FP_DEVICE_ERROR_PROTO));
            return;
        }
    };
}
unsafe extern "C" fn m_finger_complete(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut idev: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(dev as gpointer);
    if error.is_null() {
        let mut ssm_cap: *mut FpiSsm = 0 as *mut FpiSsm;
        ssm_cap = fpi_ssm_new_full(
            dev,
            Some(
                m_capture_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
            ),
            CAP_NUM_STATES as libc::c_int,
            CAP_NUM_STATES as libc::c_int,
            b"CAP_NUM_STATES\0" as *const u8 as *const libc::c_char,
        );
        fpi_ssm_start(
            ssm_cap,
            Some(
                m_capture_complete
                    as unsafe extern "C" fn(
                        *mut FpiSsm,
                        *mut FpDevice,
                        *mut GError,
                    ) -> (),
            ),
        );
    } else {
        if (*self_0).is_active != 0 {
            g_log(
                b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"Error while capturing fingerprint (%s)\0" as *const u8
                    as *const libc::c_char,
                (*error).message,
            );
            fpi_image_device_session_error(idev, error);
        } else {
            m_exit_start(idev);
            g_error_free(error);
        }
        (*self_0).is_active = 0 as libc::c_int as libc::c_uint;
    };
}
unsafe extern "C" fn m_start_fingerdetect(mut idev: *mut FpImageDevice) {
    let mut ssmf: *mut FpiSsm = 0 as *mut FpiSsm;
    ssmf = fpi_ssm_new_full(
        FP_DEVICE(idev as gpointer),
        Some(m_finger_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
        FGR_NUM_STATES as libc::c_int,
        FGR_NUM_STATES as libc::c_int,
        b"FGR_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssmf,
        Some(
            m_finger_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn m_tunevrb_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut current_block: u64;
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(dev as gpointer);
    let mut hist: [libc::c_float; 5] = [0.; 5];
    if (*self_0).is_active == 0 as libc::c_int as libc::c_uint {
        fpi_ssm_mark_completed(ssm);
        return;
    }
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            g_log(
                b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Tuning of VRT/VRB\0" as *const u8 as *const libc::c_char,
            );
            if ({
                let mut _g_boolean_var_: libc::c_int = 0;
                if (*self_0).dcoffset != 0 {
                    _g_boolean_var_ = 1 as libc::c_int;
                } else {
                    _g_boolean_var_ = 0 as libc::c_int;
                }
                _g_boolean_var_
            }) as libc::c_long != 0
            {} else {
                g_assertion_message_expr(
                    b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                    b"../libfprint/drivers/etes603.c\0" as *const u8
                        as *const libc::c_char,
                    1098 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 16],
                        &[libc::c_char; 16],
                    >(b"m_tunevrb_state\0"))
                        .as_ptr(),
                    b"self->dcoffset\0" as *const u8 as *const libc::c_char,
                );
            }
            (*self_0).vrt = 0xa as libc::c_int as guint8;
            (*self_0).vrb = 0x10 as libc::c_int as guint8;
            fpi_ssm_next_state(ssm);
            current_block = 15462640364611497761;
        }
        1 => {
            msg_get_regs(self_0, 1 as libc::c_int, 0xe0 as libc::c_int);
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 15462640364611497761;
        }
        2 => {
            if msg_parse_regs(self_0) != 0 {
                current_block = 10663290743669596359;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 15462640364611497761;
            }
        }
        3 => {
            msg_get_regs(self_0, 1 as libc::c_int, 0xe6 as libc::c_int);
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 15462640364611497761;
        }
        4 => {
            if msg_parse_regs(self_0) != 0 {
                current_block = 10663290743669596359;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 15462640364611497761;
            }
        }
        5 => {
            msg_set_regs(
                self_0,
                2 as libc::c_int,
                0xe6 as libc::c_int,
                (*self_0).dcoffset as libc::c_int - 1 as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 15462640364611497761;
        }
        6 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 10663290743669596359;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 15462640364611497761;
            }
        }
        7 => {
            g_log(
                b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Testing VRT=0x%02X VRB=0x%02X\0" as *const u8 as *const libc::c_char,
                (*self_0).vrt as libc::c_int,
                (*self_0).vrb as libc::c_int,
            );
            msg_get_frame(
                self_0,
                0x1 as libc::c_int as guint8,
                (*self_0).gain,
                (*self_0).vrt,
                (*self_0).vrb,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 15462640364611497761;
        }
        8 => {
            process_hist(
                (*self_0).ans as *mut guint8,
                384 as libc::c_int as size_t,
                hist.as_mut_ptr(),
            );
            if (hist[0 as libc::c_int as usize] + hist[1 as libc::c_int as usize])
                as libc::c_double > 0.95f64
            {
                if (*self_0).vrt as libc::c_int <= 0 as libc::c_int
                    || (*self_0).vrb as libc::c_int <= 0 as libc::c_int
                {
                    g_log(
                        b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"Image is too dark, reducing DCOffset\0" as *const u8
                            as *const libc::c_char,
                    );
                    (*self_0).dcoffset = ((*self_0).dcoffset).wrapping_sub(1);
                    fpi_ssm_jump_to_state(ssm, TUNEVRB_INIT as libc::c_int);
                } else {
                    (*self_0).vrt = ((*self_0).vrt).wrapping_sub(1);
                    (*self_0).vrb = ((*self_0).vrb).wrapping_sub(1);
                    fpi_ssm_jump_to_state(ssm, TUNEVRB_FRAME_REQ as libc::c_int);
                }
            } else if hist[4 as libc::c_int as usize] as libc::c_double > 0.95f64 {
                g_log(
                    b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"Image is too bright, increasing DCOffset\0" as *const u8
                        as *const libc::c_char,
                );
                (*self_0).dcoffset = ((*self_0).dcoffset).wrapping_add(1);
                fpi_ssm_jump_to_state(ssm, TUNEVRB_INIT as libc::c_int);
            } else if (hist[4 as libc::c_int as usize] + hist[3 as libc::c_int as usize])
                as libc::c_double > 0.4f64
            {
                if (*self_0).vrt as libc::c_int
                    >= 2 as libc::c_int * (*self_0).vrb as libc::c_int
                        - 0xa as libc::c_int
                {
                    (*self_0).vrt = ((*self_0).vrt).wrapping_add(1);
                    (*self_0).vrb = ((*self_0).vrb).wrapping_add(1);
                } else {
                    (*self_0).vrt = ((*self_0).vrt).wrapping_add(1);
                }
                if (*self_0).vrt as libc::c_int > 0x3f as libc::c_int {
                    (*self_0).vrt = 0x3f as libc::c_int as guint8;
                }
                if (*self_0).vrb as libc::c_int > 0x3a as libc::c_int {
                    (*self_0).vrb = 0x3a as libc::c_int as guint8;
                }
                fpi_ssm_jump_to_state(ssm, TUNEVRB_FRAME_REQ as libc::c_int);
            } else {
                fpi_ssm_next_state(ssm);
            }
            current_block = 15462640364611497761;
        }
        9 => {
            g_log(
                b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"-> VRT=0x%02X VRB=0x%02X\0" as *const u8 as *const libc::c_char,
                (*self_0).vrt as libc::c_int,
                (*self_0).vrb as libc::c_int,
            );
            msg_set_regs(
                self_0,
                2 as libc::c_int,
                0xe6 as libc::c_int,
                (*self_0).dcoffset as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 15462640364611497761;
        }
        10 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 10663290743669596359;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 15462640364611497761;
            }
        }
        11 => {
            msg_set_regs(
                self_0,
                4 as libc::c_int,
                0x26 as libc::c_int,
                0x11 as libc::c_int,
                0x27 as libc::c_int,
                0 as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 15462640364611497761;
        }
        12 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 10663290743669596359;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 15462640364611497761;
            }
        }
        13 => {
            msg_set_regs(
                self_0,
                6 as libc::c_int,
                0xe0 as libc::c_int,
                (*self_0).gain as libc::c_int,
                0xe1 as libc::c_int,
                (*self_0).vrt as libc::c_int,
                0xe2 as libc::c_int,
                (*self_0).vrb as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 15462640364611497761;
        }
        14 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 10663290743669596359;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 15462640364611497761;
            }
        }
        15 => {
            msg_set_mode_control(self_0, 0x30 as libc::c_int as guint8);
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 15462640364611497761;
        }
        16 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 10663290743669596359;
            } else {
                fpi_ssm_mark_completed(ssm);
                current_block = 15462640364611497761;
            }
        }
        _ => {
            g_assertion_message_expr(
                b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                b"../libfprint/drivers/etes603.c\0" as *const u8 as *const libc::c_char,
                1246 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"m_tunevrb_state\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    }
    match current_block {
        15462640364611497761 => return,
        _ => {
            fpi_ssm_mark_failed(ssm, fpi_device_error_new(FP_DEVICE_ERROR_PROTO));
            return;
        }
    };
}
unsafe extern "C" fn m_tunevrb_complete(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(dev as gpointer);
    let mut idev: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    fpi_image_device_activate_complete(idev, error);
    if error.is_null() {
        g_log(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Tuning is done. Starting finger detection.\0" as *const u8
                as *const libc::c_char,
        );
        m_start_fingerdetect(idev);
    }
    if (*self_0).is_active == 0 {
        m_exit_start(idev);
    }
}
unsafe extern "C" fn m_tunedc_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut current_block: u64;
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(dev as gpointer);
    if (*self_0).is_active == 0 as libc::c_int as libc::c_uint {
        fpi_ssm_mark_completed(ssm);
        return;
    }
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            (*self_0).gain = 0x23 as libc::c_int as guint8;
            (*self_0).tunedc_min = 0 as libc::c_int as guint8;
            (*self_0).tunedc_max = 0x35 as libc::c_int as guint8;
            g_log(
                b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Tuning DCoffset\0" as *const u8 as *const libc::c_char,
            );
            fpi_ssm_next_state(ssm);
            current_block = 12199444798915819164;
        }
        1 => {
            (*self_0)
                .dcoffset = (((*self_0).tunedc_max as libc::c_int
                + (*self_0).tunedc_min as libc::c_int) / 2 as libc::c_int) as guint8;
            g_log(
                b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Testing DCoffset=0x%02X Gain=0x%02X\0" as *const u8
                    as *const libc::c_char,
                (*self_0).dcoffset as libc::c_int,
                (*self_0).gain as libc::c_int,
            );
            msg_set_regs(
                self_0,
                2 as libc::c_int,
                0xe6 as libc::c_int,
                (*self_0).dcoffset as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 12199444798915819164;
        }
        2 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 17277203570733699438;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 12199444798915819164;
            }
        }
        3 => {
            msg_get_frame(
                self_0,
                0x1 as libc::c_int as guint8,
                (*self_0).gain,
                0x15 as libc::c_int as guint8,
                0x10 as libc::c_int as guint8,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 12199444798915819164;
        }
        4 => {
            if process_frame_empty(
                (*self_0).ans as *mut guint8,
                192 as libc::c_int as size_t,
            ) != 0
            {
                (*self_0).tunedc_max = (*self_0).dcoffset;
            } else {
                (*self_0).tunedc_min = (*self_0).dcoffset;
            }
            if ((*self_0).tunedc_min as libc::c_int + 1 as libc::c_int)
                < (*self_0).tunedc_max as libc::c_int
            {
                fpi_ssm_jump_to_state(ssm, TUNEDC_SET_DCOFFSET_REQ as libc::c_int);
            } else if ((*self_0).tunedc_max as libc::c_int) < 0x35 as libc::c_int {
                (*self_0)
                    .dcoffset = ((*self_0).tunedc_max as libc::c_int + 1 as libc::c_int)
                    as guint8;
                fpi_ssm_next_state(ssm);
            } else {
                (*self_0).gain = ((*self_0).gain).wrapping_sub(1);
                fpi_ssm_jump_to_state(ssm, TUNEDC_SET_DCOFFSET_REQ as libc::c_int);
            }
            current_block = 12199444798915819164;
        }
        5 => {
            g_log(
                b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"-> DCoffset=0x%02X Gain=0x%02X\0" as *const u8 as *const libc::c_char,
                (*self_0).dcoffset as libc::c_int,
                (*self_0).gain as libc::c_int,
            );
            msg_set_regs(
                self_0,
                4 as libc::c_int,
                0x21 as libc::c_int,
                0x23 as libc::c_int,
                0x22 as libc::c_int,
                0x21 as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 12199444798915819164;
        }
        6 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 17277203570733699438;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 12199444798915819164;
            }
        }
        7 => {
            msg_set_regs(
                self_0,
                2 as libc::c_int,
                0xe0 as libc::c_int,
                (*self_0).gain as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 12199444798915819164;
        }
        8 => {
            fpi_ssm_next_state(ssm);
            current_block = 12199444798915819164;
        }
        9 => {
            msg_set_regs(
                self_0,
                2 as libc::c_int,
                0xe6 as libc::c_int,
                (*self_0).dcoffset as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 12199444798915819164;
        }
        10 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 17277203570733699438;
            } else {
                fpi_ssm_mark_completed(ssm);
                current_block = 12199444798915819164;
            }
        }
        _ => {
            g_assertion_message_expr(
                b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                b"../libfprint/drivers/etes603.c\0" as *const u8 as *const libc::c_char,
                1382 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 15],
                    &[libc::c_char; 15],
                >(b"m_tunedc_state\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    }
    match current_block {
        12199444798915819164 => return,
        _ => {
            fpi_ssm_mark_failed(ssm, fpi_device_error_new(FP_DEVICE_ERROR_PROTO));
            return;
        }
    };
}
unsafe extern "C" fn m_tunedc_complete(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(dev as gpointer);
    let mut idev: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    if error.is_null() {
        let mut ssm_tune: *mut FpiSsm = 0 as *mut FpiSsm;
        ssm_tune = fpi_ssm_new_full(
            FP_DEVICE(idev as gpointer),
            Some(
                m_tunevrb_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
            ),
            TUNEVRB_NUM_STATES as libc::c_int,
            TUNEVRB_NUM_STATES as libc::c_int,
            b"TUNEVRB_NUM_STATES\0" as *const u8 as *const libc::c_char,
        );
        fpi_ssm_start(
            ssm_tune,
            Some(
                m_tunevrb_complete
                    as unsafe extern "C" fn(
                        *mut FpiSsm,
                        *mut FpDevice,
                        *mut GError,
                    ) -> (),
            ),
        );
    } else {
        g_log(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"Error while tuning DCOFFSET\0" as *const u8 as *const libc::c_char,
        );
        reset_param(FPI_DEVICE_ETES603(dev as gpointer));
        fpi_image_device_session_error(idev, error);
    }
    if (*self_0).is_active == 0 {
        m_exit_start(idev);
    }
}
unsafe extern "C" fn m_init_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut current_block: u64;
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(dev as gpointer);
    if (*self_0).is_active == 0 as libc::c_int as libc::c_uint {
        fpi_ssm_mark_completed(ssm);
        return;
    }
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            msg_get_regs(
                self_0,
                4 as libc::c_int,
                0x70 as libc::c_int,
                0x71 as libc::c_int,
                0x72 as libc::c_int,
                0x73 as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 14434620278749266018;
        }
        1 => {
            if msg_parse_regs(self_0) != 0 {
                current_block = 17383903826307457638;
            } else if check_info(self_0) != 0 {
                current_block = 17383903826307457638;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 14434620278749266018;
            }
        }
        2 => {
            msg_get_cmd20(self_0);
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 14434620278749266018;
        }
        3 => {
            if msg_check_cmd20(self_0) != 0 {
                current_block = 17383903826307457638;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 14434620278749266018;
            }
        }
        4 => {
            msg_get_cmd25(self_0);
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 14434620278749266018;
        }
        5 => {
            if msg_check_cmd25(self_0) != 0 {
                current_block = 17383903826307457638;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 14434620278749266018;
            }
        }
        6 => {
            msg_set_regs(
                self_0,
                18 as libc::c_int,
                0x2 as libc::c_int,
                0x30 as libc::c_int,
                0x50 as libc::c_int,
                0xf as libc::c_int,
                0xe0 as libc::c_int,
                0x4 as libc::c_int,
                0xe1 as libc::c_int,
                0x8 as libc::c_int,
                0xe2 as libc::c_int,
                0xd as libc::c_int,
                0xe5 as libc::c_int,
                0x14 as libc::c_int,
                0xe6 as libc::c_int,
                0x36 as libc::c_int,
                0xf0 as libc::c_int,
                0 as libc::c_int,
                0xf2 as libc::c_int,
                0 as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 14434620278749266018;
        }
        7 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 17383903826307457638;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 14434620278749266018;
            }
        }
        8 => {
            msg_set_regs(
                self_0,
                16 as libc::c_int,
                0x41 as libc::c_int,
                0x12 as libc::c_int,
                0x42 as libc::c_int,
                0x34 as libc::c_int,
                0x43 as libc::c_int,
                0x56 as libc::c_int,
                0x44 as libc::c_int,
                0x78 as libc::c_int,
                0x45 as libc::c_int,
                0x90 as libc::c_int,
                0x46 as libc::c_int,
                0xab as libc::c_int,
                0x47 as libc::c_int,
                0xcd as libc::c_int,
                0x48 as libc::c_int,
                0xef as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 14434620278749266018;
        }
        9 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 17383903826307457638;
            } else {
                fpi_ssm_next_state(ssm);
                current_block = 14434620278749266018;
            }
        }
        10 => {
            msg_set_regs(
                self_0,
                48 as libc::c_int,
                0x20 as libc::c_int,
                0 as libc::c_int,
                0x21 as libc::c_int,
                0x23 as libc::c_int,
                0x22 as libc::c_int,
                0x21 as libc::c_int,
                0x23 as libc::c_int,
                0x20 as libc::c_int,
                0x24 as libc::c_int,
                0x14 as libc::c_int,
                0x25 as libc::c_int,
                0x6a as libc::c_int,
                0x26 as libc::c_int,
                0 as libc::c_int,
                0x27 as libc::c_int,
                0 as libc::c_int,
                0x28 as libc::c_int,
                0 as libc::c_int,
                0x29 as libc::c_int,
                0xc0 as libc::c_int,
                0x2a as libc::c_int,
                0x50 as libc::c_int,
                0x2b as libc::c_int,
                0x50 as libc::c_int,
                0x2c as libc::c_int,
                0x4d as libc::c_int,
                0x2d as libc::c_int,
                0x3 as libc::c_int,
                0x2e as libc::c_int,
                0x6 as libc::c_int,
                0x2f as libc::c_int,
                0x6 as libc::c_int,
                0x30 as libc::c_int,
                0x10 as libc::c_int,
                0x31 as libc::c_int,
                0x2 as libc::c_int,
                0x32 as libc::c_int,
                0x14 as libc::c_int,
                0x33 as libc::c_int,
                0x34 as libc::c_int,
                0x34 as libc::c_int,
                0x1 as libc::c_int,
                0x35 as libc::c_int,
                0x8 as libc::c_int,
                0x36 as libc::c_int,
                0x3 as libc::c_int,
                0x37 as libc::c_int,
                0x21 as libc::c_int,
            );
            async_tx(
                dev,
                0x2 as libc::c_int as libc::c_uint,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        async_tx_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                ),
                ssm,
            );
            current_block = 14434620278749266018;
        }
        11 => {
            if msg_check_ok(self_0) != 0 {
                current_block = 17383903826307457638;
            } else {
                fpi_ssm_mark_completed(ssm);
                current_block = 14434620278749266018;
            }
        }
        _ => {
            g_assertion_message_expr(
                b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
                b"../libfprint/drivers/etes603.c\0" as *const u8 as *const libc::c_char,
                1520 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"m_init_state\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    }
    match current_block {
        14434620278749266018 => return,
        _ => {
            fpi_ssm_mark_failed(ssm, fpi_device_error_new(FP_DEVICE_ERROR_PROTO));
            return;
        }
    };
}
unsafe extern "C" fn m_init_complete(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut idev: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    if error.is_null() {
        let mut ssm_tune: *mut FpiSsm = 0 as *mut FpiSsm;
        ssm_tune = fpi_ssm_new_full(
            FP_DEVICE(idev as gpointer),
            Some(
                m_tunedc_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
            ),
            TUNEDC_NUM_STATES as libc::c_int,
            TUNEDC_NUM_STATES as libc::c_int,
            b"TUNEDC_NUM_STATES\0" as *const u8 as *const libc::c_char,
        );
        fpi_ssm_start(
            ssm_tune,
            Some(
                m_tunedc_complete
                    as unsafe extern "C" fn(
                        *mut FpiSsm,
                        *mut FpDevice,
                        *mut GError,
                    ) -> (),
            ),
        );
    } else {
        g_log(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"Error initializing the device\0" as *const u8 as *const libc::c_char,
        );
        reset_param(FPI_DEVICE_ETES603(dev as gpointer));
        fpi_image_device_session_error(idev, error);
    };
}
unsafe extern "C" fn dev_activate(mut idev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(idev as gpointer);
    let mut ssm: *mut FpiSsm = 0 as *mut FpiSsm;
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
        g_assertion_message_expr(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/etes603.c\0" as *const u8 as *const libc::c_char,
            1555 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"dev_activate\0"))
                .as_ptr(),
            b"self\0" as *const u8 as *const libc::c_char,
        );
    }
    (*self_0).is_active = (0 as libc::c_int == 0) as libc::c_int as libc::c_uint;
    if (*self_0).dcoffset as libc::c_int == 0 as libc::c_int {
        g_log(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Tuning device...\0" as *const u8 as *const libc::c_char,
        );
        ssm = fpi_ssm_new_full(
            FP_DEVICE(idev as gpointer),
            Some(m_init_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
            INIT_NUM_STATES as libc::c_int,
            INIT_NUM_STATES as libc::c_int,
            b"INIT_NUM_STATES\0" as *const u8 as *const libc::c_char,
        );
        fpi_ssm_start(
            ssm,
            Some(
                m_init_complete
                    as unsafe extern "C" fn(
                        *mut FpiSsm,
                        *mut FpDevice,
                        *mut GError,
                    ) -> (),
            ),
        );
    } else {
        g_log(
            b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Using previous tuning (DCOFFSET=0x%02X,VRT=0x%02X,VRB=0x%02X,GAIN=0x%02X).\0"
                as *const u8 as *const libc::c_char,
            (*self_0).dcoffset as libc::c_int,
            (*self_0).vrt as libc::c_int,
            (*self_0).vrb as libc::c_int,
            (*self_0).gain as libc::c_int,
        );
        fpi_image_device_activate_complete(idev, 0 as *mut GError);
        ssm = fpi_ssm_new_full(
            FP_DEVICE(idev as gpointer),
            Some(
                m_finger_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
            ),
            FGR_NUM_STATES as libc::c_int,
            FGR_NUM_STATES as libc::c_int,
            b"FGR_NUM_STATES\0" as *const u8 as *const libc::c_char,
        );
        fpi_ssm_start(
            ssm,
            Some(
                m_finger_complete
                    as unsafe extern "C" fn(
                        *mut FpiSsm,
                        *mut FpDevice,
                        *mut GError,
                    ) -> (),
            ),
        );
    };
}
unsafe extern "C" fn dev_deactivate(mut idev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(idev as gpointer);
    g_log(
        b"libfprint-etes603\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"deactivating\0" as *const u8 as *const libc::c_char,
    );
    if (*self_0).is_active == (0 as libc::c_int == 0) as libc::c_int as libc::c_uint {
        (*self_0).is_active = 0 as libc::c_int as libc::c_uint;
    } else {
        m_exit_start(idev);
    };
}
unsafe extern "C" fn dev_open(mut idev: *mut FpImageDevice) {
    let mut error: *mut GError = 0 as *mut GError;
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(idev as gpointer);
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
    (*self_0)
        .req = g_malloc(::core::mem::size_of::<egis_msg>() as libc::c_ulong)
        as *mut egis_msg;
    (*self_0).ans = g_malloc(64000 as libc::c_int as gsize) as *mut egis_msg;
    (*self_0)
        .fp = g_malloc((64000 as libc::c_int * 4 as libc::c_int) as gsize)
        as *mut guint8;
    fpi_image_device_open_complete(idev, 0 as *mut GError);
}
unsafe extern "C" fn dev_close(mut idev: *mut FpImageDevice) {
    let mut error: *mut GError = 0 as *mut GError;
    let mut self_0: *mut FpiDeviceEtes603 = FPI_DEVICE_ETES603(idev as gpointer);
    g_free((*self_0).req as gpointer);
    g_free((*self_0).ans as gpointer);
    g_free((*self_0).fp as gpointer);
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
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x603 as libc::c_int as guint,
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
unsafe extern "C" fn fpi_device_etes603_init(mut self_0: *mut FpiDeviceEtes603) {}
unsafe extern "C" fn fpi_device_etes603_class_init(
    mut klass: *mut FpiDeviceEtes603Class,
) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    let mut img_class: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_CLASS(
        klass as gpointer,
    );
    (*dev_class).id = b"etes603\0" as *const u8 as *const libc::c_char;
    (*dev_class).full_name = b"EgisTec ES603\0" as *const u8 as *const libc::c_char;
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
    (*img_class).img_width = 256 as libc::c_int;
    (*img_class).img_height = -(1 as libc::c_int);
}
