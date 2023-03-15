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
    fn g_byte_array_new() -> *mut GByteArray;
    fn g_byte_array_unref(array: *mut GByteArray);
    fn g_byte_array_append(
        array: *mut GByteArray,
        data: *const guint8,
        len: guint,
    ) -> *mut GByteArray;
    fn g_byte_array_set_size(array: *mut GByteArray, length: guint) -> *mut GByteArray;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_slist_free(list: *mut GSList);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_reverse(list: *mut GSList) -> *mut GSList;
    fn g_slist_foreach(list: *mut GSList, func: GFunc, user_data: gpointer);
    fn g_get_monotonic_time() -> gint64;
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn g_log_structured(log_domain: *const gchar, log_level: GLogLevelFlags, _: ...);
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
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
    fn fpi_assemble_frames(
        ctx: *mut fpi_frame_asmbl_ctx,
        stripes: *mut GSList,
    ) -> *mut FpImage;
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
    fn fpi_device_error_new_msg(
        error: FpDeviceError,
        msg: *const gchar,
        _: ...
    ) -> *mut GError;
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
pub type __int8_t = libc::c_schar;
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
pub type GFunc = Option::<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GByteArray {
    pub data: *mut guint8,
    pub len: guint,
}
pub type GByteArray = _GByteArray;
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
pub type int8_t = __int8_t;
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
pub type FpiDeviceAesX660 = _FpiDeviceAesX660;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceAesX660 {
    pub parent_instance: FpImageDevice,
}
pub type FpiDeviceAesX660Class = _FpiDeviceAesX660Class;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceAesX660Class {
    pub parent: FpImageDeviceClass,
    pub init_seqs: [*mut aesX660_cmd; 2],
    pub init_seqs_len: [gsize; 2],
    pub start_imaging_cmd: *mut guint8,
    pub start_imaging_cmd_len: gsize,
    pub assembling_ctx: *mut fpi_frame_asmbl_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aesX660_cmd {
    pub cmd: *const guint8,
    pub len: gsize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiDeviceAesX660Private {
    pub stripe_packet: *mut GByteArray,
    pub strips: *mut GSList,
    pub strips_len: size_t,
    pub deactivating: gboolean,
    pub init_seq: *mut aesX660_cmd,
    pub init_seq_len: size_t,
    pub init_cmd_idx: libc::c_uint,
    pub init_seq_idx: libc::c_uint,
}
pub const CAPTURE_NUM_STATES: capture_states = 4;
pub const CAPTURE_SET_IDLE: capture_states = 3;
pub const CAPTURE_READ_STRIPE_DATA: capture_states = 2;
pub const CAPTURE_SEND_CAPTURE_CMD: capture_states = 1;
pub const CAPTURE_SEND_LED_CMD: capture_states = 0;
pub const FINGER_DET_NUM_STATES: finger_det_states = 4;
pub const FINGER_DET_SET_IDLE: finger_det_states = 3;
pub const FINGER_DET_SEND_FD_CMD: finger_det_states = 1;
pub const FINGER_DET_READ_FD_DATA: finger_det_states = 2;
pub const FINGER_DET_SEND_LED_CMD: finger_det_states = 0;
pub const ACTIVATE_NUM_STATES: activate_states = 7;
pub const ACTIVATE_READ_CALIBRATE_DATA: activate_states = 4;
pub const ACTIVATE_SEND_CALIBRATE_CMD: activate_states = 3;
pub const ACTIVATE_SEND_INIT_CMD: activate_states = 5;
pub const ACTIVATE_SEND_READ_ID_CMD: activate_states = 1;
pub const ACTIVATE_READ_INIT_RESPONSE: activate_states = 6;
pub const ACTIVATE_READ_ID: activate_states = 2;
pub const ACTIVATE_SET_IDLE: activate_states = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub type finger_det_states = libc::c_uint;
pub type capture_states = libc::c_uint;
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
unsafe extern "C" fn FPI_DEVICE_AES_X660_GET_CLASS(
    mut ptr: gpointer,
) -> *mut FpiDeviceAesX660Class {
    return (*(ptr as *mut GTypeInstance)).g_class as *mut FpiDeviceAesX660Class;
}
#[inline]
unsafe extern "C" fn FPI_DEVICE_AES_X660(mut ptr: gpointer) -> *mut FpiDeviceAesX660 {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_aes_x660_get_type(),
    ) as *mut libc::c_void as *mut FpiDeviceAesX660;
}
static mut led_blink_cmd: [guint8; 27] = [
    0x77 as libc::c_int as guint8,
    0x18 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x3f as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0xff as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x1 as libc::c_int as guint8,
    0x1 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0xf3 as libc::c_int as guint8,
    0x1 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x1 as libc::c_int as guint8,
    0x1 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0xf3 as libc::c_int as guint8,
    0x1 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x7f as libc::c_int as guint8,
];
static mut led_solid_cmd: [guint8; 27] = [
    0x77 as libc::c_int as guint8,
    0x18 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x3f as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0xff as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x1 as libc::c_int as guint8,
    0x1 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0xe7 as libc::c_int as guint8,
    0x3 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x1 as libc::c_int as guint8,
    0x1 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x7f as libc::c_int as guint8,
];
static mut wait_for_finger_cmd: [guint8; 8] = [
    0x20 as libc::c_int as guint8,
    0x40 as libc::c_int as guint8,
    0x4 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x2 as libc::c_int as guint8,
    0x1e as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x32 as libc::c_int as guint8,
];
static mut set_idle_cmd: [guint8; 1] = [0xd as libc::c_int as guint8];
static mut read_id_cmd: [guint8; 6] = [
    0x44 as libc::c_int as guint8,
    0x2 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x8 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x7 as libc::c_int as guint8,
];
static mut calibrate_cmd: [guint8; 6] = [
    0x44 as libc::c_int as guint8,
    0x2 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x4 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x6 as libc::c_int as guint8,
];
#[inline]
unsafe extern "C" fn fpi_device_aes_x660_get_instance_private(
    mut self_0: *mut FpiDeviceAesX660,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(FpiDeviceAesX660_private_offset as glong as isize) as gpointer;
}
static mut FpiDeviceAesX660_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn fpi_device_aes_x660_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_aes_x660_get_type_once();
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
unsafe extern "C" fn fpi_device_aes_x660_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_image_device_get_type(),
        g_intern_static_string(
            b"FpiDeviceAesX660\0" as *const u8 as *const libc::c_char,
        ),
        ::core::mem::size_of::<FpiDeviceAesX660Class>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_aes_x660_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpiDeviceAesX660>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpiDeviceAesX660) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_aes_x660_init
                        as unsafe extern "C" fn(*mut FpiDeviceAesX660) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    FpiDeviceAesX660_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<FpiDeviceAesX660Private>() as libc::c_ulong,
    );
    return g_define_type_id;
}
unsafe extern "C" fn fpi_device_aes_x660_class_intern_init(mut klass: gpointer) {
    fpi_device_aes_x660_parent_class = g_type_class_peek_parent(klass);
    if FpiDeviceAesX660_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpiDeviceAesX660_private_offset);
    }
    fpi_device_aes_x660_class_init(klass as *mut FpiDeviceAesX660Class);
}
static mut fpi_device_aes_x660_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
unsafe extern "C" fn aesX660_send_cmd_timeout(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
    mut cmd: *const libc::c_uchar,
    mut cmd_len: size_t,
    mut callback: FpiUsbTransferCallback,
    mut timeout: libc::c_int,
) {
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(_dev);
    fpi_usb_transfer_fill_bulk_full(
        transfer,
        (2 as libc::c_int | 0 as libc::c_int) as guint8,
        cmd as *mut libc::c_uchar,
        cmd_len,
        None,
    );
    (*transfer).ssm = ssm;
    fpi_usb_transfer_submit(
        transfer,
        timeout as guint,
        0 as *mut GCancellable,
        callback,
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn aesX660_send_cmd(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut cmd: *const libc::c_uchar,
    mut cmd_len: size_t,
    mut callback: FpiUsbTransferCallback,
) {
    return aesX660_send_cmd_timeout(
        ssm,
        dev,
        cmd,
        cmd_len,
        callback,
        4000 as libc::c_int,
    );
}
unsafe extern "C" fn aesX660_read_response(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
    mut short_is_error: gboolean,
    mut cancellable: gboolean,
    mut buf_len: size_t,
    mut callback: FpiUsbTransferCallback,
) {
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(_dev);
    let mut cancel: *mut GCancellable = 0 as *mut GCancellable;
    if cancellable != 0 {
        cancel = fpi_device_get_cancellable(_dev);
    }
    fpi_usb_transfer_fill_bulk(
        transfer,
        (1 as libc::c_int | 0x80 as libc::c_int) as guint8,
        buf_len,
    );
    (*transfer).ssm = ssm;
    (*transfer).short_is_error = short_is_error;
    fpi_usb_transfer_submit(
        transfer,
        4000 as libc::c_int as guint,
        cancel,
        callback,
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn aesX660_read_calibrate_data_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut data: *mut libc::c_uchar = (*transfer).buffer;
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    if *data.offset(0 as libc::c_int as isize) as libc::c_int != 0x6 as libc::c_int {
        g_log(
            b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Bogus calibrate response: %.2x\0" as *const u8 as *const libc::c_char,
            *data.offset(0 as libc::c_int as isize) as libc::c_int,
        );
        fpi_ssm_mark_failed(
            (*transfer).ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Bogus calibrate response: %.2x\0" as *const u8 as *const libc::c_char,
                *data.offset(0 as libc::c_int as isize) as libc::c_int,
            ),
        );
        return;
    }
    fpi_ssm_next_state((*transfer).ssm);
}
unsafe extern "C" fn finger_det_read_fd_data_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceAesX660 = FPI_DEVICE_AES_X660(device as gpointer);
    let mut priv_0: *mut FpiDeviceAesX660Private = fpi_device_aes_x660_get_instance_private(
        self_0,
    ) as *mut FpiDeviceAesX660Private;
    let mut data: *mut libc::c_uchar = (*transfer).buffer;
    if g_error_matches(error, g_io_error_quark(), G_IO_ERROR_CANCELLED as libc::c_int)
        != 0
    {
        fpi_ssm_next_state((*transfer).ssm);
        return;
    }
    if !error.is_null() {
        g_log(
            b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Failed to read FD data\0" as *const u8 as *const libc::c_char,
        );
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    if *data.offset(0 as libc::c_int as isize) as libc::c_int != 0x40 as libc::c_int {
        g_log(
            b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Bogus FD response: %.2x\0" as *const u8 as *const libc::c_char,
            *data.offset(0 as libc::c_int as isize) as libc::c_int,
        );
        fpi_ssm_mark_failed(
            (*transfer).ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Bogus FD response %.2x\0" as *const u8 as *const libc::c_char,
                *data.offset(0 as libc::c_int as isize) as libc::c_int,
            ),
        );
        return;
    }
    if *data.offset(0x3 as libc::c_int as isize) as libc::c_int == 0x1 as libc::c_int
        || (*priv_0).deactivating != 0
    {
        fpi_ssm_next_state((*transfer).ssm);
    } else {
        g_log(
            b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Wait for finger returned %.2x as result\0" as *const u8
                as *const libc::c_char,
            *data.offset(0x3 as libc::c_int as isize) as libc::c_int,
        );
        fpi_ssm_jump_to_state((*transfer).ssm, FINGER_DET_SEND_FD_CMD as libc::c_int);
    };
}
unsafe extern "C" fn finger_det_set_idle_cmd_cb(
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
unsafe extern "C" fn finger_det_sm_complete(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceAesX660 = FPI_DEVICE_AES_X660(_dev as gpointer);
    let mut priv_0: *mut FpiDeviceAesX660Private = fpi_device_aes_x660_get_instance_private(
        self_0,
    ) as *mut FpiDeviceAesX660Private;
    g_log(
        b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Finger detection completed\0" as *const u8 as *const libc::c_char,
    );
    fpi_image_device_report_finger_status(dev, (0 as libc::c_int == 0) as libc::c_int);
    if (*priv_0).deactivating != 0 {
        complete_deactivation(dev);
        if !error.is_null() {
            g_error_free(error);
        }
    } else if !error.is_null() {
        fpi_image_device_session_error(dev, error);
    } else {
        fpi_image_device_report_finger_status(
            dev,
            (0 as libc::c_int == 0) as libc::c_int,
        );
        start_capture(dev);
    };
}
unsafe extern "C" fn finger_det_run_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            aesX660_send_cmd(
                ssm,
                dev,
                led_blink_cmd.as_ptr(),
                ::core::mem::size_of::<[guint8; 27]>() as libc::c_ulong,
                Some(
                    fpi_ssm_usb_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        1 => {
            aesX660_send_cmd_timeout(
                ssm,
                dev,
                wait_for_finger_cmd.as_ptr(),
                ::core::mem::size_of::<[guint8; 8]>() as libc::c_ulong,
                Some(
                    fpi_ssm_usb_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                0 as libc::c_int,
            );
        }
        2 => {
            aesX660_read_response(
                ssm,
                dev,
                (0 as libc::c_int == 0) as libc::c_int,
                (0 as libc::c_int == 0) as libc::c_int,
                4 as libc::c_int as size_t,
                Some(
                    finger_det_read_fd_data_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        3 => {
            aesX660_send_cmd(
                ssm,
                dev,
                set_idle_cmd.as_ptr(),
                ::core::mem::size_of::<[guint8; 1]>() as libc::c_ulong,
                Some(
                    finger_det_set_idle_cmd_cb
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
unsafe extern "C" fn start_finger_detection(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceAesX660 = FPI_DEVICE_AES_X660(dev as gpointer);
    let mut priv_0: *mut FpiDeviceAesX660Private = fpi_device_aes_x660_get_instance_private(
        self_0,
    ) as *mut FpiDeviceAesX660Private;
    let mut ssm: *mut FpiSsm = 0 as *mut FpiSsm;
    if (*priv_0).deactivating != 0 {
        complete_deactivation(dev);
        return;
    }
    ssm = fpi_ssm_new_full(
        FP_DEVICE(dev as gpointer),
        Some(
            finger_det_run_state
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        FINGER_DET_NUM_STATES as libc::c_int,
        FINGER_DET_NUM_STATES as libc::c_int,
        b"FINGER_DET_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            finger_det_sm_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn process_stripe_data(
    mut ssm: *mut FpiSsm,
    mut self_0: *mut FpiDeviceAesX660,
    mut data: *mut libc::c_uchar,
    mut length: gsize,
) -> libc::c_int {
    let mut priv_0: *mut FpiDeviceAesX660Private = fpi_device_aes_x660_get_instance_private(
        self_0,
    ) as *mut FpiDeviceAesX660Private;
    let mut cls: *mut FpiDeviceAesX660Class = FPI_DEVICE_AES_X660_GET_CLASS(
        self_0 as gpointer,
    );
    let mut stripe: *mut fpi_frame = 0 as *mut fpi_frame;
    let mut stripdata: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if length
        < (43 as libc::c_int as libc::c_uint)
            .wrapping_add(
                ((*(*cls).assembling_ctx).frame_width)
                    .wrapping_mul(8 as libc::c_int as libc::c_uint)
                    .wrapping_div(2 as libc::c_int as libc::c_uint),
            ) as libc::c_ulong
    {
        g_log(
            b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Received stripe data is too short, got %zi expected %i bytes!\0"
                as *const u8 as *const libc::c_char,
            length,
            (43 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    ((*(*cls).assembling_ctx).frame_width)
                        .wrapping_mul(8 as libc::c_int as libc::c_uint)
                        .wrapping_div(2 as libc::c_int as libc::c_uint),
                ),
        );
        return 0 as libc::c_int;
    }
    stripe = g_malloc(
        (((*(*cls).assembling_ctx).frame_width)
            .wrapping_mul(8 as libc::c_int as libc::c_uint)
            .wrapping_div(2 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<fpi_frame>() as libc::c_ulong),
    ) as *mut fpi_frame;
    stripdata = ((*stripe).data).as_mut_ptr();
    g_log(
        b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Processing frame %.2x %.2x\0" as *const u8 as *const libc::c_char,
        *data.offset(0x3 as libc::c_int as isize) as libc::c_int,
        *data.offset(0x4 as libc::c_int as isize) as libc::c_int,
    );
    (*stripe)
        .delta_x = *data.offset(16 as libc::c_int as isize) as int8_t as libc::c_int;
    (*stripe)
        .delta_y = -(*data.offset(17 as libc::c_int as isize) as int8_t as libc::c_int);
    g_log(
        b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Offset to previous frame: %d %d\0" as *const u8 as *const libc::c_char,
        (*stripe).delta_x,
        (*stripe).delta_y,
    );
    if *data.offset(0x3 as libc::c_int as isize) as libc::c_int == 0xd as libc::c_int {
        memcpy(
            stripdata as *mut libc::c_void,
            data.offset(43 as libc::c_int as isize) as *const libc::c_void,
            ((*(*cls).assembling_ctx).frame_width)
                .wrapping_mul(8 as libc::c_int as libc::c_uint)
                .wrapping_div(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
        );
        (*priv_0).strips = g_slist_prepend((*priv_0).strips, stripe as gpointer);
        (*priv_0).strips_len = ((*priv_0).strips_len).wrapping_add(1);
        return *data.offset(0x4 as libc::c_int as isize) as libc::c_int
            & 0x1 as libc::c_int;
    }
    g_free(stripe as gpointer);
    return 0 as libc::c_int;
}
unsafe extern "C" fn capture_set_idle_cmd_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(device as gpointer);
    let mut self_0: *mut FpiDeviceAesX660 = FPI_DEVICE_AES_X660(device as gpointer);
    let mut priv_0: *mut FpiDeviceAesX660Private = fpi_device_aes_x660_get_instance_private(
        self_0,
    ) as *mut FpiDeviceAesX660Private;
    let mut cls: *mut FpiDeviceAesX660Class = FPI_DEVICE_AES_X660_GET_CLASS(
        self_0 as gpointer,
    );
    if error.is_null() {
        let mut img: *mut FpImage = 0 as *mut FpImage;
        (*priv_0).strips = g_slist_reverse((*priv_0).strips);
        img = fpi_assemble_frames((*cls).assembling_ctx, (*priv_0).strips);
        (*img)
            .flags = ::core::mem::transmute::<
            libc::c_uint,
            FpiImageFlags,
        >(
            (*img).flags as libc::c_uint
                | FPI_IMAGE_PARTIAL as libc::c_int as libc::c_uint,
        );
        g_slist_foreach(
            (*priv_0).strips,
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                GFunc,
            >(Some(g_free as unsafe extern "C" fn(gpointer) -> ())),
            0 as *mut libc::c_void,
        );
        g_slist_free((*priv_0).strips);
        (*priv_0).strips = 0 as *mut GSList;
        (*priv_0).strips_len = 0 as libc::c_int as size_t;
        fpi_image_device_image_captured(dev, img);
        fpi_image_device_report_finger_status(dev, 0 as libc::c_int);
        fpi_ssm_mark_completed((*transfer).ssm);
    } else {
        fpi_ssm_mark_failed((*transfer).ssm, error);
    };
}
unsafe extern "C" fn capture_read_stripe_data_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceAesX660 = FPI_DEVICE_AES_X660(device as gpointer);
    let mut priv_0: *mut FpiDeviceAesX660Private = fpi_device_aes_x660_get_instance_private(
        self_0,
    ) as *mut FpiDeviceAesX660Private;
    let mut data: *mut libc::c_uchar = (*transfer).buffer;
    let mut finger_missing: libc::c_int = 0 as libc::c_int;
    let mut actual_length: size_t = (*transfer).actual_length as size_t;
    if !error.is_null() {
        g_byte_array_set_size((*priv_0).stripe_packet, 0 as libc::c_int as guint);
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    g_log(
        b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Got %lu bytes of data\0" as *const u8 as *const libc::c_char,
        actual_length,
    );
    while actual_length != 0 {
        let mut payload_length: gssize = 0;
        let mut still_needed_len: gssize = 0;
        let mut copy_len: gssize = 0;
        still_needed_len = if 0 as libc::c_int as libc::c_long
            > 3 as libc::c_int as libc::c_long - (*(*priv_0).stripe_packet).len as gssize
        {
            0 as libc::c_int as libc::c_long
        } else {
            3 as libc::c_int as libc::c_long - (*(*priv_0).stripe_packet).len as gssize
        };
        copy_len = (if actual_length < still_needed_len as libc::c_ulong {
            actual_length
        } else {
            still_needed_len as libc::c_ulong
        }) as gssize;
        g_byte_array_append((*priv_0).stripe_packet, data, copy_len as guint);
        data = data.offset(copy_len as isize);
        actual_length = (actual_length as libc::c_ulong)
            .wrapping_sub(copy_len as libc::c_ulong) as size_t as size_t;
        if (*(*priv_0).stripe_packet).len < 3 as libc::c_int as libc::c_uint {
            break;
        }
        payload_length = (*((*(*priv_0).stripe_packet).data)
            .offset(0x1 as libc::c_int as isize) as libc::c_int
            + ((*((*(*priv_0).stripe_packet).data).offset(0x2 as libc::c_int as isize)
                as libc::c_int) << 8 as libc::c_int)) as gssize;
        g_log(
            b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Got frame, type %.2x payload of size %.4lx\0" as *const u8
                as *const libc::c_char,
            *((*(*priv_0).stripe_packet).data).offset(0 as libc::c_int as isize)
                as libc::c_int,
            payload_length,
        );
        still_needed_len = if 0 as libc::c_int as libc::c_long
            > 3 as libc::c_int as libc::c_long + payload_length
                - (*(*priv_0).stripe_packet).len as gssize
        {
            0 as libc::c_int as libc::c_long
        } else {
            3 as libc::c_int as libc::c_long + payload_length
                - (*(*priv_0).stripe_packet).len as gssize
        };
        copy_len = (if actual_length < still_needed_len as libc::c_ulong {
            actual_length
        } else {
            still_needed_len as libc::c_ulong
        }) as gssize;
        g_byte_array_append((*priv_0).stripe_packet, data, copy_len as guint);
        data = data.offset(copy_len as isize);
        actual_length = (actual_length as libc::c_ulong)
            .wrapping_sub(copy_len as libc::c_ulong) as size_t as size_t;
        if ((*(*priv_0).stripe_packet).len as libc::c_long)
            < payload_length + 3 as libc::c_int as libc::c_long
        {
            break;
        }
        finger_missing
            |= process_stripe_data(
                (*transfer).ssm,
                self_0,
                (*(*priv_0).stripe_packet).data,
                (*(*priv_0).stripe_packet).len as gsize,
            );
        g_byte_array_set_size((*priv_0).stripe_packet, 0 as libc::c_int as guint);
    }
    g_log(
        b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"finger %s\0" as *const u8 as *const libc::c_char,
        if finger_missing != 0 {
            b"missing\0" as *const u8 as *const libc::c_char
        } else {
            b"present\0" as *const u8 as *const libc::c_char
        },
    );
    if finger_missing != 0 {
        fpi_ssm_next_state((*transfer).ssm);
    } else {
        fpi_ssm_jump_to_state((*transfer).ssm, CAPTURE_READ_STRIPE_DATA as libc::c_int);
    };
}
unsafe extern "C" fn capture_run_state(mut ssm: *mut FpiSsm, mut _dev: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceAesX660 = FPI_DEVICE_AES_X660(_dev as gpointer);
    let mut priv_0: *mut FpiDeviceAesX660Private = fpi_device_aes_x660_get_instance_private(
        self_0,
    ) as *mut FpiDeviceAesX660Private;
    let mut cls: *mut FpiDeviceAesX660Class = FPI_DEVICE_AES_X660_GET_CLASS(
        self_0 as gpointer,
    );
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            aesX660_send_cmd(
                ssm,
                _dev,
                led_solid_cmd.as_ptr(),
                ::core::mem::size_of::<[guint8; 27]>() as libc::c_ulong,
                Some(
                    fpi_ssm_usb_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        1 => {
            g_byte_array_set_size((*priv_0).stripe_packet, 0 as libc::c_int as guint);
            aesX660_send_cmd(
                ssm,
                _dev,
                (*cls).start_imaging_cmd,
                (*cls).start_imaging_cmd_len,
                Some(
                    fpi_ssm_usb_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        2 => {
            aesX660_read_response(
                ssm,
                _dev,
                0 as libc::c_int,
                0 as libc::c_int,
                4096 as libc::c_int as size_t,
                Some(
                    capture_read_stripe_data_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        3 => {
            g_log(
                b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Got %lu frames\0" as *const u8 as *const libc::c_char,
                (*priv_0).strips_len,
            );
            aesX660_send_cmd(
                ssm,
                _dev,
                set_idle_cmd.as_ptr(),
                ::core::mem::size_of::<[guint8; 1]>() as libc::c_ulong,
                Some(
                    capture_set_idle_cmd_cb
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
unsafe extern "C" fn capture_sm_complete(
    mut ssm: *mut FpiSsm,
    mut device: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceAesX660 = FPI_DEVICE_AES_X660(device as gpointer);
    let mut priv_0: *mut FpiDeviceAesX660Private = fpi_device_aes_x660_get_instance_private(
        self_0,
    ) as *mut FpiDeviceAesX660Private;
    g_log(
        b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Capture completed\0" as *const u8 as *const libc::c_char,
    );
    if (*priv_0).deactivating != 0 {
        complete_deactivation(FP_IMAGE_DEVICE(device as gpointer));
        if !error.is_null() {
            g_error_free(error);
        }
    } else if !error.is_null() {
        fpi_image_device_session_error(FP_IMAGE_DEVICE(device as gpointer), error);
    } else {
        start_finger_detection(FP_IMAGE_DEVICE(device as gpointer));
    };
}
unsafe extern "C" fn start_capture(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceAesX660 = FPI_DEVICE_AES_X660(dev as gpointer);
    let mut priv_0: *mut FpiDeviceAesX660Private = fpi_device_aes_x660_get_instance_private(
        self_0,
    ) as *mut FpiDeviceAesX660Private;
    let mut ssm: *mut FpiSsm = 0 as *mut FpiSsm;
    if (*priv_0).deactivating != 0 {
        complete_deactivation(dev);
        return;
    }
    ssm = fpi_ssm_new_full(
        FP_DEVICE(dev as gpointer),
        Some(
            capture_run_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        CAPTURE_NUM_STATES as libc::c_int,
        CAPTURE_NUM_STATES as libc::c_int,
        b"CAPTURE_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    g_log_structured(
        b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/aesx660.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"489\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"start_capture\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/aesx660.c:489\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            capture_sm_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn activate_read_id_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceAesX660 = FPI_DEVICE_AES_X660(device as gpointer);
    let mut priv_0: *mut FpiDeviceAesX660Private = fpi_device_aes_x660_get_instance_private(
        self_0,
    ) as *mut FpiDeviceAesX660Private;
    let mut cls: *mut FpiDeviceAesX660Class = FPI_DEVICE_AES_X660_GET_CLASS(
        self_0 as gpointer,
    );
    let mut data: *mut libc::c_uchar = (*transfer).buffer;
    if !error.is_null() {
        g_log(
            b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"read_id cmd failed\0" as *const u8 as *const libc::c_char,
        );
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    if *data.offset(0 as libc::c_int as isize) as libc::c_int == 0x7 as libc::c_int {
        g_log(
            b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Sensor device id: %.2x%2x, bcdDevice: %.2x.%.2x, init status: %.2x\0"
                as *const u8 as *const libc::c_char,
            *data.offset(4 as libc::c_int as isize) as libc::c_int,
            *data.offset(3 as libc::c_int as isize) as libc::c_int,
            *data.offset(5 as libc::c_int as isize) as libc::c_int,
            *data.offset(6 as libc::c_int as isize) as libc::c_int,
            *data.offset(7 as libc::c_int as isize) as libc::c_int,
        );
    } else {
        g_log(
            b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Bogus read ID response: %.2x\0" as *const u8 as *const libc::c_char,
            *data.offset(0 as libc::c_int as isize) as libc::c_int,
        );
        fpi_ssm_mark_failed(
            (*transfer).ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Bogus read ID response %.2x\0" as *const u8 as *const libc::c_char,
                *data.offset(0 as libc::c_int as isize) as libc::c_int,
            ),
        );
        return;
    }
    match (*priv_0).init_seq_idx {
        0 => {
            (*priv_0).init_seq = (*cls).init_seqs[0 as libc::c_int as usize];
            (*priv_0).init_seq_len = (*cls).init_seqs_len[0 as libc::c_int as usize];
            (*priv_0).init_seq_idx = 1 as libc::c_int as libc::c_uint;
            (*priv_0).init_cmd_idx = 0 as libc::c_int as libc::c_uint;
            fpi_ssm_jump_to_state(
                (*transfer).ssm,
                ACTIVATE_SEND_INIT_CMD as libc::c_int,
            );
        }
        1 => {
            (*priv_0).init_seq = (*cls).init_seqs[1 as libc::c_int as usize];
            (*priv_0).init_seq_len = (*cls).init_seqs_len[1 as libc::c_int as usize];
            (*priv_0).init_seq_idx = 2 as libc::c_int as libc::c_uint;
            (*priv_0).init_cmd_idx = 0 as libc::c_int as libc::c_uint;
            fpi_ssm_next_state((*transfer).ssm);
        }
        _ => {
            g_log(
                b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Failed to init device! init status: %.2x\0" as *const u8
                    as *const libc::c_char,
                *data.offset(7 as libc::c_int as isize) as libc::c_int,
            );
            fpi_ssm_mark_failed(
                (*transfer).ssm,
                fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_PROTO,
                    b"Failed to init device %.2x\0" as *const u8 as *const libc::c_char,
                    *data.offset(7 as libc::c_int as isize) as libc::c_int,
                ),
            );
        }
    };
}
unsafe extern "C" fn activate_read_init_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceAesX660 = FPI_DEVICE_AES_X660(device as gpointer);
    let mut priv_0: *mut FpiDeviceAesX660Private = fpi_device_aes_x660_get_instance_private(
        self_0,
    ) as *mut FpiDeviceAesX660Private;
    let mut data: *mut libc::c_uchar = (*transfer).buffer;
    g_log(
        b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"read_init_cb\0" as *const u8 as *const libc::c_char,
    );
    if !error.is_null() {
        g_log(
            b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"read_init transfer status: %s, actual_len: %d\0" as *const u8
                as *const libc::c_char,
            (*error).message,
            (*transfer).actual_length as gint,
        );
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    if *data.offset(0 as libc::c_int as isize) as libc::c_int != 0x42 as libc::c_int
        || *data.offset(3 as libc::c_int as isize) as libc::c_int != 0x1 as libc::c_int
    {
        g_log(
            b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Bogus read init response: %.2x %.2x\0" as *const u8 as *const libc::c_char,
            *data.offset(0 as libc::c_int as isize) as libc::c_int,
            *data.offset(3 as libc::c_int as isize) as libc::c_int,
        );
        fpi_ssm_mark_failed(
            (*transfer).ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Bogus read init response: %.2x %.2x\0" as *const u8
                    as *const libc::c_char,
                *data.offset(0 as libc::c_int as isize) as libc::c_int,
                *data.offset(3 as libc::c_int as isize) as libc::c_int,
            ),
        );
        return;
    }
    (*priv_0).init_cmd_idx = ((*priv_0).init_cmd_idx).wrapping_add(1);
    if (*priv_0).init_cmd_idx as libc::c_ulong == (*priv_0).init_seq_len {
        if (*priv_0).init_seq_idx < 2 as libc::c_int as libc::c_uint {
            fpi_ssm_jump_to_state(
                (*transfer).ssm,
                ACTIVATE_SEND_READ_ID_CMD as libc::c_int,
            );
        } else {
            fpi_ssm_mark_completed((*transfer).ssm);
        }
        return;
    }
    fpi_ssm_jump_to_state((*transfer).ssm, ACTIVATE_SEND_INIT_CMD as libc::c_int);
}
unsafe extern "C" fn activate_run_state(mut ssm: *mut FpiSsm, mut _dev: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceAesX660 = FPI_DEVICE_AES_X660(_dev as gpointer);
    let mut priv_0: *mut FpiDeviceAesX660Private = fpi_device_aes_x660_get_instance_private(
        self_0,
    ) as *mut FpiDeviceAesX660Private;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            (*priv_0).init_seq_idx = 0 as libc::c_int as libc::c_uint;
            g_log(
                b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Activate: set idle\0" as *const u8 as *const libc::c_char,
            );
            aesX660_send_cmd(
                ssm,
                _dev,
                set_idle_cmd.as_ptr(),
                ::core::mem::size_of::<[guint8; 1]>() as libc::c_ulong,
                Some(
                    fpi_ssm_usb_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        1 => {
            g_log(
                b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Activate: read ID\0" as *const u8 as *const libc::c_char,
            );
            aesX660_send_cmd(
                ssm,
                _dev,
                read_id_cmd.as_ptr(),
                ::core::mem::size_of::<[guint8; 6]>() as libc::c_ulong,
                Some(
                    fpi_ssm_usb_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        2 => {
            aesX660_read_response(
                ssm,
                _dev,
                (0 as libc::c_int == 0) as libc::c_int,
                0 as libc::c_int,
                8 as libc::c_int as size_t,
                Some(
                    activate_read_id_cb
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
            g_log(
                b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Activate: send init seq #%d cmd #%d\0" as *const u8
                    as *const libc::c_char,
                (*priv_0).init_seq_idx,
                (*priv_0).init_cmd_idx,
            );
            aesX660_send_cmd(
                ssm,
                _dev,
                (*((*priv_0).init_seq).offset((*priv_0).init_cmd_idx as isize)).cmd,
                (*((*priv_0).init_seq).offset((*priv_0).init_cmd_idx as isize)).len,
                Some(
                    fpi_ssm_usb_transfer_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        6 => {
            g_log(
                b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Activate: read init response\0" as *const u8 as *const libc::c_char,
            );
            aesX660_read_response(
                ssm,
                _dev,
                (0 as libc::c_int == 0) as libc::c_int,
                0 as libc::c_int,
                4 as libc::c_int as size_t,
                Some(
                    activate_read_init_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        3 => {
            aesX660_send_cmd(
                ssm,
                _dev,
                calibrate_cmd.as_ptr(),
                ::core::mem::size_of::<[guint8; 6]>() as libc::c_ulong,
                Some(
                    fpi_ssm_usb_transfer_cb
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
            aesX660_read_response(
                ssm,
                _dev,
                (0 as libc::c_int == 0) as libc::c_int,
                0 as libc::c_int,
                4 as libc::c_int as size_t,
                Some(
                    aesX660_read_calibrate_data_cb
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
    fpi_image_device_activate_complete(FP_IMAGE_DEVICE(_dev as gpointer), error);
    if error.is_null() {
        start_finger_detection(FP_IMAGE_DEVICE(_dev as gpointer));
    }
}
unsafe extern "C" fn aesX660_dev_activate(mut dev: *mut FpImageDevice) {
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
            activate_sm_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn aesX660_dev_deactivate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceAesX660 = FPI_DEVICE_AES_X660(dev as gpointer);
    let mut priv_0: *mut FpiDeviceAesX660Private = fpi_device_aes_x660_get_instance_private(
        self_0,
    ) as *mut FpiDeviceAesX660Private;
    (*priv_0).deactivating = (0 as libc::c_int == 0) as libc::c_int;
}
unsafe extern "C" fn aesX660_dev_init(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceAesX660 = FPI_DEVICE_AES_X660(dev as gpointer);
    let mut priv_0: *mut FpiDeviceAesX660Private = fpi_device_aes_x660_get_instance_private(
        self_0,
    ) as *mut FpiDeviceAesX660Private;
    let mut error: *mut GError = 0 as *mut GError;
    g_usb_device_claim_interface(
        fpi_device_get_usb_device(FP_DEVICE(dev as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    );
    (*priv_0).stripe_packet = g_byte_array_new();
    fpi_image_device_open_complete(dev, error);
}
unsafe extern "C" fn aesX660_dev_deinit(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceAesX660 = FPI_DEVICE_AES_X660(dev as gpointer);
    let mut priv_0: *mut FpiDeviceAesX660Private = fpi_device_aes_x660_get_instance_private(
        self_0,
    ) as *mut FpiDeviceAesX660Private;
    let mut error: *mut GError = 0 as *mut GError;
    g_usb_device_release_interface(
        fpi_device_get_usb_device(FP_DEVICE(dev as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    );
    let mut _pp: C2RustUnnamed_5 = C2RustUnnamed_5 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GByteArray) -> ()>,
        GDestroyNotify,
    >(Some(g_byte_array_unref as unsafe extern "C" fn(*mut GByteArray) -> ()));
    _pp.in_0 = &mut (*priv_0).stripe_packet as *mut *mut GByteArray as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    fpi_image_device_close_complete(dev, error);
}
unsafe extern "C" fn complete_deactivation(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceAesX660 = FPI_DEVICE_AES_X660(dev as gpointer);
    let mut priv_0: *mut FpiDeviceAesX660Private = fpi_device_aes_x660_get_instance_private(
        self_0,
    ) as *mut FpiDeviceAesX660Private;
    g_log_structured(
        b"libfprint-aesX660\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/aesx660.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"722\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"complete_deactivation\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/aesx660.c:722\0" as *const u8 as *const libc::c_char,
    );
    (*priv_0).deactivating = 0 as libc::c_int;
    g_slist_free((*priv_0).strips);
    (*priv_0).strips = 0 as *mut GSList;
    (*priv_0).strips_len = 0 as libc::c_int as size_t;
    fpi_image_device_deactivate_complete(dev, 0 as *mut GError);
}
unsafe extern "C" fn fpi_device_aes_x660_init(mut self_0: *mut FpiDeviceAesX660) {}
unsafe extern "C" fn fpi_device_aes_x660_class_init(
    mut klass: *mut FpiDeviceAesX660Class,
) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    let mut img_class: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_CLASS(
        klass as gpointer,
    );
    (*dev_class).type_0 = FP_DEVICE_TYPE_USB;
    (*dev_class).scan_type = FP_SCAN_TYPE_SWIPE;
    (*img_class)
        .img_open = Some(
        aesX660_dev_init as unsafe extern "C" fn(*mut FpImageDevice) -> (),
    );
    (*img_class)
        .img_close = Some(
        aesX660_dev_deinit as unsafe extern "C" fn(*mut FpImageDevice) -> (),
    );
    (*img_class)
        .activate = Some(
        aesX660_dev_activate as unsafe extern "C" fn(*mut FpImageDevice) -> (),
    );
    (*img_class)
        .deactivate = Some(
        aesX660_dev_deactivate as unsafe extern "C" fn(*mut FpImageDevice) -> (),
    );
}
