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
    fn g_ptr_array_find_with_equal_func(
        haystack: *mut GPtrArray,
        needle: gconstpointer,
        equal_func: GEqualFunc,
        index_: *mut guint,
    ) -> gboolean;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_new_literal(
        domain: GQuark,
        code: gint,
        message: *const gchar,
    ) -> *mut GError;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_clear_error(err: *mut *mut GError);
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
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
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strcmp0(str1: *const libc::c_char, str2: *const libc::c_char) -> libc::c_int;
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
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
    fn fpi_usb_transfer_submit_sync(
        transfer: *mut FpiUsbTransfer,
        timeout_ms: guint,
        error: *mut *mut GError,
    ) -> gboolean;
    fn fpi_usb_transfer_submit(
        transfer: *mut FpiUsbTransfer,
        timeout_ms: guint,
        cancellable: *mut GCancellable,
        callback: FpiUsbTransferCallback,
        user_data: gpointer,
    );
    fn fpi_usb_transfer_fill_interrupt(
        transfer: *mut FpiUsbTransfer,
        endpoint: guint8,
        length: gsize,
    );
    fn fpi_usb_transfer_fill_bulk(
        transfer: *mut FpiUsbTransfer,
        endpoint: guint8,
        length: gsize,
    );
    fn fpi_usb_transfer_unref(self_0: *mut FpiUsbTransfer);
    fn fpi_usb_transfer_ref(self_0: *mut FpiUsbTransfer) -> *mut FpiUsbTransfer;
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
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_cancellable_new() -> *mut GCancellable;
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
    fn g_io_error_quark() -> GQuark;
    fn g_usb_device_get_serial_number_index(self_0: *mut GUsbDevice) -> guint8;
    fn g_usb_device_open(self_0: *mut GUsbDevice, error: *mut *mut GError) -> gboolean;
    fn g_usb_device_close(self_0: *mut GUsbDevice, error: *mut *mut GError) -> gboolean;
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
    fn fpi_device_critical_enter(device: *mut FpDevice);
    fn fpi_device_critical_leave(device: *mut FpDevice);
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
    fn fpi_device_clear_storage_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_suspend_complete(device: *mut FpDevice, error: *mut GError);
    fn fpi_device_resume_complete(device: *mut FpDevice, error: *mut GError);
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
    fn bmkt_compose_message(
        cmd: *mut uint8_t,
        cmd_len: *mut libc::c_int,
        msg_id: uint8_t,
        seq_num: uint8_t,
        payload_size: uint8_t,
        payload: *const uint8_t,
    ) -> libc::c_int;
    fn bmkt_parse_message_header(
        resp_buf: *mut uint8_t,
        resp_len: libc::c_int,
        msg_resp: *mut bmkt_msg_resp_t,
    ) -> libc::c_int;
    fn bmkt_parse_message_payload(
        msg_resp: *mut bmkt_msg_resp_t,
        resp: *mut bmkt_response_t,
    ) -> libc::c_int;
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
pub type FpPrint = _FpPrint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiByteReader {
    pub data: *const guint8,
    pub size: guint,
    pub byte: guint,
}
pub type FpiDeviceSynaptics = _FpiDeviceSynaptics;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceSynaptics {
    pub parent: FpDevice,
    pub cmd_seq_num: guint8,
    pub last_seq_num: guint8,
    pub cmd_ssm: *mut FpiSsm,
    pub cmd_pending_transfer: *mut FpiUsbTransfer,
    pub cmd_complete_on_removal: gboolean,
    pub cmd_suspended: gboolean,
    pub id_idx: guint8,
    pub mis_version: bmkt_sensor_version_t,
    pub action_starting: gboolean,
    pub interrupt_cancellable: *mut GCancellable,
    pub enroll_stage: gint,
    pub finger_on_sensor: gboolean,
    pub list_result: *mut GPtrArray,
    pub enroll_resp_data: syna_enroll_resp_data,
    pub state: syna_state_t,
}
pub type syna_state_t = syna_state;
pub type syna_state = libc::c_uint;
pub const SYNA_STATE_DELETE: syna_state = 7;
pub const SYNA_STATE_VERIFY_DELAY_RESULT: syna_state = 6;
pub const SYNA_STATE_VERIFY: syna_state = 5;
pub const SYNA_STATE_IDENTIFY_DELAY_RESULT: syna_state = 4;
pub const SYNA_STATE_IDENTIFY: syna_state = 3;
pub const SYNA_STATE_ENROLL: syna_state = 2;
pub const SYNA_STATE_IDLE: syna_state = 1;
pub const SYNA_STATE_UNINIT: syna_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct syna_enroll_resp_data {
    pub progress: libc::c_int,
}
pub type bmkt_sensor_version_t = bmkt_sensor_version;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_sensor_version {
    pub build_time: uint32_t,
    pub build_num: uint32_t,
    pub version_major: uint8_t,
    pub version_minor: uint8_t,
    pub target: uint8_t,
    pub product: uint8_t,
    pub silicon_rev: uint8_t,
    pub formal_release: uint8_t,
    pub platform: uint8_t,
    pub patch: uint8_t,
    pub serial_number: [uint8_t; 6],
    pub security: uint16_t,
    pub iface: uint8_t,
    pub device_type: uint8_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiDeviceSynapticsClass {
    pub parent_class: FpDeviceClass,
}
pub const SYNAPTICS_CMD_RESUME: C2RustUnnamed_9 = 6;
pub const SYNAPTICS_CMD_SUSPENDED: C2RustUnnamed_9 = 5;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub const SYNAPTICS_CMD_WAIT_INTERRUPT: C2RustUnnamed_9 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub type SynCmdMsgCallback = Option::<
    unsafe extern "C" fn(
        *mut FpiDeviceSynaptics,
        *mut bmkt_response_t,
        *mut GError,
    ) -> (),
>;
pub type bmkt_response_t = bmkt_response;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_response {
    pub response_id: libc::c_int,
    pub result: libc::c_int,
    pub complete: libc::c_int,
    pub response: bmkt_response_data_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union bmkt_response_data_t {
    pub init_resp: bmkt_init_resp_t,
    pub enroll_resp: bmkt_enroll_resp_t,
    pub verify_resp: bmkt_verify_resp_t,
    pub id_resp: bmkt_identify_resp_t,
    pub fps_mode_resp: bmkt_fps_mode_resp_t,
    pub get_version_resp: bmkt_get_version_resp_t,
    pub db_cap_resp: bmkt_get_db_capacity_resp_t,
    pub sec_level_resp: bmkt_set_sec_level_resp_t,
    pub del_all_users_resp: bmkt_del_all_users_resp_t,
    pub enroll_templates_resp: bmkt_enroll_templates_resp_t,
    pub del_user_resp: bmkt_del_user_resp_t,
    pub del_all_user_resp: bmkt_del_all_users_resp_t,
    pub enrolled_fingers_resp: bmkt_enrolled_fingers_resp_t,
}
pub type bmkt_enrolled_fingers_resp_t = bmkt_enrolled_fingers_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_enrolled_fingers_resp {
    pub fingers: [bmkt_enrolled_fingers_t; 10],
}
pub type bmkt_enrolled_fingers_t = bmkt_enrolled_fingers;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_enrolled_fingers {
    pub finger_id: uint8_t,
    pub template_status: uint8_t,
}
pub type bmkt_del_all_users_resp_t = bmkt_del_all_users_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_del_all_users_resp {
    pub progress: libc::c_int,
}
pub type bmkt_del_user_resp_t = bmkt_del_user_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_del_user_resp {
    pub progress: libc::c_int,
}
pub type bmkt_enroll_templates_resp_t = bmkt_enroll_templates_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_enroll_templates_resp {
    pub total_query_messages: uint8_t,
    pub query_sequence: uint8_t,
    pub templates: [bmkt_enroll_template_t; 15],
}
pub type bmkt_enroll_template_t = bmkt_enroll_template;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_enroll_template {
    pub user_id_len: uint8_t,
    pub template_status: uint8_t,
    pub finger_id: uint8_t,
    pub user_id: [uint8_t; 101],
}
pub type bmkt_set_sec_level_resp_t = bmkt_set_sec_level_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_set_sec_level_resp {
    pub sec_level: bmkt_sec_level_t,
}
pub type bmkt_sec_level_t = bmkt_sec_level;
pub type bmkt_sec_level = libc::c_uint;
pub const BMKT_SECURITY_LEVEL_HIGH: bmkt_sec_level = 96;
pub const BMKT_SECURITY_LEVEL_MEDIUM: bmkt_sec_level = 64;
pub const BMKT_SECURITY_LEVEL_LOW: bmkt_sec_level = 16;
pub type bmkt_get_db_capacity_resp_t = bmkt_get_db_capacity_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_get_db_capacity_resp {
    pub total: uint8_t,
    pub empty: uint8_t,
    pub bad_slots: uint8_t,
    pub corrupt_templates: uint8_t,
}
pub type bmkt_get_version_resp_t = bmkt_get_version_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_get_version_resp {
    pub part: [uint8_t; 10],
    pub year: uint8_t,
    pub week: uint8_t,
    pub patch: uint8_t,
    pub supplier_id: [uint8_t; 2],
}
pub type bmkt_fps_mode_resp_t = bmkt_fps_mode_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_fps_mode_resp {
    pub mode: uint8_t,
    pub level2_mode: uint8_t,
    pub cmd_id: uint8_t,
    pub finger_presence: uint8_t,
}
pub type bmkt_identify_resp_t = bmkt_auth_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_auth_resp {
    pub match_result: libc::c_double,
    pub finger_id: uint8_t,
    pub user_id: [uint8_t; 100],
}
pub type bmkt_verify_resp_t = bmkt_auth_resp;
pub type bmkt_enroll_resp_t = bmkt_enroll_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_enroll_resp {
    pub progress: libc::c_int,
    pub finger_id: uint8_t,
    pub user_id: [uint8_t; 100],
}
pub type bmkt_init_resp_t = bmkt_init_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_init_resp {
    pub finger_presence: uint8_t,
}
pub const SYNAPTICS_CMD_NUM_STATES: C2RustUnnamed_9 = 7;
pub const SYNAPTICS_CMD_SEND_PENDING: C2RustUnnamed_9 = 0;
pub const SYNAPTICS_CMD_RESTART: C2RustUnnamed_9 = 4;
pub const SYNAPTICS_CMD_SEND_ASYNC: C2RustUnnamed_9 = 3;
pub const SYNAPTICS_CMD_GET_RESP: C2RustUnnamed_9 = 1;
pub type bmkt_msg_resp_t = bmkt_msg_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_msg_resp {
    pub msg_id: uint8_t,
    pub seq_num: uint8_t,
    pub payload_len: uint8_t,
    pub payload: *mut uint8_t,
    pub result: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub type C2RustUnnamed_9 = libc::c_uint;
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
unsafe extern "C" fn _fpi_slow_read32_le(mut data: *const guint8) -> guint32 {
    return (*data.offset(3 as libc::c_int as isize) as guint32) << 24 as libc::c_int
        | (*data.offset(2 as libc::c_int as isize) as guint32) << 16 as libc::c_int
        | (*data.offset(1 as libc::c_int as isize) as guint32) << 8 as libc::c_int
        | (*data.offset(0 as libc::c_int as isize) as guint32) << 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_skip_unchecked(
    mut reader: *mut FpiByteReader,
    mut nbytes: guint,
) {
    (*reader)
        .byte = ((*reader).byte as libc::c_uint).wrapping_add(nbytes) as guint as guint;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint8_unchecked(
    mut reader: *const FpiByteReader,
) -> guint8 {
    let mut val: guint8 = ((*((*reader).data)
        .offset((*reader).byte as isize)
        .offset(0 as libc::c_int as isize) as libc::c_int) << 0 as libc::c_int)
        as guint8;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint8_unchecked(
    mut reader: *mut FpiByteReader,
) -> guint8 {
    let mut val: guint8 = fpi_byte_reader_peek_uint8_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((8 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint16_le_unchecked(
    mut reader: *const FpiByteReader,
) -> guint16 {
    let mut val: guint16 = _fpi_slow_read16_le(
        ((*reader).data).offset((*reader).byte as isize),
    );
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint16_le_unchecked(
    mut reader: *mut FpiByteReader,
) -> guint16 {
    let mut val: guint16 = fpi_byte_reader_peek_uint16_le_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((16 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint32_le_unchecked(
    mut reader: *const FpiByteReader,
) -> guint32 {
    let mut val: guint32 = _fpi_slow_read32_le(
        ((*reader).data).offset((*reader).byte as isize),
    );
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint32_le_unchecked(
    mut reader: *mut FpiByteReader,
) -> guint32 {
    let mut val: guint32 = fpi_byte_reader_peek_uint32_le_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((32 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_data_unchecked(
    mut reader: *const FpiByteReader,
) -> *const guint8 {
    return ((*reader).data).offset((*reader).byte as isize);
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_data_unchecked(
    mut reader: *mut FpiByteReader,
    mut size: guint,
) -> *const guint8 {
    let mut data: *const guint8 = 0 as *const guint8;
    data = fpi_byte_reader_peek_data_unchecked(reader);
    fpi_byte_reader_skip_unchecked(reader, size);
    return data;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_remaining_unchecked(
    mut reader: *const FpiByteReader,
) -> guint {
    return ((*reader).size).wrapping_sub((*reader).byte);
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint8_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint8,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"fpi_byte_reader_get_uint8_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"fpi_byte_reader_get_uint8_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (8 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_uint8_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint16_le_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint16,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fpi_byte_reader_get_uint16_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fpi_byte_reader_get_uint16_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (16 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_uint16_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint32_le_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fpi_byte_reader_get_uint32_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fpi_byte_reader_get_uint32_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (32 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_uint32_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_init_inline(
    mut reader: *mut FpiByteReader,
    mut data: *const guint8,
    mut size: guint,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fpi_byte_reader_init_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*reader).data = data;
    (*reader).size = size;
    (*reader).byte = 0 as libc::c_int as guint;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_data_inline(
    mut reader: *mut FpiByteReader,
    mut size: guint,
    mut val: *mut *const guint8,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"fpi_byte_reader_get_data_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"fpi_byte_reader_get_data_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if size > (*reader).size
            || fpi_byte_reader_get_remaining_unchecked(reader) < size
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_data_unchecked(reader, size);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn _fpi_slow_read16_be(mut data: *const guint8) -> guint16 {
    return ((*data.offset(0 as libc::c_int as isize) as guint16 as libc::c_int)
        << 8 as libc::c_int
        | (*data.offset(1 as libc::c_int as isize) as guint16 as libc::c_int)
            << 0 as libc::c_int) as guint16;
}
#[inline]
unsafe extern "C" fn _fpi_slow_read16_le(mut data: *const guint8) -> guint16 {
    return ((*data.offset(1 as libc::c_int as isize) as guint16 as libc::c_int)
        << 8 as libc::c_int
        | (*data.offset(0 as libc::c_int as isize) as guint16 as libc::c_int)
            << 0 as libc::c_int) as guint16;
}
#[inline]
unsafe extern "C" fn FPI_DEVICE_SYNAPTICS(mut ptr: gpointer) -> *mut FpiDeviceSynaptics {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_synaptics_get_type(),
    ) as *mut libc::c_void as *mut FpiDeviceSynaptics;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_synaptics_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_synaptics_get_type_once();
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
static mut FpiDeviceSynaptics_private_offset: gint = 0;
static mut fpi_device_synaptics_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
unsafe extern "C" fn fpi_device_synaptics_class_intern_init(mut klass: gpointer) {
    fpi_device_synaptics_parent_class = g_type_class_peek_parent(klass);
    if FpiDeviceSynaptics_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &mut FpiDeviceSynaptics_private_offset,
        );
    }
    fpi_device_synaptics_class_init(klass as *mut FpiDeviceSynapticsClass);
}
#[inline(never)]
unsafe extern "C" fn fpi_device_synaptics_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_device_get_type(),
        g_intern_static_string(
            b"FpiDeviceSynaptics\0" as *const u8 as *const libc::c_char,
        ),
        ::core::mem::size_of::<FpiDeviceSynapticsClass>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_synaptics_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpiDeviceSynaptics>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpiDeviceSynaptics) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_synaptics_init
                        as unsafe extern "C" fn(*mut FpiDeviceSynaptics) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut id_table: [FpIdEntry; 15] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xbd as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xdf as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xf9 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xfc as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xc2 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0x100 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xf0 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0x103 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0x123 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0x126 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0x129 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0x168 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0x15f as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
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
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0x104 as libc::c_int as guint,
                        vid: 0x6cb as libc::c_int as guint,
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
unsafe extern "C" fn cmd_receive_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceSynaptics = FPI_DEVICE_SYNAPTICS(device as gpointer);
    let mut callback: SynCmdMsgCallback = ::core::mem::transmute::<
        gpointer,
        SynCmdMsgCallback,
    >(user_data);
    let mut res: libc::c_int = 0;
    let mut msg_resp: bmkt_msg_resp_t = bmkt_msg_resp_t {
        msg_id: 0,
        seq_num: 0,
        payload_len: 0,
        payload: 0 as *mut uint8_t,
        result: 0,
    };
    let mut resp: bmkt_response_t = bmkt_response_t {
        response_id: 0,
        result: 0,
        complete: 0,
        response: bmkt_response_data_t {
            init_resp: bmkt_init_resp_t {
                finger_presence: 0,
            },
        },
    };
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    res = bmkt_parse_message_header(
        &mut *((*transfer).buffer).offset(2 as libc::c_int as isize),
        ((*transfer).actual_length - 2 as libc::c_int as libc::c_long) as libc::c_int,
        &mut msg_resp,
    );
    if res != 0 as libc::c_int {
        g_log(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Corrupted message received\0" as *const u8 as *const libc::c_char,
        );
        fpi_ssm_mark_failed(
            (*transfer).ssm,
            fpi_device_error_new(FP_DEVICE_ERROR_PROTO),
        );
        return;
    }
    if msg_resp.msg_id as libc::c_int == 0x91 as libc::c_int {
        if msg_resp.payload_len as libc::c_int != 1 as libc::c_int {
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Corrupted finger report received\0" as *const u8 as *const libc::c_char,
            );
            fpi_ssm_mark_failed(
                (*transfer).ssm,
                fpi_device_error_new(FP_DEVICE_ERROR_PROTO),
            );
            return;
        }
        if *(msg_resp.payload).offset(0 as libc::c_int as isize) as libc::c_int
            == 0x1 as libc::c_int
        {
            (*self_0).finger_on_sensor = (0 as libc::c_int == 0) as libc::c_int;
            fpi_device_report_finger_status_changes(
                device,
                FP_FINGER_STATUS_PRESENT,
                FP_FINGER_STATUS_NONE,
            );
        } else {
            (*self_0).finger_on_sensor = 0 as libc::c_int;
            fpi_device_report_finger_status_changes(
                device,
                FP_FINGER_STATUS_NONE,
                FP_FINGER_STATUS_PRESENT,
            );
            if (*self_0).cmd_complete_on_removal != 0 {
                fpi_ssm_mark_completed((*transfer).ssm);
                return;
            }
        }
        g_log(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Finger is now %s the sensor\0" as *const u8 as *const libc::c_char,
            if (*self_0).finger_on_sensor != 0 {
                b"on\0" as *const u8 as *const libc::c_char
            } else {
                b"off\0" as *const u8 as *const libc::c_char
            },
        );
    }
    res = bmkt_parse_message_payload(&mut msg_resp, &mut resp);
    if res != 0 as libc::c_int {
        g_log(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Could not parse message payload: %i\0" as *const u8 as *const libc::c_char,
            res,
        );
        fpi_ssm_mark_failed(
            (*transfer).ssm,
            fpi_device_error_new(FP_DEVICE_ERROR_PROTO),
        );
        return;
    }
    if resp.response_id == 0x42 as libc::c_int || resp.response_id == 0x43 as libc::c_int
    {
        if resp.response_id == 0x42 as libc::c_int {
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Received cancellation success response\0" as *const u8
                    as *const libc::c_char,
            );
            fpi_ssm_mark_failed(
                (*transfer).ssm,
                g_error_new_literal(
                    g_io_error_quark(),
                    G_IO_ERROR_CANCELLED as libc::c_int,
                    b"Device reported cancellation of operation\0" as *const u8
                        as *const libc::c_char,
                ),
            );
        } else {
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Cancellation failed, this should not happen\0" as *const u8
                    as *const libc::c_char,
            );
            fpi_ssm_mark_failed(
                (*transfer).ssm,
                fpi_device_error_new(FP_DEVICE_ERROR_PROTO),
            );
        }
        return;
    }
    if msg_resp.seq_num as libc::c_int == 0 as libc::c_int {
        if msg_resp.msg_id as libc::c_int == 0xc1 as libc::c_int {
            let mut err: guint16 = 0;
            err = _fpi_slow_read16_be(msg_resp.payload as *const guint8);
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Received General Error %d from the sensor\0" as *const u8
                    as *const libc::c_char,
                err as guint,
            );
            fpi_ssm_mark_failed(
                (*transfer).ssm,
                fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_PROTO,
                    b"Received general error %u from device\0" as *const u8
                        as *const libc::c_char,
                    err as guint,
                ),
            );
            return;
        } else {
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Received message with 0 sequence number 0x%02x, ignoring!\0"
                    as *const u8 as *const libc::c_char,
                msg_resp.msg_id as libc::c_int,
            );
            fpi_ssm_next_state((*transfer).ssm);
            return;
        }
    }
    if msg_resp.seq_num as libc::c_int != (*self_0).cmd_seq_num as libc::c_int {
        g_log(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Got unexpected sequence number from device, %d instead of %d\0"
                as *const u8 as *const libc::c_char,
            msg_resp.seq_num as libc::c_int,
            (*self_0).cmd_seq_num as libc::c_int,
        );
    }
    if callback.is_some() {
        callback
            .expect("non-null function pointer")(self_0, &mut resp, 0 as *mut GError);
    }
    if !((*self_0).cmd_pending_transfer).is_null() {
        fpi_ssm_jump_to_state(
            (*transfer).ssm,
            SYNAPTICS_CMD_SEND_PENDING as libc::c_int,
        );
    } else if resp.complete == 0 || (*self_0).cmd_complete_on_removal != 0 {
        fpi_ssm_next_state((*transfer).ssm);
    } else {
        fpi_ssm_mark_completed((*transfer).ssm);
    };
}
unsafe extern "C" fn cmd_interrupt_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    g_log(
        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"interrupt transfer done\0" as *const u8 as *const libc::c_char,
    );
    fpi_device_critical_enter(device);
    if !error.is_null() {
        if g_error_matches(
            error,
            g_io_error_quark(),
            G_IO_ERROR_CANCELLED as libc::c_int,
        ) != 0
        {
            g_error_free(error);
            if (*FPI_DEVICE_SYNAPTICS(device as gpointer)).cmd_suspended != 0 {
                fpi_ssm_jump_to_state(
                    (*transfer).ssm,
                    SYNAPTICS_CMD_SUSPENDED as libc::c_int,
                );
            } else {
                fpi_ssm_jump_to_state(
                    (*transfer).ssm,
                    SYNAPTICS_CMD_GET_RESP as libc::c_int,
                );
            }
            return;
        }
        fpi_ssm_mark_failed((*transfer).ssm, error);
        return;
    }
    if *((*transfer).buffer).offset(0 as libc::c_int as isize) as libc::c_int
        & 0x4 as libc::c_int != 0
    {
        fpi_ssm_next_state((*transfer).ssm);
    } else {
        fpi_device_critical_leave(device);
        fpi_usb_transfer_submit(
            fpi_usb_transfer_ref(transfer),
            0 as libc::c_int as guint,
            0 as *mut GCancellable,
            Some(
                cmd_interrupt_cb
                    as unsafe extern "C" fn(
                        *mut FpiUsbTransfer,
                        *mut FpDevice,
                        gpointer,
                        *mut GError,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
    };
}
unsafe extern "C" fn synaptics_cmd_run_state(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    let mut self_0: *mut FpiDeviceSynaptics = FPI_DEVICE_SYNAPTICS(dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            if !((*self_0).cmd_pending_transfer).is_null() {
                (*(*self_0).cmd_pending_transfer).ssm = ssm;
                fpi_usb_transfer_submit(
                    (*self_0).cmd_pending_transfer,
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
                (*self_0).cmd_pending_transfer = 0 as *mut FpiUsbTransfer;
            } else {
                fpi_ssm_next_state(ssm);
            }
        }
        1 => {
            transfer = fpi_usb_transfer_new(dev);
            (*transfer).ssm = ssm;
            fpi_usb_transfer_fill_bulk(
                transfer,
                0x81 as libc::c_int as guint8,
                (263 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int) as gsize,
            );
            fpi_usb_transfer_submit(
                transfer,
                5000 as libc::c_int as guint,
                0 as *mut GCancellable,
                Some(
                    cmd_receive_cb
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
            fpi_device_critical_leave(dev);
            transfer = fpi_usb_transfer_new(dev);
            (*transfer).ssm = ssm;
            fpi_usb_transfer_fill_interrupt(
                transfer,
                0x83 as libc::c_int as guint8,
                7 as libc::c_int as gsize,
            );
            fpi_usb_transfer_submit(
                transfer,
                0 as libc::c_int as guint,
                (*self_0).interrupt_cancellable,
                Some(
                    cmd_interrupt_cb
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
        3 => {
            transfer = fpi_usb_transfer_new(dev);
            (*transfer).ssm = ssm;
            fpi_usb_transfer_fill_bulk(
                transfer,
                0x1 as libc::c_int as guint8,
                1 as libc::c_int as gsize,
            );
            *((*transfer).buffer)
                .offset(0 as libc::c_int as isize) = 168 as libc::c_int as guchar;
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
        4 => {
            fpi_ssm_jump_to_state(ssm, SYNAPTICS_CMD_SEND_PENDING as libc::c_int);
        }
        5 => {
            fpi_device_critical_leave(dev);
            fpi_device_suspend_complete(dev, 0 as *mut GError);
        }
        6 => {
            fpi_device_critical_enter(dev);
            fpi_ssm_jump_to_state(ssm, SYNAPTICS_CMD_WAIT_INTERRUPT as libc::c_int);
        }
        _ => {}
    };
}
unsafe extern "C" fn cmd_ssm_done(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceSynaptics = FPI_DEVICE_SYNAPTICS(dev as gpointer);
    let mut callback: SynCmdMsgCallback = ::core::mem::transmute::<
        gpointer,
        SynCmdMsgCallback,
    >(fpi_ssm_get_data(ssm));
    (*self_0).cmd_ssm = 0 as *mut FpiSsm;
    if !error.is_null() || (*self_0).cmd_complete_on_removal != 0 {
        callback
            .expect(
                "non-null function pointer",
            )(self_0, 0 as *mut bmkt_response_t, error);
    }
    fpi_device_critical_leave(dev);
    (*self_0).cmd_complete_on_removal = 0 as libc::c_int;
}
unsafe extern "C" fn cmd_forget_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    if !error.is_null() {
        g_log(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Async command sending failed: %s\0" as *const u8 as *const libc::c_char,
            (*error).message,
        );
        g_error_free(error);
    } else {
        g_log(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Async command sent successfully\0" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn synaptics_sensor_cmd(
    mut self_0: *mut FpiDeviceSynaptics,
    mut seq_num: gint,
    mut msg_id: guint8,
    mut payload: *const guint8,
    mut payload_len: gssize,
    mut callback: SynCmdMsgCallback,
) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    let mut real_seq_num: guint8 = 0;
    let mut msg_len: gint = 0;
    let mut res: gint = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !payload.is_null() || payload_len == 0 as libc::c_int as libc::c_long {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
                as *const libc::c_char,
            371 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"synaptics_sensor_cmd\0"))
                .as_ptr(),
            b"payload || payload_len == 0\0" as *const u8 as *const libc::c_char,
        );
    }
    if seq_num <= 0 as libc::c_int {
        (*self_0)
            .last_seq_num = (if 1 as libc::c_int
            > (*self_0).last_seq_num as libc::c_int + 1 as libc::c_int
                & 0xff as libc::c_int
        {
            1 as libc::c_int
        } else {
            (*self_0).last_seq_num as libc::c_int + 1 as libc::c_int
                & 0xff as libc::c_int
        }) as guint8;
        real_seq_num = (*self_0).last_seq_num;
        if seq_num == 0 as libc::c_int {
            (*self_0).cmd_seq_num = (*self_0).last_seq_num;
        }
    } else {
        real_seq_num = seq_num as guint8;
        (*self_0).last_seq_num = real_seq_num;
    }
    g_log(
        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"sequence number is %d\0" as *const u8 as *const libc::c_char,
        real_seq_num as libc::c_int,
    );
    msg_len = (4 as libc::c_int as libc::c_long + payload_len) as gint;
    transfer = fpi_usb_transfer_new(FP_DEVICE(self_0 as gpointer));
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_fill_bulk(
        transfer,
        0x1 as libc::c_int as guint8,
        (msg_len + 1 as libc::c_int) as gsize,
    );
    *((*transfer).buffer)
        .offset(0 as libc::c_int as isize) = 167 as libc::c_int as guchar;
    res = bmkt_compose_message(
        &mut *((*transfer).buffer).offset(1 as libc::c_int as isize),
        &mut msg_len,
        msg_id,
        real_seq_num,
        payload_len as uint8_t,
        payload,
    );
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if res == 0 as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
                as *const libc::c_char,
            408 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"synaptics_sensor_cmd\0"))
                .as_ptr(),
            b"res == BMKT_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (msg_len + 1 as libc::c_int) as libc::c_long == (*transfer).length {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
                as *const libc::c_char,
            409 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"synaptics_sensor_cmd\0"))
                .as_ptr(),
            b"msg_len + SENSOR_FW_CMD_HEADER_LEN == transfer->length\0" as *const u8
                as *const libc::c_char,
        );
    }
    if seq_num == -(1 as libc::c_int) {
        if ({
            let mut _g_boolean_var_: libc::c_int = 0;
            if callback.is_none() {
                _g_boolean_var_ = 1 as libc::c_int;
            } else {
                _g_boolean_var_ = 0 as libc::c_int;
            }
            _g_boolean_var_
        }) as libc::c_long != 0
        {} else {
            g_assertion_message_expr(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
                    as *const libc::c_char,
                415 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"synaptics_sensor_cmd\0"))
                    .as_ptr(),
                b"callback == NULL\0" as *const u8 as *const libc::c_char,
            );
        }
        fpi_usb_transfer_submit(
            transfer,
            1000 as libc::c_int as guint,
            0 as *mut GCancellable,
            Some(
                cmd_forget_cb
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
        if ({
            let mut _g_boolean_var_: libc::c_int = 0;
            if ((*self_0).cmd_pending_transfer).is_null() {
                _g_boolean_var_ = 1 as libc::c_int;
            } else {
                _g_boolean_var_ = 0 as libc::c_int;
            }
            _g_boolean_var_
        }) as libc::c_long != 0
        {} else {
            g_assertion_message_expr(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
                    as *const libc::c_char,
                423 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"synaptics_sensor_cmd\0"))
                    .as_ptr(),
                b"self->cmd_pending_transfer == NULL\0" as *const u8
                    as *const libc::c_char,
            );
        }
        (*self_0)
            .cmd_pending_transfer = (if 0 as libc::c_int != 0 {
            transfer as *mut libc::c_void
        } else {
            g_steal_pointer(&mut transfer as *mut *mut FpiUsbTransfer as gpointer)
        }) as *mut FpiUsbTransfer;
        if !((*self_0).cmd_ssm).is_null() {
            if ({
                let mut _g_boolean_var_: libc::c_int = 0;
                if callback.is_none() {
                    _g_boolean_var_ = 1 as libc::c_int;
                } else {
                    _g_boolean_var_ = 0 as libc::c_int;
                }
                _g_boolean_var_
            }) as libc::c_long != 0
            {} else {
                g_assertion_message_expr(
                    b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                    b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
                        as *const libc::c_char,
                    432 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 21],
                        &[libc::c_char; 21],
                    >(b"synaptics_sensor_cmd\0"))
                        .as_ptr(),
                    b"callback == NULL\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            if ({
                let mut _g_boolean_var_: libc::c_int = 0;
                if callback.is_some() {
                    _g_boolean_var_ = 1 as libc::c_int;
                } else {
                    _g_boolean_var_ = 0 as libc::c_int;
                }
                _g_boolean_var_
            }) as libc::c_long != 0
            {} else {
                g_assertion_message_expr(
                    b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                    b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
                        as *const libc::c_char,
                    437 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 21],
                        &[libc::c_char; 21],
                    >(b"synaptics_sensor_cmd\0"))
                        .as_ptr(),
                    b"callback != NULL\0" as *const u8 as *const libc::c_char,
                );
            }
            (*self_0)
                .cmd_ssm = fpi_ssm_new_full(
                FP_DEVICE(self_0 as gpointer),
                Some(
                    synaptics_cmd_run_state
                        as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                ),
                SYNAPTICS_CMD_NUM_STATES as libc::c_int,
                SYNAPTICS_CMD_NUM_STATES as libc::c_int,
                b"SYNAPTICS_CMD_NUM_STATES\0" as *const u8 as *const libc::c_char,
            );
            fpi_ssm_set_data(
                (*self_0).cmd_ssm,
                ::core::mem::transmute::<SynCmdMsgCallback, gpointer>(callback),
                None,
            );
            fpi_device_critical_enter(FP_DEVICE(self_0 as gpointer));
            fpi_ssm_start(
                (*self_0).cmd_ssm,
                Some(
                    cmd_ssm_done
                        as unsafe extern "C" fn(
                            *mut FpiSsm,
                            *mut FpDevice,
                            *mut GError,
                        ) -> (),
                ),
            );
        }
    };
}
unsafe extern "C" fn parse_print_data(
    mut data: *mut GVariant,
    mut finger: *mut guint8,
    mut user_id: *mut *const guint8,
    mut user_id_len: *mut gsize,
) -> gboolean {
    let mut user_id_var: GVariant_autoptr = 0 as GVariant_autoptr;
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
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
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
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
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
        if !user_id.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
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
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"parse_print_data\0"))
                .as_ptr(),
            b"user_id_len != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    *user_id = 0 as *const guint8;
    *user_id_len = 0 as libc::c_int as gsize;
    if g_variant_check_format_string(
        data,
        b"(y@ay)\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    g_variant_get(
        data,
        b"(y@ay)\0" as *const u8 as *const libc::c_char,
        finger,
        &mut user_id_var as *mut GVariant_autoptr,
    );
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
    return (0 as libc::c_int == 0) as libc::c_int;
}
unsafe extern "C" fn create_print(
    mut self_0: *mut FpiDeviceSynaptics,
    mut user_id: *mut guint8,
    mut finger_id: guint8,
) -> *mut FpPrint {
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    let mut user_id_safe: *mut gchar = 0 as *mut gchar;
    let mut data: *mut GVariant = 0 as *mut GVariant;
    let mut uid: *mut GVariant = 0 as *mut GVariant;
    user_id_safe = g_strndup(user_id as *mut libc::c_char, 100 as libc::c_int as gsize);
    print = fp_print_new(FP_DEVICE(self_0 as gpointer));
    uid = g_variant_new_fixed_array(
        b"y\0" as *const u8 as *const libc::c_char as *const GVariantType,
        user_id_safe as gconstpointer,
        strlen(user_id_safe),
        1 as libc::c_int as gsize,
    );
    data = g_variant_new(
        b"(y@ay)\0" as *const u8 as *const libc::c_char,
        finger_id as libc::c_int,
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
        user_id_safe,
        0 as *mut libc::c_void,
    );
    fpi_print_fill_from_user_id(print, user_id_safe);
    return print;
}
unsafe extern "C" fn verify_complete_after_finger_removal(
    mut self_0: *mut FpiDeviceSynaptics,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    if (*self_0).finger_on_sensor != 0 {
        g_log(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"delaying verify report until after finger removal!\0" as *const u8
                as *const libc::c_char,
        );
        (*self_0).cmd_complete_on_removal = (0 as libc::c_int == 0) as libc::c_int;
    } else {
        fpi_device_verify_complete(device, 0 as *mut GError);
    };
}
unsafe extern "C" fn verify_msg_cb(
    mut self_0: *mut FpiDeviceSynaptics,
    mut resp: *mut bmkt_response_t,
    mut error: *mut GError,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    let mut verify_resp: *mut bmkt_verify_resp_t = 0 as *mut bmkt_verify_resp_t;
    if (*self_0).action_starting != 0 {
        fpi_device_critical_leave(device);
        (*self_0).action_starting = 0 as libc::c_int;
    }
    if !error.is_null() {
        fpi_device_verify_complete(device, error);
        return;
    }
    if resp.is_null() && (*self_0).cmd_complete_on_removal != 0 {
        fpi_device_verify_complete(device, 0 as *mut GError);
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !resp.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
                as *const libc::c_char,
            558 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"verify_msg_cb\0"))
                .as_ptr(),
            b"resp != NULL\0" as *const u8 as *const libc::c_char,
        );
    }
    verify_resp = &mut (*resp).response.verify_resp;
    match (*resp).response_id {
        102 => {
            fpi_device_report_finger_status_changes(
                device,
                FP_FINGER_STATUS_NEEDED,
                FP_FINGER_STATUS_NONE,
            );
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Place Finger on the Sensor!\0" as *const u8 as *const libc::c_char,
            );
        }
        96 => {
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Fingerprint image capture complete!\0" as *const u8
                    as *const libc::c_char,
            );
        }
        103 => {
            if (*resp).result == 213 as libc::c_int {
                g_log(
                    b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"Match error occurred\0" as *const u8 as *const libc::c_char,
                );
                fpi_device_verify_report(
                    device,
                    FPI_MATCH_ERROR,
                    0 as *mut FpPrint,
                    fpi_device_retry_new(FP_DEVICE_RETRY_GENERAL),
                );
                verify_complete_after_finger_removal(self_0);
            } else if (*resp).result == 404 as libc::c_int {
                g_log(
                    b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"Print didn't match\0" as *const u8 as *const libc::c_char,
                );
                fpi_device_verify_report(
                    device,
                    FPI_MATCH_FAIL,
                    0 as *mut FpPrint,
                    0 as *mut GError,
                );
                verify_complete_after_finger_removal(self_0);
            } else if (*resp).result == 504 as libc::c_int {
                g_log(
                    b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"Print is not in database\0" as *const u8 as *const libc::c_char,
                );
                fpi_device_verify_complete(
                    device,
                    fpi_device_error_new(FP_DEVICE_ERROR_DATA_NOT_FOUND),
                );
            } else {
                g_log(
                    b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_WARNING,
                    b"Verify has failed: %d\0" as *const u8 as *const libc::c_char,
                    (*resp).result,
                );
                fpi_device_verify_complete(
                    device,
                    fpi_device_error_new_msg(
                        FP_DEVICE_ERROR_PROTO,
                        b"Unexpected result from device %d\0" as *const u8
                            as *const libc::c_char,
                        (*resp).result,
                    ),
                );
            }
        }
        104 => {
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Verify was successful! for user: %s finger: %d score: %f\0"
                    as *const u8 as *const libc::c_char,
                ((*verify_resp).user_id).as_mut_ptr(),
                (*verify_resp).finger_id as libc::c_int,
                (*verify_resp).match_result,
            );
            fpi_device_verify_report(
                device,
                FPI_MATCH_SUCCESS,
                0 as *mut FpPrint,
                0 as *mut GError,
            );
            verify_complete_after_finger_removal(self_0);
        }
        _ => {}
    };
}
unsafe extern "C" fn verify(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceSynaptics = FPI_DEVICE_SYNAPTICS(device as gpointer);
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    let mut data: GVariant_autoptr = 0 as GVariant_autoptr;
    let mut finger: guint8 = 0;
    let mut user_id: *const guint8 = 0 as *const guint8;
    let mut user_id_len: gsize = 0 as libc::c_int as gsize;
    fpi_device_get_verify_data(device, &mut print);
    g_object_get(
        print as gpointer,
        b"fpi-data\0" as *const u8 as *const libc::c_char,
        &mut data as *mut GVariant_autoptr,
        0 as *mut libc::c_void,
    );
    g_log(
        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"data is %p\0" as *const u8 as *const libc::c_char,
        data,
    );
    if parse_print_data(data, &mut finger, &mut user_id, &mut user_id_len) == 0 {
        fpi_device_verify_complete(
            device,
            fpi_device_error_new(FP_DEVICE_ERROR_DATA_INVALID),
        );
        return;
    }
    g_log_structured(
        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
            as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"636\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"verify\0")).as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/synaptics/synaptics.c:636\0" as *const u8
            as *const libc::c_char,
    );
    (*self_0).action_starting = (0 as libc::c_int == 0) as libc::c_int;
    fpi_device_critical_enter(device);
    synaptics_sensor_cmd(
        self_0,
        0 as libc::c_int,
        0x65 as libc::c_int as guint8,
        user_id,
        user_id_len as gssize,
        Some(
            verify_msg_cb
                as unsafe extern "C" fn(
                    *mut FpiDeviceSynaptics,
                    *mut bmkt_response_t,
                    *mut GError,
                ) -> (),
        ),
    );
}
unsafe extern "C" fn identify_complete_after_finger_removal(
    mut self_0: *mut FpiDeviceSynaptics,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    if (*self_0).finger_on_sensor != 0 {
        g_log(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"delaying identify report until after finger removal!\0" as *const u8
                as *const libc::c_char,
        );
        (*self_0).cmd_complete_on_removal = (0 as libc::c_int == 0) as libc::c_int;
    } else {
        fpi_device_identify_complete(device, 0 as *mut GError);
    };
}
unsafe extern "C" fn identify_msg_cb(
    mut self_0: *mut FpiDeviceSynaptics,
    mut resp: *mut bmkt_response_t,
    mut error: *mut GError,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    if (*self_0).action_starting != 0 {
        fpi_device_critical_leave(device);
        (*self_0).action_starting = 0 as libc::c_int;
    }
    if !error.is_null() {
        fpi_device_identify_complete(device, error);
        return;
    }
    if resp.is_null() && (*self_0).cmd_complete_on_removal != 0 {
        fpi_device_identify_complete(device, 0 as *mut GError);
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !resp.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
                as *const libc::c_char,
            685 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"identify_msg_cb\0"))
                .as_ptr(),
            b"resp != NULL\0" as *const u8 as *const libc::c_char,
        );
    }
    match (*resp).response_id {
        98 => {
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Place Finger on the Sensor!\0" as *const u8 as *const libc::c_char,
            );
        }
        226 => {
            compose_and_send_identify_msg(device);
        }
        99 => {
            if (*resp).result == 213 as libc::c_int {
                g_log(
                    b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"Match error occurred\0" as *const u8 as *const libc::c_char,
                );
                fpi_device_identify_report(
                    device,
                    0 as *mut FpPrint,
                    0 as *mut FpPrint,
                    fpi_device_retry_new(FP_DEVICE_RETRY_GENERAL),
                );
                identify_complete_after_finger_removal(self_0);
            } else if (*resp).result == 404 as libc::c_int {
                g_log(
                    b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"Print didn't match\0" as *const u8 as *const libc::c_char,
                );
                fpi_device_identify_report(
                    device,
                    0 as *mut FpPrint,
                    0 as *mut FpPrint,
                    0 as *mut GError,
                );
                identify_complete_after_finger_removal(self_0);
            } else if (*resp).result == 504 as libc::c_int {
                g_log(
                    b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"Print is not in database\0" as *const u8 as *const libc::c_char,
                );
                fpi_device_identify_complete(
                    device,
                    fpi_device_error_new(FP_DEVICE_ERROR_DATA_NOT_FOUND),
                );
            } else {
                g_log(
                    b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_WARNING,
                    b"identify has failed: %d\0" as *const u8 as *const libc::c_char,
                    (*resp).result,
                );
                fpi_device_identify_complete(
                    device,
                    fpi_device_error_new_msg(
                        FP_DEVICE_ERROR_PROTO,
                        b"Unexpected result from device %d\0" as *const u8
                            as *const libc::c_char,
                        (*resp).result,
                    ),
                );
            }
        }
        100 => {
            let mut print: *mut FpPrint = 0 as *mut FpPrint;
            let mut prints: *mut GPtrArray = 0 as *mut GPtrArray;
            let mut data: GVariant_autoptr = 0 as GVariant_autoptr;
            let mut found: gboolean = 0 as libc::c_int;
            let mut index: guint = 0;
            print = create_print(
                self_0,
                ((*resp).response.id_resp.user_id).as_mut_ptr(),
                (*resp).response.id_resp.finger_id,
            );
            fpi_device_get_identify_data(device, &mut prints);
            found = g_ptr_array_find_with_equal_func(
                prints,
                print as gconstpointer,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(*mut FpPrint, *mut FpPrint) -> gboolean,
                    >,
                    GEqualFunc,
                >(
                    Some(
                        fp_print_equal
                            as unsafe extern "C" fn(
                                *mut FpPrint,
                                *mut FpPrint,
                            ) -> gboolean,
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
            identify_complete_after_finger_removal(self_0);
        }
        _ => {}
    };
}
unsafe extern "C" fn identify(mut device: *mut FpDevice) {
    let mut prints: *mut GPtrArray = 0 as *mut GPtrArray;
    let mut self_0: *mut FpiDeviceSynaptics = FPI_DEVICE_SYNAPTICS(device as gpointer);
    fpi_device_get_identify_data(device, &mut prints);
    if (*prints).len == 0 as libc::c_int as libc::c_uint {
        fpi_device_identify_report(
            device,
            0 as *mut FpPrint,
            0 as *mut FpPrint,
            0 as *mut GError,
        );
        fpi_device_identify_complete(device, 0 as *mut GError);
        return;
    }
    (*self_0).action_starting = (0 as libc::c_int == 0) as libc::c_int;
    fpi_device_critical_enter(device);
    init_identify_msg(device);
    compose_and_send_identify_msg(device);
}
unsafe extern "C" fn init_identify_msg(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceSynaptics = FPI_DEVICE_SYNAPTICS(device as gpointer);
    (*self_0).id_idx = 0 as libc::c_int as guint8;
}
unsafe extern "C" fn compose_and_send_identify_msg(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceSynaptics = FPI_DEVICE_SYNAPTICS(device as gpointer);
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    let mut prints: *mut GPtrArray = 0 as *mut GPtrArray;
    let mut data: GVariant_autoptr = 0 as GVariant_autoptr;
    let mut finger: guint8 = 0;
    let mut user_id: *const guint8 = 0 as *const guint8;
    let mut user_id_len: gsize = 0 as libc::c_int as gsize;
    let mut payload: *mut guint8 = 0 as *mut guint8;
    let mut payload_len: guint8 = 0 as libc::c_int as guint8;
    let mut payloadOffset: guint8 = 0 as libc::c_int as guint8;
    fpi_device_get_identify_data(device, &mut prints);
    if (*prints).len > 255 as libc::c_int as libc::c_uint {
        fpi_device_identify_complete(
            device,
            fpi_device_error_new(FP_DEVICE_ERROR_DATA_INVALID),
        );
        return;
    }
    if (*self_0).id_idx as libc::c_uint >= (*prints).len {
        g_log(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Device asked for more prints than we are providing.\0" as *const u8
                as *const libc::c_char,
        );
        fpi_device_identify_complete(
            device,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_PROTO,
                b"Unexpected index\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    print = *((*prints).pdata).offset((*self_0).id_idx as isize) as *mut FpPrint;
    g_object_get(
        print as gpointer,
        b"fpi-data\0" as *const u8 as *const libc::c_char,
        &mut data as *mut GVariant_autoptr,
        0 as *mut libc::c_void,
    );
    g_log(
        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"data is %p\0" as *const u8 as *const libc::c_char,
        data,
    );
    if parse_print_data(data, &mut finger, &mut user_id, &mut user_id_len) == 0 {
        fpi_device_identify_complete(
            device,
            fpi_device_error_new(FP_DEVICE_ERROR_DATA_INVALID),
        );
        return;
    }
    if (*self_0).id_idx as libc::c_int == 0 as libc::c_int {
        payload_len = ((2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_add(user_id_len) as guint8;
        payload = g_malloc0(payload_len as gsize) as *mut guint8;
        *payload.offset(payloadOffset as isize) = (*prints).len as guint8;
        payloadOffset = (payloadOffset as libc::c_int + 1 as libc::c_int) as guint8;
        *payload.offset(payloadOffset as isize) = 1 as libc::c_int as guint8;
        payloadOffset = (payloadOffset as libc::c_int + 1 as libc::c_int) as guint8;
        *payload.offset(payloadOffset as isize) = user_id_len as guint8;
        payloadOffset = (payloadOffset as libc::c_int + 1 as libc::c_int) as guint8;
        memcpy(
            &mut *payload.offset(payloadOffset as isize) as *mut guint8
                as *mut libc::c_void,
            user_id as *const libc::c_void,
            user_id_len,
        );
        payloadOffset = (payloadOffset as libc::c_ulong).wrapping_add(user_id_len)
            as guint8 as guint8;
        g_log_structured(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"CODE_FILE\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
                as *const libc::c_char,
            b"CODE_LINE\0" as *const u8 as *const libc::c_char,
            b"851\0" as *const u8 as *const libc::c_char,
            b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"compose_and_send_identify_msg\0"))
                .as_ptr(),
            b"MESSAGE\0" as *const u8 as *const libc::c_char,
            b"%li: %s\0" as *const u8 as *const libc::c_char,
            g_get_monotonic_time(),
            b"../libfprint/drivers/synaptics/synaptics.c:851\0" as *const u8
                as *const libc::c_char,
        );
        synaptics_sensor_cmd(
            self_0,
            0 as libc::c_int,
            0xe1 as libc::c_int as guint8,
            payload,
            payloadOffset as gssize,
            Some(
                identify_msg_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceSynaptics,
                        *mut bmkt_response_t,
                        *mut GError,
                    ) -> (),
            ),
        );
    } else {
        payload_len = ((1 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_add(user_id_len) as guint8;
        payload = g_malloc0(payload_len as gsize) as *mut guint8;
        *payload.offset(payloadOffset as isize) = 1 as libc::c_int as guint8;
        payloadOffset = (payloadOffset as libc::c_int + 1 as libc::c_int) as guint8;
        *payload.offset(payloadOffset as isize) = user_id_len as guint8;
        payloadOffset = (payloadOffset as libc::c_int + 1 as libc::c_int) as guint8;
        memcpy(
            &mut *payload.offset(payloadOffset as isize) as *mut guint8
                as *mut libc::c_void,
            user_id as *const libc::c_void,
            user_id_len,
        );
        payloadOffset = (payloadOffset as libc::c_ulong).wrapping_add(user_id_len)
            as guint8 as guint8;
        synaptics_sensor_cmd(
            self_0,
            (*self_0).cmd_seq_num as gint,
            0xe3 as libc::c_int as guint8,
            payload,
            payloadOffset as gssize,
            None,
        );
    }
    (*self_0).id_idx = ((*self_0).id_idx).wrapping_add(1);
}
unsafe extern "C" fn enroll_msg_cb(
    mut self_0: *mut FpiDeviceSynaptics,
    mut resp: *mut bmkt_response_t,
    mut error: *mut GError,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    let mut enroll_resp: *mut bmkt_enroll_resp_t = 0 as *mut bmkt_enroll_resp_t;
    if (*self_0).action_starting != 0 {
        fpi_device_critical_leave(device);
        (*self_0).action_starting = 0 as libc::c_int;
    }
    if !error.is_null() {
        fpi_device_enroll_complete(device, 0 as *mut FpPrint, error);
        return;
    }
    enroll_resp = &mut (*resp).response.enroll_resp;
    match (*resp).response_id {
        84 => {
            (*self_0).enroll_stage = 0 as libc::c_int;
            fpi_device_report_finger_status_changes(
                device,
                FP_FINGER_STATUS_NEEDED,
                FP_FINGER_STATUS_NONE,
            );
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Place Finger on the Sensor!\0" as *const u8 as *const libc::c_char,
            );
        }
        96 => {
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Fingerprint image capture complete!\0" as *const u8
                    as *const libc::c_char,
            );
        }
        85 => {
            let mut done_stages: gint = 0;
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Enrollment is %d %% \0" as *const u8 as *const libc::c_char,
                (*enroll_resp).progress,
            );
            done_stages = ((*enroll_resp).progress * 8 as libc::c_int
                + 99 as libc::c_int) / 100 as libc::c_int;
            if (*enroll_resp).progress < 100 as libc::c_int {
                done_stages = if done_stages < 8 as libc::c_int - 1 as libc::c_int {
                    done_stages
                } else {
                    8 as libc::c_int - 1 as libc::c_int
                };
            }
            if (*self_0).enroll_stage == done_stages {
                fpi_device_enroll_progress(
                    device,
                    done_stages,
                    0 as *mut FpPrint,
                    fpi_device_retry_new(FP_DEVICE_RETRY_GENERAL),
                );
            }
            while (*self_0).enroll_stage < done_stages {
                (*self_0).enroll_stage += 1 as libc::c_int;
                fpi_device_enroll_progress(
                    device,
                    (*self_0).enroll_stage,
                    0 as *mut FpPrint,
                    0 as *mut GError,
                );
            }
        }
        86 => {
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Enrollment has been paused!\0" as *const u8 as *const libc::c_char,
            );
        }
        87 => {
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Enrollment has been resumed!\0" as *const u8 as *const libc::c_char,
            );
        }
        88 => {
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Enrollment has failed!: %d\0" as *const u8 as *const libc::c_char,
                (*resp).result,
            );
            if (*resp).result == 501 as libc::c_int {
                fpi_device_enroll_complete(
                    device,
                    0 as *mut FpPrint,
                    fpi_device_error_new(FP_DEVICE_ERROR_DATA_FULL),
                );
            } else {
                fpi_device_enroll_complete(
                    device,
                    0 as *mut FpPrint,
                    fpi_device_error_new_msg(
                        FP_DEVICE_ERROR_GENERAL,
                        b"Enrollment failed (%d)\0" as *const u8 as *const libc::c_char,
                        (*resp).result,
                    ),
                );
            }
        }
        89 => {
            let mut print: *mut FpPrint = 0 as *mut FpPrint;
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
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
        _ => {}
    };
}
unsafe extern "C" fn enroll(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceSynaptics = FPI_DEVICE_SYNAPTICS(device as gpointer);
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    let mut data: *mut GVariant = 0 as *mut GVariant;
    let mut uid: *mut GVariant = 0 as *mut GVariant;
    let mut finger: guint = 0;
    let mut user_id: *mut gchar = 0 as *mut gchar;
    let mut user_id_len: gssize = 0;
    let mut payload: *mut guint8 = 0 as *mut guint8;
    fpi_device_get_enroll_data(device, &mut print);
    g_log_structured(
        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
            as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"1002\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"enroll\0")).as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/synaptics/synaptics.c:1002\0" as *const u8
            as *const libc::c_char,
    );
    user_id = fpi_print_generate_user_id(print);
    user_id_len = strlen(user_id) as gssize;
    user_id_len = if (100 as libc::c_int as libc::c_long) < user_id_len {
        100 as libc::c_int as libc::c_long
    } else {
        user_id_len
    };
    finger = 1 as libc::c_int as guint;
    uid = g_variant_new_fixed_array(
        b"y\0" as *const u8 as *const libc::c_char as *const GVariantType,
        user_id as gconstpointer,
        user_id_len as gsize,
        1 as libc::c_int as gsize,
    );
    data = g_variant_new(b"(y@ay)\0" as *const u8 as *const libc::c_char, finger, uid);
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
        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"user_id: %s, finger: %d\0" as *const u8 as *const libc::c_char,
        user_id,
        finger,
    );
    payload = g_malloc0((user_id_len + 2 as libc::c_int as libc::c_long) as gsize)
        as *mut guint8;
    *payload.offset(0 as libc::c_int as isize) = 0 as libc::c_int as guint8;
    *payload.offset(1 as libc::c_int as isize) = finger as guint8;
    memcpy(
        payload.offset(2 as libc::c_int as isize) as *mut libc::c_void,
        user_id as *const libc::c_void,
        user_id_len as libc::c_ulong,
    );
    (*self_0).action_starting = (0 as libc::c_int == 0) as libc::c_int;
    fpi_device_critical_enter(device);
    synaptics_sensor_cmd(
        self_0,
        0 as libc::c_int,
        0x51 as libc::c_int as guint8,
        payload,
        user_id_len + 2 as libc::c_int as libc::c_long,
        Some(
            enroll_msg_cb
                as unsafe extern "C" fn(
                    *mut FpiDeviceSynaptics,
                    *mut bmkt_response_t,
                    *mut GError,
                ) -> (),
        ),
    );
}
unsafe extern "C" fn delete_msg_cb(
    mut self_0: *mut FpiDeviceSynaptics,
    mut resp: *mut bmkt_response_t,
    mut error: *mut GError,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    let mut del_user_resp: *mut bmkt_del_user_resp_t = 0 as *mut bmkt_del_user_resp_t;
    if !error.is_null() {
        fpi_device_critical_leave(device);
        fpi_device_delete_complete(device, error);
        return;
    }
    del_user_resp = &mut (*resp).response.del_user_resp;
    match (*resp).response_id {
        135 => {
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Deleting Enrolled Users is %d%% complete\0" as *const u8
                    as *const libc::c_char,
                (*del_user_resp).progress,
            );
        }
        130 => {
            fpi_device_critical_leave(device);
            if (*resp).result == 504 as libc::c_int
                || (*resp).result == 502 as libc::c_int
            {
                g_log(
                    b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"Database no record\0" as *const u8 as *const libc::c_char,
                );
                fpi_device_delete_complete(device, 0 as *mut GError);
            } else {
                g_log(
                    b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"Failed to delete enrolled user: %d\0" as *const u8
                        as *const libc::c_char,
                    (*resp).result,
                );
                fpi_device_delete_complete(
                    device,
                    fpi_device_error_new(FP_DEVICE_ERROR_GENERAL),
                );
            }
        }
        131 => {
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Successfully deleted enrolled user\0" as *const u8
                    as *const libc::c_char,
            );
            fpi_device_critical_leave(device);
            fpi_device_delete_complete(device, 0 as *mut GError);
        }
        _ => {}
    };
}
unsafe extern "C" fn delete_print(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceSynaptics = FPI_DEVICE_SYNAPTICS(device as gpointer);
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    let mut data: GVariant_autoptr = 0 as GVariant_autoptr;
    let mut finger: guint8 = 0;
    let mut user_id: *const guint8 = 0 as *const guint8;
    let mut user_id_len: gsize = 0 as libc::c_int as gsize;
    let mut payload: *mut guint8 = 0 as *mut guint8;
    fpi_device_get_delete_data(device, &mut print);
    g_object_get(
        print as gpointer,
        b"fpi-data\0" as *const u8 as *const libc::c_char,
        &mut data as *mut GVariant_autoptr,
        0 as *mut libc::c_void,
    );
    g_log(
        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"data is %p\0" as *const u8 as *const libc::c_char,
        data,
    );
    if parse_print_data(data, &mut finger, &mut user_id, &mut user_id_len) == 0 {
        fpi_device_delete_complete(
            device,
            fpi_device_error_new(FP_DEVICE_ERROR_DATA_INVALID),
        );
        return;
    }
    g_log_structured(
        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
            as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"1111\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"delete_print\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/synaptics/synaptics.c:1111\0" as *const u8
            as *const libc::c_char,
    );
    payload = g_malloc0((1 as libc::c_int as libc::c_ulong).wrapping_add(user_id_len))
        as *mut guint8;
    *payload.offset(0 as libc::c_int as isize) = finger;
    memcpy(
        payload.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        user_id as *const libc::c_void,
        user_id_len,
    );
    fpi_device_critical_enter(device);
    synaptics_sensor_cmd(
        self_0,
        0 as libc::c_int,
        0x81 as libc::c_int as guint8,
        payload,
        user_id_len.wrapping_add(1 as libc::c_int as libc::c_ulong) as gssize,
        Some(
            delete_msg_cb
                as unsafe extern "C" fn(
                    *mut FpiDeviceSynaptics,
                    *mut bmkt_response_t,
                    *mut GError,
                ) -> (),
        ),
    );
}
unsafe extern "C" fn clear_storage_msg_cb(
    mut self_0: *mut FpiDeviceSynaptics,
    mut resp: *mut bmkt_response_t,
    mut error: *mut GError,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    let mut del_all_user_resp: *mut bmkt_del_all_users_resp_t = 0
        as *mut bmkt_del_all_users_resp_t;
    if !error.is_null() {
        fpi_device_clear_storage_complete(device, error);
        return;
    }
    del_all_user_resp = &mut (*resp).response.del_all_user_resp;
    match (*resp).response_id {
        135 => {
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Deleting All Enrolled Users is %d%% complete\0" as *const u8
                    as *const libc::c_char,
                (*del_all_user_resp).progress,
            );
        }
        133 => {
            if (*resp).result == 502 as libc::c_int {
                fpi_device_clear_storage_complete(device, 0 as *mut GError);
            } else {
                fpi_device_clear_storage_complete(
                    device,
                    fpi_device_error_new(FP_DEVICE_ERROR_GENERAL),
                );
            }
        }
        134 => {
            g_log(
                b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Successfully deleted all enrolled user\0" as *const u8
                    as *const libc::c_char,
            );
            fpi_device_clear_storage_complete(device, 0 as *mut GError);
        }
        _ => {}
    };
}
unsafe extern "C" fn clear_storage(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceSynaptics = FPI_DEVICE_SYNAPTICS(device as gpointer);
    g_log(
        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"clear all prints in database\0" as *const u8 as *const libc::c_char,
    );
    synaptics_sensor_cmd(
        self_0,
        0 as libc::c_int,
        0x84 as libc::c_int as guint8,
        0 as *const guint8,
        0 as libc::c_int as gssize,
        Some(
            clear_storage_msg_cb
                as unsafe extern "C" fn(
                    *mut FpiDeviceSynaptics,
                    *mut bmkt_response_t,
                    *mut GError,
                ) -> (),
        ),
    );
}
unsafe extern "C" fn prob_msg_cb(
    mut self_0: *mut FpiDeviceSynaptics,
    mut resp: *mut bmkt_response_t,
    mut error: *mut GError,
) {
    let mut usb_dev: *mut GUsbDevice = 0 as *mut GUsbDevice;
    let mut serial: *mut gchar = 0 as *mut gchar;
    let mut err: *mut GError = 0 as *mut GError;
    usb_dev = fpi_device_get_usb_device(FP_DEVICE(self_0 as gpointer));
    if !error.is_null() {
        if g_error_matches(
            error,
            g_io_error_quark(),
            G_IO_ERROR_CANCELLED as libc::c_int,
        ) == 0
        {
            err = fpi_device_error_new_msg(
                FP_DEVICE_ERROR_GENERAL,
                b"unsupported firmware version\0" as *const u8 as *const libc::c_char,
            );
        }
        g_usb_device_close(usb_dev, 0 as *mut *mut GError);
        fpi_device_probe_complete(
            FP_DEVICE(self_0 as gpointer),
            0 as *const gchar,
            0 as *const gchar,
            err,
        );
        g_clear_error(&mut error);
        return;
    }
    if g_strcmp0(
        g_getenv(b"FP_DEVICE_EMULATION\0" as *const u8 as *const libc::c_char),
        b"1\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        serial = g_strdup(b"emulated-device\0" as *const u8 as *const libc::c_char);
    } else {
        serial = g_usb_device_get_string_descriptor(
            usb_dev,
            g_usb_device_get_serial_number_index(usb_dev),
            &mut err,
        );
    }
    if (*resp).result == 0 as libc::c_int || (*resp).result == 103 as libc::c_int {
        g_usb_device_close(usb_dev, 0 as *mut *mut GError);
        fpi_device_probe_complete(
            FP_DEVICE(self_0 as gpointer),
            serial,
            0 as *const gchar,
            err,
        );
    } else if (*resp).result == 102 as libc::c_int {
        synaptics_sensor_cmd(
            self_0,
            (*self_0).cmd_seq_num as gint,
            0x41 as libc::c_int as guint8,
            0 as *const guint8,
            0 as libc::c_int as gssize,
            None,
        );
    } else {
        g_log(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Probe fingerprint sensor failed with %d!\0" as *const u8
                as *const libc::c_char,
            (*resp).result,
        );
        g_usb_device_close(usb_dev, 0 as *mut *mut GError);
        fpi_device_probe_complete(
            FP_DEVICE(self_0 as gpointer),
            serial,
            0 as *const gchar,
            fpi_device_error_new(FP_DEVICE_ERROR_GENERAL),
        );
    };
}
unsafe extern "C" fn dev_probe(mut device: *mut FpDevice) {
    let mut current_block: u64;
    let mut self_0: *mut FpiDeviceSynaptics = FPI_DEVICE_SYNAPTICS(device as gpointer);
    let mut usb_dev: *mut GUsbDevice = 0 as *mut GUsbDevice;
    let mut transfer: FpiUsbTransfer_autoptr = 0 as FpiUsbTransfer_autoptr;
    let mut reader: FpiByteReader = FpiByteReader {
        data: 0 as *const guint8,
        size: 0,
        byte: 0,
    };
    let mut error: *mut GError = 0 as *mut GError;
    let mut status: guint16 = 0;
    let mut data: *const guint8 = 0 as *const guint8;
    let mut read_ok: gboolean = (0 as libc::c_int == 0) as libc::c_int;
    let mut serial: *mut gchar = 0 as *mut gchar;
    let mut retry: gboolean = (0 as libc::c_int == 0) as libc::c_int;
    g_log_structured(
        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
            as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"1231\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"dev_probe\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/synaptics/synaptics.c:1231\0" as *const u8
            as *const libc::c_char,
    );
    usb_dev = fpi_device_get_usb_device(device);
    if g_usb_device_open(usb_dev, &mut error) == 0 {
        fpi_device_probe_complete(device, 0 as *const gchar, 0 as *const gchar, error);
        return;
    }
    if g_usb_device_claim_interface(
        usb_dev,
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    ) == 0
    {
        current_block = 9804548584293845510;
    } else {
        current_block = 13183875560443969876;
    }
    loop {
        match current_block {
            9804548584293845510 => {
                g_usb_device_close(usb_dev, 0 as *mut *mut GError);
                fpi_device_probe_complete(
                    device,
                    0 as *const gchar,
                    0 as *const gchar,
                    error,
                );
                return;
            }
            _ => {
                transfer = fpi_usb_transfer_new(device);
                fpi_usb_transfer_fill_bulk(
                    transfer,
                    0x1 as libc::c_int as guint8,
                    1 as libc::c_int as gsize,
                );
                (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
                *((*transfer).buffer)
                    .offset(0 as libc::c_int as isize) = 1 as libc::c_int as guchar;
                if fpi_usb_transfer_submit_sync(
                    transfer,
                    1000 as libc::c_int as guint,
                    &mut error,
                ) == 0
                {
                    current_block = 9804548584293845510;
                    continue;
                }
                let mut _pp: C2RustUnnamed_7 = C2RustUnnamed_7 {
                    in_0: 0 as *mut libc::c_char,
                };
                let mut _p: gpointer = 0 as *mut libc::c_void;
                let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut FpiUsbTransfer) -> ()>,
                    GDestroyNotify,
                >(
                    Some(
                        fpi_usb_transfer_unref
                            as unsafe extern "C" fn(*mut FpiUsbTransfer) -> (),
                    ),
                );
                _pp
                    .in_0 = &mut transfer as *mut FpiUsbTransfer_autoptr
                    as *mut libc::c_char;
                _p = *_pp.out;
                if !_p.is_null() {
                    *_pp.out = 0 as *mut libc::c_void;
                    _destroy.expect("non-null function pointer")(_p);
                }
                transfer = fpi_usb_transfer_new(device);
                fpi_usb_transfer_fill_bulk(
                    transfer,
                    0x81 as libc::c_int as guint8,
                    40 as libc::c_int as gsize,
                );
                if fpi_usb_transfer_submit_sync(
                    transfer,
                    1000 as libc::c_int as guint,
                    &mut error,
                ) == 0
                {
                    current_block = 9804548584293845510;
                    continue;
                }
                fpi_byte_reader_init_inline(
                    &mut reader,
                    (*transfer).buffer,
                    (*transfer).actual_length as guint,
                );
                if ({
                    let mut _g_boolean_var_: libc::c_int = 0;
                    if fpi_byte_reader_get_uint16_le_inline(&mut reader, &mut status)
                        != 0
                    {
                        _g_boolean_var_ = 1 as libc::c_int;
                    } else {
                        _g_boolean_var_ = 0 as libc::c_int;
                    }
                    _g_boolean_var_
                }) as libc::c_long == 0
                {
                    g_log(
                        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_WARNING,
                        b"Transfer in response to version query was too short\0"
                            as *const u8 as *const libc::c_char,
                    );
                    error = fpi_device_error_new(FP_DEVICE_ERROR_PROTO);
                    current_block = 9804548584293845510;
                } else if status as libc::c_int != 0 as libc::c_int {
                    g_log(
                        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_WARNING,
                        b"Device responded with error: %d retry: %d\0" as *const u8
                            as *const libc::c_char,
                        status as libc::c_int,
                        retry,
                    );
                    if retry != 0 {
                        retry = 0 as libc::c_int;
                        current_block = 13183875560443969876;
                    } else {
                        error = fpi_device_error_new(FP_DEVICE_ERROR_PROTO);
                        current_block = 9804548584293845510;
                    }
                } else {
                    read_ok = (read_ok as libc::c_long
                        & ({
                            let mut _g_boolean_var_: libc::c_int = 0;
                            if fpi_byte_reader_get_uint32_le_inline(
                                &mut reader,
                                &mut (*self_0).mis_version.build_time,
                            ) != 0
                            {
                                _g_boolean_var_ = 1 as libc::c_int;
                            } else {
                                _g_boolean_var_ = 0 as libc::c_int;
                            }
                            _g_boolean_var_
                        }) as libc::c_long) as gboolean;
                    read_ok = (read_ok as libc::c_long
                        & ({
                            let mut _g_boolean_var_: libc::c_int = 0;
                            if fpi_byte_reader_get_uint32_le_inline(
                                &mut reader,
                                &mut (*self_0).mis_version.build_num,
                            ) != 0
                            {
                                _g_boolean_var_ = 1 as libc::c_int;
                            } else {
                                _g_boolean_var_ = 0 as libc::c_int;
                            }
                            _g_boolean_var_
                        }) as libc::c_long) as gboolean;
                    read_ok = (read_ok as libc::c_long
                        & ({
                            let mut _g_boolean_var_: libc::c_int = 0;
                            if fpi_byte_reader_get_uint8_inline(
                                &mut reader,
                                &mut (*self_0).mis_version.version_major,
                            ) != 0
                            {
                                _g_boolean_var_ = 1 as libc::c_int;
                            } else {
                                _g_boolean_var_ = 0 as libc::c_int;
                            }
                            _g_boolean_var_
                        }) as libc::c_long) as gboolean;
                    read_ok = (read_ok as libc::c_long
                        & ({
                            let mut _g_boolean_var_: libc::c_int = 0;
                            if fpi_byte_reader_get_uint8_inline(
                                &mut reader,
                                &mut (*self_0).mis_version.version_minor,
                            ) != 0
                            {
                                _g_boolean_var_ = 1 as libc::c_int;
                            } else {
                                _g_boolean_var_ = 0 as libc::c_int;
                            }
                            _g_boolean_var_
                        }) as libc::c_long) as gboolean;
                    read_ok = (read_ok as libc::c_long
                        & ({
                            let mut _g_boolean_var_: libc::c_int = 0;
                            if fpi_byte_reader_get_uint8_inline(
                                &mut reader,
                                &mut (*self_0).mis_version.target,
                            ) != 0
                            {
                                _g_boolean_var_ = 1 as libc::c_int;
                            } else {
                                _g_boolean_var_ = 0 as libc::c_int;
                            }
                            _g_boolean_var_
                        }) as libc::c_long) as gboolean;
                    read_ok = (read_ok as libc::c_long
                        & ({
                            let mut _g_boolean_var_: libc::c_int = 0;
                            if fpi_byte_reader_get_uint8_inline(
                                &mut reader,
                                &mut (*self_0).mis_version.product,
                            ) != 0
                            {
                                _g_boolean_var_ = 1 as libc::c_int;
                            } else {
                                _g_boolean_var_ = 0 as libc::c_int;
                            }
                            _g_boolean_var_
                        }) as libc::c_long) as gboolean;
                    read_ok = (read_ok as libc::c_long
                        & ({
                            let mut _g_boolean_var_: libc::c_int = 0;
                            if fpi_byte_reader_get_uint8_inline(
                                &mut reader,
                                &mut (*self_0).mis_version.silicon_rev,
                            ) != 0
                            {
                                _g_boolean_var_ = 1 as libc::c_int;
                            } else {
                                _g_boolean_var_ = 0 as libc::c_int;
                            }
                            _g_boolean_var_
                        }) as libc::c_long) as gboolean;
                    read_ok = (read_ok as libc::c_long
                        & ({
                            let mut _g_boolean_var_: libc::c_int = 0;
                            if fpi_byte_reader_get_uint8_inline(
                                &mut reader,
                                &mut (*self_0).mis_version.formal_release,
                            ) != 0
                            {
                                _g_boolean_var_ = 1 as libc::c_int;
                            } else {
                                _g_boolean_var_ = 0 as libc::c_int;
                            }
                            _g_boolean_var_
                        }) as libc::c_long) as gboolean;
                    read_ok = (read_ok as libc::c_long
                        & ({
                            let mut _g_boolean_var_: libc::c_int = 0;
                            if fpi_byte_reader_get_uint8_inline(
                                &mut reader,
                                &mut (*self_0).mis_version.platform,
                            ) != 0
                            {
                                _g_boolean_var_ = 1 as libc::c_int;
                            } else {
                                _g_boolean_var_ = 0 as libc::c_int;
                            }
                            _g_boolean_var_
                        }) as libc::c_long) as gboolean;
                    read_ok = (read_ok as libc::c_long
                        & ({
                            let mut _g_boolean_var_: libc::c_int = 0;
                            if fpi_byte_reader_get_uint8_inline(
                                &mut reader,
                                &mut (*self_0).mis_version.patch,
                            ) != 0
                            {
                                _g_boolean_var_ = 1 as libc::c_int;
                            } else {
                                _g_boolean_var_ = 0 as libc::c_int;
                            }
                            _g_boolean_var_
                        }) as libc::c_long) as gboolean;
                    if ({
                        let mut _g_boolean_var_: libc::c_int = 0;
                        if fpi_byte_reader_get_data_inline(
                            &mut reader,
                            ::core::mem::size_of::<[uint8_t; 6]>() as libc::c_ulong
                                as guint,
                            &mut data,
                        ) != 0
                        {
                            _g_boolean_var_ = 1 as libc::c_int;
                        } else {
                            _g_boolean_var_ = 0 as libc::c_int;
                        }
                        _g_boolean_var_
                    }) as libc::c_long != 0
                    {
                        memcpy(
                            ((*self_0).mis_version.serial_number).as_mut_ptr()
                                as *mut libc::c_void,
                            data as *const libc::c_void,
                            ::core::mem::size_of::<[uint8_t; 6]>() as libc::c_ulong,
                        );
                    } else {
                        read_ok = 0 as libc::c_int;
                    }
                    read_ok = (read_ok as libc::c_long
                        & ({
                            let mut _g_boolean_var_: libc::c_int = 0;
                            if fpi_byte_reader_get_uint16_le_inline(
                                &mut reader,
                                &mut (*self_0).mis_version.security,
                            ) != 0
                            {
                                _g_boolean_var_ = 1 as libc::c_int;
                            } else {
                                _g_boolean_var_ = 0 as libc::c_int;
                            }
                            _g_boolean_var_
                        }) as libc::c_long) as gboolean;
                    read_ok = (read_ok as libc::c_long
                        & ({
                            let mut _g_boolean_var_: libc::c_int = 0;
                            if fpi_byte_reader_get_uint8_inline(
                                &mut reader,
                                &mut (*self_0).mis_version.iface,
                            ) != 0
                            {
                                _g_boolean_var_ = 1 as libc::c_int;
                            } else {
                                _g_boolean_var_ = 0 as libc::c_int;
                            }
                            _g_boolean_var_
                        }) as libc::c_long) as gboolean;
                    read_ok = (read_ok as libc::c_long
                        & ({
                            let mut _g_boolean_var_: libc::c_int = 0;
                            if fpi_byte_reader_get_uint8_inline(
                                &mut reader,
                                &mut (*self_0).mis_version.device_type,
                            ) != 0
                            {
                                _g_boolean_var_ = 1 as libc::c_int;
                            } else {
                                _g_boolean_var_ = 0 as libc::c_int;
                            }
                            _g_boolean_var_
                        }) as libc::c_long) as gboolean;
                    if read_ok == 0 {
                        g_log(
                            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                            G_LOG_LEVEL_WARNING,
                            b"Transfer in response to version query was too short\0"
                                as *const u8 as *const libc::c_char,
                        );
                        error = fpi_device_error_new(FP_DEVICE_ERROR_PROTO);
                        current_block = 9804548584293845510;
                    } else {
                        g_log(
                            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                            G_LOG_LEVEL_DEBUG,
                            b"Build Time: %d\0" as *const u8 as *const libc::c_char,
                            (*self_0).mis_version.build_time,
                        );
                        g_log(
                            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                            G_LOG_LEVEL_DEBUG,
                            b"Build Num: %d\0" as *const u8 as *const libc::c_char,
                            (*self_0).mis_version.build_num,
                        );
                        g_log(
                            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                            G_LOG_LEVEL_DEBUG,
                            b"Version: %d.%d\0" as *const u8 as *const libc::c_char,
                            (*self_0).mis_version.version_major as libc::c_int,
                            (*self_0).mis_version.version_minor as libc::c_int,
                        );
                        g_log(
                            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                            G_LOG_LEVEL_DEBUG,
                            b"Target: %d\0" as *const u8 as *const libc::c_char,
                            (*self_0).mis_version.target as libc::c_int,
                        );
                        g_log(
                            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                            G_LOG_LEVEL_DEBUG,
                            b"Product: %d\0" as *const u8 as *const libc::c_char,
                            (*self_0).mis_version.product as libc::c_int,
                        );
                        synaptics_sensor_cmd(
                            self_0,
                            0 as libc::c_int,
                            0x11 as libc::c_int as guint8,
                            0 as *const guint8,
                            0 as libc::c_int as gssize,
                            Some(
                                prob_msg_cb
                                    as unsafe extern "C" fn(
                                        *mut FpiDeviceSynaptics,
                                        *mut bmkt_response_t,
                                        *mut GError,
                                    ) -> (),
                            ),
                        );
                        return;
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn fps_init_msg_cb(
    mut self_0: *mut FpiDeviceSynaptics,
    mut resp: *mut bmkt_response_t,
    mut error: *mut GError,
) {
    if !error.is_null() {
        if g_error_matches(
            error,
            g_io_error_quark(),
            G_IO_ERROR_CANCELLED as libc::c_int,
        ) != 0
        {
            g_clear_error(&mut error);
        }
        fpi_device_open_complete(FP_DEVICE(self_0 as gpointer), error);
        return;
    }
    if (*resp).result == 0 as libc::c_int || (*resp).result == 103 as libc::c_int {
        fpi_device_open_complete(FP_DEVICE(self_0 as gpointer), 0 as *mut GError);
    } else if (*resp).result == 102 as libc::c_int {
        synaptics_sensor_cmd(
            self_0,
            (*self_0).cmd_seq_num as gint,
            0x41 as libc::c_int as guint8,
            0 as *const guint8,
            0 as libc::c_int as gssize,
            None,
        );
    } else {
        g_log(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"Initializing fingerprint sensor failed with %d!\0" as *const u8
                as *const libc::c_char,
            (*resp).result,
        );
        fpi_device_open_complete(
            FP_DEVICE(self_0 as gpointer),
            fpi_device_error_new(FP_DEVICE_ERROR_GENERAL),
        );
    };
}
unsafe extern "C" fn fps_deinit_cb(
    mut self_0: *mut FpiDeviceSynaptics,
    mut resp: *mut bmkt_response_t,
    mut error: *mut GError,
) {
    let mut err: GError_autoptr = 0 as GError_autoptr;
    g_usb_device_release_interface(
        fpi_device_get_usb_device(FP_DEVICE(self_0 as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut err,
    );
    if error.is_null() {
        error = (if 0 as libc::c_int != 0 {
            err as *mut libc::c_void
        } else {
            g_steal_pointer(&mut err as *mut GError_autoptr as gpointer)
        }) as *mut GError;
    }
    let mut _pp: C2RustUnnamed_8 = C2RustUnnamed_8 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp
        .in_0 = &mut (*self_0).interrupt_cancellable as *mut *mut GCancellable
        as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    if error.is_null() {
        match (*resp).response_id {
            162 => {
                g_log(
                    b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"Fingerprint sensor ready to be powered down\0" as *const u8
                        as *const libc::c_char,
                );
            }
            163 => {
                g_log(
                    b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"Failed to go to power down mode: %d\0" as *const u8
                        as *const libc::c_char,
                    (*resp).result,
                );
                error = fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_GENERAL,
                    b"Power down failed: %d\0" as *const u8 as *const libc::c_char,
                    (*resp).result,
                );
            }
            _ => {}
        }
    }
    fpi_device_close_complete(FP_DEVICE(self_0 as gpointer), error);
}
unsafe extern "C" fn dev_init(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceSynaptics = FPI_DEVICE_SYNAPTICS(device as gpointer);
    let mut error: *mut GError = 0 as *mut GError;
    g_log_structured(
        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
            as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"1391\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"dev_init\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/synaptics/synaptics.c:1391\0" as *const u8
            as *const libc::c_char,
    );
    (*self_0).interrupt_cancellable = g_cancellable_new();
    if g_usb_device_claim_interface(
        fpi_device_get_usb_device(device),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    ) == 0
    {
        fpi_device_open_complete(FP_DEVICE(self_0 as gpointer), error);
        return;
    } else {
        synaptics_sensor_cmd(
            self_0,
            0 as libc::c_int,
            0x11 as libc::c_int as guint8,
            0 as *const guint8,
            0 as libc::c_int as gssize,
            Some(
                fps_init_msg_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceSynaptics,
                        *mut bmkt_response_t,
                        *mut GError,
                    ) -> (),
            ),
        );
        return;
    };
}
unsafe extern "C" fn dev_exit(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceSynaptics = FPI_DEVICE_SYNAPTICS(device as gpointer);
    g_log_structured(
        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
            as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"1412\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"dev_exit\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/synaptics/synaptics.c:1412\0" as *const u8
            as *const libc::c_char,
    );
    synaptics_sensor_cmd(
        self_0,
        0 as libc::c_int,
        0xa1 as libc::c_int as guint8,
        0 as *const guint8,
        0 as libc::c_int as gssize,
        Some(
            fps_deinit_cb
                as unsafe extern "C" fn(
                    *mut FpiDeviceSynaptics,
                    *mut bmkt_response_t,
                    *mut GError,
                ) -> (),
        ),
    );
}
unsafe extern "C" fn cancel(mut dev: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceSynaptics = FPI_DEVICE_SYNAPTICS(dev as gpointer);
    synaptics_sensor_cmd(
        self_0,
        -(1 as libc::c_int),
        0x41 as libc::c_int as guint8,
        0 as *const guint8,
        0 as libc::c_int as gssize,
        None,
    );
    g_cancellable_cancel((*self_0).interrupt_cancellable);
    let mut _pp: C2RustUnnamed_6 = C2RustUnnamed_6 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp
        .in_0 = &mut (*self_0).interrupt_cancellable as *mut *mut GCancellable
        as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    (*self_0).interrupt_cancellable = g_cancellable_new();
}
unsafe extern "C" fn suspend(mut dev: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceSynaptics = FPI_DEVICE_SYNAPTICS(dev as gpointer);
    let mut action: FpiDeviceAction = fpi_device_get_current_action(dev);
    g_log(
        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"got suspend request\0" as *const u8 as *const libc::c_char,
    );
    if action as libc::c_uint != FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
        && action as libc::c_uint
            != FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
    {
        fpi_device_suspend_complete(
            dev,
            fpi_device_error_new(FP_DEVICE_ERROR_NOT_SUPPORTED),
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !((*self_0).cmd_ssm).is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
                as *const libc::c_char,
            1448 as libc::c_int,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"suspend\0"))
                .as_ptr(),
            b"self->cmd_ssm\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_ssm_get_cur_state((*self_0).cmd_ssm)
            == SYNAPTICS_CMD_WAIT_INTERRUPT as libc::c_int
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
                as *const libc::c_char,
            1449 as libc::c_int,
            (*::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"suspend\0"))
                .as_ptr(),
            b"fpi_ssm_get_cur_state (self->cmd_ssm) == SYNAPTICS_CMD_WAIT_INTERRUPT\0"
                as *const u8 as *const libc::c_char,
        );
    }
    (*self_0).cmd_suspended = (0 as libc::c_int == 0) as libc::c_int;
    g_cancellable_cancel((*self_0).interrupt_cancellable);
    let mut _pp: C2RustUnnamed_5 = C2RustUnnamed_5 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp
        .in_0 = &mut (*self_0).interrupt_cancellable as *mut *mut GCancellable
        as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    (*self_0).interrupt_cancellable = g_cancellable_new();
}
unsafe extern "C" fn resume(mut dev: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceSynaptics = FPI_DEVICE_SYNAPTICS(dev as gpointer);
    let mut action: FpiDeviceAction = fpi_device_get_current_action(dev);
    g_log(
        b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"got resume request\0" as *const u8 as *const libc::c_char,
    );
    if action as libc::c_uint != FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
        && action as libc::c_uint
            != FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
    {
        g_assertion_message_expr(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
                as *const libc::c_char,
            1469 as libc::c_int,
            (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"resume\0"))
                .as_ptr(),
            0 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !((*self_0).cmd_ssm).is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
                as *const libc::c_char,
            1475 as libc::c_int,
            (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"resume\0"))
                .as_ptr(),
            b"self->cmd_ssm\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*self_0).cmd_suspended != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
                as *const libc::c_char,
            1476 as libc::c_int,
            (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"resume\0"))
                .as_ptr(),
            b"self->cmd_suspended\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_ssm_get_cur_state((*self_0).cmd_ssm)
            == SYNAPTICS_CMD_SUSPENDED as libc::c_int
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-synaptics\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/synaptics/synaptics.c\0" as *const u8
                as *const libc::c_char,
            1477 as libc::c_int,
            (*::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"resume\0"))
                .as_ptr(),
            b"fpi_ssm_get_cur_state (self->cmd_ssm) == SYNAPTICS_CMD_SUSPENDED\0"
                as *const u8 as *const libc::c_char,
        );
    }
    (*self_0).cmd_suspended = 0 as libc::c_int;
    fpi_ssm_jump_to_state((*self_0).cmd_ssm, SYNAPTICS_CMD_RESUME as libc::c_int);
    fpi_device_resume_complete(dev, 0 as *mut GError);
}
unsafe extern "C" fn fpi_device_synaptics_init(mut self_0: *mut FpiDeviceSynaptics) {}
unsafe extern "C" fn fpi_device_synaptics_class_init(
    mut klass: *mut FpiDeviceSynapticsClass,
) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    (*dev_class).id = b"synaptics\0" as *const u8 as *const libc::c_char;
    (*dev_class).full_name = b"Synaptics Sensors\0" as *const u8 as *const libc::c_char;
    (*dev_class).type_0 = FP_DEVICE_TYPE_USB;
    (*dev_class).scan_type = FP_SCAN_TYPE_PRESS;
    (*dev_class).id_table = id_table.as_ptr();
    (*dev_class).nr_enroll_stages = 8 as libc::c_int;
    (*dev_class).temp_hot_seconds = -(1 as libc::c_int);
    (*dev_class).open = Some(dev_init as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class).close = Some(dev_exit as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class).probe = Some(dev_probe as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class).verify = Some(verify as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class).identify = Some(identify as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class).enroll = Some(enroll as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .delete = Some(delete_print as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .clear_storage = Some(
        clear_storage as unsafe extern "C" fn(*mut FpDevice) -> (),
    );
    (*dev_class).cancel = Some(cancel as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class).suspend = Some(suspend as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class).resume = Some(resume as unsafe extern "C" fn(*mut FpDevice) -> ());
    fpi_device_class_auto_initialize_features(dev_class);
}
