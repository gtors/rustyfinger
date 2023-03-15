use ::libc;
extern "C" {
    pub type _GData;
    pub type _FpiSsm;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_free(mem: gpointer);
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
    fn fp_image_new(width: gint, height: gint) -> *mut FpImage;
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
    fn fpi_device_get_usb_device(device: *mut FpDevice) -> *mut GUsbDevice;
    fn fp_image_device_get_type() -> GType;
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
    fn fpi_ssm_next_state_delayed(machine: *mut FpiSsm, delay: libc::c_int);
    fn fpi_ssm_mark_completed(machine: *mut FpiSsm);
    fn fpi_ssm_get_cur_state(machine: *mut FpiSsm) -> libc::c_int;
    fn vfs301_proto_init(dev: *mut FpDeviceVfs301);
    fn vfs301_proto_deinit(dev: *mut FpDeviceVfs301);
    fn vfs301_proto_request_fingerprint(dev: *mut FpDeviceVfs301);
    fn vfs301_proto_peek_event(dev: *mut FpDeviceVfs301) -> libc::c_int;
    fn vfs301_proto_process_event_start(dev: *mut FpDeviceVfs301);
    fn vfs301_proto_process_event_poll(dev: *mut FpDeviceVfs301) -> libc::c_int;
    fn vfs301_extract_image(
        vfs: *mut FpDeviceVfs301,
        output: *mut libc::c_uchar,
        output_height: *mut libc::c_int,
    );
}
pub type guint8 = libc::c_uchar;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUsbDevice {
    pub parent_instance: GObject,
}
pub type GUsbDevice = _GUsbDevice;
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
pub type FpiSsm = _FpiSsm;
pub type FpiSsmCompletedCallback = Option::<
    unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
>;
pub type FpiSsmHandlerCallback = Option::<
    unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpDeviceVfs301 {
    pub parent: FpImageDevice,
    pub scanline_buf: *mut libc::c_uchar,
    pub scanline_count: libc::c_int,
    pub recv_progress: C2RustUnnamed_4,
    pub recv_exp_amt: libc::c_int,
}
pub type C2RustUnnamed_4 = libc::c_int;
pub const VFS301_FAILURE: C2RustUnnamed_4 = -1;
pub const VFS301_ENDED: C2RustUnnamed_4 = 1;
pub const VFS301_ONGOING: C2RustUnnamed_4 = 0;
pub type FpDeviceVfs301 = _FpDeviceVfs301;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpDeviceVfs301Class {
    pub parent_class: FpImageDeviceClass,
}
pub const VFS301_FP_WIDTH: C2RustUnnamed_6 = 200;
pub const M_LOOP_NUM_STATES: C2RustUnnamed_7 = 7;
pub const M_REQUEST_PRINT: C2RustUnnamed_7 = 0;
pub const VFS301_FP_OUTPUT_WIDTH: C2RustUnnamed_6 = 200;
pub const M_SUBMIT_PRINT: C2RustUnnamed_7 = 6;
pub const M_READ_PRINT_WAIT: C2RustUnnamed_7 = 4;
pub const M_READ_PRINT_POLL: C2RustUnnamed_7 = 5;
pub const M_READ_PRINT_START: C2RustUnnamed_7 = 3;
pub const M_WAIT_PRINT: C2RustUnnamed_7 = 1;
pub const M_CHECK_PRINT: C2RustUnnamed_7 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const VFS301_FP_RECV_TIMEOUT: C2RustUnnamed_6 = 2000;
pub const VFS301_FP_LINE_DIFF_THRESHOLD: C2RustUnnamed_6 = 15;
pub const VFS301_FP_SUM_LINES: C2RustUnnamed_6 = 3;
pub const VFS301_FP_FRAME_SIZE: C2RustUnnamed_6 = 288;
pub type C2RustUnnamed_7 = libc::c_uint;
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
#[inline]
unsafe extern "C" fn FPI_DEVICE_VFS301(mut ptr: gpointer) -> *mut FpDeviceVfs301 {
    return g_type_check_instance_cast(
        ptr as *mut GTypeInstance,
        fpi_device_vfs301_get_type(),
    ) as *mut libc::c_void as *mut FpDeviceVfs301;
}
static mut FpDeviceVfs301_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn fpi_device_vfs301_get_type() -> GType {
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
        let mut g_define_type_id: GType = fpi_device_vfs301_get_type_once();
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
static mut fpi_device_vfs301_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
#[inline(never)]
unsafe extern "C" fn fpi_device_vfs301_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        fp_image_device_get_type(),
        g_intern_static_string(b"FpDeviceVfs301\0" as *const u8 as *const libc::c_char),
        ::core::mem::size_of::<FpDeviceVfs301Class>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_vfs301_class_intern_init
                        as unsafe extern "C" fn(gpointer) -> (),
                ),
            ),
        ),
        ::core::mem::size_of::<FpDeviceVfs301>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpDeviceVfs301) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    fpi_device_vfs301_init
                        as unsafe extern "C" fn(*mut FpDeviceVfs301) -> (),
                ),
            ),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn fpi_device_vfs301_class_intern_init(mut klass: gpointer) {
    fpi_device_vfs301_parent_class = g_type_class_peek_parent(klass);
    if FpDeviceVfs301_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpDeviceVfs301_private_offset);
    }
    fpi_device_vfs301_class_init(klass as *mut FpDeviceVfs301Class);
}
unsafe extern "C" fn submit_image(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpImageDevice,
) -> libc::c_int {
    let mut self_0: *mut FpDeviceVfs301 = FPI_DEVICE_VFS301(dev as gpointer);
    let mut height: libc::c_int = 0;
    let mut img: *mut FpImage = 0 as *mut FpImage;
    img = fp_image_new(VFS301_FP_OUTPUT_WIDTH as libc::c_int, (*self_0).scanline_count);
    if img.is_null() {
        return 0 as libc::c_int;
    }
    vfs301_extract_image(self_0, (*img).data, &mut height);
    (*img)
        .flags = (FPI_IMAGE_COLORS_INVERTED as libc::c_int
        | FPI_IMAGE_V_FLIPPED as libc::c_int) as FpiImageFlags;
    (*img).width = VFS301_FP_OUTPUT_WIDTH as libc::c_int as guint;
    (*img).height = height as guint;
    fpi_image_device_image_captured(dev, img);
    return 1 as libc::c_int;
}
unsafe extern "C" fn m_loop_state(mut ssm: *mut FpiSsm, mut _dev: *mut FpDevice) {
    let mut dev: *mut FpImageDevice = FP_IMAGE_DEVICE(_dev as gpointer);
    let mut self_0: *mut FpDeviceVfs301 = FPI_DEVICE_VFS301(_dev as gpointer);
    match fpi_ssm_get_cur_state(ssm) {
        0 => {
            vfs301_proto_request_fingerprint(self_0);
            fpi_ssm_next_state(ssm);
        }
        1 => {
            fpi_ssm_next_state_delayed(ssm, 200 as libc::c_int);
        }
        2 => {
            if vfs301_proto_peek_event(self_0) == 0 {
                fpi_ssm_jump_to_state(ssm, M_WAIT_PRINT as libc::c_int);
            } else {
                fpi_ssm_next_state(ssm);
            }
        }
        3 => {
            fpi_image_device_report_finger_status(
                dev,
                (0 as libc::c_int == 0) as libc::c_int,
            );
            vfs301_proto_process_event_start(self_0);
            fpi_ssm_next_state(ssm);
        }
        4 => {
            fpi_ssm_next_state_delayed(ssm, 200 as libc::c_int);
        }
        5 => {
            let mut rv: libc::c_int = vfs301_proto_process_event_poll(self_0);
            if ({
                let mut _g_boolean_var_: libc::c_int = 0;
                if rv != VFS301_FAILURE as libc::c_int {
                    _g_boolean_var_ = 1 as libc::c_int;
                } else {
                    _g_boolean_var_ = 0 as libc::c_int;
                }
                _g_boolean_var_
            }) as libc::c_long != 0
            {} else {
                g_assertion_message_expr(
                    b"libfprint-vfs301\0" as *const u8 as *const libc::c_char,
                    b"../libfprint/drivers/vfs301.c\0" as *const u8
                        as *const libc::c_char,
                    124 as libc::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 13],
                        &[libc::c_char; 13],
                    >(b"m_loop_state\0"))
                        .as_ptr(),
                    b"rv != VFS301_FAILURE\0" as *const u8 as *const libc::c_char,
                );
            }
            if rv == VFS301_ONGOING as libc::c_int {
                fpi_ssm_jump_to_state(ssm, M_READ_PRINT_WAIT as libc::c_int);
            } else {
                fpi_ssm_next_state(ssm);
            }
        }
        6 => {
            if submit_image(ssm, dev) != 0 {
                fpi_ssm_mark_completed(ssm);
                fpi_image_device_report_finger_status(dev, 0 as libc::c_int);
            } else {
                fpi_ssm_jump_to_state(ssm, M_REQUEST_PRINT as libc::c_int);
            }
        }
        _ => {
            g_assertion_message_expr(
                b"libfprint-vfs301\0" as *const u8 as *const libc::c_char,
                b"../libfprint/drivers/vfs301.c\0" as *const u8 as *const libc::c_char,
                146 as libc::c_int,
                (*::core::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"m_loop_state\0"))
                    .as_ptr(),
                0 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn m_init_state(mut ssm: *mut FpiSsm, mut _dev: *mut FpDevice) {
    let mut self_0: *mut FpDeviceVfs301 = FPI_DEVICE_VFS301(_dev as gpointer);
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_ssm_get_cur_state(ssm) == 0 as libc::c_int {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-vfs301\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/vfs301.c\0" as *const u8 as *const libc::c_char,
            156 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"m_init_state\0"))
                .as_ptr(),
            b"fpi_ssm_get_cur_state (ssm) == 0\0" as *const u8 as *const libc::c_char,
        );
    }
    vfs301_proto_init(self_0);
    fpi_ssm_mark_completed(ssm);
}
unsafe extern "C" fn m_init_complete(
    mut ssm: *mut FpiSsm,
    mut dev: *mut FpDevice,
    mut error: *mut GError,
) {
    fpi_image_device_activate_complete(FP_IMAGE_DEVICE(dev as gpointer), error);
}
unsafe extern "C" fn dev_activate(mut dev: *mut FpImageDevice) {
    let mut ssm: *mut FpiSsm = 0 as *mut FpiSsm;
    ssm = fpi_ssm_new_full(
        FP_DEVICE(dev as gpointer),
        Some(m_init_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
        1 as libc::c_int,
        1 as libc::c_int,
        b"1\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(
        ssm,
        Some(
            m_init_complete
                as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice, *mut GError) -> (),
        ),
    );
}
unsafe extern "C" fn dev_deactivate(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpDeviceVfs301 = 0 as *mut FpDeviceVfs301;
    self_0 = FPI_DEVICE_VFS301(dev as gpointer);
    vfs301_proto_deinit(self_0);
    fpi_image_device_deactivate_complete(dev, 0 as *mut GError);
}
unsafe extern "C" fn dev_change_state(
    mut dev: *mut FpImageDevice,
    mut state: FpiImageDeviceState,
) {
    let mut ssm_loop: *mut FpiSsm = 0 as *mut FpiSsm;
    if state as libc::c_uint
        != FPI_IMAGE_DEVICE_STATE_AWAIT_FINGER_ON as libc::c_int as libc::c_uint
    {
        return;
    }
    ssm_loop = fpi_ssm_new_full(
        FP_DEVICE(dev as gpointer),
        Some(m_loop_state as unsafe extern "C" fn(*mut FpiSsm, *mut FpDevice) -> ()),
        M_LOOP_NUM_STATES as libc::c_int,
        M_LOOP_NUM_STATES as libc::c_int,
        b"M_LOOP_NUM_STATES\0" as *const u8 as *const libc::c_char,
    );
    fpi_ssm_start(ssm_loop, None);
}
unsafe extern "C" fn dev_open(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpDeviceVfs301 = FPI_DEVICE_VFS301(dev as gpointer);
    let mut error: *mut GError = 0 as *mut GError;
    g_usb_device_claim_interface(
        fpi_device_get_usb_device(FP_DEVICE(dev as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    );
    (*self_0).scanline_count = 0 as libc::c_int;
    fpi_image_device_open_complete(dev, error);
}
unsafe extern "C" fn dev_close(mut dev: *mut FpImageDevice) {
    let mut self_0: *mut FpDeviceVfs301 = FPI_DEVICE_VFS301(dev as gpointer);
    let mut error: *mut GError = 0 as *mut GError;
    let mut _pp: C2RustUnnamed_5 = C2RustUnnamed_5 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp
        .in_0 = &mut (*self_0).scanline_buf as *mut *mut libc::c_uchar
        as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    g_usb_device_release_interface(
        fpi_device_get_usb_device(FP_DEVICE(dev as gpointer)),
        0 as libc::c_int,
        G_USB_DEVICE_CLAIM_INTERFACE_NONE,
        &mut error,
    );
    fpi_image_device_close_complete(dev, error);
}
static mut id_table: [FpIdEntry; 3] = [
    {
        let mut init = _FpIdEntry {
            c2rust_unnamed: C2RustUnnamed_0 {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_3 {
                        pid: 0x5 as libc::c_int as guint,
                        vid: 0x138a as libc::c_int as guint,
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
                        pid: 0x8 as libc::c_int as guint,
                        vid: 0x138a as libc::c_int as guint,
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
unsafe extern "C" fn fpi_device_vfs301_init(mut self_0: *mut FpDeviceVfs301) {}
unsafe extern "C" fn fpi_device_vfs301_class_init(mut klass: *mut FpDeviceVfs301Class) {
    let mut dev_class: *mut FpDeviceClass = FP_DEVICE_CLASS(klass as gpointer);
    let mut img_class: *mut FpImageDeviceClass = FP_IMAGE_DEVICE_CLASS(
        klass as gpointer,
    );
    (*dev_class).id = b"vfs301\0" as *const u8 as *const libc::c_char;
    (*dev_class).full_name = b"Validity VFS301\0" as *const u8 as *const libc::c_char;
    (*dev_class).type_0 = FP_DEVICE_TYPE_USB;
    (*dev_class).id_table = id_table.as_ptr();
    (*dev_class).scan_type = FP_SCAN_TYPE_SWIPE;
    (*img_class)
        .img_open = Some(dev_open as unsafe extern "C" fn(*mut FpImageDevice) -> ());
    (*img_class)
        .img_close = Some(dev_close as unsafe extern "C" fn(*mut FpImageDevice) -> ());
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
    (*img_class).bz3_threshold = 24 as libc::c_int;
    (*img_class).img_width = VFS301_FP_WIDTH as libc::c_int;
    (*img_class).img_height = -(1 as libc::c_int);
}
