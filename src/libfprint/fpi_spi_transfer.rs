use ::libc;
use ::f128;
extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GTask;
    pub type _FpiSsm;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_file_get_contents(
        filename: *const gchar,
        contents: *mut *mut gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> gboolean;
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
    fn g_slice_alloc(block_size_0: gsize) -> gpointer;
    fn g_slice_free1(block_size_0: gsize, mem_block: gpointer);
    fn g_ascii_strtoull(
        nptr: *const gchar,
        endptr: *mut *mut gchar,
        base: guint,
    ) -> guint64;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    fn __errno_location() -> *mut libc::c_int;
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
    fn g_type_check_instance_is_a(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> gboolean;
    fn g_boxed_type_register_static(
        name: *const gchar,
        boxed_copy: GBoxedCopyFunc,
        boxed_free: GBoxedFreeFunc,
    ) -> GType;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
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
    fn g_task_run_in_thread(task: *mut GTask, task_func: GTaskThreadFunc);
    fn g_task_run_in_thread_sync(task: *mut GTask, task_func: GTaskThreadFunc);
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_new_error(
        task: *mut GTask,
        domain: GQuark,
        code: gint,
        format: *const libc::c_char,
        _: ...
    );
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn fp_device_get_type() -> GType;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
}
pub type guint8 = libc::c_uchar;
pub type guint16 = libc::c_ushort;
pub type guint32 = libc::c_uint;
pub type gint64 = libc::c_long;
pub type guint64 = libc::c_ulong;
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
pub type GError_autoptr = *mut GError;
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
pub type GIOErrorEnum = libc::c_uint;
pub const G_IO_ERROR_NO_SUCH_DEVICE: GIOErrorEnum = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: GIOErrorEnum = 46;
pub const G_IO_ERROR_NOT_CONNECTED: GIOErrorEnum = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: GIOErrorEnum = 44;
pub const G_IO_ERROR_BROKEN_PIPE: GIOErrorEnum = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: GIOErrorEnum = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: GIOErrorEnum = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: GIOErrorEnum = 41;
pub const G_IO_ERROR_PROXY_FAILED: GIOErrorEnum = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: GIOErrorEnum = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: GIOErrorEnum = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: GIOErrorEnum = 37;
pub const G_IO_ERROR_DBUS_ERROR: GIOErrorEnum = 36;
pub const G_IO_ERROR_INVALID_DATA: GIOErrorEnum = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: GIOErrorEnum = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: GIOErrorEnum = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: GIOErrorEnum = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: GIOErrorEnum = 31;
pub const G_IO_ERROR_FAILED_HANDLED: GIOErrorEnum = 30;
pub const G_IO_ERROR_WOULD_MERGE: GIOErrorEnum = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: GIOErrorEnum = 28;
pub const G_IO_ERROR_WOULD_BLOCK: GIOErrorEnum = 27;
pub const G_IO_ERROR_BUSY: GIOErrorEnum = 26;
pub const G_IO_ERROR_WOULD_RECURSE: GIOErrorEnum = 25;
pub const G_IO_ERROR_TIMED_OUT: GIOErrorEnum = 24;
pub const G_IO_ERROR_WRONG_ETAG: GIOErrorEnum = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: GIOErrorEnum = 22;
pub const G_IO_ERROR_READ_ONLY: GIOErrorEnum = 21;
pub const G_IO_ERROR_PENDING: GIOErrorEnum = 20;
pub const G_IO_ERROR_CANCELLED: GIOErrorEnum = 19;
pub const G_IO_ERROR_CLOSED: GIOErrorEnum = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: GIOErrorEnum = 17;
pub const G_IO_ERROR_NOT_MOUNTED: GIOErrorEnum = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: GIOErrorEnum = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: GIOErrorEnum = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: GIOErrorEnum = 13;
pub const G_IO_ERROR_NO_SPACE: GIOErrorEnum = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: GIOErrorEnum = 11;
pub const G_IO_ERROR_INVALID_FILENAME: GIOErrorEnum = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: GIOErrorEnum = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: GIOErrorEnum = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: GIOErrorEnum = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: GIOErrorEnum = 6;
pub const G_IO_ERROR_NOT_EMPTY: GIOErrorEnum = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: GIOErrorEnum = 4;
pub const G_IO_ERROR_IS_DIRECTORY: GIOErrorEnum = 3;
pub const G_IO_ERROR_EXISTS: GIOErrorEnum = 2;
pub const G_IO_ERROR_NOT_FOUND: GIOErrorEnum = 1;
pub const G_IO_ERROR_FAILED: GIOErrorEnum = 0;
pub type GAsyncResult = _GAsyncResult;
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
pub type GTaskThreadFunc = Option::<
    unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
>;
pub type GTask_autoptr = *mut GTask;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpDevice {
    pub parent_instance: GObject,
}
pub type FpDevice = _FpDevice;
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
pub type FpiSsm = _FpiSsm;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub do_free_type: Option::<unsafe extern "C" fn(*mut FpiSpiTransfer) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub do_copy_type: Option::<
        unsafe extern "C" fn(*mut FpiSpiTransfer) -> *mut FpiSpiTransfer,
    >,
    pub do_const_copy_type: Option::<
        unsafe extern "C" fn(*const FpiSpiTransfer) -> *mut FpiSpiTransfer,
    >,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spi_ioc_transfer {
    pub tx_buf: __u64,
    pub rx_buf: __u64,
    pub len: __u32,
    pub speed_hz: __u32,
    pub delay_usecs: __u16,
    pub bits_per_word: __u8,
    pub cs_change: __u8,
    pub tx_nbits: __u8,
    pub rx_nbits: __u8,
    pub word_delay_usecs: __u8,
    pub pad: __u8,
}
pub type __u8 = libc::c_uchar;
pub type __u16 = libc::c_ushort;
pub type __u32 = libc::c_uint;
pub type __u64 = libc::c_ulonglong;
#[inline]
unsafe extern "C" fn g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = 0 as *mut libc::c_void;
    ref_0 = *ptr;
    *ptr = 0 as *mut libc::c_void;
    return ref_0;
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
static mut block_size: gsize = 0 as libc::c_int as gsize;
#[no_mangle]
pub unsafe extern "C" fn fpi_spi_transfer_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_spi_transfer_get_type_once();
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
unsafe extern "C" fn fpi_spi_transfer_get_type_once() -> GType {
    let mut _g_register_boxed: Option::<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_0, C2RustUnnamed) -> GType,
    > = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
        >,
        Option::<
            unsafe extern "C" fn(*const gchar, C2RustUnnamed_0, C2RustUnnamed) -> GType,
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
        g_intern_static_string(b"FpiSpiTransfer\0" as *const u8 as *const libc::c_char),
        C2RustUnnamed_0 {
            do_copy_type: Some(
                fpi_spi_transfer_ref
                    as unsafe extern "C" fn(*mut FpiSpiTransfer) -> *mut FpiSpiTransfer,
            ),
        },
        C2RustUnnamed {
            do_free_type: Some(
                fpi_spi_transfer_unref as unsafe extern "C" fn(*mut FpiSpiTransfer) -> (),
            ),
        },
    );
    return g_define_type_id;
}
unsafe extern "C" fn dump_buffer(mut buf: *mut guchar, mut dump_len: gssize) {
    let mut line: GString_autoptr = 0 as GString_autoptr;
    line = g_string_new(b"\0" as *const u8 as *const libc::c_char);
    let mut i: gssize = 0 as libc::c_int as gssize;
    while i < dump_len {
        g_string_append_printf(
            line,
            b"%02x \0" as *const u8 as *const libc::c_char,
            *buf.offset(i as isize) as libc::c_int,
        );
        if (i + 1 as libc::c_int as libc::c_long) % 16 as libc::c_int as libc::c_long
            == 0 as libc::c_int as libc::c_long
        {
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
unsafe extern "C" fn log_transfer(
    mut transfer: *mut FpiSpiTransfer,
    mut submit: gboolean,
    mut error: *mut GError,
) {
    if !(g_getenv(b"FP_DEBUG_TRANSFER\0" as *const u8 as *const libc::c_char)).is_null()
    {
        if submit != 0 {
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"Transfer %p submitted, write length %zd, read length %zd\0"
                    as *const u8 as *const libc::c_char,
                transfer,
                (*transfer).length_wr,
                (*transfer).length_rd,
            );
            if !((*transfer).buffer_wr).is_null() {
                dump_buffer((*transfer).buffer_wr, (*transfer).length_wr);
            }
        } else {
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
                b"Transfer %p completed %s, write length %zd, read length %zd\0"
                    as *const u8 as *const libc::c_char,
                transfer,
                error_str,
                (*transfer).length_wr,
                (*transfer).length_rd,
            );
            if !((*transfer).buffer_rd).is_null() {
                dump_buffer((*transfer).buffer_rd, (*transfer).length_rd);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn fpi_spi_transfer_new(
    mut device: *mut FpDevice,
    mut spidev_fd: libc::c_int,
) -> *mut FpiSpiTransfer {
    let mut self_0: *mut FpiSpiTransfer = 0 as *mut FpiSpiTransfer;
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
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-spi-transfer.c\0" as *const u8 as *const libc::c_char,
            120 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fpi_spi_transfer_new\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if block_size == 0 as libc::c_int as libc::c_ulong {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        let mut error: GError_autoptr = 0 as GError_autoptr;
        let mut contents: *mut libc::c_char = 0 as *mut libc::c_char;
        block_size = 4096 as libc::c_int as gsize;
        if g_file_get_contents(
            b"/sys/module/spidev/parameters/bufsiz\0" as *const u8
                as *const libc::c_char,
            &mut contents,
            0 as *mut gsize,
            &mut error,
        ) != 0
        {
            block_size = if g_ascii_strtoull(
                contents,
                0 as *mut *mut gchar,
                0 as libc::c_int as guint,
            ) < 0xffff as libc::c_int as guint16 as libc::c_ulong
            {
                g_ascii_strtoull(
                    contents,
                    0 as *mut *mut gchar,
                    0 as libc::c_int as guint,
                )
            } else {
                0xffff as libc::c_int as guint16 as libc::c_ulong
            };
            if block_size == 0 as libc::c_int as libc::c_ulong {
                block_size = 4096 as libc::c_int as gsize;
                g_log(
                    b"libfprint\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_WARNING,
                    b"spidev blocksize could not be decoded, using %lu\0" as *const u8
                        as *const libc::c_char,
                    block_size,
                );
            }
        } else {
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_MESSAGE,
                b"Failed to read spidev block size, using %lu\0" as *const u8
                    as *const libc::c_char,
                block_size,
            );
        }
    }
    self_0 = ({
        let mut __s: gsize = ::core::mem::size_of::<FpiSpiTransfer>() as libc::c_ulong;
        let mut __p: gpointer = 0 as *mut libc::c_void;
        __p = g_slice_alloc(__s);
        memset(__p, 0 as libc::c_int, __s);
        __p
    }) as *mut FpiSpiTransfer;
    (*self_0).ref_count = 1 as libc::c_int as guint;
    (*self_0).length_wr = -(1 as libc::c_int) as gssize;
    (*self_0).length_rd = -(1 as libc::c_int) as gssize;
    (*self_0).device = device;
    (*self_0).spidev_fd = spidev_fd;
    return self_0;
}
unsafe extern "C" fn fpi_spi_transfer_free(mut self_0: *mut FpiSpiTransfer) {
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
            b"../libfprint/fpi-spi-transfer.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fpi_spi_transfer_free\0"))
                .as_ptr(),
            b"self\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut __n1: gint64 = (*self_0).ref_count as gint64;
    let mut __n2: gint64 = 0 as libc::c_int as gint64;
    if !(__n1 == __n2) {
        g_assertion_message_cmpnum(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-spi-transfer.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fpi_spi_transfer_free\0"))
                .as_ptr(),
            b"self->ref_count == 0\0" as *const u8 as *const libc::c_char,
            f128::f128::new(__n1),
            b"==\0" as *const u8 as *const libc::c_char,
            f128::f128::new(__n2),
            'i' as i32 as libc::c_char,
        );
    }
    if ((*self_0).free_buffer_wr).is_some() && !((*self_0).buffer_wr).is_null() {
        ((*self_0).free_buffer_wr)
            .expect("non-null function pointer")((*self_0).buffer_wr as gpointer);
    }
    if ((*self_0).free_buffer_rd).is_some() && !((*self_0).buffer_rd).is_null() {
        ((*self_0).free_buffer_rd)
            .expect("non-null function pointer")((*self_0).buffer_rd as gpointer);
    }
    (*self_0).buffer_wr = 0 as *mut guchar;
    (*self_0).buffer_rd = 0 as *mut guchar;
    g_slice_free1(
        ::core::mem::size_of::<FpiSpiTransfer>() as libc::c_ulong,
        self_0 as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_spi_transfer_ref(
    mut self_0: *mut FpiSpiTransfer,
) -> *mut FpiSpiTransfer {
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
            >(b"fpi_spi_transfer_ref\0"))
                .as_ptr(),
            b"self\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FpiSpiTransfer;
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
            >(b"fpi_spi_transfer_ref\0"))
                .as_ptr(),
            b"self->ref_count\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FpiSpiTransfer;
    }
    if 0 as libc::c_int != 0 {} else {};
    ::core::intrinsics::atomic_xadd_seqcst(
        &mut (*self_0).ref_count,
        1 as libc::c_int as guint,
    );
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_spi_transfer_unref(mut self_0: *mut FpiSpiTransfer) {
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
            >(b"fpi_spi_transfer_unref\0"))
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
            >(b"fpi_spi_transfer_unref\0"))
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
        fpi_spi_transfer_free(self_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fpi_spi_transfer_write(
    mut transfer: *mut FpiSpiTransfer,
    mut length: gsize,
) {
    fpi_spi_transfer_write_full(
        transfer,
        g_malloc0(length) as *mut guint8,
        length,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_spi_transfer_write_full(
    mut transfer: *mut FpiSpiTransfer,
    mut buffer: *mut guint8,
    mut length: gsize,
    mut free_func: GDestroyNotify,
) {
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
            b"../libfprint/fpi-spi-transfer.c\0" as *const u8 as *const libc::c_char,
            242 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fpi_spi_transfer_write_full\0"))
                .as_ptr(),
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
        );
    }
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
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fpi_spi_transfer_write_full\0"))
                .as_ptr(),
            b"transfer\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if ((*transfer).buffer_wr).is_null() {
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
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fpi_spi_transfer_write_full\0"))
                .as_ptr(),
            b"transfer->buffer_wr == NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if ((*transfer).buffer_rd).is_null() {
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
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"fpi_spi_transfer_write_full\0"))
                .as_ptr(),
            b"transfer->buffer_rd == NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*transfer).buffer_wr = buffer;
    (*transfer).length_wr = length as gssize;
    (*transfer).free_buffer_wr = free_func;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_spi_transfer_read(
    mut transfer: *mut FpiSpiTransfer,
    mut length: gsize,
) {
    fpi_spi_transfer_read_full(
        transfer,
        g_malloc0(length) as *mut guint8,
        length,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_spi_transfer_read_full(
    mut transfer: *mut FpiSpiTransfer,
    mut buffer: *mut guint8,
    mut length: gsize,
    mut free_func: GDestroyNotify,
) {
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
            b"../libfprint/fpi-spi-transfer.c\0" as *const u8 as *const libc::c_char,
            287 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_spi_transfer_read_full\0"))
                .as_ptr(),
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
        );
    }
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
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_spi_transfer_read_full\0"))
                .as_ptr(),
            b"transfer\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if ((*transfer).buffer_rd).is_null() {
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
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fpi_spi_transfer_read_full\0"))
                .as_ptr(),
            b"transfer->buffer_rd == NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*transfer).buffer_rd = buffer;
    (*transfer).length_rd = length as gssize;
    (*transfer).free_buffer_rd = free_func;
}
unsafe extern "C" fn transfer_finish_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = g_type_check_instance_cast(
        res as *mut GTypeInstance,
        g_task_get_type(),
    ) as *mut libc::c_void as *mut GTask;
    let mut transfer: *mut FpiSpiTransfer = g_task_get_task_data(task)
        as *mut FpiSpiTransfer;
    let mut error: *mut GError = 0 as *mut GError;
    let mut callback: FpiSpiTransferCallback = None;
    g_task_propagate_boolean(task, &mut error);
    log_transfer(transfer, 0 as libc::c_int, error);
    callback = (*transfer).callback;
    (*transfer).callback = None;
    callback
        .expect(
            "non-null function pointer",
        )(transfer, (*transfer).device, (*transfer).user_data, error);
}
unsafe extern "C" fn transfer_chunk(
    mut transfer: *mut FpiSpiTransfer,
    mut full_length: gsize,
    mut transferred: *mut gsize,
) -> libc::c_int {
    let mut xfer: [spi_ioc_transfer; 2] = [
        {
            let mut init = spi_ioc_transfer {
                tx_buf: 0 as libc::c_int as __u64,
                rx_buf: 0,
                len: 0,
                speed_hz: 0,
                delay_usecs: 0,
                bits_per_word: 0,
                cs_change: 0,
                tx_nbits: 0,
                rx_nbits: 0,
                word_delay_usecs: 0,
                pad: 0,
            };
            init
        },
        spi_ioc_transfer {
            tx_buf: 0,
            rx_buf: 0,
            len: 0,
            speed_hz: 0,
            delay_usecs: 0,
            bits_per_word: 0,
            cs_change: 0,
            tx_nbits: 0,
            rx_nbits: 0,
            word_delay_usecs: 0,
            pad: 0,
        },
    ];
    let mut skip: gsize = *transferred;
    let mut len: gsize = 0 as libc::c_int as gsize;
    let mut transfers: libc::c_int = 0 as libc::c_int;
    let mut status: libc::c_int = 0;
    if !((*transfer).buffer_wr).is_null() {
        if skip < (*transfer).length_wr as libc::c_ulong && len < block_size {
            xfer[transfers as usize]
                .tx_buf = ((*transfer).buffer_wr as gsize).wrapping_add(skip) as __u64;
            xfer[transfers as usize]
                .len = (if block_size
                < ((*transfer).length_wr as libc::c_ulong).wrapping_sub(skip)
            {
                block_size
            } else {
                ((*transfer).length_wr as libc::c_ulong).wrapping_sub(skip)
            }) as __u32;
            len = (len as libc::c_ulong)
                .wrapping_add(xfer[transfers as usize].len as libc::c_ulong) as gsize
                as gsize;
            skip = (skip as libc::c_ulong)
                .wrapping_add(xfer[transfers as usize].len as libc::c_ulong) as gsize
                as gsize;
            transfers += 1 as libc::c_int;
        }
        skip = (skip as libc::c_ulong)
            .wrapping_sub((*transfer).length_wr as libc::c_ulong) as gsize as gsize;
    }
    if !((*transfer).buffer_rd).is_null() {
        if skip < (*transfer).length_rd as libc::c_ulong && len < block_size {
            xfer[transfers as usize]
                .rx_buf = ((*transfer).buffer_rd as gsize).wrapping_add(skip) as __u64;
            xfer[transfers as usize]
                .len = (if block_size
                < ((*transfer).length_rd as libc::c_ulong).wrapping_sub(skip)
            {
                block_size
            } else {
                ((*transfer).length_rd as libc::c_ulong).wrapping_sub(skip)
            }) as __u32;
            len = (len as libc::c_ulong)
                .wrapping_add(xfer[transfers as usize].len as libc::c_ulong) as gsize
                as gsize;
            transfers += 1 as libc::c_int;
        }
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if transfers > 0 as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fpi-spi-transfer.c\0" as *const u8 as *const libc::c_char,
            356 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"transfer_chunk\0"))
                .as_ptr(),
            b"transfers > 0\0" as *const u8 as *const libc::c_char,
        );
    }
    if full_length < (*transferred).wrapping_add(len) {
        static mut warned: gboolean = 0 as libc::c_int;
        if warned == 0 {
            g_log(
                b"libfprint\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_MESSAGE,
                b"Split SPI transfer. In case of issues, try increasing the spidev buffer size.\0"
                    as *const u8 as *const libc::c_char,
            );
            warned = (0 as libc::c_int == 0) as libc::c_int;
        }
        xfer[(transfers - 1 as libc::c_int) as usize]
            .cs_change = (0 as libc::c_int == 0) as libc::c_int as __u8;
    }
    status = ioctl(
        (*transfer).spidev_fd,
        ((1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('k' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((0 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | ((::core::mem::size_of::<libc::c_char>()
                * (if (transfers as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<spi_ioc_transfer>() as libc::c_ulong,
                    ) < ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_ulong
                {
                    (transfers as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<spi_ioc_transfer>() as libc::c_ulong,
                        )
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) as usize) as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        xfer.as_mut_ptr(),
    );
    if status >= 0 as libc::c_int {
        *transferred = (*transferred as libc::c_ulong).wrapping_add(len) as gsize
            as gsize;
    }
    return status;
}
unsafe extern "C" fn transfer_thread_func(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut transfer: *mut FpiSpiTransfer = task_data as *mut FpiSpiTransfer;
    let mut full_length: gsize = 0;
    let mut transferred: gsize = 0 as libc::c_int as gsize;
    let mut status: libc::c_int = 0 as libc::c_int;
    if ((*transfer).buffer_wr).is_null() && ((*transfer).buffer_rd).is_null() {
        g_task_return_new_error(
            task,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as libc::c_int,
            b"Transfer with neither write or read!\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    full_length = 0 as libc::c_int as gsize;
    if !((*transfer).buffer_wr).is_null() {
        full_length = (full_length as libc::c_ulong)
            .wrapping_add((*transfer).length_wr as libc::c_ulong) as gsize as gsize;
    }
    if !((*transfer).buffer_rd).is_null() {
        full_length = (full_length as libc::c_ulong)
            .wrapping_add((*transfer).length_rd as libc::c_ulong) as gsize as gsize;
    }
    while transferred < full_length && status >= 0 as libc::c_int {
        status = transfer_chunk(transfer, full_length, &mut transferred);
    }
    if status < 0 as libc::c_int {
        g_task_return_new_error(
            task,
            g_io_error_quark(),
            g_io_error_from_errno(*__errno_location()) as gint,
            b"Error invoking ioctl for SPI transfer (%d)\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
    } else {
        g_task_return_boolean(task, (0 as libc::c_int == 0) as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_spi_transfer_submit(
    mut transfer: *mut FpiSpiTransfer,
    mut cancellable: *mut GCancellable,
    mut callback: FpiSpiTransferCallback,
    mut user_data: gpointer,
) {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
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
            >(b"fpi_spi_transfer_submit\0"))
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
            >(b"fpi_spi_transfer_submit\0"))
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
            >(b"fpi_spi_transfer_submit\0"))
                .as_ptr(),
            b"transfer->callback == NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*transfer).callback = callback;
    (*transfer).user_data = user_data;
    log_transfer(transfer, (0 as libc::c_int == 0) as libc::c_int, 0 as *mut GError);
    task = g_task_new(
        (*transfer).device as gpointer,
        cancellable,
        Some(
            transfer_finish_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        0 as *mut libc::c_void,
    );
    g_task_set_task_data(
        task,
        if 0 as libc::c_int != 0 {
            transfer as *mut libc::c_void
        } else {
            g_steal_pointer(&mut transfer as *mut *mut FpiSpiTransfer as gpointer)
        },
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FpiSpiTransfer) -> ()>,
            GDestroyNotify,
        >(
            Some(
                fpi_spi_transfer_unref as unsafe extern "C" fn(*mut FpiSpiTransfer) -> (),
            ),
        ),
    );
    g_task_run_in_thread(
        task,
        Some(
            transfer_thread_func
                as unsafe extern "C" fn(
                    *mut GTask,
                    gpointer,
                    gpointer,
                    *mut GCancellable,
                ) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_spi_transfer_submit_sync(
    mut transfer: *mut FpiSpiTransfer,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut task: GTask_autoptr = 0 as GTask_autoptr;
    let mut err: *mut GError = 0 as *mut GError;
    let mut res: gboolean = 0;
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
            >(b"fpi_spi_transfer_submit_sync\0"))
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
            >(b"fpi_spi_transfer_submit_sync\0"))
                .as_ptr(),
            b"transfer->callback == NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    log_transfer(transfer, (0 as libc::c_int == 0) as libc::c_int, 0 as *mut GError);
    task = g_task_new(
        (*transfer).device as gpointer,
        0 as *mut GCancellable,
        None,
        0 as *mut libc::c_void,
    );
    g_task_set_task_data(
        task,
        fpi_spi_transfer_ref(transfer) as gpointer,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FpiSpiTransfer) -> ()>,
            GDestroyNotify,
        >(
            Some(
                fpi_spi_transfer_unref as unsafe extern "C" fn(*mut FpiSpiTransfer) -> (),
            ),
        ),
    );
    g_task_run_in_thread_sync(
        task,
        Some(
            transfer_thread_func
                as unsafe extern "C" fn(
                    *mut GTask,
                    gpointer,
                    gpointer,
                    *mut GCancellable,
                ) -> (),
        ),
    );
    res = g_task_propagate_boolean(task, &mut err);
    log_transfer(transfer, 0 as libc::c_int, err);
    g_propagate_error(error, err);
    return res;
}
