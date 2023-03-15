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
    fn g_ptr_array_new_with_free_func(
        element_free_func: GDestroyNotify,
    ) -> *mut GPtrArray;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_clear_error(err: *mut *mut GError);
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_reverse(list: *mut GSList) -> *mut GSList;
    fn g_slist_nth_data(list: *mut GSList, n: guint) -> gpointer;
    fn g_get_monotonic_time() -> gint64;
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn g_log_structured(log_domain: *const gchar, log_level: GLogLevelFlags, _: ...);
    fn fpi_ssm_usb_transfer_cb(
        transfer: *mut FpiUsbTransfer,
        device: *mut FpDevice,
        unused_data: gpointer,
        error: *mut GError,
    );
    fn fpi_ssm_get_cur_state(machine: *mut FpiSsm) -> libc::c_int;
    fn fpi_ssm_mark_failed(machine: *mut FpiSsm, error: *mut GError);
    fn fpi_ssm_mark_completed(machine: *mut FpiSsm);
    fn fpi_ssm_jump_to_state(machine: *mut FpiSsm, state: libc::c_int);
    fn fpi_ssm_next_state(machine: *mut FpiSsm);
    fn fpi_ssm_start_subsm(parent: *mut FpiSsm, child: *mut FpiSsm);
    fn fpi_ssm_start(ssm: *mut FpiSsm, callback: FpiSsmCompletedCallback);
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
    fn g_object_unref(object: gpointer);
    fn fpi_ssm_new_full(
        dev: *mut FpDevice,
        handler: FpiSsmHandlerCallback,
        nr_states: libc::c_int,
        start_cleanup: libc::c_int,
        machine_name: *const libc::c_char,
    ) -> *mut FpiSsm;
    fn g_cancellable_new() -> *mut GCancellable;
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
    fn fpi_assemble_lines(
        ctx: *mut fpi_line_asmbl_ctx,
        lines: *mut GSList,
        num_lines: size_t,
    ) -> *mut FpImage;
    fn g_usb_device_get_pid(self_0: *mut GUsbDevice) -> guint16;
    fn g_usb_device_get_release(self_0: *mut GUsbDevice) -> guint16;
    fn g_usb_device_set_configuration(
        self_0: *mut GUsbDevice,
        configuration: gint,
        error: *mut *mut GError,
    ) -> gboolean;
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
    fn fpi_device_get_driver_data(device: *mut FpDevice) -> guint64;
    fn fpi_device_get_cancellable(device: *mut FpDevice) -> *mut GCancellable;
    fn fp_image_device_get_type() -> GType;
    fn fpi_image_device_set_bz3_threshold(
        self_0: *mut FpImageDevice,
        bz3_threshold: gint,
    );
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
    fn fpi_std_sq_dev(buf: *const guint8, size: gint) -> gint;
    fn fpi_mean_sq_diff_norm(
        buf1: *const guint8,
        buf2: *const guint8,
        size: gint,
    ) -> gint;
    fn fpi_usb_transfer_new(device: *mut FpDevice) -> *mut FpiUsbTransfer;
    fn fpi_usb_transfer_ref(self_0: *mut FpiUsbTransfer) -> *mut FpiUsbTransfer;
    fn fpi_usb_transfer_unref(self_0: *mut FpiUsbTransfer);
    fn fpi_usb_transfer_fill_bulk(
        transfer: *mut FpiUsbTransfer,
        endpoint: guint8,
        length: gsize,
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
    fn fpi_usb_transfer_fill_interrupt(
        transfer: *mut FpiUsbTransfer,
        endpoint: guint8,
        length: gsize,
    );
    fn fpi_usb_transfer_submit(
        transfer: *mut FpiUsbTransfer,
        timeout_ms: guint,
        cancellable: *mut GCancellable,
        callback: FpiUsbTransferCallback,
        user_data: gpointer,
    );
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
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
pub type GFreeFunc = Option::<unsafe extern "C" fn(gpointer) -> ()>;
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
pub struct sonly_regwrite {
    pub reg: guint8,
    pub value: guint8,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const UPEKSONLY_1001: C2RustUnnamed_4 = 2;
pub const UPEKSONLY_1000: C2RustUnnamed_4 = 1;
pub const UPEKSONLY_2016: C2RustUnnamed_4 = 0;
pub type sonly_kill_transfers_action = libc::c_uint;
pub const ITERATE_SSM: sonly_kill_transfers_action = 2;
pub const IMG_SESSION_ERROR: sonly_kill_transfers_action = 1;
pub const NOT_KILLING: sonly_kill_transfers_action = 0;
pub type sonly_fs = libc::c_uint;
pub const FINGER_REMOVED: sonly_fs = 2;
pub const FINGER_DETECTED: sonly_fs = 1;
pub const AWAIT_FINGER: sonly_fs = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceUpeksonly {
    pub parent: FpImageDevice,
    pub capturing: gboolean,
    pub deactivating: gboolean,
    pub read_reg_result: guint8,
    pub dev_model: libc::c_int,
    pub img_width: libc::c_int,
    pub loopsm: *mut FpiSsm,
    pub img_cancellable: *mut GCancellable,
    pub img_transfers: *mut GPtrArray,
    pub num_flying: libc::c_int,
    pub rows: *mut GSList,
    pub num_rows: libc::c_uint,
    pub rowbuf: *mut libc::c_uchar,
    pub rowbuf_offset: libc::c_int,
    pub wraparounds: libc::c_int,
    pub num_blank: libc::c_int,
    pub num_nonblank: libc::c_int,
    pub finger_state: sonly_fs,
    pub last_seqnum: libc::c_int,
    pub killing_transfers: sonly_kill_transfers_action,
    pub kill_error: *mut GError,
    pub kill_ssm: *mut FpiSsm,
    pub assembling_ctx: fpi_line_asmbl_ctx,
}
pub type FpiDeviceUpeksonly = _FpiDeviceUpeksonly;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiDeviceUpeksonlyClass {
    pub parent_class: FpImageDeviceClass,
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
pub const LOOPSM_NUM_STATES: loopsm_states = 6;
pub const LOOPSM_RUN_AWFSM: loopsm_states = 0;
pub const LOOPSM_FINAL: loopsm_states = 5;
pub const DEINITSM_1001_NUM_STATES: deinitsm_1001_states = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct write_regs_data {
    pub dev: *mut FpDevice,
    pub ssm: *mut FpiSsm,
    pub transfer: *mut FpiUsbTransfer,
    pub regs: *const sonly_regwrite,
    pub num_regs: size_t,
    pub regs_written: size_t,
}
pub const DEINITSM_1001_WRITEV: deinitsm_1001_states = 0;
pub const DEINITSM_1000_NUM_STATES: deinitsm_1000_states = 1;
pub const DEINITSM_1000_WRITEV: deinitsm_1000_states = 0;
pub const DEINITSM_2016_NUM_STATES: deinitsm_2016_states = 1;
pub const DEINITSM_2016_WRITEV: deinitsm_2016_states = 0;
pub const LOOPSM_RUN_DEINITSM: loopsm_states = 4;
pub const LOOPSM_CAPTURE: loopsm_states = 3;
pub const CAPSM_1001_NUM_STATES: capsm_1001_states = 7;
pub const CAPSM_1001_WRITEV_5: capsm_1001_states = 6;
pub const CAPSM_1001_WRITEV_4: capsm_1001_states = 5;
pub const CAPSM_1001_WRITEV_3: capsm_1001_states = 4;
pub const CAPSM_1001_WRITEV_2: capsm_1001_states = 3;
pub const CAPSM_1001_WRITEV_1: capsm_1001_states = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub const CAPSM_1001_FIRE_BULK: capsm_1001_states = 1;
pub const CAPSM_1001_INIT: capsm_1001_states = 0;
pub const CAPSM_1000_NUM_STATES: capsm_1000_states = 3;
pub const CAPSM_1000_WRITEV: capsm_1000_states = 2;
pub const CAPSM_1000_FIRE_BULK: capsm_1000_states = 1;
pub const CAPSM_1000_INIT: capsm_1000_states = 0;
pub const CAPSM_2016_NUM_STATES: capsm_2016_states = 5;
pub const CAPSM_2016_WRITEV: capsm_2016_states = 4;
pub const CAPSM_2016_FIRE_BULK: capsm_2016_states = 3;
pub const CAPSM_2016_WRITE_30: capsm_2016_states = 2;
pub const CAPSM_2016_WRITE_15: capsm_2016_states = 1;
pub const CAPSM_2016_INIT: capsm_2016_states = 0;
pub const LOOPSM_RUN_CAPSM: loopsm_states = 2;
pub const LOOPSM_AWAIT_FINGER: loopsm_states = 1;
pub const AWFSM_1000_NUM_STATES: awfsm_1000_states = 2;
pub const AWFSM_1000_WRITEV_2: awfsm_1000_states = 1;
pub const AWFSM_1000_WRITEV_1: awfsm_1000_states = 0;
pub const AWFSM_2016_NUM_STATES: awfsm_2016_states = 10;
pub const AWFSM_2016_WRITEV_4: awfsm_2016_states = 9;
pub const AWFSM_2016_WRITE_07: awfsm_2016_states = 8;
pub const AWFSM_2016_READ_07: awfsm_2016_states = 7;
pub const AWFSM_2016_WRITEV_3: awfsm_2016_states = 6;
pub const AWFSM_2016_WRITE_13: awfsm_2016_states = 5;
pub const AWFSM_2016_READ_13: awfsm_2016_states = 4;
pub const AWFSM_2016_WRITEV_2: awfsm_2016_states = 3;
pub const AWFSM_2016_WRITE_01: awfsm_2016_states = 2;
pub const AWFSM_2016_READ_01: awfsm_2016_states = 1;
pub const AWFSM_2016_WRITEV_1: awfsm_2016_states = 0;
pub const INITSM_1001_NUM_STATES: initsm_1001_states = 5;
pub const INITSM_1001_WRITEV_5: initsm_1001_states = 4;
pub const INITSM_1001_WRITEV_4: initsm_1001_states = 3;
pub const INITSM_1001_WRITEV_3: initsm_1001_states = 2;
pub const INITSM_1001_WRITEV_2: initsm_1001_states = 1;
pub const INITSM_1001_WRITEV_1: initsm_1001_states = 0;
pub const INITSM_1000_NUM_STATES: initsm_1000_states = 1;
pub const INITSM_1000_WRITEV_1: initsm_1000_states = 0;
pub const INITSM_2016_NUM_STATES: initsm_2016_states = 7;
pub const INITSM_2016_WRITE_05: initsm_2016_states = 6;
pub const INITSM_2016_WRITE_04: initsm_2016_states = 5;
pub const INITSM_2016_WRITE_13: initsm_2016_states = 4;
pub const INITSM_2016_READ_13: initsm_2016_states = 3;
pub const INITSM_2016_WRITE_09: initsm_2016_states = 2;
pub const INITSM_2016_READ_09: initsm_2016_states = 1;
pub const INITSM_2016_WRITEV_1: initsm_2016_states = 0;
pub type awfsm_2016_states = libc::c_uint;
pub type awfsm_1000_states = libc::c_uint;
pub type capsm_2016_states = libc::c_uint;
pub type capsm_1000_states = libc::c_uint;
pub type capsm_1001_states = libc::c_uint;
pub type deinitsm_2016_states = libc::c_uint;
pub type deinitsm_1000_states = libc::c_uint;
pub type deinitsm_1001_states = libc::c_uint;
pub type initsm_2016_states = libc::c_uint;
pub type initsm_1000_states = libc::c_uint;
pub type initsm_1001_states = libc::c_uint;
pub type loopsm_states = libc::c_uint;
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
static mut awfsm_2016_writev_1: [sonly_regwrite; 6] = [
    {
        let mut init = sonly_regwrite {
            reg: 0xa as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xa as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x9 as libc::c_int as guint8,
            value: 0x20 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3 as libc::c_int as guint8,
            value: 0x3b as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0 as libc::c_int as guint8,
            value: 0x67 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0 as libc::c_int as guint8,
            value: 0x67 as libc::c_int as guint8,
        };
        init
    },
];
static mut awfsm_1000_writev_1: [sonly_regwrite; 19] = [
    {
        let mut init = sonly_regwrite {
            reg: 0xa as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x9 as libc::c_int as guint8,
            value: 0x20 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3 as libc::c_int as guint8,
            value: 0x37 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0 as libc::c_int as guint8,
            value: 0x5f as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1 as libc::c_int as guint8,
            value: 0x6e as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1 as libc::c_int as guint8,
            value: 0xee as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xc as libc::c_int as guint8,
            value: 0x13 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xd as libc::c_int as guint8,
            value: 0xd as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xe as libc::c_int as guint8,
            value: 0xe as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xf as libc::c_int as guint8,
            value: 0xd as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x13 as libc::c_int as guint8,
            value: 0x5 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x13 as libc::c_int as guint8,
            value: 0x45 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x30 as libc::c_int as guint8,
            value: 0xe0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x15 as libc::c_int as guint8,
            value: 0x26 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x12 as libc::c_int as guint8,
            value: 0x1 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x20 as libc::c_int as guint8,
            value: 0x1 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x7 as libc::c_int as guint8,
            value: 0x10 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x10 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x11 as libc::c_int as guint8,
            value: 0xbf as libc::c_int as guint8,
        };
        init
    },
];
static mut awfsm_2016_writev_2: [sonly_regwrite; 6] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x1 as libc::c_int as guint8,
            value: 0xc6 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xc as libc::c_int as guint8,
            value: 0x13 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xd as libc::c_int as guint8,
            value: 0xd as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xe as libc::c_int as guint8,
            value: 0xe as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xf as libc::c_int as guint8,
            value: 0xd as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xb as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
];
static mut awfsm_1000_writev_2: [sonly_regwrite; 3] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x30 as libc::c_int as guint8,
            value: 0xe1 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x15 as libc::c_int as guint8,
            value: 0x6 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x15 as libc::c_int as guint8,
            value: 0x86 as libc::c_int as guint8,
        };
        init
    },
];
static mut awfsm_2016_writev_3: [sonly_regwrite; 8] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x13 as libc::c_int as guint8,
            value: 0x45 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x30 as libc::c_int as guint8,
            value: 0xe0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x12 as libc::c_int as guint8,
            value: 0x1 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x20 as libc::c_int as guint8,
            value: 0x1 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x9 as libc::c_int as guint8,
            value: 0x20 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xa as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x30 as libc::c_int as guint8,
            value: 0xe0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x20 as libc::c_int as guint8,
            value: 0x1 as libc::c_int as guint8,
        };
        init
    },
];
static mut awfsm_2016_writev_4: [sonly_regwrite; 15] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x8 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x10 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x12 as libc::c_int as guint8,
            value: 0x1 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x11 as libc::c_int as guint8,
            value: 0xbf as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x12 as libc::c_int as guint8,
            value: 0x1 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x7 as libc::c_int as guint8,
            value: 0x10 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x7 as libc::c_int as guint8,
            value: 0x10 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x5 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xb as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x15 as libc::c_int as guint8,
            value: 0x20 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x30 as libc::c_int as guint8,
            value: 0xe1 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x15 as libc::c_int as guint8,
            value: 0x24 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x15 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x15 as libc::c_int as guint8,
            value: 0x84 as libc::c_int as guint8,
        };
        init
    },
];
static mut capsm_2016_writev: [sonly_regwrite; 5] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x9 as libc::c_int as guint8,
            value: 0x28 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x13 as libc::c_int as guint8,
            value: 0x55 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xb as libc::c_int as guint8,
            value: 0x80 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x5 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
];
static mut capsm_1000_writev: [sonly_regwrite; 3] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x8 as libc::c_int as guint8,
            value: 0x80 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x13 as libc::c_int as guint8,
            value: 0x55 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xb as libc::c_int as guint8,
            value: 0x80 as libc::c_int as guint8,
        };
        init
    },
];
static mut capsm_1001_writev_1: [sonly_regwrite; 3] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x1a as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4a as libc::c_int as guint8,
            value: 0x9d as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4e as libc::c_int as guint8,
            value: 0x5 as libc::c_int as guint8,
        };
        init
    },
];
static mut capsm_1001_writev_2: [sonly_regwrite; 2] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x4d as libc::c_int as guint8,
            value: 0xc0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4e as libc::c_int as guint8,
            value: 0x9 as libc::c_int as guint8,
        };
        init
    },
];
static mut capsm_1001_writev_3: [sonly_regwrite; 9] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x4a as libc::c_int as guint8,
            value: 0x9c as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1a as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xb as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x5 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1a as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4a as libc::c_int as guint8,
            value: 0x9d as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4d as libc::c_int as guint8,
            value: 0x40 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4e as libc::c_int as guint8,
            value: 0x9 as libc::c_int as guint8,
        };
        init
    },
];
static mut capsm_1001_writev_4: [sonly_regwrite; 5] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x4a as libc::c_int as guint8,
            value: 0x9c as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1a as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1a as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4a as libc::c_int as guint8,
            value: 0x9d as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4e as libc::c_int as guint8,
            value: 0x8 as libc::c_int as guint8,
        };
        init
    },
];
static mut capsm_1001_writev_5: [sonly_regwrite; 29] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x4a as libc::c_int as guint8,
            value: 0x9c as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1a as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1a as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0 as libc::c_int as guint8,
            value: 0x5f as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1 as libc::c_int as guint8,
            value: 0xee as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3 as libc::c_int as guint8,
            value: 0x2c as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x7 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x8 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x9 as libc::c_int as guint8,
            value: 0x29 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xa as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xb as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xc as libc::c_int as guint8,
            value: 0x13 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xd as libc::c_int as guint8,
            value: 0xd as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xe as libc::c_int as guint8,
            value: 0xe as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xf as libc::c_int as guint8,
            value: 0xd as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x10 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x11 as libc::c_int as guint8,
            value: 0x8f as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x12 as libc::c_int as guint8,
            value: 0x1 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x13 as libc::c_int as guint8,
            value: 0x45 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x15 as libc::c_int as guint8,
            value: 0x26 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1e as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x20 as libc::c_int as guint8,
            value: 0x1 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x25 as libc::c_int as guint8,
            value: 0x8f as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x27 as libc::c_int as guint8,
            value: 0x23 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x30 as libc::c_int as guint8,
            value: 0xe0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x7 as libc::c_int as guint8,
            value: 0x10 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x9 as libc::c_int as guint8,
            value: 0x21 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x13 as libc::c_int as guint8,
            value: 0x75 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xb as libc::c_int as guint8,
            value: 0x80 as libc::c_int as guint8,
        };
        init
    },
];
static mut deinitsm_2016_writev: [sonly_regwrite; 4] = [
    {
        let mut init = sonly_regwrite {
            reg: 0xb as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x9 as libc::c_int as guint8,
            value: 0x20 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x13 as libc::c_int as guint8,
            value: 0x45 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x13 as libc::c_int as guint8,
            value: 0x45 as libc::c_int as guint8,
        };
        init
    },
];
static mut deinitsm_1000_writev: [sonly_regwrite; 5] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x15 as libc::c_int as guint8,
            value: 0x26 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x30 as libc::c_int as guint8,
            value: 0xe0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xb as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x13 as libc::c_int as guint8,
            value: 0x45 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x8 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
];
static mut deinitsm_1001_writev: [sonly_regwrite; 4] = [
    {
        let mut init = sonly_regwrite {
            reg: 0xb as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x13 as libc::c_int as guint8,
            value: 0x45 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x9 as libc::c_int as guint8,
            value: 0x29 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1a as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
];
static mut initsm_2016_writev_1: [sonly_regwrite; 15] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x49 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x83 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x4f as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0xf as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0xbf as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x45 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x35 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x1c as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0xae as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x44 as libc::c_int as guint8,
            value: 0x1 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x43 as libc::c_int as guint8,
            value: 0x6 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x43 as libc::c_int as guint8,
            value: 0x5 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x43 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x44 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xb as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
];
static mut initsm_1000_writev_1: [sonly_regwrite; 13] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x49 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x7f as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x7f as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x7f as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x7f as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x7f as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x7f as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x7f as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x7f as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x5 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0xb as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x8 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
];
static mut initsm_1001_writev_1: [sonly_regwrite; 18] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x4a as libc::c_int as guint8,
            value: 0x9d as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4f as libc::c_int as guint8,
            value: 0x6 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4f as libc::c_int as guint8,
            value: 0x5 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4f as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4a as libc::c_int as guint8,
            value: 0x9c as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0xa6 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x1 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x68 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0xfd as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x72 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0xef as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x5d as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0xc5 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1a as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4a as libc::c_int as guint8,
            value: 0x9d as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4c as libc::c_int as guint8,
            value: 0x1f as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4d as libc::c_int as guint8,
            value: 0xb8 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4e as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
];
static mut initsm_1001_writev_2: [sonly_regwrite; 3] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x4c as libc::c_int as guint8,
            value: 0x3 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4d as libc::c_int as guint8,
            value: 0xb8 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4e as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
];
static mut initsm_1001_writev_3: [sonly_regwrite; 7] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x4a as libc::c_int as guint8,
            value: 0x9c as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1a as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1a as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4a as libc::c_int as guint8,
            value: 0x9d as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4c as libc::c_int as guint8,
            value: 0xff as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4d as libc::c_int as guint8,
            value: 0xc0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4e as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
];
static mut initsm_1001_writev_4: [sonly_regwrite; 88] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x4a as libc::c_int as guint8,
            value: 0x9c as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1a as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x9 as libc::c_int as guint8,
            value: 0x27 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1a as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x49 as libc::c_int as guint8,
            value: 0x1 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0xa as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0xa as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0xa as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0xa as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x47 as libc::c_int as guint8,
            value: 0x4 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x49 as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x90 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0xbd as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0xbf as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x48 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x2a as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0xe3 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0xd2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x3e as libc::c_int as guint8,
            value: 0x58 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x9 as libc::c_int as guint8,
            value: 0x2f as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1a as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1a as libc::c_int as guint8,
            value: 0x2 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4a as libc::c_int as guint8,
            value: 0x9d as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4d as libc::c_int as guint8,
            value: 0x40 as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x4e as libc::c_int as guint8,
            value: 0x3 as libc::c_int as guint8,
        };
        init
    },
];
static mut initsm_1001_writev_5: [sonly_regwrite; 2] = [
    {
        let mut init = sonly_regwrite {
            reg: 0x4a as libc::c_int as guint8,
            value: 0x9c as libc::c_int as guint8,
        };
        init
    },
    {
        let mut init = sonly_regwrite {
            reg: 0x1a as libc::c_int as guint8,
            value: 0 as libc::c_int as guint8,
        };
        init
    },
];
#[inline]
unsafe extern "C" fn FPI_DEVICE_UPEKSONLY(mut ptr: gpointer) -> *mut FpiDeviceUpeksonly {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_upeksonly_get_type(),
    ) as *mut libc::c_void as *mut FpiDeviceUpeksonly;
}
#[inline(never)]
unsafe extern "C" fn fpi_device_upeksonly_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_image_device_get_type(),
        g_intern_static_string(
            b"FpiDeviceUpeksonly\0" as *const u8 as *const libc::c_char,
        ),
        ::core::mem::size_of::<FpiDeviceUpeksonlyClass>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_upeksonly_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpiDeviceUpeksonly>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpiDeviceUpeksonly) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_upeksonly_init
                        as unsafe extern "C" fn(*mut FpiDeviceUpeksonly) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn fpi_device_upeksonly_class_intern_init(mut klass: gpointer) {
    fpi_device_upeksonly_parent_class = g_type_class_peek_parent(klass);
    if FpiDeviceUpeksonly_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &mut FpiDeviceUpeksonly_private_offset,
        );
    }
    fpi_device_upeksonly_class_init(klass as *mut FpiDeviceUpeksonlyClass);
}
static mut fpi_device_upeksonly_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut FpiDeviceUpeksonly_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn fpi_device_upeksonly_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_upeksonly_get_type_once();
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
unsafe extern "C" fn upeksonly_get_deviation2(
    mut ctx: *mut fpi_line_asmbl_ctx,
    mut line1: *mut GSList,
    mut line2: *mut GSList,
) -> libc::c_int {
    let mut buf1: *mut libc::c_uchar = (*line1).data as *mut libc::c_uchar;
    let mut buf2: *mut libc::c_uchar = (*line2).data as *mut libc::c_uchar;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut mean: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*ctx).line_width > 0 as libc::c_int as libc::c_uint {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/upeksonly.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"upeksonly_get_deviation2\0"))
                .as_ptr(),
            b"ctx->line_width > 0\0" as *const u8 as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*ctx).line_width {
        mean
            += *buf1.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                + *buf2.offset(i as isize) as libc::c_int;
        i += 2 as libc::c_int;
    }
    mean = (mean as libc::c_uint)
        .wrapping_div(((*ctx).line_width).wrapping_div(2 as libc::c_int as libc::c_uint))
        as libc::c_int as libc::c_int;
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*ctx).line_width {
        let mut dev: libc::c_int = *buf1.offset((i + 1 as libc::c_int) as isize)
            as libc::c_int + *buf2.offset(i as isize) as libc::c_int - mean;
        res += dev * dev;
        i += 2 as libc::c_int;
    }
    return (res as libc::c_uint)
        .wrapping_div(((*ctx).line_width).wrapping_div(2 as libc::c_int as libc::c_uint))
        as libc::c_int;
}
unsafe extern "C" fn upeksonly_get_pixel(
    mut ctx: *mut fpi_line_asmbl_ctx,
    mut row: *mut GSList,
    mut x: libc::c_uint,
) -> libc::c_uchar {
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut offset: libc::c_uint = 0;
    if x < ((*ctx).line_width).wrapping_sub(2 as libc::c_int as libc::c_uint) {
        offset = x.wrapping_add(2 as libc::c_int as libc::c_uint);
    } else if x > ((*ctx).line_width).wrapping_sub(2 as libc::c_int as libc::c_uint)
        && x < (*ctx).line_width
    {
        offset = x
            .wrapping_sub(
                ((*ctx).line_width).wrapping_sub(2 as libc::c_int as libc::c_uint),
            );
    } else {
        return 0 as libc::c_int as libc::c_uchar
    }
    if x & 1 as libc::c_int as libc::c_uint == 0
        && !(if !row.is_null() { (*row).next } else { 0 as *mut GSList }).is_null()
        && !(if !(if !row.is_null() { (*row).next } else { 0 as *mut GSList }).is_null()
        {
            (*(if !row.is_null() { (*row).next } else { 0 as *mut GSList })).next
        } else {
            0 as *mut GSList
        })
            .is_null()
    {
        buf = (*if !if !row.is_null() { (*row).next } else { 0 as *mut GSList }.is_null()
        {
            (*if !row.is_null() { (*row).next } else { 0 as *mut GSList }).next
        } else {
            0 as *mut GSList
        })
            .data as *mut libc::c_uchar;
    } else {
        buf = (*row).data as *mut libc::c_uchar;
    }
    return *buf.offset(offset as isize);
}
unsafe extern "C" fn free_img_transfers(mut sdev: *mut FpiDeviceUpeksonly) {
    g_cancellable_cancel((*sdev).img_cancellable);
    let mut _pp: C2RustUnnamed_6 = C2RustUnnamed_6 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp
        .in_0 = &mut (*sdev).img_cancellable as *mut *mut GCancellable
        as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    let mut _pp_0: C2RustUnnamed_5 = C2RustUnnamed_5 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_0: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GPtrArray) -> ()>,
        GDestroyNotify,
    >(Some(g_ptr_array_unref as unsafe extern "C" fn(*mut GPtrArray) -> ()));
    _pp_0.in_0 = &mut (*sdev).img_transfers as *mut *mut GPtrArray as *mut libc::c_char;
    _p_0 = *_pp_0.out;
    if !_p_0.is_null() {
        *_pp_0.out = 0 as *mut libc::c_void;
        _destroy_0.expect("non-null function pointer")(_p_0);
    }
}
unsafe extern "C" fn last_transfer_killed(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(dev as gpointer);
    match (*self_0).killing_transfers as libc::c_uint {
        2 => {
            g_log(
                b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"iterate ssm\0" as *const u8 as *const libc::c_char,
            );
            fpi_ssm_next_state((*self_0).kill_ssm);
            return;
        }
        1 => {
            g_log(
                b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"session error %s\0" as *const u8 as *const libc::c_char,
                (*(*self_0).kill_error).message,
            );
            fpi_image_device_session_error(
                dev,
                (if 0 as libc::c_int != 0 {
                    (*self_0).kill_error as *mut libc::c_void
                } else {
                    g_steal_pointer(
                        &mut (*self_0).kill_error as *mut *mut GError as gpointer,
                    )
                }) as *mut GError,
            );
            return;
        }
        0 | _ => return,
    };
}
unsafe extern "C" fn cancel_img_transfers(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(dev as gpointer);
    g_cancellable_cancel((*self_0).img_cancellable);
    if (*self_0).num_flying == 0 as libc::c_int {
        last_transfer_killed(dev);
    }
}
unsafe extern "C" fn is_capturing(mut sdev: *mut FpiDeviceUpeksonly) -> gboolean {
    return ((*sdev).num_rows < 2048 as libc::c_int as libc::c_uint
        && (*sdev).finger_state as libc::c_uint
            != FINGER_REMOVED as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn handoff_img(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(dev as gpointer);
    let mut img: *mut FpImage = 0 as *mut FpImage;
    let mut elem: *mut GSList = (*self_0).rows;
    if elem.is_null() {
        g_log(
            b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"no rows?\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*self_0).rows = g_slist_reverse((*self_0).rows);
    g_log(
        b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%u rows\0" as *const u8 as *const libc::c_char,
        (*self_0).num_rows,
    );
    img = fpi_assemble_lines(
        &mut (*self_0).assembling_ctx,
        (*self_0).rows,
        (*self_0).num_rows as size_t,
    );
    g_slist_free_full(
        (*self_0).rows,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*self_0).rows = 0 as *mut GSList;
    fpi_image_device_image_captured(dev, img);
    fpi_image_device_report_finger_status(dev, 0 as libc::c_int);
    (*self_0).killing_transfers = ITERATE_SSM;
    (*self_0).kill_ssm = (*self_0).loopsm;
    cancel_img_transfers(dev);
}
unsafe extern "C" fn row_complete(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(dev as gpointer);
    (*self_0).rowbuf_offset = -(1 as libc::c_int);
    if (*self_0).num_rows > 0 as libc::c_int as libc::c_uint {
        let mut lastrow: *mut libc::c_uchar = (*(*self_0).rows).data
            as *mut libc::c_uchar;
        let mut std_sq_dev: libc::c_int = 0;
        let mut mean_sq_diff: libc::c_int = 0;
        std_sq_dev = fpi_std_sq_dev((*self_0).rowbuf, (*self_0).img_width);
        mean_sq_diff = fpi_mean_sq_diff_norm(
            lastrow,
            (*self_0).rowbuf,
            (*self_0).img_width,
        );
        match (*self_0).finger_state as libc::c_uint {
            0 => {
                if (*self_0).deactivating != 0 {
                    (*self_0).killing_transfers = ITERATE_SSM;
                    (*self_0).kill_ssm = (*self_0).loopsm;
                    cancel_img_transfers(dev);
                }
                g_log(
                    b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"std_sq_dev: %d\0" as *const u8 as *const libc::c_char,
                    std_sq_dev,
                );
                if std_sq_dev > 250 as libc::c_int {
                    (*self_0).num_nonblank += 1;
                } else {
                    (*self_0).num_nonblank = 0 as libc::c_int;
                }
                if (*self_0).num_nonblank > 32 as libc::c_int {
                    (*self_0).finger_state = FINGER_DETECTED;
                    fpi_image_device_report_finger_status(
                        dev,
                        (0 as libc::c_int == 0) as libc::c_int,
                    );
                } else {
                    return
                }
            }
            1 | 2 | _ => {}
        }
        if std_sq_dev > 250 as libc::c_int {
            (*self_0).num_blank = 0 as libc::c_int;
        } else {
            (*self_0).num_blank += 1;
            if (*self_0).num_blank > 100 as libc::c_int {
                (*self_0).finger_state = FINGER_REMOVED;
                g_log(
                    b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"detected finger removal. Blank rows: %d, Full rows: %u\0"
                        as *const u8 as *const libc::c_char,
                    (*self_0).num_blank,
                    (*self_0).num_rows,
                );
                handoff_img(dev);
                return;
            }
        }
        g_log(
            b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"mean_sq_diff: %d, std_sq_dev: %d\0" as *const u8 as *const libc::c_char,
            mean_sq_diff,
            std_sq_dev,
        );
        g_log(
            b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"num_blank: %d\0" as *const u8 as *const libc::c_char,
            (*self_0).num_blank,
        );
        if mean_sq_diff < 13 as libc::c_int {
            return;
        }
    }
    match (*self_0).finger_state as libc::c_uint {
        0 => {
            if (*self_0).num_rows == 0 {
                (*self_0)
                    .rows = g_slist_prepend(
                    (*self_0).rows,
                    (*self_0).rowbuf as gpointer,
                );
                (*self_0).num_rows = ((*self_0).num_rows).wrapping_add(1);
            } else {
                return
            }
        }
        1 | 2 => {
            (*self_0)
                .rows = g_slist_prepend((*self_0).rows, (*self_0).rowbuf as gpointer);
            (*self_0).num_rows = ((*self_0).num_rows).wrapping_add(1);
        }
        _ => {}
    }
    (*self_0).rowbuf = 0 as *mut libc::c_uchar;
    if (*self_0).num_rows >= 2048 as libc::c_int as libc::c_uint {
        g_log(
            b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"row limit met\0" as *const u8 as *const libc::c_char,
        );
        handoff_img(dev);
    }
}
unsafe extern "C" fn add_to_rowbuf(
    mut dev: *mut FpImageDevice,
    mut data: *mut libc::c_uchar,
    mut size: libc::c_int,
) {
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(dev as gpointer);
    memcpy(
        ((*self_0).rowbuf).offset((*self_0).rowbuf_offset as isize) as *mut libc::c_void,
        data as *const libc::c_void,
        size as libc::c_ulong,
    );
    (*self_0).rowbuf_offset += size;
    if (*self_0).rowbuf_offset >= (*self_0).img_width {
        row_complete(dev);
    }
}
unsafe extern "C" fn start_new_row(
    mut self_0: *mut FpiDeviceUpeksonly,
    mut data: *mut libc::c_uchar,
    mut size: libc::c_int,
) {
    if ((*self_0).rowbuf).is_null() {
        (*self_0).rowbuf = g_malloc((*self_0).img_width as gsize) as *mut libc::c_uchar;
    }
    memcpy(
        (*self_0).rowbuf as *mut libc::c_void,
        data as *const libc::c_void,
        size as libc::c_ulong,
    );
    (*self_0).rowbuf_offset = size;
}
unsafe extern "C" fn rowbuf_remaining(mut sdev: *mut FpiDeviceUpeksonly) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if (*sdev).rowbuf_offset == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    r = (*sdev).img_width - (*sdev).rowbuf_offset;
    if r > 62 as libc::c_int {
        r = 62 as libc::c_int;
    }
    return r;
}
unsafe extern "C" fn handle_packet(
    mut dev: *mut FpImageDevice,
    mut data: *mut libc::c_uchar,
) {
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(dev as gpointer);
    let mut seqnum: guint16 = ((*data.offset(0 as libc::c_int as isize) as libc::c_int)
        << 8 as libc::c_int | *data.offset(1 as libc::c_int as isize) as libc::c_int)
        as guint16;
    let mut abs_base_addr: libc::c_int = 0;
    let mut for_rowbuf: libc::c_int = 0;
    let mut next_row_addr: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut dummy_data: [libc::c_uchar; 62] = [0; 62];
    memset(
        dummy_data.as_mut_ptr() as *mut libc::c_void,
        204 as libc::c_int,
        62 as libc::c_int as libc::c_ulong,
    );
    data = data.offset(2 as libc::c_int as isize);
    if seqnum as libc::c_int != (*self_0).last_seqnum + 1 as libc::c_int {
        if seqnum as libc::c_int != 0 as libc::c_int
            && (*self_0).last_seqnum != 16383 as libc::c_int
        {
            let mut missing_data: libc::c_int = seqnum as libc::c_int
                - (*self_0).last_seqnum;
            let mut i: libc::c_int = 0;
            g_log(
                b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"lost %d packets of data between %d and %d\0" as *const u8
                    as *const libc::c_char,
                missing_data,
                (*self_0).last_seqnum,
                seqnum as libc::c_int,
            );
            i = 1 as libc::c_int;
            while i < missing_data {
                abs_base_addr = ((*self_0).last_seqnum + 1 as libc::c_int)
                    * 62 as libc::c_int;
                if (*self_0).num_rows > 1 as libc::c_int as libc::c_uint {
                    let mut row_left: libc::c_int = (*self_0).img_width
                        - (*self_0).rowbuf_offset;
                    let mut last_row: *mut libc::c_uchar = g_slist_nth_data(
                        (*self_0).rows,
                        0 as libc::c_int as guint,
                    ) as *mut libc::c_uchar;
                    if row_left >= 62 as libc::c_int {
                        memcpy(
                            dummy_data.as_mut_ptr() as *mut libc::c_void,
                            last_row.offset((*self_0).rowbuf_offset as isize)
                                as *const libc::c_void,
                            62 as libc::c_int as libc::c_ulong,
                        );
                    } else {
                        memcpy(
                            dummy_data.as_mut_ptr() as *mut libc::c_void,
                            last_row.offset((*self_0).rowbuf_offset as isize)
                                as *const libc::c_void,
                            row_left as libc::c_ulong,
                        );
                        memcpy(
                            dummy_data.as_mut_ptr().offset(row_left as isize)
                                as *mut libc::c_void,
                            last_row as *const libc::c_void,
                            (62 as libc::c_int - row_left) as libc::c_ulong,
                        );
                    }
                }
                g_log(
                    b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_WARNING,
                    b"adding dummy input for %d, i=%d\0" as *const u8
                        as *const libc::c_char,
                    (*self_0).last_seqnum + i,
                    i,
                );
                for_rowbuf = rowbuf_remaining(self_0);
                if for_rowbuf != -(1 as libc::c_int) {
                    add_to_rowbuf(dev, dummy_data.as_mut_ptr(), for_rowbuf);
                    if for_rowbuf < 62 as libc::c_int {
                        start_new_row(
                            self_0,
                            dummy_data.as_mut_ptr().offset(for_rowbuf as isize),
                            62 as libc::c_int - for_rowbuf,
                        );
                    }
                } else if abs_base_addr % (*self_0).img_width == 0 as libc::c_int {
                    start_new_row(self_0, dummy_data.as_mut_ptr(), 62 as libc::c_int);
                } else {
                    next_row_addr = (abs_base_addr / (*self_0).img_width
                        + 1 as libc::c_int) * (*self_0).img_width;
                    diff = next_row_addr - abs_base_addr;
                    if diff < 62 as libc::c_int {
                        start_new_row(
                            self_0,
                            dummy_data.as_mut_ptr().offset(diff as isize),
                            62 as libc::c_int - diff,
                        );
                    }
                }
                (*self_0).last_seqnum = (*self_0).last_seqnum + 1 as libc::c_int;
                i += 1;
            }
        }
    }
    if seqnum as libc::c_int <= (*self_0).last_seqnum {
        g_log(
            b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"detected wraparound\0" as *const u8 as *const libc::c_char,
        );
        (*self_0).wraparounds += 1;
    }
    (*self_0).last_seqnum = seqnum as libc::c_int;
    seqnum = (seqnum as libc::c_int + (*self_0).wraparounds * 16384 as libc::c_int)
        as guint16;
    abs_base_addr = seqnum as libc::c_int * 62 as libc::c_int;
    for_rowbuf = rowbuf_remaining(self_0);
    if for_rowbuf != -(1 as libc::c_int) {
        add_to_rowbuf(dev, data, for_rowbuf);
        if for_rowbuf < 62 as libc::c_int {
            start_new_row(
                self_0,
                data.offset(for_rowbuf as isize),
                62 as libc::c_int - for_rowbuf,
            );
        }
        return;
    }
    if abs_base_addr % (*self_0).img_width == 0 as libc::c_int {
        start_new_row(self_0, data, 62 as libc::c_int);
        return;
    }
    next_row_addr = (abs_base_addr / (*self_0).img_width + 1 as libc::c_int)
        * (*self_0).img_width;
    diff = next_row_addr - abs_base_addr;
    if diff < 62 as libc::c_int {
        start_new_row(self_0, data.offset(diff as isize), 62 as libc::c_int - diff);
    }
}
unsafe extern "C" fn img_data_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(device as gpointer);
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(dev as gpointer);
    let mut i: libc::c_int = 0;
    (*self_0).num_flying -= 1;
    if (*self_0).killing_transfers as u64 != 0 {
        if (*self_0).num_flying == 0 as libc::c_int {
            last_transfer_killed(dev);
        }
        g_clear_error(&mut error);
        return;
    }
    if (*transfer).actual_length % 64 as libc::c_int as libc::c_long
        != 0 as libc::c_int as libc::c_long
    {
        error = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_PROTO,
            b"Data packets need to be multiple of 64 bytes, got %zi bytes\0" as *const u8
                as *const libc::c_char,
            (*transfer).actual_length,
        );
    }
    if !error.is_null() {
        g_log(
            b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"bad status %s, terminating session\0" as *const u8 as *const libc::c_char,
            (*error).message,
        );
        (*self_0).killing_transfers = IMG_SESSION_ERROR;
        if ((*self_0).kill_error).is_null() {
            (*self_0).kill_error = error;
        } else {
            g_error_free(error);
        }
        cancel_img_transfers(dev);
        return;
    }
    i = 0 as libc::c_int;
    while (i + 64 as libc::c_int) as libc::c_long <= (*transfer).actual_length {
        if is_capturing(self_0) == 0 {
            return;
        }
        handle_packet(dev, ((*transfer).buffer).offset(i as isize));
        i += 64 as libc::c_int;
    }
    if is_capturing(self_0) != 0 {
        fpi_usb_transfer_submit(
            fpi_usb_transfer_ref(transfer),
            0 as libc::c_int as guint,
            (*self_0).img_cancellable,
            Some(
                img_data_cb
                    as unsafe extern "C" fn(
                        *mut FpiUsbTransfer,
                        *mut FpDevice,
                        gpointer,
                        *mut GError,
                    ) -> (),
            ),
            user_data,
        );
        (*self_0).num_flying += 1;
    }
}
unsafe extern "C" fn write_regs_finished(
    mut wrdata: *mut write_regs_data,
    mut error: *mut GError,
) {
    if error.is_null() {
        fpi_ssm_next_state((*wrdata).ssm);
    } else {
        fpi_ssm_mark_failed((*wrdata).ssm, error);
    }
    g_free(wrdata as gpointer);
}
unsafe extern "C" fn write_regs_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut wrdata: *mut write_regs_data = user_data as *mut write_regs_data;
    if !error.is_null() {
        write_regs_finished(wrdata, error);
        return;
    }
    (*wrdata).regs_written = ((*wrdata).regs_written).wrapping_add(1);
    write_regs_iterate(wrdata);
}
unsafe extern "C" fn write_regs_iterate(mut wrdata: *mut write_regs_data) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    let mut regwrite: *const sonly_regwrite = 0 as *const sonly_regwrite;
    if (*wrdata).regs_written >= (*wrdata).num_regs {
        write_regs_finished(wrdata, 0 as *mut GError);
        return;
    }
    regwrite = &*((*wrdata).regs).offset((*wrdata).regs_written as isize)
        as *const sonly_regwrite;
    g_log(
        b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"set %02x=%02x\0" as *const u8 as *const libc::c_char,
        (*regwrite).reg as libc::c_int,
        (*regwrite).value as libc::c_int,
    );
    transfer = fpi_usb_transfer_new((*wrdata).dev);
    fpi_usb_transfer_fill_control(
        transfer,
        G_USB_DEVICE_DIRECTION_HOST_TO_DEVICE,
        G_USB_DEVICE_REQUEST_TYPE_VENDOR,
        G_USB_DEVICE_RECIPIENT_DEVICE,
        0xc as libc::c_int as guint8,
        0 as libc::c_int as guint16,
        (*regwrite).reg as guint16,
        1 as libc::c_int as gsize,
    );
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    (*transfer).ssm = (*wrdata).ssm;
    *((*transfer).buffer).offset(0 as libc::c_int as isize) = (*regwrite).value;
    fpi_usb_transfer_submit(
        transfer,
        1000 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            write_regs_cb
                as unsafe extern "C" fn(
                    *mut FpiUsbTransfer,
                    *mut FpDevice,
                    gpointer,
                    *mut GError,
                ) -> (),
        ),
        wrdata as gpointer,
    );
}
unsafe extern "C" fn sm_write_regs(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut regs: *const sonly_regwrite,
    mut num_regs: size_t,
) {
    let mut wrdata: *mut write_regs_data = g_malloc(
        ::core::mem::size_of::<write_regs_data>() as libc::c_ulong,
    ) as *mut write_regs_data;
    (*wrdata).ssm = ssm;
    (*wrdata).regs = regs;
    (*wrdata).num_regs = num_regs;
    (*wrdata).regs_written = 0 as libc::c_int as size_t;
    (*wrdata).dev = dev;
    write_regs_iterate(wrdata);
}
unsafe extern "C" fn sm_write_reg(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
    mut reg: guint8,
    mut value: guint8,
) {
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(
        FP_DEVICE(dev as gpointer),
    );
    g_log(
        b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"set %02x=%02x\0" as *const u8 as *const libc::c_char,
        reg as libc::c_int,
        value as libc::c_int,
    );
    fpi_usb_transfer_fill_control(
        transfer,
        G_USB_DEVICE_DIRECTION_HOST_TO_DEVICE,
        G_USB_DEVICE_REQUEST_TYPE_VENDOR,
        G_USB_DEVICE_RECIPIENT_DEVICE,
        0xc as libc::c_int as guint8,
        0 as libc::c_int as guint16,
        reg as guint16,
        1 as libc::c_int as gsize,
    );
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    (*transfer).ssm = ssm;
    *((*transfer).buffer).offset(0 as libc::c_int as isize) = value;
    fpi_usb_transfer_submit(
        transfer,
        1000 as libc::c_int as guint,
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
unsafe extern "C" fn sm_read_reg_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(device as gpointer);
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(dev as gpointer);
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
    } else {
        (*self_0)
            .read_reg_result = *((*transfer).buffer).offset(0 as libc::c_int as isize);
        g_log(
            b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"read reg result = %02x\0" as *const u8 as *const libc::c_char,
            (*self_0).read_reg_result as libc::c_int,
        );
        fpi_ssm_next_state((*transfer).ssm);
    };
}
unsafe extern "C" fn sm_read_reg(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
    mut reg: guint8,
) {
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(
        FP_DEVICE(dev as gpointer),
    );
    g_log(
        b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"read reg %02x\0" as *const u8 as *const libc::c_char,
        reg as libc::c_int,
    );
    fpi_usb_transfer_fill_control(
        transfer,
        G_USB_DEVICE_DIRECTION_DEVICE_TO_HOST,
        G_USB_DEVICE_REQUEST_TYPE_VENDOR,
        G_USB_DEVICE_RECIPIENT_DEVICE,
        0xc as libc::c_int as guint8,
        0 as libc::c_int as guint16,
        reg as guint16,
        8 as libc::c_int as gsize,
    );
    (*transfer).ssm = ssm;
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_submit(
        transfer,
        1000 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            sm_read_reg_cb
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
unsafe extern "C" fn sm_await_intr_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(device as gpointer);
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(dev as gpointer);
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    g_log(
        b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"interrupt received: %02x %02x %02x %02x\0" as *const u8 as *const libc::c_char,
        *((*transfer).buffer).offset(0 as libc::c_int as isize) as libc::c_int,
        *((*transfer).buffer).offset(1 as libc::c_int as isize) as libc::c_int,
        *((*transfer).buffer).offset(2 as libc::c_int as isize) as libc::c_int,
        *((*transfer).buffer).offset(3 as libc::c_int as isize) as libc::c_int,
    );
    (*self_0).finger_state = FINGER_DETECTED;
    fpi_image_device_report_finger_status(dev, (0 as libc::c_int == 0) as libc::c_int);
    fpi_ssm_next_state((*transfer).ssm);
}
unsafe extern "C" fn sm_await_intr(mut ssm: *mut FpiSsm, mut dev: *mut FpImageDevice) {
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(
        FP_DEVICE(dev as gpointer),
    );
    g_log_structured(
        b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/upeksonly.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"752\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"sm_await_intr\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/upeksonly.c:752\0" as *const u8 as *const libc::c_char,
    );
    fpi_usb_transfer_fill_interrupt(
        transfer,
        0x83 as libc::c_int as guint8,
        4 as libc::c_int as gsize,
    );
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    (*transfer).ssm = ssm;
    fpi_usb_transfer_submit(
        transfer,
        0 as libc::c_int as guint,
        fpi_device_get_cancellable(FP_DEVICE(dev as gpointer)),
        Some(
            sm_await_intr_cb
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
unsafe extern "C" fn awfsm_2016_run_state(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(_dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            sm_write_regs(
                ssm,
                _dev,
                awfsm_2016_writev_1.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 6]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        1 => {
            sm_read_reg(ssm, dev, 0x1 as libc::c_int as guint8);
        }
        2 => {
            if (*self_0).read_reg_result as libc::c_int != 0xc6 as libc::c_int {
                sm_write_reg(
                    ssm,
                    dev,
                    0x1 as libc::c_int as guint8,
                    0x46 as libc::c_int as guint8,
                );
            } else {
                sm_write_reg(
                    ssm,
                    dev,
                    0x1 as libc::c_int as guint8,
                    0xc6 as libc::c_int as guint8,
                );
            }
        }
        3 => {
            sm_write_regs(
                ssm,
                _dev,
                awfsm_2016_writev_2.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 6]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        4 => {
            sm_read_reg(ssm, dev, 0x13 as libc::c_int as guint8);
        }
        5 => {
            if (*self_0).read_reg_result as libc::c_int != 0x45 as libc::c_int {
                sm_write_reg(
                    ssm,
                    dev,
                    0x13 as libc::c_int as guint8,
                    0x5 as libc::c_int as guint8,
                );
            } else {
                sm_write_reg(
                    ssm,
                    dev,
                    0x13 as libc::c_int as guint8,
                    0x45 as libc::c_int as guint8,
                );
            }
        }
        6 => {
            sm_write_regs(
                ssm,
                _dev,
                awfsm_2016_writev_3.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 8]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        7 => {
            sm_read_reg(ssm, dev, 0x7 as libc::c_int as guint8);
        }
        8 => {
            if (*self_0).read_reg_result as libc::c_int != 0x10 as libc::c_int
                && (*self_0).read_reg_result as libc::c_int != 0x90 as libc::c_int
            {
                g_log(
                    b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_WARNING,
                    b"odd reg7 value %x\0" as *const u8 as *const libc::c_char,
                    (*self_0).read_reg_result as libc::c_int,
                );
            }
            sm_write_reg(
                ssm,
                dev,
                0x7 as libc::c_int as guint8,
                (*self_0).read_reg_result,
            );
        }
        9 => {
            sm_write_regs(
                ssm,
                _dev,
                awfsm_2016_writev_4.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 15]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn awfsm_1000_run_state(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
) {
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            sm_write_regs(
                ssm,
                _dev,
                awfsm_1000_writev_1.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 19]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        1 => {
            sm_write_regs(
                ssm,
                _dev,
                awfsm_1000_writev_2.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 3]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn capsm_fire_bulk(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(dev as gpointer);
    let mut i: libc::c_int = 0;
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
            b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/upeksonly.c\0" as *const u8 as *const libc::c_char,
            897 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"capsm_fire_bulk\0"))
                .as_ptr(),
            b"self->capturing == FALSE\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut _pp: C2RustUnnamed_7 = C2RustUnnamed_7 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp
        .in_0 = &mut (*self_0).img_cancellable as *mut *mut GCancellable
        as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    (*self_0).img_cancellable = g_cancellable_new();
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*(*self_0).img_transfers).len {
        fpi_usb_transfer_submit(
            fpi_usb_transfer_ref(
                *((*(*self_0).img_transfers).pdata).offset(i as isize)
                    as *mut FpiUsbTransfer,
            ),
            0 as libc::c_int as guint,
            (*self_0).img_cancellable,
            Some(
                img_data_cb
                    as unsafe extern "C" fn(
                        *mut FpiUsbTransfer,
                        *mut FpDevice,
                        gpointer,
                        *mut GError,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
        (*self_0).num_flying += 1;
        i += 1;
    }
    (*self_0).capturing = (0 as libc::c_int == 0) as libc::c_int;
    fpi_ssm_next_state(ssm);
}
unsafe extern "C" fn capsm_2016_run_state(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(_dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            (*self_0).rowbuf_offset = -(1 as libc::c_int);
            (*self_0).num_rows = 0 as libc::c_int as libc::c_uint;
            (*self_0).wraparounds = -(1 as libc::c_int);
            (*self_0).num_blank = 0 as libc::c_int;
            (*self_0).num_nonblank = 0 as libc::c_int;
            (*self_0).finger_state = FINGER_DETECTED;
            (*self_0).last_seqnum = 16383 as libc::c_int;
            (*self_0).killing_transfers = NOT_KILLING;
            fpi_ssm_next_state(ssm);
        }
        1 => {
            sm_write_reg(
                ssm,
                dev,
                0x15 as libc::c_int as guint8,
                0x20 as libc::c_int as guint8,
            );
        }
        2 => {
            sm_write_reg(
                ssm,
                dev,
                0x30 as libc::c_int as guint8,
                0xe0 as libc::c_int as guint8,
            );
        }
        3 => {
            capsm_fire_bulk(ssm, _dev);
        }
        4 => {
            sm_write_regs(
                ssm,
                _dev,
                capsm_2016_writev.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 5]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn capsm_1000_run_state(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(_dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            (*self_0).rowbuf_offset = -(1 as libc::c_int);
            (*self_0).num_rows = 0 as libc::c_int as libc::c_uint;
            (*self_0).wraparounds = -(1 as libc::c_int);
            (*self_0).num_blank = 0 as libc::c_int;
            (*self_0).num_nonblank = 0 as libc::c_int;
            (*self_0).finger_state = FINGER_DETECTED;
            (*self_0).last_seqnum = 16383 as libc::c_int;
            (*self_0).killing_transfers = NOT_KILLING;
            fpi_ssm_next_state(ssm);
        }
        1 => {
            capsm_fire_bulk(ssm, _dev);
        }
        2 => {
            sm_write_regs(
                ssm,
                _dev,
                capsm_1000_writev.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 3]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn capsm_1001_run_state(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(_dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            (*self_0).rowbuf_offset = -(1 as libc::c_int);
            (*self_0).num_rows = 0 as libc::c_int as libc::c_uint;
            (*self_0).wraparounds = -(1 as libc::c_int);
            (*self_0).num_blank = 0 as libc::c_int;
            (*self_0).num_nonblank = 0 as libc::c_int;
            (*self_0).finger_state = AWAIT_FINGER;
            (*self_0).last_seqnum = 16383 as libc::c_int;
            (*self_0).killing_transfers = NOT_KILLING;
            fpi_ssm_next_state(ssm);
        }
        1 => {
            capsm_fire_bulk(ssm, _dev);
        }
        2 => {
            sm_write_regs(
                ssm,
                _dev,
                capsm_1001_writev_1.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 3]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        3 => {
            sm_write_regs(
                ssm,
                _dev,
                capsm_1001_writev_2.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 2]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        4 => {
            sm_write_regs(
                ssm,
                _dev,
                capsm_1001_writev_3.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 9]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        5 => {
            sm_write_regs(
                ssm,
                _dev,
                capsm_1001_writev_4.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 5]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        6 => {
            sm_write_regs(
                ssm,
                _dev,
                capsm_1001_writev_5.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 29]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn deinitsm_2016_run_state(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
) {
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            sm_write_regs(
                ssm,
                _dev,
                deinitsm_2016_writev.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 4]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn deinitsm_1000_run_state(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
) {
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            sm_write_regs(
                ssm,
                _dev,
                deinitsm_1000_writev.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 5]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn deinitsm_1001_run_state(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
) {
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            sm_write_regs(
                ssm,
                _dev,
                deinitsm_1001_writev.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 4]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn initsm_2016_run_state(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(_dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            sm_write_regs(
                ssm,
                _dev,
                initsm_2016_writev_1.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 15]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        1 => {
            sm_read_reg(ssm, dev, 0x9 as libc::c_int as guint8);
        }
        2 => {
            sm_write_reg(
                ssm,
                dev,
                0x9 as libc::c_int as guint8,
                ((*self_0).read_reg_result as libc::c_int & !(0x8 as libc::c_int))
                    as guint8,
            );
        }
        3 => {
            sm_read_reg(ssm, dev, 0x13 as libc::c_int as guint8);
        }
        4 => {
            sm_write_reg(
                ssm,
                dev,
                0x13 as libc::c_int as guint8,
                ((*self_0).read_reg_result as libc::c_int & !(0x10 as libc::c_int))
                    as guint8,
            );
        }
        5 => {
            sm_write_reg(
                ssm,
                dev,
                0x4 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
            );
        }
        6 => {
            sm_write_reg(
                ssm,
                dev,
                0x5 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn initsm_1000_run_state(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
) {
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            sm_write_regs(
                ssm,
                _dev,
                initsm_1000_writev_1.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 13]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn initsm_1001_run_state(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
) {
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            sm_write_regs(
                ssm,
                _dev,
                initsm_1001_writev_1.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 18]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        1 => {
            sm_write_regs(
                ssm,
                _dev,
                initsm_1001_writev_2.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 3]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        2 => {
            sm_write_regs(
                ssm,
                _dev,
                initsm_1001_writev_3.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 7]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        3 => {
            sm_write_regs(
                ssm,
                _dev,
                initsm_1001_writev_4.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 88]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        4 => {
            sm_write_regs(
                ssm,
                _dev,
                initsm_1001_writev_5.as_ptr(),
                (::core::mem::size_of::<[sonly_regwrite; 2]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<sonly_regwrite>() as libc::c_ulong,
                    ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn loopsm_run_state(mut ssm: *mut FpiSsm, mut _dev: *mut FpDevice) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(_dev as gpointer);
    let mut capsm: *mut FpiSsm = 0 as *mut FpiSsm;
    let mut deinitsm: *mut FpiSsm = 0 as *mut FpiSsm;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            match (*self_0).dev_model {
                2 => {
                    if (*self_0).deactivating != 0 {
                        fpi_ssm_mark_completed(ssm);
                    } else {
                        fpi_ssm_next_state(ssm);
                    }
                }
                _ => {
                    if (*self_0).deactivating != 0 {
                        fpi_ssm_mark_completed(ssm);
                    } else {
                        let mut awfsm: *mut FpiSsm = 0 as *mut FpiSsm;
                        match (*self_0).dev_model {
                            0 => {
                                awfsm = fpi_ssm_new_full(
                                    FP_DEVICE(dev as gpointer),
                                    Some(
                                        awfsm_2016_run_state
                                            as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                                    ),
                                    AWFSM_2016_NUM_STATES as libc::c_int,
                                    AWFSM_2016_NUM_STATES as libc::c_int,
                                    b"AWFSM_2016_NUM_STATES\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            1 => {
                                awfsm = fpi_ssm_new_full(
                                    FP_DEVICE(dev as gpointer),
                                    Some(
                                        awfsm_1000_run_state
                                            as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                                    ),
                                    AWFSM_1000_NUM_STATES as libc::c_int,
                                    AWFSM_1000_NUM_STATES as libc::c_int,
                                    b"AWFSM_1000_NUM_STATES\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            _ => {
                                g_assertion_message_expr(
                                    b"libfprint-upeksonly\0" as *const u8
                                        as *const libc::c_char,
                                    b"../libfprint/drivers/upeksonly.c\0" as *const u8
                                        as *const libc::c_char,
                                    1232 as libc::c_int,
                                    (*::core::mem::transmute::<
                                        &[u8; 17],
                                        &[libc::c_char; 17],
                                    >(b"loopsm_run_state\0"))
                                        .as_ptr(),
                                    0 as *const libc::c_char,
                                );
                            }
                        }
                        fpi_ssm_start_subsm(ssm, awfsm);
                    }
                }
            }
        }
        1 => {
            match (*self_0).dev_model {
                2 => {
                    fpi_ssm_next_state(ssm);
                }
                _ => {
                    sm_await_intr(ssm, dev);
                }
            }
        }
        2 => {
            capsm = 0 as *mut FpiSsm;
            match (*self_0).dev_model {
                0 => {
                    capsm = fpi_ssm_new_full(
                        FP_DEVICE(dev as gpointer),
                        Some(
                            capsm_2016_run_state
                                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                        ),
                        CAPSM_2016_NUM_STATES as libc::c_int,
                        CAPSM_2016_NUM_STATES as libc::c_int,
                        b"CAPSM_2016_NUM_STATES\0" as *const u8 as *const libc::c_char,
                    );
                }
                1 => {
                    capsm = fpi_ssm_new_full(
                        FP_DEVICE(dev as gpointer),
                        Some(
                            capsm_1000_run_state
                                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                        ),
                        CAPSM_1000_NUM_STATES as libc::c_int,
                        CAPSM_1000_NUM_STATES as libc::c_int,
                        b"CAPSM_1000_NUM_STATES\0" as *const u8 as *const libc::c_char,
                    );
                }
                2 => {
                    capsm = fpi_ssm_new_full(
                        FP_DEVICE(dev as gpointer),
                        Some(
                            capsm_1001_run_state
                                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                        ),
                        CAPSM_1001_NUM_STATES as libc::c_int,
                        CAPSM_1001_NUM_STATES as libc::c_int,
                        b"CAPSM_1001_NUM_STATES\0" as *const u8 as *const libc::c_char,
                    );
                }
                _ => {
                    g_assertion_message_expr(
                        b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
                        b"../libfprint/drivers/upeksonly.c\0" as *const u8
                            as *const libc::c_char,
                        1276 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 17],
                            &[libc::c_char; 17],
                        >(b"loopsm_run_state\0"))
                            .as_ptr(),
                        0 as *const libc::c_char,
                    );
                }
            }
            fpi_ssm_start_subsm(ssm, capsm);
        }
        4 => {
            deinitsm = 0 as *mut FpiSsm;
            match (*self_0).dev_model {
                0 => {
                    deinitsm = fpi_ssm_new_full(
                        FP_DEVICE(dev as gpointer),
                        Some(
                            deinitsm_2016_run_state
                                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                        ),
                        DEINITSM_2016_NUM_STATES as libc::c_int,
                        DEINITSM_2016_NUM_STATES as libc::c_int,
                        b"DEINITSM_2016_NUM_STATES\0" as *const u8 as *const libc::c_char,
                    );
                }
                1 => {
                    deinitsm = fpi_ssm_new_full(
                        FP_DEVICE(dev as gpointer),
                        Some(
                            deinitsm_1000_run_state
                                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                        ),
                        DEINITSM_1000_NUM_STATES as libc::c_int,
                        DEINITSM_1000_NUM_STATES as libc::c_int,
                        b"DEINITSM_1000_NUM_STATES\0" as *const u8 as *const libc::c_char,
                    );
                }
                2 => {
                    deinitsm = fpi_ssm_new_full(
                        FP_DEVICE(dev as gpointer),
                        Some(
                            deinitsm_1001_run_state
                                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                        ),
                        DEINITSM_1001_NUM_STATES as libc::c_int,
                        DEINITSM_1001_NUM_STATES as libc::c_int,
                        b"DEINITSM_1001_NUM_STATES\0" as *const u8 as *const libc::c_char,
                    );
                }
                _ => {
                    g_assertion_message_expr(
                        b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
                        b"../libfprint/drivers/upeksonly.c\0" as *const u8
                            as *const libc::c_char,
                        1307 as libc::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 17],
                            &[libc::c_char; 17],
                        >(b"loopsm_run_state\0"))
                            .as_ptr(),
                        0 as *const libc::c_char,
                    );
                }
            }
            (*self_0).capturing = 0 as libc::c_int;
            fpi_ssm_start_subsm(ssm, deinitsm);
        }
        5 => {
            fpi_ssm_jump_to_state(ssm, LOOPSM_RUN_AWFSM as libc::c_int);
        }
        3 | _ => {}
    };
}
unsafe extern "C" fn deactivate_done(
    mut dev: *mut FpImageDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(dev as gpointer);
    g_log_structured(
        b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/upeksonly.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"1327\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"deactivate_done\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/upeksonly.c:1327\0" as *const u8 as *const libc::c_char,
    );
    free_img_transfers(self_0);
    g_free((*self_0).rowbuf as gpointer);
    (*self_0).rowbuf = 0 as *mut libc::c_uchar;
    g_slist_free_full(
        (*self_0).rows,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*self_0).rows = 0 as *mut GSList;
    fpi_image_device_deactivate_complete(dev, error);
}
unsafe extern "C" fn dev_deactivate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(dev as gpointer);
    if (*self_0).capturing == 0 {
        deactivate_done(dev, 0 as *mut GError);
        return;
    }
    (*self_0).deactivating = (0 as libc::c_int == 0) as libc::c_int;
    (*self_0).killing_transfers = ITERATE_SSM;
    (*self_0).kill_ssm = (*self_0).loopsm;
    cancel_img_transfers(dev);
}
unsafe extern "C" fn loopsm_complete(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(_dev as gpointer);
    if (*self_0).deactivating != 0 {
        deactivate_done(dev, error);
        return;
    }
    if !error.is_null() {
        fpi_image_device_session_error(dev, error);
        return;
    }
}
unsafe extern "C" fn initsm_complete(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(_dev as gpointer);
    fpi_image_device_activate_complete(dev, error);
    if !error.is_null() {
        return;
    }
    (*self_0)
        .loopsm = fpi_ssm_new_full(
        FP_DEVICE(dev as gpointer),
        Some(loopsm_run_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
        LOOPSM_NUM_STATES as libc::c_int,
        LOOPSM_NUM_STATES as libc::c_int,
        b"LOOPSM_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        (*self_0).loopsm,
        Some(
            loopsm_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn dev_activate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(dev as gpointer);
    let mut ssm: *mut FpiSsm = 0 as *mut FpiSsm;
    let mut i: libc::c_int = 0;
    (*self_0).deactivating = 0 as libc::c_int;
    (*self_0).capturing = 0 as libc::c_int;
    (*self_0).num_flying = 0 as libc::c_int;
    (*self_0)
        .img_transfers = g_ptr_array_new_with_free_func(
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FpiUsbTransfer) -> ()>,
            GFreeFunc,
        >(
            Some(
                fpi_usb_transfer_unref as unsafe extern "C" fn(*mut FpiUsbTransfer) -> (),
            ),
        ),
    );
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
        transfer = fpi_usb_transfer_new(FP_DEVICE(dev as gpointer));
        fpi_usb_transfer_fill_bulk(
            transfer,
            0x81 as libc::c_int as guint8,
            4096 as libc::c_int as gsize,
        );
        g_ptr_array_add((*self_0).img_transfers, transfer as gpointer);
        i += 1;
    }
    match (*self_0).dev_model {
        0 => {
            ssm = fpi_ssm_new_full(
                FP_DEVICE(dev as gpointer),
                Some(
                    initsm_2016_run_state
                        as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                ),
                INITSM_2016_NUM_STATES as libc::c_int,
                INITSM_2016_NUM_STATES as libc::c_int,
                b"INITSM_2016_NUM_STATES\0" as *const u8 as *const libc::c_char,
            );
        }
        1 => {
            ssm = fpi_ssm_new_full(
                FP_DEVICE(dev as gpointer),
                Some(
                    initsm_1000_run_state
                        as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                ),
                INITSM_1000_NUM_STATES as libc::c_int,
                INITSM_1000_NUM_STATES as libc::c_int,
                b"INITSM_1000_NUM_STATES\0" as *const u8 as *const libc::c_char,
            );
        }
        2 => {
            ssm = fpi_ssm_new_full(
                FP_DEVICE(dev as gpointer),
                Some(
                    initsm_1001_run_state
                        as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                ),
                INITSM_1001_NUM_STATES as libc::c_int,
                INITSM_1001_NUM_STATES as libc::c_int,
                b"INITSM_1001_NUM_STATES\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {
            g_assertion_message_expr(
                b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
                b"../libfprint/drivers/upeksonly.c\0" as *const u8
                    as *const libc::c_char,
                1434 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"dev_activate\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    }
    fpi_ssm_start(
        ssm,
        Some(
            initsm_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn dev_deinit(mut dev: *mut FpImageDevice) {
    let mut error: *mut GError = 0 as *mut GError;
    g_usb_device_release_interface(
        fpi_device_get_usb_device(FP_DEVICE(dev as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    );
    fpi_image_device_close_complete(dev, error);
}
unsafe extern "C" fn dev_discover(mut usb_device: *mut GUsbDevice) -> gint {
    let mut pid: guint16 = g_usb_device_get_pid(usb_device);
    let mut bcd: guint16 = g_usb_device_get_release(usb_device);
    if pid as libc::c_int == 0x2016 as libc::c_int {
        if bcd as libc::c_int == 1 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    if pid as libc::c_int == 0x1000 as libc::c_int {
        if bcd as libc::c_int == 0x33 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    if pid as libc::c_int == 0x1001 as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
static mut id_table: [FpIdEntry; 4] = [
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
            driver_data: UPEKSONLY_2016 as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x1000 as libc::c_int as guint,
                        vid: 0x147e as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: UPEKSONLY_1000 as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x1001 as libc::c_int as guint,
                        vid: 0x147e as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: UPEKSONLY_1001 as libc::c_int as guint64,
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
unsafe extern "C" fn fpi_device_upeksonly_init(mut self_0: *mut FpiDeviceUpeksonly) {}
unsafe extern "C" fn fpi_device_upeksonly_class_init(
    mut klass: *mut FpiDeviceUpeksonlyClass,
) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    let mut img_class: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_CLASS(
        klass as gpointer,
    );
    (*dev_class).id = b"upeksonly\0" as *const u8 as *const libc::c_char;
    (*dev_class)
        .full_name = b"UPEK TouchStrip Sensor-Only\0" as *const u8
        as *const libc::c_char;
    (*dev_class).type_0 = FP_DEVICE_TYPE_USB;
    (*dev_class).id_table = id_table.as_ptr();
    (*dev_class).scan_type = FP_SCAN_TYPE_SWIPE;
    (*dev_class)
        .usb_discover = Some(
        dev_discover as unsafe extern "C" fn(*mut GUsbDevice) -> gint,
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
    (*img_class).img_width = -(1 as libc::c_int);
    (*img_class).img_height = -(1 as libc::c_int);
}
unsafe extern "C" fn dev_init(mut dev: *mut FpImageDevice) {
    let mut error: *mut GError = 0 as *mut GError;
    let mut self_0: *mut FpiDeviceUpeksonly = FPI_DEVICE_UPEKSONLY(dev as gpointer);
    if g_usb_device_set_configuration(
        fpi_device_get_usb_device(FP_DEVICE(dev as gpointer)),
        1 as libc::c_int,
        &mut error,
    ) == 0
    {
        g_log(
            b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"could not set configuration 1\0" as *const u8 as *const libc::c_char,
        );
        fpi_image_device_open_complete(dev, error);
    }
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
    (*self_0).assembling_ctx.max_height = 1024 as libc::c_int as libc::c_uint;
    (*self_0).assembling_ctx.resolution = 8 as libc::c_int as libc::c_uint;
    (*self_0).assembling_ctx.median_filter_size = 25 as libc::c_int as libc::c_uint;
    (*self_0).assembling_ctx.max_search_offset = 30 as libc::c_int as libc::c_uint;
    (*self_0)
        .assembling_ctx
        .get_deviation = Some(
        upeksonly_get_deviation2
            as unsafe extern "C" fn(
                *mut fpi_line_asmbl_ctx,
                *mut GSList,
                *mut GSList,
            ) -> libc::c_int,
    );
    (*self_0)
        .assembling_ctx
        .get_pixel = Some(
        upeksonly_get_pixel
            as unsafe extern "C" fn(
                *mut fpi_line_asmbl_ctx,
                *mut GSList,
                libc::c_uint,
            ) -> libc::c_uchar,
    );
    self_0 = FPI_DEVICE_UPEKSONLY(dev as gpointer);
    (*self_0)
        .dev_model = fpi_device_get_driver_data(FP_DEVICE(dev as gpointer))
        as libc::c_int;
    match (*self_0).dev_model {
        1 => {
            (*self_0).img_width = 288 as libc::c_int;
            (*self_0).assembling_ctx.line_width = 288 as libc::c_int as libc::c_uint;
        }
        2 => {
            (*self_0).img_width = 216 as libc::c_int;
            (*self_0).assembling_ctx.line_width = 216 as libc::c_int as libc::c_uint;
            fpi_image_device_set_bz3_threshold(dev, 25 as libc::c_int);
        }
        0 => {
            (*self_0).img_width = 288 as libc::c_int;
            (*self_0).assembling_ctx.line_width = 288 as libc::c_int as libc::c_uint;
        }
        _ => {
            g_assertion_message_expr(
                b"libfprint-upeksonly\0" as *const u8 as *const libc::c_char,
                b"../libfprint/drivers/upeksonly.c\0" as *const u8
                    as *const libc::c_char,
                1552 as libc::c_int,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"dev_init\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    }
    fpi_image_device_open_complete(dev, 0 as *mut GError);
}
