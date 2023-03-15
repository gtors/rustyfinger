use ::libc;
extern "C" {
    pub type _GData;
    pub type _GCancellablePrivate;
    pub type _FpiSsm;
    fn fpi_ssm_get_cur_state(machine: *mut FpiSsm) -> libc::c_int;
    fn fpi_ssm_mark_failed(machine: *mut FpiSsm, error: *mut GError);
    fn fpi_ssm_mark_completed(machine: *mut FpiSsm);
    fn fpi_ssm_next_state_delayed(machine: *mut FpiSsm, delay: libc::c_int);
    fn fpi_ssm_jump_to_state(machine: *mut FpiSsm, state: libc::c_int);
    fn fpi_ssm_next_state(machine: *mut FpiSsm);
    fn fpi_ssm_start(ssm: *mut FpiSsm, callback: FpiSsmCompletedCallback);
    fn fpi_ssm_new_full(
        dev: *mut FpDevice,
        handler: FpiSsmHandlerCallback,
        nr_states: libc::c_int,
        start_cleanup: libc::c_int,
        machine_name: *const libc::c_char,
    ) -> *mut FpiSsm;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_clear_error(err: *mut *mut GError);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_nth(list: *mut GSList, n: guint) -> *mut GSList;
    fn g_slist_foreach(list: *mut GSList, func: GFunc, user_data: gpointer);
    fn g_get_monotonic_time() -> gint64;
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn g_log_structured(log_domain: *const gchar, log_level: GLogLevelFlags, _: ...);
    fn g_return_if_fail_warning(
        log_domain: *const libc::c_char,
        pretty_function: *const libc::c_char,
        expression: *const libc::c_char,
    );
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
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
    fn fpi_do_movement_estimation(ctx: *mut fpi_frame_asmbl_ctx, stripes: *mut GSList);
    fn fpi_assemble_frames(
        ctx: *mut fpi_frame_asmbl_ctx,
        stripes: *mut GSList,
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
    fn fpi_usb_transfer_submit(
        transfer: *mut FpiUsbTransfer,
        timeout_ms: guint,
        cancellable: *mut GCancellable,
        callback: FpiUsbTransferCallback,
        user_data: gpointer,
    );
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
pub type gconstpointer = *const libc::c_void;
pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
pub type GFunc = Option::<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const G_USB_DEVICE_ERROR_LAST: C2RustUnnamed_0 = 10;
pub const G_USB_DEVICE_ERROR_PERMISSION_DENIED: C2RustUnnamed_0 = 9;
pub const G_USB_DEVICE_ERROR_FAILED: C2RustUnnamed_0 = 8;
pub const G_USB_DEVICE_ERROR_CANCELLED: C2RustUnnamed_0 = 7;
pub const G_USB_DEVICE_ERROR_ALREADY_OPEN: C2RustUnnamed_0 = 6;
pub const G_USB_DEVICE_ERROR_NOT_OPEN: C2RustUnnamed_0 = 5;
pub const G_USB_DEVICE_ERROR_NO_DEVICE: C2RustUnnamed_0 = 4;
pub const G_USB_DEVICE_ERROR_NOT_SUPPORTED: C2RustUnnamed_0 = 3;
pub const G_USB_DEVICE_ERROR_TIMED_OUT: C2RustUnnamed_0 = 2;
pub const G_USB_DEVICE_ERROR_IO: C2RustUnnamed_0 = 1;
pub const G_USB_DEVICE_ERROR_INTERNAL: C2RustUnnamed_0 = 0;
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
pub type FpiDeviceElan = _FpiDeviceElan;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceElan {
    pub parent: FpImageDevice,
    pub dev_type: libc::c_ushort,
    pub fw_ver: libc::c_ushort,
    pub process_frame: Option::<
        unsafe extern "C" fn(*mut libc::c_ushort, *mut *mut GSList) -> (),
    >,
    pub cmd: *const elan_cmd,
    pub cmd_timeout: libc::c_int,
    pub active: gboolean,
    pub deactivating: gboolean,
    pub last_read: *mut libc::c_uchar,
    pub calib_atts_left: libc::c_uchar,
    pub calib_status: libc::c_uchar,
    pub background: *mut libc::c_ushort,
    pub frame_width: libc::c_uchar,
    pub frame_height: libc::c_uchar,
    pub raw_frame_height: libc::c_uchar,
    pub num_frames: libc::c_int,
    pub frames: *mut GSList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elan_cmd {
    pub cmd: [libc::c_uchar; 2],
    pub response_len: libc::c_int,
    pub response_in: libc::c_int,
    pub devices: libc::c_ushort,
    pub never_cancel: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiDeviceElanClass {
    pub parent_class: FpImageDeviceClass,
}
pub const STOP_CAPTURE_NUM_STATES: stop_capture_states = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub const STOP_CAPTURE: stop_capture_states = 0;
pub const CAPTURE_WAIT_FINGER: capture_states = 1;
pub const CAPTURE_NUM_STATES: capture_states = 4;
pub const CAPTURE_CHECK_ENOUGH_FRAMES: capture_states = 3;
pub const CAPTURE_READ_DATA: capture_states = 2;
pub const CAPTURE_LED_ON: capture_states = 0;
pub const CALIBRATE_NUM_STATES: calibrate_states = 7;
pub const CALIBRATE_GET_STATUS: calibrate_states = 4;
pub const CALIBRATE_REPEAT_STATUS: calibrate_states = 6;
pub const CALIBRATE_GET_BACKGROUND: calibrate_states = 0;
pub const CALIBRATE_CHECK_STATUS: calibrate_states = 5;
pub const CALIBRATE_CHECK_NEEDED: calibrate_states = 3;
pub const CALIBRATE_GET_MEAN: calibrate_states = 2;
pub const CALIBRATE_SAVE_BACKGROUND: calibrate_states = 1;
pub const ACTIVATE_NUM_STATES: activate_states = 5;
pub const ACTIVATE_CMD_1: activate_states = 4;
pub const ACTIVATE_SET_SENSOR_DIM: activate_states = 3;
pub const ACTIVATE_GET_SENSOR_DIM: activate_states = 2;
pub const ACTIVATE_SET_FW_VER: activate_states = 1;
pub const ACTIVATE_GET_FW_VER: activate_states = 0;
pub type stop_capture_states = libc::c_uint;
pub type capture_states = libc::c_uint;
pub type calibrate_states = libc::c_uint;
pub type activate_states = libc::c_uint;
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
unsafe extern "C" fn FPI_DEVICE_ELAN(mut ptr: gpointer) -> *mut FpiDeviceElan {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_elan_get_type(),
    ) as *mut libc::c_void as *mut FpiDeviceElan;
}
static mut get_sensor_dim_cmd: elan_cmd = {
    let mut init = elan_cmd {
        cmd: [0 as libc::c_int as libc::c_uchar, 0xc as libc::c_int as libc::c_uchar],
        response_len: 0x4 as libc::c_int,
        response_in: 0x3 as libc::c_int | 0x80 as libc::c_int,
        devices: 0 as libc::c_int as libc::c_ushort,
        never_cancel: 0,
    };
    init
};
static mut get_fw_ver_cmd: elan_cmd = {
    let mut init = elan_cmd {
        cmd: [
            0x40 as libc::c_int as libc::c_uchar,
            0x19 as libc::c_int as libc::c_uchar,
        ],
        response_len: 0x2 as libc::c_int,
        response_in: 0x3 as libc::c_int | 0x80 as libc::c_int,
        devices: 0 as libc::c_int as libc::c_ushort,
        never_cancel: 0,
    };
    init
};
static mut activate_cmd_1: elan_cmd = {
    let mut init = elan_cmd {
        cmd: [
            0x40 as libc::c_int as libc::c_uchar,
            0x2a as libc::c_int as libc::c_uchar,
        ],
        response_len: 0x2 as libc::c_int,
        response_in: 0x3 as libc::c_int | 0x80 as libc::c_int,
        devices: ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ushort,
        never_cancel: 0,
    };
    init
};
static mut get_image_cmd: elan_cmd = {
    let mut init = elan_cmd {
        cmd: [0 as libc::c_int as libc::c_uchar, 0x9 as libc::c_int as libc::c_uchar],
        response_len: -(1 as libc::c_int),
        response_in: 0x2 as libc::c_int | 0x80 as libc::c_int,
        devices: 0 as libc::c_int as libc::c_ushort,
        never_cancel: 0,
    };
    init
};
static mut get_calib_status_cmd: elan_cmd = {
    let mut init = elan_cmd {
        cmd: [
            0x40 as libc::c_int as libc::c_uchar,
            0x23 as libc::c_int as libc::c_uchar,
        ],
        response_len: 0x1 as libc::c_int,
        response_in: 0x3 as libc::c_int | 0x80 as libc::c_int,
        devices: 0 as libc::c_int as libc::c_ushort,
        never_cancel: 0,
    };
    init
};
static mut get_calib_mean_cmd: elan_cmd = {
    let mut init = elan_cmd {
        cmd: [
            0x40 as libc::c_int as libc::c_uchar,
            0x24 as libc::c_int as libc::c_uchar,
        ],
        response_len: 0x2 as libc::c_int,
        response_in: 0x3 as libc::c_int | 0x80 as libc::c_int,
        devices: 0 as libc::c_int as libc::c_ushort,
        never_cancel: 0,
    };
    init
};
static mut led_on_cmd: elan_cmd = {
    let mut init = elan_cmd {
        cmd: [
            0x40 as libc::c_int as libc::c_uchar,
            0x31 as libc::c_int as libc::c_uchar,
        ],
        response_len: 0 as libc::c_int,
        response_in: 0x3 as libc::c_int | 0x80 as libc::c_int,
        devices: 0 as libc::c_int as libc::c_ushort,
        never_cancel: 0,
    };
    init
};
static mut pre_scan_cmd: elan_cmd = {
    let mut init = elan_cmd {
        cmd: [
            0x40 as libc::c_int as libc::c_uchar,
            0x3f as libc::c_int as libc::c_uchar,
        ],
        response_len: 0x1 as libc::c_int,
        response_in: 0x3 as libc::c_int | 0x80 as libc::c_int,
        devices: 0 as libc::c_int as libc::c_ushort,
        never_cancel: 0,
    };
    init
};
static mut stop_cmd: elan_cmd = {
    let mut init = elan_cmd {
        cmd: [0 as libc::c_int as libc::c_uchar, 0xb as libc::c_int as libc::c_uchar],
        response_len: 0 as libc::c_int,
        response_in: 0x3 as libc::c_int | 0x80 as libc::c_int,
        devices: 0 as libc::c_int as libc::c_ushort,
        never_cancel: (0 as libc::c_int == 0) as libc::c_int,
    };
    init
};
static mut elan_id_table: [FpIdEntry; 62] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0x903 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0x907 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: ((1 as libc::c_int) << 0 as libc::c_int) as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc01 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc02 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc03 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: ((1 as libc::c_int) << 1 as libc::c_int) as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc04 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc05 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc06 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc07 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc08 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc09 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc0a as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc0b as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc0c as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc0d as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc0e as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc0f as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc10 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc11 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc12 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc13 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc14 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc15 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc16 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc17 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc18 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc19 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc1a as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc1b as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc1c as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc1d as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc1e as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc1f as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc20 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc21 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc22 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc23 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc24 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc25 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc26 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc27 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc28 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc29 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc2a as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc2b as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc2c as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc2d as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc2e as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc2f as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc30 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc31 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc32 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc33 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc3d as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc42 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: ((1 as libc::c_int) << 2 as libc::c_int) as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc4b as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc4d as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc4f as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc63 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc6e as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc58 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
unsafe extern "C" fn elan_get_pixel(
    mut ctx: *mut fpi_frame_asmbl_ctx,
    mut frame: *mut fpi_frame,
    mut x: libc::c_uint,
    mut y: libc::c_uint,
) -> libc::c_uchar {
    return *((*frame).data)
        .as_mut_ptr()
        .offset(x.wrapping_add(y.wrapping_mul((*ctx).frame_width)) as isize);
}
static mut assembling_ctx: fpi_frame_asmbl_ctx = unsafe {
    {
        let mut init = fpi_frame_asmbl_ctx {
            frame_width: 0 as libc::c_int as libc::c_uint,
            frame_height: 0 as libc::c_int as libc::c_uint,
            image_width: 0 as libc::c_int as libc::c_uint,
            get_pixel: Some(
                elan_get_pixel
                    as unsafe extern "C" fn(
                        *mut fpi_frame_asmbl_ctx,
                        *mut fpi_frame,
                        libc::c_uint,
                        libc::c_uint,
                    ) -> libc::c_uchar,
            ),
        };
        init
    }
};
#[inline(never)]
unsafe extern "C" fn fpi_device_elan_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_image_device_get_type(),
        g_intern_static_string(b"FpiDeviceElan\0" as *const u8 as *const libc::c_char),
        ::core::mem::size_of::<FpiDeviceElanClass>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_elan_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpiDeviceElan>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpiDeviceElan) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_elan_init
                        as unsafe extern "C" fn(*mut FpiDeviceElan) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn fpi_device_elan_class_intern_init(mut klass: gpointer) {
    fpi_device_elan_parent_class = g_type_class_peek_parent(klass);
    if FpiDeviceElan_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpiDeviceElan_private_offset);
    }
    fpi_device_elan_class_init(klass as *mut FpiDeviceElanClass);
}
static mut FpiDeviceElan_private_offset: gint = 0;
static mut fpi_device_elan_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn fpi_device_elan_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_elan_get_type_once();
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
unsafe extern "C" fn cmp_short(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return *(a as *mut libc::c_short) as libc::c_int
        - *(b as *mut libc::c_short) as libc::c_int;
}
unsafe extern "C" fn elan_dev_reset_state(mut elandev: *mut FpiDeviceElan) {
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"100\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 21],
            &[libc::c_char; 21],
        >(b"elan_dev_reset_state\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:100\0" as *const u8 as *const libc::c_char,
    );
    (*elandev).cmd = 0 as *const elan_cmd;
    (*elandev).cmd_timeout = 10000 as libc::c_int;
    (*elandev).calib_status = 0 as libc::c_int as libc::c_uchar;
    g_free((*elandev).last_read as gpointer);
    (*elandev).last_read = 0 as *mut libc::c_uchar;
    g_slist_free_full(
        (*elandev).frames,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*elandev).frames = 0 as *mut GSList;
    (*elandev).num_frames = 0 as libc::c_int;
}
unsafe extern "C" fn elan_save_frame(
    mut self_0: *mut FpiDeviceElan,
    mut frame: *mut libc::c_ushort,
) {
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"118\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"elan_save_frame\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:118\0" as *const u8 as *const libc::c_char,
    );
    let mut frame_width: libc::c_uchar = (*self_0).frame_width;
    let mut frame_height: libc::c_uchar = (*self_0).frame_height;
    let mut raw_height: libc::c_uchar = (*self_0).raw_frame_height;
    let mut frame_margin: libc::c_uchar = ((raw_height as libc::c_int
        - (*self_0).frame_height as libc::c_int) / 2 as libc::c_int) as libc::c_uchar;
    let mut frame_idx: libc::c_int = 0;
    let mut raw_idx: libc::c_int = 0;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < frame_height as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < frame_width as libc::c_int {
            if (*self_0).dev_type as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int
                != 0
            {
                raw_idx = x
                    + (y + frame_margin as libc::c_int) * frame_width as libc::c_int;
            } else {
                raw_idx = frame_margin as libc::c_int + y
                    + x * raw_height as libc::c_int;
            }
            frame_idx = x + y * frame_width as libc::c_int;
            *frame
                .offset(
                    frame_idx as isize,
                ) = *((*self_0).last_read as *mut libc::c_ushort)
                .offset(raw_idx as isize);
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C" fn elan_save_background(mut elandev: *mut FpiDeviceElan) {
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"156\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 21],
            &[libc::c_char; 21],
        >(b"elan_save_background\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:156\0" as *const u8 as *const libc::c_char,
    );
    g_free((*elandev).background as gpointer);
    (*elandev)
        .background = g_malloc(
        (((*elandev).frame_width as libc::c_int * (*elandev).frame_height as libc::c_int)
            as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    ) as *mut libc::c_ushort;
    elan_save_frame(elandev, (*elandev).background);
}
unsafe extern "C" fn elan_save_img_frame(
    mut elandev: *mut FpiDeviceElan,
) -> libc::c_int {
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"203\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"elan_save_img_frame\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:203\0" as *const u8 as *const libc::c_char,
    );
    let mut frame_size: libc::c_uint = ((*elandev).frame_width as libc::c_int
        * (*elandev).frame_height as libc::c_int) as libc::c_uint;
    let mut frame: *mut libc::c_ushort = g_malloc(
        (frame_size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    ) as *mut libc::c_ushort;
    elan_save_frame(elandev, frame);
    let mut sum: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < frame_size {
        if *((*elandev).background).offset(i as isize) as libc::c_int
            > *frame.offset(i as isize) as libc::c_int
        {
            *frame.offset(i as isize) = 0 as libc::c_int as libc::c_ushort;
        } else {
            let ref mut fresh0 = *frame.offset(i as isize);
            *fresh0 = (*fresh0 as libc::c_int
                - *((*elandev).background).offset(i as isize) as libc::c_int)
                as libc::c_ushort;
        }
        sum = sum.wrapping_add(*frame.offset(i as isize) as libc::c_uint);
        i += 1;
    }
    if sum == 0 as libc::c_int as libc::c_uint {
        g_log(
            b"libfprint-elan\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"frame darker than background; finger present during calibration?\0"
                as *const u8 as *const libc::c_char,
        );
        g_free(frame as gpointer);
        return -(1 as libc::c_int);
    }
    (*elandev).frames = g_slist_prepend((*elandev).frames, frame as gpointer);
    (*elandev).num_frames += 1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn elan_process_frame_linear(
    mut raw_frame: *mut libc::c_ushort,
    mut frames: *mut *mut GSList,
) {
    let mut frame_size: libc::c_uint = (assembling_ctx.frame_width)
        .wrapping_mul(assembling_ctx.frame_height);
    let mut frame: *mut fpi_frame = g_malloc(
        (frame_size as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<fpi_frame>() as libc::c_ulong),
    ) as *mut fpi_frame;
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"242\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 26],
            &[libc::c_char; 26],
        >(b"elan_process_frame_linear\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:242\0" as *const u8 as *const libc::c_char,
    );
    let mut min: libc::c_ushort = 0xffff as libc::c_int as libc::c_ushort;
    let mut max: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < frame_size {
        if (*raw_frame.offset(i as isize) as libc::c_int) < min as libc::c_int {
            min = *raw_frame.offset(i as isize);
        }
        if *raw_frame.offset(i as isize) as libc::c_int > max as libc::c_int {
            max = *raw_frame.offset(i as isize);
        }
        i += 1;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if max as libc::c_int != min as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-elan\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
            254 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"elan_process_frame_linear\0"))
                .as_ptr(),
            b"max != min\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut px: libc::c_ushort = 0;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while (i_0 as libc::c_uint) < frame_size {
        px = *raw_frame.offset(i_0 as isize);
        px = ((px as libc::c_int - min as libc::c_int) * 0xff as libc::c_int
            / (max as libc::c_int - min as libc::c_int)) as libc::c_ushort;
        *((*frame).data).as_mut_ptr().offset(i_0 as isize) = px as libc::c_uchar;
        i_0 += 1;
    }
    *frames = g_slist_prepend(*frames, frame as gpointer);
}
unsafe extern "C" fn elan_process_frame_thirds(
    mut raw_frame: *mut libc::c_ushort,
    mut frames: *mut *mut GSList,
) {
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"272\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 26],
            &[libc::c_char; 26],
        >(b"elan_process_frame_thirds\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:272\0" as *const u8 as *const libc::c_char,
    );
    let mut frame_size: libc::c_uint = (assembling_ctx.frame_width)
        .wrapping_mul(assembling_ctx.frame_height);
    let mut frame: *mut fpi_frame = g_malloc(
        (frame_size as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<fpi_frame>() as libc::c_ulong),
    ) as *mut fpi_frame;
    let mut lvl0: libc::c_ushort = 0;
    let mut lvl1: libc::c_ushort = 0;
    let mut lvl2: libc::c_ushort = 0;
    let mut lvl3: libc::c_ushort = 0;
    let mut sorted: *mut libc::c_ushort = g_malloc(
        (frame_size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    ) as *mut libc::c_ushort;
    memcpy(
        sorted as *mut libc::c_void,
        raw_frame as *const libc::c_void,
        (frame_size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
    qsort(
        sorted as *mut libc::c_void,
        frame_size as size_t,
        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
        Some(
            cmp_short
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    lvl0 = *sorted.offset(0 as libc::c_int as isize);
    lvl1 = *sorted
        .offset(
            frame_size
                .wrapping_mul(3 as libc::c_int as libc::c_uint)
                .wrapping_div(10 as libc::c_int as libc::c_uint) as isize,
        );
    lvl2 = *sorted
        .offset(
            frame_size
                .wrapping_mul(65 as libc::c_int as libc::c_uint)
                .wrapping_div(100 as libc::c_int as libc::c_uint) as isize,
        );
    lvl3 = *sorted
        .offset(frame_size.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
    g_free(sorted as gpointer);
    let mut px: libc::c_ushort = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < frame_size {
        px = *raw_frame.offset(i as isize);
        if lvl0 as libc::c_int <= px as libc::c_int
            && (px as libc::c_int) < lvl1 as libc::c_int
        {
            px = ((px as libc::c_int - lvl0 as libc::c_int) * 99 as libc::c_int
                / (lvl1 as libc::c_int - lvl0 as libc::c_int)) as libc::c_ushort;
        } else if lvl1 as libc::c_int <= px as libc::c_int
            && (px as libc::c_int) < lvl2 as libc::c_int
        {
            px = (99 as libc::c_int
                + (px as libc::c_int - lvl1 as libc::c_int) * 56 as libc::c_int
                    / (lvl2 as libc::c_int - lvl1 as libc::c_int)) as libc::c_ushort;
        } else {
            px = (155 as libc::c_int
                + (px as libc::c_int - lvl2 as libc::c_int) * 100 as libc::c_int
                    / (lvl3 as libc::c_int - lvl2 as libc::c_int)) as libc::c_ushort;
        }
        *((*frame).data).as_mut_ptr().offset(i as isize) = px as libc::c_uchar;
        i += 1;
    }
    *frames = g_slist_prepend(*frames, frame as gpointer);
}
unsafe extern "C" fn elan_submit_image(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceElan = FPI_DEVICE_ELAN(dev as gpointer);
    let mut raw_frames: *mut GSList = 0 as *mut GSList;
    let mut frames: *mut GSList = 0 as *mut GSList;
    let mut img: *mut FpImage = 0 as *mut FpImage;
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"315\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"elan_submit_image\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:315\0" as *const u8 as *const libc::c_char,
    );
    raw_frames = g_slist_nth((*self_0).frames, 2 as libc::c_int as guint);
    assembling_ctx.frame_width = (*self_0).frame_width as libc::c_uint;
    assembling_ctx.frame_height = (*self_0).frame_height as libc::c_uint;
    assembling_ctx
        .image_width = ((*self_0).frame_width as libc::c_int * 3 as libc::c_int
        / 2 as libc::c_int) as libc::c_uint;
    g_slist_foreach(
        raw_frames,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_ushort, *mut *mut GSList) -> ()>,
            GFunc,
        >((*self_0).process_frame),
        &mut frames as *mut *mut GSList as gpointer,
    );
    fpi_do_movement_estimation(&mut assembling_ctx, frames);
    img = fpi_assemble_frames(&mut assembling_ctx, frames);
    (*img)
        .flags = ::core::mem::transmute::<
        libc::c_uint,
        FpiImageFlags,
    >((*img).flags as libc::c_uint | FPI_IMAGE_PARTIAL as libc::c_int as libc::c_uint);
    g_slist_free_full(frames, Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    fpi_image_device_image_captured(dev, img);
}
unsafe extern "C" fn elan_cmd_done(mut ssm: *mut FpiSsm) {
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"335\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"elan_cmd_done\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:335\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_next_state(ssm);
}
unsafe extern "C" fn elan_cmd_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut dev: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut ssm: *mut FpiSsm = (*transfer).ssm;
    let mut self_0: *mut FpiDeviceElan = FPI_DEVICE_ELAN(dev as gpointer);
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"346\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"elan_cmd_cb\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:346\0" as *const u8 as *const libc::c_char,
    );
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    if (*transfer).endpoint as libc::c_int & 0x80 as libc::c_int != 0 {
        (*self_0)
            .last_read = ({
            g_memdup2(
                (*transfer).buffer as gconstpointer,
                (*transfer).actual_length as gsize,
            )
        }) as *mut libc::c_uchar;
        elan_cmd_done(ssm);
    } else {
        g_log_structured(
            b"libfprint-elan\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"CODE_FILE\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
            b"CODE_LINE\0" as *const u8 as *const libc::c_char,
            b"366\0" as *const u8 as *const libc::c_char,
            b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"elan_cmd_cb\0"))
                .as_ptr(),
            b"MESSAGE\0" as *const u8 as *const libc::c_char,
            b"%li: %s\0" as *const u8 as *const libc::c_char,
            g_get_monotonic_time(),
            b"../libfprint/drivers/elan.c:366\0" as *const u8 as *const libc::c_char,
        );
        elan_cmd_read(ssm, dev);
    };
}
unsafe extern "C" fn elan_cmd_read(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceElan = FPI_DEVICE_ELAN(dev as gpointer);
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    let mut cancellable: *mut GCancellable = 0 as *mut GCancellable;
    let mut response_len: libc::c_int = (*(*self_0).cmd).response_len;
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"379\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"elan_cmd_read\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:379\0" as *const u8 as *const libc::c_char,
    );
    if (*(*self_0).cmd).response_len == 0 as libc::c_int {
        g_log(
            b"libfprint-elan\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"skipping read, not expecting anything\0" as *const u8
                as *const libc::c_char,
        );
        elan_cmd_done(ssm);
        return;
    }
    if (*self_0).dev_type as libc::c_int == (1 as libc::c_int) << 2 as libc::c_int {
        if (*(*self_0).cmd).response_len == 1 as libc::c_int {
            response_len = 2 as libc::c_int;
        }
    }
    if ((*(*self_0).cmd).cmd).as_ptr() == (get_image_cmd.cmd).as_ptr() {
        response_len = (*self_0).raw_frame_height as libc::c_int
            * (*self_0).frame_width as libc::c_int * 2 as libc::c_int;
    }
    let mut _pp: C2RustUnnamed_5 = C2RustUnnamed_5 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*self_0).last_read as *mut *mut libc::c_uchar as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    transfer = fpi_usb_transfer_new(dev);
    (*transfer).ssm = ssm;
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_fill_bulk(
        transfer,
        (*(*self_0).cmd).response_in as guint8,
        response_len as gsize,
    );
    if (*(*self_0).cmd).never_cancel == 0 {
        cancellable = fpi_device_get_cancellable(dev);
    }
    fpi_usb_transfer_submit(
        transfer,
        (*self_0).cmd_timeout as guint,
        cancellable,
        Some(
            elan_cmd_cb
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
unsafe extern "C" fn elan_run_cmd(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut cmd: *const elan_cmd,
    mut cmd_timeout: libc::c_int,
) {
    let mut self_0: *mut FpiDeviceElan = FPI_DEVICE_ELAN(dev as gpointer);
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    let mut cancellable: *mut GCancellable = 0 as *mut GCancellable;
    (*self_0).cmd = cmd;
    if cmd_timeout != -(1 as libc::c_int) {
        (*self_0).cmd_timeout = cmd_timeout;
    }
    if (*cmd).devices as libc::c_int != 0 as libc::c_int
        && (*cmd).devices as libc::c_int & (*self_0).dev_type as libc::c_int == 0
    {
        g_log(
            b"libfprint-elan\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"skipping command 0x%x 0x%x for this device (for devices 0x%x but device is 0x%x)\0"
                as *const u8 as *const libc::c_char,
            (*cmd).cmd[0 as libc::c_int as usize] as libc::c_int,
            (*cmd).cmd[1 as libc::c_int as usize] as libc::c_int,
            (*cmd).devices as libc::c_int,
            (*self_0).dev_type as libc::c_int,
        );
        elan_cmd_done(ssm);
        return;
    }
    transfer = fpi_usb_transfer_new(dev);
    (*transfer).ssm = ssm;
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_fill_bulk_full(
        transfer,
        (0x1 as libc::c_int | 0 as libc::c_int) as guint8,
        ((*cmd).cmd).as_ptr() as *mut guint8,
        0x2 as libc::c_int as gsize,
        None,
    );
    if (*(*self_0).cmd).never_cancel == 0 {
        cancellable = fpi_device_get_cancellable(dev);
    }
    fpi_usb_transfer_submit(
        transfer,
        (*self_0).cmd_timeout as guint,
        cancellable,
        Some(
            elan_cmd_cb
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
unsafe extern "C" fn stop_capture_run_state(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
) {
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"466\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 23],
            &[libc::c_char; 23],
        >(b"stop_capture_run_state\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:466\0" as *const u8 as *const libc::c_char,
    );
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            elan_run_cmd(ssm, dev, &stop_cmd, 10000 as libc::c_int);
        }
        _ => {}
    };
}
unsafe extern "C" fn stop_capture_complete(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceElan = FPI_DEVICE_ELAN(dev as gpointer);
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"483\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"stop_capture_complete\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:483\0" as *const u8 as *const libc::c_char,
    );
    (*self_0).active = 0 as libc::c_int;
    if (*self_0).deactivating != 0 {
        (*self_0).deactivating = 0 as libc::c_int;
        fpi_image_device_deactivate_complete(dev, error);
        return;
    }
    if error.is_null() {
        fpi_image_device_report_finger_status(dev, 0 as libc::c_int);
    } else {
        fpi_image_device_session_error(dev, error);
    };
}
unsafe extern "C" fn elan_stop_capture(mut self_0: *mut FpiDeviceElan) {
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"507\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"elan_stop_capture\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:507\0" as *const u8 as *const libc::c_char,
    );
    elan_dev_reset_state(self_0);
    let mut ssm: *mut FpiSsm = fpi_ssm_new_full(
        FP_DEVICE(self_0 as gpointer),
        Some(
            stop_capture_run_state
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        STOP_CAPTURE_NUM_STATES as libc::c_int,
        STOP_CAPTURE_NUM_STATES as libc::c_int,
        b"STOP_CAPTURE_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            stop_capture_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn capture_run_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut idev: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    let mut self_0: *mut FpiDeviceElan = FPI_DEVICE_ELAN(dev as gpointer);
    let mut r: libc::c_int = 0;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            elan_run_cmd(ssm, dev, &led_on_cmd, 10000 as libc::c_int);
        }
        1 => {
            elan_run_cmd(ssm, dev, &pre_scan_cmd, -(1 as libc::c_int));
        }
        2 => {
            if !((*self_0).last_read).is_null()
                && *((*self_0).last_read).offset(0 as libc::c_int as isize)
                    as libc::c_int == 0x55 as libc::c_int
            {
                fpi_image_device_report_finger_status(
                    idev,
                    (0 as libc::c_int == 0) as libc::c_int,
                );
                elan_run_cmd(ssm, dev, &get_image_cmd, 10000 as libc::c_int);
            } else if g_strcmp0(
                g_getenv(b"FP_DEVICE_EMULATION\0" as *const u8 as *const libc::c_char),
                b"1\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                fpi_ssm_mark_completed(ssm);
            } else {
                fpi_ssm_mark_failed(ssm, fpi_device_error_new(FP_DEVICE_ERROR_PROTO));
            }
        }
        3 => {
            r = elan_save_img_frame(self_0);
            if r < 0 as libc::c_int {
                fpi_ssm_mark_failed(ssm, fpi_device_error_new(FP_DEVICE_ERROR_GENERAL));
            } else if (*self_0).num_frames < 30 as libc::c_int {
                (*self_0).cmd_timeout = 200 as libc::c_int;
                fpi_ssm_jump_to_state(ssm, CAPTURE_WAIT_FINGER as libc::c_int);
            } else {
                fpi_ssm_next_state(ssm);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn capture_complete(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceElan = FPI_DEVICE_ELAN(_dev as gpointer);
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"586\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"capture_complete\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:586\0" as *const u8 as *const libc::c_char,
    );
    if error.is_null()
        || g_error_matches(
            error,
            g_usb_device_error_quark(),
            G_USB_DEVICE_ERROR_TIMED_OUT as libc::c_int,
        ) != 0 && fpi_ssm_get_cur_state(ssm) == CAPTURE_WAIT_FINGER as libc::c_int
    {
        if (*self_0).num_frames >= 7 as libc::c_int {
            elan_submit_image(dev);
        } else {
            g_log(
                b"libfprint-elan\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"swipe too short: want >= %d frames, got %d\0" as *const u8
                    as *const libc::c_char,
                7 as libc::c_int,
                (*self_0).num_frames,
            );
            fpi_image_device_retry_scan(dev, FP_DEVICE_RETRY_TOO_SHORT);
        }
        g_clear_error(&mut error);
    } else {
        fpi_image_device_session_error(dev, error);
    }
    elan_stop_capture(self_0);
}
unsafe extern "C" fn elan_capture(mut self_0: *mut FpiDeviceElan) {
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"620\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"elan_capture\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:620\0" as *const u8 as *const libc::c_char,
    );
    elan_dev_reset_state(self_0);
    let mut ssm: *mut FpiSsm = fpi_ssm_new_full(
        FP_DEVICE(self_0 as gpointer),
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
            capture_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn elan_need_calibration(
    mut elandev: *mut FpiDeviceElan,
) -> libc::c_int {
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"634\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"elan_need_calibration\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:634\0" as *const u8 as *const libc::c_char,
    );
    let mut calib_mean: libc::c_ushort = (*((*elandev).last_read)
        .offset(0 as libc::c_int as isize) as libc::c_int * 0xff as libc::c_int
        + *((*elandev).last_read).offset(1 as libc::c_int as isize) as libc::c_int)
        as libc::c_ushort;
    let mut bg_mean: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut delta: libc::c_uint = 0;
    let mut frame_size: libc::c_uint = ((*elandev).frame_width as libc::c_int
        * (*elandev).frame_height as libc::c_int) as libc::c_uint;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if frame_size != 0 as libc::c_int as libc::c_uint {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-elan\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
            641 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"elan_need_calibration\0"))
                .as_ptr(),
            b"frame_size != 0\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*elandev).dev_type as libc::c_int == (1 as libc::c_int) << 2 as libc::c_int {
        if calib_mean as libc::c_int > 5500 as libc::c_int
            || (calib_mean as libc::c_int) < 2500 as libc::c_int
        {
            g_log(
                b"libfprint-elan\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Forcing needed recalibration\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < frame_size {
        bg_mean = bg_mean
            .wrapping_add(*((*elandev).background).offset(i as isize) as libc::c_uint);
        i += 1;
    }
    bg_mean = bg_mean.wrapping_div(frame_size);
    delta = if bg_mean > calib_mean as libc::c_uint {
        bg_mean.wrapping_sub(calib_mean as libc::c_uint)
    } else {
        (calib_mean as libc::c_uint).wrapping_sub(bg_mean)
    };
    g_log(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"calibration mean: %d, bg mean: %d, delta: %d\0" as *const u8
            as *const libc::c_char,
        calib_mean as libc::c_int,
        bg_mean,
        delta,
    );
    return if delta > 500 as libc::c_int as libc::c_uint {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn elan_supports_calibration(
    mut elandev: *mut FpiDeviceElan,
) -> gboolean {
    if (*elandev).dev_type as libc::c_int == (1 as libc::c_int) << 2 as libc::c_int {
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    return ((*elandev).fw_ver as libc::c_int >= 0x138 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn calibrate_run_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceElan = FPI_DEVICE_ELAN(dev as gpointer);
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"691\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"calibrate_run_state\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:691\0" as *const u8 as *const libc::c_char,
    );
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            elan_run_cmd(ssm, dev, &get_image_cmd, 10000 as libc::c_int);
        }
        1 => {
            elan_save_background(self_0);
            if elan_supports_calibration(self_0) == 0 {
                g_log(
                    b"libfprint-elan\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"FW does not support calibration\0" as *const u8
                        as *const libc::c_char,
                );
                fpi_ssm_mark_completed(ssm);
            } else {
                fpi_ssm_next_state(ssm);
            }
        }
        2 => {
            elan_run_cmd(ssm, dev, &get_calib_mean_cmd, 10000 as libc::c_int);
        }
        3 => {
            if elan_need_calibration(self_0) != 0 {
                (*self_0).calib_status = 0 as libc::c_int as libc::c_uchar;
                fpi_ssm_next_state(ssm);
            } else {
                fpi_ssm_mark_completed(ssm);
            }
        }
        4 => {
            (*self_0)
                .calib_atts_left = ((*self_0).calib_atts_left as libc::c_int
                - 1 as libc::c_int) as libc::c_uchar;
            if (*self_0).calib_atts_left != 0 {
                elan_run_cmd(ssm, dev, &get_calib_status_cmd, 10000 as libc::c_int);
            } else {
                g_log(
                    b"libfprint-elan\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"calibration failed\0" as *const u8 as *const libc::c_char,
                );
                fpi_ssm_mark_failed(
                    ssm,
                    fpi_device_error_new_msg(
                        FP_DEVICE_ERROR_GENERAL,
                        b"Calibration failed!\0" as *const u8 as *const libc::c_char,
                    ),
                );
            }
        }
        5 => {
            g_log(
                b"libfprint-elan\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"calibration status: 0x%02x\0" as *const u8 as *const libc::c_char,
                *((*self_0).last_read).offset(0 as libc::c_int as isize) as libc::c_int,
            );
            if (*self_0).calib_status as libc::c_int == 0x1 as libc::c_int
                && *((*self_0).last_read).offset(0 as libc::c_int as isize)
                    as libc::c_int == 0x3 as libc::c_int
            {
                (*self_0).calib_status = 0x3 as libc::c_int as libc::c_uchar;
                fpi_ssm_jump_to_state(ssm, CALIBRATE_GET_BACKGROUND as libc::c_int);
            } else {
                if (*self_0).calib_status as libc::c_int == 0 as libc::c_int
                    && *((*self_0).last_read).offset(0 as libc::c_int as isize)
                        as libc::c_int == 0x1 as libc::c_int
                {
                    (*self_0).calib_status = 0x1 as libc::c_int as libc::c_uchar;
                }
                fpi_ssm_next_state_delayed(ssm, 50 as libc::c_int);
            }
        }
        6 => {
            fpi_ssm_jump_to_state(ssm, CALIBRATE_GET_STATUS as libc::c_int);
        }
        _ => {}
    };
}
unsafe extern "C" fn calibrate_complete(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"776\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"calibrate_complete\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:776\0" as *const u8 as *const libc::c_char,
    );
    if !error.is_null() {
        fpi_image_device_session_error(FP_IMAGE_DEVICE(dev as gpointer), error);
        elan_stop_capture(FPI_DEVICE_ELAN(dev as gpointer));
    } else {
        elan_capture(FPI_DEVICE_ELAN(dev as gpointer));
    };
}
unsafe extern "C" fn elan_calibrate(mut self_0: *mut FpiDeviceElan) {
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"792\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"elan_calibrate\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:792\0" as *const u8 as *const libc::c_char,
    );
    elan_dev_reset_state(self_0);
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
        g_return_if_fail_warning(
            b"libfprint-elan\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"elan_calibrate\0"))
                .as_ptr(),
            b"!self->active\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*self_0).active = (0 as libc::c_int == 0) as libc::c_int;
    (*self_0).calib_atts_left = 10 as libc::c_int as libc::c_uchar;
    let mut ssm: *mut FpiSsm = fpi_ssm_new_full(
        FP_DEVICE(self_0 as gpointer),
        Some(
            calibrate_run_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        CALIBRATE_NUM_STATES as libc::c_int,
        CALIBRATE_NUM_STATES as libc::c_int,
        b"CALIBRATE_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            calibrate_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn activate_run_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceElan = FPI_DEVICE_ELAN(dev as gpointer);
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"820\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"activate_run_state\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:820\0" as *const u8 as *const libc::c_char,
    );
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            elan_run_cmd(ssm, dev, &get_fw_ver_cmd, 10000 as libc::c_int);
        }
        1 => {
            (*self_0)
                .fw_ver = ((*((*self_0).last_read).offset(0 as libc::c_int as isize)
                as libc::c_int) << 8 as libc::c_int
                | *((*self_0).last_read).offset(1 as libc::c_int as isize)
                    as libc::c_int) as libc::c_ushort;
            g_log(
                b"libfprint-elan\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"FW ver 0x%04hx\0" as *const u8 as *const libc::c_char,
                (*self_0).fw_ver as libc::c_int,
            );
            fpi_ssm_next_state(ssm);
        }
        2 => {
            elan_run_cmd(ssm, dev, &get_sensor_dim_cmd, 10000 as libc::c_int);
        }
        3 => {
            if (*self_0).dev_type as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int
                != 0
            {
                (*self_0)
                    .frame_width = *((*self_0).last_read)
                    .offset(0 as libc::c_int as isize);
                (*self_0)
                    .raw_frame_height = *((*self_0).last_read)
                    .offset(2 as libc::c_int as isize);
                (*self_0).frame_height = (*self_0).raw_frame_height;
            } else {
                (*self_0)
                    .frame_width = *((*self_0).last_read)
                    .offset(2 as libc::c_int as isize);
                (*self_0)
                    .raw_frame_height = *((*self_0).last_read)
                    .offset(0 as libc::c_int as isize);
                (*self_0).frame_height = (*self_0).raw_frame_height;
            }
            if (*self_0).frame_width as libc::c_int % 2 as libc::c_int
                == 1 as libc::c_int
                && (*self_0).frame_height as libc::c_int % 2 as libc::c_int
                    == 1 as libc::c_int
            {
                (*self_0).frame_width = ((*self_0).frame_width).wrapping_add(1);
                (*self_0).frame_height = ((*self_0).frame_height).wrapping_add(1);
                (*self_0).raw_frame_height = (*self_0).frame_height;
            }
            if (*self_0).frame_height as libc::c_int > 50 as libc::c_int {
                (*self_0).frame_height = 50 as libc::c_int as libc::c_uchar;
            }
            g_log(
                b"libfprint-elan\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"sensor dimensions, WxH: %dx%d\0" as *const u8 as *const libc::c_char,
                (*self_0).frame_width as libc::c_int,
                (*self_0).raw_frame_height as libc::c_int,
            );
            fpi_ssm_next_state(ssm);
        }
        4 => {
            elan_run_cmd(ssm, dev, &activate_cmd_1, 10000 as libc::c_int);
        }
        _ => {}
    };
}
unsafe extern "C" fn activate_complete(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut idev: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"881\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"activate_complete\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:881\0" as *const u8 as *const libc::c_char,
    );
    fpi_image_device_activate_complete(idev, error);
}
unsafe extern "C" fn elan_activate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceElan = FPI_DEVICE_ELAN(dev as gpointer);
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"892\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"elan_activate\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:892\0" as *const u8 as *const libc::c_char,
    );
    elan_dev_reset_state(self_0);
    let mut ssm: *mut FpiSsm = fpi_ssm_new_full(
        FP_DEVICE(dev as gpointer),
        Some(
            activate_run_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        ACTIVATE_NUM_STATES as libc::c_int,
        ACTIVATE_NUM_STATES as libc::c_int,
        b"ACTIVATE_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            activate_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn dev_init(mut dev: *mut FpImageDevice) {
    let mut error: *mut GError = 0 as *mut GError;
    let mut self_0: *mut FpiDeviceElan = 0 as *mut FpiDeviceElan;
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"908\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"dev_init\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:908\0" as *const u8 as *const libc::c_char,
    );
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
    self_0 = FPI_DEVICE_ELAN(dev as gpointer);
    (*self_0)
        .dev_type = fpi_device_get_driver_data(FP_DEVICE(dev as gpointer))
        as libc::c_ushort;
    (*self_0).background = 0 as *mut libc::c_ushort;
    (*self_0)
        .process_frame = Some(
        elan_process_frame_thirds
            as unsafe extern "C" fn(*mut libc::c_ushort, *mut *mut GSList) -> (),
    );
    match (*self_0).dev_type as libc::c_int {
        1 => {
            (*self_0)
                .process_frame = Some(
                elan_process_frame_linear
                    as unsafe extern "C" fn(*mut libc::c_ushort, *mut *mut GSList) -> (),
            );
        }
        _ => {}
    }
    fpi_image_device_open_complete(dev, 0 as *mut GError);
}
unsafe extern "C" fn dev_deinit(mut dev: *mut FpImageDevice) {
    let mut error: *mut GError = 0 as *mut GError;
    let mut self_0: *mut FpiDeviceElan = FPI_DEVICE_ELAN(dev as gpointer);
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"939\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"dev_deinit\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:939\0" as *const u8 as *const libc::c_char,
    );
    elan_dev_reset_state(self_0);
    g_free((*self_0).background as gpointer);
    g_usb_device_release_interface(
        fpi_device_get_usb_device(FP_DEVICE(dev as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    );
    fpi_image_device_close_complete(dev, error);
}
unsafe extern "C" fn dev_activate(mut dev: *mut FpImageDevice) {
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"951\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"dev_activate\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:951\0" as *const u8 as *const libc::c_char,
    );
    elan_activate(dev);
}
unsafe extern "C" fn dev_change_state(
    mut dev: *mut FpImageDevice,
    mut state: FpiImageDeviceState,
) {
    let mut self_0: *mut FpiDeviceElan = FPI_DEVICE_ELAN(dev as gpointer);
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"960\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"dev_change_state\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:960\0" as *const u8 as *const libc::c_char,
    );
    if state as libc::c_uint
        == FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_ON as libc::c_int as libc::c_uint
    {
        elan_calibrate(self_0);
    }
}
unsafe extern "C" fn dev_deactivate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceElan = FPI_DEVICE_ELAN(dev as gpointer);
    g_log_structured(
        b"libfprint-elan\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elan.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"975\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"dev_deactivate\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elan.c:975\0" as *const u8 as *const libc::c_char,
    );
    if (*self_0).active == 0 {
        fpi_image_device_deactivate_complete(dev, 0 as *mut GError);
    } else {
        (*self_0).deactivating = (0 as libc::c_int == 0) as libc::c_int;
    };
}
unsafe extern "C" fn fpi_device_elan_init(mut self_0: *mut FpiDeviceElan) {}
unsafe extern "C" fn fpi_device_elan_class_init(mut klass: *mut FpiDeviceElanClass) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    let mut img_class: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_CLASS(
        klass as gpointer,
    );
    (*dev_class).id = b"elan\0" as *const u8 as *const libc::c_char;
    (*dev_class)
        .full_name = b"ElanTech Fingerprint Sensor\0" as *const u8
        as *const libc::c_char;
    (*dev_class).type_0 = FP_DEVICE_TYPE_USB;
    (*dev_class).id_table = elan_id_table.as_ptr();
    (*dev_class).scan_type = FP_SCAN_TYPE_SWIPE;
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
    (*img_class)
        .change_state = Some(
        dev_change_state
            as unsafe extern "C" fn(*mut FpImageDevice, FpiImageDeviceState) -> (),
    );
    (*img_class).bz3_threshold = 24 as libc::c_int;
}
