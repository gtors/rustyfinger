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
    fn g_error_free(error: *mut GError);
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_slist_free(list: *mut GSList);
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_reverse(list: *mut GSList) -> *mut GSList;
    fn g_slist_length(list: *mut GSList) -> guint;
    fn g_get_monotonic_time() -> gint64;
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
    fn fpi_usb_transfer_new(device: *mut FpDevice) -> *mut FpiUsbTransfer;
    fn fpi_usb_transfer_fill_bulk(
        transfer: *mut FpiUsbTransfer,
        endpoint: guint8,
        length: gsize,
    );
    fn aes_write_regv(
        dev: *mut FpImageDevice,
        regs: *const aes_regwrite,
        num_regs: libc::c_uint,
        callback: aes_write_regv_cb,
        user_data: *mut libc::c_void,
    );
    fn aes_get_pixel(
        ctx: *mut fpi_frame_asmbl_ctx,
        frame: *mut fpi_frame,
        x: libc::c_uint,
        y: libc::c_uint,
    ) -> libc::c_uchar;
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
pub struct aes_regwrite {
    pub reg: libc::c_uchar,
    pub value: libc::c_uchar,
}
pub type aes_write_regv_cb = Option::<
    unsafe extern "C" fn(*mut FpImageDevice, *mut GError, *mut libc::c_void) -> (),
>;
pub type FpiDeviceAes1610 = _FpiDeviceAes1610;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceAes1610 {
    pub parent: FpImageDevice,
    pub read_regs_retry_count: guint8,
    pub strips: *mut GSList,
    pub strips_len: gsize,
    pub deactivating: gboolean,
    pub blanks_count: guint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiDeviceAes1610Class {
    pub parent_class: FpImageDeviceClass,
}
pub const ACTIVATE_NUM_STATES: activate_states = 1;
pub const WRITE_INIT: activate_states = 0;
pub const CAPTURE_NUM_STATES: capture_states = 4;
pub const CAPTURE_REQUEST_STRIP: capture_states = 2;
pub const CAPTURE_READ_STRIP: capture_states = 3;
pub const CAPTURE_READ_DATA: capture_states = 1;
pub const CAPTURE_WRITE_REQS: capture_states = 0;
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
unsafe extern "C" fn FPI_DEVICE_AES1610(mut ptr: gpointer) -> *mut FpiDeviceAes1610 {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_aes1610_get_type(),
    ) as *mut libc::c_void as *mut FpiDeviceAes1610;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_aes1610_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_aes1610_get_type_once();
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
unsafe extern "C" fn fpi_device_aes1610_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_image_device_get_type(),
        g_intern_static_string(
            b"FpiDeviceAes1610\0" as *const u8 as *const libc::c_char,
        ),
        ::core::mem::size_of::<FpiDeviceAes1610Class>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_aes1610_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpiDeviceAes1610>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpiDeviceAes1610) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_aes1610_init
                        as unsafe extern "C" fn(*mut FpiDeviceAes1610) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut fpi_device_aes1610_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut FpiDeviceAes1610_private_offset: gint = 0;
unsafe extern "C" fn fpi_device_aes1610_class_intern_init(mut klass: gpointer) {
    fpi_device_aes1610_parent_class = g_type_class_peek_parent(klass);
    if FpiDeviceAes1610_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpiDeviceAes1610_private_offset);
    }
    fpi_device_aes1610_class_init(klass as *mut FpiDeviceAes1610Class);
}
static mut assembling_ctx: fpi_frame_asmbl_ctx = unsafe {
    {
        let mut init = fpi_frame_asmbl_ctx {
            frame_width: 128 as libc::c_int as libc::c_uint,
            frame_height: 8 as libc::c_int as libc::c_uint,
            image_width: (128 as libc::c_int + 128 as libc::c_int / 2 as libc::c_int)
                as libc::c_uint,
            get_pixel: Some(
                aes_get_pixel
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
unsafe extern "C" fn stub_capture_stop_cb(
    mut dev: *mut FpImageDevice,
    mut error: *mut GError,
    mut user_data: *mut libc::c_void,
) {
    if !error.is_null() {
        g_log(
            b"libfprint-aes1610\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Error stopping capture: %s\0" as *const u8 as *const libc::c_char,
            (*error).message,
        );
        g_error_free(error);
    }
}
unsafe extern "C" fn generic_write_regv_cb(
    mut dev: *mut FpImageDevice,
    mut error: *mut GError,
    mut user_data: *mut libc::c_void,
) {
    let mut ssm: *mut FpiSsm = user_data as *mut FpiSsm;
    if error.is_null() {
        fpi_ssm_next_state(ssm);
    } else {
        fpi_ssm_mark_failed(ssm, error);
    };
}
unsafe extern "C" fn generic_read_ignore_data(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut bytes: size_t,
) {
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(dev);
    fpi_usb_transfer_fill_bulk(
        transfer,
        (1 as libc::c_int | 0x80 as libc::c_int) as guint8,
        bytes,
    );
    (*transfer).ssm = ssm;
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
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
static mut finger_det_reqs: [aes_regwrite; 22] = [
    {
        let mut init = aes_regwrite {
            reg: 0x80 as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x80 as libc::c_int as libc::c_uchar,
            value: 0x12 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x85 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x8a as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x8b as libc::c_int as libc::c_uchar,
            value: 0xe as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x8c as libc::c_int as libc::c_uchar,
            value: 0x90 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x8d as libc::c_int as libc::c_uchar,
            value: 0x83 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x8e as libc::c_int as libc::c_uchar,
            value: 0x7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x8f as libc::c_int as libc::c_uchar,
            value: 0x7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x96 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x97 as libc::c_int as libc::c_uchar,
            value: 0x48 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xa1 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xa2 as libc::c_int as libc::c_uchar,
            value: 0x50 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xa6 as libc::c_int as libc::c_uchar,
            value: 0xe4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xad as libc::c_int as libc::c_uchar,
            value: 0x8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xae as libc::c_int as libc::c_uchar,
            value: 0x5b as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xaf as libc::c_int as libc::c_uchar,
            value: 0x54 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xb1 as libc::c_int as libc::c_uchar,
            value: 0x28 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xb5 as libc::c_int as libc::c_uchar,
            value: 0xab as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xb6 as libc::c_int as libc::c_uchar,
            value: 0xe as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1b as libc::c_int as libc::c_uchar,
            value: 0x2d as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x81 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
unsafe extern "C" fn finger_det_data_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(device as gpointer);
    let mut data: *mut libc::c_uchar = (*transfer).buffer;
    let mut i: libc::c_int = 0;
    let mut sum: libc::c_int = 0 as libc::c_int;
    if !error.is_null() {
        fpi_image_device_session_error(dev, error);
        return;
    }
    i = 3 as libc::c_int;
    while i < 17 as libc::c_int {
        sum
            += (*data.offset(i as isize) as libc::c_int & 0xf as libc::c_int)
                + (*data.offset(i as isize) as libc::c_int >> 4 as libc::c_int);
        i += 1;
    }
    if sum > 20 as libc::c_int {
        adjust_gain(data, 1 as libc::c_int);
        fpi_image_device_report_finger_status(
            dev,
            (0 as libc::c_int == 0) as libc::c_int,
        );
        start_capture(dev);
    } else {
        start_finger_detection(dev);
    };
}
unsafe extern "C" fn finger_det_reqs_cb(
    mut dev: *mut FpImageDevice,
    mut error: *mut GError,
    mut user_data: *mut libc::c_void,
) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    if !error.is_null() {
        fpi_image_device_session_error(dev, error);
        return;
    }
    transfer = fpi_usb_transfer_new(FP_DEVICE(dev as gpointer));
    fpi_usb_transfer_fill_bulk(
        transfer,
        (1 as libc::c_int | 0x80 as libc::c_int) as guint8,
        19 as libc::c_int as gsize,
    );
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_submit(
        transfer,
        4000 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            finger_det_data_cb
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
unsafe extern "C" fn start_finger_detection(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceAes1610 = FPI_DEVICE_AES1610(dev as gpointer);
    if (*self_0).deactivating != 0 {
        complete_deactivation(dev);
        return;
    }
    aes_write_regv(
        dev,
        finger_det_reqs.as_ptr(),
        (::core::mem::size_of::<[aes_regwrite; 22]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<aes_regwrite>() as libc::c_ulong)
            as libc::c_uint,
        Some(
            finger_det_reqs_cb
                as unsafe extern "C" fn(
                    *mut FpImageDevice,
                    *mut GError,
                    *mut libc::c_void,
                ) -> (),
        ),
        0 as *mut libc::c_void,
    );
}
static mut capture_reqs: [aes_regwrite; 123] = [
    {
        let mut init = aes_regwrite {
            reg: 0x80 as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x80 as libc::c_int as libc::c_uchar,
            value: 0x12 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x84 as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x85 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x89 as libc::c_int as libc::c_uchar,
            value: 0x64 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x8a as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x8b as libc::c_int as libc::c_uchar,
            value: 0xe as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x8c as libc::c_int as libc::c_uchar,
            value: 0x90 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xbe as libc::c_int as libc::c_uchar,
            value: 0x23 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x29 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x2a as libc::c_int as libc::c_uchar,
            value: 0xff as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x96 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x98 as libc::c_int as libc::c_uchar,
            value: 0x3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x99 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x9c as libc::c_int as libc::c_uchar,
            value: 0xa5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x9d as libc::c_int as libc::c_uchar,
            value: 0x40 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x9e as libc::c_int as libc::c_uchar,
            value: 0xc6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x9f as libc::c_int as libc::c_uchar,
            value: 0x8e as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xa2 as libc::c_int as libc::c_uchar,
            value: 0x50 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xa3 as libc::c_int as libc::c_uchar,
            value: 0xf0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xad as libc::c_int as libc::c_uchar,
            value: 0x8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xbd as libc::c_int as libc::c_uchar,
            value: 0x4f as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xaf as libc::c_int as libc::c_uchar,
            value: 0x54 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xb1 as libc::c_int as libc::c_uchar,
            value: 0x8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xb5 as libc::c_int as libc::c_uchar,
            value: 0xab as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1b as libc::c_int as libc::c_uchar,
            value: 0x2d as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xb6 as libc::c_int as libc::c_uchar,
            value: 0x4e as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xb8 as libc::c_int as libc::c_uchar,
            value: 0x70 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x2b as libc::c_int as libc::c_uchar,
            value: 0xb3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x2c as libc::c_int as libc::c_uchar,
            value: 0x5d as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x2d as libc::c_int as libc::c_uchar,
            value: 0x98 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x2e as libc::c_int as libc::c_uchar,
            value: 0xb0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x2f as libc::c_int as libc::c_uchar,
            value: 0x20 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xa2 as libc::c_int as libc::c_uchar,
            value: 0xd0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x21 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0xbe as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x30 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x29 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x9e as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x30 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0xbb as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x9d as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x22 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0xff as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x1b as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x4e as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x16 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x28 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x22 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0xff as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x15 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0xf1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x9 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x30 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0xd5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0xa as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x9e as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0xb as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x17 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x9d as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0xc as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x28 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0xd7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0xd as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x17 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0xd7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0xe as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0xa as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0xcb as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0xf as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x24 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x14 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x10 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x17 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x85 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x11 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x15 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x71 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x12 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x2b as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x36 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x13 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x12 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x14 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x30 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x97 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x15 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x21 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x32 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x16 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0xe6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x17 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x16 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x18 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x30 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x19 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x21 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x37 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x1a as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0x8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1c as libc::c_int as libc::c_uchar,
            value: 0x1b as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1d as libc::c_int as libc::c_uchar,
            value: 0x80 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x1e as libc::c_int as libc::c_uchar,
            value: 0xd5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xa2 as libc::c_int as libc::c_uchar,
            value: 0x50 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xa2 as libc::c_int as libc::c_uchar,
            value: 0x50 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x81 as libc::c_int as libc::c_uchar,
            value: 0x1 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut strip_scan_reqs: [aes_regwrite; 5] = [
    {
        let mut init = aes_regwrite {
            reg: 0xbe as libc::c_int as libc::c_uchar,
            value: 0x23 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x29 as libc::c_int as libc::c_uchar,
            value: 0x4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0x2a as libc::c_int as libc::c_uchar,
            value: 0xff as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xbd as libc::c_int as libc::c_uchar,
            value: 0x4f as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = aes_regwrite {
            reg: 0xff as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut capture_stop: [aes_regwrite; 1] = [
    {
        let mut init = aes_regwrite {
            reg: 0x81 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut list_BE_values: [libc::c_uchar; 10] = [
    0x23 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0,
    0,
];
static mut list_BD_values: [libc::c_uchar; 10] = [
    0x28 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0,
    0,
    0,
];
unsafe extern "C" fn adjust_gain(
    mut buffer: *mut libc::c_uchar,
    mut status: libc::c_int,
) -> libc::c_int {
    static mut pos_list_BE: libc::c_int = 0 as libc::c_int;
    static mut pos_list_BD: libc::c_int = 0 as libc::c_int;
    if status == 1 as libc::c_int {
        if *buffer.offset(1 as libc::c_int as isize) as libc::c_int > 0x78 as libc::c_int
        {
            strip_scan_reqs[0 as libc::c_int as usize]
                .value = 0x6b as libc::c_int as libc::c_uchar;
            strip_scan_reqs[1 as libc::c_int as usize]
                .value = 0x6 as libc::c_int as libc::c_uchar;
            strip_scan_reqs[2 as libc::c_int as usize]
                .value = 0x35 as libc::c_int as libc::c_uchar;
            strip_scan_reqs[3 as libc::c_int as usize]
                .value = 0x4b as libc::c_int as libc::c_uchar;
        } else if *buffer.offset(1 as libc::c_int as isize) as libc::c_int
            > 0x55 as libc::c_int
        {
            strip_scan_reqs[0 as libc::c_int as usize]
                .value = 0x63 as libc::c_int as libc::c_uchar;
            strip_scan_reqs[1 as libc::c_int as usize]
                .value = 0x15 as libc::c_int as libc::c_uchar;
            strip_scan_reqs[2 as libc::c_int as usize]
                .value = 0x35 as libc::c_int as libc::c_uchar;
            strip_scan_reqs[3 as libc::c_int as usize]
                .value = 0x3b as libc::c_int as libc::c_uchar;
        } else if *buffer.offset(1 as libc::c_int as isize) as libc::c_int
            > 0x40 as libc::c_int
            || *buffer.offset(16 as libc::c_int as isize) as libc::c_int
                > 0x19 as libc::c_int
        {
            strip_scan_reqs[0 as libc::c_int as usize]
                .value = 0x43 as libc::c_int as libc::c_uchar;
            strip_scan_reqs[1 as libc::c_int as usize]
                .value = 0x13 as libc::c_int as libc::c_uchar;
            strip_scan_reqs[2 as libc::c_int as usize]
                .value = 0x35 as libc::c_int as libc::c_uchar;
            strip_scan_reqs[3 as libc::c_int as usize]
                .value = 0x30 as libc::c_int as libc::c_uchar;
        } else {
            strip_scan_reqs[0 as libc::c_int as usize]
                .value = 0x23 as libc::c_int as libc::c_uchar;
            strip_scan_reqs[1 as libc::c_int as usize]
                .value = 0x7 as libc::c_int as libc::c_uchar;
            strip_scan_reqs[2 as libc::c_int as usize]
                .value = 0x35 as libc::c_int as libc::c_uchar;
            strip_scan_reqs[3 as libc::c_int as usize]
                .value = 0x28 as libc::c_int as libc::c_uchar;
        }
        capture_reqs[8 as libc::c_int as usize]
            .value = strip_scan_reqs[0 as libc::c_int as usize].value;
        capture_reqs[9 as libc::c_int as usize]
            .value = strip_scan_reqs[1 as libc::c_int as usize].value;
        capture_reqs[10 as libc::c_int as usize]
            .value = strip_scan_reqs[2 as libc::c_int as usize].value;
        capture_reqs[21 as libc::c_int as usize]
            .value = strip_scan_reqs[3 as libc::c_int as usize].value;
        g_log(
            b"libfprint-aes1610\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"first gain: %x %x %x %x %x %x %x %x\0" as *const u8 as *const libc::c_char,
            strip_scan_reqs[0 as libc::c_int as usize].reg as libc::c_int,
            strip_scan_reqs[0 as libc::c_int as usize].value as libc::c_int,
            strip_scan_reqs[1 as libc::c_int as usize].reg as libc::c_int,
            strip_scan_reqs[1 as libc::c_int as usize].value as libc::c_int,
            strip_scan_reqs[2 as libc::c_int as usize].reg as libc::c_int,
            strip_scan_reqs[2 as libc::c_int as usize].value as libc::c_int,
            strip_scan_reqs[3 as libc::c_int as usize].reg as libc::c_int,
            strip_scan_reqs[3 as libc::c_int as usize].value as libc::c_int,
        );
    } else if status == 2 as libc::c_int {
        if *buffer.offset(514 as libc::c_int as isize) as libc::c_int
            > 0x78 as libc::c_int
        {
            if pos_list_BE < 7 as libc::c_int {
                pos_list_BE += 1;
            }
            if pos_list_BD < 6 as libc::c_int {
                pos_list_BD += 1;
            }
            strip_scan_reqs[1 as libc::c_int as usize]
                .value = 0x4 as libc::c_int as libc::c_uchar;
            strip_scan_reqs[2 as libc::c_int as usize]
                .value = 0x35 as libc::c_int as libc::c_uchar;
        } else if *buffer.offset(514 as libc::c_int as isize) as libc::c_int
            > 0x55 as libc::c_int
        {
            if pos_list_BE < 2 as libc::c_int {
                pos_list_BE += 1;
            } else if pos_list_BE > 2 as libc::c_int {
                pos_list_BE -= 1;
            }
            if pos_list_BD < 2 as libc::c_int {
                pos_list_BD += 1;
            } else if pos_list_BD > 2 as libc::c_int {
                pos_list_BD -= 1;
            }
            strip_scan_reqs[1 as libc::c_int as usize]
                .value = 0x15 as libc::c_int as libc::c_uchar;
            strip_scan_reqs[2 as libc::c_int as usize]
                .value = 0x35 as libc::c_int as libc::c_uchar;
        } else if *buffer.offset(514 as libc::c_int as isize) as libc::c_int
            > 0x40 as libc::c_int
            || *buffer.offset(529 as libc::c_int as isize) as libc::c_int
                > 0x19 as libc::c_int
        {
            if pos_list_BE < 1 as libc::c_int {
                pos_list_BE += 1;
            } else if pos_list_BE > 1 as libc::c_int {
                pos_list_BE -= 1;
            }
            if pos_list_BD < 1 as libc::c_int {
                pos_list_BD += 1;
            } else if pos_list_BD > 1 as libc::c_int {
                pos_list_BD -= 1;
            }
            strip_scan_reqs[1 as libc::c_int as usize]
                .value = 0x13 as libc::c_int as libc::c_uchar;
            strip_scan_reqs[2 as libc::c_int as usize]
                .value = 0x35 as libc::c_int as libc::c_uchar;
        } else {
            if pos_list_BE > 0 as libc::c_int {
                pos_list_BE -= 1;
            }
            if pos_list_BD > 0 as libc::c_int {
                pos_list_BD -= 1;
            }
            strip_scan_reqs[1 as libc::c_int as usize]
                .value = 0x7 as libc::c_int as libc::c_uchar;
            strip_scan_reqs[2 as libc::c_int as usize]
                .value = 0x35 as libc::c_int as libc::c_uchar;
        }
        strip_scan_reqs[0 as libc::c_int as usize]
            .value = list_BE_values[pos_list_BE as usize];
        strip_scan_reqs[3 as libc::c_int as usize]
            .value = list_BD_values[pos_list_BD as usize];
        g_log(
            b"libfprint-aes1610\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"gain: %x %x %x %x %x %x %x %x\0" as *const u8 as *const libc::c_char,
            strip_scan_reqs[0 as libc::c_int as usize].reg as libc::c_int,
            strip_scan_reqs[0 as libc::c_int as usize].value as libc::c_int,
            strip_scan_reqs[1 as libc::c_int as usize].reg as libc::c_int,
            strip_scan_reqs[1 as libc::c_int as usize].value as libc::c_int,
            strip_scan_reqs[2 as libc::c_int as usize].reg as libc::c_int,
            strip_scan_reqs[2 as libc::c_int as usize].value as libc::c_int,
            strip_scan_reqs[3 as libc::c_int as usize].reg as libc::c_int,
            strip_scan_reqs[3 as libc::c_int as usize].value as libc::c_int,
        );
    } else {
        g_log(
            b"libfprint-aes1610\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"Unexpected gain status.\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn restore_gain() {
    strip_scan_reqs[0 as libc::c_int as usize]
        .value = list_BE_values[0 as libc::c_int as usize];
    strip_scan_reqs[1 as libc::c_int as usize]
        .value = 0x4 as libc::c_int as libc::c_uchar;
    strip_scan_reqs[2 as libc::c_int as usize]
        .value = 0xff as libc::c_int as libc::c_uchar;
    strip_scan_reqs[3 as libc::c_int as usize]
        .value = list_BD_values[0 as libc::c_int as usize];
    capture_reqs[8 as libc::c_int as usize]
        .value = list_BE_values[0 as libc::c_int as usize];
    capture_reqs[9 as libc::c_int as usize].value = 0x4 as libc::c_int as libc::c_uchar;
    capture_reqs[10 as libc::c_int as usize]
        .value = 0xff as libc::c_int as libc::c_uchar;
    capture_reqs[21 as libc::c_int as usize]
        .value = list_BD_values[0 as libc::c_int as usize];
}
unsafe extern "C" fn capture_read_strip_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut stripdata: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(device as gpointer);
    let mut self_0: *mut FpiDeviceAes1610 = FPI_DEVICE_AES1610(dev as gpointer);
    let mut data: *mut libc::c_uchar = (*transfer).buffer;
    let mut sum: gint = 0;
    let mut i: gint = 0;
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    sum = 0 as libc::c_int;
    i = 516 as libc::c_int;
    while i < 530 as libc::c_int {
        sum += *data.offset(i as isize) as libc::c_int;
        i += 1;
    }
    g_log(
        b"libfprint-aes1610\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"sum=%d\0" as *const u8 as *const libc::c_char,
        sum,
    );
    if sum > 0 as libc::c_int {
        let mut stripe: *mut fpi_frame = g_malloc(
            ((128 as libc::c_int * (8 as libc::c_int / 2 as libc::c_int))
                as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<fpi_frame>() as libc::c_ulong),
        ) as *mut fpi_frame;
        (*stripe).delta_x = 0 as libc::c_int;
        (*stripe).delta_y = 0 as libc::c_int;
        stripdata = ((*stripe).data).as_mut_ptr();
        memcpy(
            stripdata as *mut libc::c_void,
            data.offset(1 as libc::c_int as isize) as *const libc::c_void,
            (128 as libc::c_int * (8 as libc::c_int / 2 as libc::c_int)) as libc::c_ulong,
        );
        (*self_0).strips = g_slist_prepend((*self_0).strips, stripe as gpointer);
        (*self_0).strips_len = ((*self_0).strips_len).wrapping_add(1);
        (*self_0).blanks_count = 0 as libc::c_int as guint8;
    } else {
        (*self_0).blanks_count = ((*self_0).blanks_count).wrapping_add(1);
        g_log(
            b"libfprint-aes1610\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"got blank frame\0" as *const u8 as *const libc::c_char,
        );
    }
    adjust_gain(data, 2 as libc::c_int);
    if (*self_0).blanks_count as libc::c_int > 10 as libc::c_int
        || g_slist_length((*self_0).strips) >= 350 as libc::c_int as libc::c_uint
    {
        let mut img: *mut FpImage = 0 as *mut FpImage;
        g_log(
            b"libfprint-aes1610\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"sending stop capture.... blanks=%d  frames=%d\0" as *const u8
                as *const libc::c_char,
            (*self_0).blanks_count as libc::c_int,
            g_slist_length((*self_0).strips),
        );
        aes_write_regv(
            dev,
            capture_stop.as_ptr(),
            (::core::mem::size_of::<[aes_regwrite; 1]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<aes_regwrite>() as libc::c_ulong)
                as libc::c_uint,
            Some(
                stub_capture_stop_cb
                    as unsafe extern "C" fn(
                        *mut FpImageDevice,
                        *mut GError,
                        *mut libc::c_void,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
        (*self_0).strips = g_slist_reverse((*self_0).strips);
        fpi_do_movement_estimation(&mut assembling_ctx, (*self_0).strips);
        img = fpi_assemble_frames(&mut assembling_ctx, (*self_0).strips);
        (*img)
            .flags = ::core::mem::transmute::<
            libc::c_uint,
            FpiImageFlags,
        >(
            (*img).flags as libc::c_uint
                | FPI_IMAGE_PARTIAL as libc::c_int as libc::c_uint,
        );
        g_slist_free_full(
            (*self_0).strips,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
        (*self_0).strips = 0 as *mut GSList;
        (*self_0).strips_len = 0 as libc::c_int as gsize;
        (*self_0).blanks_count = 0 as libc::c_int as guint8;
        fpi_image_device_image_captured(dev, img);
        fpi_image_device_report_finger_status(dev, 0 as libc::c_int);
        fpi_ssm_mark_completed((*transfer).ssm);
        restore_gain();
    } else {
        fpi_ssm_jump_to_state((*transfer).ssm, CAPTURE_REQUEST_STRIP as libc::c_int);
    };
}
unsafe extern "C" fn capture_run_state(mut ssm: *mut FpiSsm, mut _dev: *mut FpDevice) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceAes1610 = FPI_DEVICE_AES1610(_dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            g_log(
                b"libfprint-aes1610\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"write reqs\0" as *const u8 as *const libc::c_char,
            );
            aes_write_regv(
                dev,
                capture_reqs.as_mut_ptr(),
                (::core::mem::size_of::<[aes_regwrite; 123]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<aes_regwrite>() as libc::c_ulong,
                    ) as libc::c_uint,
                Some(
                    generic_write_regv_cb
                        as unsafe extern "C" fn(
                            *mut FpImageDevice,
                            *mut GError,
                            *mut libc::c_void,
                        ) -> (),
                ),
                ssm as *mut libc::c_void,
            );
        }
        1 => {
            g_log(
                b"libfprint-aes1610\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"read data\0" as *const u8 as *const libc::c_char,
            );
            generic_read_ignore_data(ssm, _dev, 665 as libc::c_int as size_t);
        }
        2 => {
            g_log(
                b"libfprint-aes1610\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"request strip\0" as *const u8 as *const libc::c_char,
            );
            if (*self_0).deactivating != 0 {
                fpi_ssm_mark_completed(ssm);
            } else {
                aes_write_regv(
                    dev,
                    strip_scan_reqs.as_mut_ptr(),
                    (::core::mem::size_of::<[aes_regwrite; 5]>() as libc::c_ulong)
                        .wrapping_div(
                            ::core::mem::size_of::<aes_regwrite>() as libc::c_ulong,
                        ) as libc::c_uint,
                    Some(
                        generic_write_regv_cb
                            as unsafe extern "C" fn(
                                *mut FpImageDevice,
                                *mut GError,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                    ssm as *mut libc::c_void,
                );
            }
        }
        3 => {
            let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(_dev);
            fpi_usb_transfer_fill_bulk(
                transfer,
                (1 as libc::c_int | 0x80 as libc::c_int) as guint8,
                665 as libc::c_int as gsize,
            );
            (*transfer).ssm = ssm;
            (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
            fpi_usb_transfer_submit(
                transfer,
                4000 as libc::c_int as guint,
                0 as *mut GCancellable,
                Some(
                    capture_read_strip_cb
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
unsafe extern "C" fn capture_sm_complete(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceAes1610 = FPI_DEVICE_AES1610(_dev as gpointer);
    g_log_structured(
        b"libfprint-aes1610\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/aes1610.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"682\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"capture_sm_complete\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/aes1610.c:682\0" as *const u8 as *const libc::c_char,
    );
    if (*self_0).deactivating != 0 {
        complete_deactivation(dev);
        if !error.is_null() {
            g_error_free(error);
        }
    } else if !error.is_null() {
        fpi_image_device_session_error(dev, error);
    } else {
        start_finger_detection(dev);
    };
}
unsafe extern "C" fn start_capture(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceAes1610 = FPI_DEVICE_AES1610(dev as gpointer);
    let mut ssm: *mut FpiSsm = 0 as *mut FpiSsm;
    if (*self_0).deactivating != 0 {
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
        b"libfprint-aes1610\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/aes1610.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"713\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"start_capture\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/aes1610.c:713\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            capture_sm_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
static mut init: [aes_regwrite; 1] = [
    {
        let mut init = aes_regwrite {
            reg: 0x82 as libc::c_int as libc::c_uchar,
            value: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
unsafe extern "C" fn activate_run_state(mut ssm: *mut FpiSsm, mut _dev: *mut FpDevice) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            g_log(
                b"libfprint-aes1610\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"write init\0" as *const u8 as *const libc::c_char,
            );
            aes_write_regv(
                dev,
                init.as_ptr(),
                (::core::mem::size_of::<[aes_regwrite; 1]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<aes_regwrite>() as libc::c_ulong,
                    ) as libc::c_uint,
                Some(
                    generic_write_regv_cb
                        as unsafe extern "C" fn(
                            *mut FpImageDevice,
                            *mut GError,
                            *mut libc::c_void,
                        ) -> (),
                ),
                ssm as *mut libc::c_void,
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
        start_finger_detection(dev);
    }
}
unsafe extern "C" fn dev_activate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceAes1610 = FPI_DEVICE_AES1610(dev as gpointer);
    let mut ssm: *mut FpiSsm = fpi_ssm_new_full(
        FP_DEVICE(dev as gpointer),
        Some(
            activate_run_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        ACTIVATE_NUM_STATES as libc::c_int,
        ACTIVATE_NUM_STATES as libc::c_int,
        b"ACTIVATE_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    (*self_0).read_regs_retry_count = 0 as libc::c_int as guint8;
    fpi_ssm_start(
        ssm,
        Some(
            activate_sm_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn dev_deactivate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceAes1610 = FPI_DEVICE_AES1610(dev as gpointer);
    (*self_0).deactivating = (0 as libc::c_int == 0) as libc::c_int;
}
unsafe extern "C" fn complete_deactivation(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceAes1610 = FPI_DEVICE_AES1610(dev as gpointer);
    g_log_structured(
        b"libfprint-aes1610\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/aes1610.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"788\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"complete_deactivation\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/aes1610.c:788\0" as *const u8 as *const libc::c_char,
    );
    (*self_0).deactivating = 0 as libc::c_int;
    g_slist_free((*self_0).strips);
    (*self_0).strips = 0 as *mut GSList;
    (*self_0).strips_len = 0 as libc::c_int as gsize;
    (*self_0).blanks_count = 0 as libc::c_int as guint8;
    fpi_image_device_deactivate_complete(dev, 0 as *mut GError);
}
unsafe extern "C" fn dev_init(mut dev: *mut FpImageDevice) {
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
    fpi_image_device_open_complete(dev, 0 as *mut GError);
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
static mut id_table: [FpIdEntry; 2] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x1600 as libc::c_int as guint,
                        vid: 0x8ff as libc::c_int as guint,
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
unsafe extern "C" fn fpi_device_aes1610_init(mut self_0: *mut FpiDeviceAes1610) {}
unsafe extern "C" fn fpi_device_aes1610_class_init(
    mut klass: *mut FpiDeviceAes1610Class,
) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    let mut img_class: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_CLASS(
        klass as gpointer,
    );
    (*dev_class).id = b"aes1610\0" as *const u8 as *const libc::c_char;
    (*dev_class).full_name = b"AuthenTec AES1610\0" as *const u8 as *const libc::c_char;
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
    (*img_class).bz3_threshold = 20 as libc::c_int;
    (*img_class).img_width = 128 as libc::c_int + 128 as libc::c_int / 2 as libc::c_int;
    (*img_class).img_height = -(1 as libc::c_int);
}
