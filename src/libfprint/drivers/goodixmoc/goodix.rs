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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn g_ptr_array_new_with_free_func(
        element_free_func: GDestroyNotify,
    ) -> *mut GPtrArray;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_getenv(variable: *const gchar) -> *const gchar;
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
    fn g_return_if_fail_warning(
        log_domain: *const libc::c_char,
        pretty_function: *const libc::c_char,
        expression: *const libc::c_char,
    );
    fn g_str_has_suffix(str: *const gchar, suffix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strcmp0(str1: *const libc::c_char, str2: *const libc::c_char) -> libc::c_int;
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
    fn g_usb_device_get_serial_number_index(self_0: *mut GUsbDevice) -> guint8;
    fn g_usb_device_open(self_0: *mut GUsbDevice, error: *mut *mut GError) -> gboolean;
    fn g_usb_device_close(self_0: *mut GUsbDevice, error: *mut *mut GError) -> gboolean;
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
    fn g_usb_device_get_string_descriptor(
        self_0: *mut GUsbDevice,
        desc_index: guint8,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn fp_device_get_type() -> GType;
    fn fp_print_new(device: *mut FpDevice) -> *mut FpPrint;
    fn fp_print_get_description(print: *mut FpPrint) -> *const gchar;
    fn fp_print_equal(self_0: *mut FpPrint, other: *mut FpPrint) -> gboolean;
    fn fp_device_retry_quark() -> GQuark;
    fn fpi_print_set_type(print: *mut FpPrint, type_0: FpiPrintType);
    fn fpi_print_set_device_stored(print: *mut FpPrint, device_stored: gboolean);
    fn fpi_print_generate_user_id(print: *mut FpPrint) -> *mut gchar;
    fn fpi_print_fill_from_user_id(
        print: *mut FpPrint,
        user_id: *const libc::c_char,
    ) -> gboolean;
    fn fpi_device_class_auto_initialize_features(device_class: *mut FpDeviceClass);
    fn fpi_device_get_usb_device(device: *mut FpDevice) -> *mut GUsbDevice;
    fn fpi_device_get_current_action(device: *mut FpDevice) -> FpiDeviceAction;
    fn fpi_device_retry_new(error: FpDeviceRetry) -> *mut GError;
    fn fpi_device_error_new(error: FpDeviceError) -> *mut GError;
    fn fpi_device_error_new_msg(
        error: FpDeviceError,
        msg: *const gchar,
        _: ...
    ) -> *mut GError;
    fn fpi_device_get_enroll_data(device: *mut FpDevice, print: *mut *mut FpPrint);
    fn fpi_device_get_verify_data(device: *mut FpDevice, print: *mut *mut FpPrint);
    fn fpi_device_get_identify_data(device: *mut FpDevice, prints: *mut *mut GPtrArray);
    fn fpi_device_get_delete_data(device: *mut FpDevice, print: *mut *mut FpPrint);
    fn fpi_device_get_cancellable(device: *mut FpDevice) -> *mut GCancellable;
    fn fpi_device_set_nr_enroll_stages(device: *mut FpDevice, enroll_stages: gint);
    fn fpi_device_probe_complete(
        device: *mut FpDevice,
        device_id: *const gchar,
        device_name: *const gchar,
        error: *mut GError,
    );
    fn fpi_device_open_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_close_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_enroll_complete(
        device: *mut FpDevice,
        print: *mut FpPrint,
        error: *mut GError,
    );
    fn fpi_device_verify_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_identify_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_delete_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_list_complete(
        device: *mut FpDevice,
        prints: *mut GPtrArray,
        error: *mut GError,
    );
    fn fpi_device_clear_storage_complete(device: *mut FpDevice, error: *mut GError);
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
    fn fpi_device_identify_report(
        device: *mut FpDevice,
        match_0: *mut FpPrint,
        print: *mut FpPrint,
        error: *mut GError,
    );
    fn fpi_device_report_finger_status_changes(
        device: *mut FpDevice,
        added_status: FpFingerStatusFlags,
        removed_status: FpFingerStatusFlags,
    ) -> gboolean;
    fn fpi_usb_transfer_new(device: *mut FpDevice) -> *mut FpiUsbTransfer;
    fn fpi_usb_transfer_fill_bulk(
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
    fn gx_proto_build_package(
        ppackage: *mut uint8_t,
        package_len: *mut uint32_t,
        cmd: uint16_t,
        payload: *const uint8_t,
        payload_size: uint32_t,
    ) -> libc::c_int;
    fn gx_proto_parse_header(
        buffer: *mut uint8_t,
        buffer_len: uint32_t,
        pheader: *mut pack_header,
    ) -> libc::c_int;
    fn gx_proto_parse_body(
        cmd: uint16_t,
        buffer: *mut uint8_t,
        buffer_len: uint16_t,
        presponse: pgxfp_cmd_response_t,
    ) -> libc::c_int;
    fn gx_proto_init_sensor_config(pconfig: pgxfp_sensor_cfg_t) -> libc::c_int;
    fn gx_proto_crc32_calc(
        pchMsg: *mut uint8_t,
        wDataLen: uint32_t,
        pchMsgDst: *mut uint8_t,
    ) -> uint8_t;
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
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
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
pub type GError_autoptr = *mut GError;
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
pub type FpPrint_autoptr = *mut FpPrint;
pub type FpFingerStatusFlags = libc::c_uint;
pub const FP_FINGER_STATUS_PRESENT: FpFingerStatusFlags = 2;
pub const FP_FINGER_STATUS_NEEDED: FpFingerStatusFlags = 1;
pub const FP_FINGER_STATUS_NONE: FpFingerStatusFlags = 0;
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
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_version_info {
    pub format: [uint8_t; 2],
    pub fwtype: [uint8_t; 8],
    pub fwversion: [uint8_t; 8],
    pub customer: [uint8_t; 8],
    pub mcu: [uint8_t; 8],
    pub sensor: [uint8_t; 8],
    pub algversion: [uint8_t; 8],
    pub interface: [uint8_t; 8],
    pub protocol: [uint8_t; 8],
    pub flashVersion: [uint8_t; 8],
    pub reserved: [uint8_t; 38],
}
pub type gxfp_version_info_t = _gxfp_version_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_parse_msg {
    pub ack_cmd: uint8_t,
    pub has_no_config: bool,
}
pub type gxfp_parse_msg_t = _gxfp_parse_msg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_enroll_init {
    pub tid: [uint8_t; 32],
}
pub type gxfp_enroll_init_t = _gxfp_enroll_init;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct _template_format {
    pub _0x43_byte: uint8_t,
    pub type_0: uint8_t,
    pub finger_index: uint8_t,
    pub pad0: uint8_t,
    pub accountid: [uint8_t; 32],
    pub tid: [uint8_t; 32],
    pub payload: C2RustUnnamed_4,
    pub reserve: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct C2RustUnnamed_4 {
    pub size: uint8_t,
    pub data: [uint8_t; 56],
}
pub type template_format_t = _template_format;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_verify {
    pub match_0: bool,
    pub rejectdetail: uint32_t,
    pub template: template_format_t,
}
pub type gxfp_verify_t = _gxfp_verify;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_capturedata {
    pub img_quality: uint8_t,
    pub img_coverage: uint8_t,
}
pub type gxfp_capturedata_t = _gxfp_capturedata;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_check_duplicate {
    pub duplicate: bool,
    pub template: template_format_t,
}
pub type gxfp_check_duplicate_t = _gxfp_check_duplicate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_enroll_update {
    pub rollback: bool,
    pub img_overlay: uint8_t,
    pub img_preoverlay: uint8_t,
}
pub type gxfp_enroll_update_t = _gxfp_enroll_update;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_enum_fingerlist {
    pub finger_num: uint8_t,
    pub finger_list: [template_format_t; 20],
}
pub type gxfp_enum_fingerlist_t = _gxfp_enum_fingerlist;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_enroll_commit {
    pub result: uint8_t,
}
pub type gxfp_enroll_commit_t = _gxfp_enroll_commit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fp_finger_status {
    pub status: uint8_t,
}
pub type fp_finger_status_t = _fp_finger_status;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fp_finger_config {
    pub status: uint8_t,
    pub max_stored_prints: uint8_t,
}
pub type fp_finger_config_t = _fp_finger_config;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fp_pwr_btn_shield {
    pub resp_cmd1: uint8_t,
}
pub type fp_pwr_btn_shield_t = _fp_pwr_btn_shield;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fp_cmd_response {
    pub result: uint8_t,
    pub c2rust_unnamed: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub parse_msg: gxfp_parse_msg_t,
    pub verify: gxfp_verify_t,
    pub enroll_init: gxfp_enroll_init_t,
    pub capture_data_resp: gxfp_capturedata_t,
    pub check_duplicate_resp: gxfp_check_duplicate_t,
    pub enroll_commit: gxfp_enroll_commit_t,
    pub enroll_update: gxfp_enroll_update_t,
    pub finger_list_resp: gxfp_enum_fingerlist_t,
    pub version_info: gxfp_version_info_t,
    pub finger_status: fp_finger_status_t,
    pub finger_config: fp_finger_config_t,
    pub power_button_shield_resp: fp_pwr_btn_shield_t,
}
pub type gxfp_cmd_response_t = _fp_cmd_response;
pub type pgxfp_cmd_response_t = *mut _fp_cmd_response;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _pack_header {
    pub cmd0: uint8_t,
    pub cmd1: uint8_t,
    pub packagenum: uint8_t,
    pub reserved: uint8_t,
    pub len: uint16_t,
    pub crc8: uint8_t,
    pub rev_crc8: uint8_t,
}
pub type pack_header = _pack_header;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_sensor_cfg {
    pub config: [uint8_t; 26],
    pub reserved: [uint8_t; 98],
    pub crc_value: [uint8_t; 4],
}
pub type gxfp_sensor_cfg_t = _gxfp_sensor_cfg;
pub type pgxfp_sensor_cfg_t = *mut _gxfp_sensor_cfg;
pub type FpiDeviceGoodixMoc = _FpiDeviceGoodixMoc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceGoodixMoc {
    pub parent: FpDevice,
    pub task_ssm: *mut FpiSsm,
    pub cmd_ssm: *mut FpiSsm,
    pub cmd_transfer: *mut FpiUsbTransfer,
    pub cmd_cancelable: gboolean,
    pub sensorcfg: pgxfp_sensor_cfg_t,
    pub enroll_stage: gint,
    pub max_enroll_stage: gint,
    pub max_stored_prints: gint,
    pub list_result: *mut GPtrArray,
    pub template_id: [guint8; 32],
    pub is_power_button_shield_on: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiDeviceGoodixMocClass {
    pub parent_class: FpDeviceClass,
}
pub const FP_VERIFY_PWR_BTN_SHIELD_OFF: C2RustUnnamed_10 = 3;
pub const FP_VERIFY_NUM_STATES: C2RustUnnamed_10 = 4;
pub type SynCmdMsgCallback = Option::<
    unsafe extern "C" fn(
        *mut FpiDeviceGoodixMoc,
        *mut gxfp_cmd_response_t,
        *mut GError,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandData {
    pub cmd: guint8,
    pub callback: SynCmdMsgCallback,
}
pub const FP_CMD_NUM_STATES: C2RustUnnamed_7 = 3;
pub const FP_CMD_GET_DATA: C2RustUnnamed_7 = 2;
pub const FP_CMD_GET_ACK: C2RustUnnamed_7 = 1;
pub const FP_CMD_SEND: C2RustUnnamed_7 = 0;
pub const FP_VERIFY_IDENTIFY: C2RustUnnamed_10 = 2;
pub const FP_VERIFY_CAPTURE: C2RustUnnamed_10 = 1;
pub const FP_VERIFY_PWR_BTN_SHIELD_ON: C2RustUnnamed_10 = 0;
pub const FP_ENROLL_PWR_BTN_SHIELD_OFF: C2RustUnnamed_9 = 9;
pub const FP_ENROLL_NUM_STATES: C2RustUnnamed_9 = 10;
pub const FP_ENROLL_COMMIT: C2RustUnnamed_9 = 8;
pub const FP_ENROLL_CHECK_DUPLICATE: C2RustUnnamed_9 = 7;
pub const FP_ENROLL_CAPTURE: C2RustUnnamed_9 = 4;
pub const FP_ENROLL_WAIT_FINGER_UP: C2RustUnnamed_9 = 6;
pub const FP_ENROLL_UPDATE: C2RustUnnamed_9 = 5;
pub const FP_ENROLL_CREATE: C2RustUnnamed_9 = 3;
pub const FP_ENROLL_PWR_BTN_SHIELD_ON: C2RustUnnamed_9 = 0;
pub const FP_ENROLL_ENUM: C2RustUnnamed_9 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub const FP_INIT_NUM_STATES: C2RustUnnamed_8 = 4;
pub const FP_INIT_RESET_DEVICE: C2RustUnnamed_8 = 3;
pub const FP_INIT_TEMPLATE_LIST: C2RustUnnamed_8 = 2;
pub const FP_INIT_CONFIG: C2RustUnnamed_8 = 1;
pub const FP_INIT_VERSION: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_7 = libc::c_uint;
pub type C2RustUnnamed_8 = libc::c_uint;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const FP_ENROLL_IDENTIFY: C2RustUnnamed_9 = 2;
pub type C2RustUnnamed_10 = libc::c_uint;
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
unsafe extern "C" fn FPI_DEVICE_GOODIXMOC(mut ptr: gpointer) -> *mut FpiDeviceGoodixMoc {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_goodixmoc_get_type(),
    ) as *mut libc::c_void as *mut FpiDeviceGoodixMoc;
}
static mut fpi_device_goodixmoc_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
#[inline(never)]
unsafe extern "C" fn fpi_device_goodixmoc_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_device_get_type(),
        g_intern_static_string(
            b"FpiDeviceGoodixMoc\0" as *const u8 as *const libc::c_char,
        ),
        ::core::mem::size_of::<FpiDeviceGoodixMocClass>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_goodixmoc_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpiDeviceGoodixMoc>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpiDeviceGoodixMoc) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_goodixmoc_init
                        as unsafe extern "C" fn(*mut FpiDeviceGoodixMoc) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn fpi_device_goodixmoc_class_intern_init(mut klass: gpointer) {
    fpi_device_goodixmoc_parent_class = g_type_class_peek_parent(klass);
    if FpiDeviceGoodixMoc_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &mut FpiDeviceGoodixMoc_private_offset,
        );
    }
    fpi_device_goodixmoc_class_init(klass as *mut FpiDeviceGoodixMocClass);
}
static mut FpiDeviceGoodixMoc_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn fpi_device_goodixmoc_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_goodixmoc_get_type_once();
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
unsafe extern "C" fn fp_print_from_template(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut template: *mut template_format_t,
) -> *mut FpPrint {
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    let mut data: *mut GVariant = 0 as *mut GVariant;
    let mut tid: *mut GVariant = 0 as *mut GVariant;
    let mut uid: *mut GVariant = 0 as *mut GVariant;
    let mut userid: *mut gchar = 0 as *mut gchar;
    userid = g_strndup(
        ((*template).payload.data).as_mut_ptr() as *mut gchar,
        (*template).payload.size as gsize,
    );
    print = fp_print_new(FP_DEVICE(self_0 as gpointer));
    tid = g_variant_new_fixed_array(
        b"y\0" as *const u8 as *const libc::c_char as *const GVariantType,
        ((*template).tid).as_mut_ptr() as gconstpointer,
        32 as libc::c_int as gsize,
        1 as libc::c_int as gsize,
    );
    uid = g_variant_new_fixed_array(
        b"y\0" as *const u8 as *const libc::c_char as *const GVariantType,
        ((*template).payload.data).as_mut_ptr() as gconstpointer,
        (*template).payload.size as gsize,
        1 as libc::c_int as gsize,
    );
    data = g_variant_new(
        b"(y@ay@ay)\0" as *const u8 as *const libc::c_char,
        (*template).finger_index as libc::c_int,
        tid,
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
        userid,
        0 as *mut libc::c_void,
    );
    fpi_print_fill_from_user_id(print, userid);
    return print;
}
unsafe extern "C" fn fp_cmd_receive_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(device as gpointer);
    let mut data: *mut CommandData = user_data as *mut CommandData;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut ssm_state: libc::c_int = 0 as libc::c_int;
    let mut cmd_reponse: gxfp_cmd_response_t = {
        let mut init = _fp_cmd_response {
            result: 0 as libc::c_int as uint8_t,
            c2rust_unnamed: C2RustUnnamed_5 {
                parse_msg: gxfp_parse_msg_t {
                    ack_cmd: 0,
                    has_no_config: false,
                },
            },
        };
        init
    };
    let mut header: pack_header = pack_header {
        cmd0: 0,
        cmd1: 0,
        packagenum: 0,
        reserved: 0,
        len: 0,
        crc8: 0,
        rev_crc8: 0,
    };
    let mut crc32_calc: guint32 = 0 as libc::c_int as guint32;
    let mut cmd: guint16 = 0 as libc::c_int as guint16;
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
    ret = gx_proto_parse_header(
        (*transfer).buffer,
        (*transfer).actual_length as uint32_t,
        &mut header,
    );
    if ret != 0 as libc::c_int {
        fpi_ssm_mark_failed(
            (*transfer).ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Corrupted message header received\0" as *const u8
                    as *const libc::c_char,
            ),
        );
        return;
    }
    gx_proto_crc32_calc(
        (*transfer).buffer,
        (8 as libc::c_int + header.len as libc::c_int) as uint32_t,
        &mut crc32_calc as *mut guint32 as *mut uint8_t,
    );
    if crc32_calc
        != *(((*transfer).buffer)
            .offset(8 as libc::c_int as isize)
            .offset(header.len as libc::c_int as isize) as *mut uint32_t)
    {
        fpi_ssm_mark_failed(
            (*transfer).ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Package crc check failed\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    cmd = ((header.cmd0 as libc::c_int) << 8 as libc::c_int | header.cmd1 as libc::c_int)
        as uint16_t;
    ret = gx_proto_parse_body(
        cmd,
        &mut *((*transfer).buffer).offset(8 as libc::c_int as isize),
        header.len,
        &mut cmd_reponse,
    );
    if ret != 0 as libc::c_int {
        fpi_ssm_mark_failed(
            (*transfer).ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Corrupted message received\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    if header.cmd0 as libc::c_int == 0xaa as libc::c_int {
        if (*data).cmd as libc::c_int
            != cmd_reponse.c2rust_unnamed.parse_msg.ack_cmd as libc::c_int
        {
            fpi_ssm_mark_failed(
                (*transfer).ssm,
                fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_PROTO,
                    b"Unexpected response, got 0x%x\0" as *const u8
                        as *const libc::c_char,
                    cmd_reponse.c2rust_unnamed.parse_msg.ack_cmd as libc::c_int,
                ),
            );
            return;
        }
        fpi_ssm_next_state((*transfer).ssm);
        return;
    }
    if (*data).cmd as libc::c_int != header.cmd0 as libc::c_int {
        fpi_ssm_mark_failed(
            (*transfer).ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Unexpected cmd, got 0x%x\0" as *const u8 as *const libc::c_char,
                header.cmd0 as libc::c_int,
            ),
        );
        return;
    }
    if ((*data).callback).is_some() {
        ((*data).callback)
            .expect(
                "non-null function pointer",
            )(self_0, &mut cmd_reponse, 0 as *mut GError);
    }
    fpi_ssm_mark_completed((*transfer).ssm);
}
unsafe extern "C" fn fp_cmd_run_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(dev as gpointer);
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
            } else {
                fpi_ssm_next_state(ssm);
            }
        }
        1 => {
            transfer = fpi_usb_transfer_new(dev);
            (*transfer).ssm = ssm;
            fpi_usb_transfer_fill_bulk(
                transfer,
                (3 as libc::c_int | 0x80 as libc::c_int) as guint8,
                2048 as libc::c_int as gsize,
            );
            fpi_usb_transfer_submit(
                transfer,
                2000 as libc::c_int as guint,
                0 as *mut GCancellable,
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
        2 => {
            transfer = fpi_usb_transfer_new(dev);
            (*transfer).ssm = ssm;
            fpi_usb_transfer_fill_bulk(
                transfer,
                (3 as libc::c_int | 0x80 as libc::c_int) as guint8,
                2048 as libc::c_int as gsize,
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
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(dev as gpointer);
    let mut data: *mut CommandData = fpi_ssm_get_data(ssm) as *mut CommandData;
    (*self_0).cmd_ssm = 0 as *mut FpiSsm;
    if !error.is_null() {
        if ((*data).callback).is_some() {
            ((*data).callback)
                .expect(
                    "non-null function pointer",
                )(self_0, 0 as *mut gxfp_cmd_response_t, error);
        } else {
            g_error_free(error);
        }
    }
}
unsafe extern "C" fn alloc_cmd_transfer(
    mut dev: *mut FpDevice,
    mut cmd0: guint8,
    mut cmd1: guint8,
    mut data: *const guint8,
    mut data_len: guint16,
) -> *mut FpiUsbTransfer {
    let mut ret: gint = -(1 as libc::c_int);
    let mut transfer: FpiUsbTransfer_autoptr = 0 as FpiUsbTransfer_autoptr;
    let mut total_len: guint32 = (data_len as libc::c_int + 8 as libc::c_int
        + 4 as libc::c_int) as guint32;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !data.is_null() || data_len as libc::c_int == 0 as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"alloc_cmd_transfer\0"))
                .as_ptr(),
            b"data || data_len == 0\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FpiUsbTransfer;
    }
    transfer = fpi_usb_transfer_new(dev);
    fpi_usb_transfer_fill_bulk(
        transfer,
        (1 as libc::c_int | 0 as libc::c_int) as guint8,
        total_len as gsize,
    );
    ret = gx_proto_build_package(
        (*transfer).buffer,
        &mut total_len,
        ((cmd0 as libc::c_int) << 8 as libc::c_int | cmd1 as libc::c_int) as uint16_t,
        data,
        data_len as uint32_t,
    );
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if ret == 0 as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"alloc_cmd_transfer\0"))
                .as_ptr(),
            b"ret == 0\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FpiUsbTransfer;
    }
    return (if 0 as libc::c_int != 0 {
        transfer as *mut libc::c_void
    } else {
        g_steal_pointer(&mut transfer as *mut FpiUsbTransfer_autoptr as gpointer)
    }) as *mut FpiUsbTransfer;
}
unsafe extern "C" fn fp_cmd_ssm_done_data_free(mut data: *mut CommandData) {
    g_free(data as gpointer);
}
unsafe extern "C" fn goodix_sensor_cmd(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut cmd0: guint8,
    mut cmd1: guint8,
    mut bwait_data_delay: gboolean,
    mut payload: *const guint8,
    mut payload_len: gssize,
    mut callback: SynCmdMsgCallback,
) {
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
    transfer = alloc_cmd_transfer(
        FP_DEVICE(self_0 as gpointer),
        cmd0,
        cmd1,
        payload,
        payload_len as guint16,
    );
    (*data).cmd = cmd0;
    (*data).callback = callback;
    (*self_0)
        .cmd_transfer = (if 0 as libc::c_int != 0 {
        transfer as *mut libc::c_void
    } else {
        g_steal_pointer(&mut transfer as *mut FpiUsbTransfer_autoptr as gpointer)
    }) as *mut FpiUsbTransfer;
    (*self_0).cmd_cancelable = bwait_data_delay;
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
unsafe extern "C" fn fp_pwr_btn_shield_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if (*resp).result as libc::c_int >= 0x80 as libc::c_int {
        g_log(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Setting power button shield failed, result: 0x%x\0" as *const u8
                as *const libc::c_char,
            (*resp).result as libc::c_int,
        );
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_retry_new(FP_DEVICE_RETRY_GENERAL),
        );
        return;
    }
    if (*resp).c2rust_unnamed.power_button_shield_resp.resp_cmd1 as libc::c_int
        == 0x1 as libc::c_int
    {
        (*self_0).is_power_button_shield_on = 1 as libc::c_int;
    } else {
        (*self_0).is_power_button_shield_on = 0 as libc::c_int;
    }
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn fp_verify_capture_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if (*resp).result as libc::c_int >= 0x80 as libc::c_int {
        g_log(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Capture sample failed, result: 0x%x\0" as *const u8 as *const libc::c_char,
            (*resp).result as libc::c_int,
        );
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_retry_new(FP_DEVICE_RETRY_GENERAL),
        );
        return;
    }
    fpi_device_report_finger_status_changes(
        FP_DEVICE(self_0 as gpointer),
        FP_FINGER_STATUS_PRESENT,
        FP_FINGER_STATUS_NONE,
    );
    if (*resp).c2rust_unnamed.capture_data_resp.img_quality as libc::c_int
        == 0 as libc::c_int
    {
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_retry_new(FP_DEVICE_RETRY_REMOVE_FINGER),
        );
        return;
    } else {
        if ((*resp).c2rust_unnamed.capture_data_resp.img_coverage as libc::c_int)
            < 35 as libc::c_int
        {
            fpi_ssm_mark_failed(
                (*self_0).task_ssm,
                fpi_device_retry_new(FP_DEVICE_RETRY_CENTER_FINGER),
            );
            return;
        }
    }
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn fp_verify_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    let mut new_scan: *mut FpPrint = 0 as *mut FpPrint;
    let mut matching: *mut FpPrint = 0 as *mut FpPrint;
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if (*resp).c2rust_unnamed.verify.match_0 {
        new_scan = fp_print_from_template(
            self_0,
            &mut (*resp).c2rust_unnamed.verify.template,
        );
        if fpi_device_get_current_action(device) as libc::c_uint
            == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
        {
            fpi_device_get_verify_data(device, &mut matching);
            if fp_print_equal(matching, new_scan) == 0 {
                matching = 0 as *mut FpPrint;
            }
        } else {
            let mut templates: *mut GPtrArray = 0 as *mut GPtrArray;
            fpi_device_get_identify_data(device, &mut templates);
            let mut i: gint = 0 as libc::c_int;
            while (i as libc::c_uint) < (*templates).len {
                if fp_print_equal(
                    *((*templates).pdata).offset(i as isize) as *mut FpPrint,
                    new_scan,
                ) != 0
                {
                    matching = *((*templates).pdata).offset(i as isize) as *mut FpPrint;
                    break;
                } else {
                    i += 1;
                }
            }
        }
    }
    if fpi_device_get_current_action(device) as libc::c_uint
        == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
    {
        fpi_device_verify_report(
            device,
            (if !matching.is_null() {
                FPI_MATCH_SUCCESS as libc::c_int
            } else {
                FPI_MATCH_FAIL as libc::c_int
            }) as FpiMatchResult,
            new_scan,
            error,
        );
    } else {
        fpi_device_identify_report(device, matching, new_scan, error);
    }
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn fp_verify_sm_run_state(
    mut ssm: *mut FpiSsm,
    mut device: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(device as gpointer);
    let mut param: [guint8; 3] = [0 as libc::c_int as guint8, 0, 0];
    let mut nonce: [guint8; 32] = [
        0 as libc::c_int as guint8,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    param[0 as libc::c_int as usize] = 0x1 as libc::c_int as guint8;
    param[1 as libc::c_int
        as usize] = (*(*self_0).sensorcfg).config[10 as libc::c_int as usize];
    param[2 as libc::c_int
        as usize] = (*(*self_0).sensorcfg).config[11 as libc::c_int as usize];
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            goodix_sensor_cmd(
                self_0,
                0xe0 as libc::c_int as guint8,
                0x1 as libc::c_int as guint8,
                0 as libc::c_int,
                0 as *const guint8,
                0 as libc::c_int as gssize,
                Some(
                    fp_pwr_btn_shield_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        1 => {
            fpi_device_report_finger_status_changes(
                device,
                FP_FINGER_STATUS_NEEDED,
                FP_FINGER_STATUS_NONE,
            );
            goodix_sensor_cmd(
                self_0,
                0xa2 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
                1 as libc::c_int,
                &mut param as *mut [guint8; 3] as *const guint8,
                (::core::mem::size_of::<[guint8; 3]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<guint8>() as libc::c_ulong)
                    as gssize,
                Some(
                    fp_verify_capture_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        2 => {
            goodix_sensor_cmd(
                self_0,
                0xa5 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
                0 as libc::c_int,
                nonce.as_mut_ptr() as *const guint8,
                32 as libc::c_int as gssize,
                Some(
                    fp_verify_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        3 => {
            goodix_sensor_cmd(
                self_0,
                0xe0 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
                0 as libc::c_int,
                0 as *const guint8,
                0 as libc::c_int as gssize,
                Some(
                    fp_pwr_btn_shield_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn fp_verify_ssm_done(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(dev as gpointer);
    g_log(
        b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Verify complete!\0" as *const u8 as *const libc::c_char,
    );
    if !error.is_null() && (*error).domain == fp_device_retry_quark() {
        if fpi_device_get_current_action(dev) as libc::c_uint
            == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
        {
            fpi_device_verify_report(
                dev,
                FPI_MATCH_ERROR,
                0 as *mut FpPrint,
                (if 0 as libc::c_int != 0 {
                    error as *mut libc::c_void
                } else {
                    g_steal_pointer(&mut error as *mut *mut GError as gpointer)
                }) as *mut GError,
            );
        } else {
            fpi_device_identify_report(
                dev,
                0 as *mut FpPrint,
                0 as *mut FpPrint,
                (if 0 as libc::c_int != 0 {
                    error as *mut libc::c_void
                } else {
                    g_steal_pointer(&mut error as *mut *mut GError as gpointer)
                }) as *mut GError,
            );
        }
    }
    if fpi_device_get_current_action(dev) as libc::c_uint
        == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
    {
        fpi_device_verify_complete(dev, error);
    } else {
        fpi_device_identify_complete(dev, error);
    }
    (*self_0).task_ssm = 0 as *mut FpiSsm;
}
unsafe extern "C" fn encode_finger_id(
    mut tid: *const guint8,
    mut tid_len: guint16,
    mut uid: *const guint8,
    mut uid_len: guint16,
    mut fid: *mut *mut guint8,
    mut fid_len: *mut guint16,
) -> gboolean {
    let mut buffer: *mut guint8 = 0 as *mut guint8;
    let mut offset: guint16 = 0 as libc::c_int as guint16;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !tid.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"encode_finger_id\0"))
                .as_ptr(),
            b"tid != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !uid.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"encode_finger_id\0"))
                .as_ptr(),
            b"uid != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !fid.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"encode_finger_id\0"))
                .as_ptr(),
            b"fid != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !fid_len.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"encode_finger_id\0"))
                .as_ptr(),
            b"fid_len != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    *fid_len = (70 as libc::c_int + uid_len as libc::c_int) as guint16;
    *fid = g_malloc0((*fid_len as libc::c_int + 2 as libc::c_int) as gsize)
        as *mut guint8;
    buffer = *fid;
    offset = 0 as libc::c_int as guint16;
    let fresh0 = offset;
    offset = offset.wrapping_add(1);
    *buffer.offset(fresh0 as isize) = *fid_len as uint8_t;
    let fresh1 = offset;
    offset = offset.wrapping_add(1);
    *buffer
        .offset(
            fresh1 as isize,
        ) = (*fid_len as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as uint8_t;
    let fresh2 = offset;
    offset = offset.wrapping_add(1);
    *buffer.offset(fresh2 as isize) = 67 as libc::c_int as guint8;
    let fresh3 = offset;
    offset = offset.wrapping_add(1);
    *buffer.offset(fresh3 as isize) = 1 as libc::c_int as guint8;
    let fresh4 = offset;
    offset = offset.wrapping_add(1);
    *buffer.offset(fresh4 as isize) = 1 as libc::c_int as guint8;
    let fresh5 = offset;
    offset = offset.wrapping_add(1);
    *buffer.offset(fresh5 as isize) = 0 as libc::c_int as guint8;
    offset = (offset as libc::c_int + 32 as libc::c_int) as guint16;
    memcpy(
        &mut *buffer.offset(offset as isize) as *mut guint8 as *mut libc::c_void,
        tid as *const libc::c_void,
        (if (tid_len as libc::c_int) < 32 as libc::c_int {
            tid_len as libc::c_int
        } else {
            32 as libc::c_int
        }) as libc::c_ulong,
    );
    offset = (offset as libc::c_int + 32 as libc::c_int) as guint16;
    let fresh6 = offset;
    offset = offset.wrapping_add(1);
    *buffer.offset(fresh6 as isize) = uid_len as guint8;
    memcpy(
        &mut *buffer.offset(offset as isize) as *mut guint8 as *mut libc::c_void,
        uid as *const libc::c_void,
        uid_len as libc::c_ulong,
    );
    offset = (offset as libc::c_int + uid_len as guint8 as libc::c_int) as guint16;
    let fresh7 = offset;
    offset = offset.wrapping_add(1);
    *buffer.offset(fresh7 as isize) = 0 as libc::c_int as guint8;
    if offset as libc::c_int != *fid_len as libc::c_int + 2 as libc::c_int {
        memset(buffer as *mut libc::c_void, 0 as libc::c_int, *fid_len as libc::c_ulong);
        *fid_len = 0 as libc::c_int as guint16;
        g_log(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"offset != fid_len, %d != %d\0" as *const u8 as *const libc::c_char,
            offset as libc::c_int,
            *fid_len as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    *fid_len = (*fid_len as libc::c_int + 2 as libc::c_int) as guint16;
    return (0 as libc::c_int == 0) as libc::c_int;
}
unsafe extern "C" fn fp_enroll_enum_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if (*resp).result as libc::c_int != 0 as libc::c_int {
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_GENERAL,
                b"Failed to enumerate fingers, result: 0x%x\0" as *const u8
                    as *const libc::c_char,
                (*resp).result as libc::c_int,
            ),
        );
        return;
    }
    if (*resp).c2rust_unnamed.finger_list_resp.finger_num as libc::c_int
        >= (*self_0).max_stored_prints
    {
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_error_new(FP_DEVICE_ERROR_DATA_FULL),
        );
        return;
    }
    fpi_ssm_jump_to_state((*self_0).task_ssm, FP_ENROLL_CAPTURE as libc::c_int);
}
unsafe extern "C" fn fp_enroll_init_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    memcpy(
        ((*self_0).template_id).as_mut_ptr() as *mut libc::c_void,
        ((*resp).c2rust_unnamed.enroll_init.tid).as_mut_ptr() as *const libc::c_void,
        32 as libc::c_int as libc::c_ulong,
    );
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn fp_enroll_capture_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if (*resp).result as libc::c_int >= 0x80 as libc::c_int {
        g_log(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Capture sample failed, result: 0x%x\0" as *const u8 as *const libc::c_char,
            (*resp).result as libc::c_int,
        );
        fpi_device_enroll_progress(
            FP_DEVICE(self_0 as gpointer),
            (*self_0).enroll_stage,
            0 as *mut FpPrint,
            fpi_device_retry_new(FP_DEVICE_RETRY_GENERAL),
        );
        fpi_ssm_jump_to_state((*self_0).task_ssm, FP_ENROLL_CAPTURE as libc::c_int);
        return;
    }
    fpi_device_report_finger_status_changes(
        FP_DEVICE(self_0 as gpointer),
        FP_FINGER_STATUS_PRESENT,
        FP_FINGER_STATUS_NONE,
    );
    if ((*resp).c2rust_unnamed.capture_data_resp.img_quality as libc::c_int)
        < (*(*self_0).sensorcfg).config[4 as libc::c_int as usize] as libc::c_int
        || ((*resp).c2rust_unnamed.capture_data_resp.img_coverage as libc::c_int)
            < (*(*self_0).sensorcfg).config[5 as libc::c_int as usize] as libc::c_int
    {
        g_log(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Capture sample poor quality(%d): %d or coverage(%d): %d\0" as *const u8
                as *const libc::c_char,
            (*(*self_0).sensorcfg).config[4 as libc::c_int as usize] as libc::c_int,
            (*resp).c2rust_unnamed.capture_data_resp.img_quality as libc::c_int,
            (*(*self_0).sensorcfg).config[5 as libc::c_int as usize] as libc::c_int,
            (*resp).c2rust_unnamed.capture_data_resp.img_coverage as libc::c_int,
        );
        fpi_device_enroll_progress(
            FP_DEVICE(self_0 as gpointer),
            (*self_0).enroll_stage,
            0 as *mut FpPrint,
            fpi_device_retry_new(FP_DEVICE_RETRY_CENTER_FINGER),
        );
        fpi_ssm_jump_to_state((*self_0).task_ssm, FP_ENROLL_CAPTURE as libc::c_int);
        return;
    } else {
        fpi_ssm_next_state((*self_0).task_ssm);
    };
}
unsafe extern "C" fn fp_enroll_update_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if (*resp).c2rust_unnamed.enroll_update.img_preoverlay as libc::c_int
        > (*(*self_0).sensorcfg).config[3 as libc::c_int as usize] as libc::c_int
    {
        g_log(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Sample overlapping ratio is too High(%d): %d \0" as *const u8
                as *const libc::c_char,
            (*(*self_0).sensorcfg).config[3 as libc::c_int as usize] as libc::c_int,
            (*resp).c2rust_unnamed.enroll_update.img_preoverlay as libc::c_int,
        );
        fpi_device_enroll_progress(
            FP_DEVICE(self_0 as gpointer),
            (*self_0).enroll_stage,
            0 as *mut FpPrint,
            fpi_device_retry_new(FP_DEVICE_RETRY_REMOVE_FINGER),
        );
    } else if (*resp).c2rust_unnamed.enroll_update.rollback {
        fpi_device_enroll_progress(
            FP_DEVICE(self_0 as gpointer),
            (*self_0).enroll_stage,
            0 as *mut FpPrint,
            fpi_device_retry_new(FP_DEVICE_RETRY_GENERAL),
        );
    } else {
        (*self_0).enroll_stage += 1;
        fpi_device_enroll_progress(
            FP_DEVICE(self_0 as gpointer),
            (*self_0).enroll_stage,
            0 as *mut FpPrint,
            0 as *mut GError,
        );
    }
    if (*self_0).enroll_stage >= (*self_0).max_enroll_stage {
        fpi_ssm_jump_to_state(
            (*self_0).task_ssm,
            FP_ENROLL_CHECK_DUPLICATE as libc::c_int,
        );
        return;
    }
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn fp_enroll_check_duplicate_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if (*resp).c2rust_unnamed.check_duplicate_resp.duplicate {
        let mut print: FpPrint_autoptr = 0 as FpPrint_autoptr;
        print = g_object_ref_sink(
            fp_print_from_template(
                self_0,
                &mut (*resp).c2rust_unnamed.check_duplicate_resp.template,
            ) as gpointer,
        ) as *mut FpPrint;
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_DATA_DUPLICATE,
                b"Finger was already enrolled as '%s'\0" as *const u8
                    as *const libc::c_char,
                fp_print_get_description(print),
            ),
        );
        return;
    }
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn fp_enroll_commit_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if (*resp).result as libc::c_int >= 0x80 as libc::c_int {
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Commit template failed with errcode: 0x%x\0" as *const u8
                    as *const libc::c_char,
                (*resp).result as libc::c_int,
            ),
        );
        return;
    }
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn fp_finger_mode_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if (*resp).c2rust_unnamed.finger_status.status as libc::c_int == 0xc7 as libc::c_int
    {
        fpi_ssm_jump_to_state(
            (*self_0).task_ssm,
            FP_ENROLL_WAIT_FINGER_UP as libc::c_int,
        );
        return;
    } else {
        if (*resp).c2rust_unnamed.finger_status.status as libc::c_int != 0 as libc::c_int
        {
            fpi_ssm_mark_failed(
                (*self_0).task_ssm,
                fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_PROTO,
                    b"Switch finger mode failed\0" as *const u8 as *const libc::c_char,
                ),
            );
            return;
        }
    }
    fpi_device_report_finger_status_changes(
        FP_DEVICE(self_0 as gpointer),
        FP_FINGER_STATUS_NONE,
        FP_FINGER_STATUS_PRESENT,
    );
    if (*self_0).enroll_stage < (*self_0).max_enroll_stage {
        fpi_ssm_jump_to_state((*self_0).task_ssm, FP_ENROLL_CAPTURE as libc::c_int);
        return;
    }
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn fp_enroll_sm_run_state(
    mut ssm: *mut FpiSsm,
    mut device: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(device as gpointer);
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    let mut data: *mut GVariant = 0 as *mut GVariant;
    let mut uid: *mut GVariant = 0 as *mut GVariant;
    let mut tid: *mut GVariant = 0 as *mut GVariant;
    let mut finger: guint = 0;
    let mut user_id_len: guint16 = 0;
    let mut payload_len: guint16 = 0 as libc::c_int as guint16;
    let mut user_id: *mut gchar = 0 as *mut gchar;
    let mut payload: *mut guint8 = 0 as *mut guint8;
    let mut dummy: [guint8; 3] = [0 as libc::c_int as guint8, 0, 0];
    dummy[1 as libc::c_int
        as usize] = (*(*self_0).sensorcfg).config[4 as libc::c_int as usize];
    dummy[2 as libc::c_int
        as usize] = (*(*self_0).sensorcfg).config[5 as libc::c_int as usize];
    match fpi_ssm_get_cur_state(ssm) {
        1 => {
            goodix_sensor_cmd(
                self_0,
                0xa6 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
                0 as libc::c_int,
                &mut dummy as *mut [guint8; 3] as *const guint8,
                1 as libc::c_int as gssize,
                Some(
                    fp_enroll_enum_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        0 => {
            goodix_sensor_cmd(
                self_0,
                0xe0 as libc::c_int as guint8,
                0x1 as libc::c_int as guint8,
                0 as libc::c_int,
                0 as *const guint8,
                0 as libc::c_int as gssize,
                Some(
                    fp_pwr_btn_shield_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        3 => {
            goodix_sensor_cmd(
                self_0,
                0xa1 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
                0 as libc::c_int,
                &mut dummy as *mut [guint8; 3] as *const guint8,
                1 as libc::c_int as gssize,
                Some(
                    fp_enroll_init_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        4 => {
            fpi_device_report_finger_status_changes(
                device,
                FP_FINGER_STATUS_NEEDED,
                FP_FINGER_STATUS_NONE,
            );
            goodix_sensor_cmd(
                self_0,
                0xa2 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
                1 as libc::c_int,
                &mut dummy as *mut [guint8; 3] as *const guint8,
                3 as libc::c_int as gssize,
                Some(
                    fp_enroll_capture_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        5 => {
            dummy[0 as libc::c_int as usize] = 1 as libc::c_int as guint8;
            dummy[1 as libc::c_int
                as usize] = (*(*self_0).sensorcfg).config[2 as libc::c_int as usize];
            dummy[2 as libc::c_int
                as usize] = (*(*self_0).sensorcfg).config[3 as libc::c_int as usize];
            goodix_sensor_cmd(
                self_0,
                0xa0 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
                0 as libc::c_int,
                &mut dummy as *mut [guint8; 3] as *const guint8,
                3 as libc::c_int as gssize,
                Some(
                    fp_enroll_update_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        6 => {
            dummy[0 as libc::c_int as usize] = 0 as libc::c_int as guint8;
            goodix_sensor_cmd(
                self_0,
                0xb0 as libc::c_int as guint8,
                0x2 as libc::c_int as guint8,
                1 as libc::c_int,
                &mut dummy as *mut [guint8; 3] as *const guint8,
                1 as libc::c_int as gssize,
                Some(
                    fp_finger_mode_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        7 => {
            goodix_sensor_cmd(
                self_0,
                0xa3 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
                0 as libc::c_int,
                &mut dummy as *mut [guint8; 3] as *const guint8,
                3 as libc::c_int as gssize,
                Some(
                    fp_enroll_check_duplicate_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        8 => {
            fpi_device_get_enroll_data(device, &mut print);
            user_id = fpi_print_generate_user_id(print);
            user_id_len = strlen(user_id) as guint16;
            user_id_len = (if (100 as libc::c_int) < user_id_len as libc::c_int {
                100 as libc::c_int
            } else {
                user_id_len as libc::c_int
            }) as guint16;
            finger = 1 as libc::c_int as guint;
            if g_strcmp0(
                g_getenv(b"FP_DEVICE_EMULATION\0" as *const u8 as *const libc::c_char),
                b"1\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                memset(
                    ((*self_0).template_id).as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    32 as libc::c_int as libc::c_ulong,
                );
            }
            uid = g_variant_new_fixed_array(
                b"y\0" as *const u8 as *const libc::c_char as *const GVariantType,
                user_id as gconstpointer,
                user_id_len as gsize,
                1 as libc::c_int as gsize,
            );
            tid = g_variant_new_fixed_array(
                b"y\0" as *const u8 as *const libc::c_char as *const GVariantType,
                ((*self_0).template_id).as_mut_ptr() as gconstpointer,
                32 as libc::c_int as gsize,
                1 as libc::c_int as gsize,
            );
            data = g_variant_new(
                b"(y@ay@ay)\0" as *const u8 as *const libc::c_char,
                finger,
                tid,
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
            g_log(
                b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"user_id: %s, user_id_len: %d, finger: %d\0" as *const u8
                    as *const libc::c_char,
                user_id,
                user_id_len as libc::c_int,
                finger,
            );
            if encode_finger_id(
                ((*self_0).template_id).as_mut_ptr(),
                32 as libc::c_int as guint16,
                user_id as *mut guint8,
                user_id_len,
                &mut payload,
                &mut payload_len,
            ) == 0
            {
                fpi_ssm_mark_failed(
                    ssm,
                    fpi_device_error_new_msg(
                        FP_DEVICE_ERROR_PROTO,
                        b"encode_finger_id failed\0" as *const u8 as *const libc::c_char,
                    ),
                );
                return;
            }
            goodix_sensor_cmd(
                self_0,
                0xa4 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
                0 as libc::c_int,
                payload as *const guint8,
                payload_len as gssize,
                Some(
                    fp_enroll_commit_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        9 => {
            goodix_sensor_cmd(
                self_0,
                0xe0 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
                0 as libc::c_int,
                0 as *const guint8,
                0 as libc::c_int as gssize,
                Some(
                    fp_pwr_btn_shield_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn fp_enroll_ssm_done(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(dev as gpointer);
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    if !error.is_null() {
        fpi_device_enroll_complete(dev, 0 as *mut FpPrint, error);
        return;
    }
    g_log(
        b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Enrollment complete!\0" as *const u8 as *const libc::c_char,
    );
    fpi_device_get_enroll_data(FP_DEVICE(self_0 as gpointer), &mut print);
    fpi_device_enroll_complete(
        FP_DEVICE(self_0 as gpointer),
        g_object_ref(print as gpointer) as *mut FpPrint,
        0 as *mut GError,
    );
    (*self_0).task_ssm = 0 as *mut FpiSsm;
}
unsafe extern "C" fn fp_init_version_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    let mut fw_type: *mut gchar = 0 as *mut gchar;
    let mut fw_version: *mut gchar = 0 as *mut gchar;
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    fw_type = g_strndup(
        ((*resp).c2rust_unnamed.version_info.fwtype).as_mut_ptr() as *const libc::c_char,
        ::core::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
    );
    g_log(
        b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Firmware type: %s\0" as *const u8 as *const libc::c_char,
        fw_type,
    );
    if g_strcmp0(fw_type, b"APP\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
    {
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_NOT_SUPPORTED,
                b"Please update firmware using fwupd\0" as *const u8
                    as *const libc::c_char,
            ),
        );
        return;
    }
    fw_version = g_strndup(
        ((*resp).c2rust_unnamed.version_info.fwversion).as_mut_ptr()
            as *const libc::c_char,
        ::core::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
    );
    g_log(
        b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Firmware version: %s\0" as *const u8 as *const libc::c_char,
        fw_version,
    );
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn fp_init_config_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    (*self_0)
        .max_stored_prints = (*resp).c2rust_unnamed.finger_config.max_stored_prints
        as gint;
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn fp_init_cb_reset_or_complete(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    if !error.is_null() {
        g_log(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Template storage appears to have been corrupted! Error was: %s\0"
                as *const u8 as *const libc::c_char,
            (*error).message,
        );
        g_log(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"A known reason for this to happen is a firmware bug triggered by another storage area being initialized.\0"
                as *const u8 as *const libc::c_char,
        );
        fpi_ssm_jump_to_state((*self_0).task_ssm, FP_INIT_RESET_DEVICE as libc::c_int);
    } else {
        fpi_ssm_mark_completed((*self_0).task_ssm);
    };
}
unsafe extern "C" fn fp_init_reset_device_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    if !error.is_null() {
        g_log(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Reset failed: %s\0" as *const u8 as *const libc::c_char,
            (*error).message,
        );
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if (*resp).result as libc::c_int >= 0x80 as libc::c_int
        && (*resp).result as libc::c_int != 0x9c as libc::c_int
    {
        g_log(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Reset failed, device reported: 0x%x\0" as *const u8 as *const libc::c_char,
            (*resp).result as libc::c_int,
        );
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_GENERAL,
                b"Failed clear storage, result: 0x%x\0" as *const u8
                    as *const libc::c_char,
                (*resp).result as libc::c_int,
            ),
        );
        return;
    }
    g_log(
        b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_WARNING,
        b"Reset completed\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_mark_completed((*self_0).task_ssm);
}
unsafe extern "C" fn fp_init_sm_run_state(
    mut ssm: *mut FpiSsm,
    mut device: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(device as gpointer);
    let mut dummy: guint8 = 0 as libc::c_int as guint8;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            goodix_sensor_cmd(
                self_0,
                0xd0 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
                0 as libc::c_int,
                &mut dummy,
                1 as libc::c_int as gssize,
                Some(
                    fp_init_version_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        1 => {
            goodix_sensor_cmd(
                self_0,
                0xc0 as libc::c_int as guint8,
                0x1 as libc::c_int as guint8,
                0 as libc::c_int,
                (*self_0).sensorcfg as *mut guint8,
                ::core::mem::size_of::<gxfp_sensor_cfg_t>() as libc::c_ulong as gssize,
                Some(
                    fp_init_config_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        2 => {
            goodix_sensor_cmd(
                self_0,
                0xa6 as libc::c_int as guint8,
                0 as libc::c_int as guint8,
                0 as libc::c_int,
                &mut dummy as *mut guint8 as *const guint8,
                1 as libc::c_int as gssize,
                Some(
                    fp_init_cb_reset_or_complete
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        3 => {
            g_log(
                b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Resetting device storage, you will need to enroll all prints again!\0"
                    as *const u8 as *const libc::c_char,
            );
            goodix_sensor_cmd(
                self_0,
                0xa7 as libc::c_int as guint8,
                0x1 as libc::c_int as guint8,
                0 as libc::c_int,
                0 as *const guint8,
                0 as libc::c_int as gssize,
                Some(
                    fp_init_reset_device_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceGoodixMoc,
                            *mut gxfp_cmd_response_t,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn fp_init_ssm_done(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(dev as gpointer);
    if !error.is_null() {
        fpi_device_open_complete(dev, error);
        return;
    }
    (*self_0).task_ssm = 0 as *mut FpiSsm;
    fpi_device_open_complete(dev, 0 as *mut GError);
}
unsafe extern "C" fn parse_print_data(
    mut data: *mut GVariant,
    mut finger: *mut guint8,
    mut tid: *mut *const guint8,
    mut tid_len: *mut gsize,
    mut user_id: *mut *const guint8,
    mut user_id_len: *mut gsize,
) -> gboolean {
    let mut user_id_var: GVariant_autoptr = 0 as GVariant_autoptr;
    let mut tid_var: GVariant_autoptr = 0 as GVariant_autoptr;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !data.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"parse_print_data\0"))
                .as_ptr(),
            b"data != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !finger.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"parse_print_data\0"))
                .as_ptr(),
            b"finger != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !tid.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"parse_print_data\0"))
                .as_ptr(),
            b"tid != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !tid_len.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"parse_print_data\0"))
                .as_ptr(),
            b"tid_len != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !user_id.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"parse_print_data\0"))
                .as_ptr(),
            b"user_id != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !user_id_len.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"parse_print_data\0"))
                .as_ptr(),
            b"user_id_len != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    *tid = 0 as *const guint8;
    *tid_len = 0 as libc::c_int as gsize;
    *user_id = 0 as *const guint8;
    *user_id_len = 0 as libc::c_int as gsize;
    if g_variant_check_format_string(
        data,
        b"(y@ay@ay)\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    g_variant_get(
        data,
        b"(y@ay@ay)\0" as *const u8 as *const libc::c_char,
        finger,
        &mut tid_var as *mut GVariant_autoptr,
        &mut user_id_var as *mut GVariant_autoptr,
    );
    *tid = g_variant_get_fixed_array(tid_var, tid_len, 1 as libc::c_int as gsize)
        as *const guint8;
    *user_id = g_variant_get_fixed_array(
        user_id_var,
        user_id_len,
        1 as libc::c_int as gsize,
    ) as *const guint8;
    if *user_id_len == 0 as libc::c_int as libc::c_ulong
        || *user_id_len > 100 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if *user_id_len <= 0 as libc::c_int as libc::c_ulong
        || **user_id.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
    {
        return 0 as libc::c_int;
    }
    if *tid_len != 32 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
unsafe extern "C" fn fp_template_delete_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    if !error.is_null() {
        fpi_device_delete_complete(device, error);
        return;
    }
    if (*resp).result as libc::c_int >= 0x80 as libc::c_int
        && (*resp).result as libc::c_int != 0x9c as libc::c_int
    {
        fpi_device_delete_complete(
            FP_DEVICE(self_0 as gpointer),
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_GENERAL,
                b"Failed delete enrolled users, result: 0x%x\0" as *const u8
                    as *const libc::c_char,
                (*resp).result as libc::c_int,
            ),
        );
        return;
    }
    g_log(
        b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Successfully deleted enrolled user\0" as *const u8 as *const libc::c_char,
    );
    fpi_device_delete_complete(device, 0 as *mut GError);
}
unsafe extern "C" fn fp_template_delete_all_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    if !error.is_null() {
        fpi_device_clear_storage_complete(device, error);
        return;
    }
    if (*resp).result as libc::c_int >= 0x80 as libc::c_int
        && (*resp).result as libc::c_int != 0x9c as libc::c_int
    {
        fpi_device_clear_storage_complete(
            FP_DEVICE(self_0 as gpointer),
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_GENERAL,
                b"Failed clear storage, result: 0x%x\0" as *const u8
                    as *const libc::c_char,
                (*resp).result as libc::c_int,
            ),
        );
        return;
    }
    g_log(
        b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Successfully cleared storage\0" as *const u8 as *const libc::c_char,
    );
    fpi_device_clear_storage_complete(device, 0 as *mut GError);
}
unsafe extern "C" fn fp_template_list_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    if !error.is_null() {
        fpi_device_list_complete(
            FP_DEVICE(self_0 as gpointer),
            0 as *mut GPtrArray,
            error,
        );
        return;
    }
    if (*resp).result as libc::c_int != 0 as libc::c_int {
        g_log(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Failed to query enrolled users: %d\0" as *const u8 as *const libc::c_char,
            (*resp).result as libc::c_int,
        );
        fpi_device_list_complete(
            FP_DEVICE(self_0 as gpointer),
            0 as *mut GPtrArray,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_GENERAL,
                b"Failed to query enrolled users, result: 0x%x\0" as *const u8
                    as *const libc::c_char,
                (*resp).result as libc::c_int,
            ),
        );
        return;
    }
    (*self_0)
        .list_result = g_ptr_array_new_with_free_func(
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    if (*resp).c2rust_unnamed.finger_list_resp.finger_num as libc::c_int
        == 0 as libc::c_int
    {
        g_log(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Database is empty\0" as *const u8 as *const libc::c_char,
        );
        fpi_device_list_complete(
            device,
            (if 0 as libc::c_int != 0 {
                (*self_0).list_result as *mut libc::c_void
            } else {
                g_steal_pointer(
                    &mut (*self_0).list_result as *mut *mut GPtrArray as gpointer,
                )
            }) as *mut GPtrArray,
            0 as *mut GError,
        );
        return;
    }
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < (*resp).c2rust_unnamed.finger_list_resp.finger_num as libc::c_int {
        let mut print: *mut FpPrint = 0 as *mut FpPrint;
        print = fp_print_from_template(
            self_0,
            &mut *((*resp).c2rust_unnamed.finger_list_resp.finger_list)
                .as_mut_ptr()
                .offset(n as isize),
        );
        g_ptr_array_add(
            (*self_0).list_result,
            g_object_ref_sink(print as gpointer) as *mut FpPrint as gpointer,
        );
        n += 1;
    }
    g_log(
        b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Query complete!\0" as *const u8 as *const libc::c_char,
    );
    fpi_device_list_complete(
        device,
        (if 0 as libc::c_int != 0 {
            (*self_0).list_result as *mut libc::c_void
        } else {
            g_steal_pointer(
                &mut (*self_0).list_result as *mut *mut GPtrArray as gpointer,
            )
        }) as *mut GPtrArray,
        0 as *mut GError,
    );
}
unsafe extern "C" fn gx_fp_probe(mut device: *mut FpDevice) {
    let mut current_block: u64;
    let mut usb_dev: *mut GUsbDevice = 0 as *mut GUsbDevice;
    let mut error: *mut GError = 0 as *mut GError;
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(device as gpointer);
    let mut serial: *mut gchar = 0 as *mut gchar;
    let mut productid: gint = 0 as libc::c_int;
    usb_dev = fpi_device_get_usb_device(device);
    if g_usb_device_open(usb_dev, &mut error) == 0 {
        fpi_device_probe_complete(device, 0 as *const gchar, 0 as *const gchar, error);
        return;
    }
    if !(g_usb_device_reset(usb_dev, &mut error) == 0) {
        if !(g_usb_device_claim_interface(
            usb_dev,
            0 as libc::c_int,
            G_USB_DEVICE_CLAIM_INTERFACE_NONE,
            &mut error,
        ) == 0)
        {
            if g_strcmp0(
                g_getenv(b"FP_DEVICE_EMULATION\0" as *const u8 as *const libc::c_char),
                b"1\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                serial = g_strdup(
                    b"emulated-device\0" as *const u8 as *const libc::c_char,
                );
                current_block = 12599329904712511516;
            } else {
                serial = g_usb_device_get_string_descriptor(
                    usb_dev,
                    g_usb_device_get_serial_number_index(usb_dev),
                    &mut error,
                );
                if !serial.is_null()
                    && g_str_has_suffix(
                        serial,
                        b"B0\0" as *const u8 as *const libc::c_char,
                    ) == 0
                {
                    g_log(
                        b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_WARNING,
                        b"Device with serial %s not supported\0" as *const u8
                            as *const libc::c_char,
                        serial,
                    );
                }
                if !error.is_null() {
                    g_usb_device_release_interface(
                        fpi_device_get_usb_device(FP_DEVICE(device as gpointer)),
                        0 as libc::c_int,
                        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
                        0 as *mut *mut GError,
                    );
                    current_block = 4854203436479462056;
                } else {
                    current_block = 12599329904712511516;
                }
            }
            match current_block {
                4854203436479462056 => {}
                _ => {
                    productid = g_usb_device_get_pid(usb_dev) as gint;
                    match productid {
                        25750 | 24738 | 24596 | 24724 | 24732 | 25372 | 25420 | 25476
                        | 25500 | 25516 | 25532 | 25548 | 27284 | 26010 => {
                            (*self_0).max_enroll_stage = 12 as libc::c_int;
                        }
                        _ => {
                            (*self_0).max_enroll_stage = 8 as libc::c_int;
                        }
                    }
                    fpi_device_set_nr_enroll_stages(device, (*self_0).max_enroll_stage);
                    g_usb_device_close(usb_dev, 0 as *mut *mut GError);
                    fpi_device_probe_complete(device, serial, 0 as *const gchar, error);
                    return;
                }
            }
        }
    }
    g_usb_device_close(usb_dev, 0 as *mut *mut GError);
    fpi_device_probe_complete(device, 0 as *const gchar, 0 as *const gchar, error);
}
unsafe extern "C" fn gx_fp_init(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(device as gpointer);
    let mut error: *mut GError = 0 as *mut GError;
    let mut ret: libc::c_int = 0 as libc::c_int;
    (*self_0).max_stored_prints = 20 as libc::c_int;
    (*self_0).is_power_button_shield_on = 0 as libc::c_int;
    (*self_0)
        .sensorcfg = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<gxfp_sensor_cfg_t>()
            as libc::c_ulong;
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
    }) as *mut gxfp_sensor_cfg_t;
    ret = gx_proto_init_sensor_config((*self_0).sensorcfg);
    if ret != 0 as libc::c_int {
        error = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_GENERAL,
            b"Init sensor failed\0" as *const u8 as *const libc::c_char,
        );
        fpi_device_open_complete(FP_DEVICE(self_0 as gpointer), error);
        return;
    }
    (*(*self_0).sensorcfg)
        .config[6 as libc::c_int as usize] = (*self_0).max_enroll_stage as uint8_t;
    if g_usb_device_reset(fpi_device_get_usb_device(device), &mut error) == 0 {
        fpi_device_open_complete(FP_DEVICE(self_0 as gpointer), error);
        return;
    }
    if g_usb_device_claim_interface(
        fpi_device_get_usb_device(device),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    ) == 0
    {
        fpi_device_open_complete(FP_DEVICE(self_0 as gpointer), error);
        return;
    }
    (*self_0)
        .task_ssm = fpi_ssm_new_full(
        device,
        Some(
            fp_init_sm_run_state
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        FP_INIT_NUM_STATES as libc::c_int,
        FP_INIT_NUM_STATES as libc::c_int,
        b"FP_INIT_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        (*self_0).task_ssm,
        Some(
            fp_init_ssm_done
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn gx_fp_release_interface(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut error: *mut GError,
) {
    let mut release_error: GError_autoptr = 0 as GError_autoptr;
    let mut _pp: C2RustUnnamed_6 = C2RustUnnamed_6 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*self_0).sensorcfg as *mut pgxfp_sensor_cfg_t as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    g_usb_device_release_interface(
        fpi_device_get_usb_device(FP_DEVICE(self_0 as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut release_error,
    );
    if error.is_null() {
        error = (if 0 as libc::c_int != 0 {
            release_error as *mut libc::c_void
        } else {
            g_steal_pointer(&mut release_error as *mut GError_autoptr as gpointer)
        }) as *mut GError;
    }
    fpi_device_close_complete(FP_DEVICE(self_0 as gpointer), error);
}
unsafe extern "C" fn gx_fp_exit_cb(
    mut self_0: *mut FpiDeviceGoodixMoc,
    mut resp: *mut gxfp_cmd_response_t,
    mut error: *mut GError,
) {
    if (*resp).result as libc::c_int >= 0x80 as libc::c_int {
        g_log(
            b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Setting power button shield failed, result: 0x%x\0" as *const u8
                as *const libc::c_char,
            (*resp).result as libc::c_int,
        );
    }
    (*self_0).is_power_button_shield_on = 0 as libc::c_int;
    gx_fp_release_interface(self_0, error);
}
unsafe extern "C" fn gx_fp_exit(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(device as gpointer);
    if (*self_0).is_power_button_shield_on != 0 {
        goodix_sensor_cmd(
            self_0,
            0xe0 as libc::c_int as guint8,
            0 as libc::c_int as guint8,
            0 as libc::c_int,
            0 as *const guint8,
            0 as libc::c_int as gssize,
            Some(
                gx_fp_exit_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceGoodixMoc,
                        *mut gxfp_cmd_response_t,
                        *mut GError,
                    ) -> (),
            ),
        );
    } else {
        gx_fp_release_interface(self_0, 0 as *mut GError);
    };
}
unsafe extern "C" fn gx_fp_verify_identify(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(device as gpointer);
    (*self_0)
        .task_ssm = fpi_ssm_new_full(
        device,
        Some(
            fp_verify_sm_run_state
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        FP_VERIFY_NUM_STATES as libc::c_int,
        FP_VERIFY_PWR_BTN_SHIELD_OFF as libc::c_int,
        b"verify\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        (*self_0).task_ssm,
        Some(
            fp_verify_ssm_done
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn gx_fp_enroll(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(device as gpointer);
    (*self_0).enroll_stage = 0 as libc::c_int;
    (*self_0)
        .task_ssm = fpi_ssm_new_full(
        device,
        Some(
            fp_enroll_sm_run_state
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        FP_ENROLL_NUM_STATES as libc::c_int,
        FP_ENROLL_PWR_BTN_SHIELD_OFF as libc::c_int,
        b"enroll\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        (*self_0).task_ssm,
        Some(
            fp_enroll_ssm_done
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn gx_fp_template_list(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(device as gpointer);
    let mut dummy: [guint8; 1] = [0 as libc::c_int as guint8];
    g_log_structured(
        b"libfprint-goodixmoc\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/goodixmoc/goodix.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"1530\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"gx_fp_template_list\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/goodixmoc/goodix.c:1530\0" as *const u8
            as *const libc::c_char,
    );
    goodix_sensor_cmd(
        self_0,
        0xa6 as libc::c_int as guint8,
        0 as libc::c_int as guint8,
        0 as libc::c_int,
        &mut dummy as *mut [guint8; 1] as *const guint8,
        1 as libc::c_int as gssize,
        Some(
            fp_template_list_cb
                as unsafe extern "C" fn(
                    *mut FpiDeviceGoodixMoc,
                    *mut gxfp_cmd_response_t,
                    *mut GError,
                ) -> (),
        ),
    );
}
unsafe extern "C" fn gx_fp_template_delete(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(device as gpointer);
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    let mut data: GVariant_autoptr = 0 as GVariant_autoptr;
    let mut finger: guint8 = 0;
    let mut user_id: *const guint8 = 0 as *const guint8;
    let mut user_id_len: gsize = 0 as libc::c_int as gsize;
    let mut tid: *const guint8 = 0 as *const guint8;
    let mut tid_len: gsize = 0 as libc::c_int as gsize;
    let mut payload_len: gsize = 0 as libc::c_int as gsize;
    let mut payload: *mut guint8 = 0 as *mut guint8;
    fpi_device_get_delete_data(device, &mut print);
    g_object_get(
        print as gpointer,
        b"fpi-data\0" as *const u8 as *const libc::c_char,
        &mut data as *mut GVariant_autoptr,
        0 as *mut libc::c_void,
    );
    if parse_print_data(
        data,
        &mut finger,
        &mut tid,
        &mut tid_len,
        &mut user_id,
        &mut user_id_len,
    ) == 0
    {
        fpi_device_delete_complete(
            device,
            fpi_device_error_new(FP_DEVICE_ERROR_DATA_INVALID),
        );
        return;
    }
    if encode_finger_id(
        tid,
        tid_len as guint16,
        user_id,
        user_id_len as guint16,
        &mut payload,
        &mut payload_len as *mut gsize as *mut guint16,
    ) == 0
    {
        fpi_device_delete_complete(
            device,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"encode_finger_id failed\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    goodix_sensor_cmd(
        self_0,
        0xa7 as libc::c_int as guint8,
        0 as libc::c_int as guint8,
        0 as libc::c_int,
        payload as *const guint8,
        payload_len as gssize,
        Some(
            fp_template_delete_cb
                as unsafe extern "C" fn(
                    *mut FpiDeviceGoodixMoc,
                    *mut gxfp_cmd_response_t,
                    *mut GError,
                ) -> (),
        ),
    );
}
unsafe extern "C" fn gx_fp_template_delete_all(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceGoodixMoc = FPI_DEVICE_GOODIXMOC(device as gpointer);
    goodix_sensor_cmd(
        self_0,
        0xa7 as libc::c_int as guint8,
        0x1 as libc::c_int as guint8,
        0 as libc::c_int,
        0 as *const guint8,
        0 as libc::c_int as gssize,
        Some(
            fp_template_delete_all_cb
                as unsafe extern "C" fn(
                    *mut FpiDeviceGoodixMoc,
                    *mut gxfp_cmd_response_t,
                    *mut GError,
                ) -> (),
        ),
    );
}
unsafe extern "C" fn fpi_device_goodixmoc_init(mut self_0: *mut FpiDeviceGoodixMoc) {}
static mut id_table: [FpIdEntry; 21] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5840 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x6014 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x6094 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x609c as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x60a2 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x631c as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x634c as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x6384 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x639c as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x63ac as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x63bc as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x63cc as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x6496 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x6584 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x658c as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x6592 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x6594 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x659a as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x659c as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
                        pid: 0x6a94 as libc::c_int as guint,
                        vid: 0x27c6 as libc::c_int as guint,
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
unsafe extern "C" fn fpi_device_goodixmoc_class_init(
    mut klass: *mut FpiDeviceGoodixMocClass,
) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    (*dev_class).id = b"goodixmoc\0" as *const u8 as *const libc::c_char;
    (*dev_class)
        .full_name = b"Goodix MOC Fingerprint Sensor\0" as *const u8
        as *const libc::c_char;
    (*dev_class).type_0 = FP_DEVICE_TYPE_USB;
    (*dev_class).scan_type = FP_SCAN_TYPE_PRESS;
    (*dev_class).id_table = id_table.as_ptr();
    (*dev_class).nr_enroll_stages = 8 as libc::c_int;
    (*dev_class).temp_hot_seconds = -(1 as libc::c_int);
    (*dev_class).open = Some(gx_fp_init as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class).close = Some(gx_fp_exit as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class).probe = Some(gx_fp_probe as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .enroll = Some(gx_fp_enroll as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .delete = Some(
        gx_fp_template_delete as unsafe extern "C" fn(*mut FpDevice) -> (),
    );
    (*dev_class)
        .clear_storage = Some(
        gx_fp_template_delete_all as unsafe extern "C" fn(*mut FpDevice) -> (),
    );
    (*dev_class)
        .list = Some(gx_fp_template_list as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .verify = Some(
        gx_fp_verify_identify as unsafe extern "C" fn(*mut FpDevice) -> (),
    );
    (*dev_class)
        .identify = Some(
        gx_fp_verify_identify as unsafe extern "C" fn(*mut FpDevice) -> (),
    );
    fpi_device_class_auto_initialize_features(dev_class);
    (*dev_class)
        .features = ::core::mem::transmute::<
        libc::c_uint,
        FpDeviceFeature,
    >(
        (*dev_class).features as libc::c_uint
            | FP_DEVICE_FEATURE_DUPLICATES_CHECK as libc::c_int as libc::c_uint,
    );
}
