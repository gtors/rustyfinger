use ::libc;
extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    fn g_error_free(error: *mut GError);
    fn g_error_copy(error: *const GError) -> *mut GError;
    fn g_nullify_pointer(nullify_location: *mut gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_source_destroy(source: *mut GSource);
    fn g_source_set_name(source: *mut GSource, name: *const libc::c_char);
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
    fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, _: ...) -> *mut gchar;
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
    fn fp_device_get_driver(device: *mut FpDevice) -> *const gchar;
    fn fp_device_get_device_id(device: *mut FpDevice) -> *const gchar;
    fn fpi_device_add_timeout(
        device: *mut FpDevice,
        interval: gint,
        func: FpTimeoutFunc,
        user_data: gpointer,
        destroy_notify: GDestroyNotify,
    ) -> *mut GSource;
}
pub type guint8 = libc::c_uchar;
pub type guint16 = libc::c_ushort;
pub type guint32 = libc::c_uint;
pub type gssize = libc::c_long;
pub type gsize = libc::c_ulong;
pub type gchar = libc::c_char;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type guchar = libc::c_uchar;
pub type guint = libc::c_uint;
pub type gpointer = *mut libc::c_void;
pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpDevice {
    pub parent_instance: GObject,
}
pub type FpDevice = _FpDevice;
pub type FpTimeoutFunc = Option::<unsafe extern "C" fn(*mut FpDevice, gpointer) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiSsm {
    pub dev: *mut FpDevice,
    pub name: *const libc::c_char,
    pub parentsm: *mut FpiSsm,
    pub ssm_data: gpointer,
    pub ssm_data_destroy: GDestroyNotify,
    pub nr_states: libc::c_int,
    pub start_cleanup: libc::c_int,
    pub cur_state: libc::c_int,
    pub completed: gboolean,
    pub silence: gboolean,
    pub timeout: *mut GSource,
    pub error: *mut GError,
    pub callback: FpiSsmCompletedCallback,
    pub handler: FpiSsmHandlerCallback,
}
pub type FpiSsmHandlerCallback = Option::<
    unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
>;
pub type FpiSsmCompletedCallback = Option::<
    unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpiSpiTransfer {
    pub device: *mut FpDevice,
    pub ssm: *mut FpiSsm,
    pub length_wr: gssize,
    pub length_rd: gssize,
    pub buffer_wr: *mut guchar,
    pub buffer_rd: *mut guchar,
    pub ref_count: guint,
    pub spidev_fd: libc::c_int,
    pub user_data: gpointer,
    pub callback: FpiSpiTransferCallback,
    pub free_buffer_wr: GDestroyNotify,
    pub free_buffer_rd: GDestroyNotify,
}
pub type FpiSpiTransferCallback = Option::<
    unsafe extern "C" fn(*mut FpiSpiTransfer, *mut FpDevice, gpointer, *mut GError) -> (),
>;
pub type FpiSpiTransfer = _FpiSpiTransfer;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiSsmJumpToStateDelayedData {
    pub machine: *mut FpiSsm,
    pub next_state: libc::c_int,
}
#[inline]
unsafe extern "C" fn g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = 0 as *mut libc::c_void;
    ref_0 = *ptr;
    *ptr = 0 as *mut libc::c_void;
    return ref_0;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_new_full(
    mut dev: *mut FpDevice,
    mut handler: FpiSsmHandlerCallback,
    mut nr_states: libc::c_int,
    mut start_cleanup: libc::c_int,
    mut machine_name: *const libc::c_char,
) -> *mut FpiSsm {
    let mut machine: *mut FpiSsm = 0 as *mut FpiSsm;
    if dev.is_null() {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"dev == NULL\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"fpi_ssm_new_full\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            128 as libc::c_int,
        );
        g_free(s as gpointer);
    }
    if nr_states < 1 as libc::c_int {
        let mut s_0: *mut libc::c_char = 0 as *mut libc::c_char;
        s_0 = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"nr_states < 1\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s_0,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"fpi_ssm_new_full\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            129 as libc::c_int,
        );
        g_free(s_0 as gpointer);
    }
    if start_cleanup < 1 as libc::c_int {
        let mut s_1: *mut libc::c_char = 0 as *mut libc::c_char;
        s_1 = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"start_cleanup < 1\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s_1,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"fpi_ssm_new_full\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            130 as libc::c_int,
        );
        g_free(s_1 as gpointer);
    }
    if start_cleanup > nr_states {
        let mut s_2: *mut libc::c_char = 0 as *mut libc::c_char;
        s_2 = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"start_cleanup > nr_states\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s_2,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"fpi_ssm_new_full\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
        );
        g_free(s_2 as gpointer);
    }
    if handler.is_none() {
        let mut s_3: *mut libc::c_char = 0 as *mut libc::c_char;
        s_3 = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"handler == NULL\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s_3,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"fpi_ssm_new_full\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
        );
        g_free(s_3 as gpointer);
    }
    machine = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<FpiSsm>() as libc::c_ulong;
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
    }) as *mut FpiSsm;
    (*machine).handler = handler;
    (*machine).nr_states = nr_states;
    (*machine).start_cleanup = start_cleanup;
    (*machine).dev = dev;
    (*machine).name = g_strdup(machine_name);
    (*machine).completed = (0 as libc::c_int == 0) as libc::c_int;
    return machine;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_set_data(
    mut machine: *mut FpiSsm,
    mut ssm_data: gpointer,
    mut ssm_data_destroy: GDestroyNotify,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !machine.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"fpi_ssm_set_data\0"))
                .as_ptr(),
            b"machine\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ((*machine).ssm_data_destroy).is_some() && !((*machine).ssm_data).is_null() {
        ((*machine).ssm_data_destroy)
            .expect("non-null function pointer")((*machine).ssm_data);
    }
    (*machine).ssm_data = ssm_data;
    (*machine).ssm_data_destroy = ssm_data_destroy;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_get_data(mut machine: *mut FpiSsm) -> gpointer {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !machine.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"fpi_ssm_get_data\0"))
                .as_ptr(),
            b"machine\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_void;
    }
    return (*machine).ssm_data;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_get_device(mut machine: *mut FpiSsm) -> *mut FpDevice {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !machine.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"fpi_ssm_get_device\0"))
                .as_ptr(),
            b"machine\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FpDevice;
    }
    return (*machine).dev;
}
unsafe extern "C" fn fpi_ssm_clear_delayed_action(mut machine: *mut FpiSsm) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !machine.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"fpi_ssm_clear_delayed_action\0"))
                .as_ptr(),
            b"machine\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut _pp: C2RustUnnamed = C2RustUnnamed {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GSource) -> ()>,
        GDestroyNotify,
    >(Some(g_source_destroy as unsafe extern "C" fn(*mut GSource) -> ()));
    _pp.in_0 = &mut (*machine).timeout as *mut *mut GSource as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
}
unsafe extern "C" fn fpi_ssm_set_delayed_action_timeout(
    mut machine: *mut FpiSsm,
    mut delay: libc::c_int,
    mut callback: FpTimeoutFunc,
    mut user_data: gpointer,
    mut destroy_func: GDestroyNotify,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !machine.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"fpi_ssm_set_delayed_action_timeout\0"))
                .as_ptr(),
            b"machine\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*machine).completed != 0 {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"machine->completed\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"fpi_ssm_set_delayed_action_timeout\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_int,
        );
        g_free(s as gpointer);
    }
    if !((*machine).timeout).is_null() {
        let mut s_0: *mut libc::c_char = 0 as *mut libc::c_char;
        s_0 = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"machine->timeout != NULL\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s_0,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"fpi_ssm_set_delayed_action_timeout\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int,
        );
        g_free(s_0 as gpointer);
    }
    fpi_ssm_clear_delayed_action(machine);
    (*machine)
        .timeout = fpi_device_add_timeout(
        (*machine).dev,
        delay,
        callback,
        user_data,
        destroy_func,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_free(mut machine: *mut FpiSsm) {
    if machine.is_null() {
        return;
    }
    if !((*machine).timeout).is_null() {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"machine->timeout != NULL\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"fpi_ssm_free\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            237 as libc::c_int,
        );
        g_free(s as gpointer);
    }
    if ((*machine).ssm_data_destroy).is_some() {
        let mut _pp: C2RustUnnamed_2 = C2RustUnnamed_2 {
            in_0: 0 as *mut libc::c_char,
        };
        let mut _p: gpointer = 0 as *mut libc::c_void;
        let mut _destroy: GDestroyNotify = (*machine).ssm_data_destroy;
        _pp.in_0 = &mut (*machine).ssm_data as *mut gpointer as *mut libc::c_char;
        _p = *_pp.out;
        if !_p.is_null() {
            *_pp.out = 0 as *mut libc::c_void;
            _destroy.expect("non-null function pointer")(_p);
        }
    }
    let mut _pp_0: C2RustUnnamed_1 = C2RustUnnamed_1 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_0: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GError) -> ()>,
        GDestroyNotify,
    >(Some(g_error_free as unsafe extern "C" fn(*mut GError) -> ()));
    _pp_0.in_0 = &mut (*machine).error as *mut *mut GError as *mut libc::c_char;
    _p_0 = *_pp_0.out;
    if !_p_0.is_null() {
        *_pp_0.out = 0 as *mut libc::c_void;
        _destroy_0.expect("non-null function pointer")(_p_0);
    }
    let mut _pp_1: C2RustUnnamed_0 = C2RustUnnamed_0 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_1: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_1: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp_1.in_0 = &mut (*machine).name as *mut *const libc::c_char as *mut libc::c_char;
    _p_1 = *_pp_1.out;
    if !_p_1.is_null() {
        *_pp_1.out = 0 as *mut libc::c_void;
        _destroy_1.expect("non-null function pointer")(_p_1);
    }
    fpi_ssm_clear_delayed_action(machine);
    g_free(machine as gpointer);
}
unsafe extern "C" fn __ssm_call_handler(
    mut machine: *mut FpiSsm,
    mut force_msg: gboolean,
) {
    if force_msg != 0 || (*machine).silence == 0 {
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"[%s] %s entering state %d\0" as *const u8 as *const libc::c_char,
            fp_device_get_driver((*machine).dev),
            (*machine).name,
            (*machine).cur_state,
        );
    }
    ((*machine).handler).expect("non-null function pointer")(machine, (*machine).dev);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_start(
    mut ssm: *mut FpiSsm,
    mut callback: FpiSsmCompletedCallback,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !ssm.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"fpi_ssm_start\0"))
                .as_ptr(),
            b"ssm != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*ssm).completed == 0 {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"!ssm->completed\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"fpi_ssm_start\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            275 as libc::c_int,
        );
        g_free(s as gpointer);
    }
    (*ssm).callback = callback;
    (*ssm).cur_state = 0 as libc::c_int;
    (*ssm).completed = 0 as libc::c_int;
    (*ssm).error = 0 as *mut GError;
    __ssm_call_handler(ssm, (0 as libc::c_int == 0) as libc::c_int);
}
unsafe extern "C" fn __subsm_complete(
    mut ssm: *mut FpiSsm,
    mut _dev: *mut FpDevice,
    mut error: *mut GError,
) {
    let mut parent: *mut FpiSsm = (*ssm).parentsm;
    if parent.is_null() {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"!parent\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"__subsm_complete\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            288 as libc::c_int,
        );
        g_free(s as gpointer);
    }
    if !error.is_null() {
        fpi_ssm_mark_failed(parent, error);
    } else {
        fpi_ssm_next_state(parent);
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_start_subsm(
    mut parent: *mut FpiSsm,
    mut child: *mut FpiSsm,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !parent.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fpi_ssm_start_subsm\0"))
                .as_ptr(),
            b"parent != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !child.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fpi_ssm_start_subsm\0"))
                .as_ptr(),
            b"child != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if !((*parent).timeout).is_null() {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"parent->timeout\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fpi_ssm_start_subsm\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
        );
        g_free(s as gpointer);
    }
    (*child).parentsm = parent;
    fpi_ssm_clear_delayed_action(parent);
    fpi_ssm_clear_delayed_action(child);
    fpi_ssm_start(
        child,
        Some(
            __subsm_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_mark_completed(mut machine: *mut FpiSsm) {
    let mut next_state: libc::c_int = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !machine.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"fpi_ssm_mark_completed\0"))
                .as_ptr(),
            b"machine != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*machine).completed != 0 {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"machine->completed\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"fpi_ssm_mark_completed\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            337 as libc::c_int,
        );
        g_free(s as gpointer);
    }
    if !((*machine).timeout).is_null() {
        let mut s_0: *mut libc::c_char = 0 as *mut libc::c_char;
        s_0 = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"machine->timeout != NULL\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s_0,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"fpi_ssm_mark_completed\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            338 as libc::c_int,
        );
        g_free(s_0 as gpointer);
    }
    fpi_ssm_clear_delayed_action(machine);
    if (*machine).cur_state < (*machine).start_cleanup {
        next_state = (*machine).start_cleanup;
    } else {
        next_state = (*machine).cur_state + 1 as libc::c_int;
    }
    if next_state < (*machine).nr_states {
        (*machine).cur_state = next_state;
        __ssm_call_handler(machine, (0 as libc::c_int == 0) as libc::c_int);
        return;
    }
    (*machine).completed = (0 as libc::c_int == 0) as libc::c_int;
    if !((*machine).error).is_null() {
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"[%s] %s completed with error: %s\0" as *const u8 as *const libc::c_char,
            fp_device_get_driver((*machine).dev),
            (*machine).name,
            (*(*machine).error).message,
        );
    } else {
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_DEBUG,
            b"[%s] %s completed successfully\0" as *const u8 as *const libc::c_char,
            fp_device_get_driver((*machine).dev),
            (*machine).name,
        );
    }
    if ((*machine).callback).is_some() {
        let mut error: *mut GError = if !((*machine).error).is_null() {
            g_error_copy((*machine).error)
        } else {
            0 as *mut GError
        };
        ((*machine).callback)
            .expect("non-null function pointer")(machine, (*machine).dev, error);
    }
    fpi_ssm_free(machine);
}
unsafe extern "C" fn on_device_timeout_complete(
    mut dev: *mut FpDevice,
    mut user_data: gpointer,
) {
    let mut machine: *mut FpiSsm = user_data as *mut FpiSsm;
    (*machine).timeout = 0 as *mut GSource;
    fpi_ssm_mark_completed(machine);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_mark_completed_delayed(
    mut machine: *mut FpiSsm,
    mut delay: libc::c_int,
) {
    let mut source_name: *mut libc::c_char = 0 as *mut libc::c_char;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !machine.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"fpi_ssm_mark_completed_delayed\0"))
                .as_ptr(),
            b"machine != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    fpi_ssm_set_delayed_action_timeout(
        machine,
        delay,
        Some(
            on_device_timeout_complete
                as unsafe extern "C" fn(*mut FpDevice, gpointer) -> (),
        ),
        machine as gpointer,
        None,
    );
    source_name = g_strdup_printf(
        b"[%s] ssm %s complete %d\0" as *const u8 as *const libc::c_char,
        fp_device_get_device_id((*machine).dev),
        (*machine).name,
        (*machine).cur_state + 1 as libc::c_int,
    );
    g_source_set_name((*machine).timeout, source_name);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_mark_failed(
    mut machine: *mut FpiSsm,
    mut error: *mut GError,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !machine.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fpi_ssm_mark_failed\0"))
                .as_ptr(),
            b"machine != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !error.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            420 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fpi_ssm_mark_failed\0"))
                .as_ptr(),
            b"error\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((*machine).error).is_null() && (*machine).cur_state < (*machine).start_cleanup {
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_WARNING,
            b"[%s] SSM %s already has an error set, ignoring new error %s\0" as *const u8
                as *const libc::c_char,
            fp_device_get_driver((*machine).dev),
            (*machine).name,
            (*error).message,
        );
        g_error_free(error);
        return;
    }
    g_log(
        b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"[%s] SSM %s failed in state %d%s with error: %s\0" as *const u8
            as *const libc::c_char,
        fp_device_get_driver((*machine).dev),
        (*machine).name,
        (*machine).cur_state,
        if (*machine).cur_state >= (*machine).start_cleanup {
            b" (cleanup)\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        (*error).message,
    );
    if ((*machine).error).is_null() {
        (*machine)
            .error = (if 0 as libc::c_int != 0 {
            error as *mut libc::c_void
        } else {
            g_steal_pointer(&mut error as *mut *mut GError as gpointer)
        }) as *mut GError;
    } else {
        g_error_free(error);
    }
    fpi_ssm_mark_completed(machine);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_next_state(mut machine: *mut FpiSsm) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !machine.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"fpi_ssm_next_state\0"))
                .as_ptr(),
            b"machine != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*machine).completed != 0 {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"machine->completed\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"fpi_ssm_next_state\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            456 as libc::c_int,
        );
        g_free(s as gpointer);
    }
    if !((*machine).timeout).is_null() {
        let mut s_0: *mut libc::c_char = 0 as *mut libc::c_char;
        s_0 = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"machine->timeout != NULL\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s_0,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"fpi_ssm_next_state\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            457 as libc::c_int,
        );
        g_free(s_0 as gpointer);
    }
    fpi_ssm_clear_delayed_action(machine);
    (*machine).cur_state += 1;
    if (*machine).cur_state == (*machine).nr_states {
        fpi_ssm_mark_completed(machine);
    } else {
        __ssm_call_handler(machine, 0 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_cancel_delayed_state_change(mut machine: *mut FpiSsm) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !machine.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"fpi_ssm_cancel_delayed_state_change\0"))
                .as_ptr(),
            b"machine\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*machine).completed != 0 {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"machine->completed\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"fpi_ssm_cancel_delayed_state_change\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            472 as libc::c_int,
        );
        g_free(s as gpointer);
    }
    if ((*machine).timeout).is_null() {
        let mut s_0: *mut libc::c_char = 0 as *mut libc::c_char;
        s_0 = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"machine->timeout == NULL\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s_0,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"fpi_ssm_cancel_delayed_state_change\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            473 as libc::c_int,
        );
        g_free(s_0 as gpointer);
    }
    g_log(
        b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"[%s] %s cancelled delayed state change\0" as *const u8 as *const libc::c_char,
        fp_device_get_driver((*machine).dev),
        (*machine).name,
    );
    fpi_ssm_clear_delayed_action(machine);
}
unsafe extern "C" fn on_device_timeout_next_state(
    mut dev: *mut FpDevice,
    mut user_data: gpointer,
) {
    let mut machine: *mut FpiSsm = user_data as *mut FpiSsm;
    (*machine).timeout = 0 as *mut GSource;
    fpi_ssm_next_state(machine);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_next_state_delayed(
    mut machine: *mut FpiSsm,
    mut delay: libc::c_int,
) {
    let mut source_name: *mut libc::c_char = 0 as *mut libc::c_char;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !machine.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_ssm_next_state_delayed\0"))
                .as_ptr(),
            b"machine != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    fpi_ssm_set_delayed_action_timeout(
        machine,
        delay,
        Some(
            on_device_timeout_next_state
                as unsafe extern "C" fn(*mut FpDevice, gpointer) -> (),
        ),
        machine as gpointer,
        None,
    );
    source_name = g_strdup_printf(
        b"[%s] ssm %s jump to next state %d\0" as *const u8 as *const libc::c_char,
        fp_device_get_device_id((*machine).dev),
        (*machine).name,
        (*machine).cur_state + 1 as libc::c_int,
    );
    g_source_set_name((*machine).timeout, source_name);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_jump_to_state(
    mut machine: *mut FpiSsm,
    mut state: libc::c_int,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !machine.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fpi_ssm_jump_to_state\0"))
                .as_ptr(),
            b"machine != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*machine).completed != 0 {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"machine->completed\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fpi_ssm_jump_to_state\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            532 as libc::c_int,
        );
        g_free(s as gpointer);
    }
    if state < 0 as libc::c_int || state > (*machine).nr_states {
        let mut s_0: *mut libc::c_char = 0 as *mut libc::c_char;
        s_0 = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"state < 0 || state > machine->nr_states\0" as *const u8
                as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s_0,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fpi_ssm_jump_to_state\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            533 as libc::c_int,
        );
        g_free(s_0 as gpointer);
    }
    if !((*machine).timeout).is_null() {
        let mut s_1: *mut libc::c_char = 0 as *mut libc::c_char;
        s_1 = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"machine->timeout != NULL\0" as *const u8 as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s_1,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fpi_ssm_jump_to_state\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            534 as libc::c_int,
        );
        g_free(s_1 as gpointer);
    }
    fpi_ssm_clear_delayed_action(machine);
    (*machine).cur_state = state;
    if (*machine).cur_state == (*machine).nr_states {
        fpi_ssm_mark_completed(machine);
    } else {
        __ssm_call_handler(machine, 0 as libc::c_int);
    };
}
unsafe extern "C" fn on_device_timeout_jump_to_state(
    mut dev: *mut FpDevice,
    mut user_data: gpointer,
) {
    let mut data: *mut FpiSsmJumpToStateDelayedData = user_data
        as *mut FpiSsmJumpToStateDelayedData;
    (*(*data).machine).timeout = 0 as *mut GSource;
    fpi_ssm_jump_to_state((*data).machine, (*data).next_state);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_jump_to_state_delayed(
    mut machine: *mut FpiSsm,
    mut state: libc::c_int,
    mut delay: libc::c_int,
) {
    let mut data: *mut FpiSsmJumpToStateDelayedData = 0
        as *mut FpiSsmJumpToStateDelayedData;
    let mut source_name: *mut libc::c_char = 0 as *mut libc::c_char;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !machine.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"fpi_ssm_jump_to_state_delayed\0"))
                .as_ptr(),
            b"machine != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if state < 0 as libc::c_int || state > (*machine).nr_states {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = g_strconcat(
            b"BUG: (\0" as *const u8 as *const libc::c_char,
            b"state < 0 || state > machine->nr_states\0" as *const u8
                as *const libc::c_char,
            b")\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        g_log(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s() %s:%d\0" as *const u8 as *const libc::c_char,
            s,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"fpi_ssm_jump_to_state_delayed\0"))
                .as_ptr(),
            b"../libfprint/fpi-ssm.c\0" as *const u8 as *const libc::c_char,
            579 as libc::c_int,
        );
        g_free(s as gpointer);
    }
    data = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<FpiSsmJumpToStateDelayedData>()
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
    }) as *mut FpiSsmJumpToStateDelayedData;
    (*data).machine = machine;
    (*data).next_state = state;
    fpi_ssm_set_delayed_action_timeout(
        machine,
        delay,
        Some(
            on_device_timeout_jump_to_state
                as unsafe extern "C" fn(*mut FpDevice, gpointer) -> (),
        ),
        data as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    source_name = g_strdup_printf(
        b"[%s] ssm %s jump to state %d\0" as *const u8 as *const libc::c_char,
        fp_device_get_device_id((*machine).dev),
        (*machine).name,
        state,
    );
    g_source_set_name((*machine).timeout, source_name);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_get_cur_state(mut machine: *mut FpiSsm) -> libc::c_int {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !machine.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fpi_ssm_get_cur_state\0"))
                .as_ptr(),
            b"machine != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    return (*machine).cur_state;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_get_error(mut machine: *mut FpiSsm) -> *mut GError {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !machine.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"fpi_ssm_get_error\0"))
                .as_ptr(),
            b"machine != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut GError;
    }
    return (*machine).error;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_dup_error(mut machine: *mut FpiSsm) -> *mut GError {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !machine.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"fpi_ssm_dup_error\0"))
                .as_ptr(),
            b"machine != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut GError;
    }
    if !((*machine).error).is_null() {
        return g_error_copy((*machine).error);
    }
    return 0 as *mut GError;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_silence_debug(mut machine: *mut FpiSsm) {
    (*machine).silence = (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_usb_transfer_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut unused_data: gpointer,
    mut error: *mut GError,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !((*transfer).ssm).is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"fpi_ssm_usb_transfer_cb\0"))
                .as_ptr(),
            b"transfer->ssm\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
    } else {
        fpi_ssm_next_state((*transfer).ssm);
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_usb_transfer_with_weak_pointer_cb(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut weak_ptr: gpointer,
    mut error: *mut GError,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !((*transfer).ssm).is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"fpi_ssm_usb_transfer_with_weak_pointer_cb\0"))
                .as_ptr(),
            b"transfer->ssm\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if !weak_ptr.is_null() {
        g_nullify_pointer(weak_ptr as *mut gpointer);
    }
    fpi_ssm_usb_transfer_cb(transfer, device, weak_ptr, error);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_spi_transfer_cb(
    mut transfer: *mut FpiSpiTransfer,
    mut device: *mut FpDevice,
    mut unused_data: gpointer,
    mut error: *mut GError,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !((*transfer).ssm).is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"fpi_ssm_spi_transfer_cb\0"))
                .as_ptr(),
            b"transfer->ssm\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if !error.is_null() {
        fpi_ssm_mark_failed((*transfer).ssm, error);
    } else {
        fpi_ssm_next_state((*transfer).ssm);
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_ssm_spi_transfer_with_weak_pointer_cb(
    mut transfer: *mut FpiSpiTransfer,
    mut device: *mut FpDevice,
    mut weak_ptr: gpointer,
    mut error: *mut GError,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !((*transfer).ssm).is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-SSM\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"fpi_ssm_spi_transfer_with_weak_pointer_cb\0"))
                .as_ptr(),
            b"transfer->ssm\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if !weak_ptr.is_null() {
        g_nullify_pointer(weak_ptr as *mut gpointer);
    }
    fpi_ssm_spi_transfer_cb(transfer, device, weak_ptr, error);
}
