use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GAsyncInitable;
    pub type _GCancellablePrivate;
    pub type _GTask;
    pub type _FpImage;
    pub type _FpPrint;
    fn g_ptr_array_new_full(
        reserved_size: guint,
        element_free_func: GDestroyNotify,
    ) -> *mut GPtrArray;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_clear_error(err: *mut *mut GError);
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_main_context_iteration(
        context: *mut GMainContext,
        may_block: gboolean,
    ) -> gboolean;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_destroy(source: *mut GSource);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_source_set_name(source: *mut GSource, name: *const libc::c_char);
    fn g_idle_source_new() -> *mut GSource;
    fn g_get_monotonic_time() -> gint64;
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
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
    fn g_cclosure_marshal_VOID__VOID(
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
    fn g_object_class_install_properties(
        oclass: *mut GObjectClass,
        n_pspecs: guint,
        pspecs: *mut *mut GParamSpec,
    );
    fn g_object_get(object: gpointer, first_property_name: *const gchar, _: ...);
    fn g_object_ref_sink(object: gpointer) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_get_object(value: *const GValue) -> gpointer;
    fn g_value_dup_object(value: *const GValue) -> gpointer;
    fn g_value_set_enum(value: *mut GValue, v_enum: gint);
    fn g_value_set_flags(value: *mut GValue, v_flags: guint);
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_uint(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: guint,
        maximum: guint,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_uint64(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: guint64,
        maximum: guint64,
        default_value: guint64,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_enum(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        enum_type: GType,
        default_value: gint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_flags(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags_type: GType,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    fn g_value_get_uint64(value: *const GValue) -> guint64;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_set_static_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_get_string(value: *const GValue) -> *const gchar;
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn g_type_check_class_cast(
        g_class: *mut GTypeClass,
        is_a_type: GType,
    ) -> *mut GTypeClass;
    fn g_type_check_instance_is_a(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> gboolean;
    fn g_type_check_instance_cast(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> *mut GTypeInstance;
    fn g_type_class_adjust_private_offset(
        g_class: gpointer,
        private_size_or_offset: *mut gint,
    );
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_add_interface_static(
        instance_type: GType,
        interface_type: GType,
        info: *const GInterfaceInfo,
    );
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const gchar,
        class_size: guint,
        class_init: GClassInitFunc,
        instance_size: guint,
        instance_init: GInstanceInitFunc,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_async_initable_get_type() -> GType;
    fn g_cancellable_new() -> *mut GCancellable;
    fn g_cancellable_connect(
        cancellable: *mut GCancellable,
        callback: GCallback,
        data: gpointer,
        data_destroy_func: GDestroyNotify,
    ) -> gulong;
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
    fn g_task_get_type() -> GType;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_set_task_data(
        task: *mut GTask,
        task_data: gpointer,
        task_data_destroy: GDestroyNotify,
    );
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_context(task: *mut GTask) -> *mut GMainContext;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_return_error_if_cancelled(task: *mut GTask) -> gboolean;
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn g_task_propagate_int(task: *mut GTask, error: *mut *mut GError) -> gssize;
    fn g_usb_device_get_type() -> GType;
    fn g_usb_device_open(self_0: *mut GUsbDevice, error: *mut *mut GError) -> gboolean;
    fn fp_temperature_get_type() -> GType;
    fn fp_finger_status_flags_get_type() -> GType;
    fn fp_scan_type_get_type() -> GType;
    fn fpi_device_probe_complete(
        device: *mut FpDevice,
        device_id: *const gchar,
        device_name: *const gchar,
        error: *mut GError,
    );
    fn fpi_device_configure_wakeup(device: *mut FpDevice, enabled: gboolean);
    fn fpi_device_add_timeout(
        device: *mut FpDevice,
        interval: gint,
        func: FpTimeoutFunc,
        user_data: gpointer,
        destroy_notify: GDestroyNotify,
    ) -> *mut GSource;
    fn fpi_device_report_finger_status(
        device: *mut FpDevice,
        finger_status: FpFingerStatusFlags,
    ) -> gboolean;
    fn fp_print_get_type() -> GType;
    fn fp_print_compatible(self_0: *mut FpPrint, device: *mut FpDevice) -> gboolean;
    fn fpi_device_error_new(error: FpDeviceError) -> *mut GError;
    fn fpi_device_suspend(device: *mut FpDevice);
    fn fpi_device_resume(device: *mut FpDevice);
    fn fpi_device_update_temp(device: *mut FpDevice, is_active: gboolean);
    fn fpi_device_error_new_msg(
        error: FpDeviceError,
        msg: *const gchar,
        _: ...
    ) -> *mut GError;
}
pub type guint8 = libc::c_uchar;
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
pub struct _GTypeInterface {
    pub g_type: GType,
    pub g_instance_type: GType,
}
pub type GTypeInterface = _GTypeInterface;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInterfaceInfo {
    pub interface_init: GInterfaceInitFunc,
    pub interface_finalize: GInterfaceFinalizeFunc,
    pub interface_data: gpointer,
}
pub type GInterfaceFinalizeFunc = Option::<
    unsafe extern "C" fn(gpointer, gpointer) -> (),
>;
pub type GInterfaceInitFunc = Option::<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GInterfaceInfo = _GInterfaceInfo;
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
pub type GTask = _GTask;
pub type GAsyncReadyCallback = Option::<
    unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GAsyncInitableIface {
    pub g_iface: GTypeInterface,
    pub init_async: Option::<
        unsafe extern "C" fn(
            *mut GAsyncInitable,
            libc::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub init_finish: Option::<
        unsafe extern "C" fn(
            *mut GAsyncInitable,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> gboolean,
    >,
}
pub type GAsyncInitableIface = _GAsyncInitableIface;
pub type GAsyncResult_autoptr = *mut GAsyncResult;
pub type GTask_autoptr = *mut GTask;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUsbDevice {
    pub parent_instance: GObject,
}
pub type GUsbDevice = _GUsbDevice;
pub type FpImage = _FpImage;
pub type FpDevice = _FpDevice;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpDevice {
    pub parent_instance: GObject,
}
pub type FpDeviceClass = _FpDeviceClass;
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
pub const N_PROPS: C2RustUnnamed_21 = 15;
pub const PROP_FPI_DRIVER_DATA: C2RustUnnamed_21 = 14;
pub const PROP_FPI_UDEV_DATA_HIDRAW: C2RustUnnamed_21 = 13;
pub const PROP_FPI_UDEV_DATA_SPIDEV: C2RustUnnamed_21 = 12;
pub const PROP_FPI_USB_DEVICE: C2RustUnnamed_21 = 11;
pub const PROP_FPI_ENVIRON: C2RustUnnamed_21 = 10;
pub const REMOVED_SIGNAL: C2RustUnnamed_22 = 0;
pub const PROP_REMOVED: C2RustUnnamed_21 = 5;
pub const PROP_OPEN: C2RustUnnamed_21 = 4;
pub const PROP_NAME: C2RustUnnamed_21 = 3;
pub const PROP_DEVICE_ID: C2RustUnnamed_21 = 2;
pub const PROP_DRIVER: C2RustUnnamed_21 = 1;
pub const FP_TEMPERATURE_COLD: FpTemperature = 0;
pub const PROP_TEMPERATURE: C2RustUnnamed_21 = 9;
pub const FP_FINGER_STATUS_NONE: FpFingerStatusFlags = 0;
pub const PROP_FINGER_STATUS: C2RustUnnamed_21 = 8;
pub const PROP_SCAN_TYPE: C2RustUnnamed_21 = 7;
pub const PROP_NR_ENROLL_STAGES: C2RustUnnamed_21 = 6;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpDevicePrivate {
    pub type_0: FpDeviceType,
    pub usb_device: *mut GUsbDevice,
    pub virtual_env: *const gchar,
    pub udev_data: C2RustUnnamed_4,
    pub is_removed: gboolean,
    pub is_open: gboolean,
    pub is_suspended: gboolean,
    pub device_id: *mut gchar,
    pub device_name: *mut gchar,
    pub scan_type: FpScanType,
    pub features: FpDeviceFeature,
    pub driver_data: guint64,
    pub nr_enroll_stages: gint,
    pub sources: *mut GSList,
    pub current_action: FpiDeviceAction,
    pub current_task: *mut GTask,
    pub current_cancellation_reason: *mut GError,
    pub current_user_cb: GAsyncReadyCallback,
    pub current_cancellable: *mut GCancellable,
    pub current_cancellable_id: gulong,
    pub current_task_cancellable_id: gulong,
    pub current_idle_cancel_source: *mut GSource,
    pub current_task_idle_return_source: *mut GSource,
    pub wait_for_finger: gboolean,
    pub finger_status: FpFingerStatusFlags,
    pub critical_section: guint,
    pub critical_section_flush_source: *mut GSource,
    pub cancel_queued: gboolean,
    pub suspend_queued: gboolean,
    pub resume_queued: gboolean,
    pub suspend_resume_task: *mut GTask,
    pub suspend_error: *mut GError,
    pub temp_timeout: *mut GSource,
    pub temp_current: FpTemperature,
    pub temp_hot_seconds: gint32,
    pub temp_cold_seconds: gint32,
    pub temp_last_update: gint64,
    pub temp_last_active: gboolean,
    pub temp_current_ratio: gdouble,
}
pub type FpTemperature = libc::c_uint;
pub const FP_TEMPERATURE_HOT: FpTemperature = 2;
pub const FP_TEMPERATURE_WARM: FpTemperature = 1;
pub type FpFingerStatusFlags = libc::c_uint;
pub const FP_FINGER_STATUS_PRESENT: FpFingerStatusFlags = 2;
pub const FP_FINGER_STATUS_NEEDED: FpFingerStatusFlags = 1;
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
pub struct C2RustUnnamed_4 {
    pub spidev_path: *mut gchar,
    pub hidraw_path: *mut gchar,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
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
pub type FpTimeoutFunc = Option::<unsafe extern "C" fn(*mut FpDevice, gpointer) -> ()>;
pub type FpPrint = _FpPrint;
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
pub type FpEnrollProgress = Option::<
    unsafe extern "C" fn(*mut FpDevice, gint, *mut FpPrint, gpointer, *mut GError) -> (),
>;
pub type FpMatchCb = Option::<
    unsafe extern "C" fn(
        *mut FpDevice,
        *mut FpPrint,
        *mut FpPrint,
        gpointer,
        *mut GError,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpEnrollData {
    pub print: *mut FpPrint,
    pub enroll_progress_cb: FpEnrollProgress,
    pub enroll_progress_data: gpointer,
    pub enroll_progress_destroy: GDestroyNotify,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub const FPI_PRINT_UNDEFINED: FpiPrintType = 0;
pub type FpiPrintType = libc::c_uint;
pub const FPI_PRINT_NBIS: FpiPrintType = 2;
pub const FPI_PRINT_RAW: FpiPrintType = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpMatchData {
    pub enrolled_print: *mut FpPrint,
    pub gallery: *mut GPtrArray,
    pub result_reported: gboolean,
    pub match_0: *mut FpPrint,
    pub print: *mut FpPrint,
    pub error: *mut GError,
    pub match_cb: FpMatchCb,
    pub match_data: gpointer,
    pub match_destroy: GDestroyNotify,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_19 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub const FPI_MATCH_ERROR: C2RustUnnamed_20 = -1;
pub const FPI_MATCH_SUCCESS: C2RustUnnamed_20 = 1;
pub type C2RustUnnamed_20 = libc::c_int;
pub const FPI_MATCH_FAIL: C2RustUnnamed_20 = 0;
pub type C2RustUnnamed_21 = libc::c_uint;
pub const PROP_0: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = libc::c_uint;
pub const N_SIGNALS: C2RustUnnamed_22 = 1;
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
unsafe extern "C" fn FP_DEVICE_GET_CLASS(mut ptr: gpointer) -> *mut FpDeviceClass {
    return (*(ptr as *mut GTypeInstance)).g_class as *mut FpDeviceClass;
}
#[inline]
unsafe extern "C" fn FP_IS_DEVICE(mut ptr: gpointer) -> gboolean {
    return ({
        let mut __inst: *mut GTypeInstance = ptr as *mut GTypeInstance;
        let mut __t: GType = fp_device_get_type();
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
unsafe extern "C" fn FP_IS_PRINT(mut ptr: gpointer) -> gboolean {
    return ({
        let mut __inst: *mut GTypeInstance = ptr as *mut GTypeInstance;
        let mut __t: GType = fp_print_get_type();
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
#[no_mangle]
pub unsafe extern "C" fn fp_device_get_type() -> GType {
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
        let mut g_define_type_id: GType = fp_device_get_type_once();
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
unsafe extern "C" fn fp_device_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        g_intern_static_string(b"FpDevice\0" as *const u8 as *const libc::c_char),
        ::core::mem::size_of::<FpDeviceClass>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(Some(fp_device_class_intern_init as unsafe extern "C" fn(gpointer) -> ())),
        ),
        ::core::mem::size_of::<FpDevice>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpDevice) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(Some(fp_device_init as unsafe extern "C" fn(*mut FpDevice) -> ())),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    let g_implement_interface_info: GInterfaceInfo = {
        let mut init = _GInterfaceInfo {
            interface_init: ::core::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                GInterfaceInitFunc,
            >(
                ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut GAsyncInitableIface) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(
                    Some(
                        fp_device_async_initable_iface_init
                            as unsafe extern "C" fn(*mut GAsyncInitableIface) -> (),
                    ),
                ),
            ),
            interface_finalize: None,
            interface_data: 0 as *mut libc::c_void,
        };
        init
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_async_initable_get_type(),
        &g_implement_interface_info,
    );
    FpDevice_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<FpDevicePrivate>() as libc::c_ulong,
    );
    return g_define_type_id;
}
unsafe extern "C" fn fp_device_class_intern_init(mut klass: gpointer) {
    fp_device_parent_class = g_type_class_peek_parent(klass);
    if FpDevice_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpDevice_private_offset);
    }
    fp_device_class_init(klass as *mut FpDeviceClass);
}
#[inline]
unsafe extern "C" fn fp_device_get_instance_private(
    mut self_0: *mut FpDevice,
) -> gpointer {
    return (self_0 as *mut guint8).offset(FpDevice_private_offset as glong as isize)
        as gpointer;
}
static mut FpDevice_private_offset: gint = 0;
static mut fp_device_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut properties: [*mut GParamSpec; 15] = [0 as *const GParamSpec
    as *mut GParamSpec; 15];
static mut signals: [guint; 1] = [0 as libc::c_int as guint];
#[no_mangle]
pub unsafe extern "C" fn fp_device_retry_quark() -> GQuark {
    static mut q: GQuark = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if q == 0 as libc::c_int as libc::c_uint {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        q = g_quark_from_static_string(
            b"fp - device - retry - quark\0" as *const u8 as *const libc::c_char,
        );
    }
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_error_quark() -> GQuark {
    static mut q: GQuark = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if q == 0 as libc::c_int as libc::c_uint {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        q = g_quark_from_static_string(
            b"fp - device - error - quark\0" as *const u8 as *const libc::c_char,
        );
    }
    return q;
}
unsafe extern "C" fn fp_device_cancel_in_idle_cb(mut user_data: gpointer) -> gboolean {
    let mut self_0: *mut FpDevice = user_data as *mut FpDevice;
    let mut cls: *mut FpDeviceClass = FP_DEVICE_GET_CLASS(self_0 as gpointer);
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(self_0)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if ((*cls).cancel).is_some() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-device.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fp_device_cancel_in_idle_cb\0"))
                .as_ptr(),
            b"cls->cancel\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            != FPI_DEVICE_ACTION_NONE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-device.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fp_device_cancel_in_idle_cb\0"))
                .as_ptr(),
            b"priv->current_action != FPI_DEVICE_ACTION_NONE\0" as *const u8
                as *const libc::c_char,
        );
    }
    g_log(
        b"libfprint-device\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Idle cancelling on ongoing operation!\0" as *const u8 as *const libc::c_char,
    );
    (*priv_0).current_idle_cancel_source = 0 as *mut GSource;
    if (*priv_0).critical_section != 0 {
        (*priv_0).cancel_queued = (0 as libc::c_int == 0) as libc::c_int;
    } else {
        ((*cls).cancel).expect("non-null function pointer")(self_0);
    }
    fpi_device_report_finger_status(self_0, FP_FINGER_STATUS_NONE);
    return 0 as libc::c_int;
}
unsafe extern "C" fn fp_device_cancelled_cb(
    mut cancellable: *mut GCancellable,
    mut self_0: *mut FpDevice,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(self_0)
        as *mut FpDevicePrivate;
    (*priv_0).current_idle_cancel_source = g_idle_source_new();
    g_source_set_callback(
        (*priv_0).current_idle_cancel_source,
        Some(fp_device_cancel_in_idle_cb as unsafe extern "C" fn(gpointer) -> gboolean),
        self_0 as gpointer,
        None,
    );
    g_source_attach(
        (*priv_0).current_idle_cancel_source,
        g_task_get_context((*priv_0).current_task),
    );
    g_source_unref((*priv_0).current_idle_cancel_source);
}
unsafe extern "C" fn fp_device_task_cancelled_cb(
    mut cancellable: *mut GCancellable,
    mut self_0: *mut FpDevice,
) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(self_0)
        as *mut FpDevicePrivate;
    g_cancellable_cancel((*priv_0).current_cancellable);
}
unsafe extern "C" fn setup_task_cancellable(mut device: *mut FpDevice) {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut cls: *mut FpDeviceClass = FP_DEVICE_GET_CLASS(device as gpointer);
    (*priv_0).current_cancellable = g_cancellable_new();
    if ((*cls).cancel).is_some() {
        (*priv_0)
            .current_cancellable_id = g_cancellable_connect(
            (*priv_0).current_cancellable,
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut GCancellable, *mut FpDevice) -> ()>,
                GCallback,
            >(
                Some(
                    fp_device_cancelled_cb
                        as unsafe extern "C" fn(*mut GCancellable, *mut FpDevice) -> (),
                ),
            ),
            device as gpointer,
            None,
        );
    }
    if !(g_task_get_cancellable((*priv_0).current_task)).is_null() {
        (*priv_0)
            .current_task_cancellable_id = g_cancellable_connect(
            g_task_get_cancellable((*priv_0).current_task),
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut GCancellable, *mut FpDevice) -> ()>,
                GCallback,
            >(
                Some(
                    fp_device_task_cancelled_cb
                        as unsafe extern "C" fn(*mut GCancellable, *mut FpDevice) -> (),
                ),
            ),
            device as gpointer,
            None,
        );
    }
}
unsafe extern "C" fn fp_device_constructed(mut object: *mut GObject) {
    let mut self_0: *mut FpDevice = object as *mut FpDevice;
    let mut cls: *mut FpDeviceClass = FP_DEVICE_GET_CLASS(self_0 as gpointer);
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(self_0)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*cls).features as libc::c_uint
            != FP_DEVICE_FEATURE_NONE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-device.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fp_device_constructed\0"))
                .as_ptr(),
            b"cls->features != FP_DEVICE_FEATURE_NONE\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*priv_0).type_0 = (*cls).type_0;
    if (*cls).nr_enroll_stages != 0 {
        (*priv_0).nr_enroll_stages = (*cls).nr_enroll_stages;
    }
    (*priv_0).scan_type = (*cls).scan_type;
    (*priv_0).features = (*cls).features;
    (*priv_0).device_name = g_strdup((*cls).full_name);
    (*priv_0).device_id = g_strdup(b"0\0" as *const u8 as *const libc::c_char);
    if (*cls).temp_hot_seconds > 0 as libc::c_int {
        (*priv_0).temp_hot_seconds = (*cls).temp_hot_seconds;
        (*priv_0).temp_cold_seconds = (*cls).temp_cold_seconds;
        if ({
            let mut _g_boolean_var_: libc::c_int = 0;
            if (*priv_0).temp_cold_seconds > 0 as libc::c_int {
                _g_boolean_var_ = 1 as libc::c_int;
            } else {
                _g_boolean_var_ = 0 as libc::c_int;
            }
            _g_boolean_var_
        }) as libc::c_long != 0
        {} else {
            g_assertion_message_expr(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                b"../libfprint/fp-device.c\0" as *const u8 as *const libc::c_char,
                182 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"fp_device_constructed\0"))
                    .as_ptr(),
                b"priv->temp_cold_seconds > 0\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if (*cls).temp_hot_seconds == 0 as libc::c_int {
        (*priv_0).temp_hot_seconds = 3 as libc::c_int * 60 as libc::c_int;
        (*priv_0).temp_cold_seconds = 9 as libc::c_int * 60 as libc::c_int;
    } else {
        (*priv_0).temp_hot_seconds = -(1 as libc::c_int);
        (*priv_0).temp_cold_seconds = -(1 as libc::c_int);
    }
    (*priv_0).temp_current = FP_TEMPERATURE_COLD;
    (*priv_0).temp_current_ratio = 0.26894142136999512075f64;
    (*priv_0).temp_last_update = g_get_monotonic_time();
    (*priv_0).temp_last_active = 0 as libc::c_int;
    ((*(g_type_check_class_cast(
        fp_device_parent_class as *mut GTypeClass,
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
    ) as *mut libc::c_void as *mut GObjectClass))
        .constructed)
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn fp_device_finalize(mut object: *mut GObject) {
    let mut self_0: *mut FpDevice = object as *mut FpDevice;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(self_0)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).current_action as libc::c_uint
            == FPI_DEVICE_ACTION_NONE as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-device.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"fp_device_finalize\0"))
                .as_ptr(),
            b"priv->current_action == FPI_DEVICE_ACTION_NONE\0" as *const u8
                as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if ((*priv_0).current_task).is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-device.c\0" as *const u8 as *const libc::c_char,
            218 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"fp_device_finalize\0"))
                .as_ptr(),
            b"priv->current_task == NULL\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*priv_0).is_open != 0 {
        g_log(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"User destroyed open device! Not cleaning up properly!\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut _pp: C2RustUnnamed_14 = C2RustUnnamed_14 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GSource) -> ()>,
        GDestroyNotify,
    >(Some(g_source_destroy as unsafe extern "C" fn(*mut GSource) -> ()));
    _pp.in_0 = &mut (*priv_0).temp_timeout as *mut *mut GSource as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    g_slist_free_full(
        (*priv_0).sources,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GSource) -> ()>,
            GDestroyNotify,
        >(Some(g_source_destroy as unsafe extern "C" fn(*mut GSource) -> ())),
    );
    let mut _pp_0: C2RustUnnamed_13 = C2RustUnnamed_13 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_0: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GSource) -> ()>,
        GDestroyNotify,
    >(Some(g_source_destroy as unsafe extern "C" fn(*mut GSource) -> ()));
    _pp_0
        .in_0 = &mut (*priv_0).current_idle_cancel_source as *mut *mut GSource
        as *mut libc::c_char;
    _p_0 = *_pp_0.out;
    if !_p_0.is_null() {
        *_pp_0.out = 0 as *mut libc::c_void;
        _destroy_0.expect("non-null function pointer")(_p_0);
    }
    let mut _pp_1: C2RustUnnamed_12 = C2RustUnnamed_12 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_1: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_1: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GSource) -> ()>,
        GDestroyNotify,
    >(Some(g_source_destroy as unsafe extern "C" fn(*mut GSource) -> ()));
    _pp_1
        .in_0 = &mut (*priv_0).current_task_idle_return_source as *mut *mut GSource
        as *mut libc::c_char;
    _p_1 = *_pp_1.out;
    if !_p_1.is_null() {
        *_pp_1.out = 0 as *mut libc::c_void;
        _destroy_1.expect("non-null function pointer")(_p_1);
    }
    let mut _pp_2: C2RustUnnamed_11 = C2RustUnnamed_11 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_2: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_2: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GSource) -> ()>,
        GDestroyNotify,
    >(Some(g_source_destroy as unsafe extern "C" fn(*mut GSource) -> ()));
    _pp_2
        .in_0 = &mut (*priv_0).critical_section_flush_source as *mut *mut GSource
        as *mut libc::c_char;
    _p_2 = *_pp_2.out;
    if !_p_2.is_null() {
        *_pp_2.out = 0 as *mut libc::c_void;
        _destroy_2.expect("non-null function pointer")(_p_2);
    }
    let mut _pp_3: C2RustUnnamed_10 = C2RustUnnamed_10 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_3: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_3: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp_3.in_0 = &mut (*priv_0).device_id as *mut *mut gchar as *mut libc::c_char;
    _p_3 = *_pp_3.out;
    if !_p_3.is_null() {
        *_pp_3.out = 0 as *mut libc::c_void;
        _destroy_3.expect("non-null function pointer")(_p_3);
    }
    let mut _pp_4: C2RustUnnamed_9 = C2RustUnnamed_9 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_4: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_4: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp_4.in_0 = &mut (*priv_0).device_name as *mut *mut gchar as *mut libc::c_char;
    _p_4 = *_pp_4.out;
    if !_p_4.is_null() {
        *_pp_4.out = 0 as *mut libc::c_void;
        _destroy_4.expect("non-null function pointer")(_p_4);
    }
    let mut _pp_5: C2RustUnnamed_8 = C2RustUnnamed_8 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_5: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_5: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp_5.in_0 = &mut (*priv_0).usb_device as *mut *mut GUsbDevice as *mut libc::c_char;
    _p_5 = *_pp_5.out;
    if !_p_5.is_null() {
        *_pp_5.out = 0 as *mut libc::c_void;
        _destroy_5.expect("non-null function pointer")(_p_5);
    }
    let mut _pp_6: C2RustUnnamed_7 = C2RustUnnamed_7 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_6: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_6: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp_6.in_0 = &mut (*priv_0).virtual_env as *mut *const gchar as *mut libc::c_char;
    _p_6 = *_pp_6.out;
    if !_p_6.is_null() {
        *_pp_6.out = 0 as *mut libc::c_void;
        _destroy_6.expect("non-null function pointer")(_p_6);
    }
    let mut _pp_7: C2RustUnnamed_6 = C2RustUnnamed_6 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_7: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_7: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp_7
        .in_0 = &mut (*priv_0).udev_data.spidev_path as *mut *mut gchar
        as *mut libc::c_char;
    _p_7 = *_pp_7.out;
    if !_p_7.is_null() {
        *_pp_7.out = 0 as *mut libc::c_void;
        _destroy_7.expect("non-null function pointer")(_p_7);
    }
    let mut _pp_8: C2RustUnnamed_5 = C2RustUnnamed_5 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_8: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_8: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp_8
        .in_0 = &mut (*priv_0).udev_data.hidraw_path as *mut *mut gchar
        as *mut libc::c_char;
    _p_8 = *_pp_8.out;
    if !_p_8.is_null() {
        *_pp_8.out = 0 as *mut libc::c_void;
        _destroy_8.expect("non-null function pointer")(_p_8);
    }
    ((*(g_type_check_class_cast(
        fp_device_parent_class as *mut GTypeClass,
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
    ) as *mut libc::c_void as *mut GObjectClass))
        .finalize)
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn fp_device_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut FpDevice = FP_DEVICE(object as gpointer);
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(self_0)
        as *mut FpDevicePrivate;
    let mut cls: *mut FpDeviceClass = FP_DEVICE_GET_CLASS(self_0 as gpointer);
    match prop_id {
        6 => {
            g_value_set_uint(value, (*priv_0).nr_enroll_stages as guint);
        }
        7 => {
            g_value_set_enum(value, (*priv_0).scan_type as gint);
        }
        8 => {
            g_value_set_flags(value, (*priv_0).finger_status as guint);
        }
        9 => {
            g_value_set_enum(value, (*priv_0).temp_current as gint);
        }
        1 => {
            g_value_set_static_string(
                value,
                (*FP_DEVICE_GET_CLASS(self_0 as gpointer)).id,
            );
        }
        2 => {
            g_value_set_string(value, (*priv_0).device_id);
        }
        3 => {
            g_value_set_string(value, (*priv_0).device_name);
        }
        4 => {
            g_value_set_boolean(value, (*priv_0).is_open);
        }
        5 => {
            g_value_set_boolean(value, (*priv_0).is_removed);
        }
        11 => {
            g_value_set_object(value, (*priv_0).usb_device as gpointer);
        }
        12 => {
            if (*cls).type_0 as libc::c_uint
                == FP_DEVICE_TYPE_UDEV as libc::c_int as libc::c_uint
            {
                g_value_set_string(value, g_strdup((*priv_0).udev_data.spidev_path));
            } else {
                g_value_set_string(value, 0 as *const gchar);
            }
        }
        13 => {
            if (*cls).type_0 as libc::c_uint
                == FP_DEVICE_TYPE_UDEV as libc::c_int as libc::c_uint
            {
                g_value_set_string(value, g_strdup((*priv_0).udev_data.hidraw_path));
            } else {
                g_value_set_string(value, 0 as *const gchar);
            }
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const libc::c_char,
                b"../libfprint/fp-device.c\0" as *const u8 as *const libc::c_char,
                308 as libc::c_int,
                b"property\0" as *const u8 as *const libc::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn fp_device_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut FpDevice = FP_DEVICE(object as gpointer);
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(self_0)
        as *mut FpDevicePrivate;
    let mut cls: *mut FpDeviceClass = FP_DEVICE_GET_CLASS(self_0 as gpointer);
    match prop_id {
        10 => {
            if (*cls).type_0 as libc::c_uint
                == FP_DEVICE_TYPE_VIRTUAL as libc::c_int as libc::c_uint
            {
                (*priv_0).virtual_env = g_value_dup_string(value);
            } else if ({
                let mut _g_boolean_var_: libc::c_int = 0;
                if (g_value_get_string(value)).is_null() {
                    _g_boolean_var_ = 1 as libc::c_int;
                } else {
                    _g_boolean_var_ = 0 as libc::c_int;
                }
                _g_boolean_var_
            }) as libc::c_long != 0
            {} else {
                g_assertion_message_expr(
                    b"libfprint-device\0" as *const u8 as *const libc::c_char,
                    b"../libfprint/fp-device.c\0" as *const u8 as *const libc::c_char,
                    329 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"fp_device_set_property\0"))
                        .as_ptr(),
                    b"g_value_get_string (value) == NULL\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        11 => {
            if (*cls).type_0 as libc::c_uint
                == FP_DEVICE_TYPE_USB as libc::c_int as libc::c_uint
            {
                (*priv_0).usb_device = g_value_dup_object(value) as *mut GUsbDevice;
            } else if ({
                let mut _g_boolean_var_: libc::c_int = 0;
                if (g_value_get_object(value)).is_null() {
                    _g_boolean_var_ = 1 as libc::c_int;
                } else {
                    _g_boolean_var_ = 0 as libc::c_int;
                }
                _g_boolean_var_
            }) as libc::c_long != 0
            {} else {
                g_assertion_message_expr(
                    b"libfprint-device\0" as *const u8 as *const libc::c_char,
                    b"../libfprint/fp-device.c\0" as *const u8 as *const libc::c_char,
                    336 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"fp_device_set_property\0"))
                        .as_ptr(),
                    b"g_value_get_object (value) == NULL\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        12 => {
            if (*cls).type_0 as libc::c_uint
                == FP_DEVICE_TYPE_UDEV as libc::c_int as libc::c_uint
            {
                (*priv_0).udev_data.spidev_path = g_value_dup_string(value);
            } else if ({
                let mut _g_boolean_var_: libc::c_int = 0;
                if (g_value_get_string(value)).is_null() {
                    _g_boolean_var_ = 1 as libc::c_int;
                } else {
                    _g_boolean_var_ = 0 as libc::c_int;
                }
                _g_boolean_var_
            }) as libc::c_long != 0
            {} else {
                g_assertion_message_expr(
                    b"libfprint-device\0" as *const u8 as *const libc::c_char,
                    b"../libfprint/fp-device.c\0" as *const u8 as *const libc::c_char,
                    343 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"fp_device_set_property\0"))
                        .as_ptr(),
                    b"g_value_get_string (value) == NULL\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        13 => {
            if (*cls).type_0 as libc::c_uint
                == FP_DEVICE_TYPE_UDEV as libc::c_int as libc::c_uint
            {
                (*priv_0).udev_data.hidraw_path = g_value_dup_string(value);
            } else if ({
                let mut _g_boolean_var_: libc::c_int = 0;
                if (g_value_get_string(value)).is_null() {
                    _g_boolean_var_ = 1 as libc::c_int;
                } else {
                    _g_boolean_var_ = 0 as libc::c_int;
                }
                _g_boolean_var_
            }) as libc::c_long != 0
            {} else {
                g_assertion_message_expr(
                    b"libfprint-device\0" as *const u8 as *const libc::c_char,
                    b"../libfprint/fp-device.c\0" as *const u8 as *const libc::c_char,
                    350 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"fp_device_set_property\0"))
                        .as_ptr(),
                    b"g_value_get_string (value) == NULL\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        14 => {
            (*priv_0).driver_data = g_value_get_uint64(value);
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const libc::c_char,
                b"../libfprint/fp-device.c\0" as *const u8 as *const libc::c_char,
                358 as libc::c_int,
                b"property\0" as *const u8 as *const libc::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn device_idle_probe_cb(
    mut self_0: *mut FpDevice,
    mut user_data: gpointer,
) {
    fpi_device_configure_wakeup(self_0, 0 as libc::c_int);
    if ((*FP_DEVICE_GET_CLASS(self_0 as gpointer)).probe).is_none() {
        fpi_device_probe_complete(
            self_0,
            0 as *const gchar,
            0 as *const gchar,
            0 as *mut GError,
        );
    } else {
        ((*FP_DEVICE_GET_CLASS(self_0 as gpointer)).probe)
            .expect("non-null function pointer")(self_0);
    };
}
unsafe extern "C" fn fp_device_async_initable_init_async(
    mut initable: *mut GAsyncInitable,
    mut io_priority: libc::c_int,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut self_0: *mut FpDevice = FP_DEVICE(initable as gpointer);
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(self_0)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*priv_0).is_open == 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-device.c\0" as *const u8 as *const libc::c_char,
            392 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"fp_device_async_initable_init_async\0"))
                .as_ptr(),
            b"!priv->is_open\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if ((*priv_0).current_task).is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-device.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"fp_device_async_initable_init_async\0"))
                .as_ptr(),
            b"!priv->current_task\0" as *const u8 as *const libc::c_char,
        );
    }
    task = g_task_new(self_0 as gpointer, cancellable, callback, user_data);
    if g_task_return_error_if_cancelled(task) != 0 {
        return;
    }
    (*priv_0).current_action = FPI_DEVICE_ACTION_PROBE;
    (*priv_0)
        .current_task = (if 0 as libc::c_int != 0 {
        task as *mut libc::c_void
    } else {
        g_steal_pointer(&mut task as *mut GTask_autoptr as gpointer)
    }) as *mut GTask;
    setup_task_cancellable(self_0);
    g_source_set_name(
        fpi_device_add_timeout(
            self_0,
            0 as libc::c_int,
            Some(
                device_idle_probe_cb
                    as unsafe extern "C" fn(*mut FpDevice, gpointer) -> (),
            ),
            0 as *mut libc::c_void,
            None,
        ),
        b"libusb probe in idle\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn fp_device_async_initable_init_finish(
    mut initable: *mut GAsyncInitable,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    return g_task_propagate_boolean(
        g_type_check_instance_cast(res as *mut GTypeInstance, g_task_get_type())
            as *mut libc::c_void as *mut GTask,
        error,
    );
}
unsafe extern "C" fn fp_device_async_initable_iface_init(
    mut iface: *mut GAsyncInitableIface,
) {
    (*iface)
        .init_async = Some(
        fp_device_async_initable_init_async
            as unsafe extern "C" fn(
                *mut GAsyncInitable,
                libc::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    );
    (*iface)
        .init_finish = Some(
        fp_device_async_initable_init_finish
            as unsafe extern "C" fn(
                *mut GAsyncInitable,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gboolean,
    );
}
unsafe extern "C" fn fp_device_class_init(mut klass: *mut FpDeviceClass) {
    let mut object_class: *mut GObjectClass = g_type_check_class_cast(
        klass as *mut GTypeClass,
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
    ) as *mut libc::c_void as *mut GObjectClass;
    (*object_class)
        .constructed = Some(
        fp_device_constructed as unsafe extern "C" fn(*mut GObject) -> (),
    );
    (*object_class)
        .finalize = Some(fp_device_finalize as unsafe extern "C" fn(*mut GObject) -> ());
    (*object_class)
        .get_property = Some(
        fp_device_get_property
            as unsafe extern "C" fn(
                *mut GObject,
                guint,
                *mut GValue,
                *mut GParamSpec,
            ) -> (),
    );
    (*object_class)
        .set_property = Some(
        fp_device_set_property
            as unsafe extern "C" fn(
                *mut GObject,
                guint,
                *const GValue,
                *mut GParamSpec,
            ) -> (),
    );
    properties[PROP_NR_ENROLL_STAGES as libc::c_int
        as usize] = g_param_spec_uint(
        b"nr-enroll-stages\0" as *const u8 as *const libc::c_char,
        b"EnrollStages\0" as *const u8 as *const libc::c_char,
        b"Number of enroll stages needed on the device\0" as *const u8
            as *const libc::c_char,
        0 as libc::c_int as guint,
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint),
        0 as libc::c_int as guint,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READABLE as libc::c_int)
            as GParamFlags,
    );
    properties[PROP_SCAN_TYPE as libc::c_int
        as usize] = g_param_spec_enum(
        b"scan-type\0" as *const u8 as *const libc::c_char,
        b"ScanType\0" as *const u8 as *const libc::c_char,
        b"The scan type of the device\0" as *const u8 as *const libc::c_char,
        fp_scan_type_get_type(),
        FP_SCAN_TYPE_SWIPE as libc::c_int,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READABLE as libc::c_int)
            as GParamFlags,
    );
    properties[PROP_FINGER_STATUS as libc::c_int
        as usize] = g_param_spec_flags(
        b"finger-status\0" as *const u8 as *const libc::c_char,
        b"FingerStatus\0" as *const u8 as *const libc::c_char,
        b"The status of the finger\0" as *const u8 as *const libc::c_char,
        fp_finger_status_flags_get_type(),
        FP_FINGER_STATUS_NONE as libc::c_int as guint,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READABLE as libc::c_int)
            as GParamFlags,
    );
    properties[PROP_TEMPERATURE as libc::c_int
        as usize] = g_param_spec_enum(
        b"temperature\0" as *const u8 as *const libc::c_char,
        b"Temperature\0" as *const u8 as *const libc::c_char,
        b"The temperature estimation for device to prevent overheating.\0" as *const u8
            as *const libc::c_char,
        fp_temperature_get_type(),
        FP_TEMPERATURE_COLD as libc::c_int,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READABLE as libc::c_int)
            as GParamFlags,
    );
    properties[PROP_DRIVER as libc::c_int
        as usize] = g_param_spec_string(
        b"driver\0" as *const u8 as *const libc::c_char,
        b"Driver\0" as *const u8 as *const libc::c_char,
        b"String describing the driver\0" as *const u8 as *const libc::c_char,
        0 as *const gchar,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READABLE as libc::c_int)
            as GParamFlags,
    );
    properties[PROP_DEVICE_ID as libc::c_int
        as usize] = g_param_spec_string(
        b"device-id\0" as *const u8 as *const libc::c_char,
        b"Device ID\0" as *const u8 as *const libc::c_char,
        b"String describing the device, often generic but may be a serial number\0"
            as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READABLE as libc::c_int)
            as GParamFlags,
    );
    properties[PROP_NAME as libc::c_int
        as usize] = g_param_spec_string(
        b"name\0" as *const u8 as *const libc::c_char,
        b"Device Name\0" as *const u8 as *const libc::c_char,
        b"Human readable name for the device\0" as *const u8 as *const libc::c_char,
        0 as *const gchar,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READABLE as libc::c_int)
            as GParamFlags,
    );
    properties[PROP_OPEN as libc::c_int
        as usize] = g_param_spec_boolean(
        b"open\0" as *const u8 as *const libc::c_char,
        b"Opened\0" as *const u8 as *const libc::c_char,
        b"Whether the device is open or not\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READABLE as libc::c_int)
            as GParamFlags,
    );
    properties[PROP_REMOVED as libc::c_int
        as usize] = g_param_spec_boolean(
        b"removed\0" as *const u8 as *const libc::c_char,
        b"Removed\0" as *const u8 as *const libc::c_char,
        b"Whether the device has been removed from the system\0" as *const u8
            as *const libc::c_char,
        0 as libc::c_int,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READABLE as libc::c_int)
            as GParamFlags,
    );
    signals[REMOVED_SIGNAL as libc::c_int
        as usize] = g_signal_new(
        b"removed\0" as *const u8 as *const libc::c_char,
        (*(klass as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        0 as libc::c_int as guint,
        None,
        0 as *mut libc::c_void,
        Some(
            g_cclosure_marshal_VOID__VOID
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
        0 as libc::c_int as guint,
    );
    properties[PROP_FPI_ENVIRON as libc::c_int
        as usize] = g_param_spec_string(
        b"fpi-environ\0" as *const u8 as *const libc::c_char,
        b"Virtual Environment\0" as *const u8 as *const libc::c_char,
        b"Private: The environment variable for the virtual device\0" as *const u8
            as *const libc::c_char,
        0 as *const gchar,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_WRITABLE as libc::c_int
            | G_PARAM_CONSTRUCT_ONLY as libc::c_int) as GParamFlags,
    );
    properties[PROP_FPI_USB_DEVICE as libc::c_int
        as usize] = g_param_spec_object(
        b"fpi-usb-device\0" as *const u8 as *const libc::c_char,
        b"USB Device\0" as *const u8 as *const libc::c_char,
        b"Private: The USB device for the device\0" as *const u8 as *const libc::c_char,
        g_usb_device_get_type(),
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READWRITE as libc::c_int
            | G_PARAM_CONSTRUCT_ONLY as libc::c_int) as GParamFlags,
    );
    properties[PROP_FPI_UDEV_DATA_SPIDEV as libc::c_int
        as usize] = g_param_spec_string(
        b"fpi-udev-data-spidev\0" as *const u8 as *const libc::c_char,
        b"Udev data: spidev path\0" as *const u8 as *const libc::c_char,
        b"Private: The path to /dev/spidevN.M\0" as *const u8 as *const libc::c_char,
        0 as *const gchar,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READWRITE as libc::c_int
            | G_PARAM_CONSTRUCT_ONLY as libc::c_int) as GParamFlags,
    );
    properties[PROP_FPI_UDEV_DATA_HIDRAW as libc::c_int
        as usize] = g_param_spec_string(
        b"fpi-udev-data-hidraw\0" as *const u8 as *const libc::c_char,
        b"Udev data: hidraw path\0" as *const u8 as *const libc::c_char,
        b"Private: The path to /dev/hidrawN\0" as *const u8 as *const libc::c_char,
        0 as *const gchar,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READWRITE as libc::c_int
            | G_PARAM_CONSTRUCT_ONLY as libc::c_int) as GParamFlags,
    );
    properties[PROP_FPI_DRIVER_DATA as libc::c_int
        as usize] = g_param_spec_uint64(
        b"fpi-driver-data\0" as *const u8 as *const libc::c_char,
        b"Driver Data\0" as *const u8 as *const libc::c_char,
        b"Private: The driver data from the ID table entry\0" as *const u8
            as *const libc::c_char,
        0 as libc::c_int as guint64,
        0xffffffffffffffff as libc::c_ulong,
        0 as libc::c_int as guint64,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_WRITABLE as libc::c_int
            | G_PARAM_CONSTRUCT_ONLY as libc::c_int) as GParamFlags,
    );
    g_object_class_install_properties(
        object_class,
        N_PROPS as libc::c_int as guint,
        properties.as_mut_ptr(),
    );
}
unsafe extern "C" fn fp_device_init(mut self_0: *mut FpDevice) {}
#[no_mangle]
pub unsafe extern "C" fn fp_device_get_driver(
    mut device: *mut FpDevice,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fp_device_get_driver\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *const gchar;
    }
    return (*FP_DEVICE_GET_CLASS(device as gpointer)).id;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_get_device_id(
    mut device: *mut FpDevice,
) -> *const gchar {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"fp_device_get_device_id\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *const gchar;
    }
    return (*priv_0).device_id;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_get_name(mut device: *mut FpDevice) -> *const gchar {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"fp_device_get_name\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *const gchar;
    }
    return (*priv_0).device_name;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_is_open(mut device: *mut FpDevice) -> gboolean {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"fp_device_is_open\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    return (*priv_0).is_open;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_get_scan_type(
    mut device: *mut FpDevice,
) -> FpScanType {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"fp_device_get_scan_type\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return FP_SCAN_TYPE_SWIPE;
    }
    return (*priv_0).scan_type;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_get_finger_status(
    mut device: *mut FpDevice,
) -> FpFingerStatusFlags {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fp_device_get_finger_status\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return FP_FINGER_STATUS_NONE;
    }
    return (*priv_0).finger_status;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_get_nr_enroll_stages(
    mut device: *mut FpDevice,
) -> gint {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"fp_device_get_nr_enroll_stages\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return (*priv_0).nr_enroll_stages;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_get_temperature(
    mut device: *mut FpDevice,
) -> FpTemperature {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"fp_device_get_temperature\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 4294967295 as FpTemperature;
    }
    return (*priv_0).temp_current;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_supports_identify(
    mut device: *mut FpDevice,
) -> gboolean {
    let mut cls: *mut FpDeviceClass = FP_DEVICE_GET_CLASS(device as gpointer);
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fp_device_supports_identify\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    return (((*cls).identify).is_some()
        && (*priv_0).features as libc::c_uint
            & FP_DEVICE_FEATURE_IDENTIFY as libc::c_int as libc::c_uint != 0)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_supports_capture(
    mut device: *mut FpDevice,
) -> gboolean {
    let mut cls: *mut FpDeviceClass = FP_DEVICE_GET_CLASS(device as gpointer);
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fp_device_supports_capture\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    return (((*cls).capture).is_some()
        && (*priv_0).features as libc::c_uint
            & FP_DEVICE_FEATURE_CAPTURE as libc::c_int as libc::c_uint != 0)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_has_storage(mut device: *mut FpDevice) -> gboolean {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fp_device_has_storage\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    return ((*priv_0).features as libc::c_uint
        & FP_DEVICE_FEATURE_STORAGE as libc::c_int as libc::c_uint != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_open(
    mut device: *mut FpDevice,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut error: *mut GError = 0 as *mut GError;
    task = g_task_new(device as gpointer, cancellable, callback, user_data);
    if g_task_return_error_if_cancelled(task) != 0 {
        return;
    }
    if (*priv_0).is_open != 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_ALREADY_OPEN));
        return;
    }
    if !((*priv_0).current_task).is_null() || (*priv_0).is_suspended != 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_BUSY));
        return;
    }
    match (*priv_0).type_0 as libc::c_uint {
        2 => {
            if g_usb_device_open((*priv_0).usb_device, &mut error) == 0 {
                g_task_return_error(task, error);
                return;
            }
        }
        0 | 1 => {}
        _ => {
            g_assertion_message_expr(
                b"libfprint-device\0" as *const u8 as *const libc::c_char,
                b"../libfprint/fp-device.c\0" as *const u8 as *const libc::c_char,
                856 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 15],
                    &[libc::c_char; 15],
                >(b"fp_device_open\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    }
    (*priv_0).current_action = FPI_DEVICE_ACTION_OPEN;
    (*priv_0)
        .current_task = (if 0 as libc::c_int != 0 {
        task as *mut libc::c_void
    } else {
        g_steal_pointer(&mut task as *mut GTask_autoptr as gpointer)
    }) as *mut GTask;
    setup_task_cancellable(device);
    fpi_device_report_finger_status(device, FP_FINGER_STATUS_NONE);
    ((*FP_DEVICE_GET_CLASS(device as gpointer)).open)
        .expect("non-null function pointer")(device);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_open_finish(
    mut device: *mut FpDevice,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    return g_task_propagate_boolean(
        g_type_check_instance_cast(result as *mut GTypeInstance, g_task_get_type())
            as *mut libc::c_void as *mut GTask,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_close(
    mut device: *mut FpDevice,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    task = g_task_new(device as gpointer, cancellable, callback, user_data);
    if g_task_return_error_if_cancelled(task) != 0 {
        return;
    }
    if (*priv_0).is_open == 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_NOT_OPEN));
        return;
    }
    if !((*priv_0).current_task).is_null() || (*priv_0).is_suspended != 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_BUSY));
        return;
    }
    (*priv_0).current_action = FPI_DEVICE_ACTION_CLOSE;
    (*priv_0)
        .current_task = (if 0 as libc::c_int != 0 {
        task as *mut libc::c_void
    } else {
        g_steal_pointer(&mut task as *mut GTask_autoptr as gpointer)
    }) as *mut GTask;
    setup_task_cancellable(device);
    ((*FP_DEVICE_GET_CLASS(device as gpointer)).close)
        .expect("non-null function pointer")(device);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_close_finish(
    mut device: *mut FpDevice,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    return g_task_propagate_boolean(
        g_type_check_instance_cast(result as *mut GTypeInstance, g_task_get_type())
            as *mut libc::c_void as *mut GTask,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_suspend(
    mut device: *mut FpDevice,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    task = g_task_new(device as gpointer, cancellable, callback, user_data);
    if !((*priv_0).suspend_resume_task).is_null() || (*priv_0).is_suspended != 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_BUSY));
        return;
    }
    if (*priv_0).is_removed != 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_REMOVED));
        return;
    }
    (*priv_0)
        .suspend_resume_task = (if 0 as libc::c_int != 0 {
        task as *mut libc::c_void
    } else {
        g_steal_pointer(&mut task as *mut GTask_autoptr as gpointer)
    }) as *mut GTask;
    fpi_device_suspend(device);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_suspend_finish(
    mut device: *mut FpDevice,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    return g_task_propagate_boolean(
        g_type_check_instance_cast(result as *mut GTypeInstance, g_task_get_type())
            as *mut libc::c_void as *mut GTask,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_resume(
    mut device: *mut FpDevice,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    task = g_task_new(device as gpointer, cancellable, callback, user_data);
    if !((*priv_0).suspend_resume_task).is_null() || (*priv_0).is_suspended == 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_BUSY));
        return;
    }
    if (*priv_0).is_removed != 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_REMOVED));
        return;
    }
    (*priv_0)
        .suspend_resume_task = (if 0 as libc::c_int != 0 {
        task as *mut libc::c_void
    } else {
        g_steal_pointer(&mut task as *mut GTask_autoptr as gpointer)
    }) as *mut GTask;
    fpi_device_resume(device);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_resume_finish(
    mut device: *mut FpDevice,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    return g_task_propagate_boolean(
        g_type_check_instance_cast(result as *mut GTypeInstance, g_task_get_type())
            as *mut libc::c_void as *mut GTask,
        error,
    );
}
unsafe extern "C" fn enroll_data_free(mut data: *mut FpEnrollData) {
    if ((*data).enroll_progress_destroy).is_some() {
        ((*data).enroll_progress_destroy)
            .expect("non-null function pointer")((*data).enroll_progress_data);
    }
    (*data).enroll_progress_data = 0 as *mut libc::c_void;
    let mut _pp: C2RustUnnamed_15 = C2RustUnnamed_15 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*data).print as *mut *mut FpPrint as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    g_free(data as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_enroll(
    mut device: *mut FpDevice,
    mut template_print: *mut FpPrint,
    mut cancellable: *mut GCancellable,
    mut progress_cb: FpEnrollProgress,
    mut progress_data: gpointer,
    mut progress_destroy: GDestroyNotify,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut data: *mut FpEnrollData = 0 as *mut FpEnrollData;
    let mut print_type: FpiPrintType = FPI_PRINT_UNDEFINED;
    task = g_task_new(device as gpointer, cancellable, callback, user_data);
    if g_task_return_error_if_cancelled(task) != 0 {
        return;
    }
    if (*priv_0).is_open == 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_NOT_OPEN));
        return;
    }
    if !((*priv_0).current_task).is_null() || (*priv_0).is_suspended != 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_BUSY));
        return;
    }
    if FP_IS_PRINT(template_print as gpointer) == 0 {
        g_task_return_error(
            task,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_DATA_INVALID,
                b"User did not pass a print template!\0" as *const u8
                    as *const libc::c_char,
            ),
        );
        return;
    }
    g_object_get(
        template_print as gpointer,
        b"fpi-type\0" as *const u8 as *const libc::c_char,
        &mut print_type as *mut FpiPrintType,
        0 as *mut libc::c_void,
    );
    if print_type as libc::c_uint != FPI_PRINT_UNDEFINED as libc::c_int as libc::c_uint {
        if fp_device_has_feature(device, FP_DEVICE_FEATURE_UPDATE_PRINT) == 0 {
            g_task_return_error(
                task,
                fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_DATA_INVALID,
                    b"A device does not support print updates!\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            return;
        }
        if fp_print_compatible(template_print, device) == 0 {
            g_task_return_error(
                task,
                fpi_device_error_new_msg(
                    FP_DEVICE_ERROR_DATA_INVALID,
                    b"The print and device must have a matching driver and device id for a fingerprint update to succeed\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            return;
        }
    }
    fpi_device_update_temp(device, (0 as libc::c_int == 0) as libc::c_int);
    if (*priv_0).temp_current as libc::c_uint
        == FP_TEMPERATURE_HOT as libc::c_int as libc::c_uint
    {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_TOO_HOT));
        fpi_device_update_temp(device, 0 as libc::c_int);
        return;
    }
    (*priv_0).current_action = FPI_DEVICE_ACTION_ENROLL;
    (*priv_0)
        .current_task = (if 0 as libc::c_int != 0 {
        task as *mut libc::c_void
    } else {
        g_steal_pointer(&mut task as *mut GTask_autoptr as gpointer)
    }) as *mut GTask;
    setup_task_cancellable(device);
    data = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<FpEnrollData>() as libc::c_ulong;
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
    }) as *mut FpEnrollData;
    (*data).print = g_object_ref_sink(template_print as gpointer) as *mut FpPrint;
    (*data).enroll_progress_cb = progress_cb;
    (*data).enroll_progress_data = progress_data;
    (*data).enroll_progress_destroy = progress_destroy;
    g_task_set_task_data(
        (*priv_0).current_task,
        data as gpointer,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FpEnrollData) -> ()>,
            GDestroyNotify,
        >(Some(enroll_data_free as unsafe extern "C" fn(*mut FpEnrollData) -> ())),
    );
    ((*FP_DEVICE_GET_CLASS(device as gpointer)).enroll)
        .expect("non-null function pointer")(device);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_enroll_finish(
    mut device: *mut FpDevice,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut FpPrint {
    return g_task_propagate_pointer(
        g_type_check_instance_cast(result as *mut GTypeInstance, g_task_get_type())
            as *mut libc::c_void as *mut GTask,
        error,
    ) as *mut FpPrint;
}
unsafe extern "C" fn match_data_free(mut data: *mut FpMatchData) {
    let mut _pp: C2RustUnnamed_19 = C2RustUnnamed_19 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*data).print as *mut *mut FpPrint as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    let mut _pp_0: C2RustUnnamed_18 = C2RustUnnamed_18 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_0: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp_0.in_0 = &mut (*data).match_0 as *mut *mut FpPrint as *mut libc::c_char;
    _p_0 = *_pp_0.out;
    if !_p_0.is_null() {
        *_pp_0.out = 0 as *mut libc::c_void;
        _destroy_0.expect("non-null function pointer")(_p_0);
    }
    g_clear_error(&mut (*data).error);
    if ((*data).match_destroy).is_some() {
        ((*data).match_destroy).expect("non-null function pointer")((*data).match_data);
    }
    (*data).match_data = 0 as *mut libc::c_void;
    let mut _pp_1: C2RustUnnamed_17 = C2RustUnnamed_17 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_1: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_1: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp_1.in_0 = &mut (*data).enrolled_print as *mut *mut FpPrint as *mut libc::c_char;
    _p_1 = *_pp_1.out;
    if !_p_1.is_null() {
        *_pp_1.out = 0 as *mut libc::c_void;
        _destroy_1.expect("non-null function pointer")(_p_1);
    }
    let mut _pp_2: C2RustUnnamed_16 = C2RustUnnamed_16 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_2: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_2: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GPtrArray) -> ()>,
        GDestroyNotify,
    >(Some(g_ptr_array_unref as unsafe extern "C" fn(*mut GPtrArray) -> ()));
    _pp_2.in_0 = &mut (*data).gallery as *mut *mut GPtrArray as *mut libc::c_char;
    _p_2 = *_pp_2.out;
    if !_p_2.is_null() {
        *_pp_2.out = 0 as *mut libc::c_void;
        _destroy_2.expect("non-null function pointer")(_p_2);
    }
    g_free(data as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_verify(
    mut device: *mut FpDevice,
    mut enrolled_print: *mut FpPrint,
    mut cancellable: *mut GCancellable,
    mut match_cb: FpMatchCb,
    mut match_data: gpointer,
    mut match_destroy: GDestroyNotify,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut cls: *mut FpDeviceClass = FP_DEVICE_GET_CLASS(device as gpointer);
    let mut data: *mut FpMatchData = 0 as *mut FpMatchData;
    task = g_task_new(device as gpointer, cancellable, callback, user_data);
    if g_task_return_error_if_cancelled(task) != 0 {
        return;
    }
    if (*priv_0).is_open == 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_NOT_OPEN));
        return;
    }
    if !((*priv_0).current_task).is_null() || (*priv_0).is_suspended != 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_BUSY));
        return;
    }
    if ((*cls).verify).is_none()
        || (*priv_0).features as libc::c_uint
            & FP_DEVICE_FEATURE_VERIFY as libc::c_int as libc::c_uint == 0
    {
        g_task_return_error(
            task,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_NOT_SUPPORTED,
                b"Device has no verification support\0" as *const u8
                    as *const libc::c_char,
            ),
        );
        return;
    }
    fpi_device_update_temp(device, (0 as libc::c_int == 0) as libc::c_int);
    if (*priv_0).temp_current as libc::c_uint
        == FP_TEMPERATURE_HOT as libc::c_int as libc::c_uint
    {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_TOO_HOT));
        fpi_device_update_temp(device, 0 as libc::c_int);
        return;
    }
    (*priv_0).current_action = FPI_DEVICE_ACTION_VERIFY;
    (*priv_0)
        .current_task = (if 0 as libc::c_int != 0 {
        task as *mut libc::c_void
    } else {
        g_steal_pointer(&mut task as *mut GTask_autoptr as gpointer)
    }) as *mut GTask;
    setup_task_cancellable(device);
    data = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<FpMatchData>() as libc::c_ulong;
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
    }) as *mut FpMatchData;
    (*data).enrolled_print = g_object_ref(enrolled_print as gpointer) as *mut FpPrint;
    (*data).match_cb = match_cb;
    (*data).match_data = match_data;
    (*data).match_destroy = match_destroy;
    g_task_set_task_data(
        (*priv_0).current_task,
        data as gpointer,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FpMatchData) -> ()>,
            GDestroyNotify,
        >(Some(match_data_free as unsafe extern "C" fn(*mut FpMatchData) -> ())),
    );
    ((*cls).verify).expect("non-null function pointer")(device);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_verify_finish(
    mut device: *mut FpDevice,
    mut result: *mut GAsyncResult,
    mut match_0: *mut gboolean,
    mut print: *mut *mut FpPrint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut res: gint = 0;
    res = g_task_propagate_int(
        g_type_check_instance_cast(result as *mut GTypeInstance, g_task_get_type())
            as *mut libc::c_void as *mut GTask,
        error,
    ) as gint;
    if !print.is_null() {
        let mut data: *mut FpMatchData = 0 as *mut FpMatchData;
        data = g_task_get_task_data(
            g_type_check_instance_cast(result as *mut GTypeInstance, g_task_get_type())
                as *mut libc::c_void as *mut GTask,
        ) as *mut FpMatchData;
        *print = if !data.is_null() { (*data).print } else { 0 as *mut FpPrint };
        if !(*print).is_null() {
            g_object_ref(*print as gpointer);
        }
    }
    if !match_0.is_null() {
        *match_0 = (res == FPI_MATCH_SUCCESS as libc::c_int) as libc::c_int;
    }
    return (res != FPI_MATCH_ERROR as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_identify(
    mut device: *mut FpDevice,
    mut prints: *mut GPtrArray,
    mut cancellable: *mut GCancellable,
    mut match_cb: FpMatchCb,
    mut match_data: gpointer,
    mut match_destroy: GDestroyNotify,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut cls: *mut FpDeviceClass = FP_DEVICE_GET_CLASS(device as gpointer);
    let mut data: *mut FpMatchData = 0 as *mut FpMatchData;
    let mut i: libc::c_int = 0;
    task = g_task_new(device as gpointer, cancellable, callback, user_data);
    if g_task_return_error_if_cancelled(task) != 0 {
        return;
    }
    if (*priv_0).is_open == 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_NOT_OPEN));
        return;
    }
    if !((*priv_0).current_task).is_null() || (*priv_0).is_suspended != 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_BUSY));
        return;
    }
    if ((*cls).identify).is_none()
        || (*priv_0).features as libc::c_uint
            & FP_DEVICE_FEATURE_IDENTIFY as libc::c_int as libc::c_uint == 0
    {
        g_task_return_error(
            task,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_NOT_SUPPORTED,
                b"Device has not identification support\0" as *const u8
                    as *const libc::c_char,
            ),
        );
        return;
    }
    if prints.is_null() {
        g_task_return_error(
            task,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_DATA_INVALID,
                b"Invalid gallery array\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    fpi_device_update_temp(device, (0 as libc::c_int == 0) as libc::c_int);
    if (*priv_0).temp_current as libc::c_uint
        == FP_TEMPERATURE_HOT as libc::c_int as libc::c_uint
    {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_TOO_HOT));
        fpi_device_update_temp(device, 0 as libc::c_int);
        return;
    }
    (*priv_0).current_action = FPI_DEVICE_ACTION_IDENTIFY;
    (*priv_0)
        .current_task = (if 0 as libc::c_int != 0 {
        task as *mut libc::c_void
    } else {
        g_steal_pointer(&mut task as *mut GTask_autoptr as gpointer)
    }) as *mut GTask;
    setup_task_cancellable(device);
    data = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<FpMatchData>() as libc::c_ulong;
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
    }) as *mut FpMatchData;
    (*data)
        .gallery = g_ptr_array_new_full(
        (*prints).len,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*prints).len {
        g_ptr_array_add(
            (*data).gallery,
            g_object_ref(*((*prints).pdata).offset(i as isize)),
        );
        i += 1;
    }
    (*data).match_cb = match_cb;
    (*data).match_data = match_data;
    (*data).match_destroy = match_destroy;
    g_task_set_task_data(
        (*priv_0).current_task,
        data as gpointer,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FpMatchData) -> ()>,
            GDestroyNotify,
        >(Some(match_data_free as unsafe extern "C" fn(*mut FpMatchData) -> ())),
    );
    ((*cls).identify).expect("non-null function pointer")(device);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_identify_finish(
    mut device: *mut FpDevice,
    mut result: *mut GAsyncResult,
    mut match_0: *mut *mut FpPrint,
    mut print: *mut *mut FpPrint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut data: *mut FpMatchData = 0 as *mut FpMatchData;
    data = g_task_get_task_data(
        g_type_check_instance_cast(result as *mut GTypeInstance, g_task_get_type())
            as *mut libc::c_void as *mut GTask,
    ) as *mut FpMatchData;
    if !print.is_null() {
        *print = if !data.is_null() { (*data).print } else { 0 as *mut FpPrint };
        if !(*print).is_null() {
            g_object_ref(*print as gpointer);
        }
    }
    if !match_0.is_null() {
        *match_0 = if !data.is_null() { (*data).match_0 } else { 0 as *mut FpPrint };
        if !(*match_0).is_null() {
            g_object_ref(*match_0 as gpointer);
        }
    }
    return g_task_propagate_boolean(
        g_type_check_instance_cast(result as *mut GTypeInstance, g_task_get_type())
            as *mut libc::c_void as *mut GTask,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_capture(
    mut device: *mut FpDevice,
    mut wait_for_finger: gboolean,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut cls: *mut FpDeviceClass = FP_DEVICE_GET_CLASS(device as gpointer);
    task = g_task_new(device as gpointer, cancellable, callback, user_data);
    if g_task_return_error_if_cancelled(task) != 0 {
        return;
    }
    if (*priv_0).is_open == 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_NOT_OPEN));
        return;
    }
    if !((*priv_0).current_task).is_null() || (*priv_0).is_suspended != 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_BUSY));
        return;
    }
    if ((*cls).capture).is_none()
        || (*priv_0).features as libc::c_uint
            & FP_DEVICE_FEATURE_CAPTURE as libc::c_int as libc::c_uint == 0
    {
        g_task_return_error(
            task,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_NOT_SUPPORTED,
                b"Device has no verification support\0" as *const u8
                    as *const libc::c_char,
            ),
        );
        return;
    }
    fpi_device_update_temp(device, (0 as libc::c_int == 0) as libc::c_int);
    if (*priv_0).temp_current as libc::c_uint
        == FP_TEMPERATURE_HOT as libc::c_int as libc::c_uint
    {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_TOO_HOT));
        fpi_device_update_temp(device, 0 as libc::c_int);
        return;
    }
    (*priv_0).current_action = FPI_DEVICE_ACTION_CAPTURE;
    (*priv_0)
        .current_task = (if 0 as libc::c_int != 0 {
        task as *mut libc::c_void
    } else {
        g_steal_pointer(&mut task as *mut GTask_autoptr as gpointer)
    }) as *mut GTask;
    setup_task_cancellable(device);
    (*priv_0).wait_for_finger = wait_for_finger;
    ((*cls).capture).expect("non-null function pointer")(device);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_capture_finish(
    mut device: *mut FpDevice,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut FpImage {
    return g_task_propagate_pointer(
        g_type_check_instance_cast(result as *mut GTypeInstance, g_task_get_type())
            as *mut libc::c_void as *mut GTask,
        error,
    ) as *mut FpImage;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_delete_print(
    mut device: *mut FpDevice,
    mut enrolled_print: *mut FpPrint,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut cls: *mut FpDeviceClass = FP_DEVICE_GET_CLASS(device as gpointer);
    task = g_task_new(device as gpointer, cancellable, callback, user_data);
    if g_task_return_error_if_cancelled(task) != 0 {
        return;
    }
    if (*priv_0).is_open == 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_NOT_OPEN));
        return;
    }
    if !((*priv_0).current_task).is_null() || (*priv_0).is_suspended != 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_BUSY));
        return;
    }
    if ((*cls).delete).is_none()
        || (*priv_0).features as libc::c_uint
            & FP_DEVICE_FEATURE_STORAGE_DELETE as libc::c_int as libc::c_uint == 0
    {
        g_task_return_boolean(task, (0 as libc::c_int == 0) as libc::c_int);
        return;
    }
    (*priv_0).current_action = FPI_DEVICE_ACTION_DELETE;
    (*priv_0)
        .current_task = (if 0 as libc::c_int != 0 {
        task as *mut libc::c_void
    } else {
        g_steal_pointer(&mut task as *mut GTask_autoptr as gpointer)
    }) as *mut GTask;
    setup_task_cancellable(device);
    g_task_set_task_data(
        (*priv_0).current_task,
        g_object_ref(enrolled_print as gpointer) as *mut FpPrint as gpointer,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    ((*cls).delete).expect("non-null function pointer")(device);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_delete_print_finish(
    mut device: *mut FpDevice,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    return g_task_propagate_boolean(
        g_type_check_instance_cast(result as *mut GTypeInstance, g_task_get_type())
            as *mut libc::c_void as *mut GTask,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_list_prints(
    mut device: *mut FpDevice,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut cls: *mut FpDeviceClass = FP_DEVICE_GET_CLASS(device as gpointer);
    task = g_task_new(device as gpointer, cancellable, callback, user_data);
    if g_task_return_error_if_cancelled(task) != 0 {
        return;
    }
    if (*priv_0).is_open == 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_NOT_OPEN));
        return;
    }
    if !((*priv_0).current_task).is_null() || (*priv_0).is_suspended != 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_BUSY));
        return;
    }
    if ((*cls).list).is_none()
        || (*priv_0).features as libc::c_uint
            & FP_DEVICE_FEATURE_STORAGE as libc::c_int as libc::c_uint == 0
    {
        g_task_return_error(
            task,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_NOT_SUPPORTED,
                b"Device has no storage\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    (*priv_0).current_action = FPI_DEVICE_ACTION_LIST;
    (*priv_0)
        .current_task = (if 0 as libc::c_int != 0 {
        task as *mut libc::c_void
    } else {
        g_steal_pointer(&mut task as *mut GTask_autoptr as gpointer)
    }) as *mut GTask;
    setup_task_cancellable(device);
    ((*cls).list).expect("non-null function pointer")(device);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_list_prints_finish(
    mut device: *mut FpDevice,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GPtrArray {
    return g_task_propagate_pointer(
        g_type_check_instance_cast(result as *mut GTypeInstance, g_task_get_type())
            as *mut libc::c_void as *mut GTask,
        error,
    ) as *mut GPtrArray;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_clear_storage(
    mut device: *mut FpDevice,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    let mut cls: *mut FpDeviceClass = FP_DEVICE_GET_CLASS(device as gpointer);
    task = g_task_new(device as gpointer, cancellable, callback, user_data);
    if g_task_return_error_if_cancelled(task) != 0 {
        return;
    }
    if (*priv_0).is_open == 0 {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_NOT_OPEN));
        return;
    }
    if !((*priv_0).current_task).is_null() {
        g_task_return_error(task, fpi_device_error_new(FP_DEVICE_ERROR_BUSY));
        return;
    }
    if (*priv_0).features as libc::c_uint
        & FP_DEVICE_FEATURE_STORAGE as libc::c_int as libc::c_uint == 0
    {
        g_task_return_error(
            task,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_NOT_SUPPORTED,
                b"Device has no storage.\0" as *const u8 as *const libc::c_char,
            ),
        );
        return;
    }
    if (*priv_0).features as libc::c_uint
        & FP_DEVICE_FEATURE_STORAGE_CLEAR as libc::c_int as libc::c_uint == 0
    {
        g_task_return_error(
            task,
            fpi_device_error_new_msg(
                FP_DEVICE_ERROR_NOT_SUPPORTED,
                b"Device doesn't support clearing storage.\0" as *const u8
                    as *const libc::c_char,
            ),
        );
        return;
    }
    (*priv_0).current_action = FPI_DEVICE_ACTION_CLEAR_STORAGE;
    (*priv_0)
        .current_task = (if 0 as libc::c_int != 0 {
        task as *mut libc::c_void
    } else {
        g_steal_pointer(&mut task as *mut GTask_autoptr as gpointer)
    }) as *mut GTask;
    setup_task_cancellable(device);
    ((*cls).clear_storage).expect("non-null function pointer")(device);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_clear_storage_finish(
    mut device: *mut FpDevice,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    return g_task_propagate_boolean(
        g_type_check_instance_cast(result as *mut GTypeInstance, g_task_get_type())
            as *mut libc::c_void as *mut GTask,
        error,
    );
}
unsafe extern "C" fn async_result_ready(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut *mut GTask = user_data as *mut *mut GTask;
    *task = g_object_ref(
        g_type_check_instance_cast(res as *mut GTypeInstance, g_task_get_type())
            as *mut libc::c_void as *mut GTask as gpointer,
    ) as *mut GTask;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_open_sync(
    mut device: *mut FpDevice,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut task: GAsyncResult_autoptr = 0 as GAsyncResult_autoptr;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fp_device_open_sync\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    fp_device_open(
        device,
        cancellable,
        Some(
            async_result_ready
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        &mut task as *mut GAsyncResult_autoptr as gpointer,
    );
    while task.is_null() {
        g_main_context_iteration(
            0 as *mut GMainContext,
            (0 as libc::c_int == 0) as libc::c_int,
        );
    }
    return fp_device_open_finish(device, task, error);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_close_sync(
    mut device: *mut FpDevice,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut task: GAsyncResult_autoptr = 0 as GAsyncResult_autoptr;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fp_device_close_sync\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    fp_device_close(
        device,
        cancellable,
        Some(
            async_result_ready
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        &mut task as *mut GAsyncResult_autoptr as gpointer,
    );
    while task.is_null() {
        g_main_context_iteration(
            0 as *mut GMainContext,
            (0 as libc::c_int == 0) as libc::c_int,
        );
    }
    return fp_device_close_finish(device, task, error);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_enroll_sync(
    mut device: *mut FpDevice,
    mut template_print: *mut FpPrint,
    mut cancellable: *mut GCancellable,
    mut progress_cb: FpEnrollProgress,
    mut progress_data: gpointer,
    mut error: *mut *mut GError,
) -> *mut FpPrint {
    let mut task: GAsyncResult_autoptr = 0 as GAsyncResult_autoptr;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fp_device_enroll_sync\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FpPrint;
    }
    fp_device_enroll(
        device,
        template_print,
        cancellable,
        progress_cb,
        progress_data,
        None,
        Some(
            async_result_ready
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        &mut task as *mut GAsyncResult_autoptr as gpointer,
    );
    while task.is_null() {
        g_main_context_iteration(
            0 as *mut GMainContext,
            (0 as libc::c_int == 0) as libc::c_int,
        );
    }
    return fp_device_enroll_finish(device, task, error);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_verify_sync(
    mut device: *mut FpDevice,
    mut enrolled_print: *mut FpPrint,
    mut cancellable: *mut GCancellable,
    mut match_cb: FpMatchCb,
    mut match_data: gpointer,
    mut match_0: *mut gboolean,
    mut print: *mut *mut FpPrint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut task: GAsyncResult_autoptr = 0 as GAsyncResult_autoptr;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fp_device_verify_sync\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    fp_device_verify(
        device,
        enrolled_print,
        cancellable,
        match_cb,
        match_data,
        None,
        Some(
            async_result_ready
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        &mut task as *mut GAsyncResult_autoptr as gpointer,
    );
    while task.is_null() {
        g_main_context_iteration(
            0 as *mut GMainContext,
            (0 as libc::c_int == 0) as libc::c_int,
        );
    }
    return fp_device_verify_finish(device, task, match_0, print, error);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_identify_sync(
    mut device: *mut FpDevice,
    mut prints: *mut GPtrArray,
    mut cancellable: *mut GCancellable,
    mut match_cb: FpMatchCb,
    mut match_data: gpointer,
    mut match_0: *mut *mut FpPrint,
    mut print: *mut *mut FpPrint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut task: GAsyncResult_autoptr = 0 as GAsyncResult_autoptr;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"fp_device_identify_sync\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    fp_device_identify(
        device,
        prints,
        cancellable,
        match_cb,
        match_data,
        None,
        Some(
            async_result_ready
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        &mut task as *mut GAsyncResult_autoptr as gpointer,
    );
    while task.is_null() {
        g_main_context_iteration(
            0 as *mut GMainContext,
            (0 as libc::c_int == 0) as libc::c_int,
        );
    }
    return fp_device_identify_finish(device, task, match_0, print, error);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_capture_sync(
    mut device: *mut FpDevice,
    mut wait_for_finger: gboolean,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut FpImage {
    let mut task: GAsyncResult_autoptr = 0 as GAsyncResult_autoptr;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"fp_device_capture_sync\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FpImage;
    }
    fp_device_capture(
        device,
        wait_for_finger,
        cancellable,
        Some(
            async_result_ready
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        &mut task as *mut GAsyncResult_autoptr as gpointer,
    );
    while task.is_null() {
        g_main_context_iteration(
            0 as *mut GMainContext,
            (0 as libc::c_int == 0) as libc::c_int,
        );
    }
    return fp_device_capture_finish(device, task, error);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_delete_print_sync(
    mut device: *mut FpDevice,
    mut enrolled_print: *mut FpPrint,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut task: GAsyncResult_autoptr = 0 as GAsyncResult_autoptr;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fp_device_delete_print_sync\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    fp_device_delete_print(
        device,
        enrolled_print,
        cancellable,
        Some(
            async_result_ready
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        &mut task as *mut GAsyncResult_autoptr as gpointer,
    );
    while task.is_null() {
        g_main_context_iteration(
            0 as *mut GMainContext,
            (0 as libc::c_int == 0) as libc::c_int,
        );
    }
    return fp_device_delete_print_finish(device, task, error);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_list_prints_sync(
    mut device: *mut FpDevice,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GPtrArray {
    let mut task: GAsyncResult_autoptr = 0 as GAsyncResult_autoptr;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fp_device_list_prints_sync\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut GPtrArray;
    }
    fp_device_list_prints(
        device,
        0 as *mut GCancellable,
        Some(
            async_result_ready
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        &mut task as *mut GAsyncResult_autoptr as gpointer,
    );
    while task.is_null() {
        g_main_context_iteration(
            0 as *mut GMainContext,
            (0 as libc::c_int == 0) as libc::c_int,
        );
    }
    return fp_device_list_prints_finish(device, task, error);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_clear_storage_sync(
    mut device: *mut FpDevice,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut task: GAsyncResult_autoptr = 0 as GAsyncResult_autoptr;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"fp_device_clear_storage_sync\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    fp_device_clear_storage(
        device,
        cancellable,
        Some(
            async_result_ready
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        &mut task as *mut GAsyncResult_autoptr as gpointer,
    );
    while task.is_null() {
        g_main_context_iteration(
            0 as *mut GMainContext,
            (0 as libc::c_int == 0) as libc::c_int,
        );
    }
    return fp_device_clear_storage_finish(device, task, error);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_suspend_sync(
    mut device: *mut FpDevice,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut task: GAsyncResult_autoptr = 0 as GAsyncResult_autoptr;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"fp_device_suspend_sync\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    fp_device_suspend(
        device,
        cancellable,
        Some(
            async_result_ready
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        &mut task as *mut GAsyncResult_autoptr as gpointer,
    );
    while task.is_null() {
        g_main_context_iteration(
            0 as *mut GMainContext,
            (0 as libc::c_int == 0) as libc::c_int,
        );
    }
    return fp_device_suspend_finish(device, task, error);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_resume_sync(
    mut device: *mut FpDevice,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut task: GAsyncResult_autoptr = 0 as GAsyncResult_autoptr;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fp_device_resume_sync\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    fp_device_resume(
        device,
        cancellable,
        Some(
            async_result_ready
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        &mut task as *mut GAsyncResult_autoptr as gpointer,
    );
    while task.is_null() {
        g_main_context_iteration(
            0 as *mut GMainContext,
            (0 as libc::c_int == 0) as libc::c_int,
        );
    }
    return fp_device_resume_finish(device, task, error);
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_get_features(
    mut device: *mut FpDevice,
) -> FpDeviceFeature {
    let mut priv_0: *mut FpDevicePrivate = fp_device_get_instance_private(device)
        as *mut FpDevicePrivate;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"fp_device_get_features\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return FP_DEVICE_FEATURE_NONE;
    }
    return (*priv_0).features;
}
#[no_mangle]
pub unsafe extern "C" fn fp_device_has_feature(
    mut device: *mut FpDevice,
    mut feature: FpDeviceFeature,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_DEVICE(device as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-device\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fp_device_has_feature\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if feature as libc::c_uint == FP_DEVICE_FEATURE_NONE as libc::c_int as libc::c_uint {
        return (fp_device_get_features(device) as libc::c_uint
            == feature as libc::c_uint) as libc::c_int;
    }
    return (fp_device_get_features(device) as libc::c_uint & feature as libc::c_uint
        == feature as libc::c_uint) as libc::c_int;
}
