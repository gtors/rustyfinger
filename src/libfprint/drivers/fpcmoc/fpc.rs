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
    fn g_ptr_array_sized_new(reserved_size: guint) -> *mut GPtrArray;
    fn g_ptr_array_ref(array: *mut GPtrArray) -> *mut GPtrArray;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_free(mem: gpointer);
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
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
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
    fn g_object_set(object: gpointer, first_property_name: *const gchar, _: ...);
    fn g_object_get(object: gpointer, first_property_name: *const gchar, _: ...);
    fn g_object_ref_sink(object: gpointer) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_cancellable_new() -> *mut GCancellable;
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
    fn g_io_error_quark() -> GQuark;
    fn g_usb_device_get_pid(self_0: *mut GUsbDevice) -> guint16;
    fn g_usb_device_get_product_index(self_0: *mut GUsbDevice) -> guint8;
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
    fn fpi_ssm_get_error(machine: *mut FpiSsm) -> *mut GError;
    fn fpi_ssm_get_cur_state(machine: *mut FpiSsm) -> libc::c_int;
}
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
pub type GPtrArray_autoptr = *mut GPtrArray;
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
pub type FpiSsmCompletedCallback = Option::<
    unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
>;
pub type FpiSsmHandlerCallback = Option::<
    unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
>;
pub type FpiDeviceFpcMoc = _FpiDeviceFpcMoc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceFpcMoc {
    pub parent: FpDevice,
    pub task_ssm: *mut FpiSsm,
    pub cmd_ssm: *mut FpiSsm,
    pub cmd_suspended: gboolean,
    pub enroll_stage: gint,
    pub immobile_stage: gint,
    pub max_enroll_stage: gint,
    pub max_immobile_stage: gint,
    pub max_stored_prints: gint,
    pub cmd_data_timeout: guint,
    pub dbid: *mut guint8,
    pub do_cleanup: gboolean,
    pub interrupt_cancellable: *mut GCancellable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiDeviceFpcMocClass {
    pub parent_class: FpDeviceClass,
}
pub const FP_CLEAR_NUM_STATES: C2RustUnnamed_17 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandData {
    pub cmdtype: FpcCmdType,
    pub request: guint8,
    pub value: guint16,
    pub index: guint16,
    pub data: *mut guint8,
    pub data_len: gsize,
    pub callback: SynCmdMsgCallback,
}
pub type SynCmdMsgCallback = Option::<
    unsafe extern "C" fn(*mut FpiDeviceFpcMoc, *mut libc::c_void, *mut GError) -> (),
>;
pub type FpcCmdType = libc::c_uint;
pub const FPC_CMDTYPE_FROM_DEVICE: FpcCmdType = 3;
pub const FPC_CMDTYPE_TO_DEVICE_EVTDATA: FpcCmdType = 2;
pub const FPC_CMDTYPE_TO_DEVICE: FpcCmdType = 1;
pub const FPC_CMDTYPE_UNKNOWN: FpcCmdType = 0;
pub const FP_CMD_NUM_STATES: C2RustUnnamed_13 = 4;
pub const FP_CMD_GET_DATA: C2RustUnnamed_13 = 1;
pub const FP_CMD_RESUME: C2RustUnnamed_13 = 3;
pub const FP_CMD_SUSPENDED: C2RustUnnamed_13 = 2;
pub type fpc_cmd_response_t = _fp_cmd_response;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fp_cmd_response {
    pub c2rust_unnamed: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub evt_hdr: evt_hdr_t,
    pub evt_inited: evt_initiated_t,
    pub evt_enum_fids: evt_enum_fids_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct evt_enum_fids_t {
    pub hdr: evt_hdr_t,
    pub status: gint,
    pub num_ids: guint32,
    pub fid_data: [fpc_fid_data_t; 10],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct fpc_fid_data_t {
    pub subfactor: guint8,
    pub identity_type: guint32,
    pub identity_size: guint32,
    pub identity: [guint8; 68],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evt_hdr_t {
    pub cmdid: guint32,
    pub length: guint32,
    pub status: guint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evt_initiated_t {
    pub hdr: evt_hdr_t,
    pub sensor: guint16,
    pub hw_id: guint16,
    pub img_w: guint16,
    pub img_h: guint16,
    pub fw_version: [guint8; 16],
    pub fw_capabilities: guint16,
}
pub const FP_CMD_SEND: C2RustUnnamed_13 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub in_0: *mut libc::c_char,
    pub out: *mut *mut GObject,
}
pub type FPC_DB_OP = _FPC_DELETE_DB;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _FPC_DELETE_DB {
    pub reserved: guint32,
    pub database_id_size: guint32,
    pub data: [guint8; 16],
}
pub const FP_CLEAR_CREATE_DB: C2RustUnnamed_17 = 1;
pub const FP_CLEAR_DELETE_DB: C2RustUnnamed_17 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub in_0: *mut libc::c_char,
    pub out: *mut *mut GObject,
}
pub const FP_VERIFY_CANCEL: C2RustUnnamed_16 = 3;
pub const FP_VERIFY_NUM_STATES: C2RustUnnamed_16 = 4;
pub type FPC_IDENTIFY = _FPC_IDENTIFY;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FPC_IDENTIFY {
    pub status: gint32,
    pub identity_type: guint32,
    pub identity_offset: guint32,
    pub identity_size: guint32,
    pub subfactor: guint32,
    pub data: [guint8; 68],
}
pub const FP_VERIFY_IDENTIFY: C2RustUnnamed_16 = 2;
pub type pfpc_cmd_response_t = *mut _fp_cmd_response;
pub const FP_VERIFY_GET_IMG: C2RustUnnamed_16 = 1;
pub const FP_VERIFY_CAPTURE: C2RustUnnamed_16 = 0;
pub type FPC_FID_DATA = _FPC_FID_DATA;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FPC_FID_DATA {
    pub identity_type: guint32,
    pub reserved: guint32,
    pub identity_size: guint32,
    pub subfactor: guint32,
    pub data: [guint8; 68],
}
pub const FP_ENROLL_DICARD: C2RustUnnamed_15 = 9;
pub const FP_ENROLL_NUM_STATES: C2RustUnnamed_15 = 11;
pub type FPC_END_ENROL = _FPC_END_ENROL;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FPC_END_ENROL {
    pub status: gint32,
    pub fid: guint32,
}
pub const FP_ENROLL_CLEANUP: C2RustUnnamed_15 = 10;
pub const FP_ENROLL_COMMIT: C2RustUnnamed_15 = 8;
pub const FP_ENROLL_BINDID: C2RustUnnamed_15 = 7;
pub const FP_ENROLL_CHECK_DUPLICATE: C2RustUnnamed_15 = 6;
pub const FP_ENROLL_COMPLETE: C2RustUnnamed_15 = 5;
pub const FP_ENROLL_CAPTURE: C2RustUnnamed_15 = 2;
pub type FPC_ENROL = _FPC_ENROL;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FPC_ENROL {
    pub status: gint32,
    pub remaining: guint32,
}
pub const FPC_ENROL_STATUS_IMAGE_LOW_QUALITY: C2RustUnnamed_12 = 6;
pub const FPC_ENROL_STATUS_IMAGE_LOW_COVERAGE: C2RustUnnamed_12 = 4;
pub const FPC_ENROL_STATUS_PROGRESS: C2RustUnnamed_12 = 1;
pub const FPC_ENROL_STATUS_IMAGE_TOO_SIMILAR: C2RustUnnamed_12 = 5;
pub const FPC_ENROL_STATUS_COMPLETED: C2RustUnnamed_12 = 0;
pub const FPC_ENROL_STATUS_FAILED_ALREADY_ENROLED: C2RustUnnamed_12 = 3;
pub const FPC_ENROL_STATUS_FAILED_COULD_NOT_COMPLETE: C2RustUnnamed_12 = 2;
pub const FP_ENROLL_UPDATE: C2RustUnnamed_15 = 4;
pub const FP_ENROLL_GET_IMG: C2RustUnnamed_15 = 3;
pub type FPC_BEGIN_ENROL = _FPC_BEGIN_ENROL;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FPC_BEGIN_ENROL {
    pub status: gint32,
    pub reserved1: guint32,
    pub reserved2: guint32,
}
pub const FP_ENROLL_CREATE: C2RustUnnamed_15 = 1;
pub const FP_ENROLL_ENUM: C2RustUnnamed_15 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub const FP_INIT_NUM_STATES: C2RustUnnamed_14 = 2;
pub type FPC_LOAD_DB = _FPC_LOAD_DB;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FPC_LOAD_DB {
    pub status: gint32,
    pub reserved: guint32,
    pub database_id_size: guint32,
    pub data: [guint8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub const FP_LOAD_DB: C2RustUnnamed_14 = 1;
pub const FP_INIT: C2RustUnnamed_14 = 0;
pub type C2RustUnnamed_12 = libc::c_uint;
pub type C2RustUnnamed_13 = libc::c_uint;
pub type C2RustUnnamed_14 = libc::c_uint;
pub type C2RustUnnamed_15 = libc::c_uint;
pub type C2RustUnnamed_16 = libc::c_uint;
pub type C2RustUnnamed_17 = libc::c_uint;
#[inline]
unsafe extern "C" fn g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = 0 as *mut libc::c_void;
    ref_0 = *ptr;
    *ptr = 0 as *mut libc::c_void;
    return ref_0;
}
#[inline]
unsafe extern "C" fn g_set_object(
    mut object_ptr: *mut *mut GObject,
    mut new_object: *mut GObject,
) -> gboolean {
    let mut old_object: *mut GObject = *object_ptr;
    if old_object == new_object {
        return 0 as libc::c_int;
    }
    if !new_object.is_null() {
        g_object_ref(new_object as gpointer);
    }
    *object_ptr = new_object;
    if !old_object.is_null() {
        g_object_unref(old_object as gpointer);
    }
    return (0 as libc::c_int == 0) as libc::c_int;
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
unsafe extern "C" fn FPI_DEVICE_FPCMOC(mut ptr: gpointer) -> *mut FpiDeviceFpcMoc {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_fpcmoc_get_type(),
    ) as *mut libc::c_void as *mut FpiDeviceFpcMoc;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_device_fpcmoc_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_fpcmoc_get_type_once();
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
unsafe extern "C" fn fpi_device_fpcmoc_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_device_get_type(),
        g_intern_static_string(b"FpiDeviceFpcMoc\0" as *const u8 as *const libc::c_char),
        ::core::mem::size_of::<FpiDeviceFpcMocClass>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_fpcmoc_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpiDeviceFpcMoc>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpiDeviceFpcMoc) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_fpcmoc_init
                        as unsafe extern "C" fn(*mut FpiDeviceFpcMoc) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn fpi_device_fpcmoc_class_intern_init(mut klass: gpointer) {
    fpi_device_fpcmoc_parent_class = g_type_class_peek_parent(klass);
    if FpiDeviceFpcMoc_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpiDeviceFpcMoc_private_offset);
    }
    fpi_device_fpcmoc_class_init(klass as *mut FpiDeviceFpcMocClass);
}
static mut FpiDeviceFpcMoc_private_offset: gint = 0;
static mut fpi_device_fpcmoc_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut id_table: [FpIdEntry; 6] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_1 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_4 {
                        pid: 0xffe0 as libc::c_int as guint,
                        vid: 0x10a5 as libc::c_int as guint,
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
                        pid: 0xa305 as libc::c_int as guint,
                        vid: 0x10a5 as libc::c_int as guint,
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
                        pid: 0xda04 as libc::c_int as guint,
                        vid: 0x10a5 as libc::c_int as guint,
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
                        pid: 0xd805 as libc::c_int as guint,
                        vid: 0x10a5 as libc::c_int as guint,
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
                        pid: 0xd205 as libc::c_int as guint,
                        vid: 0x10a5 as libc::c_int as guint,
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
unsafe extern "C" fn fpc_suspend_resume_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut ssm_state: libc::c_int = fpi_ssm_get_cur_state((*transfer).ssm);
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s current ssm state: %d\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"fpc_suspend_resume_cb\0"))
            .as_ptr(),
        ssm_state,
    );
    if ssm_state == FP_CMD_SUSPENDED as libc::c_int {
        if !error.is_null() {
            fpi_ssm_mark_failed((*transfer).ssm, error);
        }
        fpi_device_suspend_complete(device, error);
    } else if ssm_state == FP_CMD_RESUME as libc::c_int {
        if !error.is_null() {
            fpi_ssm_mark_failed((*transfer).ssm, error);
        } else {
            fpi_ssm_jump_to_state((*transfer).ssm, FP_CMD_GET_DATA as libc::c_int);
        }
        fpi_device_resume_complete(device, error);
    }
}
unsafe extern "C" fn fpc_cmd_receive_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(device as gpointer);
    let mut data: *mut CommandData = user_data as *mut CommandData;
    let mut ssm_state: libc::c_int = 0 as libc::c_int;
    if g_error_matches(error, g_io_error_quark(), G_IO_ERROR_CANCELLED as libc::c_int)
        != 0 && (*self_0).cmd_suspended != 0
    {
        g_error_free(error);
        fpi_ssm_jump_to_state((*transfer).ssm, FP_CMD_SUSPENDED as libc::c_int);
        return;
    }
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
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s current ssm state: %d\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"fpc_cmd_receive_cb\0"))
            .as_ptr(),
        ssm_state,
    );
    if (*data).cmdtype as libc::c_uint
        == FPC_CMDTYPE_TO_DEVICE as libc::c_int as libc::c_uint
    {
        if ((*data).callback).is_some() {
            ((*data).callback)
                .expect(
                    "non-null function pointer",
                )(self_0, 0 as *mut libc::c_void, 0 as *mut GError);
        }
        fpi_ssm_mark_completed((*transfer).ssm);
        return;
    } else {
        if (*data).cmdtype as libc::c_uint
            == FPC_CMDTYPE_TO_DEVICE_EVTDATA as libc::c_int as libc::c_uint
        {
            if ssm_state == FP_CMD_SEND as libc::c_int {
                fpi_ssm_next_state((*transfer).ssm);
                return;
            }
            if ssm_state == FP_CMD_GET_DATA as libc::c_int {
                let mut evt_data: fpc_cmd_response_t = {
                    let mut init = _fp_cmd_response {
                        c2rust_unnamed: C2RustUnnamed_5 {
                            evt_hdr: {
                                let mut init = evt_hdr_t {
                                    cmdid: 0 as libc::c_int as guint32,
                                    length: 0,
                                    status: 0,
                                };
                                init
                            },
                        },
                    };
                    init
                };
                g_log(
                    b"libfprint\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"%s recv evt data length: %ld\0" as *const u8
                        as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 19],
                        &[libc::c_char; 19],
                    >(b"fpc_cmd_receive_cb\0"))
                        .as_ptr(),
                    (*transfer).actual_length,
                );
                if (*transfer).actual_length == 0 as libc::c_int as libc::c_long {
                    g_log(
                        b"libfprint\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_CRITICAL,
                        b"%s Expect data but actual_length = 0\0" as *const u8
                            as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 19],
                            &[libc::c_char; 19],
                        >(b"fpc_cmd_receive_cb\0"))
                            .as_ptr(),
                    );
                    fpi_ssm_mark_failed(
                        (*transfer).ssm,
                        fpi_device_error_new(FP_DEVICE_ERROR_DATA_INVALID),
                    );
                    return;
                }
                memcpy(
                    &mut evt_data as *mut fpc_cmd_response_t as *mut libc::c_void,
                    (*transfer).buffer as *const libc::c_void,
                    (*transfer).actual_length as libc::c_ulong,
                );
                if ((*data).callback).is_some() {
                    ((*data).callback)
                        .expect(
                            "non-null function pointer",
                        )(
                        self_0,
                        &mut evt_data as *mut fpc_cmd_response_t as *mut guint8
                            as *mut libc::c_void,
                        0 as *mut GError,
                    );
                }
                fpi_ssm_mark_completed((*transfer).ssm);
                return;
            }
        } else if (*data).cmdtype as libc::c_uint
            == FPC_CMDTYPE_FROM_DEVICE as libc::c_int as libc::c_uint
        {
            if (*transfer).actual_length == 0 as libc::c_int as libc::c_long {
                g_log(
                    b"libfprint\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_CRITICAL,
                    b"%s Expect data but actual_length = 0\0" as *const u8
                        as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 19],
                        &[libc::c_char; 19],
                    >(b"fpc_cmd_receive_cb\0"))
                        .as_ptr(),
                );
                fpi_ssm_mark_failed(
                    (*transfer).ssm,
                    fpi_device_error_new(FP_DEVICE_ERROR_DATA_INVALID),
                );
                return;
            }
            if ((*data).callback).is_some() {
                ((*data).callback)
                    .expect(
                        "non-null function pointer",
                    )(self_0, (*transfer).buffer as *mut libc::c_void, 0 as *mut GError);
            }
            fpi_ssm_mark_completed((*transfer).ssm);
            return;
        } else {
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"%s incorrect cmdtype (%x) \0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 19],
                    &[libc::c_char; 19],
                >(b"fpc_cmd_receive_cb\0"))
                    .as_ptr(),
                (*data).cmdtype as libc::c_uint,
            );
            fpi_ssm_mark_failed(
                (*transfer).ssm,
                fpi_device_error_new(FP_DEVICE_ERROR_DATA_INVALID),
            );
            return;
        }
    }
    fpi_ssm_mark_failed((*transfer).ssm, fpi_device_error_new(FP_DEVICE_ERROR_GENERAL));
}
unsafe extern "C" fn fpc_send_ctrl_cmd(mut dev: *mut FpDevice) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(dev as gpointer);
    let mut cmd_data: *mut CommandData = fpi_ssm_get_data((*self_0).cmd_ssm)
        as *mut CommandData;
    let mut cmdtype: FpcCmdType = FPC_CMDTYPE_UNKNOWN;
    if cmd_data.is_null() {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s No cmd_data is set \0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"fpc_send_ctrl_cmd\0"))
                .as_ptr(),
        );
        fpi_ssm_mark_failed(
            (*self_0).cmd_ssm,
            fpi_device_error_new(FP_DEVICE_ERROR_GENERAL),
        );
    }
    cmdtype = (*cmd_data).cmdtype;
    if cmdtype as libc::c_uint != FPC_CMDTYPE_FROM_DEVICE as libc::c_int as libc::c_uint
        && (*cmd_data).data_len != 0 && ((*cmd_data).data).is_null()
    {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s data buffer is null but len is not! \0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"fpc_send_ctrl_cmd\0"))
                .as_ptr(),
        );
        fpi_ssm_mark_failed(
            (*self_0).cmd_ssm,
            fpi_device_error_new(FP_DEVICE_ERROR_GENERAL),
        );
    }
    if cmdtype as libc::c_uint == FPC_CMDTYPE_UNKNOWN as libc::c_int as libc::c_uint {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s unknown cmd type \0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"fpc_send_ctrl_cmd\0"))
                .as_ptr(),
        );
        fpi_ssm_mark_failed(
            (*self_0).cmd_ssm,
            fpi_device_error_new(FP_DEVICE_ERROR_DATA_INVALID),
        );
    }
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s CMD: 0x%x, value: 0x%x, index: %x type: %d\0" as *const u8
            as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"fpc_send_ctrl_cmd\0"))
            .as_ptr(),
        (*cmd_data).request as libc::c_int,
        (*cmd_data).value as libc::c_int,
        (*cmd_data).index as libc::c_int,
        cmdtype as libc::c_uint,
    );
    transfer = fpi_usb_transfer_new(dev);
    fpi_usb_transfer_fill_control(
        transfer,
        (if cmdtype as libc::c_uint
            == FPC_CMDTYPE_FROM_DEVICE as libc::c_int as libc::c_uint
        {
            G_USB_DEVICE_DIRECTION_DEVICE_TO_HOST as libc::c_int
        } else {
            G_USB_DEVICE_DIRECTION_HOST_TO_DEVICE as libc::c_int
        }) as GUsbDeviceDirection,
        G_USB_DEVICE_REQUEST_TYPE_VENDOR,
        G_USB_DEVICE_RECIPIENT_DEVICE,
        (*cmd_data).request,
        (*cmd_data).value,
        (*cmd_data).index,
        (*cmd_data).data_len,
    );
    (*transfer).ssm = (*self_0).cmd_ssm;
    if cmdtype as libc::c_uint != FPC_CMDTYPE_FROM_DEVICE as libc::c_int as libc::c_uint
        && !((*cmd_data).data).is_null()
        && (*cmd_data).data_len != 0 as libc::c_int as libc::c_ulong
    {
        memcpy(
            (*transfer).buffer as *mut libc::c_void,
            (*cmd_data).data as *const libc::c_void,
            (*cmd_data).data_len,
        );
    }
    fpi_usb_transfer_submit(
        transfer,
        1000 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            fpc_cmd_receive_cb
                as unsafe extern "C" fn(
                    *mut FpiUsbTransfer,
                    *mut FpDevice,
                    gpointer,
                    *mut GError,
                ) -> (),
        ),
        fpi_ssm_get_data((*transfer).ssm),
    );
}
unsafe extern "C" fn fpc_cmd_ssm_done(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(dev as gpointer);
    let mut data: *mut CommandData = fpi_ssm_get_data(ssm) as *mut CommandData;
    if !error.is_null() {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s error: %s \0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"fpc_cmd_ssm_done\0"))
                .as_ptr(),
            (*error).message,
        );
        if ((*data).callback).is_some() {
            ((*data).callback)
                .expect(
                    "non-null function pointer",
                )(self_0, 0 as *mut libc::c_void, error);
        }
    }
    (*self_0).cmd_ssm = 0 as *mut FpiSsm;
}
unsafe extern "C" fn fpc_cmd_run_state(mut ssm: *mut FpiSsm, mut dev: *mut FpDevice) {
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            fpc_send_ctrl_cmd(dev);
        }
        1 => {
            transfer = fpi_usb_transfer_new(dev);
            (*transfer).ssm = ssm;
            fpi_usb_transfer_fill_bulk(
                transfer,
                (1 as libc::c_int | 0x80 as libc::c_int) as guint8,
                2048 as libc::c_int as gsize,
            );
            fpi_usb_transfer_submit(
                transfer,
                (*self_0).cmd_data_timeout,
                (*self_0).interrupt_cancellable,
                Some(
                    fpc_cmd_receive_cb
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
            fpi_usb_transfer_fill_control(
                transfer,
                G_USB_DEVICE_DIRECTION_HOST_TO_DEVICE,
                G_USB_DEVICE_REQUEST_TYPE_VENDOR,
                G_USB_DEVICE_RECIPIENT_DEVICE,
                0x8 as libc::c_int as guint8,
                0x11 as libc::c_int as guint16,
                0 as libc::c_int as guint16,
                0 as libc::c_int as gsize,
            );
            fpi_usb_transfer_submit(
                transfer,
                1000 as libc::c_int as guint,
                0 as *mut GCancellable,
                Some(
                    fpc_suspend_resume_cb
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
            fpi_usb_transfer_fill_control(
                transfer,
                G_USB_DEVICE_DIRECTION_HOST_TO_DEVICE,
                G_USB_DEVICE_REQUEST_TYPE_VENDOR,
                G_USB_DEVICE_RECIPIENT_DEVICE,
                0x8 as libc::c_int as guint8,
                0x10 as libc::c_int as guint16,
                0 as libc::c_int as guint16,
                0 as libc::c_int as gsize,
            );
            fpi_usb_transfer_submit(
                transfer,
                1000 as libc::c_int as guint,
                0 as *mut GCancellable,
                Some(
                    fpc_suspend_resume_cb
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
unsafe extern "C" fn fpc_sensor_cmd(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut wait_data_delay: gboolean,
    mut cmd_data: *mut CommandData,
) {
    let mut data: *mut CommandData = 0 as *mut CommandData;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !cmd_data.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"fpc_sensor_cmd\0"))
                .as_ptr(),
            b"cmd_data\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*cmd_data).cmdtype as libc::c_uint
            != FPC_CMDTYPE_UNKNOWN as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"fpc_sensor_cmd\0"))
                .as_ptr(),
            b"cmd_data->cmdtype != FPC_CMDTYPE_UNKNOWN\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    data = ({
        g_memdup2(
            cmd_data as gconstpointer,
            ::core::mem::size_of::<CommandData>() as libc::c_ulong,
        )
    }) as *mut CommandData;
    if wait_data_delay != 0 {
        (*self_0).cmd_data_timeout = 0 as libc::c_int as guint;
        let mut _object_ptr: C2RustUnnamed_7 = C2RustUnnamed_7 {
            in_0: 0 as *mut libc::c_char,
        };
        _object_ptr
            .in_0 = &mut (*self_0).interrupt_cancellable as *mut *mut GCancellable
            as *mut libc::c_char;
        if 0 as libc::c_int != 0 {
            (*self_0).interrupt_cancellable = g_cancellable_new();
        } else {};
        g_set_object(_object_ptr.out, g_cancellable_new() as *mut GObject);
    } else {
        (*self_0).cmd_data_timeout = 5000 as libc::c_int as guint;
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
    }
    (*self_0)
        .cmd_ssm = fpi_ssm_new_full(
        FP_DEVICE(self_0 as gpointer),
        Some(
            fpc_cmd_run_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        FP_CMD_NUM_STATES as libc::c_int,
        FP_CMD_NUM_STATES as libc::c_int,
        b"FP_CMD_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_set_data(
        (*self_0).cmd_ssm,
        data as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    fpi_ssm_start(
        (*self_0).cmd_ssm,
        Some(
            fpc_cmd_ssm_done
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn fpc_dev_release_interface(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut error: *mut GError,
) {
    let mut release_error: GError_autoptr = 0 as GError_autoptr;
    g_usb_device_release_interface(
        fpi_device_get_usb_device(FP_DEVICE(self_0 as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut release_error,
    );
    if !error.is_null() {
        fpi_device_close_complete(FP_DEVICE(self_0 as gpointer), error);
        return;
    }
    fpi_device_close_complete(FP_DEVICE(self_0 as gpointer), release_error);
}
unsafe extern "C" fn check_data(
    mut data: *mut libc::c_void,
    mut error: *mut *mut GError,
) -> gboolean {
    if !(*error).is_null() {
        return 0 as libc::c_int;
    }
    if data.is_null() {
        g_propagate_error(error, fpi_device_error_new(FP_DEVICE_ERROR_DATA_INVALID));
        return 0 as libc::c_int;
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
unsafe extern "C" fn fpc_evt_cb(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut presp: pfpc_cmd_response_t = 0 as pfpc_cmd_response_t;
    if check_data(data, &mut error) == 0 {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s error: %s\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"fpc_evt_cb\0"))
                .as_ptr(),
            (*error).message,
        );
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    presp = data as pfpc_cmd_response_t;
    match (*presp).c2rust_unnamed.evt_hdr.cmdid {
        49 => {
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"%s Enum Fids: status = %d, NumFids = %d\0" as *const u8
                    as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"fpc_evt_cb\0"))
                    .as_ptr(),
                (*presp).c2rust_unnamed.evt_enum_fids.status,
                (*presp).c2rust_unnamed.evt_enum_fids.num_ids,
            );
            if (*presp).c2rust_unnamed.evt_enum_fids.status != 0
                || (*presp).c2rust_unnamed.evt_enum_fids.num_ids
                    > 10 as libc::c_int as libc::c_uint
            {
                fpi_ssm_mark_failed(
                    (*self_0).task_ssm,
                    fpi_device_error_new_msg(
                        FP_DEVICE_ERROR_DATA_INVALID,
                        b"Get Fids failed\0" as *const u8 as *const libc::c_char,
                    ),
                );
                return;
            }
        }
        2 => {
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"%s INIT: status=%d, Sensor = %d, HWID = 0x%04X, WxH = %d x %d\0"
                    as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"fpc_evt_cb\0"))
                    .as_ptr(),
                (*presp).c2rust_unnamed.evt_inited.hdr.status,
                (*presp).c2rust_unnamed.evt_inited.sensor as libc::c_int,
                (*presp).c2rust_unnamed.evt_inited.hw_id as libc::c_int,
                (*presp).c2rust_unnamed.evt_inited.img_w as libc::c_int,
                (*presp).c2rust_unnamed.evt_inited.img_h as libc::c_int,
            );
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"%s INIT: FW version: %s\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"fpc_evt_cb\0"))
                    .as_ptr(),
                ((*presp).c2rust_unnamed.evt_inited.fw_version).as_mut_ptr()
                    as *mut gchar,
            );
        }
        6 => {
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"%s Got finger down event\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"fpc_evt_cb\0"))
                    .as_ptr(),
            );
            fpi_device_report_finger_status_changes(
                FP_DEVICE(self_0 as gpointer),
                FP_FINGER_STATUS_PRESENT,
                FP_FINGER_STATUS_NONE,
            );
        }
        8 => {
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"%s Got capture event\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"fpc_evt_cb\0"))
                    .as_ptr(),
            );
            fpi_device_report_finger_status_changes(
                FP_DEVICE(self_0 as gpointer),
                FP_FINGER_STATUS_NONE,
                FP_FINGER_STATUS_PRESENT,
            );
        }
        _ => {
            fpi_ssm_mark_failed(
                (*self_0).task_ssm,
                fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_GENERAL,
                    b"Unknown Evt (0x%x)!\0" as *const u8 as *const libc::c_char,
                    (*presp).c2rust_unnamed.evt_hdr.cmdid,
                ),
            );
            return;
        }
    }
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn fpc_do_abort_cb(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut data: *mut libc::c_void,
    mut error: *mut GError,
) {
    if !error.is_null() {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s error: %s \0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"fpc_do_abort_cb\0"))
                .as_ptr(),
            (*error).message,
        );
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s Do abort for reasons\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"fpc_do_abort_cb\0"))
            .as_ptr(),
    );
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn fpc_do_cleanup_cb(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut data: *mut libc::c_void,
    mut error: *mut GError,
) {
    if !error.is_null() {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s error: %s \0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"fpc_do_cleanup_cb\0"))
                .as_ptr(),
            (*error).message,
        );
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s Do cleanup for reasons\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"fpc_do_cleanup_cb\0"))
            .as_ptr(),
    );
    (*self_0).do_cleanup = 0 as libc::c_int;
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn fpc_template_delete_cb(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut resp: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    fpi_device_delete_complete(device, error);
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
            b"libfprint\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"parse_print_data\0"))
                .as_ptr(),
            b"data\0" as *const u8 as *const libc::c_char,
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
            b"libfprint\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"parse_print_data\0"))
                .as_ptr(),
            b"finger\0" as *const u8 as *const libc::c_char,
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
            b"libfprint\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"parse_print_data\0"))
                .as_ptr(),
            b"user_id\0" as *const u8 as *const libc::c_char,
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
            b"libfprint\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"parse_print_data\0"))
                .as_ptr(),
            b"user_id_len\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    *user_id = 0 as *const guint8;
    *user_id_len = 0 as libc::c_int as gsize;
    *finger = 0 as libc::c_int as guint8;
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
        || *user_id_len > 68 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if *user_id_len <= 0 as libc::c_int as libc::c_ulong
        || **user_id.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || **user_id.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
    {
        return 0 as libc::c_int;
    }
    if *finger as libc::c_int != 0xf5 as libc::c_int {
        return 0 as libc::c_int;
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
unsafe extern "C" fn fpc_print_from_data(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut fid_data: *mut fpc_fid_data_t,
) -> *mut FpPrint {
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    let mut data: *mut GVariant = 0 as *mut GVariant;
    let mut uid: *mut GVariant = 0 as *mut GVariant;
    let mut userid: *mut gchar = 0 as *mut gchar;
    userid = g_strndup(
        ((*fid_data).identity).as_mut_ptr() as *mut gchar,
        (*fid_data).identity_size as gsize,
    );
    print = fp_print_new(FP_DEVICE(self_0 as gpointer));
    uid = g_variant_new_fixed_array(
        b"y\0" as *const u8 as *const libc::c_char as *const GVariantType,
        ((*fid_data).identity).as_mut_ptr() as gconstpointer,
        (*fid_data).identity_size as gsize,
        1 as libc::c_int as gsize,
    );
    data = g_variant_new(
        b"(y@ay)\0" as *const u8 as *const libc::c_char,
        (*fid_data).subfactor as libc::c_int,
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
unsafe extern "C" fn fpc_template_list_cb(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut list_result: GPtrArray_autoptr = 0 as GPtrArray_autoptr;
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    let mut presp: pfpc_cmd_response_t = 0 as pfpc_cmd_response_t;
    if !error.is_null() {
        fpi_device_list_complete(
            FP_DEVICE(self_0 as gpointer),
            0 as *mut GPtrArray,
            error,
        );
        return;
    }
    if data.is_null() {
        fpi_device_list_complete(
            FP_DEVICE(self_0 as gpointer),
            0 as *mut GPtrArray,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_DATA_INVALID,
                b"Data is null\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    presp = data as pfpc_cmd_response_t;
    if (*presp).c2rust_unnamed.evt_hdr.cmdid != 0x31 as libc::c_int as libc::c_uint {
        fpi_device_list_complete(
            FP_DEVICE(self_0 as gpointer),
            0 as *mut GPtrArray,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_DATA_INVALID,
                b"Recv evt is incorrect: 0x%x\0" as *const u8 as *const libc::c_char,
                (*presp).c2rust_unnamed.evt_hdr.cmdid,
            ),
        );
        return;
    }
    if (*presp).c2rust_unnamed.evt_enum_fids.num_ids > 10 as libc::c_int as libc::c_uint
    {
        fpi_device_list_complete(
            FP_DEVICE(self_0 as gpointer),
            0 as *mut GPtrArray,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_DATA_FULL,
                b"Database is full\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    list_result = g_ptr_array_new_with_free_func(
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    if (*presp).c2rust_unnamed.evt_enum_fids.num_ids == 0 as libc::c_int as libc::c_uint
    {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Database is empty\0" as *const u8 as *const libc::c_char,
        );
        fpi_device_list_complete(
            device,
            (if 0 as libc::c_int != 0 {
                list_result as *mut libc::c_void
            } else {
                g_steal_pointer(&mut list_result as *mut GPtrArray_autoptr as gpointer)
            }) as *mut GPtrArray,
            0 as *mut GError,
        );
        return;
    }
    let mut n: libc::c_int = 0 as libc::c_int;
    while (n as libc::c_uint) < (*presp).c2rust_unnamed.evt_enum_fids.num_ids {
        let mut print: *mut FpPrint = 0 as *mut FpPrint;
        let mut fid_data: *mut fpc_fid_data_t = &mut *((*presp)
            .c2rust_unnamed
            .evt_enum_fids
            .fid_data)
            .as_mut_ptr()
            .offset(n as isize) as *mut fpc_fid_data_t;
        if (*fid_data).subfactor as libc::c_int != 0xf5 as libc::c_int
            && (*fid_data).identity_type != 0x3 as libc::c_int as libc::c_uint
        {
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Incompatible template found (0x%x, 0x%x)\0" as *const u8
                    as *const libc::c_char,
                (*fid_data).subfactor as libc::c_int,
                (*fid_data).identity_type,
            );
        } else {
            print = fpc_print_from_data(self_0, fid_data);
            g_ptr_array_add(
                list_result,
                g_object_ref_sink(print as gpointer) as *mut FpPrint as gpointer,
            );
        }
        n += 1;
    }
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Query templates complete!\0" as *const u8 as *const libc::c_char,
    );
    fpi_device_list_complete(
        device,
        (if 0 as libc::c_int != 0 {
            list_result as *mut libc::c_void
        } else {
            g_steal_pointer(&mut list_result as *mut GPtrArray_autoptr as gpointer)
        }) as *mut GPtrArray,
        0 as *mut GError,
    );
}
unsafe extern "C" fn fpc_enroll_create_cb(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut presp: *mut FPC_BEGIN_ENROL = 0 as *mut FPC_BEGIN_ENROL;
    if check_data(data, &mut error) == 0 {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s error: %s \0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fpc_enroll_create_cb\0"))
                .as_ptr(),
            (*error).message,
        );
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    presp = data as *mut FPC_BEGIN_ENROL;
    if (*presp).status != 0 as libc::c_int {
        error = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_GENERAL,
            b"End Enroll failed: %d\0" as *const u8 as *const libc::c_char,
            (*presp).status,
        );
    }
    if !error.is_null() {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s error: %s \0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fpc_enroll_create_cb\0"))
                .as_ptr(),
            (*error).message,
        );
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    } else {
        (*self_0).do_cleanup = (0 as libc::c_int == 0) as libc::c_int;
        fpi_ssm_next_state((*self_0).task_ssm);
    };
}
unsafe extern "C" fn fpc_enroll_update_cb(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut presp: *mut FPC_ENROL = 0 as *mut FPC_ENROL;
    if check_data(data, &mut error) == 0 {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s error: %s \0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fpc_enroll_update_cb\0"))
                .as_ptr(),
            (*error).message,
        );
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    presp = data as *mut FPC_ENROL;
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Enrol Update status: %d, remaining: %d\0" as *const u8 as *const libc::c_char,
        (*presp).status,
        (*presp).remaining,
    );
    let mut current_block_25: u64;
    match (*presp).status {
        2 => {
            error = fpi_device_error_new(FP_DEVICE_ERROR_GENERAL);
        }
        3 => {
            error = fpi_device_error_new(FP_DEVICE_ERROR_DATA_DUPLICATE);
        }
        0 => {
            (*self_0).enroll_stage += 1;
            fpi_device_enroll_progress(
                FP_DEVICE(self_0 as gpointer),
                (*self_0).enroll_stage,
                0 as *mut FpPrint,
                0 as *mut GError,
            );
            fpi_ssm_jump_to_state((*self_0).task_ssm, FP_ENROLL_COMPLETE as libc::c_int);
            return;
        }
        5 => {
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Sample overlapping ratio is too High\0" as *const u8
                    as *const libc::c_char,
            );
            if (*self_0).max_immobile_stage != 0 {
                if (*self_0).immobile_stage >= (*self_0).max_immobile_stage {
                    g_log(
                        b"libfprint\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"Skip similar handle due to customer enrollment %d(%d)\0"
                            as *const u8 as *const libc::c_char,
                        (*self_0).immobile_stage,
                        (*self_0).max_immobile_stage,
                    );
                    fpi_ssm_jump_to_state(
                        (*self_0).task_ssm,
                        FPC_ENROL_STATUS_PROGRESS as libc::c_int,
                    );
                    current_block_25 = 6669252993407410313;
                } else {
                    (*self_0).immobile_stage += 1;
                    current_block_25 = 12039483399334584727;
                }
            } else {
                current_block_25 = 12039483399334584727;
            }
            match current_block_25 {
                6669252993407410313 => {}
                _ => {
                    fpi_device_enroll_progress(
                        FP_DEVICE(self_0 as gpointer),
                        (*self_0).enroll_stage,
                        0 as *mut FpPrint,
                        fpi_device_retry_new(FP_DEVICE_RETRY_REMOVE_FINGER),
                    );
                }
            }
        }
        1 => {
            (*self_0).enroll_stage += 1;
            fpi_device_enroll_progress(
                FP_DEVICE(self_0 as gpointer),
                (*self_0).enroll_stage,
                0 as *mut FpPrint,
                0 as *mut GError,
            );
            if (*self_0).enroll_stage
                >= (*self_0).max_enroll_stage - (*self_0).max_immobile_stage
            {
                fpi_ssm_jump_to_state(
                    (*self_0).task_ssm,
                    FP_ENROLL_COMPLETE as libc::c_int,
                );
            }
        }
        4 => {
            fpi_device_enroll_progress(
                FP_DEVICE(self_0 as gpointer),
                (*self_0).enroll_stage,
                0 as *mut FpPrint,
                fpi_device_retry_new(FP_DEVICE_RETRY_CENTER_FINGER),
            );
        }
        6 => {
            fpi_device_enroll_progress(
                FP_DEVICE(self_0 as gpointer),
                (*self_0).enroll_stage,
                0 as *mut FpPrint,
                fpi_device_retry_new(FP_DEVICE_RETRY_TOO_SHORT),
            );
        }
        _ => {
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"%s Unknown result code: %d \0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"fpc_enroll_update_cb\0"))
                    .as_ptr(),
                (*presp).status,
            );
            error = fpi_device_error_new_msg(
                FP_DEVICE_ERROR_GENERAL,
                b"Enroll failed: %d\0" as *const u8 as *const libc::c_char,
                (*presp).status,
            );
        }
    }
    if !error.is_null() {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s error: %s \0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fpc_enroll_update_cb\0"))
                .as_ptr(),
            (*error).message,
        );
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    } else {
        fpi_ssm_jump_to_state((*self_0).task_ssm, FP_ENROLL_CAPTURE as libc::c_int);
    };
}
unsafe extern "C" fn fpc_enroll_complete_cb(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut presp: *mut FPC_END_ENROL = 0 as *mut FPC_END_ENROL;
    (*self_0).do_cleanup = 0 as libc::c_int;
    if check_data(data, &mut error) != 0 {
        presp = data as *mut FPC_END_ENROL;
        if (*presp).status != 0 as libc::c_int {
            error = fpi_device_error_new_msg(
                FP_DEVICE_ERROR_GENERAL,
                b"End Enroll failed: %d\0" as *const u8 as *const libc::c_char,
                (*presp).status,
            );
        } else {
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Enrol End status: %d, fid: 0x%x\0" as *const u8 as *const libc::c_char,
                (*presp).status,
                (*presp).fid,
            );
        }
    }
    if !error.is_null() {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s error: %s \0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"fpc_enroll_complete_cb\0"))
                .as_ptr(),
            (*error).message,
        );
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    } else {
        fpi_ssm_next_state((*self_0).task_ssm);
    };
}
unsafe extern "C" fn fpc_enroll_check_duplicate_cb(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut presp: *mut FPC_IDENTIFY = 0 as *mut FPC_IDENTIFY;
    if check_data(data, &mut error) != 0 {
        presp = data as *mut FPC_IDENTIFY;
        if (*presp).status == 0 as libc::c_int
            && (*presp).subfactor == 0xf5 as libc::c_int as libc::c_uint
            && (*presp).identity_type == 0x3 as libc::c_int as libc::c_uint
            && (*presp).identity_size <= 68 as libc::c_int as libc::c_uint
        {
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"%s Got a duplicated template\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"fpc_enroll_check_duplicate_cb\0"))
                    .as_ptr(),
            );
            error = fpi_device_error_new(FP_DEVICE_ERROR_DATA_DUPLICATE);
        }
    }
    if !error.is_null() {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s error: %s \0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"fpc_enroll_check_duplicate_cb\0"))
                .as_ptr(),
            (*error).message,
        );
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    } else {
        fpi_ssm_next_state((*self_0).task_ssm);
    };
}
unsafe extern "C" fn fpc_enroll_bindid_cb(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut data: *mut libc::c_void,
    mut error: *mut GError,
) {
    if !error.is_null() {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s error: %s \0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fpc_enroll_bindid_cb\0"))
                .as_ptr(),
            (*error).message,
        );
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn fpc_enroll_commit_cb(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut result: *mut gint32 = 0 as *mut gint32;
    if check_data(data, &mut error) != 0 {
        result = data as *mut gint32;
        if *result != 0 as libc::c_int {
            error = fpi_device_error_new_msg(
                FP_DEVICE_ERROR_DATA_FULL,
                b"Save DB failed: %d\0" as *const u8 as *const libc::c_char,
                *result,
            );
        }
    }
    if !error.is_null() {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s error: %s \0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fpc_enroll_commit_cb\0"))
                .as_ptr(),
            (*error).message,
        );
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    } else {
        fpi_ssm_mark_completed((*self_0).task_ssm);
    };
}
unsafe extern "C" fn fpc_enroll_sm_run_state(
    mut ssm: *mut FpiSsm,
    mut device: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(device as gpointer);
    let mut cmd_data: CommandData = {
        let mut init = CommandData {
            cmdtype: FPC_CMDTYPE_UNKNOWN,
            request: 0,
            value: 0,
            index: 0,
            data: 0 as *mut guint8,
            data_len: 0,
            callback: None,
        };
        init
    };
    let mut recv_data_len: gsize = 0 as libc::c_int as gsize;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            let mut pquery_data: FPC_FID_DATA = {
                let mut init = _FPC_FID_DATA {
                    identity_type: 0 as libc::c_int as guint32,
                    reserved: 0,
                    identity_size: 0,
                    subfactor: 0,
                    data: [0; 68],
                };
                init
            };
            let mut query_data_len: gsize = 0 as libc::c_int as gsize;
            let mut wildcard_value: guint32 = 0x25066282 as libc::c_int as guint32;
            query_data_len = ::core::mem::size_of::<FPC_FID_DATA>() as libc::c_ulong;
            pquery_data.identity_type = 0x1 as libc::c_int as guint32;
            pquery_data.reserved = 16 as libc::c_int as guint32;
            pquery_data
                .identity_size = ::core::mem::size_of::<guint32>() as libc::c_ulong
                as guint32;
            pquery_data.subfactor = 0xff as libc::c_int as guint32;
            memcpy(
                &mut *(pquery_data.data).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut guint8 as *mut libc::c_void,
                &mut wildcard_value as *mut guint32 as *const libc::c_void,
                pquery_data.identity_size as libc::c_ulong,
            );
            cmd_data.cmdtype = FPC_CMDTYPE_TO_DEVICE_EVTDATA;
            cmd_data.request = 0x70 as libc::c_int as guint8;
            cmd_data.value = 0 as libc::c_int as guint16;
            cmd_data.index = 0 as libc::c_int as guint16;
            cmd_data.data = &mut pquery_data as *mut FPC_FID_DATA as *mut guint8;
            cmd_data.data_len = query_data_len;
            cmd_data
                .callback = Some(
                fpc_evt_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceFpcMoc,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            );
            fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
        }
        1 => {
            recv_data_len = ::core::mem::size_of::<FPC_BEGIN_ENROL>() as libc::c_ulong;
            cmd_data.cmdtype = FPC_CMDTYPE_FROM_DEVICE;
            cmd_data.request = 0x67 as libc::c_int as guint8;
            cmd_data.value = 0 as libc::c_int as guint16;
            cmd_data.index = 0 as libc::c_int as guint16;
            cmd_data.data = 0 as *mut guint8;
            cmd_data.data_len = recv_data_len;
            cmd_data
                .callback = Some(
                fpc_enroll_create_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceFpcMoc,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            );
            fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
        }
        2 => {
            let mut capture_id: guint32 = 0x701100f as libc::c_int as guint32;
            fpi_device_report_finger_status_changes(
                device,
                FP_FINGER_STATUS_NEEDED,
                FP_FINGER_STATUS_NONE,
            );
            cmd_data.cmdtype = FPC_CMDTYPE_TO_DEVICE_EVTDATA;
            cmd_data.request = 0x2 as libc::c_int as guint8;
            cmd_data.value = 0x1 as libc::c_int as guint16;
            cmd_data.index = 0 as libc::c_int as guint16;
            cmd_data.data = &mut capture_id as *mut guint32 as *mut guint8;
            cmd_data.data_len = ::core::mem::size_of::<guint32>() as libc::c_ulong;
            cmd_data
                .callback = Some(
                fpc_evt_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceFpcMoc,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            );
            fpc_sensor_cmd(
                self_0,
                (0 as libc::c_int == 0) as libc::c_int,
                &mut cmd_data,
            );
        }
        3 => {
            cmd_data.cmdtype = FPC_CMDTYPE_TO_DEVICE_EVTDATA;
            cmd_data.request = 0x9 as libc::c_int as guint8;
            cmd_data.value = 0 as libc::c_int as guint16;
            cmd_data.index = 0 as libc::c_int as guint16;
            cmd_data.data = 0 as *mut guint8;
            cmd_data.data_len = 0 as libc::c_int as gsize;
            cmd_data
                .callback = Some(
                fpc_evt_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceFpcMoc,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            );
            fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
        }
        4 => {
            recv_data_len = ::core::mem::size_of::<FPC_ENROL>() as libc::c_ulong;
            cmd_data.cmdtype = FPC_CMDTYPE_FROM_DEVICE;
            cmd_data.request = 0x68 as libc::c_int as guint8;
            cmd_data.value = 0 as libc::c_int as guint16;
            cmd_data.index = 0 as libc::c_int as guint16;
            cmd_data.data = 0 as *mut guint8;
            cmd_data.data_len = recv_data_len;
            cmd_data
                .callback = Some(
                fpc_enroll_update_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceFpcMoc,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            );
            fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
        }
        5 => {
            recv_data_len = ::core::mem::size_of::<FPC_END_ENROL>() as libc::c_ulong;
            cmd_data.cmdtype = FPC_CMDTYPE_FROM_DEVICE;
            cmd_data.request = 0x69 as libc::c_int as guint8;
            cmd_data.value = 0 as libc::c_int as guint16;
            cmd_data.index = 0 as libc::c_int as guint16;
            cmd_data.data = 0 as *mut guint8;
            cmd_data.data_len = recv_data_len;
            cmd_data
                .callback = Some(
                fpc_enroll_complete_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceFpcMoc,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            );
            fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
        }
        6 => {
            recv_data_len = ::core::mem::size_of::<FPC_IDENTIFY>() as libc::c_ulong;
            cmd_data.cmdtype = FPC_CMDTYPE_FROM_DEVICE;
            cmd_data.request = 0x6b as libc::c_int as guint8;
            cmd_data.value = 0 as libc::c_int as guint16;
            cmd_data.index = 0 as libc::c_int as guint16;
            cmd_data.data = 0 as *mut guint8;
            cmd_data.data_len = recv_data_len;
            cmd_data
                .callback = Some(
                fpc_enroll_check_duplicate_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceFpcMoc,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            );
            fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
        }
        7 => {
            let mut data: FPC_FID_DATA = {
                let mut init = _FPC_FID_DATA {
                    identity_type: 0 as libc::c_int as guint32,
                    reserved: 0,
                    identity_size: 0,
                    subfactor: 0,
                    data: [0; 68],
                };
                init
            };
            let mut data_len: gsize = 0 as libc::c_int as gsize;
            let mut print: *mut FpPrint = 0 as *mut FpPrint;
            let mut fpi_data: *mut GVariant = 0 as *mut GVariant;
            let mut uid: *mut GVariant = 0 as *mut GVariant;
            let mut finger: guint = 0xf5 as libc::c_int as guint;
            let mut user_id: *mut gchar = 0 as *mut gchar;
            let mut user_id_len: gssize = 0;
            let mut payload: *mut guint8 = 0 as *mut guint8;
            fpi_device_get_enroll_data(device, &mut print);
            user_id = fpi_print_generate_user_id(print);
            user_id_len = strlen(user_id) as gssize;
            user_id_len = if (68 as libc::c_int as libc::c_long) < user_id_len {
                68 as libc::c_int as libc::c_long
            } else {
                user_id_len
            };
            uid = g_variant_new_fixed_array(
                b"y\0" as *const u8 as *const libc::c_char as *const GVariantType,
                user_id as gconstpointer,
                user_id_len as gsize,
                1 as libc::c_int as gsize,
            );
            fpi_data = g_variant_new(
                b"(y@ay)\0" as *const u8 as *const libc::c_char,
                finger,
                uid,
            );
            fpi_print_set_type(print, FPI_PRINT_RAW);
            fpi_print_set_device_stored(print, (0 as libc::c_int == 0) as libc::c_int);
            g_object_set(
                print as gpointer,
                b"fpi-data\0" as *const u8 as *const libc::c_char,
                fpi_data,
                0 as *mut libc::c_void,
            );
            g_object_set(
                print as gpointer,
                b"description\0" as *const u8 as *const libc::c_char,
                user_id,
                0 as *mut libc::c_void,
            );
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"user_id: %s, finger: 0x%x\0" as *const u8 as *const libc::c_char,
                user_id,
                finger,
            );
            data_len = ::core::mem::size_of::<FPC_FID_DATA>() as libc::c_ulong;
            data.identity_type = 0x3 as libc::c_int as guint32;
            data.reserved = 16 as libc::c_int as guint32;
            data.identity_size = user_id_len as guint32;
            data.subfactor = finger;
            memcpy(
                &mut *(data.data).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut guint8 as *mut libc::c_void,
                user_id as *const libc::c_void,
                user_id_len as libc::c_ulong,
            );
            cmd_data.cmdtype = FPC_CMDTYPE_TO_DEVICE;
            cmd_data.request = 0x6a as libc::c_int as guint8;
            cmd_data.value = 0 as libc::c_int as guint16;
            cmd_data.index = 0 as libc::c_int as guint16;
            cmd_data.data = &mut data as *mut FPC_FID_DATA as *mut guint8;
            cmd_data.data_len = data_len;
            cmd_data
                .callback = Some(
                fpc_enroll_bindid_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceFpcMoc,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            );
            fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
        }
        8 => {
            recv_data_len = ::core::mem::size_of::<gint32>() as libc::c_ulong;
            cmd_data.cmdtype = FPC_CMDTYPE_FROM_DEVICE;
            cmd_data.request = 0x61 as libc::c_int as guint8;
            cmd_data.value = 0 as libc::c_int as guint16;
            cmd_data.index = 0 as libc::c_int as guint16;
            cmd_data.data = 0 as *mut guint8;
            cmd_data.data_len = recv_data_len;
            cmd_data
                .callback = Some(
                fpc_enroll_commit_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceFpcMoc,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            );
            fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
        }
        9 => {
            cmd_data.cmdtype = FPC_CMDTYPE_TO_DEVICE;
            cmd_data.request = 0x3 as libc::c_int as guint8;
            cmd_data.value = 0x1 as libc::c_int as guint16;
            cmd_data.index = 0 as libc::c_int as guint16;
            cmd_data.data = 0 as *mut guint8;
            cmd_data.data_len = 0 as libc::c_int as gsize;
            cmd_data
                .callback = Some(
                fpc_do_abort_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceFpcMoc,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            );
            fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
        }
        10 => {
            if (*self_0).do_cleanup == (0 as libc::c_int == 0) as libc::c_int {
                recv_data_len = ::core::mem::size_of::<FPC_END_ENROL>() as libc::c_ulong;
                cmd_data.cmdtype = FPC_CMDTYPE_FROM_DEVICE;
                cmd_data.request = 0x69 as libc::c_int as guint8;
                cmd_data.value = 0 as libc::c_int as guint16;
                cmd_data.index = 0 as libc::c_int as guint16;
                cmd_data.data = 0 as *mut guint8;
                cmd_data.data_len = recv_data_len;
                cmd_data
                    .callback = Some(
                    fpc_do_cleanup_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceFpcMoc,
                            *mut libc::c_void,
                            *mut GError,
                        ) -> (),
                );
                fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
            } else {
                fpi_ssm_next_state((*self_0).task_ssm);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn fpc_enroll_ssm_done(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(dev as gpointer);
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Enrollment complete!\0" as *const u8 as *const libc::c_char,
    );
    if !(fpi_ssm_get_error(ssm)).is_null() {
        error = fpi_ssm_get_error(ssm);
    }
    if !error.is_null() {
        fpi_device_enroll_complete(dev, 0 as *mut FpPrint, error);
        (*self_0).task_ssm = 0 as *mut FpiSsm;
        return;
    }
    fpi_device_get_enroll_data(FP_DEVICE(self_0 as gpointer), &mut print);
    fpi_device_enroll_complete(
        FP_DEVICE(self_0 as gpointer),
        g_object_ref(print as gpointer) as *mut FpPrint,
        0 as *mut GError,
    );
    (*self_0).task_ssm = 0 as *mut FpiSsm;
}
unsafe extern "C" fn fpc_verify_cb(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut templates: GPtrArray_autoptr = 0 as GPtrArray_autoptr;
    let mut device: *mut FpDevice = FP_DEVICE(self_0 as gpointer);
    let mut found: gboolean = 0 as libc::c_int;
    let mut current_action: FpiDeviceAction = FPI_DEVICE_ACTION_NONE;
    let mut presp: *mut FPC_IDENTIFY = 0 as *mut FPC_IDENTIFY;
    if check_data(data, &mut error) == 0 {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s error: %s \0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"fpc_verify_cb\0"))
                .as_ptr(),
            (*error).message,
        );
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    presp = data as *mut FPC_IDENTIFY;
    current_action = fpi_device_get_current_action(device);
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if current_action as libc::c_uint
            == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
            || current_action as libc::c_uint
                == FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/fpcmoc/fpc.c\0" as *const u8 as *const libc::c_char,
            1195 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"fpc_verify_cb\0"))
                .as_ptr(),
            b"current_action == FPI_DEVICE_ACTION_VERIFY || current_action == FPI_DEVICE_ACTION_IDENTIFY\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if (*presp).status == 0 as libc::c_int
        && (*presp).subfactor == 0xf5 as libc::c_int as libc::c_uint
        && (*presp).identity_type == 0x3 as libc::c_int as libc::c_uint
        && (*presp).identity_size <= 68 as libc::c_int as libc::c_uint
    {
        let mut match_0: *mut FpPrint = 0 as *mut FpPrint;
        let mut print: *mut FpPrint = 0 as *mut FpPrint;
        let mut cnt: gint = 0 as libc::c_int;
        let mut fid_data: fpc_fid_data_t = {
            let mut init = fpc_fid_data_t {
                subfactor: 0 as libc::c_int as guint8,
                identity_type: 0,
                identity_size: 0,
                identity: [0; 68],
            };
            init
        };
        fid_data.subfactor = (*presp).subfactor as guint8;
        fid_data.identity_type = (*presp).identity_type;
        fid_data.identity_size = (*presp).identity_size;
        memcpy(
            (fid_data.identity).as_mut_ptr() as *mut libc::c_void,
            &mut *((*presp).data).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut guint8 as *const libc::c_void,
            fid_data.identity_size as libc::c_ulong,
        );
        match_0 = fpc_print_from_data(self_0, &mut fid_data);
        if current_action as libc::c_uint
            == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
        {
            templates = g_ptr_array_sized_new(1 as libc::c_int as guint);
            fpi_device_get_verify_data(device, &mut print);
            g_ptr_array_add(templates, print as gpointer);
        } else {
            fpi_device_get_identify_data(device, &mut templates);
            g_ptr_array_ref(templates);
        }
        cnt = 0 as libc::c_int;
        while (cnt as libc::c_uint) < (*templates).len {
            print = *((*templates).pdata).offset(cnt as isize) as *mut FpPrint;
            if fp_print_equal(print, match_0) != 0 {
                found = (0 as libc::c_int == 0) as libc::c_int;
                break;
            } else {
                cnt += 1;
            }
        }
        if found != 0 {
            if current_action as libc::c_uint
                == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
            {
                fpi_device_verify_report(device, FPI_MATCH_SUCCESS, match_0, error);
            } else {
                fpi_device_identify_report(device, print, match_0, error);
            }
            fpi_ssm_mark_completed((*self_0).task_ssm);
            return;
        }
    }
    if found == 0 {
        if current_action as libc::c_uint
            == FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
        {
            fpi_device_verify_report(device, FPI_MATCH_FAIL, 0 as *mut FpPrint, error);
        } else {
            fpi_device_identify_report(
                device,
                0 as *mut FpPrint,
                0 as *mut FpPrint,
                error,
            );
        }
    }
    fpi_ssm_mark_completed((*self_0).task_ssm);
}
unsafe extern "C" fn fpc_verify_sm_run_state(
    mut ssm: *mut FpiSsm,
    mut device: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(device as gpointer);
    let mut cmd_data: CommandData = {
        let mut init = CommandData {
            cmdtype: FPC_CMDTYPE_UNKNOWN,
            request: 0,
            value: 0,
            index: 0,
            data: 0 as *mut guint8,
            data_len: 0,
            callback: None,
        };
        init
    };
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            let mut capture_id: guint32 = 0x701100f as libc::c_int as guint32;
            fpi_device_report_finger_status_changes(
                device,
                FP_FINGER_STATUS_NEEDED,
                FP_FINGER_STATUS_NONE,
            );
            cmd_data.cmdtype = FPC_CMDTYPE_TO_DEVICE_EVTDATA;
            cmd_data.request = 0x2 as libc::c_int as guint8;
            cmd_data.value = 0x1 as libc::c_int as guint16;
            cmd_data.index = 0 as libc::c_int as guint16;
            cmd_data.data = &mut capture_id as *mut guint32 as *mut guint8;
            cmd_data.data_len = ::core::mem::size_of::<guint32>() as libc::c_ulong;
            cmd_data
                .callback = Some(
                fpc_evt_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceFpcMoc,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            );
            fpc_sensor_cmd(
                self_0,
                (0 as libc::c_int == 0) as libc::c_int,
                &mut cmd_data,
            );
        }
        1 => {
            cmd_data.cmdtype = FPC_CMDTYPE_TO_DEVICE_EVTDATA;
            cmd_data.request = 0x9 as libc::c_int as guint8;
            cmd_data.value = 0 as libc::c_int as guint16;
            cmd_data.index = 0 as libc::c_int as guint16;
            cmd_data.data = 0 as *mut guint8;
            cmd_data.data_len = 0 as libc::c_int as gsize;
            cmd_data
                .callback = Some(
                fpc_evt_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceFpcMoc,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            );
            fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
        }
        2 => {
            let mut recv_data_len: gsize = ::core::mem::size_of::<FPC_IDENTIFY>()
                as libc::c_ulong;
            cmd_data.cmdtype = FPC_CMDTYPE_FROM_DEVICE;
            cmd_data.request = 0x6b as libc::c_int as guint8;
            cmd_data.value = 0 as libc::c_int as guint16;
            cmd_data.index = 0 as libc::c_int as guint16;
            cmd_data.data = 0 as *mut guint8;
            cmd_data.data_len = recv_data_len;
            cmd_data
                .callback = Some(
                fpc_verify_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceFpcMoc,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            );
            fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
        }
        3 => {
            cmd_data.cmdtype = FPC_CMDTYPE_TO_DEVICE;
            cmd_data.request = 0x3 as libc::c_int as guint8;
            cmd_data.value = 0x1 as libc::c_int as guint16;
            cmd_data.index = 0 as libc::c_int as guint16;
            cmd_data.data = 0 as *mut guint8;
            cmd_data.data_len = 0 as libc::c_int as gsize;
            cmd_data
                .callback = Some(
                fpc_do_abort_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceFpcMoc,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            );
            fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
        }
        _ => {}
    };
}
unsafe extern "C" fn fpc_verify_ssm_done(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(dev as gpointer);
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Verify_identify complete!\0" as *const u8 as *const libc::c_char,
    );
    if !(fpi_ssm_get_error(ssm)).is_null() {
        error = fpi_ssm_get_error(ssm);
    }
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
unsafe extern "C" fn fpc_clear_storage_cb(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut resp: *mut libc::c_void,
    mut error: *mut GError,
) {
    if !error.is_null() {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s error: %s \0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fpc_clear_storage_cb\0"))
                .as_ptr(),
            (*error).message,
        );
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    fpi_ssm_next_state((*self_0).task_ssm);
}
unsafe extern "C" fn fpc_clear_sm_run_state(
    mut ssm: *mut FpiSsm,
    mut device: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(device as gpointer);
    let mut cmd_data: CommandData = {
        let mut init = CommandData {
            cmdtype: FPC_CMDTYPE_UNKNOWN,
            request: 0,
            value: 0,
            index: 0,
            data: 0 as *mut guint8,
            data_len: 0,
            callback: None,
        };
        init
    };
    let mut data: FPC_DB_OP = _FPC_DELETE_DB {
        reserved: 0 as libc::c_int as guint32,
    };
    let mut data_len: gsize = 0 as libc::c_int as gsize;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            if !((*self_0).dbid).is_null() {
                data_len = ::core::mem::size_of::<FPC_DB_OP>() as libc::c_ulong;
                data.database_id_size = 16 as libc::c_int as guint32;
                data.reserved = 8 as libc::c_int as guint32;
                memcpy(
                    &mut *(data.data).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut guint8 as *mut libc::c_void,
                    (*self_0).dbid as *const libc::c_void,
                    16 as libc::c_int as libc::c_ulong,
                );
                cmd_data.cmdtype = FPC_CMDTYPE_TO_DEVICE;
                cmd_data.request = 0x62 as libc::c_int as guint8;
                cmd_data.value = 0 as libc::c_int as guint16;
                cmd_data.index = 0 as libc::c_int as guint16;
                cmd_data.data = &mut data as *mut FPC_DB_OP as *mut guint8;
                cmd_data.data_len = data_len;
                cmd_data
                    .callback = Some(
                    fpc_clear_storage_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceFpcMoc,
                            *mut libc::c_void,
                            *mut GError,
                        ) -> (),
                );
                fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
            } else {
                fpi_ssm_mark_failed(
                    (*self_0).task_ssm,
                    fpi_device_error_new_msg(
                        FP_DEVICE_ERROR_NOT_SUPPORTED,
                        b"No DBID found\0" as *const u8 as *const libc::c_char,
                    ),
                );
            }
        }
        1 => {
            if !((*self_0).dbid).is_null() {
                data_len = ::core::mem::size_of::<FPC_DB_OP>() as libc::c_ulong;
                data.database_id_size = 16 as libc::c_int as guint32;
                data.reserved = 8 as libc::c_int as guint32;
                memcpy(
                    &mut *(data.data).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut guint8 as *mut libc::c_void,
                    (*self_0).dbid as *const libc::c_void,
                    16 as libc::c_int as libc::c_ulong,
                );
                cmd_data.cmdtype = FPC_CMDTYPE_TO_DEVICE;
                cmd_data.request = 0x60 as libc::c_int as guint8;
                cmd_data.value = 0x1 as libc::c_int as guint16;
                cmd_data.index = 0 as libc::c_int as guint16;
                cmd_data.data = &mut data as *mut FPC_DB_OP as *mut guint8;
                cmd_data.data_len = data_len;
                cmd_data
                    .callback = Some(
                    fpc_clear_storage_cb
                        as unsafe extern "C" fn(
                            *mut FpiDeviceFpcMoc,
                            *mut libc::c_void,
                            *mut GError,
                        ) -> (),
                );
                fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
            } else {
                fpi_ssm_mark_failed(
                    (*self_0).task_ssm,
                    fpi_device_error_new_msg(
                        FP_DEVICE_ERROR_NOT_SUPPORTED,
                        b"No DBID found\0" as *const u8 as *const libc::c_char,
                    ),
                );
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn fpc_clear_ssm_done(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(dev as gpointer);
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Clear Storage complete!\0" as *const u8 as *const libc::c_char,
    );
    if !(fpi_ssm_get_error(ssm)).is_null() {
        error = fpi_ssm_get_error(ssm);
    }
    fpi_device_clear_storage_complete(dev, error);
    (*self_0).task_ssm = 0 as *mut FpiSsm;
}
unsafe extern "C" fn fpc_init_load_db_cb(
    mut self_0: *mut FpiDeviceFpcMoc,
    mut data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut presp: *mut FPC_LOAD_DB = 0 as *mut FPC_LOAD_DB;
    if !error.is_null() {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s error: %s \0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fpc_init_load_db_cb\0"))
                .as_ptr(),
            (*error).message,
        );
        fpi_ssm_mark_failed((*self_0).task_ssm, error);
        return;
    }
    if data.is_null() {
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_error_new(FP_DEVICE_ERROR_DATA_INVALID),
        );
        return;
    }
    presp = data as *mut FPC_LOAD_DB;
    if (*presp).status != 0 {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s Load DB failed: %d - Expect to create a new one\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fpc_init_load_db_cb\0"))
                .as_ptr(),
            (*presp).status,
        );
        fpi_ssm_next_state((*self_0).task_ssm);
        return;
    }
    let mut _pp: C2RustUnnamed_11 = C2RustUnnamed_11 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*self_0).dbid as *mut *mut guint8 as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    (*self_0)
        .dbid = ({
        g_memdup2(
            ((*presp).data).as_mut_ptr() as gconstpointer,
            16 as libc::c_int as gsize,
        )
    }) as *mut guint8;
    if ((*self_0).dbid).is_null() {
        fpi_ssm_mark_failed(
            (*self_0).task_ssm,
            fpi_device_error_new(FP_DEVICE_ERROR_GENERAL),
        );
        return;
    }
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s got dbid size: %d\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"fpc_init_load_db_cb\0"))
            .as_ptr(),
        (*presp).database_id_size,
    );
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s dbid: 0x%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x\0"
            as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"fpc_init_load_db_cb\0"))
            .as_ptr(),
        (*presp).data[0 as libc::c_int as usize] as libc::c_int,
        (*presp).data[1 as libc::c_int as usize] as libc::c_int,
        (*presp).data[2 as libc::c_int as usize] as libc::c_int,
        (*presp).data[3 as libc::c_int as usize] as libc::c_int,
        (*presp).data[4 as libc::c_int as usize] as libc::c_int,
        (*presp).data[5 as libc::c_int as usize] as libc::c_int,
        (*presp).data[6 as libc::c_int as usize] as libc::c_int,
        (*presp).data[7 as libc::c_int as usize] as libc::c_int,
        (*presp).data[8 as libc::c_int as usize] as libc::c_int,
        (*presp).data[9 as libc::c_int as usize] as libc::c_int,
        (*presp).data[10 as libc::c_int as usize] as libc::c_int,
        (*presp).data[11 as libc::c_int as usize] as libc::c_int,
        (*presp).data[12 as libc::c_int as usize] as libc::c_int,
        (*presp).data[13 as libc::c_int as usize] as libc::c_int,
        (*presp).data[14 as libc::c_int as usize] as libc::c_int,
        (*presp).data[15 as libc::c_int as usize] as libc::c_int,
    );
    fpi_ssm_mark_completed((*self_0).task_ssm);
}
unsafe extern "C" fn fpc_init_sm_run_state(
    mut ssm: *mut FpiSsm,
    mut device: *mut FpDevice,
) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(device as gpointer);
    let mut session_id: guint32 = 0x77ff12 as libc::c_int as guint32;
    let mut cmd_data: CommandData = {
        let mut init = CommandData {
            cmdtype: FPC_CMDTYPE_UNKNOWN,
            request: 0,
            value: 0,
            index: 0,
            data: 0 as *mut guint8,
            data_len: 0,
            callback: None,
        };
        init
    };
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            cmd_data.cmdtype = FPC_CMDTYPE_TO_DEVICE_EVTDATA;
            cmd_data.request = 0x1 as libc::c_int as guint8;
            cmd_data.value = 0x1 as libc::c_int as guint16;
            cmd_data.index = 0 as libc::c_int as guint16;
            cmd_data.data = &mut session_id as *mut guint32 as *mut guint8;
            cmd_data.data_len = ::core::mem::size_of::<guint32>() as libc::c_ulong;
            cmd_data
                .callback = Some(
                fpc_evt_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceFpcMoc,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            );
            fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
        }
        1 => {
            let mut recv_data_len: gsize = ::core::mem::size_of::<FPC_LOAD_DB>()
                as libc::c_ulong;
            cmd_data.cmdtype = FPC_CMDTYPE_FROM_DEVICE;
            cmd_data.request = 0x60 as libc::c_int as guint8;
            cmd_data.value = 0 as libc::c_int as guint16;
            cmd_data.index = 0 as libc::c_int as guint16;
            cmd_data.data = 0 as *mut guint8;
            cmd_data.data_len = recv_data_len;
            cmd_data
                .callback = Some(
                fpc_init_load_db_cb
                    as unsafe extern "C" fn(
                        *mut FpiDeviceFpcMoc,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            );
            fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
        }
        _ => {}
    };
}
unsafe extern "C" fn fpc_init_ssm_done(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(dev as gpointer);
    if !(fpi_ssm_get_error(ssm)).is_null() {
        error = fpi_ssm_get_error(ssm);
    }
    fpi_device_open_complete(dev, error);
    (*self_0).task_ssm = 0 as *mut FpiSsm;
}
unsafe extern "C" fn fpc_dev_probe(mut device: *mut FpDevice) {
    let mut usb_dev: *mut GUsbDevice = 0 as *mut GUsbDevice;
    let mut error: *mut GError = 0 as *mut GError;
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(device as gpointer);
    let mut product: *mut gchar = 0 as *mut gchar;
    let mut product_id: gint = 0 as libc::c_int;
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s enter --> \0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"fpc_dev_probe\0"))
            .as_ptr(),
    );
    usb_dev = fpi_device_get_usb_device(device);
    if g_usb_device_open(usb_dev, &mut error) == 0 {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"%s g_usb_device_open failed %s\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"fpc_dev_probe\0"))
                .as_ptr(),
            (*error).message,
        );
        fpi_device_probe_complete(device, 0 as *const gchar, 0 as *const gchar, error);
        return;
    }
    if g_usb_device_reset(usb_dev, &mut error) == 0 {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"%s g_usb_device_reset failed %s\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"fpc_dev_probe\0"))
                .as_ptr(),
            (*error).message,
        );
        g_usb_device_close(usb_dev, 0 as *mut *mut GError);
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
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"%s g_usb_device_claim_interface failed %s\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"fpc_dev_probe\0"))
                .as_ptr(),
            (*error).message,
        );
        g_usb_device_close(usb_dev, 0 as *mut *mut GError);
        fpi_device_probe_complete(device, 0 as *const gchar, 0 as *const gchar, error);
        return;
    }
    product = g_usb_device_get_string_descriptor(
        usb_dev,
        g_usb_device_get_product_index(usb_dev),
        &mut error,
    );
    if !product.is_null() {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"Device name: %s\0" as *const u8 as *const libc::c_char,
            product,
        );
    }
    if !error.is_null() {
        g_log(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"%s g_usb_device_get_string_descriptor failed %s\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"fpc_dev_probe\0"))
                .as_ptr(),
            (*error).message,
        );
        g_usb_device_release_interface(
            fpi_device_get_usb_device(FP_DEVICE(device as gpointer)),
            0 as libc::c_int,
            G_USB_DEVICE_CLAIM_INTERFACE_NONE,
            0 as *mut *mut GError,
        );
        g_usb_device_close(usb_dev, 0 as *mut *mut GError);
        fpi_device_probe_complete(device, 0 as *const gchar, 0 as *const gchar, error);
        return;
    }
    product_id = g_usb_device_get_pid(usb_dev) as gint;
    (*self_0).max_immobile_stage = 0 as libc::c_int;
    match product_id {
        65504 | 41733 | 55301 | 55812 | 53765 => {
            (*self_0).max_enroll_stage = 25 as libc::c_int;
        }
        _ => {
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"Device %x is not supported\0" as *const u8 as *const libc::c_char,
                product_id,
            );
            (*self_0).max_enroll_stage = 25 as libc::c_int;
        }
    }
    fpi_device_set_nr_enroll_stages(device, (*self_0).max_enroll_stage);
    g_usb_device_release_interface(
        fpi_device_get_usb_device(FP_DEVICE(device as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        0 as *mut *mut GError,
    );
    g_usb_device_close(usb_dev, 0 as *mut *mut GError);
    fpi_device_probe_complete(device, 0 as *const gchar, product, error);
}
unsafe extern "C" fn fpc_dev_open(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(device as gpointer);
    let mut error: *mut GError = 0 as *mut GError;
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s enter -->\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"fpc_dev_open\0"))
            .as_ptr(),
    );
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
            fpc_init_sm_run_state
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        FP_INIT_NUM_STATES as libc::c_int,
        FP_INIT_NUM_STATES as libc::c_int,
        b"FP_INIT_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        (*self_0).task_ssm,
        Some(
            fpc_init_ssm_done
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn fpc_dev_close(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(device as gpointer);
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s enter -->\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"fpc_dev_close\0"))
            .as_ptr(),
    );
    let mut _pp: C2RustUnnamed_10 = C2RustUnnamed_10 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*self_0).dbid as *mut *mut guint8 as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    let mut _pp_0: C2RustUnnamed_9 = C2RustUnnamed_9 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_0: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp_0
        .in_0 = &mut (*self_0).interrupt_cancellable as *mut *mut GCancellable
        as *mut libc::c_char;
    _p_0 = *_pp_0.out;
    if !_p_0.is_null() {
        *_pp_0.out = 0 as *mut libc::c_void;
        _destroy_0.expect("non-null function pointer")(_p_0);
    }
    fpc_dev_release_interface(self_0, 0 as *mut GError);
}
unsafe extern "C" fn fpc_dev_verify_identify(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(device as gpointer);
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s enter -->\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 24],
            &[libc::c_char; 24],
        >(b"fpc_dev_verify_identify\0"))
            .as_ptr(),
    );
    (*self_0)
        .task_ssm = fpi_ssm_new_full(
        device,
        Some(
            fpc_verify_sm_run_state
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        FP_VERIFY_NUM_STATES as libc::c_int,
        FP_VERIFY_CANCEL as libc::c_int,
        b"verify_identify\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        (*self_0).task_ssm,
        Some(
            fpc_verify_ssm_done
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn fpc_dev_enroll(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(device as gpointer);
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s enter -->\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"fpc_dev_enroll\0"))
            .as_ptr(),
    );
    (*self_0).enroll_stage = 0 as libc::c_int;
    (*self_0).immobile_stage = 0 as libc::c_int;
    (*self_0)
        .task_ssm = fpi_ssm_new_full(
        device,
        Some(
            fpc_enroll_sm_run_state
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        FP_ENROLL_NUM_STATES as libc::c_int,
        FP_ENROLL_DICARD as libc::c_int,
        b"enroll\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        (*self_0).task_ssm,
        Some(
            fpc_enroll_ssm_done
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn fpc_dev_template_list(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(device as gpointer);
    let mut cmd_data: CommandData = {
        let mut init = CommandData {
            cmdtype: FPC_CMDTYPE_UNKNOWN,
            request: 0,
            value: 0,
            index: 0,
            data: 0 as *mut guint8,
            data_len: 0,
            callback: None,
        };
        init
    };
    let mut pquery_data: FPC_FID_DATA = {
        let mut init = _FPC_FID_DATA {
            identity_type: 0 as libc::c_int as guint32,
            reserved: 0,
            identity_size: 0,
            subfactor: 0,
            data: [0; 68],
        };
        init
    };
    let mut query_data_len: gsize = 0 as libc::c_int as gsize;
    let mut wildcard_value: guint32 = 0x25066282 as libc::c_int as guint32;
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s enter -->\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"fpc_dev_template_list\0"))
            .as_ptr(),
    );
    query_data_len = ::core::mem::size_of::<FPC_FID_DATA>() as libc::c_ulong;
    pquery_data.identity_type = 0x1 as libc::c_int as guint32;
    pquery_data.reserved = 16 as libc::c_int as guint32;
    pquery_data
        .identity_size = ::core::mem::size_of::<guint32>() as libc::c_ulong as guint32;
    pquery_data.subfactor = 0xff as libc::c_int as guint32;
    memcpy(
        &mut *(pquery_data.data).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut guint8 as *mut libc::c_void,
        &mut wildcard_value as *mut guint32 as *const libc::c_void,
        pquery_data.identity_size as libc::c_ulong,
    );
    cmd_data.cmdtype = FPC_CMDTYPE_TO_DEVICE_EVTDATA;
    cmd_data.request = 0x70 as libc::c_int as guint8;
    cmd_data.value = 0 as libc::c_int as guint16;
    cmd_data.index = 0 as libc::c_int as guint16;
    cmd_data.data = &mut pquery_data as *mut FPC_FID_DATA as *mut guint8;
    cmd_data.data_len = query_data_len;
    cmd_data
        .callback = Some(
        fpc_template_list_cb
            as unsafe extern "C" fn(
                *mut FpiDeviceFpcMoc,
                *mut libc::c_void,
                *mut GError,
            ) -> (),
    );
    fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
}
unsafe extern "C" fn fpc_dev_suspend(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(device as gpointer);
    let mut action: FpiDeviceAction = fpi_device_get_current_action(device);
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s enter -->\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"fpc_dev_suspend\0"))
            .as_ptr(),
    );
    if action as libc::c_uint != FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
        && action as libc::c_uint
            != FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
    {
        fpi_device_suspend_complete(
            device,
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
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/fpcmoc/fpc.c\0" as *const u8 as *const libc::c_char,
            1760 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"fpc_dev_suspend\0"))
                .as_ptr(),
            b"self->cmd_ssm\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_ssm_get_cur_state((*self_0).cmd_ssm) == FP_CMD_GET_DATA as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/fpcmoc/fpc.c\0" as *const u8 as *const libc::c_char,
            1761 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"fpc_dev_suspend\0"))
                .as_ptr(),
            b"fpi_ssm_get_cur_state (self->cmd_ssm) == FP_CMD_GET_DATA\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*self_0).cmd_suspended = (0 as libc::c_int == 0) as libc::c_int;
    g_cancellable_cancel((*self_0).interrupt_cancellable);
}
unsafe extern "C" fn fpc_dev_resume(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(device as gpointer);
    let mut action: FpiDeviceAction = fpi_device_get_current_action(device);
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s enter -->\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"fpc_dev_resume\0"))
            .as_ptr(),
    );
    if action as libc::c_uint != FPI_DEVICE_ACTION_VERIFY as libc::c_int as libc::c_uint
        && action as libc::c_uint
            != FPI_DEVICE_ACTION_IDENTIFY as libc::c_int as libc::c_uint
    {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/fpcmoc/fpc.c\0" as *const u8 as *const libc::c_char,
            1776 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"fpc_dev_resume\0"))
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
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/fpcmoc/fpc.c\0" as *const u8 as *const libc::c_char,
            1781 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"fpc_dev_resume\0"))
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
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/fpcmoc/fpc.c\0" as *const u8 as *const libc::c_char,
            1782 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"fpc_dev_resume\0"))
                .as_ptr(),
            b"self->cmd_suspended\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_ssm_get_cur_state((*self_0).cmd_ssm) == FP_CMD_SUSPENDED as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/fpcmoc/fpc.c\0" as *const u8 as *const libc::c_char,
            1783 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"fpc_dev_resume\0"))
                .as_ptr(),
            b"fpi_ssm_get_cur_state (self->cmd_ssm) == FP_CMD_SUSPENDED\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*self_0).cmd_suspended = 0 as libc::c_int;
    let mut _object_ptr: C2RustUnnamed_8 = C2RustUnnamed_8 {
        in_0: 0 as *mut libc::c_char,
    };
    _object_ptr
        .in_0 = &mut (*self_0).interrupt_cancellable as *mut *mut GCancellable
        as *mut libc::c_char;
    if 0 as libc::c_int != 0 {
        (*self_0).interrupt_cancellable = g_cancellable_new();
    } else {};
    g_set_object(_object_ptr.out, g_cancellable_new() as *mut GObject);
    fpi_ssm_jump_to_state((*self_0).cmd_ssm, FP_CMD_RESUME as libc::c_int);
}
unsafe extern "C" fn fpc_dev_cancel(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(device as gpointer);
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s enter -->\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"fpc_dev_cancel\0"))
            .as_ptr(),
    );
    g_cancellable_cancel((*self_0).interrupt_cancellable);
}
unsafe extern "C" fn fpc_dev_template_delete(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(device as gpointer);
    let mut cmd_data: CommandData = {
        let mut init = CommandData {
            cmdtype: FPC_CMDTYPE_UNKNOWN,
            request: 0,
            value: 0,
            index: 0,
            data: 0 as *mut guint8,
            data_len: 0,
            callback: None,
        };
        init
    };
    let mut print: *mut FpPrint = 0 as *mut FpPrint;
    let mut fpi_data: GVariant_autoptr = 0 as GVariant_autoptr;
    let mut data: FPC_FID_DATA = {
        let mut init = _FPC_FID_DATA {
            identity_type: 0 as libc::c_int as guint32,
            reserved: 0,
            identity_size: 0,
            subfactor: 0,
            data: [0; 68],
        };
        init
    };
    let mut data_len: gsize = 0 as libc::c_int as gsize;
    let mut finger: guint8 = 0 as libc::c_int as guint8;
    let mut user_id: *const guint8 = 0 as *const guint8;
    let mut user_id_len: gsize = 0 as libc::c_int as gsize;
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s enter -->\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 24],
            &[libc::c_char; 24],
        >(b"fpc_dev_template_delete\0"))
            .as_ptr(),
    );
    fpi_device_get_delete_data(device, &mut print);
    g_object_get(
        print as gpointer,
        b"fpi-data\0" as *const u8 as *const libc::c_char,
        &mut fpi_data as *mut GVariant_autoptr,
        0 as *mut libc::c_void,
    );
    if parse_print_data(fpi_data, &mut finger, &mut user_id, &mut user_id_len) == 0 {
        fpi_device_delete_complete(
            device,
            fpi_device_error_new(FP_DEVICE_ERROR_DATA_INVALID),
        );
        return;
    }
    data_len = ::core::mem::size_of::<FPC_FID_DATA>() as libc::c_ulong;
    data.identity_type = 0x3 as libc::c_int as guint32;
    data.reserved = 16 as libc::c_int as guint32;
    data.identity_size = user_id_len as guint32;
    data.subfactor = finger as guint32;
    memcpy(
        &mut *(data.data).as_mut_ptr().offset(0 as libc::c_int as isize) as *mut guint8
            as *mut libc::c_void,
        user_id as *const libc::c_void,
        user_id_len,
    );
    cmd_data.cmdtype = FPC_CMDTYPE_TO_DEVICE;
    cmd_data.request = 0x63 as libc::c_int as guint8;
    cmd_data.value = 0 as libc::c_int as guint16;
    cmd_data.index = 0 as libc::c_int as guint16;
    cmd_data.data = &mut data as *mut FPC_FID_DATA as *mut guint8;
    cmd_data.data_len = data_len;
    cmd_data
        .callback = Some(
        fpc_template_delete_cb
            as unsafe extern "C" fn(
                *mut FpiDeviceFpcMoc,
                *mut libc::c_void,
                *mut GError,
            ) -> (),
    );
    fpc_sensor_cmd(self_0, 0 as libc::c_int, &mut cmd_data);
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s exit <--\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 24],
            &[libc::c_char; 24],
        >(b"fpc_dev_template_delete\0"))
            .as_ptr(),
    );
}
unsafe extern "C" fn fpc_dev_clear_storage(mut device: *mut FpDevice) {
    let mut self_0: *mut FpiDeviceFpcMoc = FPI_DEVICE_FPCMOC(device as gpointer);
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s enter -->\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"fpc_dev_clear_storage\0"))
            .as_ptr(),
    );
    (*self_0)
        .task_ssm = fpi_ssm_new_full(
        device,
        Some(
            fpc_clear_sm_run_state
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
        ),
        FP_CLEAR_NUM_STATES as libc::c_int,
        FP_CLEAR_NUM_STATES as libc::c_int,
        b"Clear storage\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        (*self_0).task_ssm,
        Some(
            fpc_clear_ssm_done
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn fpi_device_fpcmoc_init(mut self_0: *mut FpiDeviceFpcMoc) {
    g_log(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"%s enter -->\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 23],
            &[libc::c_char; 23],
        >(b"fpi_device_fpcmoc_init\0"))
            .as_ptr(),
    );
    g_log_structured(
        b"libfprint\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/fpcmoc/fpc.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"1861\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 23],
            &[libc::c_char; 23],
        >(b"fpi_device_fpcmoc_init\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/fpcmoc/fpc.c:1861\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn fpi_device_fpcmoc_class_init(mut klass: *mut FpiDeviceFpcMocClass) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    (*dev_class).id = b"fpcmoc\0" as *const u8 as *const libc::c_char;
    (*dev_class)
        .full_name = b"FPC MOC Fingerprint Sensor\0" as *const u8 as *const libc::c_char;
    (*dev_class).type_0 = FP_DEVICE_TYPE_USB;
    (*dev_class).scan_type = FP_SCAN_TYPE_PRESS;
    (*dev_class).id_table = id_table.as_ptr();
    (*dev_class).nr_enroll_stages = 25 as libc::c_int;
    (*dev_class).temp_hot_seconds = -(1 as libc::c_int);
    (*dev_class).open = Some(fpc_dev_open as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .close = Some(fpc_dev_close as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .probe = Some(fpc_dev_probe as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .enroll = Some(fpc_dev_enroll as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .delete = Some(
        fpc_dev_template_delete as unsafe extern "C" fn(*mut FpDevice) -> (),
    );
    (*dev_class)
        .list = Some(fpc_dev_template_list as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .verify = Some(
        fpc_dev_verify_identify as unsafe extern "C" fn(*mut FpDevice) -> (),
    );
    (*dev_class)
        .identify = Some(
        fpc_dev_verify_identify as unsafe extern "C" fn(*mut FpDevice) -> (),
    );
    (*dev_class)
        .suspend = Some(fpc_dev_suspend as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .resume = Some(fpc_dev_resume as unsafe extern "C" fn(*mut FpDevice) -> ());
    (*dev_class)
        .clear_storage = Some(
        fpc_dev_clear_storage as unsafe extern "C" fn(*mut FpDevice) -> (),
    );
    (*dev_class)
        .cancel = Some(fpc_dev_cancel as unsafe extern "C" fn(*mut FpDevice) -> ());
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
