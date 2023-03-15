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
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
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
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
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
    fn fpi_usb_transfer_ref(self_0: *mut FpiUsbTransfer) -> *mut FpiUsbTransfer;
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
    fn fpi_ssm_next_state_delayed(machine: *mut FpiSsm, delay: libc::c_int);
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
pub type C2RustUnnamed_4 = libc::c_uint;
pub const M_LOOP_NUM_STATES: C2RustUnnamed_4 = 7;
pub const M_SUBMIT_PRINT: C2RustUnnamed_4 = 6;
pub const M_READ_PRINT_POLL: C2RustUnnamed_4 = 5;
pub const M_READ_PRINT_START: C2RustUnnamed_4 = 4;
pub const M_READ_PRINT_PRESTART: C2RustUnnamed_4 = 3;
pub const M_CHECK_PRINT: C2RustUnnamed_4 = 2;
pub const M_REQUEST_PRINT: C2RustUnnamed_4 = 1;
pub const M_WAIT_PRINT: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceNb1010 {
    pub parent: FpImageDevice,
    pub ssm: *mut FpiSsm,
    pub scanline_buf: *mut guint8,
    pub deactivating: gboolean,
    pub partial_received: libc::c_int,
}
pub type FpiDeviceNb1010 = _FpiDeviceNb1010;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiDeviceNb1010Class {
    pub parent_class: FpImageDeviceClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
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
static mut nb1010_cmd_check_finger: [guint8; 8] = [
    0x80 as libc::c_int as guint8,
    0x38 as libc::c_int as guint8,
    0x1 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x8 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
];
static mut nb1010_cmd_precapture: [guint8; 12] = [
    0x80 as libc::c_int as guint8,
    0xd as libc::c_int as guint8,
    0x3 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0xc as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x4 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
];
static mut nb1010_cmd_capture: [guint8; 32] = [
    0x80 as libc::c_int as guint8,
    0x12 as libc::c_int as guint8,
    0x4 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x20 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x2 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0x20 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
    0 as libc::c_int as guint8,
];
#[inline]
unsafe extern "C" fn FPI_DEVICE_NB1010(mut ptr: gpointer) -> *mut FpiDeviceNb1010 {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_nb1010_get_type(),
    ) as *mut libc::c_void as *mut FpiDeviceNb1010;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_nb1010_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_nb1010_get_type_once();
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
static mut fpi_device_nb1010_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut FpiDeviceNb1010_private_offset: gint = 0;
unsafe extern "C" fn fpi_device_nb1010_class_intern_init(mut klass: gpointer) {
    fpi_device_nb1010_parent_class = g_type_class_peek_parent(klass);
    if FpiDeviceNb1010_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpiDeviceNb1010_private_offset);
    }
    fpi_device_nb1010_class_init(klass as *mut FpiDeviceNb1010Class);
}
#[inline(never)]
unsafe extern "C" fn fpi_device_nb1010_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_image_device_get_type(),
        g_intern_static_string(b"FpiDeviceNb1010\0" as *const u8 as *const libc::c_char),
        ::core::mem::size_of::<FpiDeviceNb1010Class>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_nb1010_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpiDeviceNb1010>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpiDeviceNb1010) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_nb1010_init
                        as unsafe extern "C" fn(*mut FpiDeviceNb1010) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn nb1010_dev_init(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceNb1010 = FPI_DEVICE_NB1010(dev as gpointer);
    let mut error: *mut GError = 0 as *mut GError;
    g_usb_device_claim_interface(
        fpi_device_get_usb_device(FP_DEVICE(dev as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    );
    (*self_0)
        .scanline_buf = g_malloc0((256 as libc::c_int * 180 as libc::c_int) as gsize)
        as *mut guint8;
    fpi_image_device_open_complete(dev, error);
    g_log(
        b"libfprint-nb1010\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"nb1010 Initialized\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn nb1010_dev_deinit(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceNb1010 = FPI_DEVICE_NB1010(dev as gpointer);
    let mut error: *mut GError = 0 as *mut GError;
    let mut _pp: C2RustUnnamed_5 = C2RustUnnamed_5 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*self_0).scanline_buf as *mut *mut guint8 as *mut libc::c_char;
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
    g_log(
        b"libfprint-nb1010\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"nb1010 Deinitialized\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn nb1010_dev_activate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceNb1010 = FPI_DEVICE_NB1010(dev as gpointer);
    (*self_0).deactivating = 0 as libc::c_int;
    fpi_image_device_activate_complete(dev, 0 as *mut GError);
    g_log(
        b"libfprint-nb1010\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"nb1010 Activated\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn nb1010_dev_deactivated(
    mut dev: *mut FpImageDevice,
    mut err: *mut GError,
) {
    fpi_image_device_deactivate_complete(dev, err);
    g_log(
        b"libfprint-nb1010\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"nb1010 Deactivated\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn nb1010_dev_deactivate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceNb1010 = FPI_DEVICE_NB1010(dev as gpointer);
    (*self_0).deactivating = (0 as libc::c_int == 0) as libc::c_int;
    if ((*self_0).ssm).is_null() {
        nb1010_dev_deactivated(dev, 0 as *mut GError);
    }
}
unsafe extern "C" fn nb1010_request_fingerprint(mut dev: *mut FpiDeviceNb1010) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    transfer = fpi_usb_transfer_new(FP_DEVICE(dev as gpointer));
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    (*transfer).ssm = (*dev).ssm;
    fpi_usb_transfer_fill_bulk_full(
        transfer,
        (0x2 as libc::c_int | 0 as libc::c_int) as guint8,
        nb1010_cmd_check_finger.as_mut_ptr(),
        (::core::mem::size_of::<[guint8; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<guint8>() as libc::c_ulong),
        None,
    );
    fpi_usb_transfer_submit(
        transfer,
        500 as libc::c_int as guint,
        fpi_device_get_cancellable(FP_DEVICE(dev as gpointer)),
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
unsafe extern "C" fn nb1010_check_fingerprint_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut dev: *mut FpDevice,
    mut unused_data: gpointer,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceNb1010 = FPI_DEVICE_NB1010(dev as gpointer);
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    if (*self_0).deactivating != 0 {
        fpi_ssm_mark_completed((*transfer).ssm);
        return;
    }
    if *((*transfer).buffer).offset(12 as libc::c_int as isize) as libc::c_int
        > 0x30 as libc::c_int
    {
        fpi_ssm_next_state((*transfer).ssm);
    } else {
        fpi_ssm_jump_to_state((*transfer).ssm, M_WAIT_PRINT as libc::c_int);
    };
}
unsafe extern "C" fn nb1010_cmd_check_fingerprint(mut dev: *mut FpiDeviceNb1010) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    transfer = fpi_usb_transfer_new(FP_DEVICE(dev as gpointer));
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    (*transfer).ssm = (*dev).ssm;
    fpi_usb_transfer_fill_bulk(
        transfer,
        (0x3 as libc::c_int | 0x80 as libc::c_int) as guint8,
        16 as libc::c_int as gsize,
    );
    fpi_usb_transfer_submit(
        transfer,
        500 as libc::c_int as guint,
        fpi_device_get_cancellable(FP_DEVICE(dev as gpointer)),
        Some(
            nb1010_check_fingerprint_cb
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
unsafe extern "C" fn nb1010_read_ignore_data_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut dev: *mut FpDevice,
    mut unused_data: gpointer,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceNb1010 = FPI_DEVICE_NB1010(dev as gpointer);
    let mut new_transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    if (*self_0).deactivating != 0 {
        fpi_ssm_mark_completed((*transfer).ssm);
        return;
    }
    new_transfer = fpi_usb_transfer_new(dev);
    (*new_transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    (*new_transfer).ssm = (*transfer).ssm;
    fpi_usb_transfer_fill_bulk(
        new_transfer,
        (0x3 as libc::c_int | 0x80 as libc::c_int) as guint8,
        16 as libc::c_int as gsize,
    );
    fpi_usb_transfer_submit(
        new_transfer,
        500 as libc::c_int as guint,
        fpi_device_get_cancellable(FP_DEVICE(dev as gpointer)),
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
unsafe extern "C" fn nb1010_write_ignore_read(
    mut dev: *mut FpiDeviceNb1010,
    mut buf: *mut guint8,
    mut len: gsize,
) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    transfer = fpi_usb_transfer_new(FP_DEVICE(dev as gpointer));
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    (*transfer).ssm = (*dev).ssm;
    fpi_usb_transfer_fill_bulk_full(
        transfer,
        (0x2 as libc::c_int | 0 as libc::c_int) as guint8,
        buf,
        len,
        None,
    );
    fpi_usb_transfer_submit(
        transfer,
        500 as libc::c_int as guint,
        fpi_device_get_cancellable(FP_DEVICE(dev as gpointer)),
        Some(
            nb1010_read_ignore_data_cb
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
unsafe extern "C" fn nb1010_read_capture_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut dev: *mut FpDevice,
    mut unused_data: gpointer,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceNb1010 = FPI_DEVICE_NB1010(dev as gpointer);
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    if (*self_0).deactivating != 0 {
        fpi_ssm_mark_completed((*transfer).ssm);
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*transfer).actual_length == 540 as libc::c_int as libc::c_long {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-nb1010\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/nb1010.c\0" as *const u8 as *const libc::c_char,
            272 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"nb1010_read_capture_cb\0"))
                .as_ptr(),
            b"transfer->actual_length == NB1010_CAPTURE_RECV_LEN\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut offset: size_t = ((*self_0).partial_received * 2 as libc::c_int
        * 256 as libc::c_int) as size_t;
    memcpy(
        ((*self_0).scanline_buf).offset(offset as isize) as *mut libc::c_void,
        ((*transfer).buffer).offset(25 as libc::c_int as isize) as *const libc::c_void,
        (2 as libc::c_int * 256 as libc::c_int) as libc::c_ulong,
    );
    (*self_0).partial_received += 1;
    if (*self_0).partial_received == 180 as libc::c_int / 2 as libc::c_int {
        fpi_ssm_next_state((*transfer).ssm);
        return;
    }
    fpi_usb_transfer_submit(
        fpi_usb_transfer_ref(transfer),
        500 as libc::c_int as guint,
        fpi_device_get_cancellable(FP_DEVICE(dev as gpointer)),
        Some(
            nb1010_read_capture_cb
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
unsafe extern "C" fn nb1010_read_capture(mut dev: *mut FpiDeviceNb1010) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    transfer = fpi_usb_transfer_new(FP_DEVICE(dev as gpointer));
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    (*transfer).ssm = (*dev).ssm;
    fpi_usb_transfer_fill_bulk(
        transfer,
        (0x3 as libc::c_int | 0x80 as libc::c_int) as guint8,
        540 as libc::c_int as gsize,
    );
    fpi_usb_transfer_submit(
        transfer,
        500 as libc::c_int as guint,
        fpi_device_get_cancellable(FP_DEVICE(dev as gpointer)),
        Some(
            nb1010_read_capture_cb
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
unsafe extern "C" fn submit_image(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
) -> libc::c_int {
    let mut self_0: *mut FpiDeviceNb1010 = FPI_DEVICE_NB1010(dev as gpointer);
    let mut img: *mut FpImage = 0 as *mut FpImage;
    img = fp_image_new(256 as libc::c_int, 180 as libc::c_int);
    if img.is_null() {
        return 0 as libc::c_int;
    }
    memcpy(
        (*img).data as *mut libc::c_void,
        (*self_0).scanline_buf as *const libc::c_void,
        (256 as libc::c_int * 180 as libc::c_int) as libc::c_ulong,
    );
    fpi_image_device_image_captured(dev, img);
    return 1 as libc::c_int;
}
unsafe extern "C" fn m_loop_complete(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
    mut error: *mut GError,
) {
    g_log(
        b"libfprint-nb1010\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"nb1010 ssm complete cb\0" as *const u8 as *const libc::c_char,
    );
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceNb1010 = FPI_DEVICE_NB1010(_dev as gpointer);
    (*self_0).ssm = 0 as *mut FpiSsm;
    if (*self_0).deactivating != 0 {
        nb1010_dev_deactivated(dev, error);
    } else if !error.is_null() {
        fpi_image_device_session_error(dev, error);
    }
}
unsafe extern "C" fn m_loop_state(mut ssm: *mut FpiSsm, mut _dev: *mut FpDevice) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceNb1010 = FPI_DEVICE_NB1010(_dev as gpointer);
    if (*self_0).deactivating != 0 {
        g_log(
            b"libfprint-nb1010\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"deactivating, marking completed\0" as *const u8 as *const libc::c_char,
        );
        fpi_ssm_mark_completed(ssm);
        return;
    }
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            fpi_ssm_next_state_delayed(ssm, 50 as libc::c_int);
        }
        1 => {
            nb1010_request_fingerprint(self_0);
        }
        2 => {
            nb1010_cmd_check_fingerprint(self_0);
        }
        3 => {
            fpi_image_device_report_finger_status(
                dev,
                (0 as libc::c_int == 0) as libc::c_int,
            );
            nb1010_write_ignore_read(
                self_0,
                nb1010_cmd_precapture.as_mut_ptr(),
                (::core::mem::size_of::<[guint8; 12]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<guint8>() as libc::c_ulong),
            );
        }
        4 => {
            (*self_0).partial_received = 0 as libc::c_int;
            nb1010_write_ignore_read(
                self_0,
                nb1010_cmd_capture.as_mut_ptr(),
                (::core::mem::size_of::<[guint8; 32]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<guint8>() as libc::c_ulong),
            );
        }
        5 => {
            nb1010_read_capture(self_0);
        }
        6 => {
            if submit_image(ssm, dev) != 0 {
                fpi_ssm_mark_completed(ssm);
                fpi_image_device_report_finger_status(dev, 0 as libc::c_int);
            } else {
                fpi_ssm_jump_to_state(ssm, M_WAIT_PRINT as libc::c_int);
            }
        }
        _ => {
            g_assertion_message_expr(
                b"libfprint-nb1010\0" as *const u8 as *const libc::c_char,
                b"../libfprint/drivers/nb1010.c\0" as *const u8 as *const libc::c_char,
                394 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"m_loop_state\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn nb1010_dev_change_state(
    mut dev: *mut FpImageDevice,
    mut state: FpiImageDeviceState,
) {
    let mut self_0: *mut FpiDeviceNb1010 = FPI_DEVICE_NB1010(dev as gpointer);
    let mut ssm_loop: *mut FpiSsm = 0 as *mut FpiSsm;
    if state as libc::c_uint
        == FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_ON as libc::c_int as libc::c_uint
    {
        ssm_loop = fpi_ssm_new_full(
            FP_DEVICE(dev as gpointer),
            Some(m_loop_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
            M_LOOP_NUM_STATES as libc::c_int,
            M_LOOP_NUM_STATES as libc::c_int,
            b"M_LOOP_NUM_STATES\0" as *const u8 as *const libc::c_char,
        );
        (*self_0).ssm = ssm_loop;
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
static mut id_table: [FpIdEntry; 2] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x1010 as libc::c_int as guint,
                        vid: 0x298d as libc::c_int as guint,
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
unsafe extern "C" fn fpi_device_nb1010_init(mut self_0: *mut FpiDeviceNb1010) {}
unsafe extern "C" fn fpi_device_nb1010_class_init(mut klass: *mut FpiDeviceNb1010Class) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    let mut img_class: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_CLASS(
        klass as gpointer,
    );
    (*dev_class).id = b"nb1010\0" as *const u8 as *const libc::c_char;
    (*dev_class)
        .full_name = b"NextBiometrics NB-1010-U\0" as *const u8 as *const libc::c_char;
    (*dev_class).type_0 = FP_DEVICE_TYPE_USB;
    (*dev_class).id_table = id_table.as_ptr();
    (*dev_class).scan_type = FP_SCAN_TYPE_PRESS;
    (*img_class).img_height = 180 as libc::c_int;
    (*img_class).img_width = 256 as libc::c_int;
    (*img_class).bz3_threshold = 24 as libc::c_int;
    (*img_class)
        .img_open = Some(
        nb1010_dev_init as unsafe extern "C" fn(*mut FpImageDevice) -> (),
    );
    (*img_class)
        .img_close = Some(
        nb1010_dev_deinit as unsafe extern "C" fn(*mut FpImageDevice) -> (),
    );
    (*img_class)
        .activate = Some(
        nb1010_dev_activate as unsafe extern "C" fn(*mut FpImageDevice) -> (),
    );
    (*img_class)
        .deactivate = Some(
        nb1010_dev_deactivate as unsafe extern "C" fn(*mut FpImageDevice) -> (),
    );
    (*img_class)
        .change_state = Some(
        nb1010_dev_change_state
            as unsafe extern "C" fn(*mut FpImageDevice, FpiImageDeviceState) -> (),
    );
}
