use ::libc;
use ::f128;
extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _FpiSsm;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_new(
        domain: GQuark,
        code: gint,
        format: *const gchar,
        _: ...
    ) -> *mut GError;
    fn g_error_new_literal(
        domain: GQuark,
        code: gint,
        message: *const gchar,
    ) -> *mut GError;
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_set_size(string: *mut GString, len: gsize) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, _: ...);
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
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
    fn g_assertion_message_cmpnum(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
        arg1: f128::f128,
        cmp: *const libc::c_char,
        arg2: f128::f128,
        numtype: libc::c_char,
    );
    fn g_type_check_instance_cast(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> *mut GTypeInstance;
    fn g_boxed_type_register_static(
        name: *const gchar,
        boxed_copy: GBoxedCopyFunc,
        boxed_free: GBoxedFreeFunc,
    ) -> GType;
    fn g_cancellable_is_cancelled(cancellable: *mut GCancellable) -> gboolean;
    fn g_io_error_quark() -> GQuark;
    fn g_usb_device_get_type() -> GType;
    fn g_usb_device_error_quark() -> GQuark;
    fn g_usb_device_control_transfer(
        self_0: *mut GUsbDevice,
        direction: GUsbDeviceDirection,
        request_type: GUsbDeviceRequestType,
        recipient: GUsbDeviceRecipient,
        request: guint8,
        value: guint16,
        idx: guint16,
        data: *mut guint8,
        length: gsize,
        actual_length: *mut gsize,
        timeout: guint,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_usb_device_bulk_transfer(
        self_0: *mut GUsbDevice,
        endpoint: guint8,
        data: *mut guint8,
        length: gsize,
        actual_length: *mut gsize,
        timeout: guint,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_usb_device_interrupt_transfer(
        self_0: *mut GUsbDevice,
        endpoint: guint8,
        data: *mut guint8,
        length: gsize,
        actual_length: *mut gsize,
        timeout: guint,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_usb_device_control_transfer_async(
        self_0: *mut GUsbDevice,
        direction: GUsbDeviceDirection,
        request_type: GUsbDeviceRequestType,
        recipient: GUsbDeviceRecipient,
        request: guint8,
        value: guint16,
        idx: guint16,
        data: *mut guint8,
        length: gsize,
        timeout: guint,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_usb_device_control_transfer_finish(
        self_0: *mut GUsbDevice,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_usb_device_bulk_transfer_async(
        self_0: *mut GUsbDevice,
        endpoint: guint8,
        data: *mut guint8,
        length: gsize,
        timeout: guint,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_usb_device_bulk_transfer_finish(
        self_0: *mut GUsbDevice,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_usb_device_interrupt_transfer_async(
        self_0: *mut GUsbDevice,
        endpoint: guint8,
        data: *mut guint8,
        length: gsize,
        timeout: guint,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_usb_device_interrupt_transfer_finish(
        self_0: *mut GUsbDevice,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gssize;
    fn fpi_device_get_usb_device(device: *mut FpDevice) -> *mut GUsbDevice;
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
pub type gint64 = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
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
pub type GString_autoptr = *mut GString;
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
pub type GBoxedCopyFunc = Option::<unsafe extern "C" fn(gpointer) -> gpointer>;
pub type GBoxedFreeFunc = Option::<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
pub type C2RustUnnamed = libc::c_uint;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed = 0;
pub type GAsyncResult = _GAsyncResult;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const G_USB_DEVICE_ERROR_LAST: C2RustUnnamed_0 = 10;
pub const G_USB_DEVICE_ERROR_PERMISSION_DENIED: C2RustUnnamed_0 = 9;
pub const G_USB_DEVICE_ERROR_FAILED: C2RustUnnamed_0 = 8;
pub const G_USB_DEVICE_ERROR_CANCELLED: C2RustUnnamed_0 = 7;
pub const G_USB_DEVICE_ERROR_ALREADY_OPEN: C2RustUnnamed_0 = 6;
pub const G_USB_DEVICE_ERROR_NOT_OPEN: C2RustUnnamed_0 = 5;
pub const G_USB_DEVICE_ERROR_NO_DEVICE: C2RustUnnamed_0 = 4;
pub const G_USB_DEVICE_ERROR_NOT_SUPPORTED: C2RustUnnamed_0 = 3;
pub const G_USB_DEVICE_ERROR_TIMED_OUT: C2RustUnnamed_0 = 2;
pub const G_USB_DEVICE_ERROR_IO: C2RustUnnamed_0 = 1;
pub const G_USB_DEVICE_ERROR_INTERNAL: C2RustUnnamed_0 = 0;
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
pub union C2RustUnnamed_1 {
    pub do_free_type: Option::<unsafe extern "C" fn(*mut FpiUsbTransfer) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub do_copy_type: Option::<
        unsafe extern "C" fn(*mut FpiUsbTransfer) -> *mut FpiUsbTransfer,
    >,
    pub do_const_copy_type: Option::<
        unsafe extern "C" fn(*const FpiUsbTransfer) -> *mut FpiUsbTransfer,
    >,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[inline]
unsafe extern "C" fn G_USB_DEVICE(mut ptr: gpointer) -> *mut GUsbDevice {
    return g_type_check_instance_cast(ptr as *mut GTypeInstance, g_usb_device_get_type())
        as *mut libc::c_void as *mut GUsbDevice;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_usb_transfer_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_usb_transfer_get_type_once();
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
unsafe extern "C" fn fpi_usb_transfer_get_type_once() -> GType {
    let mut _g_register_boxed: Option::<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_2, C2RustUnnamed_1) -> GType,
    > = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
        >,
        Option::<
            unsafe extern "C" fn(*const gchar, C2RustUnnamed_2, C2RustUnnamed_1) -> GType,
        >,
    >(
        Some(
            g_boxed_type_register_static
                as unsafe extern "C" fn(
                    *const gchar,
                    GBoxedCopyFunc,
                    GBoxedFreeFunc,
                ) -> GType,
        ),
    );
    let mut g_define_type_id: GType = _g_register_boxed
        .expect(
            "non-null function pointer",
        )(
        g_intern_static_string(b"FpiUsbTransfer\0" as *const u8 as *const libc::c_char),
        C2RustUnnamed_2 {
            do_copy_type: Some(
                fpi_usb_transfer_ref
                    as unsafe extern "C" fn(*mut FpiUsbTransfer) -> *mut FpiUsbTransfer,
            ),
        },
        C2RustUnnamed_1 {
            do_free_type: Some(
                fpi_usb_transfer_unref as unsafe extern "C" fn(*mut FpiUsbTransfer) -> (),
            ),
        },
    );
    return g_define_type_id;
}
unsafe extern "C" fn log_transfer(
    mut transfer: *mut FpiUsbTransfer,
    mut submit: gboolean,
    mut error: *mut GError,
) {
    if !(g_getenv(b"FP_DEBUG_TRANSFER\0" as *const u8 as *const libc::c_char)).is_null()
    {
        if submit == 0 {
            let mut error_str: *mut gchar = 0 as *mut gchar;
            if !error.is_null() {
                error_str = g_strdup_printf(
                    b"with error (%s)\0" as *const u8 as *const libc::c_char,
                    (*error).message,
                );
            } else {
                error_str = g_strdup(
                    b"successfully\0" as *const u8 as *const libc::c_char,
                );
            }
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Transfer %p completed %s, requested length %zd, actual length %zd, endpoint 0x%x\0"
                    as *const u8 as *const libc::c_char,
                transfer,
                error_str,
                (*transfer).length,
                (*transfer).actual_length,
                (*transfer).endpoint as libc::c_int,
            );
        } else {
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Transfer %p submitted, requested length %zd, endpoint 0x%x\0"
                    as *const u8 as *const libc::c_char,
                transfer,
                (*transfer).length,
                (*transfer).endpoint as libc::c_int,
            );
        }
        if (submit == 0) as libc::c_int
            == ((*transfer).endpoint as libc::c_int & 0x80 as libc::c_int != 0)
                as libc::c_int
        {
            let mut line: GString_autoptr = 0 as GString_autoptr;
            let mut dump_len: gssize = 0;
            dump_len = if (*transfer).endpoint as libc::c_int & 0x80 as libc::c_int != 0
            {
                (*transfer).actual_length
            } else {
                (*transfer).length
            };
            line = g_string_new(b"\0" as *const u8 as *const libc::c_char);
            let mut i: gint = 0 as libc::c_int;
            while (i as libc::c_long) < dump_len {
                g_string_append_printf(
                    line,
                    b"%02x \0" as *const u8 as *const libc::c_char,
                    *((*transfer).buffer).offset(i as isize) as libc::c_int,
                );
                if (i + 1 as libc::c_int) % 16 as libc::c_int == 0 as libc::c_int {
                    g_log(
                        b"libfprint\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        (*line).str_0,
                    );
                    g_string_set_size(line, 0 as libc::c_int as gsize);
                }
                i += 1;
            }
            if (*line).len != 0 {
                g_log(
                    b"libfprint\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_DEBUG,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    (*line).str_0,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn fpi_usb_transfer_new(
    mut device: *mut FpDevice,
) -> *mut FpiUsbTransfer {
    let mut self_0: *mut FpiUsbTransfer = 0 as *mut FpiUsbTransfer;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !device.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-usb-transfer.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fpi_usb_transfer_new\0"))
                .as_ptr(),
            b"device != NULL\0" as *const u8 as *const libc::c_char,
        );
    }
    self_0 = ({
        let mut __s: gsize = ::core::mem::size_of::<FpiUsbTransfer>() as libc::c_ulong;
        let mut __p: gpointer = 0 as *mut libc::c_void;
        __p = g_slice_alloc(__s);
        memset(__p, 0 as libc::c_int, __s);
        __p
    }) as *mut FpiUsbTransfer;
    (*self_0).ref_count = 1 as libc::c_int as guint;
    (*self_0).type_0 = FP_TRANSFER_NONE;
    (*self_0).device = device;
    return self_0;
}
unsafe extern "C" fn fpi_usb_transfer_free(mut self_0: *mut FpiUsbTransfer) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !self_0.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-usb-transfer.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fpi_usb_transfer_free\0"))
                .as_ptr(),
            b"self\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut __n1: gint64 = (*self_0).ref_count as gint64;
    let mut __n2: gint64 = 0 as libc::c_int as gint64;
    if !(__n1 == __n2) {
        g_assertion_message_cmpnum(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-usb-transfer.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fpi_usb_transfer_free\0"))
                .as_ptr(),
            b"self->ref_count == 0\0" as *const u8 as *const libc::c_char,
            f128::f128::new(__n1),
            b"==\0" as *const u8 as *const libc::c_char,
            f128::f128::new(__n2),
            'i' as i32 as libc::c_char,
        );
    }
    if ((*self_0).free_buffer).is_some() && !((*self_0).buffer).is_null() {
        ((*self_0).free_buffer)
            .expect("non-null function pointer")((*self_0).buffer as gpointer);
    }
    (*self_0).buffer = 0 as *mut guchar;
    g_slice_free1(
        ::core::mem::size_of::<FpiUsbTransfer>() as libc::c_ulong,
        self_0 as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_usb_transfer_ref(
    mut self_0: *mut FpiUsbTransfer,
) -> *mut FpiUsbTransfer {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !self_0.is_null() {
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
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fpi_usb_transfer_ref\0"))
                .as_ptr(),
            b"self\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FpiUsbTransfer;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*self_0).ref_count != 0 {
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
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fpi_usb_transfer_ref\0"))
                .as_ptr(),
            b"self->ref_count\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FpiUsbTransfer;
    }
    if 0 as libc::c_int != 0 {} else {};
    ::core::intrinsics::atomic_xadd_seqcst(
        &mut (*self_0).ref_count,
        1 as libc::c_int as guint,
    );
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_usb_transfer_unref(mut self_0: *mut FpiUsbTransfer) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !self_0.is_null() {
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
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"fpi_usb_transfer_unref\0"))
                .as_ptr(),
            b"self\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*self_0).ref_count != 0 {
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
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"fpi_usb_transfer_unref\0"))
                .as_ptr(),
            b"self->ref_count\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        if 0 as libc::c_int != 0 {} else {};
        (::core::intrinsics::atomic_xsub_seqcst(
            &mut (*self_0).ref_count as *mut guint,
            1 as libc::c_int as guint,
        ) == 1 as libc::c_int as libc::c_uint) as libc::c_int
    }) != 0
    {
        fpi_usb_transfer_free(self_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fpi_usb_transfer_fill_bulk(
    mut transfer: *mut FpiUsbTransfer,
    mut endpoint: guint8,
    mut length: gsize,
) {
    fpi_usb_transfer_fill_bulk_full(
        transfer,
        endpoint,
        g_malloc0(length) as *mut guint8,
        length,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_usb_transfer_fill_bulk_full(
    mut transfer: *mut FpiUsbTransfer,
    mut endpoint: guint8,
    mut buffer: *mut guint8,
    mut length: gsize,
    mut free_func: GDestroyNotify,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*transfer).type_0 as libc::c_int == FP_TRANSFER_NONE as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-usb-transfer.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"fpi_usb_transfer_fill_bulk_full\0"))
                .as_ptr(),
            b"transfer->type == FP_TRANSFER_NONE\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !buffer.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-usb-transfer.c\0" as *const u8 as *const libc::c_char,
            204 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"fpi_usb_transfer_fill_bulk_full\0"))
                .as_ptr(),
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
        );
    }
    (*transfer).type_0 = FP_TRANSFER_BULK;
    (*transfer).endpoint = endpoint;
    (*transfer).buffer = buffer;
    (*transfer).length = length as gssize;
    (*transfer).free_buffer = free_func;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_usb_transfer_fill_control(
    mut transfer: *mut FpiUsbTransfer,
    mut direction: GUsbDeviceDirection,
    mut request_type: GUsbDeviceRequestType,
    mut recipient: GUsbDeviceRecipient,
    mut request: guint8,
    mut value: guint16,
    mut idx: guint16,
    mut length: gsize,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*transfer).type_0 as libc::c_int == FP_TRANSFER_NONE as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-usb-transfer.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"fpi_usb_transfer_fill_control\0"))
                .as_ptr(),
            b"transfer->type == FP_TRANSFER_NONE\0" as *const u8 as *const libc::c_char,
        );
    }
    (*transfer).type_0 = FP_TRANSFER_CONTROL;
    (*transfer).direction = direction;
    (*transfer).request_type = request_type;
    (*transfer).recipient = recipient;
    (*transfer).request = request;
    (*transfer).value = value;
    (*transfer).idx = idx;
    (*transfer).length = length as gssize;
    (*transfer).buffer = g_malloc0(length) as *mut guchar;
    (*transfer).free_buffer = Some(g_free as unsafe extern "C" fn(gpointer) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn fpi_usb_transfer_fill_interrupt(
    mut transfer: *mut FpiUsbTransfer,
    mut endpoint: guint8,
    mut length: gsize,
) {
    fpi_usb_transfer_fill_interrupt_full(
        transfer,
        endpoint,
        g_malloc0(length) as *mut guint8,
        length,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_usb_transfer_fill_interrupt_full(
    mut transfer: *mut FpiUsbTransfer,
    mut endpoint: guint8,
    mut buffer: *mut guint8,
    mut length: gsize,
    mut free_func: GDestroyNotify,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*transfer).type_0 as libc::c_int == FP_TRANSFER_NONE as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-usb-transfer.c\0" as *const u8 as *const libc::c_char,
            291 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fpi_usb_transfer_fill_interrupt_full\0"))
                .as_ptr(),
            b"transfer->type == FP_TRANSFER_NONE\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !buffer.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-usb-transfer.c\0" as *const u8 as *const libc::c_char,
            292 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fpi_usb_transfer_fill_interrupt_full\0"))
                .as_ptr(),
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
        );
    }
    (*transfer).type_0 = FP_TRANSFER_INTERRUPT;
    (*transfer).endpoint = endpoint;
    (*transfer).buffer = buffer;
    (*transfer).length = length as gssize;
    (*transfer).free_buffer = free_func;
}
unsafe extern "C" fn transfer_finish_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut error: *mut GError = 0 as *mut GError;
    let mut transfer: *mut FpiUsbTransfer = user_data as *mut FpiUsbTransfer;
    let mut callback: FpiUsbTransferCallback = None;
    match (*transfer).type_0 as libc::c_int {
        2 => {
            (*transfer)
                .actual_length = g_usb_device_bulk_transfer_finish(
                G_USB_DEVICE(source_object as gpointer),
                res,
                &mut error,
            );
        }
        0 => {
            (*transfer)
                .actual_length = g_usb_device_control_transfer_finish(
                G_USB_DEVICE(source_object as gpointer),
                res,
                &mut error,
            );
        }
        3 => {
            (*transfer)
                .actual_length = g_usb_device_interrupt_transfer_finish(
                G_USB_DEVICE(source_object as gpointer),
                res,
                &mut error,
            );
        }
        -1 | _ => {
            g_assertion_message_expr(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                b"../libfprint/fpi-usb-transfer.c\0" as *const u8 as *const libc::c_char,
                334 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 19],
                    &[libc::c_char; 19],
                >(b"transfer_finish_cb\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    }
    log_transfer(transfer, 0 as libc::c_int, error);
    if error.is_null() && (*transfer).short_is_error != 0
        && (*transfer).actual_length > 0 as libc::c_int as libc::c_long
        && (*transfer).actual_length != (*transfer).length
    {
        error = g_error_new(
            g_usb_device_error_quark(),
            G_USB_DEVICE_ERROR_IO as libc::c_int,
            b"Unexpected short error of %zd size (expected %zd)\0" as *const u8
                as *const libc::c_char,
            (*transfer).actual_length,
            (*transfer).length,
        );
    }
    callback = (*transfer).callback;
    (*transfer).callback = None;
    callback
        .expect(
            "non-null function pointer",
        )(transfer, (*transfer).device, (*transfer).user_data, error);
    fpi_usb_transfer_unref(transfer);
}
unsafe extern "C" fn transfer_cancel_cb(
    mut device: *mut FpDevice,
    mut user_data: gpointer,
) {
    let mut transfer: *mut FpiUsbTransfer = user_data as *mut FpiUsbTransfer;
    let mut error: *mut GError = 0 as *mut GError;
    let mut callback: FpiUsbTransferCallback = None;
    error = g_error_new_literal(
        g_io_error_quark(),
        G_IO_ERROR_CANCELLED as libc::c_int,
        b"Transfer was cancelled before being started\0" as *const u8
            as *const libc::c_char,
    );
    callback = (*transfer).callback;
    (*transfer).callback = None;
    (*transfer).actual_length = -(1 as libc::c_int) as gssize;
    callback
        .expect(
            "non-null function pointer",
        )(transfer, (*transfer).device, (*transfer).user_data, error);
    fpi_usb_transfer_unref(transfer);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_usb_transfer_submit(
    mut transfer: *mut FpiUsbTransfer,
    mut timeout_ms: guint,
    mut cancellable: *mut GCancellable,
    mut callback: FpiUsbTransferCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !transfer.is_null() {
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
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"fpi_usb_transfer_submit\0"))
                .as_ptr(),
            b"transfer\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
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
        g_return_if_fail_warning(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"fpi_usb_transfer_submit\0"))
                .as_ptr(),
            b"callback\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if ((*transfer).callback).is_none() {
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
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"fpi_usb_transfer_submit\0"))
                .as_ptr(),
            b"transfer->callback == NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*transfer).callback = callback;
    (*transfer).user_data = user_data;
    log_transfer(transfer, (0 as libc::c_int == 0) as libc::c_int, 0 as *mut GError);
    if !cancellable.is_null() && g_cancellable_is_cancelled(cancellable) != 0 {
        fpi_device_add_timeout(
            (*transfer).device,
            0 as libc::c_int,
            Some(
                transfer_cancel_cb as unsafe extern "C" fn(*mut FpDevice, gpointer) -> (),
            ),
            transfer as gpointer,
            None,
        );
        return;
    }
    match (*transfer).type_0 as libc::c_int {
        2 => {
            g_usb_device_bulk_transfer_async(
                fpi_device_get_usb_device((*transfer).device),
                (*transfer).endpoint,
                (*transfer).buffer,
                (*transfer).length as gsize,
                timeout_ms,
                cancellable,
                Some(
                    transfer_finish_cb
                        as unsafe extern "C" fn(
                            *mut GObject,
                            *mut GAsyncResult,
                            gpointer,
                        ) -> (),
                ),
                transfer as gpointer,
            );
        }
        0 => {
            g_usb_device_control_transfer_async(
                fpi_device_get_usb_device((*transfer).device),
                (*transfer).direction,
                (*transfer).request_type,
                (*transfer).recipient,
                (*transfer).request,
                (*transfer).value,
                (*transfer).idx,
                (*transfer).buffer,
                (*transfer).length as gsize,
                timeout_ms,
                cancellable,
                Some(
                    transfer_finish_cb
                        as unsafe extern "C" fn(
                            *mut GObject,
                            *mut GAsyncResult,
                            gpointer,
                        ) -> (),
                ),
                transfer as gpointer,
            );
        }
        3 => {
            g_usb_device_interrupt_transfer_async(
                fpi_device_get_usb_device((*transfer).device),
                (*transfer).endpoint,
                (*transfer).buffer,
                (*transfer).length as gsize,
                timeout_ms,
                cancellable,
                Some(
                    transfer_finish_cb
                        as unsafe extern "C" fn(
                            *mut GObject,
                            *mut GAsyncResult,
                            gpointer,
                        ) -> (),
                ),
                transfer as gpointer,
            );
        }
        -1 | _ => {
            fpi_usb_transfer_unref(transfer);
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"file %s: line %d (%s): should not be reached\0" as *const u8
                    as *const libc::c_char,
                b"../libfprint/fpi-usb-transfer.c\0" as *const u8 as *const libc::c_char,
                463 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"fpi_usb_transfer_submit\0"))
                    .as_ptr(),
            );
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_usb_transfer_submit_sync(
    mut transfer: *mut FpiUsbTransfer,
    mut timeout_ms: guint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut res: gboolean = 0;
    let mut actual_length: gsize = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !transfer.is_null() {
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
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"fpi_usb_transfer_submit_sync\0"))
                .as_ptr(),
            b"transfer\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if ((*transfer).callback).is_none() {
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
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"fpi_usb_transfer_submit_sync\0"))
                .as_ptr(),
            b"transfer->callback == NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    log_transfer(transfer, (0 as libc::c_int == 0) as libc::c_int, 0 as *mut GError);
    match (*transfer).type_0 as libc::c_int {
        2 => {
            res = g_usb_device_bulk_transfer(
                fpi_device_get_usb_device((*transfer).device),
                (*transfer).endpoint,
                (*transfer).buffer,
                (*transfer).length as gsize,
                &mut actual_length,
                timeout_ms,
                0 as *mut GCancellable,
                error,
            );
        }
        0 => {
            res = g_usb_device_control_transfer(
                fpi_device_get_usb_device((*transfer).device),
                (*transfer).direction,
                (*transfer).request_type,
                (*transfer).recipient,
                (*transfer).request,
                (*transfer).value,
                (*transfer).idx,
                (*transfer).buffer,
                (*transfer).length as gsize,
                &mut actual_length,
                timeout_ms,
                0 as *mut GCancellable,
                error,
            );
        }
        3 => {
            res = g_usb_device_interrupt_transfer(
                fpi_device_get_usb_device((*transfer).device),
                (*transfer).endpoint,
                (*transfer).buffer,
                (*transfer).length as gsize,
                &mut actual_length,
                timeout_ms,
                0 as *mut GCancellable,
                error,
            );
        }
        -1 | _ => {
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_CRITICAL,
                b"file %s: line %d (%s): should not be reached\0" as *const u8
                    as *const libc::c_char,
                b"../libfprint/fpi-usb-transfer.c\0" as *const u8 as *const libc::c_char,
                539 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"fpi_usb_transfer_submit_sync\0"))
                    .as_ptr(),
            );
            return 0 as libc::c_int;
        }
    }
    log_transfer(transfer, 0 as libc::c_int, *error);
    if res == 0 {
        (*transfer).actual_length = -(1 as libc::c_int) as gssize;
    } else {
        (*transfer).actual_length = actual_length as gssize;
    }
    return res;
}
