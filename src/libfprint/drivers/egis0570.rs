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
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn fpi_ssm_usb_transfer_cb(
        transfer: *mut FpiUsbTransfer,
        device: *mut FpDevice,
        unused_data: gpointer,
        error: *mut GError,
    );
    fn fpi_ssm_get_cur_state(machine: *mut FpiSsm) -> libc::c_int;
    fn fpi_ssm_mark_failed(machine: *mut FpiSsm, error: *mut GError);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_reverse(list: *mut GSList) -> *mut GSList;
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
    fn g_object_get(object: gpointer, first_property_name: *const gchar, _: ...);
    fn fpi_do_movement_estimation(ctx: *mut fpi_frame_asmbl_ctx, stripes: *mut GSList);
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
    fn fpi_image_resize(
        orig: *mut FpImage,
        w_factor: guint,
        h_factor: guint,
    ) -> *mut FpImage;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpDeviceEgis0570 {
    pub parent: FpImageDevice,
    pub running: gboolean,
    pub stop: gboolean,
    pub strips: *mut GSList,
    pub background: *mut guint8,
    pub strips_len: gsize,
    pub pkt_num: libc::c_int,
    pub pkt_type: libc::c_int,
}
pub type FpDeviceEgis0570 = _FpDeviceEgis0570;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpDeviceEgis0570Class {
    pub parent_class: FpImageDeviceClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub const SM_STATES_NUM: sm_states = 6;
pub const SM_START: sm_states = 1;
pub const SM_DONE: sm_states = 5;
pub const SM_REC_DATA: sm_states = 4;
pub const SM_REQ: sm_states = 2;
pub const SM_RESP: sm_states = 3;
pub const SM_INIT: sm_states = 0;
pub type sm_states = libc::c_uint;
static mut init_pkts: [[libc::c_uchar; 7]; 23] = [
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x20 as libc::c_int as libc::c_uchar,
        0x3f as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x58 as libc::c_int as libc::c_uchar,
        0x3f as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x21 as libc::c_int as libc::c_uchar,
        0x9 as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x57 as libc::c_int as libc::c_uchar,
        0x9 as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x22 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x56 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x23 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x55 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x24 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x54 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x16 as libc::c_int as libc::c_uchar,
        0x3e as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x9 as libc::c_int as libc::c_uchar,
        0xb as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x14 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x15 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0xf as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x10 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0x38 as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x12 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x13 as libc::c_int as libc::c_uchar,
        0x71 as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x80 as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x80 as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2f as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x6 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
    ],
];
static mut repeat_pkts: [[libc::c_uchar; 7]; 4] = [
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0xf as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0xf as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2f as libc::c_int as libc::c_uchar,
    ],
    [
        0x45 as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x6 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xfe as libc::c_int as libc::c_uchar,
    ],
];
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
#[inline]
unsafe extern "C" fn FPI_DEVICE_EGIS0570(mut ptr: gpointer) -> *mut FpDeviceEgis0570 {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_egis0570_get_type(),
    ) as *mut libc::c_void as *mut FpDeviceEgis0570;
}
unsafe extern "C" fn fpi_device_egis0570_class_intern_init(mut klass: gpointer) {
    fpi_device_egis0570_parent_class = g_type_class_peek_parent(klass);
    if FpDeviceEgis0570_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpDeviceEgis0570_private_offset);
    }
    fpi_device_egis0570_class_init(klass as *mut FpDeviceEgis0570Class);
}
static mut fpi_device_egis0570_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
#[inline(never)]
unsafe extern "C" fn fpi_device_egis0570_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_image_device_get_type(),
        g_intern_static_string(
            b"FpDeviceEgis0570\0" as *const u8 as *const libc::c_char,
        ),
        ::core::mem::size_of::<FpDeviceEgis0570Class>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_egis0570_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpDeviceEgis0570>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpDeviceEgis0570) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_egis0570_init
                        as unsafe extern "C" fn(*mut FpDeviceEgis0570) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_egis0570_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_egis0570_get_type_once();
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
static mut FpDeviceEgis0570_private_offset: gint = 0;
unsafe extern "C" fn egis_get_pixel(
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
            frame_width: 114 as libc::c_int as libc::c_uint,
            frame_height: 17 as libc::c_int as libc::c_uint,
            image_width: (114 as libc::c_int * 4 as libc::c_int / 3 as libc::c_int)
                as libc::c_uint,
            get_pixel: Some(
                egis_get_pixel
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
unsafe extern "C" fn is_last_pkt(mut dev: *mut FpDevice) -> gboolean {
    let mut self_0: *mut FpDeviceEgis0570 = FPI_DEVICE_EGIS0570(dev as gpointer);
    let mut type_0: libc::c_int = (*self_0).pkt_type;
    let mut num: libc::c_int = (*self_0).pkt_num;
    let mut r: gboolean = 0;
    r = (type_0 == 0 as libc::c_int
        && num as libc::c_ulong
            == (::core::mem::size_of::<[[libc::c_uchar; 7]; 23]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<[libc::c_uchar; 7]>() as libc::c_ulong,
                )
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
    r
        |= (type_0 == 1 as libc::c_int
            && num as libc::c_ulong
                == (::core::mem::size_of::<[[libc::c_uchar; 7]; 4]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<[libc::c_uchar; 7]>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
    return r;
}
unsafe extern "C" fn postprocess_frames(
    mut self_0: *mut FpDeviceEgis0570,
    mut img: *mut guint8,
) -> libc::c_char {
    let mut mean: [size_t; 5] = [
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    ];
    if ((*self_0).background).is_null() {
        (*self_0)
            .background = g_malloc((114 as libc::c_int * 17 as libc::c_int) as gsize)
            as *mut guint8;
        memset(
            (*self_0).background as *mut libc::c_void,
            255 as libc::c_int,
            (114 as libc::c_int * 17 as libc::c_int) as libc::c_ulong,
        );
        let mut k: size_t = 0 as libc::c_int as size_t;
        while k < 5 as libc::c_int as libc::c_ulong {
            let mut frame: *mut guint8 = &mut *img
                .offset(
                    k
                        .wrapping_mul(6498 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            ((57 as libc::c_int - 17 as libc::c_int) / 2 as libc::c_int
                                * 114 as libc::c_int) as libc::c_ulong,
                        ) as isize,
                ) as *mut guint8;
            let mut i: size_t = 0 as libc::c_int as size_t;
            while i < (114 as libc::c_int * 17 as libc::c_int) as libc::c_ulong {
                *((*self_0).background)
                    .offset(
                        i as isize,
                    ) = (if (*((*self_0).background).offset(i as isize) as libc::c_int)
                    < *frame.offset(i as isize) as libc::c_int
                {
                    *((*self_0).background).offset(i as isize) as libc::c_int
                } else {
                    *frame.offset(i as isize) as libc::c_int
                }) as guint8;
                i = (i as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
            k = (k as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
        return 0 as libc::c_int as libc::c_char;
    }
    let mut k_0: size_t = 0 as libc::c_int as size_t;
    while k_0 < 5 as libc::c_int as libc::c_ulong {
        let mut frame_0: *mut guint8 = &mut *img
            .offset(
                k_0
                    .wrapping_mul(6498 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        ((57 as libc::c_int - 17 as libc::c_int) / 2 as libc::c_int
                            * 114 as libc::c_int) as libc::c_ulong,
                    ) as isize,
            ) as *mut guint8;
        let mut i_0: size_t = 0 as libc::c_int as size_t;
        while i_0 < (114 as libc::c_int * 17 as libc::c_int) as libc::c_ulong {
            if *frame_0.offset(i_0 as isize) as libc::c_int - 3 as libc::c_int
                > *((*self_0).background).offset(i_0 as isize) as libc::c_int
            {
                let ref mut fresh0 = *frame_0.offset(i_0 as isize);
                *fresh0 = (*fresh0 as libc::c_int
                    - *((*self_0).background).offset(i_0 as isize) as libc::c_int)
                    as guint8;
            } else {
                *frame_0.offset(i_0 as isize) = 0 as libc::c_int as guint8;
            }
            mean[k_0
                as usize] = (mean[k_0 as usize] as libc::c_ulong)
                .wrapping_add(*frame_0.offset(i_0 as isize) as libc::c_ulong) as size_t
                as size_t;
            i_0 = (i_0 as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
        mean[k_0
            as usize] = (mean[k_0 as usize] as libc::c_ulong)
            .wrapping_div((114 as libc::c_int * 17 as libc::c_int) as libc::c_ulong)
            as size_t as size_t;
        k_0 = (k_0 as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    let mut result: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut k_1: size_t = 0 as libc::c_int as size_t;
    while k_1 < 5 as libc::c_int as libc::c_ulong {
        g_log(
            b"libfprint-egis0570\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Finger status (picture number, mean) : %ld , %ld\0" as *const u8
                as *const libc::c_char,
            k_1,
            mean[k_1 as usize],
        );
        if mean[k_1 as usize] > 20 as libc::c_int as libc::c_ulong {
            result = (result as libc::c_int | (1 as libc::c_int) << k_1) as libc::c_char;
        }
        k_1 = (k_1 as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    return result;
}
unsafe extern "C" fn data_resp_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut dev: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut stripdata: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: gboolean = 0 as libc::c_int;
    let mut img_self: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    let mut self_0: *mut FpDeviceEgis0570 = FPI_DEVICE_EGIS0570(dev as gpointer);
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    let mut where_finger_is: libc::c_int = postprocess_frames(self_0, (*transfer).buffer)
        as libc::c_int;
    if where_finger_is > 0 as libc::c_int {
        let mut state: FpiImageDeviceState = FPI_IMAGE_DEVICE_STATE_INACTIVE;
        fpi_image_device_report_finger_status(
            img_self,
            (0 as libc::c_int == 0) as libc::c_int,
        );
        g_object_get(
            dev as gpointer,
            b"fpi-image-device-state\0" as *const u8 as *const libc::c_char,
            &mut state as *mut FpiImageDeviceState,
            0 as *mut libc::c_void,
        );
        if state as libc::c_uint
            == FPI_IMAGE_DEVICE_STATE_CAPTURE as libc::c_int as libc::c_uint
        {
            let mut k: size_t = 0 as libc::c_int as size_t;
            while k < 5 as libc::c_int as libc::c_ulong {
                if where_finger_is & (1 as libc::c_int) << k != 0 {
                    let mut stripe: *mut fpi_frame = g_malloc(
                        ((114 as libc::c_int * 17 as libc::c_int) as libc::c_ulong)
                            .wrapping_add(
                                ::core::mem::size_of::<fpi_frame>() as libc::c_ulong,
                            ),
                    ) as *mut fpi_frame;
                    (*stripe).delta_x = 0 as libc::c_int;
                    (*stripe).delta_y = 0 as libc::c_int;
                    stripdata = ((*stripe).data).as_mut_ptr();
                    memcpy(
                        stripdata as *mut libc::c_void,
                        ((*transfer).buffer)
                            .offset(
                                k
                                    .wrapping_mul(6498 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(
                                        (114 as libc::c_int
                                            * (57 as libc::c_int - 17 as libc::c_int)
                                            / 2 as libc::c_int) as libc::c_ulong,
                                    ) as isize,
                            ) as *const libc::c_void,
                        (114 as libc::c_int * 17 as libc::c_int) as libc::c_ulong,
                    );
                    (*self_0)
                        .strips = g_slist_prepend((*self_0).strips, stripe as gpointer);
                    (*self_0)
                        .strips_len = ((*self_0).strips_len as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as gsize
                        as gsize;
                    k = (k as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                } else {
                    end = (0 as libc::c_int == 0) as libc::c_int;
                    break;
                }
            }
        }
    } else {
        end = (0 as libc::c_int == 0) as libc::c_int;
    }
    if end != 0 {
        if (*self_0).stop == 0
            && (*self_0).strips_len > 0 as libc::c_int as libc::c_ulong
        {
            let mut img: FpImage_autoptr = 0 as FpImage_autoptr;
            (*self_0).strips = g_slist_reverse((*self_0).strips);
            fpi_do_movement_estimation(&mut assembling_ctx, (*self_0).strips);
            img = fpi_assemble_frames(&mut assembling_ctx, (*self_0).strips);
            (*img)
                .flags = ::core::mem::transmute::<
                libc::c_uint,
                FpiImageFlags,
            >(
                (*img).flags as libc::c_uint
                    | (FPI_IMAGE_COLORS_INVERTED as libc::c_int
                        | FPI_IMAGE_PARTIAL as libc::c_int) as libc::c_uint,
            );
            g_slist_free_full(
                (*self_0).strips,
                Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            );
            (*self_0).strips = 0 as *mut GSList;
            (*self_0).strips_len = 0 as libc::c_int as gsize;
            let mut resizeImage: *mut FpImage = fpi_image_resize(
                img,
                2 as libc::c_int as guint,
                2 as libc::c_int as guint,
            );
            fpi_image_device_image_captured(
                img_self,
                (if 0 as libc::c_int != 0 {
                    resizeImage as *mut libc::c_void
                } else {
                    g_steal_pointer(&mut resizeImage as *mut *mut FpImage as gpointer)
                }) as *mut FpImage,
            );
        }
        fpi_image_device_report_finger_status(img_self, 0 as libc::c_int);
    }
    fpi_ssm_next_state((*transfer).ssm);
}
unsafe extern "C" fn recv_data_resp(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(dev);
    fpi_usb_transfer_fill_bulk(
        transfer,
        0x83 as libc::c_int as guint8,
        32512 as libc::c_int as gsize,
    );
    (*transfer).ssm = ssm;
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_submit(
        transfer,
        10000 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            data_resp_cb
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
unsafe extern "C" fn cmd_resp_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut dev: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
    }
}
unsafe extern "C" fn recv_cmd_resp(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(dev);
    fpi_usb_transfer_fill_bulk(
        transfer,
        0x83 as libc::c_int as guint8,
        7 as libc::c_int as gsize,
    );
    (*transfer).ssm = ssm;
    fpi_usb_transfer_submit(
        transfer,
        10000 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            cmd_resp_cb
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
unsafe extern "C" fn send_cmd_req(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut pkt: *mut libc::c_uchar,
) {
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(dev);
    fpi_usb_transfer_fill_bulk_full(
        transfer,
        0x4 as libc::c_int as guint8,
        pkt,
        7 as libc::c_int as gsize,
        None,
    );
    (*transfer).ssm = ssm;
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_submit(
        transfer,
        10000 as libc::c_int as guint,
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
unsafe extern "C" fn ssm_run_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut self_0: *mut FpDeviceEgis0570 = FPI_DEVICE_EGIS0570(dev as gpointer);
    let mut img_dev: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            (*self_0).pkt_type = 0 as libc::c_int;
            fpi_ssm_next_state(ssm);
        }
        1 => {
            if (*self_0).stop != 0 {
                g_log(
                    b"libfprint-egis0570\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"deactivating, marking completed\0" as *const u8
                        as *const libc::c_char,
                );
                fpi_ssm_mark_completed(ssm);
                fpi_image_device_deactivate_complete(img_dev, 0 as *mut GError);
            } else {
                (*self_0).pkt_num = 0 as libc::c_int;
                fpi_ssm_next_state(ssm);
            }
        }
        2 => {
            if (*self_0).pkt_type == 0 as libc::c_int {
                send_cmd_req(
                    ssm,
                    dev,
                    (init_pkts[(*self_0).pkt_num as usize]).as_mut_ptr(),
                );
            } else {
                send_cmd_req(
                    ssm,
                    dev,
                    (repeat_pkts[(*self_0).pkt_num as usize]).as_mut_ptr(),
                );
            }
        }
        3 => {
            if is_last_pkt(dev) == 0 as libc::c_int {
                recv_cmd_resp(ssm, dev);
                (*self_0).pkt_num += 1 as libc::c_int;
                fpi_ssm_jump_to_state(ssm, SM_REQ as libc::c_int);
            } else {
                if (*self_0).pkt_type == 0 as libc::c_int {
                    (*self_0).pkt_type = 1 as libc::c_int;
                }
                fpi_ssm_next_state(ssm);
            }
        }
        4 => {
            recv_data_resp(ssm, dev);
        }
        5 => {
            fpi_ssm_jump_to_state(ssm, SM_START as libc::c_int);
        }
        _ => {
            g_assertion_message_expr(
                b"libfprint-egis0570\0" as *const u8 as *const libc::c_char,
                b"../libfprint/drivers/egis0570.c\0" as *const u8 as *const libc::c_char,
                329 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"ssm_run_state\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn loop_complete(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut img_dev: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    let mut self_0: *mut FpDeviceEgis0570 = FPI_DEVICE_EGIS0570(dev as gpointer);
    (*self_0).running = 0 as libc::c_int;
    let mut _pp: C2RustUnnamed_4 = C2RustUnnamed_4 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*self_0).background as *mut *mut guint8 as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    if !error.is_null() {
        fpi_image_device_session_error(img_dev, error);
    }
}
unsafe extern "C" fn dev_activate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpDeviceEgis0570 = FPI_DEVICE_EGIS0570(dev as gpointer);
    let mut ssm: *mut FpiSsm = fpi_ssm_new_full(
        FP_DEVICE(dev as gpointer),
        Some(ssm_run_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
        SM_STATES_NUM as libc::c_int,
        SM_STATES_NUM as libc::c_int,
        b"SM_STATES_NUM\0" as *const u8 as *const libc::c_char,
    );
    (*self_0).stop = 0 as libc::c_int;
    fpi_ssm_start(
        ssm,
        Some(
            loop_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
    (*self_0).running = (0 as libc::c_int == 0) as libc::c_int;
    fpi_image_device_activate_complete(dev, 0 as *mut GError);
}
unsafe extern "C" fn dev_init(mut dev: *mut FpImageDevice) {
    let mut error: *mut GError = 0 as *mut GError;
    g_usb_device_claim_interface(
        fpi_device_get_usb_device(FP_DEVICE(dev as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    );
    fpi_image_device_open_complete(dev, error);
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
unsafe extern "C" fn dev_deactivate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpDeviceEgis0570 = FPI_DEVICE_EGIS0570(dev as gpointer);
    if (*self_0).running != 0 {
        (*self_0).stop = (0 as libc::c_int == 0) as libc::c_int;
    } else {
        fpi_image_device_deactivate_complete(dev, 0 as *mut GError);
    };
}
static mut id_table: [FpIdEntry; 3] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x570 as libc::c_int as guint,
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
                        pid: 0x571 as libc::c_int as guint,
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
            driver_data: 0,
        };
        init
    },
];
unsafe extern "C" fn fpi_device_egis0570_init(mut self_0: *mut FpDeviceEgis0570) {}
unsafe extern "C" fn fpi_device_egis0570_class_init(
    mut klass: *mut FpDeviceEgis0570Class,
) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    let mut img_class: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_CLASS(
        klass as gpointer,
    );
    (*dev_class).id = b"egis0570\0" as *const u8 as *const libc::c_char;
    (*dev_class)
        .full_name = b"Egis Technology Inc. (aka. LighTuning) 0570\0" as *const u8
        as *const libc::c_char;
    (*dev_class).type_0 = FP_DEVICE_TYPE_USB;
    (*dev_class).id_table = id_table.as_ptr();
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
    (*img_class).img_width = 114 as libc::c_int;
    (*img_class).img_height = -(1 as libc::c_int);
    (*img_class).bz3_threshold = 25 as libc::c_int;
}
