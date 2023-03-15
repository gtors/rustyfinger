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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn g_ptr_array_new_with_free_func(
        element_free_func: GDestroyNotify,
    ) -> *mut GPtrArray;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_ptr_array_find_with_equal_func(
        haystack: *mut GPtrArray,
        needle: gconstpointer,
        equal_func: GEqualFunc,
        index_: *mut guint,
    ) -> gboolean;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_get_monotonic_time() -> gint64;
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
    fn g_variant_new(format_string: *const gchar, _: ...) -> *mut GVariant;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, _: ...);
    fn g_variant_check_format_string(
        value: *mut GVariant,
        format_string: *const gchar,
        copy_only: gboolean,
    ) -> gboolean;
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn g_log_structured(log_domain: *const gchar, log_level: GLogLevelFlags, _: ...);
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn fpi_ssm_usb_transfer_cb(
        transfer: *mut FpiUsbTransfer,
        device: *mut FpDevice,
        unused_data: gpointer,
        error: *mut GError,
    );
    fn fpi_ssm_get_cur_state(machine: *mut FpiSsm) -> libc::c_int;
    fn fpi_ssm_get_data(machine: *mut FpiSsm) -> gpointer;
    fn fpi_ssm_set_data(
        machine: *mut FpiSsm,
        ssm_data: gpointer,
        ssm_data_destroy: GDestroyNotify,
    );
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
    fn fpi_usb_transfer_fill_bulk_full(
        transfer: *mut FpiUsbTransfer,
        endpoint: guint8,
        buffer: *mut guint8,
        length: gsize,
        free_func: GDestroyNotify,
    );
    fn fpi_usb_transfer_fill_bulk(
        transfer: *mut FpiUsbTransfer,
        endpoint: guint8,
        length: gsize,
    );
    fn fpi_usb_transfer_new(device: *mut FpDevice) -> *mut FpiUsbTransfer;
    fn fpi_device_identify_report(
        device: *mut FpDevice,
        match_0: *mut FpPrint,
        print: *mut FpPrint,
        error: *mut GError,
    );
    fn fpi_device_verify_report(
        device: *mut FpDevice,
        result: FpiMatchResult,
        print: *mut FpPrint,
        error: *mut GError,
    );
    fn fpi_device_enroll_progress(
        device: *mut FpDevice,
        completed_stages: gint,
        print: *mut FpPrint,
        error: *mut GError,
    );
    fn fpi_device_list_complete(
        device: *mut FpDevice,
        prints: *mut GPtrArray,
        error: *mut GError,
    );
    fn fpi_device_delete_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_identify_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_verify_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_enroll_complete(
        device: *mut FpDevice,
        print: *mut FpPrint,
        error: *mut GError,
    );
    fn fpi_device_close_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_open_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_action_error(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_set_nr_enroll_stages(device: *mut FpDevice, enroll_stages: gint);
    fn fpi_device_get_cancellable(device: *mut FpDevice) -> *mut GCancellable;
    fn fpi_device_get_delete_data(device: *mut FpDevice, print: *mut *mut FpPrint);
    fn fpi_device_get_identify_data(device: *mut FpDevice, prints: *mut *mut GPtrArray);
    fn fpi_device_get_verify_data(device: *mut FpDevice, print: *mut *mut FpPrint);
    fn fpi_device_get_enroll_data(device: *mut FpDevice, print: *mut *mut FpPrint);
    fn fpi_device_error_new_msg(
        error: FpDeviceError,
        msg: *const gchar,
        _: ...
    ) -> *mut GError;
    fn fpi_device_error_new(error: FpDeviceError) -> *mut GError;
    fn fpi_device_retry_new(error: FpDeviceRetry) -> *mut GError;
    fn fpi_device_get_current_action(device: *mut FpDevice) -> FpiDeviceAction;
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
    fn g_object_ref_sink(object: gpointer) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_usb_device_get_pid(self_0: *mut GUsbDevice) -> guint16;
    fn g_usb_device_reset(self_0: *mut GUsbDevice, error: *mut *mut GError) -> gboolean;
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
    fn fp_print_new(device: *mut FpDevice) -> *mut FpPrint;
    fn fp_print_get_description(print: *mut FpPrint) -> *const gchar;
    fn fp_print_equal(self_0: *mut FpPrint, other: *mut FpPrint) -> gboolean;
    fn fpi_print_set_type(print: *mut FpPrint, type_0: FpiPrintType);
    fn fpi_print_set_device_stored(print: *mut FpPrint, device_stored: gboolean);
    fn fpi_print_generate_user_id(print: *mut FpPrint) -> *mut gchar;
    fn fpi_print_fill_from_user_id(
        print: *mut FpPrint,
        user_id: *const libc::c_char,
    ) -> gboolean;
    fn fpi_device_class_auto_initialize_features(device_class: *mut FpDeviceClass);
    fn fpi_device_get_usb_device(device: *mut FpDevice) -> *mut GUsbDevice;
}
pub type guint8 = libc::c_uchar;
pub type guint16 = libc::c_ushort;
pub type gint32 = libc::c_int;
pub type guint32 = libc::c_uint;
pub type gint64 = libc::c_long;
pub type guint64 = libc::c_ulong;
pub type gssize = libc::c_long;
pub type gsize = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
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
pub type GEqualFunc = Option::<
    unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
>;
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
pub type FpiDeviceAction = libc::c_uint;
pub const FPI_DEVICE_ACTION_CLEAR_STORAGE: FpiDeviceAction = 10;
pub const FPI_DEVICE_ACTION_DELETE: FpiDeviceAction = 9;
pub const FPI_DEVICE_ACTION_LIST: FpiDeviceAction = 8;
pub const FPI_DEVICE_ACTION_CAPTURE: FpiDeviceAction = 7;
pub const FPI_DEVICE_ACTION_IDENTIFY: FpiDeviceAction = 6;
pub const FPI_DEVICE_ACTION_VERIFY: FpiDeviceAction = 5;
pub const FPI_DEVICE_ACTION_ENROLL: FpiDeviceAction = 4;
pub const FPI_DEVICE_ACTION_CLOSE: FpiDeviceAction = 3;
pub const FPI_DEVICE_ACTION_OPEN: FpiDeviceAction = 2;
pub const FPI_DEVICE_ACTION_PROBE: FpiDeviceAction = 1;
pub const FPI_DEVICE_ACTION_NONE: FpiDeviceAction = 0;
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
pub type FpiUsbTransfer_autoptr = *mut FpiUsbTransfer;
pub type FpiSsmCompletedCallback = Option::<
    unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
>;
pub type FpiSsmHandlerCallback = Option::<
    unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
>;
pub type uint8_t = __uint8_t;
pub type libusb_endpoint_direction = libc::c_uint;
pub const LIBUSB_ENDPOINT_IN: libusb_endpoint_direction = 128;
pub const LIBUSB_ENDPOINT_OUT: libusb_endpoint_direction = 0;
pub type FpiDeviceElanmoc = _FpiDeviceElanmoc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceElanmoc {
    pub parent: FpDevice,
    pub task_ssm: *mut FpiSsm,
    pub cmd_ssm: *mut FpiSsm,
    pub cmd_transfer: *mut FpiUsbTransfer,
    pub cmd_cancelable: gboolean,
    pub cmd_len_in: gsize,
    pub fw_ver: libc::c_ushort,
    pub x_trace: libc::c_uchar,
    pub y_trace: libc::c_uchar,
    pub num_frames: libc::c_int,
    pub curr_enrolled: libc::c_int,
    pub max_moc_enroll_time: libc::c_int,
    pub cancel_result: libc::c_int,
    pub cmd_retry_cnt: libc::c_int,
    pub list_index: libc::c_int,
    pub list_result: *mut GPtrArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiDeviceElanmocClass {
    pub parent_class: FpDeviceClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub const MOC_LIST_NUM_STATES: moc_list_states = 2;
pub const MOC_LIST_GET_FINGER: moc_list_states = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elanmoc_cmd {
    pub cmd_header: [libc::c_uchar; 3],
    pub cmd_len: libc::c_int,
    pub resp_len: libc::c_int,
}
pub type SynCmdMsgCallback = Option::<
    unsafe extern "C" fn(*mut FpiDeviceElanmoc, *mut uint8_t, gsize, *mut GError) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandData {
    pub callback: SynCmdMsgCallback,
}
pub const FP_CMD_NUM_STATES: C2RustUnnamed_5 = 2;
pub const FP_CMD_GET: C2RustUnnamed_5 = 1;
pub const FP_CMD_SEND: C2RustUnnamed_5 = 0;
pub const MOC_LIST_GET_ENROLLED: moc_list_states = 0;
pub const DELETE_NUM_STATES: delete_states = 1;
pub const DELETE_SEND_CMD: delete_states = 0;
pub const IDENTIFY_NUM_STATES: identify_states = 2;
pub const RSP_VERIFY_FAIL: verify_status = 0;
pub const RSP_VERIFY_OK: verify_status = 1;
pub const IDENTIFY_WAIT_FINGER: identify_states = 1;
pub const IDENTIFY_SET_MODE: identify_states = 0;
pub const MOC_ENROLL_NUM_STATES: moc_enroll_states = 4;
pub const ENROLL_RSP_ENROLL_OK: enroll_states = 2;
pub const ENROLL_RSP_ENROLL_CANCEL_REPORT: enroll_states = 3;
pub const ENROLL_RSP_ENROLL_REPORT: enroll_states = 1;
pub const ENROLL_RSP_RETRY: enroll_states = 0;
pub const MOC_ENROLL_COMMIT_RESULT: moc_enroll_states = 3;
pub const MOC_ENROLL_WAIT_FINGER: moc_enroll_states = 2;
pub const MOC_ENROLL_REENROLL_CHECK: moc_enroll_states = 1;
pub const MOC_ENROLL_GET_ENROLLED_NUM: moc_enroll_states = 0;
pub const DEV_EXIT_STATES: dev_exit_states = 1;
pub const DEV_EXIT_ABOVE: dev_exit_states = 0;
pub const DEV_INIT_STATES: dev_init_states = 5;
pub const DEV_GET_ENROLLED: dev_init_states = 4;
pub const DEV_GET_DIM: dev_init_states = 3;
pub const DEV_GET_VER: dev_init_states = 2;
pub const DEV_SET_MODE: dev_init_states = 1;
pub const DEV_WAIT_READY: dev_init_states = 0;
pub type moc_enroll_states = libc::c_uint;
pub type moc_list_states = libc::c_uint;
pub type delete_states = libc::c_uint;
pub type dev_init_states = libc::c_uint;
pub type dev_exit_states = libc::c_uint;
pub type C2RustUnnamed_5 = libc::c_uint;
pub type enroll_states = libc::c_uint;
pub const ENROLL_NUM_STATES: enroll_states = 4;
pub type verify_status = libc::c_uint;
pub const RSP_VERIFY_STATES: verify_status = 2;
pub type identify_states = libc::c_uint;
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
unsafe extern "C" fn FPI_DEVICE_ELANMOC(mut ptr: gpointer) -> *mut FpiDeviceElanmoc {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_elanmoc_get_type(),
    ) as *mut libc::c_void as *mut FpiDeviceElanmoc;
}
static mut elanmoc_get_userid_cmd: elanmoc_cmd = {
    let mut init = elanmoc_cmd {
        cmd_header: [
            0x43 as libc::c_int as libc::c_uchar,
            0x21 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
        ],
        cmd_len: 3 as libc::c_int,
        resp_len: 97 as libc::c_int,
    };
    init
};
static mut cal_status_cmd: elanmoc_cmd = {
    let mut init = elanmoc_cmd {
        cmd_header: [
            0x40 as libc::c_int as libc::c_uchar,
            0xff as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
        ],
        cmd_len: 3 as libc::c_int,
        resp_len: 2 as libc::c_int,
    };
    init
};
static mut fw_ver_cmd: elanmoc_cmd = {
    let mut init = elanmoc_cmd {
        cmd_header: [
            0x40 as libc::c_int as libc::c_uchar,
            0x19 as libc::c_int as libc::c_uchar,
            0,
        ],
        cmd_len: 2 as libc::c_int,
        resp_len: 2 as libc::c_int,
    };
    init
};
static mut sensor_dim_cmd: elanmoc_cmd = {
    let mut init = elanmoc_cmd {
        cmd_header: [
            0 as libc::c_int as libc::c_uchar,
            0xc as libc::c_int as libc::c_uchar,
            0,
        ],
        cmd_len: 2 as libc::c_int,
        resp_len: 4 as libc::c_int,
    };
    init
};
static mut elanmoc_above_cmd: elanmoc_cmd = {
    let mut init = elanmoc_cmd {
        cmd_header: [
            0x40 as libc::c_int as libc::c_uchar,
            0xff as libc::c_int as libc::c_uchar,
            0x2 as libc::c_int as libc::c_uchar,
        ],
        cmd_len: 3 as libc::c_int,
        resp_len: 0 as libc::c_int,
    };
    init
};
static mut elanmoc_check_reenroll_cmd: elanmoc_cmd = {
    let mut init = elanmoc_cmd {
        cmd_header: [
            0x40 as libc::c_int as libc::c_uchar,
            0xff as libc::c_int as libc::c_uchar,
            0x22 as libc::c_int as libc::c_uchar,
        ],
        cmd_len: 3 as libc::c_int + (92 as libc::c_int + 3 as libc::c_int),
        resp_len: 2 as libc::c_int,
    };
    init
};
static mut elanmoc_enroll_cmd: elanmoc_cmd = {
    let mut init = elanmoc_cmd {
        cmd_header: [
            0x40 as libc::c_int as libc::c_uchar,
            0xff as libc::c_int as libc::c_uchar,
            0x1 as libc::c_int as libc::c_uchar,
        ],
        cmd_len: 7 as libc::c_int,
        resp_len: 2 as libc::c_int,
    };
    init
};
static mut elanmoc_enroll_commit_cmd: elanmoc_cmd = {
    let mut init = elanmoc_cmd {
        cmd_header: [
            0x40 as libc::c_int as libc::c_uchar,
            0xff as libc::c_int as libc::c_uchar,
            0x11 as libc::c_int as libc::c_uchar,
        ],
        cmd_len: 128 as libc::c_int,
        resp_len: 2 as libc::c_int,
    };
    init
};
static mut elanmoc_set_mod_cmd: elanmoc_cmd = {
    let mut init = elanmoc_cmd {
        cmd_header: [
            0x40 as libc::c_int as libc::c_uchar,
            0xff as libc::c_int as libc::c_uchar,
            0x14 as libc::c_int as libc::c_uchar,
        ],
        cmd_len: 4 as libc::c_int,
        resp_len: 2 as libc::c_int,
    };
    init
};
static mut elanmoc_verify_cmd: elanmoc_cmd = {
    let mut init = elanmoc_cmd {
        cmd_header: [
            0x40 as libc::c_int as libc::c_uchar,
            0xff as libc::c_int as libc::c_uchar,
            0x73 as libc::c_int as libc::c_uchar,
        ],
        cmd_len: 5 as libc::c_int,
        resp_len: 2 as libc::c_int,
    };
    init
};
static mut enrolled_number_cmd: elanmoc_cmd = {
    let mut init = elanmoc_cmd {
        cmd_header: [
            0x40 as libc::c_int as libc::c_uchar,
            0xff as libc::c_int as libc::c_uchar,
            0x4 as libc::c_int as libc::c_uchar,
        ],
        cmd_len: 3 as libc::c_int,
        resp_len: 2 as libc::c_int,
    };
    init
};
static mut elanmoc_delete_cmd: elanmoc_cmd = {
    let mut init = elanmoc_cmd {
        cmd_header: [
            0x40 as libc::c_int as libc::c_uchar,
            0xff as libc::c_int as libc::c_uchar,
            0x13 as libc::c_int as libc::c_uchar,
        ],
        cmd_len: 128 as libc::c_int,
        resp_len: 2 as libc::c_int,
    };
    init
};
static mut fpi_device_elanmoc_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn fpi_device_elanmoc_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_elanmoc_get_type_once();
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
unsafe extern "C" fn fpi_device_elanmoc_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_device_get_type(),
        g_intern_static_string(
            b"FpiDeviceElanmoc\0" as *const u8 as *const libc::c_char,
        ),
        ::core::mem::size_of::<FpiDeviceElanmocClass>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_elanmoc_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpiDeviceElanmoc>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpiDeviceElanmoc) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_elanmoc_init
                        as unsafe extern "C" fn(*mut FpiDeviceElanmoc) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn fpi_device_elanmoc_class_intern_init(mut klass: gpointer) {
    fpi_device_elanmoc_parent_class = g_type_class_peek_parent(klass);
    if FpiDeviceElanmoc_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpiDeviceElanmoc_private_offset);
    }
    fpi_device_elanmoc_class_init(klass as *mut FpiDeviceElanmocClass);
}
static mut FpiDeviceElanmoc_private_offset: gint = 0;
static mut id_table: [FpIdEntry; 7] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0xc7d as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
                        pid: 0xc7e as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
                        pid: 0xc82 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
                        pid: 0xc88 as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
                        pid: 0xc8c as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
                        pid: 0xc8d as libc::c_int as guint,
                        vid: 0x4f3 as libc::c_int as guint,
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
unsafe extern "C" fn elanmoc_compose_cmd(
    mut cmd_info: *const elanmoc_cmd,
) -> *mut uint8_t {
    let mut cmd_buf: *mut libc::c_char = 0 as *mut libc::c_char;
    cmd_buf = g_malloc0((*cmd_info).cmd_len as gsize) as *mut libc::c_char;
    if (*cmd_info).cmd_len < 3 as libc::c_int {
        memcpy(
            cmd_buf as *mut libc::c_void,
            &(*cmd_info).cmd_header as *const [libc::c_uchar; 3] as *const libc::c_void,
            (*cmd_info).cmd_len as libc::c_ulong,
        );
    } else {
        memcpy(
            cmd_buf as *mut libc::c_void,
            &(*cmd_info).cmd_header as *const [libc::c_uchar; 3] as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        );
    }
    return (if 0 as libc::c_int != 0 {
        cmd_buf as *mut libc::c_void
    } else {
        g_steal_pointer(&mut cmd_buf as *mut *mut libc::c_char as gpointer)
    }) as *mut uint8_t;
}
unsafe extern "C" fn elanmoc_cmd_ack_cb(
    mut self_0: *mut FpiDeviceElanmoc,
    mut buffer_in: *mut uint8_t,
    mut length_in: gsize,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if length_in == 0 as libc::c_int as libc::c_ulong {
        fpi_ssm_next_state((*self_0).task_ssm);
        return;
    }
    if *buffer_in.offset(0 as libc::c_int as isize) as libc::c_int != 0x40 as libc::c_int
        && *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int
            != 0 as libc::c_int
    {
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Can't get response!!\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else {
        fpi_ssm_next_state((*self_0).task_ssm);
    };
}
unsafe extern "C" fn fp_cmd_receive_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut userdata: gpointer,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceElanmoc = FPI_DEVICE_ELANMOC(device as gpointer);
    let mut data: *mut CommandData = userdata as *mut CommandData;
    let mut ssm_state: libc::c_int = 0 as libc::c_int;
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    if data.is_null() {
        fpi_ssm_mark_failed(
            (*transfer).ssm,
            fpi_device_error_new(FP_DEVICE_ERROR_GENERAL),
        );
        return;
    }
    ssm_state = fpi_ssm_get_cur_state((*transfer).ssm);
    if (*transfer).actual_length == 0 as libc::c_int as libc::c_long {
        fpi_ssm_jump_to_state((*transfer).ssm, ssm_state);
        return;
    }
    if ((*data).callback).is_some() {
        ((*data).callback)
            .expect(
                "non-null function pointer",
            )(
            self_0,
            (*transfer).buffer,
            (*transfer).actual_length as gsize,
            0 as *mut GError,
        );
    }
    fpi_ssm_mark_completed((*transfer).ssm);
}
unsafe extern "C" fn fp_cmd_run_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    let mut self_0: *mut FpiDeviceElanmoc = FPI_DEVICE_ELANMOC(dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            if !((*self_0).cmd_transfer).is_null() {
                (*(*self_0).cmd_transfer).ssm = ssm;
                fpi_usb_transfer_submit(
                    (if 0 as libc::c_int != 0 {
                        (*self_0).cmd_transfer as *mut libc::c_void
                    } else {
                        g_steal_pointer(
                            &mut (*self_0).cmd_transfer as *mut *mut FpiUsbTransfer
                                as gpointer,
                        )
                    }) as *mut FpiUsbTransfer,
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
            } else {
                fpi_ssm_next_state(ssm);
            }
        }
        1 => {
            if (*self_0).cmd_len_in == 0 as libc::c_int as libc::c_ulong {
                let mut data: *mut CommandData = fpi_ssm_get_data(ssm)
                    as *mut CommandData;
                if ((*data).callback).is_some() {
                    ((*data).callback)
                        .expect(
                            "non-null function pointer",
                        )(
                        self_0,
                        0 as *mut uint8_t,
                        0 as libc::c_int as gsize,
                        0 as *mut GError,
                    );
                }
                fpi_ssm_mark_completed(ssm);
                return;
            }
            transfer = fpi_usb_transfer_new(dev);
            (*transfer).ssm = ssm;
            fpi_usb_transfer_fill_bulk(
                transfer,
                (if (*self_0).cmd_cancelable != 0 {
                    0x4 as libc::c_int | LIBUSB_ENDPOINT_IN as libc::c_int
                } else {
                    0x3 as libc::c_int | LIBUSB_ENDPOINT_IN as libc::c_int
                }) as guint8,
                (*self_0).cmd_len_in,
            );
            fpi_usb_transfer_submit(
                transfer,
                (if (*self_0).cmd_cancelable != 0 {
                    0 as libc::c_int
                } else {
                    5000 as libc::c_int
                }) as guint,
                if (*self_0).cmd_cancelable != 0 {
                    fpi_device_get_cancellable(dev)
                } else {
                    0 as *mut GCancellable
                },
                Some(
                    fp_cmd_receive_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            gpointer,
                            *mut GError,
                        ) -> (),
                ),
                fpi_ssm_get_data(ssm),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn fp_cmd_ssm_done(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceElanmoc = FPI_DEVICE_ELANMOC(dev as gpointer);
    let mut data: *mut CommandData = fpi_ssm_get_data(ssm) as *mut CommandData;
    (*self_0).cmd_ssm = 0 as *mut FpiSsm;
    if !error.is_null() {
        if ((*data).callback).is_some() {
            ((*data).callback)
                .expect(
                    "non-null function pointer",
                )(self_0, 0 as *mut uint8_t, 0 as libc::c_int as gsize, error);
        } else {
            g_error_free(error);
        }
    }
}
unsafe extern "C" fn fp_cmd_ssm_done_data_free(mut data: *mut CommandData) {
    g_free(data as gpointer);
}
unsafe extern "C" fn elanmoc_get_cmd(
    mut device: *mut FpDevice,
    mut buffer_out: *mut guint8,
    mut length_out: gsize,
    mut length_in: gsize,
    mut cancelable: gboolean,
    mut callback: SynCmdMsgCallback,
) {
    let mut self_0: *mut FpiDeviceElanmoc = FPI_DEVICE_ELANMOC(device as gpointer);
    let mut transfer: FpiUsbTransfer_autoptr = 0 as FpiUsbTransfer_autoptr;
    let mut data: *mut CommandData = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<CommandData>() as libc::c_ulong;
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
    }) as *mut CommandData;
    transfer = fpi_usb_transfer_new(device);
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_fill_bulk_full(
        transfer,
        (0x1 as libc::c_int | LIBUSB_ENDPOINT_OUT as libc::c_int) as guint8,
        buffer_out,
        length_out,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*data).callback = callback;
    (*self_0)
        .cmd_transfer = (if 0 as libc::c_int != 0 {
        transfer as *mut libc::c_void
    } else {
        g_steal_pointer(&mut transfer as *mut FpiUsbTransfer_autoptr as gpointer)
    }) as *mut FpiUsbTransfer;
    (*self_0).cmd_len_in = length_in;
    (*self_0).cmd_cancelable = cancelable;
    (*self_0)
        .cmd_ssm = fpi_ssm_new_full(
        FP_DEVICE(self_0 as gpointer),
        Some(fp_cmd_run_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
        FP_CMD_NUM_STATES as libc::c_int,
        FP_CMD_NUM_STATES as libc::c_int,
        b"FP_CMD_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_set_data(
        (*self_0).cmd_ssm,
        data as gpointer,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut CommandData) -> ()>,
            GDestroyNotify,
        >(
            Some(
                fp_cmd_ssm_done_data_free as unsafe extern "C" fn(*mut CommandData) -> (),
            ),
        ),
    );
    fpi_ssm_start(
        (*self_0).cmd_ssm,
        Some(
            fp_cmd_ssm_done
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn enroll_status_report(
    mut self_0: *mut FpiDeviceElanmoc,
    mut enroll_status_id: libc::c_int,
    mut data: libc::c_int,
    mut error: *mut GError,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    if !error.is_null() {
        fpi_device_enroll_complete(device, 0 as *mut FpPrint, error);
        return;
    }
    match enroll_status_id {
        0 => {
            fpi_device_enroll_progress(
                device,
                (*self_0).num_frames,
                0 as *mut FpPrint,
                fpi_device_retry_new(FP_DEVICE_RETRY_CENTER_FINGER),
            );
        }
        1 => {
            fpi_device_enroll_progress(
                device,
                (*self_0).num_frames,
                0 as *mut FpPrint,
                0 as *mut GError,
            );
        }
        2 => {
            let mut print: *mut FpPrint = 0 as *mut FpPrint;
            g_log(
                b"libfprint-elanmoc\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Enrollment was successful!\0" as *const u8 as *const libc::c_char,
            );
            fpi_device_get_enroll_data(device, &mut print);
            fpi_device_enroll_complete(
                device,
                g_object_ref(print as gpointer) as *mut FpPrint,
                0 as *mut GError,
            );
        }
        3 => {
            fpi_device_enroll_complete(
                device,
                0 as *mut FpPrint,
                fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_GENERAL,
                    b"Enrollment failed (%d) (ENROLL_RSP_ENROLL_CANCEL_REPORT)\0"
                        as *const u8 as *const libc::c_char,
                    data,
                ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn elanmoc_get_enrolled_cb(
    mut self_0: *mut FpiDeviceElanmoc,
    mut buffer_in: *mut uint8_t,
    mut length_in: gsize,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if *buffer_in.offset(0 as libc::c_int as isize) as libc::c_int != 0x40 as libc::c_int
    {
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Can't get response!!\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else {
        g_log(
            b"libfprint-elanmoc\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"elanmoc Current enrolled fingers in the Chipset: %d (0x%.2X 0x%.2X)\0"
                as *const u8 as *const libc::c_char,
            *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int,
            *buffer_in.offset(0 as libc::c_int as isize) as libc::c_int,
            *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int,
        );
        (*self_0)
            .curr_enrolled = *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int;
        fpi_ssm_next_state((*self_0).task_ssm);
    };
}
unsafe extern "C" fn elanmoc_reenroll_cb(
    mut self_0: *mut FpiDeviceElanmoc,
    mut buffer_in: *mut uint8_t,
    mut length_in: gsize,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if *buffer_in.offset(0 as libc::c_int as isize) as libc::c_int != 0x40 as libc::c_int
    {
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Can't get response!!\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else {
        if (*self_0).curr_enrolled == 9 as libc::c_int + 1 as libc::c_int
            && *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
        {
            g_log(
                b"libfprint-elanmoc\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"elanmoc_reenroll_cb over enroll max\0" as *const u8
                    as *const libc::c_char,
            );
            fpi_ssm_mark_failed(
                (*self_0).task_ssm,
                fpi_device_error_new(FP_DEVICE_ERROR_DATA_FULL),
            );
            return;
        }
        if *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int
            == 0 as libc::c_int
        {
            g_log(
                b"libfprint-elanmoc\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"##### Normal Enrollment Case! #####\0" as *const u8
                    as *const libc::c_char,
            );
        } else if *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int
            == 0x1 as libc::c_int
        {
            g_log(
                b"libfprint-elanmoc\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"##### Re-Enrollment Case! #####\0" as *const u8 as *const libc::c_char,
            );
        }
        (*self_0).num_frames = 0 as libc::c_int;
        fpi_ssm_next_state((*self_0).task_ssm);
    };
}
unsafe extern "C" fn elanmoc_enroll_cb(
    mut self_0: *mut FpiDeviceElanmoc,
    mut buffer_in: *mut uint8_t,
    mut length_in: gsize,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if *buffer_in.offset(0 as libc::c_int as isize) as libc::c_int != 0x40 as libc::c_int
    {
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Can't get response!!\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else {
        if *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int
            == 0 as libc::c_int
        {
            (*self_0).num_frames += 1 as libc::c_int;
            enroll_status_report(
                self_0,
                ENROLL_RSP_ENROLL_REPORT as libc::c_int,
                (*self_0).num_frames,
                0 as *mut GError,
            );
        } else {
            enroll_status_report(
                self_0,
                ENROLL_RSP_RETRY as libc::c_int,
                (*self_0).num_frames,
                0 as *mut GError,
            );
        }
        if (*self_0).num_frames == (*self_0).max_moc_enroll_time
            && *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
        {
            fpi_ssm_next_state((*self_0).task_ssm);
        } else if (*self_0).num_frames < (*self_0).max_moc_enroll_time {
            fpi_ssm_jump_to_state(
                (*self_0).task_ssm,
                MOC_ENROLL_WAIT_FINGER as libc::c_int,
            );
        } else {
            fpi_ssm_mark_failed((*self_0).task_ssm, error);
        }
    };
}
unsafe extern "C" fn elanmoc_commit_cb(
    mut self_0: *mut FpiDeviceElanmoc,
    mut buffer_in: *mut uint8_t,
    mut length_in: gsize,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if length_in == 0 as libc::c_int as libc::c_ulong {
        fpi_ssm_next_state((*self_0).task_ssm);
        return;
    }
    if *buffer_in.offset(0 as libc::c_int as isize) as libc::c_int != 0x40 as libc::c_int
        && *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int
            != 0 as libc::c_int
    {
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Can't get response!!\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else {
        g_log(
            b"libfprint-elanmoc\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"elanmoc_commit_cb success\0" as *const u8 as *const libc::c_char,
        );
        enroll_status_report(
            self_0,
            ENROLL_RSP_ENROLL_OK as libc::c_int,
            (*self_0).num_frames,
            0 as *mut GError,
        );
        fpi_ssm_next_state((*self_0).task_ssm);
    };
}
unsafe extern "C" fn elan_enroll_run_state(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceElanmoc = FPI_DEVICE_ELANMOC(dev as gpointer);
    let mut cmd_buf: *mut guint8 = 0 as *mut guint8;
    let mut data: *mut guint8 = 0 as *mut guint8;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            cmd_buf = elanmoc_compose_cmd(&enrolled_number_cmd);
            elanmoc_get_cmd(
                dev,
                cmd_buf,
                enrolled_number_cmd.cmd_len as gsize,
                enrolled_number_cmd.resp_len as gsize,
                0 as libc::c_int,
                Some(
                    elanmoc_get_enrolled_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceElanmoc,
                            *mut uint8_t,
                            gsize,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        1 => {
            data = fpi_ssm_get_data(ssm) as *mut guint8;
            cmd_buf = elanmoc_compose_cmd(&elanmoc_check_reenroll_cmd);
            memcpy(
                cmd_buf.offset(3 as libc::c_int as isize) as *mut libc::c_void,
                data as *const libc::c_void,
                (92 as libc::c_int + 3 as libc::c_int) as libc::c_ulong,
            );
            elanmoc_get_cmd(
                dev,
                cmd_buf,
                elanmoc_check_reenroll_cmd.cmd_len as gsize,
                elanmoc_check_reenroll_cmd.resp_len as gsize,
                0 as libc::c_int,
                Some(
                    elanmoc_reenroll_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceElanmoc,
                            *mut uint8_t,
                            gsize,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        2 => {
            cmd_buf = elanmoc_compose_cmd(&elanmoc_enroll_cmd);
            *cmd_buf
                .offset(3 as libc::c_int as isize) = (*self_0).curr_enrolled as guint8;
            *cmd_buf
                .offset(
                    4 as libc::c_int as isize,
                ) = (*self_0).max_moc_enroll_time as guint8;
            *cmd_buf.offset(5 as libc::c_int as isize) = (*self_0).num_frames as guint8;
            elanmoc_get_cmd(
                dev,
                cmd_buf,
                elanmoc_enroll_cmd.cmd_len as gsize,
                elanmoc_enroll_cmd.resp_len as gsize,
                1 as libc::c_int,
                Some(
                    elanmoc_enroll_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceElanmoc,
                            *mut uint8_t,
                            gsize,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        3 => {
            data = fpi_ssm_get_data(ssm) as *mut guint8;
            cmd_buf = elanmoc_compose_cmd(&elanmoc_enroll_commit_cmd);
            memcpy(
                cmd_buf.offset(5 as libc::c_int as isize) as *mut libc::c_void,
                data as *const libc::c_void,
                (92 as libc::c_int + 3 as libc::c_int) as libc::c_ulong,
            );
            elanmoc_get_cmd(
                dev,
                cmd_buf,
                elanmoc_enroll_commit_cmd.cmd_len as gsize,
                elanmoc_enroll_commit_cmd.resp_len as gsize,
                0 as libc::c_int,
                Some(
                    elanmoc_commit_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceElanmoc,
                            *mut uint8_t,
                            gsize,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn task_ssm_done(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceElanmoc = FPI_DEVICE_ELANMOC(dev as gpointer);
    (*self_0).task_ssm = 0 as *mut FpiSsm;
    let mut _pp: C2RustUnnamed_4 = C2RustUnnamed_4 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GPtrArray) -> ()>,
        GDestroyNotify,
    >(Some(g_ptr_array_unref as unsafe extern "C" fn(*mut GPtrArray) -> ()));
    _pp.in_0 = &mut (*self_0).list_result as *mut *mut GPtrArray as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    if !error.is_null() {
        fpi_device_action_error(dev, error);
    }
}
unsafe extern "C" fn create_print_from_response(
    mut self_0: *mut FpiDeviceElanmoc,
    mut buffer_in: *mut uint8_t,
    mut length_in: gsize,
    mut error: *mut *mut GError,
) -> *mut FpPrint {
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    let mut data: *mut GVariant = 0 as *mut GVariant;
    let mut uid: *mut GVariant = 0 as *mut GVariant;
    let mut userid: *mut gchar = 0 as *mut gchar;
    let mut userid_safe: *mut gchar = 0 as *mut gchar;
    let mut userid_len: libc::c_int = 0 as libc::c_int;
    if *buffer_in.offset(0 as libc::c_int as isize) as libc::c_int != 0x43 as libc::c_int
    {
        g_propagate_error(
            error,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Can't get response!!\0" as *const u8 as *const libc::c_char,
            ),
        );
        return 0 as *mut FpPrint;
    }
    if *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int {
        g_propagate_error(
            error,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Device returned error %d rather than print!\0" as *const u8
                    as *const libc::c_char,
                *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int,
            ),
        );
        return 0 as *mut FpPrint;
    }
    userid_len = *buffer_in.offset(4 as libc::c_int as isize) as libc::c_int;
    if userid_len as libc::c_ulong
        > length_in.wrapping_sub(5 as libc::c_int as libc::c_ulong)
    {
        g_propagate_error(
            error,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Packet too short for payload length!\0" as *const u8
                    as *const libc::c_char,
            ),
        );
        return 0 as *mut FpPrint;
    }
    userid = ({
        g_memdup2(
            &mut *buffer_in.offset(5 as libc::c_int as isize) as *mut uint8_t
                as gconstpointer,
            userid_len as gsize,
        )
    }) as *mut gchar;
    userid_safe = g_strndup(
        &mut *buffer_in.offset(5 as libc::c_int as isize) as *mut uint8_t
            as *const libc::c_char,
        userid_len as gsize,
    );
    print = fp_print_new(FP_DEVICE(self_0 as gpointer));
    uid = g_variant_new_fixed_array(
        b"y\0" as *const u8 as *const libc::c_char as *const GVariantType,
        userid as gconstpointer,
        userid_len as gsize,
        1 as libc::c_int as gsize,
    );
    data = g_variant_new(
        b"(yy@ay)\0" as *const u8 as *const libc::c_char,
        *buffer_in.offset(2 as libc::c_int as isize) as libc::c_int,
        *buffer_in.offset(3 as libc::c_int as isize) as libc::c_int,
        uid,
    );
    fpi_print_set_type(print, FPI_PRINT_RAW);
    fpi_print_set_device_stored(print, (0 as libc::c_int == 0) as libc::c_int);
    g_object_set(
        print as gpointer,
        b"fpi-data\0" as *const u8 as *const libc::c_char,
        data,
        0 as *mut libc::c_void,
    );
    g_object_set(
        print as gpointer,
        b"description\0" as *const u8 as *const libc::c_char,
        userid_safe,
        0 as *mut libc::c_void,
    );
    fpi_print_fill_from_user_id(print, userid_safe);
    return print;
}
unsafe extern "C" fn elanmoc_get_userid_cb(
    mut self_0: *mut FpiDeviceElanmoc,
    mut buffer_in: *mut uint8_t,
    mut length_in: gsize,
    mut error: *mut GError,
) {
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if *buffer_in.offset(0 as libc::c_int as isize) as libc::c_int != 0x43 as libc::c_int
    {
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Can't get response!!\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    (*self_0).list_index += 1;
    if *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int != 0xfe as libc::c_int
    {
        print = create_print_from_response(self_0, buffer_in, length_in, &mut error);
        if print.is_null() {
            fpi_ssm_mark_failed((*self_0).task_ssm, error);
            return;
        }
        g_ptr_array_add(
            (*self_0).list_result,
            g_object_ref_sink(print as gpointer) as *mut FpPrint as gpointer,
        );
    }
    if (*self_0).list_index <= 9 as libc::c_int {
        fpi_ssm_jump_to_state((*self_0).task_ssm, MOC_LIST_GET_FINGER as libc::c_int);
    } else {
        fpi_device_list_complete(
            FP_DEVICE(self_0 as gpointer),
            (if 0 as libc::c_int != 0 {
                (*self_0).list_result as *mut libc::c_void
            } else {
                g_steal_pointer(
                    &mut (*self_0).list_result as *mut *mut GPtrArray as gpointer,
                )
            }) as *mut GPtrArray,
            0 as *mut GError,
        );
        fpi_ssm_next_state((*self_0).task_ssm);
    };
}
unsafe extern "C" fn elan_list_run_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceElanmoc = FPI_DEVICE_ELANMOC(dev as gpointer);
    let mut cmd_buf: *mut guint8 = 0 as *mut guint8;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            cmd_buf = elanmoc_compose_cmd(&enrolled_number_cmd);
            elanmoc_get_cmd(
                dev,
                cmd_buf,
                enrolled_number_cmd.cmd_len as gsize,
                enrolled_number_cmd.resp_len as gsize,
                0 as libc::c_int,
                Some(
                    elanmoc_get_enrolled_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceElanmoc,
                            *mut uint8_t,
                            gsize,
                            *mut GError,
                        ) -> (),
                ),
            );
            (*self_0).list_index = 0 as libc::c_int;
        }
        1 => {
            cmd_buf = elanmoc_compose_cmd(&elanmoc_get_userid_cmd);
            *cmd_buf.offset(2 as libc::c_int as isize) = (*self_0).list_index as guint8;
            elanmoc_get_cmd(
                dev,
                cmd_buf,
                elanmoc_get_userid_cmd.cmd_len as gsize,
                elanmoc_get_userid_cmd.resp_len as gsize,
                0 as libc::c_int,
                Some(
                    elanmoc_get_userid_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceElanmoc,
                            *mut uint8_t,
                            gsize,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn elanmoc_list(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceElanmoc = FPI_DEVICE_ELANMOC(device as gpointer);
    (*self_0)
        .list_result = g_ptr_array_new_with_free_func(
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*self_0)
        .task_ssm = fpi_ssm_new_full(
        FP_DEVICE(self_0 as gpointer),
        Some(
            elan_list_run_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        MOC_LIST_NUM_STATES as libc::c_int,
        MOC_LIST_NUM_STATES as libc::c_int,
        b"MOC_LIST_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        (*self_0).task_ssm,
        Some(
            task_ssm_done
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn elanmoc_match_report_cb(
    mut self_0: *mut FpiDeviceElanmoc,
    mut buffer_in: *mut uint8_t,
    mut length_in: gsize,
    mut error: *mut GError,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    let mut verify_print: *mut FpPrint = 0 as *mut FpPrint;
    let mut prints: *mut GPtrArray = 0 as *mut GPtrArray;
    let mut found: gboolean = 0 as libc::c_int;
    let mut index: guint = 0;
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if *buffer_in.offset(0 as libc::c_int as isize) as libc::c_int != 0x43 as libc::c_int
    {
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Can't get response!!\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    print = create_print_from_response(self_0, buffer_in, length_in, &mut error);
    if print.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    g_log(
        b"libfprint-elanmoc\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Verify/Identify successful for: %s\0" as *const u8 as *const libc::c_char,
        fp_print_get_description(print),
    );
    if fpi_device_get_current_action(device) as libc::c_uint
        == FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
    {
        fpi_device_get_identify_data(device, &mut prints);
        found = g_ptr_array_find_with_equal_func(
            prints,
            print as gconstpointer,
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpPrint, *mut FpPrint) -> gboolean>,
                GEqualFunc,
            >(
                Some(
                    fp_print_equal
                        as unsafe extern "C" fn(*mut FpPrint, *mut FpPrint) -> gboolean,
                ),
            ),
            &mut index,
        );
        if found != 0 {
            fpi_device_identify_report(
                device,
                *((*prints).pdata).offset(index as isize) as *mut FpPrint,
                print,
                0 as *mut GError,
            );
        } else {
            fpi_device_identify_report(
                device,
                0 as *mut FpPrint,
                print,
                0 as *mut GError,
            );
        }
        fpi_device_identify_complete(device, 0 as *mut GError);
    } else {
        fpi_device_get_verify_data(device, &mut verify_print);
        if fp_print_equal(verify_print, print) != 0 {
            fpi_device_verify_report(device, FPI_MATCH_SUCCESS, print, 0 as *mut GError);
        } else {
            fpi_device_verify_report(device, FPI_MATCH_FAIL, print, 0 as *mut GError);
        }
        fpi_device_verify_complete(device, 0 as *mut GError);
    };
}
unsafe extern "C" fn identify_status_report(
    mut self_0: *mut FpiDeviceElanmoc,
    mut verify_status_id: libc::c_int,
    mut data: libc::c_int,
    mut error: *mut GError,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    let mut cmd_buf: *mut guint8 = 0 as *mut guint8;
    if !error.is_null() {
        if fpi_device_get_current_action(device) as libc::c_uint
            == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
        {
            fpi_device_verify_complete(device, error);
        } else {
            fpi_device_identify_complete(device, error);
        }
        return;
    }
    match verify_status_id {
        0 => {
            if data == 0xfd as libc::c_int {
                if fpi_device_get_current_action(device) as libc::c_uint
                    == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
                {
                    fpi_device_verify_report(
                        device,
                        FPI_MATCH_FAIL,
                        0 as *mut FpPrint,
                        0 as *mut GError,
                    );
                    fpi_device_verify_complete(device, 0 as *mut GError);
                } else {
                    fpi_device_identify_report(
                        device,
                        0 as *mut FpPrint,
                        0 as *mut FpPrint,
                        0 as *mut GError,
                    );
                    fpi_device_identify_complete(device, 0 as *mut GError);
                }
            } else {
                let mut retry_error: *mut GError = 0 as *mut GError;
                match data {
                    65 | 67 | 68 | 66 => {
                        retry_error = fpi_device_retry_new(
                            FP_DEVICE_RETRY_CENTER_FINGER,
                        );
                    }
                    _ => {
                        retry_error = fpi_device_retry_new(FP_DEVICE_RETRY_GENERAL);
                    }
                }
                if fpi_device_get_current_action(device) as libc::c_uint
                    == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
                {
                    fpi_device_verify_report(
                        device,
                        FPI_MATCH_ERROR,
                        0 as *mut FpPrint,
                        retry_error,
                    );
                    fpi_device_verify_complete(device, 0 as *mut GError);
                } else {
                    fpi_device_identify_report(
                        device,
                        0 as *mut FpPrint,
                        0 as *mut FpPrint,
                        retry_error,
                    );
                    fpi_device_identify_complete(device, 0 as *mut GError);
                }
            }
        }
        1 => {
            g_log(
                b"libfprint-elanmoc\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Verify was successful! for user: %d mesg_code: %d \0" as *const u8
                    as *const libc::c_char,
                data,
                verify_status_id,
            );
            cmd_buf = elanmoc_compose_cmd(&elanmoc_get_userid_cmd);
            *cmd_buf.offset(2 as libc::c_int as isize) = data as guint8;
            elanmoc_get_cmd(
                device,
                cmd_buf,
                elanmoc_get_userid_cmd.cmd_len as gsize,
                elanmoc_get_userid_cmd.resp_len as gsize,
                0 as libc::c_int,
                Some(
                    elanmoc_match_report_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceElanmoc,
                            *mut uint8_t,
                            gsize,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn elanmoc_identify_cb(
    mut self_0: *mut FpiDeviceElanmoc,
    mut buffer_in: *mut uint8_t,
    mut length_in: gsize,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int == 0xfd as libc::c_int
    {
        identify_status_report(
            self_0,
            RSP_VERIFY_FAIL as libc::c_int,
            *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int,
            error,
        );
    } else if *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int
        <= 9 as libc::c_int
    {
        identify_status_report(
            self_0,
            RSP_VERIFY_OK as libc::c_int,
            *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int,
            error,
        );
    } else {
        identify_status_report(
            self_0,
            RSP_VERIFY_FAIL as libc::c_int,
            *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int,
            error,
        );
    }
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn elan_identify_run_state(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
) {
    let mut cmd_buf: *mut guint8 = 0 as *mut guint8;
    g_log(
        b"libfprint-elanmoc\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"elanmoc %s \0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 24],
            &[libc::c_char; 24],
        >(b"elan_identify_run_state\0"))
            .as_ptr(),
    );
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            g_log(
                b"libfprint-elanmoc\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"elanmoc %s IDENTIFY_SET_MODE\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"elan_identify_run_state\0"))
                    .as_ptr(),
            );
            cmd_buf = elanmoc_compose_cmd(&elanmoc_set_mod_cmd);
            *cmd_buf.offset(3 as libc::c_int as isize) = 0x3 as libc::c_int as guint8;
            elanmoc_get_cmd(
                dev,
                cmd_buf,
                elanmoc_set_mod_cmd.cmd_len as gsize,
                elanmoc_set_mod_cmd.resp_len as gsize,
                0 as libc::c_int,
                Some(
                    elanmoc_cmd_ack_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceElanmoc,
                            *mut uint8_t,
                            gsize,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        1 => {
            g_log(
                b"libfprint-elanmoc\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"elanmoc %s VERIFY_WAIT_FINGER\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"elan_identify_run_state\0"))
                    .as_ptr(),
            );
            cmd_buf = elanmoc_compose_cmd(&elanmoc_verify_cmd);
            elanmoc_get_cmd(
                dev,
                cmd_buf,
                elanmoc_verify_cmd.cmd_len as gsize,
                elanmoc_verify_cmd.resp_len as gsize,
                1 as libc::c_int,
                Some(
                    elanmoc_identify_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceElanmoc,
                            *mut uint8_t,
                            gsize,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn elanmoc_enroll(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceElanmoc = FPI_DEVICE_ELANMOC(device as gpointer);
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    let mut data: *mut GVariant = 0 as *mut GVariant;
    let mut uid: *mut GVariant = 0 as *mut GVariant;
    let mut user_id: *mut gchar = 0 as *mut gchar;
    let mut user_id_len: gsize = 0;
    let mut userdata: *mut guint8 = g_malloc0(
        (92 as libc::c_int + 3 as libc::c_int) as gsize,
    ) as *mut guint8;
    fpi_device_get_enroll_data(device, &mut print);
    user_id = fpi_print_generate_user_id(print);
    user_id_len = strlen(user_id);
    user_id_len = if (92 as libc::c_int as libc::c_ulong) < user_id_len {
        92 as libc::c_int as libc::c_ulong
    } else {
        user_id_len
    };
    uid = g_variant_new_fixed_array(
        b"y\0" as *const u8 as *const libc::c_char as *const GVariantType,
        user_id as gconstpointer,
        user_id_len,
        1 as libc::c_int as gsize,
    );
    data = g_variant_new(
        b"(yy@ay)\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        0 as libc::c_int,
        uid,
    );
    fpi_print_set_type(print, FPI_PRINT_RAW);
    fpi_print_set_device_stored(print, (0 as libc::c_int == 0) as libc::c_int);
    g_object_set(
        print as gpointer,
        b"fpi-data\0" as *const u8 as *const libc::c_char,
        data,
        0 as *mut libc::c_void,
    );
    g_object_set(
        print as gpointer,
        b"description\0" as *const u8 as *const libc::c_char,
        user_id,
        0 as *mut libc::c_void,
    );
    *userdata.offset(0 as libc::c_int as isize) = 0 as libc::c_int as guint8;
    *userdata.offset(1 as libc::c_int as isize) = 0 as libc::c_int as guint8;
    *userdata.offset(2 as libc::c_int as isize) = user_id_len as guint8;
    memcpy(
        userdata.offset(3 as libc::c_int as isize) as *mut libc::c_void,
        user_id as *const libc::c_void,
        user_id_len,
    );
    (*self_0)
        .task_ssm = fpi_ssm_new_full(
        FP_DEVICE(self_0 as gpointer),
        Some(
            elan_enroll_run_state
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        MOC_ENROLL_NUM_STATES as libc::c_int,
        MOC_ENROLL_NUM_STATES as libc::c_int,
        b"MOC_ENROLL_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_set_data(
        (*self_0).task_ssm,
        userdata as gpointer,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut CommandData) -> ()>,
            GDestroyNotify,
        >(
            Some(
                fp_cmd_ssm_done_data_free as unsafe extern "C" fn(*mut CommandData) -> (),
            ),
        ),
    );
    fpi_ssm_start(
        (*self_0).task_ssm,
        Some(
            task_ssm_done
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn elanmoc_delete_cb(
    mut self_0: *mut FpiDeviceElanmoc,
    mut buffer_in: *mut uint8_t,
    mut length_in: gsize,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if *buffer_in.offset(0 as libc::c_int as isize) as libc::c_int != 0x40 as libc::c_int
        && *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int
            != 0 as libc::c_int
    {
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Can't get response!!\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else {
        fpi_device_delete_complete(FP_DEVICE(self_0 as gpointer), 0 as *mut GError);
        fpi_ssm_next_state((*self_0).task_ssm);
    };
}
unsafe extern "C" fn elan_delete_run_state(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
) {
    let mut cmd_buf: *mut guint8 = 0 as *mut guint8;
    let mut data: *mut guint8 = fpi_ssm_get_data(ssm) as *mut guint8;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            cmd_buf = elanmoc_compose_cmd(&elanmoc_delete_cmd);
            memcpy(
                cmd_buf.offset(3 as libc::c_int as isize) as *mut libc::c_void,
                data as *const libc::c_void,
                (92 as libc::c_int + 3 as libc::c_int) as libc::c_ulong,
            );
            elanmoc_get_cmd(
                dev,
                cmd_buf,
                elanmoc_delete_cmd.cmd_len as gsize,
                elanmoc_delete_cmd.resp_len as gsize,
                0 as libc::c_int,
                Some(
                    elanmoc_delete_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceElanmoc,
                            *mut uint8_t,
                            gsize,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn elanmoc_delete_print(mut device: *mut FpDevice) {
    let mut data: GVariant_autoptr = 0 as GVariant_autoptr;
    let mut user_id_var: GVariant_autoptr = 0 as GVariant_autoptr;
    let mut self_0: *mut FpiDeviceElanmoc = FPI_DEVICE_ELANMOC(device as gpointer);
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    let mut user_id: *const guint8 = 0 as *const guint8;
    let mut user_id_safe: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut user_id_len: gsize = 0 as libc::c_int as gsize;
    let mut userid_buf: *mut guint8 = 0 as *mut guint8;
    fpi_device_get_delete_data(device, &mut print);
    g_object_get(
        print as gpointer,
        b"fpi-data\0" as *const u8 as *const libc::c_char,
        &mut data as *mut GVariant_autoptr,
        0 as *mut libc::c_void,
    );
    if g_variant_check_format_string(
        data,
        b"(yy@ay)\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) == 0
    {
        fpi_device_delete_complete(
            device,
            fpi_device_error_new(FP_DEVICE_ERROR_DATA_INVALID),
        );
        return;
    }
    userid_buf = g_malloc0((92 as libc::c_int + 3 as libc::c_int) as gsize)
        as *mut guint8;
    g_variant_get(
        data,
        b"(yy@ay)\0" as *const u8 as *const libc::c_char,
        &mut *userid_buf.offset(0 as libc::c_int as isize) as *mut guint8,
        &mut *userid_buf.offset(1 as libc::c_int as isize) as *mut guint8,
        &mut user_id_var as *mut GVariant_autoptr,
    );
    user_id = g_variant_get_fixed_array(
        user_id_var,
        &mut user_id_len,
        1 as libc::c_int as gsize,
    ) as *const guint8;
    user_id_safe = g_strndup(user_id as *const libc::c_char, user_id_len);
    user_id_len = if (92 as libc::c_int as libc::c_ulong) < user_id_len {
        92 as libc::c_int as libc::c_ulong
    } else {
        user_id_len
    };
    *userid_buf.offset(2 as libc::c_int as isize) = user_id_len as guint8;
    memcpy(
        userid_buf.offset(3 as libc::c_int as isize) as *mut libc::c_void,
        user_id as *const libc::c_void,
        user_id_len,
    );
    g_log(
        b"libfprint-elanmoc\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Delete Finger, user_id = %s!\0" as *const u8 as *const libc::c_char,
        user_id_safe,
    );
    (*self_0)
        .task_ssm = fpi_ssm_new_full(
        device,
        Some(
            elan_delete_run_state
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        DELETE_NUM_STATES as libc::c_int,
        DELETE_NUM_STATES as libc::c_int,
        b"DELETE_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_set_data(
        (*self_0).task_ssm,
        userid_buf as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    fpi_ssm_start(
        (*self_0).task_ssm,
        Some(
            task_ssm_done
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn elanmoc_identify(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceElanmoc = FPI_DEVICE_ELANMOC(device as gpointer);
    (*self_0)
        .task_ssm = fpi_ssm_new_full(
        device,
        Some(
            elan_identify_run_state
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        IDENTIFY_NUM_STATES as libc::c_int,
        IDENTIFY_NUM_STATES as libc::c_int,
        b"IDENTIFY_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        (*self_0).task_ssm,
        Some(
            task_ssm_done
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn task_ssm_init_done(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceElanmoc = FPI_DEVICE_ELANMOC(dev as gpointer);
    if !error.is_null() {
        g_usb_device_release_interface(
            fpi_device_get_usb_device(dev),
            0 as libc::c_int,
            G_USB_DEVICE_CLAIM_INTERFACE_NONE,
            0 as *mut *mut GError,
        );
    }
    fpi_device_open_complete(FP_DEVICE(self_0 as gpointer), error);
}
unsafe extern "C" fn elanmoc_cmd_ver_cb(
    mut self_0: *mut FpiDeviceElanmoc,
    mut buffer_in: *mut uint8_t,
    mut length_in: gsize,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    (*self_0)
        .fw_ver = ((*buffer_in.offset(0 as libc::c_int as isize) as libc::c_int)
        << 8 as libc::c_int
        | *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int) as libc::c_ushort;
    g_log(
        b"libfprint-elanmoc\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"elanmoc  FW Version %x \0" as *const u8 as *const libc::c_char,
        (*self_0).fw_ver as libc::c_int,
    );
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn elanmoc_cmd_dim_cb(
    mut self_0: *mut FpiDeviceElanmoc,
    mut buffer_in: *mut uint8_t,
    mut length_in: gsize,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    (*self_0).x_trace = *buffer_in.offset(0 as libc::c_int as isize);
    (*self_0).y_trace = *buffer_in.offset(2 as libc::c_int as isize);
    g_log(
        b"libfprint-elanmoc\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"elanmoc last_read DIM 0x%.2X(%d) 0x%.2X(%d)\0" as *const u8
            as *const libc::c_char,
        (*self_0).x_trace as libc::c_int,
        (*self_0).x_trace as libc::c_int,
        (*self_0).y_trace as libc::c_int,
        (*self_0).y_trace as libc::c_int,
    );
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn elanmoc_get_status_cb(
    mut self_0: *mut FpiDeviceElanmoc,
    mut buffer_in: *mut uint8_t,
    mut length_in: gsize,
    mut error: *mut GError,
) {
    let mut cmd_buf: *mut guint8 = 0 as *mut guint8;
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if *buffer_in.offset(1 as libc::c_int as isize) as libc::c_int != 0x3 as libc::c_int
        && (*self_0).cmd_retry_cnt != 0 as libc::c_int
    {
        (*self_0).cmd_retry_cnt -= 1;
        cmd_buf = elanmoc_compose_cmd(&cal_status_cmd);
        elanmoc_get_cmd(
            FP_DEVICE(self_0 as gpointer),
            cmd_buf,
            cal_status_cmd.cmd_len as gsize,
            cal_status_cmd.resp_len as gsize,
            0 as libc::c_int,
            Some(
                elanmoc_get_status_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceElanmoc,
                        *mut uint8_t,
                        gsize,
                        *mut GError,
                    ) -> (),
            ),
        );
    } else {
        if (*self_0).cmd_retry_cnt == 0 as libc::c_int {
            fpi_ssm_mark_failed(
                (*self_0).task_ssm,
                fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_PROTO,
                    b"Sensor not ready\0" as *const u8 as *const libc::c_char,
                ),
            );
            return;
        }
        fpi_ssm_next_state((*self_0).task_ssm);
    };
}
unsafe extern "C" fn dev_init_handler(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceElanmoc = FPI_DEVICE_ELANMOC(dev as gpointer);
    let mut cmd_buf: *mut guint8 = 0 as *mut guint8;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            (*self_0).cmd_retry_cnt = 500 as libc::c_int;
            cmd_buf = elanmoc_compose_cmd(&cal_status_cmd);
            elanmoc_get_cmd(
                dev,
                cmd_buf,
                cal_status_cmd.cmd_len as gsize,
                cal_status_cmd.resp_len as gsize,
                0 as libc::c_int,
                Some(
                    elanmoc_get_status_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceElanmoc,
                            *mut uint8_t,
                            gsize,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        1 => {
            cmd_buf = elanmoc_compose_cmd(&elanmoc_set_mod_cmd);
            *cmd_buf.offset(3 as libc::c_int as isize) = 0x3 as libc::c_int as guint8;
            elanmoc_get_cmd(
                dev,
                cmd_buf,
                elanmoc_set_mod_cmd.cmd_len as gsize,
                elanmoc_set_mod_cmd.resp_len as gsize,
                0 as libc::c_int,
                Some(
                    elanmoc_cmd_ack_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceElanmoc,
                            *mut uint8_t,
                            gsize,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        2 => {
            cmd_buf = elanmoc_compose_cmd(&fw_ver_cmd);
            elanmoc_get_cmd(
                dev,
                cmd_buf,
                fw_ver_cmd.cmd_len as gsize,
                fw_ver_cmd.resp_len as gsize,
                0 as libc::c_int,
                Some(
                    elanmoc_cmd_ver_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceElanmoc,
                            *mut uint8_t,
                            gsize,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        3 => {
            cmd_buf = elanmoc_compose_cmd(&sensor_dim_cmd);
            elanmoc_get_cmd(
                dev,
                cmd_buf,
                sensor_dim_cmd.cmd_len as gsize,
                sensor_dim_cmd.resp_len as gsize,
                0 as libc::c_int,
                Some(
                    elanmoc_cmd_dim_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceElanmoc,
                            *mut uint8_t,
                            gsize,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        4 => {
            cmd_buf = elanmoc_compose_cmd(&enrolled_number_cmd);
            elanmoc_get_cmd(
                dev,
                cmd_buf,
                enrolled_number_cmd.cmd_len as gsize,
                enrolled_number_cmd.resp_len as gsize,
                0 as libc::c_int,
                Some(
                    elanmoc_get_enrolled_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceElanmoc,
                            *mut uint8_t,
                            gsize,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn elanmoc_open(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceElanmoc = FPI_DEVICE_ELANMOC(device as gpointer);
    let mut error: *mut GError = 0 as *mut GError;
    let mut productid: gint = 0 as libc::c_int;
    if !(g_usb_device_reset(fpi_device_get_usb_device(device), &mut error) == 0) {
        if !(g_usb_device_claim_interface(
            fpi_device_get_usb_device(device),
            0 as libc::c_int,
            G_USB_DEVICE_CLAIM_INTERFACE_NONE,
            &mut error,
        ) == 0)
        {
            productid = g_usb_device_get_pid(fpi_device_get_usb_device(device)) as gint;
            match productid {
                3212 => {
                    (*self_0).max_moc_enroll_time = 11 as libc::c_int;
                }
                3213 => {
                    (*self_0).max_moc_enroll_time = 17 as libc::c_int;
                }
                _ => {
                    (*self_0).max_moc_enroll_time = 9 as libc::c_int;
                }
            }
            fpi_device_set_nr_enroll_stages(device, (*self_0).max_moc_enroll_time);
            (*self_0)
                .task_ssm = fpi_ssm_new_full(
                FP_DEVICE(self_0 as gpointer),
                Some(
                    dev_init_handler
                        as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                ),
                DEV_INIT_STATES as libc::c_int,
                DEV_INIT_STATES as libc::c_int,
                b"DEV_INIT_STATES\0" as *const u8 as *const libc::c_char,
            );
            fpi_ssm_start(
                (*self_0).task_ssm,
                Some(
                    task_ssm_init_done
                        as unsafe extern "C" fn(
                            *mut FpiSsm,
                            *mut FpDevice,
                            *mut GError,
                        ) -> (),
                ),
            );
            return;
        }
    }
    fpi_device_open_complete(FP_DEVICE(self_0 as gpointer), error);
}
unsafe extern "C" fn task_ssm_exit_done(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceElanmoc = FPI_DEVICE_ELANMOC(dev as gpointer);
    g_usb_device_release_interface(
        fpi_device_get_usb_device(FP_DEVICE(self_0 as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    );
    fpi_device_close_complete(FP_DEVICE(self_0 as gpointer), error);
    (*self_0).task_ssm = 0 as *mut FpiSsm;
}
unsafe extern "C" fn dev_exit_handler(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut cmd_buf: *mut guint8 = 0 as *mut guint8;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            cmd_buf = elanmoc_compose_cmd(&elanmoc_above_cmd);
            elanmoc_get_cmd(
                dev,
                cmd_buf,
                elanmoc_above_cmd.cmd_len as gsize,
                elanmoc_above_cmd.resp_len as gsize,
                0 as libc::c_int,
                Some(
                    elanmoc_cmd_ack_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceElanmoc,
                            *mut uint8_t,
                            gsize,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn elanmoc_close(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceElanmoc = FPI_DEVICE_ELANMOC(device as gpointer);
    g_log(
        b"libfprint-elanmoc\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Elanmoc dev_exit\0" as *const u8 as *const libc::c_char,
    );
    (*self_0)
        .task_ssm = fpi_ssm_new_full(
        FP_DEVICE(self_0 as gpointer),
        Some(dev_exit_handler as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
        DEV_EXIT_STATES as libc::c_int,
        DEV_EXIT_STATES as libc::c_int,
        b"DEV_EXIT_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        (*self_0).task_ssm,
        Some(
            task_ssm_exit_done
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn fpi_device_elanmoc_init(mut self_0: *mut FpiDeviceElanmoc) {
    g_log_structured(
        b"libfprint-elanmoc\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/elanmoc/elanmoc.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"1145\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 24],
            &[libc::c_char; 24],
        >(b"fpi_device_elanmoc_init\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/elanmoc/elanmoc.c:1145\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn fpi_device_elanmoc_class_init(
    mut klass: *mut FpiDeviceElanmocClass,
) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    (*dev_class).id = b"elanmoc\0" as *const u8 as *const libc::c_char;
    (*dev_class).full_name = b"Elan MOC Sensors\0" as *const u8 as *const libc::c_char;
    (*dev_class).type_0 = FP_DEVICE_TYPE_USB;
    (*dev_class).scan_type = FP_SCAN_TYPE_PRESS;
    (*dev_class).id_table = id_table.as_ptr();
    (*dev_class).nr_enroll_stages = 9 as libc::c_int;
    (*dev_class).temp_hot_seconds = -(1 as libc::c_int);
    (*dev_class).open = Some(elanmoc_open as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .close = Some(elanmoc_close as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .verify = Some(elanmoc_identify as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .enroll = Some(elanmoc_enroll as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .identify = Some(elanmoc_identify as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .delete = Some(
        elanmoc_delete_print as unsafe extern "C" fn(*mut FpDevice) -> (),
    );
    (*dev_class).list = Some(elanmoc_list as unsafe extern "C" fn(*mut FpDevice) -> ());
    fpi_device_class_auto_initialize_features(dev_class);
}
