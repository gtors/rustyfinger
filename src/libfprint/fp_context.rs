use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GAsyncInitable;
    pub type _GCancellablePrivate;
    pub type _GUdevClientPrivate;
    pub type _GUdevDevicePrivate;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn g_array_unref(array: *mut GArray);
    fn g_array_remove_index(array: *mut GArray, index_: guint) -> *mut GArray;
    fn g_ptr_array_new_with_free_func(
        element_free_func: GDestroyNotify,
    ) -> *mut GPtrArray;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_remove_index_fast(array: *mut GPtrArray, index_: guint) -> gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_ptr_array_find(
        haystack: *mut GPtrArray,
        needle: gconstpointer,
        index_: *mut guint,
    ) -> gboolean;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_delete_link(list: *mut GList, link_: *mut GList) -> *mut GList;
    fn g_list_foreach(list: *mut GList, func: GFunc, user_data: gpointer);
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_remove(list: *mut GSList, data: gconstpointer) -> *mut GSList;
    fn g_main_context_iteration(
        context: *mut GMainContext,
        may_block: gboolean,
    ) -> gboolean;
    fn g_main_context_get_thread_default() -> *mut GMainContext;
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_destroy(source: *mut GSource);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_idle_source_new() -> *mut GSource;
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn g_return_if_fail_warning(
        log_domain: *const libc::c_char,
        pretty_function: *const libc::c_char,
        expression: *const libc::c_char,
    );
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strcmp0(str1: *const libc::c_char, str2: *const libc::c_char) -> libc::c_int;
    fn g_type_class_ref(type_0: GType) -> gpointer;
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
    fn g_type_check_instance_is_a(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> gboolean;
    fn g_type_check_class_cast(
        g_class: *mut GTypeClass,
        is_a_type: GType,
    ) -> *mut GTypeClass;
    fn g_cclosure_marshal_VOID__OBJECT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn g_signal_new(
        signal_name: *const gchar,
        itype: GType,
        signal_flags: GSignalFlags,
        class_offset: guint,
        accumulator: GSignalAccumulator,
        accu_data: gpointer,
        c_marshaller: GSignalCMarshaller,
        return_type: GType,
        n_params: guint,
        _: ...
    ) -> guint;
    fn g_signal_emit(instance: gpointer, signal_id: guint, detail: GQuark, _: ...);
    fn g_object_new(
        object_type: GType,
        first_property_name: *const gchar,
        _: ...
    ) -> gpointer;
    fn g_object_get(object: gpointer, first_property_name: *const gchar, _: ...);
    fn g_object_unref(object: gpointer);
    fn g_signal_connect_object(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        gobject: gpointer,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_object_run_dispose(object: *mut GObject);
    fn g_async_initable_get_type() -> GType;
    fn g_async_initable_new_async(
        object_type: GType,
        io_priority: libc::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
        first_property_name: *const gchar,
        _: ...
    );
    fn g_async_initable_new_finish(
        initable: *mut GAsyncInitable,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GObject;
    fn g_cancellable_new() -> *mut GCancellable;
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
    fn g_io_error_quark() -> GQuark;
    fn g_usb_device_get_vid(self_0: *mut GUsbDevice) -> guint16;
    fn g_usb_device_get_pid(self_0: *mut GUsbDevice) -> guint16;
    fn g_usb_context_new(error: *mut *mut GError) -> *mut GUsbContext;
    fn g_usb_context_enumerate(self_0: *mut GUsbContext);
    fn g_usb_context_set_debug(self_0: *mut GUsbContext, flags: GLogLevelFlags);
    fn fp_device_get_type() -> GType;
    fn fpi_get_driver_types() -> *mut GArray;
    fn fpi_device_remove(device: *mut FpDevice);
    fn fpi_device_get_usb_device(device: *mut FpDevice) -> *mut GUsbDevice;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn g_udev_device_get_sysfs_path(device: *mut GUdevDevice) -> *const gchar;
    fn g_udev_device_get_device_file(device: *mut GUdevDevice) -> *const gchar;
    fn g_udev_device_get_parent_with_subsystem(
        device: *mut GUdevDevice,
        subsystem: *const gchar,
        devtype: *const gchar,
    ) -> *mut GUdevDevice;
    fn g_udev_client_query_by_subsystem(
        client: *mut GUdevClient,
        subsystem: *const gchar,
    ) -> *mut GList;
    fn g_udev_device_get_property(
        device: *mut GUdevDevice,
        key: *const gchar,
    ) -> *const gchar;
    fn g_udev_client_new(subsystems: *const *const gchar) -> *mut GUdevClient;
}
pub type guint8 = libc::c_uchar;
pub type guint16 = libc::c_ushort;
pub type gint32 = libc::c_int;
pub type guint32 = libc::c_uint;
pub type gint64 = libc::c_long;
pub type guint64 = libc::c_ulong;
pub type gsize = libc::c_ulong;
pub type gchar = libc::c_char;
pub type glong = libc::c_long;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type gulong = libc::c_ulong;
pub type guint = libc::c_uint;
pub type gfloat = libc::c_float;
pub type gdouble = libc::c_double;
pub type gpointer = *mut libc::c_void;
pub type gconstpointer = *const libc::c_void;
pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
pub type GFunc = Option::<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
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
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
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
pub type GStrv = *mut *mut gchar;
pub type GError_autoptr = *mut GError;
pub type GList_autoptr = *mut GList;
pub type GSource_autoptr = *mut GSource;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GClosure {
    #[bitfield(name = "ref_count", ty = "guint", bits = "0..=14")]
    #[bitfield(name = "meta_marshal_nouse", ty = "guint", bits = "15..=15")]
    #[bitfield(name = "n_guards", ty = "guint", bits = "16..=16")]
    #[bitfield(name = "n_fnotifiers", ty = "guint", bits = "17..=18")]
    #[bitfield(name = "n_inotifiers", ty = "guint", bits = "19..=26")]
    #[bitfield(name = "in_inotify", ty = "guint", bits = "27..=27")]
    #[bitfield(name = "floating", ty = "guint", bits = "28..=28")]
    #[bitfield(name = "derivative_flag", ty = "guint", bits = "29..=29")]
    #[bitfield(name = "in_marshal", ty = "guint", bits = "30..=30")]
    #[bitfield(name = "is_invalid", ty = "guint", bits = "31..=31")]
    pub ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub marshal: Option::<
        unsafe extern "C" fn(
            *mut GClosure,
            *mut GValue,
            guint,
            *const GValue,
            gpointer,
            gpointer,
        ) -> (),
    >,
    pub data: gpointer,
    pub notifiers: *mut GClosureNotifyData,
}
pub type GClosureNotifyData = _GClosureNotifyData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GClosureNotifyData {
    pub data: gpointer,
    pub notify: GClosureNotify,
}
pub type GClosureNotify = Option::<unsafe extern "C" fn(gpointer, *mut GClosure) -> ()>;
pub type GClosure = _GClosure;
pub type GCallback = Option::<unsafe extern "C" fn() -> ()>;
pub type GClosureMarshal = Option::<
    unsafe extern "C" fn(
        *mut GClosure,
        *mut GValue,
        guint,
        *const GValue,
        gpointer,
        gpointer,
    ) -> (),
>;
pub type GSignalFlags = libc::c_uint;
pub const G_SIGNAL_ACCUMULATOR_FIRST_RUN: GSignalFlags = 131072;
pub const G_SIGNAL_DEPRECATED: GSignalFlags = 256;
pub const G_SIGNAL_MUST_COLLECT: GSignalFlags = 128;
pub const G_SIGNAL_NO_HOOKS: GSignalFlags = 64;
pub const G_SIGNAL_ACTION: GSignalFlags = 32;
pub const G_SIGNAL_DETAILED: GSignalFlags = 16;
pub const G_SIGNAL_NO_RECURSE: GSignalFlags = 8;
pub const G_SIGNAL_RUN_CLEANUP: GSignalFlags = 4;
pub const G_SIGNAL_RUN_LAST: GSignalFlags = 2;
pub const G_SIGNAL_RUN_FIRST: GSignalFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSignalInvocationHint {
    pub signal_id: guint,
    pub detail: GQuark,
    pub run_type: GSignalFlags,
}
pub type GSignalInvocationHint = _GSignalInvocationHint;
pub type GSignalCMarshaller = GClosureMarshal;
pub type GSignalAccumulator = Option::<
    unsafe extern "C" fn(
        *mut GSignalInvocationHint,
        *mut GValue,
        *const GValue,
        gpointer,
    ) -> gboolean,
>;
pub type GConnectFlags = libc::c_uint;
pub const G_CONNECT_SWAPPED: GConnectFlags = 2;
pub const G_CONNECT_AFTER: GConnectFlags = 1;
pub const G_CONNECT_DEFAULT: GConnectFlags = 0;
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
pub type GAsyncResult = _GAsyncResult;
pub type GAsyncInitable = _GAsyncInitable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GAsyncReadyCallback = Option::<
    unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUsbDevice {
    pub parent_instance: GObject,
}
pub type GUsbDevice = _GUsbDevice;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUsbContext {
    pub parent_instance: GObject,
}
pub type GUsbContext = _GUsbContext;
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
pub type FpDeviceClass_autoptr = *mut FpDeviceClass;
pub type FpContext = _FpContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpContext {
    pub parent_instance: GObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpContextPrivate {
    pub usb_ctx: *mut GUsbContext,
    pub cancellable: *mut GCancellable,
    pub sources: *mut GSList,
    pub pending_devices: gint,
    pub enumerated: gboolean,
    pub drivers: *mut GArray,
    pub devices: *mut GPtrArray,
}
pub const DEVICE_ADDED_SIGNAL: C2RustUnnamed_9 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RemoveDeviceData {
    pub context: *mut FpContext,
    pub device: *mut FpDevice,
    pub source: *mut GSource,
}
pub const DEVICE_REMOVED_SIGNAL: C2RustUnnamed_9 = 1;
pub type FpContextClass = _FpContextClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpContextClass {
    pub parent_class: GObjectClass,
    pub device_added: Option::<
        unsafe extern "C" fn(*mut FpContext, *mut FpDevice) -> (),
    >,
    pub device_removed: Option::<
        unsafe extern "C" fn(*mut FpContext, *mut FpDevice) -> (),
    >,
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
pub type GUdevClient_autoptr = *mut GUdevClient;
pub type GUdevClient = _GUdevClient;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUdevClient {
    pub parent: GObject,
    pub priv_0: *mut GUdevClientPrivate,
}
pub type GUdevClientPrivate = _GUdevClientPrivate;
pub type GUdevDevice = _GUdevDevice;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUdevDevice {
    pub parent: GObject,
    pub priv_0: *mut GUdevDevicePrivate,
}
pub type GUdevDevicePrivate = _GUdevDevicePrivate;
pub type GUdevDevice_autoptr = *mut GUdevDevice;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_9 = 2;
#[inline]
unsafe extern "C" fn g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = 0 as *mut libc::c_void;
    ref_0 = *ptr;
    *ptr = 0 as *mut libc::c_void;
    return ref_0;
}
#[inline]
unsafe extern "C" fn FP_IS_CONTEXT(mut ptr: gpointer) -> gboolean {
    return ({
        let mut __inst: *mut GTypeInstance = ptr as *mut GTypeInstance;
        let mut __t: GType = fp_context_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = 0 as libc::c_int;
        } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
            __r = (0 as libc::c_int == 0) as libc::c_int;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    });
}
#[inline]
unsafe extern "C" fn FP_DEVICE(mut ptr: gpointer) -> *mut FpDevice {
    return g_type_check_instance_cast(ptr as *mut GTypeInstance, fp_device_get_type())
        as *mut libc::c_void as *mut FpDevice;
}
#[inline]
unsafe extern "C" fn FP_DEVICE_GET_CLASS(mut ptr: gpointer) -> *mut FpDeviceClass {
    return (*(ptr as *mut GTypeInstance)).g_class as *mut FpDeviceClass;
}
#[inline]
unsafe extern "C" fn FP_CONTEXT(mut ptr: gpointer) -> *mut FpContext {
    return g_type_check_instance_cast(ptr as *mut GTypeInstance, fp_context_get_type())
        as *mut libc::c_void as *mut FpContext;
}
unsafe extern "C" fn fp_context_class_intern_init(mut klass: gpointer) {
    fp_context_parent_class = g_type_class_peek_parent(klass);
    if FpContext_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpContext_private_offset);
    }
    fp_context_class_init(klass as *mut FpContextClass);
}
#[no_mangle]
pub unsafe extern "C" fn fp_context_get_type() -> GType {
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
        let mut g_define_type_id: GType = fp_context_get_type_once();
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
unsafe extern "C" fn fp_context_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        g_intern_static_string(b"FpContext\0" as *const u8 as *const libc::c_char),
        ::core::mem::size_of::<FpContextClass>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(Some(fp_context_class_intern_init as unsafe extern "C" fn(gpointer) -> ())),
        ),
        ::core::mem::size_of::<FpContext>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpContext) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(Some(fp_context_init as unsafe extern "C" fn(*mut FpContext) -> ())),
        ),
        G_TYPE_FLAG_NONE,
    );
    FpContext_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<FpContextPrivate>() as libc::c_ulong,
    );
    return g_define_type_id;
}
static mut fp_context_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut FpContext_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn fp_context_get_instance_private(
    mut self_0: *mut FpContext,
) -> gpointer {
    return (self_0 as *mut guint8).offset(FpContext_private_offset as glong as isize)
        as gpointer;
}
static mut signals: [guint; 2] = [0 as libc::c_int as guint, 0];
unsafe extern "C" fn get_drivers_whitelist_env() -> *const libc::c_char {
    return g_getenv(b"FP_DRIVERS_WHITELIST\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn is_driver_allowed(mut driver: *const gchar) -> gboolean {
    let mut whitelisted_drivers: GStrv = 0 as GStrv;
    let mut fp_drivers_whitelist_env: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !driver.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-context\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"is_driver_allowed\0"))
                .as_ptr(),
            b"driver\0" as *const u8 as *const libc::c_char,
        );
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    fp_drivers_whitelist_env = get_drivers_whitelist_env();
    if fp_drivers_whitelist_env.is_null() {
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    whitelisted_drivers = g_strsplit(
        fp_drivers_whitelist_env,
        b":\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int),
    );
    i = 0 as libc::c_int;
    while !(*whitelisted_drivers.offset(i as isize)).is_null() {
        if g_strcmp0(driver, *whitelisted_drivers.offset(i as isize)) == 0 as libc::c_int
        {
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn remove_device_idle_cb(mut data: *mut RemoveDeviceData) -> gboolean {
    let mut priv_0: *mut FpContextPrivate = fp_context_get_instance_private(
        (*data).context,
    ) as *mut FpContextPrivate;
    let mut idx: guint = 0 as libc::c_int as guint;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if g_ptr_array_find((*priv_0).devices, (*data).device as gconstpointer, &mut idx)
            != 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-context\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"remove_device_idle_cb\0"))
                .as_ptr(),
            b"g_ptr_array_find (priv->devices, data->device, &idx)\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    g_signal_emit(
        (*data).context as gpointer,
        signals[DEVICE_REMOVED_SIGNAL as libc::c_int as usize],
        0 as libc::c_int as GQuark,
        (*data).device,
    );
    g_ptr_array_remove_index_fast((*priv_0).devices, idx);
    return 0 as libc::c_int;
}
unsafe extern "C" fn remove_device_data_free(mut data: *mut RemoveDeviceData) {
    let mut priv_0: *mut FpContextPrivate = fp_context_get_instance_private(
        (*data).context,
    ) as *mut FpContextPrivate;
    (*priv_0)
        .sources = g_slist_remove((*priv_0).sources, (*data).source as gconstpointer);
    g_free(data as gpointer);
}
unsafe extern "C" fn remove_device(
    mut context: *mut FpContext,
    mut device: *mut FpDevice,
) {
    let mut source: GSource_autoptr = 0 as GSource_autoptr;
    let mut priv_0: *mut FpContextPrivate = fp_context_get_instance_private(context)
        as *mut FpContextPrivate;
    let mut data: *mut RemoveDeviceData = 0 as *mut RemoveDeviceData;
    data = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<RemoveDeviceData>() as libc::c_ulong;
        let mut __p: gpointer = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = g_malloc(__n);
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut RemoveDeviceData;
    (*data).context = context;
    (*data).device = device;
    (*data).source = g_idle_source_new();
    source = (*data).source;
    g_source_set_callback(
        source,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GSourceFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut RemoveDeviceData) -> gboolean>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    remove_device_idle_cb
                        as unsafe extern "C" fn(*mut RemoveDeviceData) -> gboolean,
                ),
            ),
        ),
        data as gpointer,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RemoveDeviceData) -> ()>,
            GDestroyNotify,
        >(
            Some(
                remove_device_data_free
                    as unsafe extern "C" fn(*mut RemoveDeviceData) -> (),
            ),
        ),
    );
    g_source_attach(source, g_main_context_get_thread_default());
    (*priv_0).sources = g_slist_prepend((*priv_0).sources, source as gpointer);
}
unsafe extern "C" fn device_remove_on_notify_open_cb(
    mut context: *mut FpContext,
    mut pspec: *mut GParamSpec,
    mut device: *mut FpDevice,
) {
    remove_device(context, device);
}
unsafe extern "C" fn device_removed_cb(
    mut context: *mut FpContext,
    mut device: *mut FpDevice,
) {
    let mut open: gboolean = 0 as libc::c_int;
    g_object_get(
        device as gpointer,
        b"open\0" as *const u8 as *const libc::c_char,
        &mut open as *mut gboolean,
        0 as *mut libc::c_void,
    );
    if open != 0 {
        g_signal_connect_object(
            device as gpointer,
            b"notify::open\0" as *const u8 as *const libc::c_char,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut FpContext,
                        *mut GParamSpec,
                        *mut FpDevice,
                    ) -> (),
                >,
                GCallback,
            >(
                Some(
                    device_remove_on_notify_open_cb
                        as unsafe extern "C" fn(
                            *mut FpContext,
                            *mut GParamSpec,
                            *mut FpDevice,
                        ) -> (),
                ),
            ),
            context as gpointer,
            G_CONNECT_SWAPPED,
        );
    } else {
        remove_device(context, device);
    };
}
unsafe extern "C" fn async_device_init_done_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut error: GError_autoptr = 0 as GError_autoptr;
    let mut device: *mut FpDevice = 0 as *mut FpDevice;
    let mut context: *mut FpContext = 0 as *mut FpContext;
    let mut priv_0: *mut FpContextPrivate = 0 as *mut FpContextPrivate;
    device = FP_DEVICE(
        g_async_initable_new_finish(
            g_type_check_instance_cast(
                source_object as *mut GTypeInstance,
                g_async_initable_get_type(),
            ) as *mut libc::c_void as *mut GAsyncInitable,
            res,
            &mut error,
        ) as gpointer,
    );
    if g_error_matches(
        error as *const GError,
        g_io_error_quark(),
        G_IO_ERROR_CANCELLED as libc::c_int,
    ) != 0
    {
        return;
    }
    context = FP_CONTEXT(user_data);
    priv_0 = fp_context_get_instance_private(context) as *mut FpContextPrivate;
    (*priv_0).pending_devices -= 1;
    if !error.is_null() {
        g_log(
            b"libfprint-context\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_MESSAGE,
            b"Ignoring device due to initialization error: %s\0" as *const u8
                as *const libc::c_char,
            (*error).message,
        );
        return;
    }
    g_ptr_array_add((*priv_0).devices, device as gpointer);
    g_signal_connect_object(
        device as gpointer,
        b"removed\0" as *const u8 as *const libc::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FpContext, *mut FpDevice) -> ()>,
            GCallback,
        >(
            Some(
                device_removed_cb
                    as unsafe extern "C" fn(*mut FpContext, *mut FpDevice) -> (),
            ),
        ),
        context as gpointer,
        G_CONNECT_SWAPPED,
    );
    g_signal_emit(
        context as gpointer,
        signals[DEVICE_ADDED_SIGNAL as libc::c_int as usize],
        0 as libc::c_int as GQuark,
        device,
    );
}
unsafe extern "C" fn usb_device_added_cb(
    mut self_0: *mut FpContext,
    mut device: *mut GUsbDevice,
    mut usb_ctx: *mut GUsbContext,
) {
    let mut priv_0: *mut FpContextPrivate = fp_context_get_instance_private(self_0)
        as *mut FpContextPrivate;
    let mut found_driver: GType = ((1 as libc::c_int) << 2 as libc::c_int) as GType;
    let mut found_entry: *const FpIdEntry = 0 as *const FpIdEntry;
    let mut found_score: gint = 0 as libc::c_int;
    let mut i: gint = 0;
    let mut pid: guint16 = 0;
    let mut vid: guint16 = 0;
    pid = g_usb_device_get_pid(device);
    vid = g_usb_device_get_vid(device);
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*(*priv_0).drivers).len {
        let mut driver: GType = *((*(*priv_0).drivers).data as *mut libc::c_void
            as *mut GType)
            .offset(i as isize);
        let mut cls: FpDeviceClass_autoptr = g_type_class_ref(driver)
            as FpDeviceClass_autoptr;
        let mut entry: *const FpIdEntry = 0 as *const FpIdEntry;
        if !((*cls).type_0 as libc::c_uint
            != FP_DEVICE_TYPE_USB as libc::c_int as libc::c_uint)
        {
            entry = (*cls).id_table;
            while (*entry).c2rust_unnamed.c2rust_unnamed.pid != 0 {
                let mut driver_score: gint = 50 as libc::c_int;
                if !((*entry).c2rust_unnamed.c2rust_unnamed.pid != pid as libc::c_uint
                    || (*entry).c2rust_unnamed.c2rust_unnamed.vid != vid as libc::c_uint)
                {
                    if ((*cls).usb_discover).is_some() {
                        driver_score = ((*cls).usb_discover)
                            .expect("non-null function pointer")(device);
                    }
                    if !(driver_score <= found_score) {
                        found_score = driver_score;
                        found_driver = driver;
                        found_entry = entry;
                    }
                }
                entry = entry.offset(1);
            }
        }
        i += 1;
    }
    if found_driver == ((1 as libc::c_int) << 2 as libc::c_int) as GType {
        g_log(
            b"libfprint-context\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"No driver found for USB device %04X:%04X\0" as *const u8
                as *const libc::c_char,
            vid as libc::c_int,
            pid as libc::c_int,
        );
        return;
    }
    (*priv_0).pending_devices += 1;
    g_async_initable_new_async(
        found_driver,
        300 as libc::c_int,
        (*priv_0).cancellable,
        Some(
            async_device_init_done_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        self_0 as gpointer,
        b"fpi-usb-device\0" as *const u8 as *const libc::c_char,
        device,
        b"fpi-driver-data\0" as *const u8 as *const libc::c_char,
        (*found_entry).driver_data,
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn usb_device_removed_cb(
    mut self_0: *mut FpContext,
    mut device: *mut GUsbDevice,
    mut usb_ctx: *mut GUsbContext,
) {
    let mut priv_0: *mut FpContextPrivate = fp_context_get_instance_private(self_0)
        as *mut FpContextPrivate;
    let mut i: gint = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*(*priv_0).devices).len {
        let mut dev: *mut FpDevice = *((*(*priv_0).devices).pdata).offset(i as isize)
            as *mut FpDevice;
        let mut cls: *mut FpDeviceClass = FP_DEVICE_GET_CLASS(dev as gpointer);
        if !((*cls).type_0 as libc::c_uint
            != FP_DEVICE_TYPE_USB as libc::c_int as libc::c_uint)
        {
            if fpi_device_get_usb_device(dev) == device {
                fpi_device_remove(dev);
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn fp_context_finalize(mut object: *mut GObject) {
    let mut self_0: *mut FpContext = object as *mut FpContext;
    let mut priv_0: *mut FpContextPrivate = fp_context_get_instance_private(self_0)
        as *mut FpContextPrivate;
    g_cancellable_cancel((*priv_0).cancellable);
    let mut _pp: C2RustUnnamed_8 = C2RustUnnamed_8 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*priv_0).cancellable as *mut *mut GCancellable as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    let mut _pp_0: C2RustUnnamed_7 = C2RustUnnamed_7 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_0: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GArray) -> ()>,
        GDestroyNotify,
    >(Some(g_array_unref as unsafe extern "C" fn(*mut GArray) -> ()));
    _pp_0.in_0 = &mut (*priv_0).drivers as *mut *mut GArray as *mut libc::c_char;
    _p_0 = *_pp_0.out;
    if !_p_0.is_null() {
        *_pp_0.out = 0 as *mut libc::c_void;
        _destroy_0.expect("non-null function pointer")(_p_0);
    }
    let mut _pp_1: C2RustUnnamed_6 = C2RustUnnamed_6 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_1: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_1: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GPtrArray) -> ()>,
        GDestroyNotify,
    >(Some(g_ptr_array_unref as unsafe extern "C" fn(*mut GPtrArray) -> ()));
    _pp_1.in_0 = &mut (*priv_0).devices as *mut *mut GPtrArray as *mut libc::c_char;
    _p_1 = *_pp_1.out;
    if !_p_1.is_null() {
        *_pp_1.out = 0 as *mut libc::c_void;
        _destroy_1.expect("non-null function pointer")(_p_1);
    }
    g_slist_free_full(
        (if 0 as libc::c_int != 0 {
            (*priv_0).sources as *mut libc::c_void
        } else {
            g_steal_pointer(&mut (*priv_0).sources as *mut *mut GSList as gpointer)
        }) as *mut GSList,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GSource) -> ()>,
            GDestroyNotify,
        >(Some(g_source_destroy as unsafe extern "C" fn(*mut GSource) -> ())),
    );
    if !((*priv_0).usb_ctx).is_null() {
        g_object_run_dispose(
            g_type_check_instance_cast(
                (*priv_0).usb_ctx as *mut GTypeInstance,
                ((20 as libc::c_int) << 2 as libc::c_int) as GType,
            ) as *mut libc::c_void as *mut GObject,
        );
    }
    let mut _pp_2: C2RustUnnamed_5 = C2RustUnnamed_5 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_2: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_2: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp_2.in_0 = &mut (*priv_0).usb_ctx as *mut *mut GUsbContext as *mut libc::c_char;
    _p_2 = *_pp_2.out;
    if !_p_2.is_null() {
        *_pp_2.out = 0 as *mut libc::c_void;
        _destroy_2.expect("non-null function pointer")(_p_2);
    }
    ((*(g_type_check_class_cast(
        fp_context_parent_class as *mut GTypeClass,
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
    ) as *mut libc::c_void as *mut GObjectClass))
        .finalize)
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn fp_context_class_init(mut klass: *mut FpContextClass) {
    let mut object_class: *mut GObjectClass = g_type_check_class_cast(
        klass as *mut GTypeClass,
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
    ) as *mut libc::c_void as *mut GObjectClass;
    (*object_class)
        .finalize = Some(
        fp_context_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    );
    signals[DEVICE_ADDED_SIGNAL as libc::c_int
        as usize] = g_signal_new(
        b"device-added\0" as *const u8 as *const libc::c_char,
        (*(klass as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        136 as libc::c_ulong as glong as guint,
        None,
        0 as *mut libc::c_void,
        Some(
            g_cclosure_marshal_VOID__OBJECT
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        ((1 as libc::c_int) << 2 as libc::c_int) as GType,
        1 as libc::c_int as guint,
        fp_device_get_type(),
    );
    signals[DEVICE_REMOVED_SIGNAL as libc::c_int
        as usize] = g_signal_new(
        b"device-removed\0" as *const u8 as *const libc::c_char,
        (*(klass as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        144 as libc::c_ulong as glong as guint,
        None,
        0 as *mut libc::c_void,
        Some(
            g_cclosure_marshal_VOID__OBJECT
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        ((1 as libc::c_int) << 2 as libc::c_int) as GType,
        1 as libc::c_int as guint,
        fp_device_get_type(),
    );
}
unsafe extern "C" fn fp_context_init(mut self_0: *mut FpContext) {
    let mut error: GError_autoptr = 0 as GError_autoptr;
    let mut priv_0: *mut FpContextPrivate = fp_context_get_instance_private(self_0)
        as *mut FpContextPrivate;
    let mut i: guint = 0;
    g_log(
        b"libfprint-context\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Initializing FpContext (libfprint version 1.94.5)\0" as *const u8
            as *const libc::c_char,
    );
    (*priv_0).drivers = fpi_get_driver_types();
    if !(get_drivers_whitelist_env()).is_null() {
        i = 0 as libc::c_int as guint;
        while i < (*(*priv_0).drivers).len {
            let mut driver: GType = *((*(*priv_0).drivers).data as *mut libc::c_void
                as *mut GType)
                .offset(i as isize);
            let mut cls: FpDeviceClass_autoptr = g_type_class_ref(driver)
                as FpDeviceClass_autoptr;
            if is_driver_allowed((*cls).id) == 0 {
                g_array_remove_index((*priv_0).drivers, i);
            } else {
                i = i.wrapping_add(1);
            }
        }
    }
    (*priv_0)
        .devices = g_ptr_array_new_with_free_func(
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*priv_0).cancellable = g_cancellable_new();
    (*priv_0).usb_ctx = g_usb_context_new(&mut error);
    if ((*priv_0).usb_ctx).is_null() {
        g_log(
            b"libfprint-context\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_MESSAGE,
            b"Could not initialise USB Subsystem: %s\0" as *const u8
                as *const libc::c_char,
            (*error).message,
        );
    } else {
        g_usb_context_set_debug((*priv_0).usb_ctx, G_LOG_LEVEL_INFO);
        g_signal_connect_object(
            (*priv_0).usb_ctx as gpointer,
            b"device-added\0" as *const u8 as *const libc::c_char,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut FpContext,
                        *mut GUsbDevice,
                        *mut GUsbContext,
                    ) -> (),
                >,
                GCallback,
            >(
                Some(
                    usb_device_added_cb
                        as unsafe extern "C" fn(
                            *mut FpContext,
                            *mut GUsbDevice,
                            *mut GUsbContext,
                        ) -> (),
                ),
            ),
            self_0 as gpointer,
            G_CONNECT_SWAPPED,
        );
        g_signal_connect_object(
            (*priv_0).usb_ctx as gpointer,
            b"device-removed\0" as *const u8 as *const libc::c_char,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut FpContext,
                        *mut GUsbDevice,
                        *mut GUsbContext,
                    ) -> (),
                >,
                GCallback,
            >(
                Some(
                    usb_device_removed_cb
                        as unsafe extern "C" fn(
                            *mut FpContext,
                            *mut GUsbDevice,
                            *mut GUsbContext,
                        ) -> (),
                ),
            ),
            self_0 as gpointer,
            G_CONNECT_SWAPPED,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn fp_context_new() -> *mut FpContext {
    return g_object_new(fp_context_get_type(), 0 as *const gchar) as *mut FpContext;
}
#[no_mangle]
pub unsafe extern "C" fn fp_context_enumerate(mut context: *mut FpContext) {
    let mut priv_0: *mut FpContextPrivate = fp_context_get_instance_private(context)
        as *mut FpContextPrivate;
    let mut dispatched: gboolean = 0;
    let mut i: gint = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_CONTEXT(context as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-context\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fp_context_enumerate\0"))
                .as_ptr(),
            b"FP_IS_CONTEXT (context)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*priv_0).enumerated != 0 {
        return;
    }
    (*priv_0).enumerated = (0 as libc::c_int == 0) as libc::c_int;
    if !((*priv_0).usb_ctx).is_null() {
        g_usb_context_enumerate((*priv_0).usb_ctx);
    }
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*(*priv_0).drivers).len {
        let mut driver: GType = *((*(*priv_0).drivers).data as *mut libc::c_void
            as *mut GType)
            .offset(i as isize);
        let mut cls: FpDeviceClass_autoptr = g_type_class_ref(driver)
            as FpDeviceClass_autoptr;
        let mut entry: *const FpIdEntry = 0 as *const FpIdEntry;
        if !((*cls).type_0 as libc::c_uint
            != FP_DEVICE_TYPE_VIRTUAL as libc::c_int as libc::c_uint)
        {
            entry = (*cls).id_table;
            while (*entry).c2rust_unnamed.c2rust_unnamed.pid != 0 {
                let mut val: *const gchar = 0 as *const gchar;
                val = g_getenv((*entry).c2rust_unnamed.virtual_envvar);
                if !(val.is_null()
                    || *val.offset(0 as libc::c_int as isize) as libc::c_int
                        == '\0' as i32)
                {
                    g_log(
                        b"libfprint-context\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"Found virtual environment device: %s, %s\0" as *const u8
                            as *const libc::c_char,
                        (*entry).c2rust_unnamed.virtual_envvar,
                        val,
                    );
                    (*priv_0).pending_devices += 1;
                    g_async_initable_new_async(
                        driver,
                        300 as libc::c_int,
                        (*priv_0).cancellable,
                        Some(
                            async_device_init_done_cb
                                as unsafe extern "C" fn(
                                    *mut GObject,
                                    *mut GAsyncResult,
                                    gpointer,
                                ) -> (),
                        ),
                        context as gpointer,
                        b"fpi-environ\0" as *const u8 as *const libc::c_char,
                        val,
                        b"fpi-driver-data\0" as *const u8 as *const libc::c_char,
                        (*entry).driver_data,
                        0 as *mut libc::c_void,
                    );
                    g_log(
                        b"libfprint-context\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"created\0" as *const u8 as *const libc::c_char,
                    );
                }
                entry = entry.offset(1);
            }
        }
        i += 1;
    }
    let mut udev_client: GUdevClient_autoptr = g_udev_client_new(
        0 as *const *const gchar,
    );
    let mut spidev_devices: GList_autoptr = g_udev_client_query_by_subsystem(
        udev_client,
        b"spidev\0" as *const u8 as *const libc::c_char,
    );
    let mut hidraw_devices: GList_autoptr = g_udev_client_query_by_subsystem(
        udev_client,
        b"hidraw\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*(*priv_0).drivers).len {
        let mut driver_0: GType = *((*(*priv_0).drivers).data as *mut libc::c_void
            as *mut GType)
            .offset(i as isize);
        let mut cls_0: FpDeviceClass_autoptr = g_type_class_ref(driver_0)
            as FpDeviceClass_autoptr;
        let mut entry_0: *const FpIdEntry = 0 as *const FpIdEntry;
        if !((*cls_0).type_0 as libc::c_uint
            != FP_DEVICE_TYPE_UDEV as libc::c_int as libc::c_uint)
        {
            let mut current_block_36: u64;
            entry_0 = (*cls_0).id_table;
            while (*entry_0).c2rust_unnamed.c2rust_unnamed_0.udev_types as u64 != 0 {
                let mut matched_spidev: *mut GList = 0 as *mut GList;
                let mut matched_hidraw: *mut GList = 0 as *mut GList;
                if (*entry_0).c2rust_unnamed.c2rust_unnamed_0.udev_types as libc::c_uint
                    & FPI_DEVICE_UDEV_SUBTYPE_SPIDEV as libc::c_int as libc::c_uint != 0
                {
                    matched_spidev = spidev_devices;
                    while !matched_spidev.is_null() {
                        let mut sysfs: *const gchar = g_udev_device_get_sysfs_path(
                            (*matched_spidev).data as *mut GUdevDevice,
                        );
                        if !sysfs.is_null() {
                            if !(strstr(
                                sysfs,
                                (*entry_0).c2rust_unnamed.c2rust_unnamed_0.spi_acpi_id,
                            ))
                                .is_null()
                            {
                                break;
                            }
                        }
                        matched_spidev = (*matched_spidev).next;
                    }
                    if matched_spidev.is_null() {
                        current_block_36 = 1608152415753874203;
                    } else {
                        current_block_36 = 15597372965620363352;
                    }
                } else {
                    current_block_36 = 15597372965620363352;
                }
                match current_block_36 {
                    15597372965620363352 => {
                        if (*entry_0).c2rust_unnamed.c2rust_unnamed_0.udev_types
                            as libc::c_uint
                            & FPI_DEVICE_UDEV_SUBTYPE_HIDRAW as libc::c_int
                                as libc::c_uint != 0
                        {
                            matched_hidraw = hidraw_devices;
                            while !matched_hidraw.is_null() {
                                let mut parent: GUdevDevice_autoptr = g_udev_device_get_parent_with_subsystem(
                                    (*matched_hidraw).data as *mut GUdevDevice,
                                    b"hid\0" as *const u8 as *const libc::c_char,
                                    0 as *const gchar,
                                );
                                let mut hid_id: *const gchar = g_udev_device_get_property(
                                    parent,
                                    b"HID_ID\0" as *const u8 as *const libc::c_char,
                                );
                                let mut vendor: guint32 = 0;
                                let mut product: guint32 = 0;
                                if !(parent.is_null() || hid_id.is_null()) {
                                    if !(sscanf(
                                        hid_id,
                                        b"%*X:%X:%X\0" as *const u8 as *const libc::c_char,
                                        &mut vendor as *mut guint32,
                                        &mut product as *mut guint32,
                                    ) != 2 as libc::c_int)
                                    {
                                        if vendor
                                            == (*entry_0).c2rust_unnamed.c2rust_unnamed_0.hid_id.vid
                                            && product
                                                == (*entry_0).c2rust_unnamed.c2rust_unnamed_0.hid_id.pid
                                        {
                                            break;
                                        }
                                    }
                                }
                                matched_hidraw = (*matched_hidraw).next;
                            }
                            if matched_hidraw.is_null() {
                                current_block_36 = 1608152415753874203;
                            } else {
                                current_block_36 = 6174974146017752131;
                            }
                        } else {
                            current_block_36 = 6174974146017752131;
                        }
                        match current_block_36 {
                            1608152415753874203 => {}
                            _ => {
                                (*priv_0).pending_devices += 1;
                                g_async_initable_new_async(
                                    driver_0,
                                    300 as libc::c_int,
                                    (*priv_0).cancellable,
                                    Some(
                                        async_device_init_done_cb
                                            as unsafe extern "C" fn(
                                                *mut GObject,
                                                *mut GAsyncResult,
                                                gpointer,
                                            ) -> (),
                                    ),
                                    context as gpointer,
                                    b"fpi-driver-data\0" as *const u8 as *const libc::c_char,
                                    (*entry_0).driver_data,
                                    b"fpi-udev-data-spidev\0" as *const u8
                                        as *const libc::c_char,
                                    if !matched_spidev.is_null() {
                                        g_udev_device_get_device_file(
                                            (*matched_spidev).data as *mut GUdevDevice,
                                        )
                                    } else {
                                        0 as *const gchar
                                    },
                                    b"fpi-udev-data-hidraw\0" as *const u8
                                        as *const libc::c_char,
                                    if !matched_hidraw.is_null() {
                                        g_udev_device_get_device_file(
                                            (*matched_hidraw).data as *mut GUdevDevice,
                                        )
                                    } else {
                                        0 as *const gchar
                                    },
                                    0 as *mut libc::c_void,
                                );
                                if !matched_spidev.is_null() {
                                    g_object_unref((*matched_spidev).data);
                                    spidev_devices = g_list_delete_link(
                                        spidev_devices,
                                        matched_spidev,
                                    );
                                }
                                if !matched_hidraw.is_null() {
                                    g_object_unref((*matched_hidraw).data);
                                    hidraw_devices = g_list_delete_link(
                                        hidraw_devices,
                                        matched_hidraw,
                                    );
                                }
                            }
                        }
                    }
                    _ => {}
                }
                entry_0 = entry_0.offset(1);
            }
        }
        i += 1;
    }
    g_list_foreach(
        spidev_devices,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(gpointer) -> ()>,
            GFunc,
        >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ())),
        0 as *mut libc::c_void,
    );
    g_list_foreach(
        hidraw_devices,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(gpointer) -> ()>,
            GFunc,
        >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ())),
        0 as *mut libc::c_void,
    );
    dispatched = (0 as libc::c_int == 0) as libc::c_int;
    while (*priv_0).pending_devices != 0 || dispatched != 0 {
        dispatched = g_main_context_iteration(
            0 as *mut GMainContext,
            ((*priv_0).pending_devices != 0) as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn fp_context_get_devices(
    mut context: *mut FpContext,
) -> *mut GPtrArray {
    let mut priv_0: *mut FpContextPrivate = fp_context_get_instance_private(context)
        as *mut FpContextPrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_CONTEXT(context as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-context\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"fp_context_get_devices\0"))
                .as_ptr(),
            b"FP_IS_CONTEXT (context)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut GPtrArray;
    }
    fp_context_enumerate(context);
    return (*priv_0).devices;
}
