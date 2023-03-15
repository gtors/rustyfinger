use ::libc;
extern "C" {
    pub type PK11SlotInfoStr;
    pub type PK11SymKeyStr;
    pub type PK11ContextStr;
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GRand;
    pub type _GCancellablePrivate;
    pub type _GUsbInterface;
    pub type _FpiSsm;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn NSS_NoDB_Init(configdir: *const libc::c_char) -> SECStatus;
    fn NSS_Shutdown() -> SECStatus;
    fn SECITEM_FreeItem(zap: *mut SECItem, freeit: PRBool);
    fn PK11_FreeSlot(slot: *mut PK11SlotInfo);
    fn PK11_GetBestSlot(
        type_0: CK_MECHANISM_TYPE,
        wincx: *mut libc::c_void,
    ) -> *mut PK11SlotInfo;
    fn PK11_ParamFromIV(type_0: CK_MECHANISM_TYPE, iv: *mut SECItem) -> *mut SECItem;
    fn PK11_FreeSymKey(key: *mut PK11SymKey);
    fn PK11_ImportSymKey(
        slot: *mut PK11SlotInfo,
        type_0: CK_MECHANISM_TYPE,
        origin: PK11Origin,
        operation: CK_ATTRIBUTE_TYPE,
        key: *mut SECItem,
        wincx: *mut libc::c_void,
    ) -> *mut PK11SymKey;
    fn PK11_DestroyContext(context: *mut PK11Context, freeit: PRBool);
    fn PK11_CreateContextBySymKey(
        type_0: CK_MECHANISM_TYPE,
        operation: CK_ATTRIBUTE_TYPE,
        symKey: *mut PK11SymKey,
        param: *const SECItem,
    ) -> *mut PK11Context;
    fn PK11_CipherOp(
        context: *mut PK11Context,
        out: *mut libc::c_uchar,
        outlen: *mut libc::c_int,
        maxout: libc::c_int,
        in_0: *const libc::c_uchar,
        inlen: libc::c_int,
    ) -> SECStatus;
    fn PK11_Finalize(context: *mut PK11Context) -> SECStatus;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_new_literal(
        domain: GQuark,
        code: gint,
        message: *const gchar,
    ) -> *mut GError;
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_clear_error(err: *mut *mut GError);
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_setenv(
        variable: *const gchar,
        value: *const gchar,
        overwrite: gboolean,
    ) -> gboolean;
    fn g_free(mem: gpointer);
    fn g_source_destroy(source: *mut GSource);
    fn g_get_monotonic_time() -> gint64;
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn g_log_structured(log_domain: *const gchar, log_level: GLogLevelFlags, _: ...);
    fn g_rand_new() -> *mut GRand;
    fn g_rand_free(rand_: *mut GRand);
    fn g_rand_set_seed(rand_: *mut GRand, seed: guint32);
    fn g_rand_int_range(rand_: *mut GRand, begin: gint32, end: gint32) -> gint32;
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
    fn g_object_unref(object: gpointer);
    fn g_cancellable_new() -> *mut GCancellable;
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
    fn g_io_error_quark() -> GQuark;
    fn fp_image_new(width: gint, height: gint) -> *mut FpImage;
    fn g_usb_interface_get_number(self_0: *mut GUsbInterface) -> guint8;
    fn g_usb_interface_get_class(self_0: *mut GUsbInterface) -> guint8;
    fn g_usb_interface_get_subclass(self_0: *mut GUsbInterface) -> guint8;
    fn g_usb_interface_get_protocol(self_0: *mut GUsbInterface) -> guint8;
    fn g_usb_device_error_quark() -> GQuark;
    fn g_usb_device_get_interfaces(
        self_0: *mut GUsbDevice,
        error: *mut *mut GError,
    ) -> *mut GPtrArray;
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
    fn fp_device_error_quark() -> GQuark;
    fn fpi_device_get_usb_device(device: *mut FpDevice) -> *mut GUsbDevice;
    fn fpi_device_error_new_msg(
        error: FpDeviceError,
        msg: *const gchar,
        _: ...
    ) -> *mut GError;
    fn fpi_device_get_driver_data(device: *mut FpDevice) -> guint64;
    fn fpi_device_add_timeout(
        device: *mut FpDevice,
        interval: gint,
        func: FpTimeoutFunc,
        user_data: gpointer,
        destroy_notify: GDestroyNotify,
    ) -> *mut GSource;
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
    fn fpi_ssm_new_full(
        dev: *mut FpDevice,
        handler: FpiSsmHandlerCallback,
        nr_states: libc::c_int,
        start_cleanup: libc::c_int,
        machine_name: *const libc::c_char,
    ) -> *mut FpiSsm;
    fn fpi_ssm_start(ssm: *mut FpiSsm, callback: FpiSsmCompletedCallback);
    fn fpi_ssm_start_subsm(parent: *mut FpiSsm, child: *mut FpiSsm);
    fn fpi_ssm_next_state(machine: *mut FpiSsm);
    fn fpi_ssm_jump_to_state(machine: *mut FpiSsm, state: libc::c_int);
    fn fpi_ssm_next_state_delayed(machine: *mut FpiSsm, delay: libc::c_int);
    fn fpi_ssm_jump_to_state_delayed(
        machine: *mut FpiSsm,
        state: libc::c_int,
        delay: libc::c_int,
    );
    fn fpi_ssm_mark_completed(machine: *mut FpiSsm);
    fn fpi_ssm_mark_failed(machine: *mut FpiSsm, error: *mut GError);
    fn fpi_ssm_get_cur_state(machine: *mut FpiSsm) -> libc::c_int;
}
pub type PRIntn = libc::c_int;
pub type PRBool = PRIntn;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type SECItemType = libc::c_uint;
pub const siBMPString: SECItemType = 15;
pub const siUTF8String: SECItemType = 14;
pub const siVisibleString: SECItemType = 13;
pub const siGeneralizedTime: SECItemType = 12;
pub const siUTCTime: SECItemType = 11;
pub const siUnsignedInteger: SECItemType = 10;
pub const siDEROID: SECItemType = 9;
pub const siAsciiString: SECItemType = 8;
pub const siAsciiNameString: SECItemType = 7;
pub const siEncodedNameBuffer: SECItemType = 6;
pub const siDERNameBuffer: SECItemType = 5;
pub const siEncodedCertBuffer: SECItemType = 4;
pub const siDERCertBuffer: SECItemType = 3;
pub const siCipherDataBuffer: SECItemType = 2;
pub const siClearDataBuffer: SECItemType = 1;
pub const siBuffer: SECItemType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SECItemStr {
    pub type_0: SECItemType,
    pub data: *mut libc::c_uchar,
    pub len: libc::c_uint,
}
pub type SECItem = SECItemStr;
pub type _SECStatus = libc::c_int;
pub const SECSuccess: _SECStatus = 0;
pub const SECFailure: _SECStatus = -1;
pub const SECWouldBlock: _SECStatus = -2;
pub type SECStatus = _SECStatus;
pub type CK_ULONG = libc::c_ulong;
pub type CK_ATTRIBUTE_TYPE = CK_ULONG;
pub type CK_MECHANISM_TYPE = CK_ULONG;
pub type PK11SlotInfo = PK11SlotInfoStr;
pub type PK11SymKey = PK11SymKeyStr;
pub type PK11Context = PK11ContextStr;
pub type PK11Origin = libc::c_uint;
pub const PK11_OriginUnwrap: PK11Origin = 4;
pub const PK11_OriginFortezzaHack: PK11Origin = 3;
pub const PK11_OriginGenerated: PK11Origin = 2;
pub const PK11_OriginDerive: PK11Origin = 1;
pub const PK11_OriginNULL: PK11Origin = 0;
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
pub type GMainContext = _GMainContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSource {
    pub callback_data: gpointer,
    pub callback_funcs: *mut GSourceCallbackFuncs,
    pub source_funcs: *const GSourceFuncs,
    pub ref_count: guint,
    pub context: *mut GMainContext,
    pub priority: gint,
    pub flags: guint,
    pub source_id: guint,
    pub poll_fds: *mut GSList,
    pub prev: *mut GSource,
    pub next: *mut GSource,
    pub name: *mut libc::c_char,
    pub priv_0: *mut GSourcePrivate,
}
pub type GSourcePrivate = _GSourcePrivate;
pub type GSource = _GSource;
pub type GSourceFuncs = _GSourceFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceFuncs {
    pub prepare: Option::<unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean>,
    pub check: Option::<unsafe extern "C" fn(*mut GSource) -> gboolean>,
    pub dispatch: Option::<
        unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
    >,
    pub finalize: Option::<unsafe extern "C" fn(*mut GSource) -> ()>,
    pub closure_callback: GSourceFunc,
    pub closure_marshal: GSourceDummyMarshal,
}
pub type GSourceDummyMarshal = Option::<unsafe extern "C" fn() -> ()>;
pub type GSourceFunc = Option::<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type GSourceCallbackFuncs = _GSourceCallbackFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceCallbackFuncs {
    pub ref_0: Option::<unsafe extern "C" fn(gpointer) -> ()>,
    pub unref: Option::<unsafe extern "C" fn(gpointer) -> ()>,
    pub get: Option::<
        unsafe extern "C" fn(
            gpointer,
            *mut GSource,
            *mut GSourceFunc,
            *mut gpointer,
        ) -> (),
    >,
}
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
pub type GRand = _GRand;
pub type GPtrArray_autoptr = *mut GPtrArray;
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
pub type GUsbInterface = _GUsbInterface;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const G_USB_DEVICE_ERROR_LAST: C2RustUnnamed_1 = 10;
pub const G_USB_DEVICE_ERROR_PERMISSION_DENIED: C2RustUnnamed_1 = 9;
pub const G_USB_DEVICE_ERROR_FAILED: C2RustUnnamed_1 = 8;
pub const G_USB_DEVICE_ERROR_CANCELLED: C2RustUnnamed_1 = 7;
pub const G_USB_DEVICE_ERROR_ALREADY_OPEN: C2RustUnnamed_1 = 6;
pub const G_USB_DEVICE_ERROR_NOT_OPEN: C2RustUnnamed_1 = 5;
pub const G_USB_DEVICE_ERROR_NO_DEVICE: C2RustUnnamed_1 = 4;
pub const G_USB_DEVICE_ERROR_NOT_SUPPORTED: C2RustUnnamed_1 = 3;
pub const G_USB_DEVICE_ERROR_TIMED_OUT: C2RustUnnamed_1 = 2;
pub const G_USB_DEVICE_ERROR_IO: C2RustUnnamed_1 = 1;
pub const G_USB_DEVICE_ERROR_INTERNAL: C2RustUnnamed_1 = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub driver_data: guint64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub virtual_envvar: *const gchar,
    pub c2rust_unnamed_0: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub udev_types: FpiDeviceUdevSubtypeFlags,
    pub spi_acpi_id: *const gchar,
    pub hid_id: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub pid: guint,
    pub vid: guint,
}
pub type FpiDeviceUdevSubtypeFlags = libc::c_uint;
pub const FPI_DEVICE_UDEV_SUBTYPE_HIDRAW: FpiDeviceUdevSubtypeFlags = 2;
pub const FPI_DEVICE_UDEV_SUBTYPE_SPIDEV: FpiDeviceUdevSubtypeFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
pub type FpTimeoutFunc = Option::<unsafe extern "C" fn(*mut FpDevice, gpointer) -> ()>;
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
pub type C2RustUnnamed_6 = libc::c_uint;
pub const IRQDATA_DEATH: C2RustUnnamed_6 = 2048;
pub const IRQDATA_FINGER_OFF: C2RustUnnamed_6 = 512;
pub const IRQDATA_FINGER_ON: C2RustUnnamed_6 = 257;
pub const IRQDATA_SCANPWR_ON: C2RustUnnamed_6 = 22186;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const REG_CHALLENGE: C2RustUnnamed_7 = 8208;
pub const REG_RESPONSE: C2RustUnnamed_7 = 8192;
pub const REG_DEVICE_INFO: C2RustUnnamed_7 = 240;
pub const REG_MODE: C2RustUnnamed_7 = 78;
pub const REG_SCRAMBLE_DATA_KEY: C2RustUnnamed_7 = 52;
pub const REG_SCRAMBLE_DATA_INDEX: C2RustUnnamed_7 = 51;
pub const REG_HWSTAT: C2RustUnnamed_7 = 7;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const MODE_READY: C2RustUnnamed_8 = 128;
pub const MODE_OFF: C2RustUnnamed_8 = 112;
pub const MODE_CAPTURE_AUX: C2RustUnnamed_8 = 48;
pub const MODE_CAPTURE: C2RustUnnamed_8 = 32;
pub const MODE_AWAIT_FINGER_OFF: C2RustUnnamed_8 = 18;
pub const MODE_AWAIT_FINGER_ON: C2RustUnnamed_8 = 16;
pub const MODE_INIT: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const DP_URU4000B: C2RustUnnamed_9 = 5;
pub const DP_URU4000: C2RustUnnamed_9 = 4;
pub const MS_STANDALONE_V2: C2RustUnnamed_9 = 3;
pub const MS_STANDALONE: C2RustUnnamed_9 = 2;
pub const MS_INTELLIMOUSE: C2RustUnnamed_9 = 1;
pub const MS_KBD: C2RustUnnamed_9 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uru4k_dev_profile {
    pub name: *const libc::c_char,
    pub auth_cr: gboolean,
    pub image_not_flipped: gboolean,
}
pub type irq_cb_fn = Option::<
    unsafe extern "C" fn(
        *mut FpImageDevice,
        *mut GError,
        uint16_t,
        *mut libc::c_void,
    ) -> (),
>;
pub type irqs_stopped_cb_fn = Option::<unsafe extern "C" fn(*mut FpImageDevice) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiDeviceUru4000 {
    pub parent: FpImageDevice,
    pub profile: *const uru4k_dev_profile,
    pub interface: uint8_t,
    pub activate_state: FpiImageDeviceState,
    pub last_reg_rd: [libc::c_uchar; 16],
    pub last_hwstat: libc::c_uchar,
    pub irq_cancellable: *mut GCancellable,
    pub img_transfer: *mut FpiUsbTransfer,
    pub img_data: *mut libc::c_void,
    pub img_data_actual_length: libc::c_int,
    pub img_lines_done: uint16_t,
    pub img_block: uint16_t,
    pub rand: *mut GRand,
    pub img_enc_seed: uint32_t,
    pub irq_cb: irq_cb_fn,
    pub irq_cb_data: *mut libc::c_void,
    pub irqs_stopped_cb: irqs_stopped_cb_fn,
    pub rebootpwr_ctr: libc::c_int,
    pub powerup_ctr: libc::c_int,
    pub powerup_hwstat: libc::c_uchar,
    pub scanpwr_irq_timeouts: libc::c_int,
    pub scanpwr_irq_timeout: *mut GSource,
    pub fwfixer_offset: libc::c_int,
    pub fwfixer_value: libc::c_uchar,
    pub cipher: CK_MECHANISM_TYPE,
    pub slot: *mut PK11SlotInfo,
    pub symkey: *mut PK11SymKey,
    pub param: *mut SECItem,
}
pub type FpiDeviceUru4000 = _FpiDeviceUru4000;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiDeviceUru4000Class {
    pub parent_class: FpImageDeviceClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uru4k_image {
    pub unknown_00: [uint8_t; 4],
    pub num_lines: uint16_t,
    pub key_number: uint8_t,
    pub unknown_07: [uint8_t; 9],
    pub block_info: [C2RustUnnamed_11; 15],
    pub unknown_2E: [uint8_t; 18],
    pub data: [[uint8_t; 384]; 290],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub flags: uint8_t,
    pub num_lines: uint8_t,
}
pub const IMAGING_NUM_STATES: imaging_states = 5;
pub const IMAGING_CAPTURE: imaging_states = 0;
pub const BLOCKF_NOT_PRESENT: C2RustUnnamed_15 = 1;
pub const IMAGING_REPORT_IMAGE: imaging_states = 4;
pub const BLOCKF_ENCRYPTED: C2RustUnnamed_15 = 2;
pub const BLOCKF_NO_KEY_UPDATE: C2RustUnnamed_15 = 4;
pub const IMAGING_SEND_INDEX: imaging_states = 1;
pub const BLOCKF_CHANGE_KEY: C2RustUnnamed_15 = 128;
pub const IMAGING_DECODE: imaging_states = 3;
pub const IMAGING_READ_KEY: imaging_states = 2;
pub const INIT_NUM_STATES: init_states = 9;
pub const INIT_REPORT_VERSION: init_states = 8;
pub const INIT_GET_VERSION: init_states = 7;
pub const INIT_DONE: init_states = 6;
pub const INIT_GET_HWSTAT: init_states = 0;
pub const INIT_AWAIT_SCAN_POWER: init_states = 5;
pub const POWERUP_NUM_STATES: powerup_states = 7;
pub const POWERUP_SET_HWSTAT: powerup_states = 1;
pub const POWERUP_CHALLENGE_RESPONSE_SUCCESS: powerup_states = 6;
pub const POWERUP_CHALLENGE_RESPONSE: powerup_states = 5;
pub const POWERUP_PAUSE: powerup_states = 4;
pub const POWERUP_CHECK_HWSTAT: powerup_states = 3;
pub const POWERUP_GET_HWSTAT: powerup_states = 2;
pub const POWERUP_INIT: powerup_states = 0;
pub const INIT_POWERUP: init_states = 4;
pub const INIT_CHECK_HWSTAT_POWERDOWN: init_states = 3;
pub const REBOOTPWR_NUM_STATES: rebootpwr_states = 4;
pub const REBOOTPWR_GET_HWSTAT: rebootpwr_states = 1;
pub const REBOOTPWR_PAUSE: rebootpwr_states = 3;
pub const REBOOTPWR_CHECK_HWSTAT: rebootpwr_states = 2;
pub const REBOOTPWR_SET_HWSTAT: rebootpwr_states = 0;
pub const INIT_REBOOT_POWER: init_states = 2;
pub const INIT_CHECK_HWSTAT_REBOOT: init_states = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub type imaging_states = libc::c_uint;
pub type C2RustUnnamed_15 = libc::c_uint;
pub type rebootpwr_states = libc::c_uint;
pub type powerup_states = libc::c_uint;
pub type init_states = libc::c_uint;
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
static mut uru4k_dev_info: [uru4k_dev_profile; 6] = [
    {
        let mut init = uru4k_dev_profile {
            name: b"Microsoft Keyboard with Fingerprint Reader\0" as *const u8
                as *const libc::c_char,
            auth_cr: 0 as libc::c_int,
            image_not_flipped: 0,
        };
        init
    },
    {
        let mut init = uru4k_dev_profile {
            name: b"Microsoft Wireless IntelliMouse with Fingerprint Reader\0"
                as *const u8 as *const libc::c_char,
            auth_cr: 0 as libc::c_int,
            image_not_flipped: 0,
        };
        init
    },
    {
        let mut init = uru4k_dev_profile {
            name: b"Microsoft Fingerprint Reader\0" as *const u8 as *const libc::c_char,
            auth_cr: 0 as libc::c_int,
            image_not_flipped: 0,
        };
        init
    },
    {
        let mut init = uru4k_dev_profile {
            name: b"Microsoft Fingerprint Reader v2\0" as *const u8
                as *const libc::c_char,
            auth_cr: (0 as libc::c_int == 0) as libc::c_int,
            image_not_flipped: 0,
        };
        init
    },
    {
        let mut init = uru4k_dev_profile {
            name: b"Digital Persona U.are.U 4000\0" as *const u8 as *const libc::c_char,
            auth_cr: 0 as libc::c_int,
            image_not_flipped: 0,
        };
        init
    },
    {
        let mut init = uru4k_dev_profile {
            name: b"Digital Persona U.are.U 4000B\0" as *const u8 as *const libc::c_char,
            auth_cr: 0 as libc::c_int,
            image_not_flipped: (0 as libc::c_int == 0) as libc::c_int,
        };
        init
    },
];
#[inline]
unsafe extern "C" fn FPI_DEVICE_URU4000(mut ptr: gpointer) -> *mut FpiDeviceUru4000 {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_uru4000_get_type(),
    ) as *mut libc::c_void as *mut FpiDeviceUru4000;
}
static mut fpi_device_uru4000_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn fpi_device_uru4000_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_uru4000_get_type_once();
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
unsafe extern "C" fn fpi_device_uru4000_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_image_device_get_type(),
        g_intern_static_string(
            b"FpiDeviceUru4000\0" as *const u8 as *const libc::c_char,
        ),
        ::core::mem::size_of::<FpiDeviceUru4000Class>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_uru4000_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpiDeviceUru4000>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpiDeviceUru4000) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_uru4000_init
                        as unsafe extern "C" fn(*mut FpiDeviceUru4000) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn fpi_device_uru4000_class_intern_init(mut klass: gpointer) {
    fpi_device_uru4000_parent_class = g_type_class_peek_parent(klass);
    if FpiDeviceUru4000_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpiDeviceUru4000_private_offset);
    }
    fpi_device_uru4000_class_init(klass as *mut FpiDeviceUru4000Class);
}
static mut FpiDeviceUru4000_private_offset: gint = 0;
static mut crkey: [libc::c_uchar; 16] = [
    0x79 as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0x79 as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0x98 as libc::c_int as libc::c_uchar,
    0xe0 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn write_regs(
    mut dev: *mut FpImageDevice,
    mut first_reg: uint16_t,
    mut num_regs: uint16_t,
    mut values: *mut libc::c_uchar,
    mut callback: FpiUsbTransferCallback,
    mut user_data: *mut libc::c_void,
) {
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(
        FP_DEVICE(dev as gpointer),
    );
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_fill_control(
        transfer,
        G_USB_DEVICE_DIRECTION_HOST_TO_DEVICE,
        G_USB_DEVICE_REQUEST_TYPE_VENDOR,
        G_USB_DEVICE_RECIPIENT_DEVICE,
        0x4 as libc::c_int as guint8,
        first_reg,
        0 as libc::c_int as guint16,
        num_regs as gsize,
    );
    memcpy(
        (*transfer).buffer as *mut libc::c_void,
        values as *const libc::c_void,
        num_regs as libc::c_ulong,
    );
    fpi_usb_transfer_submit(
        transfer,
        5000 as libc::c_int as guint,
        0 as *mut GCancellable,
        callback,
        user_data,
    );
}
unsafe extern "C" fn write_reg(
    mut dev: *mut FpImageDevice,
    mut reg: uint16_t,
    mut value: libc::c_uchar,
    mut callback: FpiUsbTransferCallback,
    mut user_data: *mut libc::c_void,
) {
    write_regs(dev, reg, 1 as libc::c_int as uint16_t, &mut value, callback, user_data);
}
unsafe extern "C" fn read_regs(
    mut dev: *mut FpImageDevice,
    mut first_reg: uint16_t,
    mut num_regs: uint16_t,
    mut callback: FpiUsbTransferCallback,
    mut user_data: *mut libc::c_void,
) {
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(
        FP_DEVICE(dev as gpointer),
    );
    fpi_usb_transfer_fill_control(
        transfer,
        G_USB_DEVICE_DIRECTION_DEVICE_TO_HOST,
        G_USB_DEVICE_REQUEST_TYPE_VENDOR,
        G_USB_DEVICE_RECIPIENT_DEVICE,
        0x4 as libc::c_int as guint8,
        first_reg,
        0 as libc::c_int as guint16,
        num_regs as gsize,
    );
    fpi_usb_transfer_submit(
        transfer,
        5000 as libc::c_int as guint,
        0 as *mut GCancellable,
        callback,
        user_data,
    );
}
unsafe extern "C" fn response_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut dev: *mut FpDevice,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut ssm: *mut FpiSsm = user_data as *mut FpiSsm;
    if error.is_null() {
        fpi_ssm_next_state(ssm);
    } else {
        fpi_ssm_mark_failed(ssm, error);
    };
}
unsafe extern "C" fn challenge_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut dev: *mut FpDevice,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut ssm: *mut FpiSsm = user_data as *mut FpiSsm;
    let mut self_0: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(dev as gpointer);
    let mut respdata: [libc::c_uchar; 16] = [0; 16];
    let mut ctx: *mut PK11Context = 0 as *mut PK11Context;
    let mut outlen: libc::c_int = 0;
    if !error.is_null() {
        fpi_ssm_mark_failed(ssm, error);
        return;
    }
    ctx = PK11_CreateContextBySymKey(
        (*self_0).cipher,
        0x104 as libc::c_ulong,
        (*self_0).symkey,
        (*self_0).param,
    );
    if PK11_CipherOp(
        ctx,
        respdata.as_mut_ptr(),
        &mut outlen,
        16 as libc::c_int,
        (*transfer).buffer,
        16 as libc::c_int,
    ) as libc::c_int != SECSuccess as libc::c_int
        || PK11_Finalize(ctx) as libc::c_int != SECSuccess as libc::c_int
    {
        g_log(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"Failed to encrypt challenge data\0" as *const u8 as *const libc::c_char,
        );
        error = fpi_device_error_new_msg(
            FP_DEVICE_ERROR_PROTO,
            b"Failed to encrypt challenge data\0" as *const u8 as *const libc::c_char,
        );
    }
    PK11_DestroyContext(ctx, 1 as libc::c_int);
    if error.is_null() {
        write_regs(
            FP_IMAGE_DEVICE(dev as gpointer),
            REG_RESPONSE as libc::c_int as uint16_t,
            16 as libc::c_int as uint16_t,
            respdata.as_mut_ptr(),
            Some(
                response_cb
                    as unsafe extern "C" fn(
                        *mut FpiUsbTransfer,
                        *mut FpDevice,
                        *mut libc::c_void,
                        *mut GError,
                    ) -> (),
            ),
            ssm as *mut libc::c_void,
        );
    } else {
        fpi_ssm_mark_failed(ssm, error);
    };
}
unsafe extern "C" fn sm_do_challenge_response(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
) {
    g_log_structured(
        b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"CODE_FILE\0" as *const u8 as *const libc::c_char,
        b"../libfprint/drivers/uru4000.c\0" as *const u8 as *const libc::c_char,
        b"CODE_LINE\0" as *const u8 as *const libc::c_char,
        b"291\0" as *const u8 as *const libc::c_char,
        b"CODE_FUNC\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 25],
            &[libc::c_char; 25],
        >(b"sm_do_challenge_response\0"))
            .as_ptr(),
        b"MESSAGE\0" as *const u8 as *const libc::c_char,
        b"%li: %s\0" as *const u8 as *const libc::c_char,
        g_get_monotonic_time(),
        b"../libfprint/drivers/uru4000.c:291\0" as *const u8 as *const libc::c_char,
    );
    read_regs(
        dev,
        REG_CHALLENGE as libc::c_int as uint16_t,
        16 as libc::c_int as uint16_t,
        Some(
            challenge_cb
                as unsafe extern "C" fn(
                    *mut FpiUsbTransfer,
                    *mut FpDevice,
                    *mut libc::c_void,
                    *mut GError,
                ) -> (),
        ),
        ssm as *mut libc::c_void,
    );
}
unsafe extern "C" fn irq_handler(
    mut transfer: *mut FpiUsbTransfer,
    mut dev: *mut FpDevice,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut imgdev: *mut FpImageDevice = FP_IMAGE_DEVICE(dev as gpointer);
    let mut urudev: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(dev as gpointer);
    let mut data: *mut libc::c_uchar = (*transfer).buffer;
    let mut type_0: uint16_t = 0;
    let mut _pp: C2RustUnnamed_12 = C2RustUnnamed_12 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp
        .in_0 = &mut (*urudev).irq_cancellable as *mut *mut GCancellable
        as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    if g_error_matches(error, g_io_error_quark(), G_IO_ERROR_CANCELLED as libc::c_int)
        != 0
    {
        g_log(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"cancelled\0" as *const u8 as *const libc::c_char,
        );
        if ((*urudev).irqs_stopped_cb).is_some() {
            ((*urudev).irqs_stopped_cb).expect("non-null function pointer")(imgdev);
        }
        (*urudev).irqs_stopped_cb = None;
        g_clear_error(&mut error);
        return;
    } else {
        if !error.is_null() {
            if ((*urudev).irq_cb).is_some() {
                ((*urudev).irq_cb)
                    .expect(
                        "non-null function pointer",
                    )(
                    imgdev,
                    error,
                    0 as libc::c_int as uint16_t,
                    (*urudev).irq_cb_data,
                );
            } else {
                g_log(
                    b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"ignoring interrupt error: %s\0" as *const u8
                        as *const libc::c_char,
                    (*error).message,
                );
                g_clear_error(&mut error);
            }
            return;
        }
    }
    start_irq_handler(imgdev);
    type_0 = ((*(data as *mut uint16_t) as libc::c_int >> 8 as libc::c_int) as guint16
        as libc::c_int
        | ((*(data as *mut uint16_t) as libc::c_int) << 8 as libc::c_int) as guint16
            as libc::c_int) as guint16;
    g_log(
        b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"recv irq type %04x\0" as *const u8 as *const libc::c_char,
        type_0 as libc::c_int,
    );
    if type_0 as libc::c_int == IRQDATA_DEATH as libc::c_int {
        g_log(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"oh no! got the interrupt OF DEATH! expect things to go bad\0" as *const u8
                as *const libc::c_char,
        );
    }
    if ((*urudev).irq_cb).is_some() {
        ((*urudev).irq_cb)
            .expect(
                "non-null function pointer",
            )(imgdev, 0 as *mut GError, type_0, (*urudev).irq_cb_data);
    } else {
        g_log(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"ignoring interrupt\0" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn start_irq_handler(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(dev as gpointer);
    let mut transfer: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if ((*self_0).irq_cancellable).is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/uru4000.c\0" as *const u8 as *const libc::c_char,
            359 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"start_irq_handler\0"))
                .as_ptr(),
            b"self->irq_cancellable == NULL\0" as *const u8 as *const libc::c_char,
        );
    }
    (*self_0).irq_cancellable = g_cancellable_new();
    transfer = fpi_usb_transfer_new(FP_DEVICE(dev as gpointer));
    (*transfer).ssm = 0 as *mut FpiSsm;
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_fill_interrupt(
        transfer,
        (1 as libc::c_int | 0x80 as libc::c_int) as guint8,
        64 as libc::c_int as gsize,
    );
    fpi_usb_transfer_submit(
        transfer,
        0 as libc::c_int as guint,
        (*self_0).irq_cancellable,
        Some(
            irq_handler
                as unsafe extern "C" fn(
                    *mut FpiUsbTransfer,
                    *mut FpDevice,
                    *mut libc::c_void,
                    *mut GError,
                ) -> (),
        ),
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn stop_irq_handler(
    mut dev: *mut FpImageDevice,
    mut cb: irqs_stopped_cb_fn,
) {
    let mut self_0: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(dev as gpointer);
    if !((*self_0).irq_cancellable).is_null() {
        g_cancellable_cancel((*self_0).irq_cancellable);
        (*self_0).irqs_stopped_cb = cb;
    } else {
        cb.expect("non-null function pointer")(dev);
    };
}
unsafe extern "C" fn finger_presence_irq_cb(
    mut dev: *mut FpImageDevice,
    mut error: *mut GError,
    mut type_0: uint16_t,
    mut user_data: *mut libc::c_void,
) {
    if !error.is_null() {
        fpi_image_device_session_error(dev, error);
    } else if type_0 as libc::c_int == IRQDATA_FINGER_ON as libc::c_int {
        fpi_image_device_report_finger_status(
            dev,
            (0 as libc::c_int == 0) as libc::c_int,
        );
    } else if type_0 as libc::c_int == IRQDATA_FINGER_OFF as libc::c_int {
        fpi_image_device_report_finger_status(dev, 0 as libc::c_int);
    } else if type_0 as libc::c_int != IRQDATA_SCANPWR_ON as libc::c_int {
        g_log(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"ignoring unexpected interrupt %04x\0" as *const u8 as *const libc::c_char,
            type_0 as libc::c_int,
        );
    }
}
unsafe extern "C" fn change_state_write_reg_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut dev: *mut FpDevice,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    if !error.is_null() {
        fpi_image_device_session_error(FP_IMAGE_DEVICE(dev as gpointer), error);
    }
}
unsafe extern "C" fn dev_change_state(
    mut dev: *mut FpImageDevice,
    mut state: FpiImageDeviceState,
) {
    let mut self_0: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(dev as gpointer);
    (*self_0).activate_state = state;
    if !((*self_0).img_transfer).is_null() {
        return;
    }
    execute_state_change(dev);
}
unsafe extern "C" fn sm_write_reg_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut dev: *mut FpDevice,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut ssm: *mut FpiSsm = user_data as *mut FpiSsm;
    if !error.is_null() {
        fpi_ssm_mark_failed(ssm, error);
    } else {
        fpi_ssm_next_state(ssm);
    };
}
unsafe extern "C" fn sm_write_regs(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
    mut first_reg: uint16_t,
    mut num_regs: uint16_t,
    mut data: *mut libc::c_void,
) {
    write_regs(
        dev,
        first_reg,
        num_regs,
        data as *mut libc::c_uchar,
        Some(
            sm_write_reg_cb
                as unsafe extern "C" fn(
                    *mut FpiUsbTransfer,
                    *mut FpDevice,
                    *mut libc::c_void,
                    *mut GError,
                ) -> (),
        ),
        ssm as *mut libc::c_void,
    );
}
unsafe extern "C" fn sm_write_reg(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
    mut reg: uint16_t,
    mut value: libc::c_uchar,
) {
    sm_write_regs(
        ssm,
        dev,
        reg,
        1 as libc::c_int as uint16_t,
        &mut value as *mut libc::c_uchar as *mut libc::c_void,
    );
}
unsafe extern "C" fn sm_read_reg_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut dev: *mut FpDevice,
    mut user_data: *mut libc::c_void,
    mut error: *mut GError,
) {
    let mut ssm: *mut FpiSsm = user_data as *mut FpiSsm;
    let mut self_0: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(dev as gpointer);
    if !error.is_null() {
        fpi_ssm_mark_failed(ssm, error);
    } else {
        memcpy(
            ((*self_0).last_reg_rd).as_mut_ptr() as *mut libc::c_void,
            (*transfer).buffer as *const libc::c_void,
            (*transfer).actual_length as libc::c_ulong,
        );
        g_log(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"reg value %x\0" as *const u8 as *const libc::c_char,
            (*self_0).last_reg_rd[0 as libc::c_int as usize] as libc::c_int,
        );
        fpi_ssm_next_state(ssm);
    };
}
unsafe extern "C" fn sm_read_regs(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
    mut reg: uint16_t,
    mut num_regs: uint16_t,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if num_regs as libc::c_ulong
            <= ::core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/uru4000.c\0" as *const u8 as *const libc::c_char,
            492 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"sm_read_regs\0"))
                .as_ptr(),
            b"num_regs <= member_size (FpiDeviceUru4000, last_reg_rd)\0" as *const u8
                as *const libc::c_char,
        );
    }
    g_log(
        b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"read %d regs at %x\0" as *const u8 as *const libc::c_char,
        num_regs as libc::c_int,
        reg as libc::c_int,
    );
    read_regs(
        dev,
        reg,
        num_regs,
        Some(
            sm_read_reg_cb
                as unsafe extern "C" fn(
                    *mut FpiUsbTransfer,
                    *mut FpDevice,
                    *mut libc::c_void,
                    *mut GError,
                ) -> (),
        ),
        ssm as *mut libc::c_void,
    );
}
unsafe extern "C" fn sm_read_reg(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
    mut reg: uint16_t,
) {
    sm_read_regs(ssm, dev, reg, 1 as libc::c_int as uint16_t);
}
unsafe extern "C" fn sm_set_hwstat(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
    mut value: libc::c_uchar,
) {
    g_log(
        b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"set %02x\0" as *const u8 as *const libc::c_char,
        value as libc::c_int,
    );
    sm_write_reg(ssm, dev, REG_HWSTAT as libc::c_int as uint16_t, value);
}
unsafe extern "C" fn image_transfer_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut dev: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(dev as gpointer);
    let mut ssm: *mut FpiSsm = (*transfer).ssm;
    if !error.is_null() {
        g_log(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"error\0" as *const u8 as *const libc::c_char,
        );
        fpi_ssm_mark_failed(ssm, error);
    } else {
        (*self_0)
            .img_data = ({
            g_memdup2(
                (*transfer).buffer as gconstpointer,
                ::core::mem::size_of::<uru4k_image>() as libc::c_ulong,
            )
        });
        (*self_0).img_data_actual_length = (*transfer).actual_length as libc::c_int;
        fpi_ssm_next_state(ssm);
    };
}
unsafe extern "C" fn update_key(mut key: uint32_t) -> uint32_t {
    let mut bit: uint32_t = key & 0x9248144d as libc::c_uint;
    bit ^= bit << 16 as libc::c_int;
    bit ^= bit << 8 as libc::c_int;
    bit ^= bit << 4 as libc::c_int;
    bit ^= bit << 2 as libc::c_int;
    bit ^= bit << 1 as libc::c_int;
    return bit & 0x80000000 as libc::c_uint | key >> 1 as libc::c_int;
}
unsafe extern "C" fn do_decode(
    mut data: *mut uint8_t,
    mut num_bytes: libc::c_int,
    mut key: uint32_t,
) -> uint32_t {
    let mut xorbyte: uint8_t = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_bytes - 1 as libc::c_int {
        xorbyte = ((key >> 4 as libc::c_int & 1 as libc::c_int as libc::c_uint)
            << 0 as libc::c_int) as uint8_t;
        xorbyte = (xorbyte as libc::c_uint
            | (key >> 8 as libc::c_int & 1 as libc::c_int as libc::c_uint)
                << 1 as libc::c_int) as uint8_t;
        xorbyte = (xorbyte as libc::c_uint
            | (key >> 11 as libc::c_int & 1 as libc::c_int as libc::c_uint)
                << 2 as libc::c_int) as uint8_t;
        xorbyte = (xorbyte as libc::c_uint
            | (key >> 14 as libc::c_int & 1 as libc::c_int as libc::c_uint)
                << 3 as libc::c_int) as uint8_t;
        xorbyte = (xorbyte as libc::c_uint
            | (key >> 18 as libc::c_int & 1 as libc::c_int as libc::c_uint)
                << 4 as libc::c_int) as uint8_t;
        xorbyte = (xorbyte as libc::c_uint
            | (key >> 21 as libc::c_int & 1 as libc::c_int as libc::c_uint)
                << 5 as libc::c_int) as uint8_t;
        xorbyte = (xorbyte as libc::c_uint
            | (key >> 24 as libc::c_int & 1 as libc::c_int as libc::c_uint)
                << 6 as libc::c_int) as uint8_t;
        xorbyte = (xorbyte as libc::c_uint
            | (key >> 29 as libc::c_int & 1 as libc::c_int as libc::c_uint)
                << 7 as libc::c_int) as uint8_t;
        key = update_key(key);
        *data
            .offset(
                i as isize,
            ) = (*data.offset((i + 1 as libc::c_int) as isize) as libc::c_int
            ^ xorbyte as libc::c_int) as uint8_t;
        i += 1;
    }
    *data.offset(i as isize) = 0 as libc::c_int as uint8_t;
    return update_key(key);
}
unsafe extern "C" fn calc_dev2(mut img: *mut uru4k_image) -> libc::c_int {
    let mut b: [*mut uint8_t; 2] = [0 as *mut uint8_t, 0 as *mut uint8_t];
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut mean: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    idx = 0 as libc::c_int;
    r = idx;
    i = r;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[C2RustUnnamed_11; 15]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong)
        && idx < 2 as libc::c_int
    {
        if !((*img).block_info[i as usize].flags as libc::c_int
            & BLOCKF_NOT_PRESENT as libc::c_int != 0)
        {
            j = 0 as libc::c_int;
            while j < (*img).block_info[i as usize].num_lines as libc::c_int
                && idx < 2 as libc::c_int
            {
                let fresh0 = r;
                r = r + 1;
                let fresh1 = idx;
                idx = idx + 1;
                b[fresh1 as usize] = ((*img).data[fresh0 as usize]).as_mut_ptr();
                j += 1;
            }
        }
        i += 1;
    }
    if (b[0 as libc::c_int as usize]).is_null()
        || (b[1 as libc::c_int as usize]).is_null()
    {
        g_log(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"NULL! %p %p\0" as *const u8 as *const libc::c_char,
            b[0 as libc::c_int as usize],
            b[1 as libc::c_int as usize],
        );
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 384 as libc::c_int {
        mean
            += *(b[0 as libc::c_int as usize]).offset(i as isize) as libc::c_int
                + *(b[1 as libc::c_int as usize]).offset(i as isize) as libc::c_int;
        i += 1;
    }
    mean /= 384 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 384 as libc::c_int {
        let mut dev: libc::c_int = *(b[0 as libc::c_int as usize]).offset(i as isize)
            as libc::c_int
            + *(b[1 as libc::c_int as usize]).offset(i as isize) as libc::c_int - mean;
        res += dev * dev;
        i += 1;
    }
    return res / 384 as libc::c_int;
}
unsafe extern "C" fn imaging_run_state(mut ssm: *mut FpiSsm, mut _dev: *mut FpDevice) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(_dev as gpointer);
    let mut img: *mut uru4k_image = (*self_0).img_data as *mut uru4k_image;
    let mut fpimg: *mut FpImage = 0 as *mut FpImage;
    let mut key: uint32_t = 0;
    let mut flags: uint8_t = 0;
    let mut num_lines: uint8_t = 0;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut to: libc::c_int = 0;
    let mut dev2: libc::c_int = 0;
    let mut buf: [libc::c_uchar; 5] = [0; 5];
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            (*self_0).img_lines_done = 0 as libc::c_int as uint16_t;
            (*self_0).img_block = 0 as libc::c_int as uint16_t;
            fpi_usb_transfer_submit(
                fpi_usb_transfer_ref((*self_0).img_transfer),
                0 as libc::c_int as guint,
                0 as *mut GCancellable,
                Some(
                    image_transfer_cb
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
        1 => {
            g_log(
                b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"hw header lines %d\0" as *const u8 as *const libc::c_char,
                (*img).num_lines as libc::c_int,
            );
            if (*img).num_lines as libc::c_int >= 290 as libc::c_int
                || (*self_0).img_data_actual_length
                    < (*img).num_lines as libc::c_int * 384 as libc::c_int
                        + 64 as libc::c_int
            {
                g_log(
                    b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_CRITICAL,
                    b"bad captured image (%d lines) or size mismatch %d < %d\0"
                        as *const u8 as *const libc::c_char,
                    (*img).num_lines as libc::c_int,
                    (*self_0).img_data_actual_length,
                    (*img).num_lines as libc::c_int * 384 as libc::c_int
                        + 64 as libc::c_int,
                );
                fpi_ssm_jump_to_state(ssm, IMAGING_CAPTURE as libc::c_int);
                return;
            }
            dev2 = calc_dev2(img);
            g_log(
                b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"dev2: %d\0" as *const u8 as *const libc::c_char,
                dev2,
            );
            if dev2 < 5000 as libc::c_int {
                fpi_ssm_jump_to_state(ssm, IMAGING_REPORT_IMAGE as libc::c_int);
                return;
            }
            g_log(
                b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"image seems to be encrypted\0" as *const u8 as *const libc::c_char,
            );
            buf[0 as libc::c_int as usize] = (*img).key_number;
            buf[1 as libc::c_int as usize] = (*self_0).img_enc_seed as libc::c_uchar;
            buf[2 as libc::c_int
                as usize] = ((*self_0).img_enc_seed >> 8 as libc::c_int)
                as libc::c_uchar;
            buf[3 as libc::c_int
                as usize] = ((*self_0).img_enc_seed >> 16 as libc::c_int)
                as libc::c_uchar;
            buf[4 as libc::c_int
                as usize] = ((*self_0).img_enc_seed >> 24 as libc::c_int)
                as libc::c_uchar;
            sm_write_regs(
                ssm,
                dev,
                REG_SCRAMBLE_DATA_INDEX as libc::c_int as uint16_t,
                5 as libc::c_int as uint16_t,
                buf.as_mut_ptr() as *mut libc::c_void,
            );
        }
        2 => {
            sm_read_regs(
                ssm,
                dev,
                REG_SCRAMBLE_DATA_KEY as libc::c_int as uint16_t,
                4 as libc::c_int as uint16_t,
            );
        }
        3 => {
            key = (*self_0).last_reg_rd[0 as libc::c_int as usize] as uint32_t;
            key
                |= (((*self_0).last_reg_rd[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int) as libc::c_uint;
            key
                |= (((*self_0).last_reg_rd[2 as libc::c_int as usize] as libc::c_int)
                    << 16 as libc::c_int) as libc::c_uint;
            key
                |= (((*self_0).last_reg_rd[3 as libc::c_int as usize] as libc::c_int)
                    << 24 as libc::c_int) as libc::c_uint;
            key ^= (*self_0).img_enc_seed;
            g_log(
                b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"encryption id %02x -> key %08x\0" as *const u8 as *const libc::c_char,
                (*img).key_number as libc::c_int,
                key,
            );
            while ((*self_0).img_block as libc::c_ulong)
                < (::core::mem::size_of::<[C2RustUnnamed_11; 15]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong,
                    )
                && ((*self_0).img_lines_done as libc::c_int)
                    < (*img).num_lines as libc::c_int
            {
                flags = (*img).block_info[(*self_0).img_block as usize].flags;
                num_lines = (*img).block_info[(*self_0).img_block as usize].num_lines;
                if num_lines as libc::c_int == 0 as libc::c_int {
                    break;
                }
                g_log(
                    b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"%d %02x %d\0" as *const u8 as *const libc::c_char,
                    (*self_0).img_block as libc::c_int,
                    flags as libc::c_int,
                    num_lines as libc::c_int,
                );
                if flags as libc::c_int & BLOCKF_CHANGE_KEY as libc::c_int != 0 {
                    g_log(
                        b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"changing encryption keys.\0" as *const u8
                            as *const libc::c_char,
                    );
                    (*img)
                        .block_info[(*self_0).img_block as usize]
                        .flags = ((*img).block_info[(*self_0).img_block as usize].flags
                        as libc::c_int & !(BLOCKF_CHANGE_KEY as libc::c_int)) as uint8_t;
                    (*img).key_number = ((*img).key_number).wrapping_add(1);
                    (*self_0)
                        .img_enc_seed = g_rand_int_range(
                        (*self_0).rand,
                        0 as libc::c_int,
                        2147483647 as libc::c_int,
                    ) as uint32_t;
                    g_log(
                        b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"New image encryption seed: %d\0" as *const u8
                            as *const libc::c_char,
                        (*self_0).img_enc_seed,
                    );
                    fpi_ssm_jump_to_state(ssm, IMAGING_SEND_INDEX as libc::c_int);
                    return;
                }
                match flags as libc::c_int
                    & (BLOCKF_NO_KEY_UPDATE as libc::c_int
                        | BLOCKF_ENCRYPTED as libc::c_int)
                {
                    2 => {
                        g_log(
                            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                            G_LOG_LEVEL_DEBUG,
                            b"decoding %d lines\0" as *const u8 as *const libc::c_char,
                            num_lines as libc::c_int,
                        );
                        key = do_decode(
                            &mut *(*((*img).data)
                                .as_mut_ptr()
                                .offset((*self_0).img_lines_done as isize))
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize),
                            384 as libc::c_int * num_lines as libc::c_int,
                            key,
                        );
                    }
                    0 => {
                        g_log(
                            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                            G_LOG_LEVEL_DEBUG,
                            b"skipping %d lines\0" as *const u8 as *const libc::c_char,
                            num_lines as libc::c_int,
                        );
                        r = 0 as libc::c_int;
                        while r < 384 as libc::c_int * num_lines as libc::c_int {
                            key = update_key(key);
                            r += 1;
                        }
                    }
                    _ => {}
                }
                if flags as libc::c_int & BLOCKF_NOT_PRESENT as libc::c_int
                    == 0 as libc::c_int
                {
                    (*self_0)
                        .img_lines_done = ((*self_0).img_lines_done as libc::c_int
                        + num_lines as libc::c_int) as uint16_t;
                }
                (*self_0).img_block = ((*self_0).img_block).wrapping_add(1);
            }
            fpi_ssm_next_state(ssm);
        }
        4 => {
            fpimg = fp_image_new(384 as libc::c_int, 290 as libc::c_int);
            r = 0 as libc::c_int;
            to = r;
            i = 0 as libc::c_int;
            while (i as libc::c_ulong)
                < (::core::mem::size_of::<[C2RustUnnamed_11; 15]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong,
                    ) && r < (*img).num_lines as libc::c_int
            {
                flags = (*img).block_info[i as usize].flags;
                num_lines = (*img).block_info[i as usize].num_lines;
                if num_lines as libc::c_int == 0 as libc::c_int {
                    break;
                }
                memcpy(
                    &mut *((*fpimg).data).offset(to as isize) as *mut guint8
                        as *mut libc::c_void,
                    &mut *(*((*img).data).as_mut_ptr().offset(r as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut uint8_t
                        as *const libc::c_void,
                    (num_lines as libc::c_int * 384 as libc::c_int) as libc::c_ulong,
                );
                if flags as libc::c_int & BLOCKF_NOT_PRESENT as libc::c_int == 0 {
                    r += num_lines as libc::c_int;
                }
                to += num_lines as libc::c_int * 384 as libc::c_int;
                i += 1;
            }
            (*fpimg).flags = FPI_IMAGE_COLORS_INVERTED;
            if (*(*self_0).profile).image_not_flipped != 0 {
                (*fpimg)
                    .flags = ::core::mem::transmute::<
                    libc::c_uint,
                    FpiImageFlags,
                >(
                    (*fpimg).flags as libc::c_uint
                        | (FPI_IMAGE_V_FLIPPED as libc::c_int
                            | FPI_IMAGE_H_FLIPPED as libc::c_int) as libc::c_uint,
                );
            }
            fpi_image_device_image_captured(dev, fpimg);
            if (*self_0).activate_state as libc::c_uint
                == FPI_IMAGE_DEVICE_STATE_CAPTURE as libc::c_int as libc::c_uint
            {
                fpi_ssm_jump_to_state(ssm, IMAGING_CAPTURE as libc::c_int);
            } else {
                fpi_ssm_mark_completed(ssm);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn imaging_complete(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut self_0: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(dev as gpointer);
    if !error.is_null() {
        fpi_image_device_session_error(FP_IMAGE_DEVICE(dev as gpointer), error);
    }
    let mut _pp: C2RustUnnamed_10 = C2RustUnnamed_10 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut FpiUsbTransfer) -> ()>,
        GDestroyNotify,
    >(Some(fpi_usb_transfer_unref as unsafe extern "C" fn(*mut FpiUsbTransfer) -> ()));
    _pp
        .in_0 = &mut (*self_0).img_transfer as *mut *mut FpiUsbTransfer
        as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    g_free((*self_0).img_data);
    (*self_0).img_data = 0 as *mut libc::c_void;
    (*self_0).img_data_actual_length = 0 as libc::c_int;
    execute_state_change(FP_IMAGE_DEVICE(dev as gpointer));
}
unsafe extern "C" fn rebootpwr_run_state(mut ssm: *mut FpiSsm, mut _dev: *mut FpDevice) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(_dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            (*self_0).rebootpwr_ctr = 100 as libc::c_int;
            sm_set_hwstat(
                ssm,
                dev,
                ((*self_0).last_hwstat as libc::c_int & 0xf as libc::c_int)
                    as libc::c_uchar,
            );
        }
        1 => {
            sm_read_reg(ssm, dev, REG_HWSTAT as libc::c_int as uint16_t);
        }
        2 => {
            (*self_0).last_hwstat = (*self_0).last_reg_rd[0 as libc::c_int as usize];
            if (*self_0).last_hwstat as libc::c_int & 0x1 as libc::c_int != 0 {
                fpi_ssm_mark_completed(ssm);
            } else {
                fpi_ssm_next_state(ssm);
            }
        }
        3 => {
            (*self_0).rebootpwr_ctr -= 1;
            if (*self_0).rebootpwr_ctr == 0 {
                g_log(
                    b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_CRITICAL,
                    b"could not reboot device power\0" as *const u8
                        as *const libc::c_char,
                );
                fpi_ssm_mark_failed(
                    ssm,
                    fpi_device_error_new_msg(
                        fp_device_error_quark() as FpDeviceError,
                        b"Could not reboot device\0" as *const u8 as *const libc::c_char,
                    ),
                );
            } else {
                fpi_ssm_jump_to_state_delayed(
                    ssm,
                    10 as libc::c_int,
                    REBOOTPWR_GET_HWSTAT as libc::c_int,
                );
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn powerup_run_state(mut ssm: *mut FpiSsm, mut _dev: *mut FpDevice) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(_dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            (*self_0).powerup_ctr = 100 as libc::c_int;
            (*self_0)
                .powerup_hwstat = ((*self_0).last_hwstat as libc::c_int
                & 0xf as libc::c_int) as libc::c_uchar;
            fpi_ssm_next_state(ssm);
        }
        1 => {
            sm_set_hwstat(ssm, dev, (*self_0).powerup_hwstat);
        }
        2 => {
            sm_read_reg(ssm, dev, REG_HWSTAT as libc::c_int as uint16_t);
        }
        3 => {
            (*self_0).last_hwstat = (*self_0).last_reg_rd[0 as libc::c_int as usize];
            if (*self_0).last_reg_rd[0 as libc::c_int as usize] as libc::c_int
                & 0x80 as libc::c_int == 0 as libc::c_int
            {
                fpi_ssm_mark_completed(ssm);
            } else {
                fpi_ssm_next_state(ssm);
            }
        }
        4 => {
            (*self_0).powerup_ctr -= 1;
            if (*self_0).powerup_ctr == 0 {
                g_log(
                    b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_CRITICAL,
                    b"could not power device up\0" as *const u8 as *const libc::c_char,
                );
                fpi_ssm_mark_failed(
                    ssm,
                    fpi_device_error_new_msg(
                        FP_DEVICE_ERROR_GENERAL,
                        b"could not power device up\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
            } else if (*(*self_0).profile).auth_cr == 0 {
                fpi_ssm_jump_to_state_delayed(
                    ssm,
                    POWERUP_SET_HWSTAT as libc::c_int,
                    10 as libc::c_int,
                );
            } else {
                fpi_ssm_next_state_delayed(ssm, 10 as libc::c_int);
            }
        }
        5 => {
            sm_do_challenge_response(ssm, dev);
        }
        6 => {
            fpi_ssm_jump_to_state(ssm, POWERUP_SET_HWSTAT as libc::c_int);
        }
        _ => {}
    };
}
unsafe extern "C" fn init_scanpwr_irq_cb(
    mut dev: *mut FpImageDevice,
    mut error: *mut GError,
    mut type_0: uint16_t,
    mut user_data: *mut libc::c_void,
) {
    let mut ssm: *mut FpiSsm = user_data as *mut FpiSsm;
    let mut urudev: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(dev as gpointer);
    if !error.is_null() {
        fpi_ssm_mark_failed(ssm, error);
    } else if type_0 as libc::c_int != IRQDATA_SCANPWR_ON as libc::c_int {
        g_log(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"ignoring interrupt\0" as *const u8 as *const libc::c_char,
        );
    } else if fpi_ssm_get_cur_state(ssm) != INIT_AWAIT_SCAN_POWER as libc::c_int {
        g_log(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"early scanpwr interrupt\0" as *const u8 as *const libc::c_char,
        );
        (*urudev).scanpwr_irq_timeouts = -(1 as libc::c_int);
    } else {
        g_log(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"late scanpwr interrupt\0" as *const u8 as *const libc::c_char,
        );
        fpi_ssm_next_state(ssm);
    };
}
unsafe extern "C" fn init_scanpwr_timeout(
    mut dev: *mut FpDevice,
    mut user_data: *mut libc::c_void,
) {
    let mut ssm: *mut FpiSsm = user_data as *mut FpiSsm;
    let mut self_0: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(dev as gpointer);
    g_log(
        b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_WARNING,
        b"powerup timed out\0" as *const u8 as *const libc::c_char,
    );
    (*self_0).irq_cb = None;
    (*self_0).scanpwr_irq_timeout = 0 as *mut GSource;
    (*self_0).scanpwr_irq_timeouts += 1;
    if (*self_0).scanpwr_irq_timeouts >= 3 as libc::c_int {
        g_log(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"powerup timed out 3 times, giving up\0" as *const u8 as *const libc::c_char,
        );
        fpi_ssm_mark_failed(
            ssm,
            g_error_new_literal(
                g_usb_device_error_quark(),
                G_USB_DEVICE_ERROR_TIMED_OUT as libc::c_int,
                b"Powerup timed out 3 times, giving up\0" as *const u8
                    as *const libc::c_char,
            ),
        );
    } else {
        fpi_ssm_jump_to_state(ssm, INIT_GET_HWSTAT as libc::c_int);
    };
}
unsafe extern "C" fn init_run_state(mut ssm: *mut FpiSsm, mut _dev: *mut FpDevice) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(_dev as gpointer);
    let mut rebootsm: *mut FpiSsm = 0 as *mut FpiSsm;
    let mut powerupsm: *mut FpiSsm = 0 as *mut FpiSsm;
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            sm_read_reg(ssm, dev, REG_HWSTAT as libc::c_int as uint16_t);
        }
        1 => {
            (*self_0).last_hwstat = (*self_0).last_reg_rd[0 as libc::c_int as usize];
            if (*self_0).last_hwstat as libc::c_int & 0x84 as libc::c_int
                == 0x84 as libc::c_int
            {
                fpi_ssm_next_state(ssm);
            } else {
                fpi_ssm_jump_to_state(ssm, INIT_CHECK_HWSTAT_POWERDOWN as libc::c_int);
            }
        }
        2 => {
            rebootsm = fpi_ssm_new_full(
                FP_DEVICE(dev as gpointer),
                Some(
                    rebootpwr_run_state
                        as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                ),
                REBOOTPWR_NUM_STATES as libc::c_int,
                REBOOTPWR_NUM_STATES as libc::c_int,
                b"REBOOTPWR_NUM_STATES\0" as *const u8 as *const libc::c_char,
            );
            fpi_ssm_start_subsm(ssm, rebootsm);
        }
        3 => {
            if (*self_0).last_hwstat as libc::c_int & 0x80 as libc::c_int
                == 0 as libc::c_int
            {
                sm_set_hwstat(
                    ssm,
                    dev,
                    ((*self_0).last_hwstat as libc::c_int | 0x80 as libc::c_int)
                        as libc::c_uchar,
                );
            } else {
                fpi_ssm_next_state(ssm);
            }
        }
        4 => {
            if ((*self_0).irq_cancellable).is_null() {
                fpi_ssm_mark_failed(
                    ssm,
                    fpi_device_error_new_msg(
                        FP_DEVICE_ERROR_GENERAL,
                        b"IRQ handler should be running but is not\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                return;
            }
            (*self_0).irq_cb_data = ssm as *mut libc::c_void;
            (*self_0)
                .irq_cb = Some(
                init_scanpwr_irq_cb
                    as unsafe extern "C" fn(
                        *mut FpImageDevice,
                        *mut GError,
                        uint16_t,
                        *mut libc::c_void,
                    ) -> (),
            );
            powerupsm = fpi_ssm_new_full(
                FP_DEVICE(dev as gpointer),
                Some(
                    powerup_run_state
                        as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                ),
                POWERUP_NUM_STATES as libc::c_int,
                POWERUP_NUM_STATES as libc::c_int,
                b"POWERUP_NUM_STATES\0" as *const u8 as *const libc::c_char,
            );
            fpi_ssm_start_subsm(ssm, powerupsm);
        }
        5 => {
            if (*self_0).scanpwr_irq_timeouts < 0 as libc::c_int {
                fpi_ssm_next_state(ssm);
            } else {
                (*self_0)
                    .scanpwr_irq_timeout = fpi_device_add_timeout(
                    _dev,
                    300 as libc::c_int,
                    Some(
                        init_scanpwr_timeout
                            as unsafe extern "C" fn(
                                *mut FpDevice,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                    ssm as gpointer,
                    None,
                );
            }
        }
        6 => {
            if !((*self_0).scanpwr_irq_timeout).is_null() {
                g_source_destroy((*self_0).scanpwr_irq_timeout);
                (*self_0).scanpwr_irq_timeout = 0 as *mut GSource;
            }
            (*self_0).irq_cb_data = 0 as *mut libc::c_void;
            (*self_0).irq_cb = None;
            fpi_ssm_next_state(ssm);
        }
        7 => {
            sm_read_regs(
                ssm,
                dev,
                REG_DEVICE_INFO as libc::c_int as uint16_t,
                16 as libc::c_int as uint16_t,
            );
        }
        8 => {
            g_log(
                b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Versions %02x%02x and %02x%02x\0" as *const u8 as *const libc::c_char,
                (*self_0).last_reg_rd[10 as libc::c_int as usize] as libc::c_int,
                (*self_0).last_reg_rd[11 as libc::c_int as usize] as libc::c_int,
                (*self_0).last_reg_rd[4 as libc::c_int as usize] as libc::c_int,
                (*self_0).last_reg_rd[5 as libc::c_int as usize] as libc::c_int,
            );
            fpi_ssm_mark_completed(ssm);
        }
        _ => {}
    };
}
unsafe extern "C" fn activate_initsm_complete(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    fpi_image_device_activate_complete(FP_IMAGE_DEVICE(dev as gpointer), error);
}
unsafe extern "C" fn dev_activate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(dev as gpointer);
    let mut ssm: *mut FpiSsm = 0 as *mut FpiSsm;
    start_irq_handler(dev);
    (*self_0).scanpwr_irq_timeouts = 0 as libc::c_int;
    ssm = fpi_ssm_new_full(
        FP_DEVICE(dev as gpointer),
        Some(init_run_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
        INIT_NUM_STATES as libc::c_int,
        INIT_NUM_STATES as libc::c_int,
        b"INIT_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            activate_initsm_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn deactivate_irqs_stopped(mut dev: *mut FpImageDevice) {
    fpi_image_device_deactivate_complete(dev, 0 as *mut GError);
}
unsafe extern "C" fn deactivate_write_reg_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut dev: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    stop_irq_handler(
        FP_IMAGE_DEVICE(dev as gpointer),
        Some(deactivate_irqs_stopped as unsafe extern "C" fn(*mut FpImageDevice) -> ()),
    );
}
unsafe extern "C" fn dev_deactivate(mut dev: *mut FpImageDevice) {}
unsafe extern "C" fn execute_state_change(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(dev as gpointer);
    let mut ssm: *mut FpiSsm = 0 as *mut FpiSsm;
    match (*self_0).activate_state as libc::c_uint {
        2 => {
            g_log(
                b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"deactivating\0" as *const u8 as *const libc::c_char,
            );
            (*self_0).irq_cb = None;
            (*self_0).irq_cb_data = 0 as *mut libc::c_void;
            write_reg(
                dev,
                REG_MODE as libc::c_int as uint16_t,
                MODE_OFF as libc::c_int as libc::c_uchar,
                Some(
                    deactivate_write_reg_cb
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
            g_log(
                b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"wait finger on\0" as *const u8 as *const libc::c_char,
            );
            if ((*self_0).irq_cancellable).is_null() {
                fpi_image_device_session_error(
                    dev,
                    fpi_device_error_new_msg(
                        FP_DEVICE_ERROR_GENERAL,
                        b"IRQ handler should be running but is not\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                return;
            }
            (*self_0)
                .irq_cb = Some(
                finger_presence_irq_cb
                    as unsafe extern "C" fn(
                        *mut FpImageDevice,
                        *mut GError,
                        uint16_t,
                        *mut libc::c_void,
                    ) -> (),
            );
            write_reg(
                dev,
                REG_MODE as libc::c_int as uint16_t,
                MODE_AWAIT_FINGER_ON as libc::c_int as libc::c_uchar,
                Some(
                    change_state_write_reg_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            *mut libc::c_void,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
        }
        5 => {
            g_log(
                b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"starting capture\0" as *const u8 as *const libc::c_char,
            );
            (*self_0).irq_cb = None;
            ssm = fpi_ssm_new_full(
                FP_DEVICE(dev as gpointer),
                Some(
                    imaging_run_state
                        as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
                ),
                IMAGING_NUM_STATES as libc::c_int,
                IMAGING_NUM_STATES as libc::c_int,
                b"IMAGING_NUM_STATES\0" as *const u8 as *const libc::c_char,
            );
            (*self_0)
                .img_enc_seed = g_rand_int_range(
                (*self_0).rand,
                0 as libc::c_int,
                2147483647 as libc::c_int,
            ) as uint32_t;
            g_log(
                b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Image encryption seed: %d\0" as *const u8 as *const libc::c_char,
                (*self_0).img_enc_seed,
            );
            (*self_0).img_transfer = fpi_usb_transfer_new(FP_DEVICE(dev as gpointer));
            (*(*self_0).img_transfer).ssm = ssm;
            (*(*self_0).img_transfer).short_is_error = 0 as libc::c_int;
            fpi_usb_transfer_fill_bulk(
                (*self_0).img_transfer,
                (2 as libc::c_int | 0x80 as libc::c_int) as guint8,
                ::core::mem::size_of::<uru4k_image>() as libc::c_ulong,
            );
            fpi_ssm_start(
                ssm,
                Some(
                    imaging_complete
                        as unsafe extern "C" fn(
                            *mut FpiSsm,
                            *mut FpDevice,
                            *mut GError,
                        ) -> (),
                ),
            );
            write_reg(
                dev,
                REG_MODE as libc::c_int as uint16_t,
                MODE_CAPTURE as libc::c_int as libc::c_uchar,
                Some(
                    change_state_write_reg_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            *mut libc::c_void,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
        }
        6 => {
            g_log(
                b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"await finger off\0" as *const u8 as *const libc::c_char,
            );
            if ((*self_0).irq_cancellable).is_null() {
                fpi_image_device_session_error(
                    dev,
                    fpi_device_error_new_msg(
                        FP_DEVICE_ERROR_GENERAL,
                        b"IRQ handler should be running but is not\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                return;
            }
            (*self_0)
                .irq_cb = Some(
                finger_presence_irq_cb
                    as unsafe extern "C" fn(
                        *mut FpImageDevice,
                        *mut GError,
                        uint16_t,
                        *mut libc::c_void,
                    ) -> (),
            );
            write_reg(
                dev,
                REG_MODE as libc::c_int as uint16_t,
                MODE_AWAIT_FINGER_OFF as libc::c_int as libc::c_uchar,
                Some(
                    change_state_write_reg_cb
                        as unsafe extern "C" fn(
                            *mut FpiUsbTransfer,
                            *mut FpDevice,
                            *mut libc::c_void,
                            *mut GError,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
        }
        3 | 1 | 0 | _ => {}
    };
}
unsafe extern "C" fn dev_init(mut dev: *mut FpImageDevice) {
    let mut error: *mut GError = 0 as *mut GError;
    let mut self_0: *mut FpiDeviceUru4000 = 0 as *mut FpiDeviceUru4000;
    let mut interfaces: GPtrArray_autoptr = 0 as GPtrArray_autoptr;
    let mut iface: *mut GUsbInterface = 0 as *mut GUsbInterface;
    let mut driver_data: guint64 = 0;
    let mut rv: SECStatus = SECSuccess;
    let mut item: SECItem = SECItem {
        type_0: siBuffer,
        data: 0 as *mut libc::c_uchar,
        len: 0,
    };
    let mut i: libc::c_int = 0;
    interfaces = g_usb_device_get_interfaces(
        fpi_device_get_usb_device(FP_DEVICE(dev as gpointer)),
        &mut error,
    );
    if !error.is_null() {
        fpi_image_device_open_complete(dev, error);
        return;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*interfaces).len {
        let mut cur_iface: *mut GUsbInterface = *((*interfaces).pdata).offset(i as isize)
            as *mut GUsbInterface;
        if g_usb_interface_get_class(cur_iface) as libc::c_int == 255 as libc::c_int
            && g_usb_interface_get_subclass(cur_iface) as libc::c_int
                == 255 as libc::c_int
            && g_usb_interface_get_protocol(cur_iface) as libc::c_int
                == 255 as libc::c_int
        {
            iface = cur_iface;
            break;
        } else {
            i += 1;
        }
    }
    if iface.is_null() {
        g_log(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"could not find interface\0" as *const u8 as *const libc::c_char,
        );
        fpi_image_device_open_complete(
            dev,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_GENERAL,
                b"Could not find interface\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    if g_usb_device_claim_interface(
        fpi_device_get_usb_device(FP_DEVICE(dev as gpointer)),
        g_usb_interface_get_number(iface) as gint,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    ) == 0
    {
        fpi_image_device_open_complete(dev, error);
        return;
    }
    g_setenv(
        b"P11_KIT_NO_USER_CONFIG\0" as *const u8 as *const libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    rv = NSS_NoDB_Init(b".\0" as *const u8 as *const libc::c_char);
    if rv as libc::c_int != SECSuccess as libc::c_int {
        g_log(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"could not initialise NSS\0" as *const u8 as *const libc::c_char,
        );
        fpi_image_device_open_complete(
            dev,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_GENERAL,
                b"Could not initialise NSS\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    self_0 = FPI_DEVICE_URU4000(dev as gpointer);
    let mut _pp: C2RustUnnamed_14 = C2RustUnnamed_14 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GRand) -> ()>,
        GDestroyNotify,
    >(Some(g_rand_free as unsafe extern "C" fn(*mut GRand) -> ()));
    _pp.in_0 = &mut (*self_0).rand as *mut *mut GRand as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    (*self_0).rand = g_rand_new();
    if g_strcmp0(
        g_getenv(b"FP_DEVICE_EMULATION\0" as *const u8 as *const libc::c_char),
        b"1\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        g_rand_set_seed((*self_0).rand, 0xfacade as libc::c_int as guint32);
    }
    driver_data = fpi_device_get_driver_data(FP_DEVICE(dev as gpointer));
    (*self_0)
        .profile = &*uru4k_dev_info.as_ptr().offset(driver_data as isize)
        as *const uru4k_dev_profile;
    (*self_0).interface = g_usb_interface_get_number(iface);
    (*self_0).cipher = 0x1081 as libc::c_ulong;
    (*self_0).slot = PK11_GetBestSlot((*self_0).cipher, 0 as *mut libc::c_void);
    if ((*self_0).slot).is_null() {
        g_log(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"could not get encryption slot\0" as *const u8 as *const libc::c_char,
        );
        fpi_image_device_open_complete(
            dev,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_GENERAL,
                b"Could not get encryption slot\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    item.type_0 = siBuffer;
    item.data = crkey.as_ptr() as *mut libc::c_uchar;
    item
        .len = ::core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong
        as libc::c_uint;
    (*self_0)
        .symkey = PK11_ImportSymKey(
        (*self_0).slot,
        (*self_0).cipher,
        PK11_OriginUnwrap,
        0x104 as libc::c_ulong,
        &mut item,
        0 as *mut libc::c_void,
    );
    if ((*self_0).symkey).is_null() {
        g_log(
            b"libfprint-uru4000\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"failed to import key into NSS\0" as *const u8 as *const libc::c_char,
        );
        PK11_FreeSlot((*self_0).slot);
        (*self_0).slot = 0 as *mut PK11SlotInfo;
        fpi_image_device_open_complete(
            dev,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_GENERAL,
                b"Failed to import key into NSS\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    (*self_0).param = PK11_ParamFromIV((*self_0).cipher, 0 as *mut SECItem);
    fpi_image_device_open_complete(dev, 0 as *mut GError);
}
unsafe extern "C" fn dev_deinit(mut dev: *mut FpImageDevice) {
    let mut error: *mut GError = 0 as *mut GError;
    let mut self_0: *mut FpiDeviceUru4000 = FPI_DEVICE_URU4000(dev as gpointer);
    if !((*self_0).symkey).is_null() {
        PK11_FreeSymKey((*self_0).symkey);
    }
    if !((*self_0).param).is_null() {
        SECITEM_FreeItem((*self_0).param, 1 as libc::c_int);
    }
    if !((*self_0).slot).is_null() {
        PK11_FreeSlot((*self_0).slot);
    }
    NSS_Shutdown();
    g_usb_device_release_interface(
        fpi_device_get_usb_device(FP_DEVICE(dev as gpointer)),
        (*self_0).interface as gint,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    );
    let mut _pp: C2RustUnnamed_13 = C2RustUnnamed_13 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GRand) -> ()>,
        GDestroyNotify,
    >(Some(g_rand_free as unsafe extern "C" fn(*mut GRand) -> ()));
    _pp.in_0 = &mut (*self_0).rand as *mut *mut GRand as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    fpi_image_device_close_complete(dev, error);
}
static mut id_table: [FpIdEntry; 8] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_2 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_5 {
                        pid: 0xbb as libc::c_int as guint,
                        vid: 0x45e as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: MS_KBD as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_2 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_5 {
                        pid: 0xbc as libc::c_int as guint,
                        vid: 0x45e as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: MS_INTELLIMOUSE as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_2 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_5 {
                        pid: 0xbd as libc::c_int as guint,
                        vid: 0x45e as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: MS_STANDALONE as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_2 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_5 {
                        pid: 0xca as libc::c_int as guint,
                        vid: 0x45e as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: MS_STANDALONE_V2 as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_2 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_5 {
                        pid: 0x7 as libc::c_int as guint,
                        vid: 0x5ba as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: DP_URU4000 as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_2 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_5 {
                        pid: 0x8 as libc::c_int as guint,
                        vid: 0x5ba as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: DP_URU4000 as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_2 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_5 {
                        pid: 0xa as libc::c_int as guint,
                        vid: 0x5ba as libc::c_int as guint,
                    };
                    init
                },
            },
            driver_data: DP_URU4000B as libc::c_int as guint64,
        };
        init
    },
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_2 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_5 {
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
unsafe extern "C" fn fpi_device_uru4000_init(mut self_0: *mut FpiDeviceUru4000) {}
unsafe extern "C" fn fpi_device_uru4000_class_init(
    mut klass: *mut FpiDeviceUru4000Class,
) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    let mut img_class: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_CLASS(
        klass as gpointer,
    );
    (*dev_class).id = b"uru4000\0" as *const u8 as *const libc::c_char;
    (*dev_class)
        .full_name = b"Digital Persona U.are.U 4000/4000B/4500\0" as *const u8
        as *const libc::c_char;
    (*dev_class).type_0 = FP_DEVICE_TYPE_USB;
    (*dev_class).id_table = id_table.as_ptr();
    (*dev_class).scan_type = FP_SCAN_TYPE_PRESS;
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
    (*img_class).img_width = 384 as libc::c_int;
    (*img_class).img_height = 290 as libc::c_int;
}
