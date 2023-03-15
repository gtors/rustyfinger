use ::libc;
extern "C" {
    pub type _GData;
    pub type _GCancellablePrivate;
    pub type _FpiSsm;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn g_type_check_instance_cast(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> *mut GTypeInstance;
    fn fp_device_get_type() -> GType;
    fn fp_image_device_get_type() -> GType;
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
}
pub type size_t = libc::c_ulong;
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
pub struct fpi_frame {
    pub delta_x: libc::c_int,
    pub delta_y: libc::c_int,
    pub data: [libc::c_uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fpi_frame_asmbl_ctx {
    pub frame_width: libc::c_uint,
    pub frame_height: libc::c_uint,
    pub image_width: libc::c_uint,
    pub get_pixel: Option::<
        unsafe extern "C" fn(
            *mut fpi_frame_asmbl_ctx,
            *mut fpi_frame,
            libc::c_uint,
            libc::c_uint,
        ) -> libc::c_uchar,
    >,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpImageDevice {
    pub parent_instance: FpDevice,
}
pub type FpImageDevice = _FpImageDevice;
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
pub struct aes_regwrite {
    pub reg: libc::c_uchar,
    pub value: libc::c_uchar,
}
pub type aes_write_regv_cb = Option::<
    unsafe extern "C" fn(*mut FpImageDevice, *mut GError, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct write_regv_data {
    pub num_regs: libc::c_uint,
    pub regs: *const aes_regwrite,
    pub offset: libc::c_uint,
    pub callback: aes_write_regv_cb,
    pub user_data: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn FP_DEVICE(mut ptr: gpointer) -> *mut FpDevice {
    return g_type_check_instance_cast(ptr as *mut GTypeInstance, fp_device_get_type())
        as *mut libc::c_void as *mut FpDevice;
}
#[inline]
unsafe extern "C" fn FP_IMAGE_DEVICE(mut ptr: gpointer) -> *mut FpImageDevice {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fp_image_device_get_type(),
    ) as *mut libc::c_void as *mut FpImageDevice;
}
unsafe extern "C" fn write_regv_trf_complete(
    mut transfer: *mut FpiUsbTransfer,
    mut device: *mut FpDevice,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut wdata: *mut write_regv_data = user_data as *mut write_regv_data;
    if !error.is_null() {
        ((*wdata).callback)
            .expect(
                "non-null function pointer",
            )(FP_IMAGE_DEVICE(device as gpointer), error, (*wdata).user_data);
        g_free(wdata as gpointer);
    } else {
        continue_write_regv(FP_IMAGE_DEVICE(device as gpointer), wdata);
    };
}
unsafe extern "C" fn do_write_regv(
    mut dev: *mut FpImageDevice,
    mut wdata: *mut write_regv_data,
    mut upper_bound: libc::c_int,
) {
    let mut offset: libc::c_uint = (*wdata).offset;
    let mut num: libc::c_uint = (upper_bound as libc::c_uint)
        .wrapping_sub(offset)
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    let mut alloc_size: size_t = num.wrapping_mul(2 as libc::c_int as libc::c_uint)
        as size_t;
    let mut i: libc::c_uint = 0;
    let mut data_offset: size_t = 0 as libc::c_int as size_t;
    let mut transfer: *mut FpiUsbTransfer = fpi_usb_transfer_new(
        FP_DEVICE(dev as gpointer),
    );
    fpi_usb_transfer_fill_bulk(
        transfer,
        (2 as libc::c_int | 0 as libc::c_int) as guint8,
        alloc_size,
    );
    i = offset;
    while i < offset.wrapping_add(num) {
        let mut regwrite: *const aes_regwrite = &*((*wdata).regs).offset(i as isize)
            as *const aes_regwrite;
        let fresh0 = data_offset;
        data_offset = data_offset.wrapping_add(1);
        *((*transfer).buffer).offset(fresh0 as isize) = (*regwrite).reg;
        let fresh1 = data_offset;
        data_offset = data_offset.wrapping_add(1);
        *((*transfer).buffer).offset(fresh1 as isize) = (*regwrite).value;
        i = i.wrapping_add(1);
    }
    (*transfer).short_is_error = (0 as libc::c_int == 0) as libc::c_int;
    fpi_usb_transfer_submit(
        transfer,
        4000 as libc::c_int as guint,
        0 as *mut GCancellable,
        Some(
            write_regv_trf_complete
                as unsafe extern "C" fn(
                    *mut FpiUsbTransfer,
                    *mut FpDevice,
                    gpointer,
                    *mut GError,
                ) -> (),
        ),
        wdata as gpointer,
    );
}
unsafe extern "C" fn continue_write_regv(
    mut dev: *mut FpImageDevice,
    mut wdata: *mut write_regv_data,
) {
    let mut offset: libc::c_uint = (*wdata).offset;
    let mut regs_remaining: libc::c_uint = 0;
    let mut limit: libc::c_uint = 0;
    let mut upper_bound: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    while 0 as libc::c_int == 0 {
        if offset >= (*wdata).num_regs {
            g_log(
                b"libfprint-aeslib\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_DEBUG,
                b"all registers written\0" as *const u8 as *const libc::c_char,
            );
            ((*wdata).callback)
                .expect(
                    "non-null function pointer",
                )(dev, 0 as *mut GError, (*wdata).user_data);
            g_free(wdata as gpointer);
            return;
        }
        if (*((*wdata).regs).offset(offset as isize)).reg != 0 {
            break;
        }
        offset = offset.wrapping_add(1);
    }
    (*wdata).offset = offset;
    regs_remaining = ((*wdata).num_regs).wrapping_sub(offset);
    limit = if regs_remaining < 16 as libc::c_int as libc::c_uint {
        regs_remaining
    } else {
        16 as libc::c_int as libc::c_uint
    };
    upper_bound = offset
        .wrapping_add(limit)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    i = offset as libc::c_int;
    while i as libc::c_uint <= upper_bound {
        if (*((*wdata).regs).offset(i as isize)).reg == 0 {
            upper_bound = (i - 1 as libc::c_int) as libc::c_uint;
            break;
        } else {
            i += 1;
        }
    }
    do_write_regv(dev, wdata, upper_bound as libc::c_int);
    (*wdata).offset = upper_bound.wrapping_add(1 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn aes_write_regv(
    mut dev: *mut FpImageDevice,
    mut regs: *const aes_regwrite,
    mut num_regs: libc::c_uint,
    mut callback: aes_write_regv_cb,
    mut user_data: *mut libc::c_void,
) {
    let mut wdata: *mut write_regv_data = 0 as *mut write_regv_data;
    g_log(
        b"libfprint-aeslib\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"write %d regs\0" as *const u8 as *const libc::c_char,
        num_regs,
    );
    wdata = g_malloc(::core::mem::size_of::<write_regv_data>() as libc::c_ulong)
        as *mut write_regv_data;
    (*wdata).num_regs = num_regs;
    (*wdata).regs = regs;
    (*wdata).offset = 0 as libc::c_int as libc::c_uint;
    (*wdata).callback = callback;
    (*wdata).user_data = user_data;
    continue_write_regv(dev, wdata);
}
#[no_mangle]
pub unsafe extern "C" fn aes_get_pixel(
    mut ctx: *mut fpi_frame_asmbl_ctx,
    mut frame: *mut fpi_frame,
    mut x: libc::c_uint,
    mut y: libc::c_uint,
) -> libc::c_uchar {
    let mut ret: libc::c_uchar = 0;
    ret = *((*frame).data)
        .as_mut_ptr()
        .offset(
            x
                .wrapping_mul((*ctx).frame_height >> 1 as libc::c_int)
                .wrapping_add(y >> 1 as libc::c_int) as isize,
        );
    ret = (if y.wrapping_rem(2 as libc::c_int as libc::c_uint) != 0 {
        ret as libc::c_int >> 4 as libc::c_int
    } else {
        ret as libc::c_int & 0xf as libc::c_int
    }) as libc::c_uchar;
    ret = (ret as libc::c_int * 17 as libc::c_int) as libc::c_uchar;
    return ret;
}
