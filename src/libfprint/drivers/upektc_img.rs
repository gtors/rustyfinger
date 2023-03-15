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
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn g_strconcat(string1: *const gchar, _: ...) -> *mut gchar;
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
    fn g_usb_device_get_pid(self_0: *mut GUsbDevice) -> guint16;
    fn g_usb_device_get_release(self_0: *mut GUsbDevice) -> guint16;
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
    fn fpi_ssm_next_state(machine: *mut FpiSsm);
    fn fpi_ssm_jump_to_state(machine: *mut FpiSsm, state: libc::c_int);
    fn fpi_ssm_mark_completed(machine: *mut FpiSsm);
    fn fpi_ssm_mark_failed(machine: *mut FpiSsm, error: *mut GError);
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
pub type gint16 = libc::c_short;
pub type guint16 = libc::c_ushort;
pub type gint32 = libc::c_int;
pub type guint32 = libc::c_uint;
pub type gint64 = libc::c_long;
pub type guint64 = libc::c_ulong;
pub type gssize = libc::c_long;
pub type gsize = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
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
pub type GError_autoptr = *mut GError;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type FpiDeviceUpektcImg = _FpiDeviceUpektcImg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceUpektcImg {
    pub parent: FpImageDevice,
    pub cmd: [libc::c_uchar; 64],
    pub response: [libc::c_uchar; 2052],
    pub image_bits: *mut libc::c_uchar,
    pub seq: libc::c_uchar,
    pub image_size: size_t,
    pub response_rest: size_t,
    pub deactivating: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiDeviceUpektcImgClass {
    pub parent_class: FpImageDeviceClass,
}
pub const ACTIVATE_NUM_STATES: activate_states = 12;
pub const ACTIVATE_READ_INIT_4_RESP: activate_states = 11;
pub const ACTIVATE_READ_INIT_3_RESP: activate_states = 9;
pub const ACTIVATE_READ_INIT_2_RESP: activate_states = 5;
pub const ACTIVATE_READ_INIT_1_RESP: activate_states = 3;
pub const ACTIVATE_READ_CTRL_RESP_2: activate_states = 7;
pub const ACTIVATE_READ_CTRL_RESP_1: activate_states = 1;
pub const ACTIVATE_INIT_4: activate_states = 10;
pub const ACTIVATE_INIT_3: activate_states = 8;
pub const ACTIVATE_INIT_2: activate_states = 4;
pub const ACTIVATE_INIT_1: activate_states = 2;
pub const ACTIVATE_CONTROL_REQ_2: activate_states = 6;
pub const ACTIVATE_CONTROL_REQ_1: activate_states = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub const DEACTIVATE_NUM_STATES: deactivate_states = 2;
pub const DEACTIVATE_READ_DEINIT_DATA: deactivate_states = 1;
pub const CAPTURE_READ_DATA: capture_states = 1;
pub const DEACTIVATE_DEINIT: deactivate_states = 0;
pub const CAPTURE_NUM_STATES: capture_states = 7;
pub const CAPTURE_READ_DATA_TERM: capture_states = 2;
pub const CAPTURE_ACK_00_28_TERM: capture_states = 6;
pub const CAPTURE_ACK_FRAME: capture_states = 5;
pub const CAPTURE_ACK_08: capture_states = 4;
pub const CAPTURE_ACK_00_28: capture_states = 3;
pub const CAPTURE_INIT_CAPTURE: capture_states = 0;
pub type capture_states = libc::c_uint;
pub type deactivate_states = libc::c_uint;
pub type activate_states = libc::c_uint;
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
static mut upek2020_init_1: [libc::c_uchar; 22] = [
    'C' as i32 as libc::c_uchar,
    'i' as i32 as libc::c_uchar,
    'a' as i32 as libc::c_uchar,
    'o' as i32 as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xda as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
];
static mut upek2020_init_2: [libc::c_uchar; 10] = [
    0x43 as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
];
static mut upek2020_init_3: [libc::c_uchar; 22] = [
    'C' as i32 as libc::c_uchar,
    'i' as i32 as libc::c_uchar,
    'a' as i32 as libc::c_uchar,
    'o' as i32 as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xbc as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
];
static mut upek2020_init_4: [libc::c_uchar; 16] = [
    'C' as i32 as libc::c_uchar,
    'i' as i32 as libc::c_uchar,
    'a' as i32 as libc::c_uchar,
    'o' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x6 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
];
static mut upek2020_deinit: [libc::c_uchar; 10] = [
    'C' as i32 as libc::c_uchar,
    'i' as i32 as libc::c_uchar,
    'a' as i32 as libc::c_uchar,
    'o' as i32 as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
];
static mut upek2020_init_capture: [libc::c_uchar; 23] = [
    'C' as i32 as libc::c_uchar,
    'i' as i32 as libc::c_uchar,
    'a' as i32 as libc::c_uchar,
    'o' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
];
static mut upek2020_ack_00_28: [libc::c_uchar; 17] = [
    'C' as i32 as libc::c_uchar,
    'i' as i32 as libc::c_uchar,
    'a' as i32 as libc::c_uchar,
    'o' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0xc4 as libc::c_int as libc::c_uchar,
];
static mut upek2020_ack_08: [libc::c_uchar; 9] = [
    'C' as i32 as libc::c_uchar,
    'i' as i32 as libc::c_uchar,
    'a' as i32 as libc::c_uchar,
    'o' as i32 as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
];
static mut upek2020_ack_frame: [libc::c_uchar; 10] = [
    'C' as i32 as libc::c_uchar,
    'i' as i32 as libc::c_uchar,
    'a' as i32 as libc::c_uchar,
    'o' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
];
#[inline]
unsafe extern "C" fn FPI_DEVICE_UPEKTC_IMG(
    mut ptr: gpointer,
) -> *mut FpiDeviceUpektcImg {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_upektc_img_get_type(),
    ) as *mut libc::c_void as *mut FpiDeviceUpektcImg;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_upektc_img_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_upektc_img_get_type_once();
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
unsafe extern "C" fn fpi_device_upektc_img_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_image_device_get_type(),
        g_intern_static_string(
            b"FpiDeviceUpektcImg\0" as *const u8 as *const libc::c_char,
        ),
        ::core::mem::size_of::<FpiDeviceUpektcImgClass>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_upektc_img_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpiDeviceUpektcImg>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpiDeviceUpektcImg) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_upektc_img_init
                        as unsafe extern "C" fn(*mut FpiDeviceUpektcImg) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut FpiDeviceUpektcImg_private_offset: gint = 0;
static mut fpi_device_upektc_img_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
unsafe extern "C" fn fpi_device_upektc_img_class_intern_init(mut klass: gpointer) {
    fpi_device_upektc_img_parent_class = g_type_class_peek_parent(klass);
    if FpiDeviceUpektcImg_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &mut FpiDeviceUpektcImg_private_offset,
        );
    }
    fpi_device_upektc_img_class_init(klass as *mut FpiDeviceUpektcImgClass);
}
unsafe extern "C" fn upektc_img_cmd_fix_seq(
    mut cmd_buf: *mut libc::c_uchar,
    mut seq: libc::c_uchar,
) {
    let mut byte: uint8_t = 0;
    byte = *cmd_buf.offset(5 as libc::c_int as isize);
    byte = (byte as libc::c_int & 0xf as libc::c_int) as uint8_t;
    byte = (byte as libc::c_int | (seq as libc::c_int) << 4 as libc::c_int) as uint8_t;
    *cmd_buf.offset(5 as libc::c_int as isize) = byte;
}
unsafe extern "C" fn upektc_img_cmd_update_crc(
    mut cmd_buf: *mut libc::c_uchar,
    mut size: size_t,
) {
    let mut crc: uint16_t = udf_crc(
        cmd_buf.offset(4 as libc::c_int as isize),
        size.wrapping_sub(6 as libc::c_int as libc::c_ulong),
    );
    *cmd_buf
        .offset(
            size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
        ) = (crc as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    *cmd_buf
        .offset(
            size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = ((crc as libc::c_int & 0xff00 as libc::c_int) >> 8 as libc::c_int)
        as libc::c_uchar;
}
unsafe extern "C" fn upektc_img_submit_req(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
    mut buf: *const libc::c_uchar,
    mut buf_size: size_t,
    mut seq: libc::c_uchar,
    mut cb: FpiUsbTransferCallback,
) {
    let mut self_0: *mut FpiDeviceUpektcImg = FPI_DEVICE_UPEKTC_IMG(dev as gpointer);
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(
        FP_DEVICE(dev as gpointer),
    );
    if buf_size > 64 as libc::c_int as libc::c_ulong {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"buf_size > MAX_CMD_SIZE\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-upektc_img\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"upektc_img_submit_req\0"))
                .as_ptr(),
            b"../libfprint/drivers/upektc_img.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
        );
        g_free(s as gpointer);
    }
    memcpy(
        ((*self_0).cmd).as_mut_ptr() as *mut libc::c_void,
        buf as *const libc::c_void,
        buf_size,
    );
    upektc_img_cmd_fix_seq(((*self_0).cmd).as_mut_ptr(), seq);
    upektc_img_cmd_update_crc(((*self_0).cmd).as_mut_ptr(), buf_size);
    fpi_usb_transfer_fill_bulk_full(
        transfer,
        (2 as libc::c_int | 0 as libc::c_int) as guint8,
        ((*self_0).cmd).as_mut_ptr(),
        buf_size,
        None,
    );
    (*transfer).ssm = ssm;
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_submit(
        transfer,
        4000 as libc::c_int as guint,
        0 as *mut GCancellable,
        cb,
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn upektc_img_read_data(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
    mut buf_size: size_t,
    mut buf_offset: size_t,
    mut cb: FpiUsbTransferCallback,
) {
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(
        FP_DEVICE(dev as gpointer),
    );
    let mut self_0: *mut FpiDeviceUpektcImg = FPI_DEVICE_UPEKTC_IMG(dev as gpointer);
    if buf_offset.wrapping_add(buf_size) > 2052 as libc::c_int as libc::c_ulong {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"buf_offset + buf_size > MAX_RESPONSE_SIZE\0" as *const u8
                as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-upektc_img\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"upektc_img_read_data\0"))
                .as_ptr(),
            b"../libfprint/drivers/upektc_img.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
        );
        g_free(s as gpointer);
    }
    fpi_usb_transfer_fill_bulk_full(
        transfer,
        (1 as libc::c_int | 0x80 as libc::c_int) as guint8,
        ((*self_0).response).as_mut_ptr().offset(buf_offset as isize),
        buf_size,
        None,
    );
    (*transfer).ssm = ssm;
    fpi_usb_transfer_submit(
        transfer,
        4000 as libc::c_int as guint,
        0 as *mut GCancellable,
        cb,
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn capture_reqs_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    match fpi_ssm_get_cur_state((*transfer).ssm) {
        6 => {
            fpi_ssm_jump_to_state(
                (*transfer).ssm,
                CAPTURE_READ_DATA_TERM as libc::c_int,
            );
        }
        _ => {
            fpi_ssm_jump_to_state((*transfer).ssm, CAPTURE_READ_DATA as libc::c_int);
        }
    };
}
unsafe extern "C" fn upektc_img_process_image_frame(
    mut image_buf: *mut libc::c_uchar,
    mut cmd_res: *mut libc::c_uchar,
) -> libc::c_int {
    let mut offset: libc::c_int = 8 as libc::c_int;
    let mut len: libc::c_int = (*cmd_res.offset(5 as libc::c_int as isize) as libc::c_int
        & 0xf as libc::c_int) << 8 as libc::c_int
        | *cmd_res.offset(6 as libc::c_int as isize) as libc::c_int;
    len -= 1 as libc::c_int;
    if *cmd_res.offset(7 as libc::c_int as isize) as libc::c_int == 0x2c as libc::c_int {
        len -= 10 as libc::c_int;
        offset += 10 as libc::c_int;
    }
    if *cmd_res.offset(7 as libc::c_int as isize) as libc::c_int == 0x20 as libc::c_int {
        len -= 4 as libc::c_int;
    }
    memcpy(
        image_buf as *mut libc::c_void,
        cmd_res.offset(offset as isize) as *const libc::c_void,
        len as libc::c_ulong,
    );
    return len;
}
unsafe extern "C" fn capture_read_data_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(device as gpointer);
    let mut self_0: *mut FpiDeviceUpektcImg = FPI_DEVICE_UPEKTC_IMG(dev as gpointer);
    let mut data: *mut libc::c_uchar = ((*self_0).response).as_mut_ptr();
    let mut img: *mut FpImage = 0 as *mut FpImage;
    let mut response_size: size_t = 0;
    if !error.is_null() {
        g_log(
            b"libfprint-upektc_img\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"request is not completed, %s\0" as *const u8 as *const libc::c_char,
            (*error).message,
        );
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    if (*self_0).deactivating != 0 {
        g_log(
            b"libfprint-upektc_img\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Deactivate requested\0" as *const u8 as *const libc::c_char,
        );
        fpi_ssm_mark_completed((*transfer).ssm);
        return;
    }
    g_log(
        b"libfprint-upektc_img\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"request completed, len: %.4x\0" as *const u8 as *const libc::c_char,
        (*transfer).actual_length as gint,
    );
    if (*transfer).actual_length == 0 as libc::c_int as libc::c_long {
        fpi_ssm_jump_to_state((*transfer).ssm, fpi_ssm_get_cur_state((*transfer).ssm));
        return;
    }
    if fpi_ssm_get_cur_state((*transfer).ssm) == CAPTURE_READ_DATA_TERM as libc::c_int {
        g_log(
            b"libfprint-upektc_img\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Terminating SSM\0" as *const u8 as *const libc::c_char,
        );
        fpi_ssm_mark_completed((*transfer).ssm);
        return;
    }
    if (*self_0).response_rest == 0 {
        response_size = (((*data.offset(5 as libc::c_int as isize) as libc::c_int
            & 0xf as libc::c_int) << 8 as libc::c_int)
            + *data.offset(6 as libc::c_int as isize) as libc::c_int) as size_t;
        response_size = (response_size as libc::c_ulong)
            .wrapping_add(9 as libc::c_int as libc::c_ulong) as size_t as size_t;
        if response_size > (*transfer).actual_length as libc::c_ulong {
            g_log(
                b"libfprint-upektc_img\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"response_size is %lu, actual_length is %d\0" as *const u8
                    as *const libc::c_char,
                response_size,
                (*transfer).actual_length as gint,
            );
            g_log(
                b"libfprint-upektc_img\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Waiting for rest of transfer\0" as *const u8 as *const libc::c_char,
            );
            if (*self_0).response_rest != 0 {
                let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
                s = g_strconcat(
                    b"BUG: (\0" as *const u8 as *const libc::c_char,
                    b"self->response_rest\0" as *const u8 as *const libc::c_char,
                    b")\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void,
                );
                g_log(
                    b"libfprint-upektc_img\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_CRITICAL,
                    b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
                    s,
                    (*::core::mem::transmute::<
                        &[u8; 21],
                        &[libc::c_char; 21],
                    >(b"capture_read_data_cb\0"))
                        .as_ptr(),
                    b"../libfprint/drivers/upektc_img.c\0" as *const u8
                        as *const libc::c_char,
                    226 as libc::c_int,
                );
                g_free(s as gpointer);
            }
            (*self_0)
                .response_rest = response_size
                .wrapping_sub((*transfer).actual_length as libc::c_ulong);
            fpi_ssm_jump_to_state((*transfer).ssm, CAPTURE_READ_DATA as libc::c_int);
            return;
        }
    }
    (*self_0).response_rest = 0 as libc::c_int as size_t;
    match *data.offset(4 as libc::c_int as isize) as libc::c_int {
        0 => {
            let mut current_block_72: u64;
            match *data.offset(7 as libc::c_int as isize) as libc::c_int {
                40 => {
                    g_log(
                        b"libfprint-upektc_img\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"18th byte is %.2x\0" as *const u8 as *const libc::c_char,
                        *data.offset(18 as libc::c_int as isize) as libc::c_int,
                    );
                    match *data.offset(18 as libc::c_int as isize) as libc::c_int {
                        12 => {
                            fpi_ssm_jump_to_state(
                                (*transfer).ssm,
                                CAPTURE_ACK_00_28 as libc::c_int,
                            );
                        }
                        0 => {
                            fpi_ssm_jump_to_state(
                                (*transfer).ssm,
                                CAPTURE_ACK_00_28 as libc::c_int,
                            );
                        }
                        30 => {
                            g_log(
                                b"libfprint-upektc_img\0" as *const u8
                                    as *const libc::c_char,
                                G_LOG_LEVEL_CRITICAL,
                                b"short scan, aborting\0" as *const u8
                                    as *const libc::c_char,
                            );
                            fpi_image_device_retry_scan(dev, FP_DEVICE_RETRY_TOO_SHORT);
                            fpi_image_device_report_finger_status(dev, 0 as libc::c_int);
                            fpi_ssm_jump_to_state(
                                (*transfer).ssm,
                                CAPTURE_ACK_00_28_TERM as libc::c_int,
                            );
                        }
                        29 => {
                            g_log(
                                b"libfprint-upektc_img\0" as *const u8
                                    as *const libc::c_char,
                                G_LOG_LEVEL_CRITICAL,
                                b"too much horisontal movement, aborting\0" as *const u8
                                    as *const libc::c_char,
                            );
                            fpi_image_device_retry_scan(
                                dev,
                                FP_DEVICE_RETRY_CENTER_FINGER,
                            );
                            fpi_image_device_report_finger_status(dev, 0 as libc::c_int);
                            fpi_ssm_jump_to_state(
                                (*transfer).ssm,
                                CAPTURE_ACK_00_28_TERM as libc::c_int,
                            );
                        }
                        _ => {
                            g_log(
                                b"libfprint-upektc_img\0" as *const u8
                                    as *const libc::c_char,
                                G_LOG_LEVEL_CRITICAL,
                                b"something bad happened, stop scan\0" as *const u8
                                    as *const libc::c_char,
                            );
                            fpi_image_device_retry_scan(
                                dev,
                                fp_device_retry_quark() as FpDeviceRetry,
                            );
                            fpi_image_device_report_finger_status(dev, 0 as libc::c_int);
                            fpi_ssm_jump_to_state(
                                (*transfer).ssm,
                                CAPTURE_ACK_00_28_TERM as libc::c_int,
                            );
                        }
                    }
                    current_block_72 = 9512719473022792396;
                }
                44 => {
                    fpi_image_device_report_finger_status(
                        dev,
                        (0 as libc::c_int == 0) as libc::c_int,
                    );
                    current_block_72 = 3910509402789218435;
                }
                36 => {
                    current_block_72 = 3910509402789218435;
                }
                32 => {
                    (*self_0)
                        .image_size = ((*self_0).image_size as libc::c_ulong)
                        .wrapping_add(
                            upektc_img_process_image_frame(
                                ((*self_0).image_bits)
                                    .offset((*self_0).image_size as isize),
                                data,
                            ) as libc::c_ulong,
                        ) as size_t as size_t;
                    if (*self_0).image_size
                        != (144 as libc::c_int * 384 as libc::c_int) as libc::c_ulong
                    {
                        let mut s_0: *mut libc::c_char = 0 as *mut libc::c_char;
                        s_0 = g_strconcat(
                            b"BUG: (\0" as *const u8 as *const libc::c_char,
                            b"self->image_size != IMAGE_SIZE\0" as *const u8
                                as *const libc::c_char,
                            b")\0" as *const u8 as *const libc::c_char,
                            0 as *mut libc::c_void,
                        );
                        g_log(
                            b"libfprint-upektc_img\0" as *const u8
                                as *const libc::c_char,
                            G_LOG_LEVEL_CRITICAL,
                            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
                            s_0,
                            (*::core::mem::transmute::<
                                &[u8; 21],
                                &[libc::c_char; 21],
                            >(b"capture_read_data_cb\0"))
                                .as_ptr(),
                            b"../libfprint/drivers/upektc_img.c\0" as *const u8
                                as *const libc::c_char,
                            310 as libc::c_int,
                        );
                        g_free(s_0 as gpointer);
                    }
                    g_log(
                        b"libfprint-upektc_img\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"Image size is %lu\0" as *const u8 as *const libc::c_char,
                        (*self_0).image_size,
                    );
                    img = fp_image_new(144 as libc::c_int, 384 as libc::c_int);
                    (*img)
                        .flags = ::core::mem::transmute::<
                        libc::c_uint,
                        FpiImageFlags,
                    >(
                        (*img).flags as libc::c_uint
                            | FPI_IMAGE_PARTIAL as libc::c_int as libc::c_uint,
                    );
                    memcpy(
                        (*img).data as *mut libc::c_void,
                        (*self_0).image_bits as *const libc::c_void,
                        (144 as libc::c_int * 384 as libc::c_int) as libc::c_ulong,
                    );
                    fpi_image_device_image_captured(dev, img);
                    fpi_image_device_report_finger_status(dev, 0 as libc::c_int);
                    fpi_ssm_mark_completed((*transfer).ssm);
                    current_block_72 = 9512719473022792396;
                }
                _ => {
                    g_log(
                        b"libfprint-upektc_img\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_CRITICAL,
                        b"Unknown response!\0" as *const u8 as *const libc::c_char,
                    );
                    fpi_ssm_mark_failed(
                        (*transfer).ssm,
                        fpi_device_error_new(FP_DEVICE_ERROR_GENERAL),
                    );
                    current_block_72 = 9512719473022792396;
                }
            }
            match current_block_72 {
                3910509402789218435 => {
                    (*self_0)
                        .image_size = ((*self_0).image_size as libc::c_ulong)
                        .wrapping_add(
                            upektc_img_process_image_frame(
                                ((*self_0).image_bits)
                                    .offset((*self_0).image_size as isize),
                                data,
                            ) as libc::c_ulong,
                        ) as size_t as size_t;
                    fpi_ssm_jump_to_state(
                        (*transfer).ssm,
                        CAPTURE_ACK_FRAME as libc::c_int,
                    );
                }
                _ => {}
            }
        }
        8 => {
            fpi_ssm_jump_to_state((*transfer).ssm, CAPTURE_ACK_08 as libc::c_int);
        }
        _ => {
            g_log(
                b"libfprint-upektc_img\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"Not handled response!\0" as *const u8 as *const libc::c_char,
            );
            fpi_ssm_mark_failed(
                (*transfer).ssm,
                fpi_device_error_new(FP_DEVICE_ERROR_GENERAL),
            );
        }
    };
}
unsafe extern "C" fn capture_run_state(mut ssm: *mut FpiSsm, mut _dev: *mut FpDevice) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceUpektcImg = FPI_DEVICE_UPEKTC_IMG(_dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            upektc_img_submit_req(
                ssm,
                dev,
                upek2020_init_capture.as_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 23]>() as libc::c_ulong,
                (*self_0).seq,
                Some(
                    capture_reqs_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
            (*self_0).seq = ((*self_0).seq).wrapping_add(1);
        }
        1 | 2 => {
            if (*self_0).response_rest == 0 {
                upektc_img_read_data(
                    ssm,
                    dev,
                    64 as libc::c_int as size_t,
                    0 as libc::c_int as size_t,
                    Some(
                        capture_read_data_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                );
            } else {
                upektc_img_read_data(
                    ssm,
                    dev,
                    (2052 as libc::c_int - 64 as libc::c_int) as size_t,
                    64 as libc::c_int as size_t,
                    Some(
                        capture_read_data_cb
                            as unsafe extern "C" fn(
                                *mut FpiUsbTransfer,
                                *mut FpDevice,
                                gpointer,
                                *mut GError,
                            ) -> (),
                    ),
                );
            }
        }
        3 | 6 => {
            upektc_img_submit_req(
                ssm,
                dev,
                upek2020_ack_00_28.as_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 17]>() as libc::c_ulong,
                (*self_0).seq,
                Some(
                    capture_reqs_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
            (*self_0).seq = ((*self_0).seq).wrapping_add(1);
        }
        4 => {
            upektc_img_submit_req(
                ssm,
                dev,
                upek2020_ack_08.as_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 9]>() as libc::c_ulong,
                0 as libc::c_int as libc::c_uchar,
                Some(
                    capture_reqs_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        5 => {
            upektc_img_submit_req(
                ssm,
                dev,
                upek2020_ack_frame.as_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 10]>() as libc::c_ulong,
                (*self_0).seq,
                Some(
                    capture_reqs_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
            (*self_0).seq = ((*self_0).seq).wrapping_add(1);
        }
        _ => {}
    };
}
unsafe extern "C" fn capture_sm_complete(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
    mut error_arg: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceUpektcImg = FPI_DEVICE_UPEKTC_IMG(_dev as gpointer);
    let mut error: GError_autoptr = error_arg;
    if (*self_0).deactivating != 0 {
        start_deactivation(dev);
    } else if !error.is_null() {
        fpi_image_device_session_error(
            dev,
            (if 0 as libc::c_int != 0 {
                error as *mut libc::c_void
            } else {
                g_steal_pointer(&mut error as *mut GError_autoptr as gpointer)
            }) as *mut GError,
        );
    } else {
        start_capture(dev);
    };
}
unsafe extern "C" fn start_capture(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceUpektcImg = FPI_DEVICE_UPEKTC_IMG(dev as gpointer);
    let mut ssm: *mut FpiSsm = 0 as *mut FpiSsm;
    (*self_0).image_size = 0 as libc::c_int as size_t;
    ssm = fpi_ssm_new_full(
        FP_DEVICE(dev as gpointer),
        Some(
            capture_run_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        CAPTURE_NUM_STATES as libc::c_int,
        CAPTURE_NUM_STATES as libc::c_int,
        b"CAPTURE_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            capture_sm_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn deactivate_reqs_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    if error.is_null() {
        fpi_ssm_jump_to_state((*transfer).ssm, CAPTURE_READ_DATA as libc::c_int);
    } else {
        fpi_ssm_mark_failed((*transfer).ssm, error);
    };
}
unsafe extern "C" fn deactivate_read_data_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    if error.is_null() {
        fpi_ssm_mark_completed((*transfer).ssm);
    } else {
        fpi_ssm_mark_failed((*transfer).ssm, error);
    };
}
unsafe extern "C" fn deactivate_run_state(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceUpektcImg = FPI_DEVICE_UPEKTC_IMG(_dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            upektc_img_submit_req(
                ssm,
                dev,
                upek2020_deinit.as_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 10]>() as libc::c_ulong,
                (*self_0).seq,
                Some(
                    deactivate_reqs_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
            (*self_0).seq = ((*self_0).seq).wrapping_add(1);
        }
        1 => {
            upektc_img_read_data(
                ssm,
                dev,
                64 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                Some(
                    deactivate_read_data_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn deactivate_sm_complete(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceUpektcImg = FPI_DEVICE_UPEKTC_IMG(_dev as gpointer);
    g_log(
        b"libfprint-upektc_img\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Deactivate completed\0" as *const u8 as *const libc::c_char,
    );
    (*self_0).deactivating = 0 as libc::c_int;
    fpi_image_device_deactivate_complete(dev, error);
}
unsafe extern "C" fn start_deactivation(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceUpektcImg = FPI_DEVICE_UPEKTC_IMG(dev as gpointer);
    let mut ssm: *mut FpiSsm = 0 as *mut FpiSsm;
    (*self_0).image_size = 0 as libc::c_int as size_t;
    ssm = fpi_ssm_new_full(
        FP_DEVICE(dev as gpointer),
        Some(
            deactivate_run_state
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        DEACTIVATE_NUM_STATES as libc::c_int,
        DEACTIVATE_NUM_STATES as libc::c_int,
        b"DEACTIVATE_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            deactivate_sm_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn init_reqs_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    if error.is_null() {
        fpi_ssm_next_state((*transfer).ssm);
    } else {
        fpi_ssm_mark_failed((*transfer).ssm, error);
    };
}
unsafe extern "C" fn init_read_data_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    if error.is_null() {
        fpi_ssm_next_state((*transfer).ssm);
    } else {
        fpi_ssm_mark_failed((*transfer).ssm, error);
    };
}
unsafe extern "C" fn activate_run_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    let mut idev: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    let mut self_0: *mut FpiDeviceUpektcImg = FPI_DEVICE_UPEKTC_IMG(dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 | 6 => {
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
            *((*transfer).buffer)
                .offset(0 as libc::c_int as isize) = '\0' as i32 as guchar;
            (*transfer).ssm = ssm;
            fpi_usb_transfer_submit(
                transfer,
                4000 as libc::c_int as guint,
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
            upektc_img_submit_req(
                ssm,
                idev,
                upek2020_init_1.as_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 22]>() as libc::c_ulong,
                0 as libc::c_int as libc::c_uchar,
                Some(
                    init_reqs_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        4 => {
            upektc_img_submit_req(
                ssm,
                idev,
                upek2020_init_2.as_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 10]>() as libc::c_ulong,
                0 as libc::c_int as libc::c_uchar,
                Some(
                    init_reqs_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        8 => {
            upektc_img_submit_req(
                ssm,
                idev,
                upek2020_init_3.as_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 22]>() as libc::c_ulong,
                0 as libc::c_int as libc::c_uchar,
                Some(
                    init_reqs_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        10 => {
            upektc_img_submit_req(
                ssm,
                idev,
                upek2020_init_4.as_ptr(),
                ::core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
                (*self_0).seq,
                Some(
                    init_reqs_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
            (*self_0).seq = ((*self_0).seq).wrapping_add(1);
        }
        1 | 7 | 3 | 5 | 9 | 11 => {
            upektc_img_read_data(
                ssm,
                idev,
                64 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                Some(
                    init_read_data_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn activate_sm_complete(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    fpi_image_device_activate_complete(dev, error);
    if error.is_null() {
        start_capture(dev);
    }
}
unsafe extern "C" fn dev_activate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceUpektcImg = FPI_DEVICE_UPEKTC_IMG(dev as gpointer);
    let mut ssm: *mut FpiSsm = fpi_ssm_new_full(
        FP_DEVICE(dev as gpointer),
        Some(
            activate_run_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        ACTIVATE_NUM_STATES as libc::c_int,
        ACTIVATE_NUM_STATES as libc::c_int,
        b"ACTIVATE_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    (*self_0).seq = 0 as libc::c_int as libc::c_uchar;
    fpi_ssm_start(
        ssm,
        Some(
            activate_sm_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn dev_deactivate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceUpektcImg = FPI_DEVICE_UPEKTC_IMG(dev as gpointer);
    (*self_0).deactivating = (0 as libc::c_int == 0) as libc::c_int;
}
unsafe extern "C" fn dev_init(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceUpektcImg = FPI_DEVICE_UPEKTC_IMG(dev as gpointer);
    let mut error: *mut GError = 0 as *mut GError;
    if g_usb_device_claim_interface(
        fpi_device_get_usb_device(FP_DEVICE(dev as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    ) == 0
    {
        fpi_image_device_open_complete(dev, error);
        return;
    }
    (*self_0)
        .image_bits = g_malloc0(
        (144 as libc::c_int * 384 as libc::c_int * 2 as libc::c_int) as gsize,
    ) as *mut libc::c_uchar;
    fpi_image_device_open_complete(dev, 0 as *mut GError);
}
unsafe extern "C" fn dev_deinit(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceUpektcImg = FPI_DEVICE_UPEKTC_IMG(dev as gpointer);
    let mut error: *mut GError = 0 as *mut GError;
    let mut _pp: C2RustUnnamed_4 = C2RustUnnamed_4 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*self_0).image_bits as *mut *mut libc::c_uchar as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    g_usb_device_release_interface(
        fpi_device_get_usb_device(FP_DEVICE(dev as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    );
    fpi_image_device_close_complete(dev, error);
}
unsafe extern "C" fn discover(mut usb_device: *mut GUsbDevice) -> libc::c_int {
    let mut pid: gint16 = g_usb_device_get_pid(usb_device) as gint16;
    let mut bcd: gint16 = g_usb_device_get_release(usb_device) as gint16;
    if pid as libc::c_int == 0x2020 as libc::c_int
        && bcd as libc::c_int == 1 as libc::c_int
    {
        return 100 as libc::c_int;
    }
    if pid as libc::c_int == 0x2016 as libc::c_int
        && bcd as libc::c_int == 2 as libc::c_int
    {
        return 100 as libc::c_int;
    }
    return 0 as libc::c_int;
}
static mut id_table: [FpIdEntry; 3] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x2016 as libc::c_int as guint,
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
                        pid: 0x2020 as libc::c_int as guint,
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
unsafe extern "C" fn fpi_device_upektc_img_init(mut self_0: *mut FpiDeviceUpektcImg) {}
unsafe extern "C" fn fpi_device_upektc_img_class_init(
    mut klass: *mut FpiDeviceUpektcImgClass,
) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    let mut img_class: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_CLASS(
        klass as gpointer,
    );
    (*dev_class).id = b"upektc_img\0" as *const u8 as *const libc::c_char;
    (*dev_class)
        .full_name = b"Upek TouchChip Fingerprint Coprocessor\0" as *const u8
        as *const libc::c_char;
    (*dev_class).type_0 = FP_DEVICE_TYPE_USB;
    (*dev_class).id_table = id_table.as_ptr();
    (*dev_class).scan_type = FP_SCAN_TYPE_SWIPE;
    (*dev_class)
        .usb_discover = Some(
        discover as unsafe extern "C" fn(*mut GUsbDevice) -> libc::c_int,
    );
    (*img_class)
        .img_open = Some(dev_init as unsafe extern "C" fn(*mut FpImageDevice) -> ());
    (*img_class)
        .img_close = Some(dev_deinit as unsafe extern "C" fn(*mut FpImageDevice) -> ());
    (*img_class)
        .activate = Some(dev_activate as unsafe extern "C" fn(*mut FpImageDevice) -> ());
    (*img_class)
        .deactivate = Some(
        dev_deactivate as unsafe extern "C" fn(*mut FpImageDevice) -> (),
    );
    (*img_class).bz3_threshold = 20 as libc::c_int;
    (*img_class).img_width = 144 as libc::c_int;
    (*img_class).img_height = 384 as libc::c_int;
}
