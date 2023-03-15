use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        format: *const gchar,
        _: ...
    );
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_date_new_julian(julian_day: guint32) -> *mut GDate;
    fn g_date_free(date: *mut GDate);
    fn g_date_copy(date: *const GDate) -> *mut GDate;
    fn g_date_valid(date: *const GDate) -> gboolean;
    fn g_date_get_julian(date: *const GDate) -> guint32;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_variant_type_checked_(_: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_new_variant(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_new_fixed_array(
        element_type: *const GVariantType,
        elements: gconstpointer,
        n_elements: gsize,
        element_size: gsize,
    ) -> *mut GVariant;
    fn g_variant_n_children(value: *mut GVariant) -> gsize;
    fn g_variant_get_child_value(value: *mut GVariant, index_: gsize) -> *mut GVariant;
    fn g_variant_get_fixed_array(
        value: *mut GVariant,
        n_elements: *mut gsize,
        element_size: gsize,
    ) -> gconstpointer;
    fn g_variant_get_size(value: *mut GVariant) -> gsize;
    fn g_variant_get_data(value: *mut GVariant) -> gconstpointer;
    fn g_variant_store(value: *mut GVariant, data: gpointer);
    fn g_variant_equal(one: gconstpointer, two: gconstpointer) -> gboolean;
    fn g_variant_get_normal_form(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_byteswap(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_new_from_data(
        type_0: *const GVariantType,
        data: gconstpointer,
        size: gsize,
        trusted: gboolean,
        notify: GDestroyNotify,
        user_data: gpointer,
    ) -> *mut GVariant;
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_open(
        builder: *mut GVariantBuilder,
        type_0: *const GVariantType,
    );
    fn g_variant_builder_close(builder: *mut GVariantBuilder);
    fn g_variant_builder_add_value(builder: *mut GVariantBuilder, value: *mut GVariant);
    fn g_variant_builder_add(
        builder: *mut GVariantBuilder,
        format_string: *const gchar,
        _: ...
    );
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, _: ...);
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
    fn g_strcmp0(str1: *const libc::c_char, str2: *const libc::c_char) -> libc::c_int;
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
    fn g_type_name(type_0: GType) -> *const gchar;
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
    fn g_type_check_instance_is_a(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> gboolean;
    fn g_type_check_class_cast(
        g_class: *mut GTypeClass,
        is_a_type: GType,
    ) -> *mut GTypeClass;
    fn g_date_get_type() -> GType;
    fn g_value_set_boxed(value: *mut GValue, v_boxed: gconstpointer);
    fn g_value_dup_boxed(value: *const GValue) -> gpointer;
    fn g_initially_unowned_get_type() -> GType;
    fn g_object_class_install_properties(
        oclass: *mut GObjectClass,
        n_pspecs: guint,
        pspecs: *mut *mut GParamSpec,
    );
    fn g_object_new(
        object_type: GType,
        first_property_name: *const gchar,
        _: ...
    ) -> gpointer;
    fn g_object_set(object: gpointer, first_property_name: *const gchar, _: ...);
    fn g_object_notify_by_pspec(object: *mut GObject, pspec: *mut GParamSpec);
    fn g_object_ref_sink(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_set_enum(value: *mut GValue, v_enum: gint);
    fn g_value_get_enum(value: *const GValue) -> gint;
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
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
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_boxed(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        boxed_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_pointer(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_variant(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        type_0: *const GVariantType,
        default_value: *mut GVariant,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn g_value_set_pointer(value: *mut GValue, v_pointer: gpointer);
    fn g_value_get_pointer(value: *const GValue) -> gpointer;
    fn g_value_set_variant(value: *mut GValue, variant: *mut GVariant);
    fn g_value_dup_variant(value: *const GValue) -> *mut GVariant;
    fn fpi_print_type_get_type() -> GType;
    fn g_io_error_quark() -> GQuark;
    fn fp_image_get_type() -> GType;
    fn fp_device_get_type() -> GType;
    fn fp_finger_get_type() -> GType;
    fn fpi_print_set_type(print: *mut FpPrint, type_0: FpiPrintType);
    fn fp_device_get_device_id(device: *mut FpDevice) -> *const gchar;
    fn fp_device_get_driver(device: *mut FpDevice) -> *const gchar;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GDate {
    #[bitfield(name = "julian_days", ty = "guint", bits = "0..=31")]
    #[bitfield(name = "julian", ty = "guint", bits = "32..=32")]
    #[bitfield(name = "dmy", ty = "guint", bits = "33..=33")]
    #[bitfield(name = "day", ty = "guint", bits = "34..=39")]
    #[bitfield(name = "month", ty = "guint", bits = "40..=43")]
    #[bitfield(name = "year", ty = "guint", bits = "44..=59")]
    pub julian_days_julian_dmy_day_month_year: [u8; 8],
}
pub type GDate = _GDate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantBuilder {
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: C2RustUnnamed_0,
    pub x: [gsize; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub partial_magic: gsize,
    pub type_0: *const GVariantType,
    pub y: [gsize; 14],
}
pub type GVariantBuilder = _GVariantBuilder;
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
pub type GDate_autoptr = *mut GDate;
pub type GVariant_autoptr = *mut GVariant;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed_1; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub type GInitiallyUnowned = _GObject;
pub type GInitiallyUnownedClass = _GObjectClass;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_2 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_2 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_2 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_2 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_2 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_2 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_2 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_2 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_2 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_2 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_2 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_2 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_2 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_2 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_2 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_2 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_2 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_2 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_2 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_2 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_2 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_2 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_2 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_2 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_2 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_2 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_2 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_2 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_2 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_2 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_2 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_2 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_2 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_2 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_2 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_2 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_2 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_2 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_2 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_2 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_2 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_2 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_2 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_2 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_2 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_2 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_2 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_2 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_2 = 0;
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
pub struct _FpDevice {
    pub parent_instance: GObject,
}
pub type FpDevice = _FpDevice;
pub type FpPrint = _FpPrint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FpPrint {
    pub parent_instance: GInitiallyUnowned,
    pub type_0: FpiPrintType,
    pub driver: *mut gchar,
    pub device_id: *mut gchar,
    pub device_stored: gboolean,
    pub image: *mut FpImage,
    pub finger: FpFinger,
    pub username: *mut gchar,
    pub description: *mut gchar,
    pub enroll_date: *mut GDate,
    pub data: *mut GVariant,
    pub prints: *mut GPtrArray,
}
pub type FpFinger = libc::c_uint;
pub const FP_FINGER_LAST: FpFinger = 10;
pub const FP_FINGER_FIRST: FpFinger = 1;
pub const FP_FINGER_RIGHT_LITTLE: FpFinger = 10;
pub const FP_FINGER_RIGHT_RING: FpFinger = 9;
pub const FP_FINGER_RIGHT_MIDDLE: FpFinger = 8;
pub const FP_FINGER_RIGHT_INDEX: FpFinger = 7;
pub const FP_FINGER_RIGHT_THUMB: FpFinger = 6;
pub const FP_FINGER_LEFT_LITTLE: FpFinger = 5;
pub const FP_FINGER_LEFT_RING: FpFinger = 4;
pub const FP_FINGER_LEFT_MIDDLE: FpFinger = 3;
pub const FP_FINGER_LEFT_INDEX: FpFinger = 2;
pub const FP_FINGER_LEFT_THUMB: FpFinger = 1;
pub const FP_FINGER_UNKNOWN: FpFinger = 0;
pub type FpiPrintType = libc::c_uint;
pub const FPI_PRINT_NBIS: FpiPrintType = 2;
pub const FPI_PRINT_RAW: FpiPrintType = 1;
pub const FPI_PRINT_UNDEFINED: FpiPrintType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpPrintClass {
    pub parent_class: GInitiallyUnownedClass,
}
pub const N_PROPS: C2RustUnnamed_19 = 12;
pub const PROP_FPI_PRINTS: C2RustUnnamed_19 = 11;
pub const PROP_FPI_DATA: C2RustUnnamed_19 = 10;
pub const PROP_FPI_TYPE: C2RustUnnamed_19 = 9;
pub const PROP_ENROLL_DATE: C2RustUnnamed_19 = 8;
pub const PROP_DESCRIPTION: C2RustUnnamed_19 = 7;
pub const PROP_USERNAME: C2RustUnnamed_19 = 6;
pub const PROP_FINGER: C2RustUnnamed_19 = 5;
pub const PROP_IMAGE: C2RustUnnamed_19 = 4;
pub const PROP_DEVICE_STORED: C2RustUnnamed_19 = 3;
pub const PROP_DEVICE_ID: C2RustUnnamed_19 = 2;
pub const PROP_DRIVER: C2RustUnnamed_19 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
pub type FpPrint_autoptr = *mut FpPrint;
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
pub struct xyt_struct {
    pub nrows: libc::c_int,
    pub xcol: [libc::c_int; 200],
    pub ycol: [libc::c_int; 200],
    pub thetacol: [libc::c_int; 200],
}
pub type C2RustUnnamed_19 = libc::c_uint;
pub const PROP_0: C2RustUnnamed_19 = 0;
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
#[inline]
unsafe extern "C" fn FP_PRINT(mut ptr: gpointer) -> *mut FpPrint {
    return g_type_check_instance_cast(ptr as *mut GTypeInstance, fp_print_get_type())
        as *mut libc::c_void as *mut FpPrint;
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
pub unsafe extern "C" fn fp_print_get_type() -> GType {
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
        let mut g_define_type_id: GType = fp_print_get_type_once();
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
unsafe extern "C" fn fp_print_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_initially_unowned_get_type(),
        g_intern_static_string(b"FpPrint\0" as *const u8 as *const libc::c_char),
        ::core::mem::size_of::<FpPrintClass>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(Some(fp_print_class_intern_init as unsafe extern "C" fn(gpointer) -> ())),
        ),
        ::core::mem::size_of::<FpPrint>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpPrint) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(Some(fp_print_init as unsafe extern "C" fn(*mut FpPrint) -> ())),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn fp_print_class_intern_init(mut klass: gpointer) {
    fp_print_parent_class = g_type_class_peek_parent(klass);
    if FpPrint_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpPrint_private_offset);
    }
    fp_print_class_init(klass as *mut FpPrintClass);
}
static mut FpPrint_private_offset: gint = 0;
static mut fp_print_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut properties: [*mut GParamSpec; 12] = [0 as *const GParamSpec
    as *mut GParamSpec; 12];
unsafe extern "C" fn fp_print_finalize(mut object: *mut GObject) {
    let mut self_0: *mut FpPrint = object as *mut FpPrint;
    let mut _pp: C2RustUnnamed_15 = C2RustUnnamed_15 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*self_0).image as *mut *mut FpImage as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    let mut _pp_0: C2RustUnnamed_14 = C2RustUnnamed_14 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_0: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp_0.in_0 = &mut (*self_0).device_id as *mut *mut gchar as *mut libc::c_char;
    _p_0 = *_pp_0.out;
    if !_p_0.is_null() {
        *_pp_0.out = 0 as *mut libc::c_void;
        _destroy_0.expect("non-null function pointer")(_p_0);
    }
    let mut _pp_1: C2RustUnnamed_13 = C2RustUnnamed_13 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_1: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_1: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp_1.in_0 = &mut (*self_0).driver as *mut *mut gchar as *mut libc::c_char;
    _p_1 = *_pp_1.out;
    if !_p_1.is_null() {
        *_pp_1.out = 0 as *mut libc::c_void;
        _destroy_1.expect("non-null function pointer")(_p_1);
    }
    let mut _pp_2: C2RustUnnamed_12 = C2RustUnnamed_12 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_2: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_2: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp_2.in_0 = &mut (*self_0).username as *mut *mut gchar as *mut libc::c_char;
    _p_2 = *_pp_2.out;
    if !_p_2.is_null() {
        *_pp_2.out = 0 as *mut libc::c_void;
        _destroy_2.expect("non-null function pointer")(_p_2);
    }
    let mut _pp_3: C2RustUnnamed_11 = C2RustUnnamed_11 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_3: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_3: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp_3.in_0 = &mut (*self_0).description as *mut *mut gchar as *mut libc::c_char;
    _p_3 = *_pp_3.out;
    if !_p_3.is_null() {
        *_pp_3.out = 0 as *mut libc::c_void;
        _destroy_3.expect("non-null function pointer")(_p_3);
    }
    let mut _pp_4: C2RustUnnamed_10 = C2RustUnnamed_10 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_4: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_4: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GDate) -> ()>,
        GDestroyNotify,
    >(Some(g_date_free as unsafe extern "C" fn(*mut GDate) -> ()));
    _pp_4.in_0 = &mut (*self_0).enroll_date as *mut *mut GDate as *mut libc::c_char;
    _p_4 = *_pp_4.out;
    if !_p_4.is_null() {
        *_pp_4.out = 0 as *mut libc::c_void;
        _destroy_4.expect("non-null function pointer")(_p_4);
    }
    let mut _pp_5: C2RustUnnamed_9 = C2RustUnnamed_9 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_5: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_5: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GVariant) -> ()>,
        GDestroyNotify,
    >(Some(g_variant_unref as unsafe extern "C" fn(*mut GVariant) -> ()));
    _pp_5.in_0 = &mut (*self_0).data as *mut *mut GVariant as *mut libc::c_char;
    _p_5 = *_pp_5.out;
    if !_p_5.is_null() {
        *_pp_5.out = 0 as *mut libc::c_void;
        _destroy_5.expect("non-null function pointer")(_p_5);
    }
    let mut _pp_6: C2RustUnnamed_8 = C2RustUnnamed_8 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_6: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_6: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GPtrArray) -> ()>,
        GDestroyNotify,
    >(Some(g_ptr_array_unref as unsafe extern "C" fn(*mut GPtrArray) -> ()));
    _pp_6.in_0 = &mut (*self_0).prints as *mut *mut GPtrArray as *mut libc::c_char;
    _p_6 = *_pp_6.out;
    if !_p_6.is_null() {
        *_pp_6.out = 0 as *mut libc::c_void;
        _destroy_6.expect("non-null function pointer")(_p_6);
    }
    ((*(g_type_check_class_cast(
        fp_print_parent_class as *mut GTypeClass,
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
    ) as *mut libc::c_void as *mut GObjectClass))
        .finalize)
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn fp_print_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut FpPrint = FP_PRINT(object as gpointer);
    match prop_id {
        1 => {
            g_value_set_string(value, (*self_0).driver);
        }
        2 => {
            g_value_set_string(value, (*self_0).device_id);
        }
        3 => {
            g_value_set_boolean(value, (*self_0).device_stored);
        }
        4 => {
            g_value_set_object(value, (*self_0).image as gpointer);
        }
        5 => {
            g_value_set_enum(value, (*self_0).finger as gint);
        }
        6 => {
            g_value_set_string(value, (*self_0).username);
        }
        7 => {
            g_value_set_string(value, (*self_0).description);
        }
        8 => {
            g_value_set_boxed(value, (*self_0).enroll_date as gconstpointer);
        }
        9 => {
            g_value_set_enum(value, (*self_0).type_0 as gint);
        }
        10 => {
            g_value_set_variant(value, (*self_0).data);
        }
        11 => {
            g_value_set_pointer(value, (*self_0).prints as gpointer);
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                b"libfprint-print\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const libc::c_char,
                b"../libfprint/fp-print.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int,
                b"property\0" as *const u8 as *const libc::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn fp_print_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut FpPrint = FP_PRINT(object as gpointer);
    match prop_id {
        9 => {
            fpi_print_set_type(self_0, g_value_get_enum(value) as FpiPrintType);
        }
        1 => {
            (*self_0).driver = g_value_dup_string(value);
        }
        2 => {
            (*self_0).device_id = g_value_dup_string(value);
        }
        3 => {
            (*self_0).device_stored = g_value_get_boolean(value);
        }
        5 => {
            (*self_0).finger = g_value_get_enum(value) as FpFinger;
        }
        6 => {
            let mut _pp: C2RustUnnamed_7 = C2RustUnnamed_7 {
                in_0: 0 as *mut libc::c_char,
            };
            let mut _p: gpointer = 0 as *mut libc::c_void;
            let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                GDestroyNotify,
            >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
            _pp.in_0 = &mut (*self_0).username as *mut *mut gchar as *mut libc::c_char;
            _p = *_pp.out;
            if !_p.is_null() {
                *_pp.out = 0 as *mut libc::c_void;
                _destroy.expect("non-null function pointer")(_p);
            }
            (*self_0).username = g_value_dup_string(value);
        }
        7 => {
            let mut _pp_0: C2RustUnnamed_6 = C2RustUnnamed_6 {
                in_0: 0 as *mut libc::c_char,
            };
            let mut _p_0: gpointer = 0 as *mut libc::c_void;
            let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                GDestroyNotify,
            >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
            _pp_0
                .in_0 = &mut (*self_0).description as *mut *mut gchar
                as *mut libc::c_char;
            _p_0 = *_pp_0.out;
            if !_p_0.is_null() {
                *_pp_0.out = 0 as *mut libc::c_void;
                _destroy_0.expect("non-null function pointer")(_p_0);
            }
            (*self_0).description = g_value_dup_string(value);
        }
        8 => {
            let mut _pp_1: C2RustUnnamed_5 = C2RustUnnamed_5 {
                in_0: 0 as *mut libc::c_char,
            };
            let mut _p_1: gpointer = 0 as *mut libc::c_void;
            let mut _destroy_1: GDestroyNotify = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut GDate) -> ()>,
                GDestroyNotify,
            >(Some(g_date_free as unsafe extern "C" fn(*mut GDate) -> ()));
            _pp_1
                .in_0 = &mut (*self_0).enroll_date as *mut *mut GDate
                as *mut libc::c_char;
            _p_1 = *_pp_1.out;
            if !_p_1.is_null() {
                *_pp_1.out = 0 as *mut libc::c_void;
                _destroy_1.expect("non-null function pointer")(_p_1);
            }
            (*self_0).enroll_date = g_value_dup_boxed(value) as *mut GDate;
        }
        10 => {
            let mut _pp_2: C2RustUnnamed_4 = C2RustUnnamed_4 {
                in_0: 0 as *mut libc::c_char,
            };
            let mut _p_2: gpointer = 0 as *mut libc::c_void;
            let mut _destroy_2: GDestroyNotify = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut GVariant) -> ()>,
                GDestroyNotify,
            >(Some(g_variant_unref as unsafe extern "C" fn(*mut GVariant) -> ()));
            _pp_2.in_0 = &mut (*self_0).data as *mut *mut GVariant as *mut libc::c_char;
            _p_2 = *_pp_2.out;
            if !_p_2.is_null() {
                *_pp_2.out = 0 as *mut libc::c_void;
                _destroy_2.expect("non-null function pointer")(_p_2);
            }
            (*self_0).data = g_value_dup_variant(value);
        }
        11 => {
            let mut _pp_3: C2RustUnnamed_3 = C2RustUnnamed_3 {
                in_0: 0 as *mut libc::c_char,
            };
            let mut _p_3: gpointer = 0 as *mut libc::c_void;
            let mut _destroy_3: GDestroyNotify = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut GPtrArray) -> ()>,
                GDestroyNotify,
            >(Some(g_ptr_array_unref as unsafe extern "C" fn(*mut GPtrArray) -> ()));
            _pp_3
                .in_0 = &mut (*self_0).prints as *mut *mut GPtrArray
                as *mut libc::c_char;
            _p_3 = *_pp_3.out;
            if !_p_3.is_null() {
                *_pp_3.out = 0 as *mut libc::c_void;
                _destroy_3.expect("non-null function pointer")(_p_3);
            }
            (*self_0).prints = g_value_get_pointer(value) as *mut GPtrArray;
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                b"libfprint-print\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const libc::c_char,
                b"../libfprint/fp-print.c\0" as *const u8 as *const libc::c_char,
                202 as libc::c_int,
                b"property\0" as *const u8 as *const libc::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn fp_print_constructed(mut obj: *mut GObject) {
    let mut self_0: *mut FpPrint = FP_PRINT(obj as gpointer);
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !((*self_0).driver).is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-print.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fp_print_constructed\0"))
                .as_ptr(),
            b"self->driver != NULL\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !((*self_0).device_id).is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-print.c\0" as *const u8 as *const libc::c_char,
            212 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fp_print_constructed\0"))
                .as_ptr(),
            b"self->device_id != NULL\0" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn fp_print_class_init(mut klass: *mut FpPrintClass) {
    let mut object_class: *mut GObjectClass = g_type_check_class_cast(
        klass as *mut GTypeClass,
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
    ) as *mut libc::c_void as *mut GObjectClass;
    (*object_class)
        .constructed = Some(
        fp_print_constructed as unsafe extern "C" fn(*mut GObject) -> (),
    );
    (*object_class)
        .finalize = Some(fp_print_finalize as unsafe extern "C" fn(*mut GObject) -> ());
    (*object_class)
        .get_property = Some(
        fp_print_get_property
            as unsafe extern "C" fn(
                *mut GObject,
                guint,
                *mut GValue,
                *mut GParamSpec,
            ) -> (),
    );
    (*object_class)
        .set_property = Some(
        fp_print_set_property
            as unsafe extern "C" fn(
                *mut GObject,
                guint,
                *const GValue,
                *mut GParamSpec,
            ) -> (),
    );
    properties[PROP_DRIVER as libc::c_int
        as usize] = g_param_spec_string(
        b"driver\0" as *const u8 as *const libc::c_char,
        b"Driver\0" as *const u8 as *const libc::c_char,
        b"The name of the driver that created the print\0" as *const u8
            as *const libc::c_char,
        0 as *const gchar,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READWRITE as libc::c_int
            | G_PARAM_CONSTRUCT_ONLY as libc::c_int) as GParamFlags,
    );
    properties[PROP_DEVICE_ID as libc::c_int
        as usize] = g_param_spec_string(
        b"device-id\0" as *const u8 as *const libc::c_char,
        b"Device ID\0" as *const u8 as *const libc::c_char,
        b"Unique ID allowing to check if a device is compatible with the print\0"
            as *const u8 as *const libc::c_char,
        0 as *const gchar,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READWRITE as libc::c_int
            | G_PARAM_CONSTRUCT_ONLY as libc::c_int) as GParamFlags,
    );
    properties[PROP_DEVICE_STORED as libc::c_int
        as usize] = g_param_spec_boolean(
        b"device-stored\0" as *const u8 as *const libc::c_char,
        b"Device Stored\0" as *const u8 as *const libc::c_char,
        b"Whether the print is a handle for data that is stored on the device\0"
            as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READWRITE as libc::c_int
            | G_PARAM_CONSTRUCT_ONLY as libc::c_int) as GParamFlags,
    );
    properties[PROP_IMAGE as libc::c_int
        as usize] = g_param_spec_object(
        b"image\0" as *const u8 as *const libc::c_char,
        b"Image\0" as *const u8 as *const libc::c_char,
        b"The image that was used for the print, only valid for newly enrolled prints on image based devices\0"
            as *const u8 as *const libc::c_char,
        fp_image_get_type(),
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READABLE as libc::c_int)
            as GParamFlags,
    );
    properties[PROP_FINGER as libc::c_int
        as usize] = g_param_spec_enum(
        b"finger\0" as *const u8 as *const libc::c_char,
        b"Finger\0" as *const u8 as *const libc::c_char,
        b"The enrolled finger\0" as *const u8 as *const libc::c_char,
        fp_finger_get_type(),
        FP_FINGER_UNKNOWN as libc::c_int,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READWRITE as libc::c_int)
            as GParamFlags,
    );
    properties[PROP_USERNAME as libc::c_int
        as usize] = g_param_spec_string(
        b"username\0" as *const u8 as *const libc::c_char,
        b"Username\0" as *const u8 as *const libc::c_char,
        b"The username that the enrolled print belongs to\0" as *const u8
            as *const libc::c_char,
        0 as *const gchar,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READWRITE as libc::c_int)
            as GParamFlags,
    );
    properties[PROP_DESCRIPTION as libc::c_int
        as usize] = g_param_spec_string(
        b"description\0" as *const u8 as *const libc::c_char,
        b"Description\0" as *const u8 as *const libc::c_char,
        b"A user defined description for the print\0" as *const u8
            as *const libc::c_char,
        0 as *const gchar,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READWRITE as libc::c_int)
            as GParamFlags,
    );
    properties[PROP_ENROLL_DATE as libc::c_int
        as usize] = g_param_spec_boxed(
        b"enroll-date\0" as *const u8 as *const libc::c_char,
        b"Enroll Date\0" as *const u8 as *const libc::c_char,
        b"The date of enrollment\0" as *const u8 as *const libc::c_char,
        g_date_get_type(),
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READWRITE as libc::c_int)
            as GParamFlags,
    );
    properties[PROP_FPI_TYPE as libc::c_int
        as usize] = g_param_spec_enum(
        b"fpi-type\0" as *const u8 as *const libc::c_char,
        b"Type\0" as *const u8 as *const libc::c_char,
        b"Private: The type of the print data\0" as *const u8 as *const libc::c_char,
        fpi_print_type_get_type(),
        FPI_PRINT_UNDEFINED as libc::c_int,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READWRITE as libc::c_int
            | G_PARAM_EXPLICIT_NOTIFY as libc::c_int) as GParamFlags,
    );
    properties[PROP_FPI_DATA as libc::c_int
        as usize] = g_param_spec_variant(
        b"fpi-data\0" as *const u8 as *const libc::c_char,
        b"Raw Data\0" as *const u8 as *const libc::c_char,
        b"The raw data for internal use only\0" as *const u8 as *const libc::c_char,
        b"*\0" as *const u8 as *const libc::c_char as *const GVariantType,
        0 as *mut GVariant,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READWRITE as libc::c_int)
            as GParamFlags,
    );
    properties[PROP_FPI_PRINTS as libc::c_int
        as usize] = g_param_spec_pointer(
        b"fpi-prints\0" as *const u8 as *const libc::c_char,
        b"Prints\0" as *const u8 as *const libc::c_char,
        b"Prints for internal use only\0" as *const u8 as *const libc::c_char,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READWRITE as libc::c_int)
            as GParamFlags,
    );
    g_object_class_install_properties(
        object_class,
        N_PROPS as libc::c_int as guint,
        properties.as_mut_ptr(),
    );
}
unsafe extern "C" fn fp_print_init(mut self_0: *mut FpPrint) {}
#[no_mangle]
pub unsafe extern "C" fn fp_print_new(mut device: *mut FpDevice) -> *mut FpPrint {
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
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"fp_print_new\0"))
                .as_ptr(),
            b"device\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FpPrint;
    }
    return g_object_new(
        fp_print_get_type(),
        b"driver\0" as *const u8 as *const libc::c_char,
        fp_device_get_driver(device),
        b"device-id\0" as *const u8 as *const libc::c_char,
        fp_device_get_device_id(device),
        0 as *mut libc::c_void,
    ) as *mut FpPrint;
}
#[no_mangle]
pub unsafe extern "C" fn fp_print_get_driver(mut print: *mut FpPrint) -> *const gchar {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(print as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fp_print_get_driver\0"))
                .as_ptr(),
            b"FP_IS_PRINT (print)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *const gchar;
    }
    return (*print).driver;
}
#[no_mangle]
pub unsafe extern "C" fn fp_print_get_device_id(
    mut print: *mut FpPrint,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(print as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"fp_print_get_device_id\0"))
                .as_ptr(),
            b"FP_IS_PRINT (print)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *const gchar;
    }
    return (*print).device_id;
}
#[no_mangle]
pub unsafe extern "C" fn fp_print_get_device_stored(
    mut print: *mut FpPrint,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(print as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"fp_print_get_device_stored\0"))
                .as_ptr(),
            b"FP_IS_PRINT (print)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    return (*print).device_stored;
}
#[no_mangle]
pub unsafe extern "C" fn fp_print_get_image(mut print: *mut FpPrint) -> *mut FpImage {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(print as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"fp_print_get_image\0"))
                .as_ptr(),
            b"FP_IS_PRINT (print)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FpImage;
    }
    return (*print).image;
}
#[no_mangle]
pub unsafe extern "C" fn fp_print_get_finger(mut print: *mut FpPrint) -> FpFinger {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(print as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fp_print_get_finger\0"))
                .as_ptr(),
            b"FP_IS_PRINT (print)\0" as *const u8 as *const libc::c_char,
        );
        return FP_FINGER_UNKNOWN;
    }
    return (*print).finger;
}
#[no_mangle]
pub unsafe extern "C" fn fp_print_get_username(mut print: *mut FpPrint) -> *const gchar {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(print as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fp_print_get_username\0"))
                .as_ptr(),
            b"FP_IS_PRINT (print)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *const gchar;
    }
    return (*print).username;
}
#[no_mangle]
pub unsafe extern "C" fn fp_print_get_description(
    mut print: *mut FpPrint,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(print as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"fp_print_get_description\0"))
                .as_ptr(),
            b"FP_IS_PRINT (print)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *const gchar;
    }
    return (*print).description;
}
#[no_mangle]
pub unsafe extern "C" fn fp_print_get_enroll_date(
    mut print: *mut FpPrint,
) -> *const GDate {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(print as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"fp_print_get_enroll_date\0"))
                .as_ptr(),
            b"FP_IS_PRINT (print)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *const GDate;
    }
    return (*print).enroll_date;
}
#[no_mangle]
pub unsafe extern "C" fn fp_print_set_finger(
    mut print: *mut FpPrint,
    mut finger: FpFinger,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(print as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fp_print_set_finger\0"))
                .as_ptr(),
            b"FP_IS_PRINT (print)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*print).finger = finger;
    g_object_notify_by_pspec(
        g_type_check_instance_cast(
            print as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        properties[PROP_FINGER as libc::c_int as usize],
    );
}
#[no_mangle]
pub unsafe extern "C" fn fp_print_set_username(
    mut print: *mut FpPrint,
    mut username: *const gchar,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(print as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fp_print_set_username\0"))
                .as_ptr(),
            b"FP_IS_PRINT (print)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut _pp: C2RustUnnamed_16 = C2RustUnnamed_16 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*print).username as *mut *mut gchar as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    (*print).username = g_strdup(username);
    g_object_notify_by_pspec(
        g_type_check_instance_cast(
            print as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        properties[PROP_USERNAME as libc::c_int as usize],
    );
}
#[no_mangle]
pub unsafe extern "C" fn fp_print_set_description(
    mut print: *mut FpPrint,
    mut description: *const gchar,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(print as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"fp_print_set_description\0"))
                .as_ptr(),
            b"FP_IS_PRINT (print)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut _pp: C2RustUnnamed_17 = C2RustUnnamed_17 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*print).description as *mut *mut gchar as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    (*print).description = g_strdup(description);
    g_object_notify_by_pspec(
        g_type_check_instance_cast(
            print as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        properties[PROP_DESCRIPTION as libc::c_int as usize],
    );
}
#[no_mangle]
pub unsafe extern "C" fn fp_print_set_enroll_date(
    mut print: *mut FpPrint,
    mut enroll_date: *const GDate,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(print as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"fp_print_set_enroll_date\0"))
                .as_ptr(),
            b"FP_IS_PRINT (print)\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut _pp: C2RustUnnamed_18 = C2RustUnnamed_18 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GDate) -> ()>,
        GDestroyNotify,
    >(Some(g_date_free as unsafe extern "C" fn(*mut GDate) -> ()));
    _pp.in_0 = &mut (*print).enroll_date as *mut *mut GDate as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    if !enroll_date.is_null() {
        (*print).enroll_date = g_date_copy(enroll_date);
    }
    g_object_notify_by_pspec(
        g_type_check_instance_cast(
            print as *mut GTypeInstance,
            ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        ) as *mut libc::c_void as *mut GObject,
        properties[PROP_ENROLL_DATE as libc::c_int as usize],
    );
}
#[no_mangle]
pub unsafe extern "C" fn fp_print_compatible(
    mut self_0: *mut FpPrint,
    mut device: *mut FpDevice,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(self_0 as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fp_print_compatible\0"))
                .as_ptr(),
            b"FP_IS_PRINT (self)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
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
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"fp_print_compatible\0"))
                .as_ptr(),
            b"FP_IS_DEVICE (device)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if g_strcmp0((*self_0).driver, fp_device_get_driver(device)) != 0 {
        return 0 as libc::c_int;
    }
    if g_strcmp0((*self_0).device_id, fp_device_get_device_id(device)) != 0 {
        return 0 as libc::c_int;
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fp_print_equal(
    mut self_0: *mut FpPrint,
    mut other: *mut FpPrint,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(self_0 as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"fp_print_equal\0"))
                .as_ptr(),
            b"FP_IS_PRINT (self)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if FP_IS_PRINT(other as gpointer) != 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"fp_print_equal\0"))
                .as_ptr(),
            b"FP_IS_PRINT (other)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*self_0).type_0 as libc::c_uint
            != FPI_PRINT_UNDEFINED as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"fp_print_equal\0"))
                .as_ptr(),
            b"self->type != FPI_PRINT_UNDEFINED\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*other).type_0 as libc::c_uint
            != FPI_PRINT_UNDEFINED as libc::c_int as libc::c_uint
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_return_if_fail_warning(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"fp_print_equal\0"))
                .as_ptr(),
            b"other->type != FPI_PRINT_UNDEFINED\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if (*self_0).type_0 as libc::c_uint != (*other).type_0 as libc::c_uint {
        return 0 as libc::c_int;
    }
    if g_strcmp0((*self_0).driver, (*other).driver) != 0 {
        return 0 as libc::c_int;
    }
    if g_strcmp0((*self_0).device_id, (*other).device_id) != 0 {
        return 0 as libc::c_int;
    }
    if (*self_0).type_0 as libc::c_uint == FPI_PRINT_RAW as libc::c_int as libc::c_uint {
        return g_variant_equal(
            (*self_0).data as gconstpointer,
            (*other).data as gconstpointer,
        )
    } else if (*self_0).type_0 as libc::c_uint
        == FPI_PRINT_NBIS as libc::c_int as libc::c_uint
    {
        let mut i: guint = 0;
        if (*(*self_0).prints).len != (*(*other).prints).len {
            return 0 as libc::c_int;
        }
        i = 0 as libc::c_int as guint;
        while i < (*(*self_0).prints).len {
            let mut a: *mut xyt_struct = *((*(*self_0).prints).pdata).offset(i as isize)
                as *mut xyt_struct;
            let mut b: *mut xyt_struct = *((*(*other).prints).pdata).offset(i as isize)
                as *mut xyt_struct;
            if memcmp(
                a as *const libc::c_void,
                b as *const libc::c_void,
                ::core::mem::size_of::<xyt_struct>() as libc::c_ulong,
            ) != 0 as libc::c_int
            {
                return 0 as libc::c_int;
            }
            i = i.wrapping_add(1);
        }
        return (0 as libc::c_int == 0) as libc::c_int;
    } else {
        g_assertion_message_expr(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-print.c\0" as *const u8 as *const libc::c_char,
            632 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"fp_print_equal\0"))
                .as_ptr(),
            0 as *const libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn fp_print_serialize(
    mut print: *mut FpPrint,
    mut data: *mut *mut guchar,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut result: GVariant_autoptr = 0 as GVariant_autoptr;
    let mut builder: GVariantBuilder = {
        let mut init = _GVariantBuilder {
            u: C2RustUnnamed {
                s: {
                    let mut init = C2RustUnnamed_0 {
                        partial_magic: 2942751021 as libc::c_uint as gsize,
                        type_0: g_variant_type_checked_(
                            b"(issbymsmsia{sv}v)\0" as *const u8 as *const libc::c_char,
                        ),
                        y: [
                            0 as libc::c_int as gsize,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ],
                    };
                    init
                },
            },
        };
        init
    };
    let mut len: gsize = 0;
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
        g_assertion_message_expr(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-print.c\0" as *const u8 as *const libc::c_char,
            662 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"fp_print_serialize\0"))
                .as_ptr(),
            b"data\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !length.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-print.c\0" as *const u8 as *const libc::c_char,
            663 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"fp_print_serialize\0"))
                .as_ptr(),
            b"length\0" as *const u8 as *const libc::c_char,
        );
    }
    g_variant_builder_add(
        &mut builder as *mut GVariantBuilder,
        b"i\0" as *const u8 as *const libc::c_char,
        (*print).type_0 as libc::c_uint,
    );
    g_variant_builder_add(
        &mut builder as *mut GVariantBuilder,
        b"s\0" as *const u8 as *const libc::c_char,
        (*print).driver,
    );
    g_variant_builder_add(
        &mut builder as *mut GVariantBuilder,
        b"s\0" as *const u8 as *const libc::c_char,
        (*print).device_id,
    );
    g_variant_builder_add(
        &mut builder as *mut GVariantBuilder,
        b"b\0" as *const u8 as *const libc::c_char,
        (*print).device_stored,
    );
    g_variant_builder_add(
        &mut builder as *mut GVariantBuilder,
        b"y\0" as *const u8 as *const libc::c_char,
        (*print).finger as libc::c_uint,
    );
    g_variant_builder_add(
        &mut builder as *mut GVariantBuilder,
        b"ms\0" as *const u8 as *const libc::c_char,
        (*print).username,
    );
    g_variant_builder_add(
        &mut builder as *mut GVariantBuilder,
        b"ms\0" as *const u8 as *const libc::c_char,
        (*print).description,
    );
    if !((*print).enroll_date).is_null() && g_date_valid((*print).enroll_date) != 0 {
        g_variant_builder_add(
            &mut builder as *mut GVariantBuilder,
            b"i\0" as *const u8 as *const libc::c_char,
            g_date_get_julian((*print).enroll_date),
        );
    } else {
        g_variant_builder_add(
            &mut builder as *mut GVariantBuilder,
            b"i\0" as *const u8 as *const libc::c_char,
            -(0x7fffffff as libc::c_int) - 1 as libc::c_int,
        );
    }
    g_variant_builder_open(
        &mut builder,
        b"a{sv}\0" as *const u8 as *const libc::c_char as *const GVariantType,
    );
    g_variant_builder_close(&mut builder);
    if (*print).type_0 as libc::c_uint == FPI_PRINT_NBIS as libc::c_int as libc::c_uint {
        let mut nested: GVariantBuilder = {
            let mut init = _GVariantBuilder {
                u: C2RustUnnamed {
                    s: {
                        let mut init = C2RustUnnamed_0 {
                            partial_magic: 2942751021 as libc::c_uint as gsize,
                            type_0: g_variant_type_checked_(
                                b"(a(aiaiai))\0" as *const u8 as *const libc::c_char,
                            ),
                            y: [
                                0 as libc::c_int as gsize,
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                            ],
                        };
                        init
                    },
                },
            };
            init
        };
        let mut i: guint = 0;
        g_variant_builder_open(
            &mut nested,
            g_variant_type_checked_(b"a(aiaiai)\0" as *const u8 as *const libc::c_char),
        );
        i = 0 as libc::c_int as guint;
        while i < (*(*print).prints).len {
            let mut xyt: *mut xyt_struct = *((*(*print).prints).pdata).offset(i as isize)
                as *mut xyt_struct;
            g_variant_builder_open(
                &mut nested,
                g_variant_type_checked_(
                    b"(aiaiai)\0" as *const u8 as *const libc::c_char,
                ),
            );
            g_variant_builder_add_value(
                &mut nested,
                g_variant_new_fixed_array(
                    b"i\0" as *const u8 as *const libc::c_char as *const GVariantType,
                    ((*xyt).xcol).as_mut_ptr() as gconstpointer,
                    (*xyt).nrows as gsize,
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ),
            );
            g_variant_builder_add_value(
                &mut nested,
                g_variant_new_fixed_array(
                    b"i\0" as *const u8 as *const libc::c_char as *const GVariantType,
                    ((*xyt).ycol).as_mut_ptr() as gconstpointer,
                    (*xyt).nrows as gsize,
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ),
            );
            g_variant_builder_add_value(
                &mut nested,
                g_variant_new_fixed_array(
                    b"i\0" as *const u8 as *const libc::c_char as *const GVariantType,
                    ((*xyt).thetacol).as_mut_ptr() as gconstpointer,
                    (*xyt).nrows as gsize,
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ),
            );
            g_variant_builder_close(&mut nested);
            i = i.wrapping_add(1);
        }
        g_variant_builder_close(&mut nested);
        g_variant_builder_add(
            &mut builder as *mut GVariantBuilder,
            b"v\0" as *const u8 as *const libc::c_char,
            g_variant_builder_end(&mut nested),
        );
    } else {
        g_variant_builder_add(
            &mut builder as *mut GVariantBuilder,
            b"v\0" as *const u8 as *const libc::c_char,
            g_variant_new_variant((*print).data),
        );
    }
    result = g_variant_builder_end(&mut builder);
    if 1234 as libc::c_int == 4321 as libc::c_int {
        let mut tmp: *mut GVariant = 0 as *mut GVariant;
        tmp = g_variant_byteswap(result);
        g_variant_unref(result);
        result = tmp;
    }
    len = g_variant_get_size(result);
    len = (len as libc::c_ulong).wrapping_add(3 as libc::c_int as libc::c_ulong) as gsize
        as gsize;
    *data = g_malloc(len) as *mut guchar;
    *length = len;
    *(*data).offset(0 as libc::c_int as isize) = 'F' as i32 as guchar;
    *(*data).offset(1 as libc::c_int as isize) = 'P' as i32 as guchar;
    *(*data).offset(2 as libc::c_int as isize) = '3' as i32 as guchar;
    g_variant_get_data(result);
    g_variant_store(result, (*data).offset(3 as libc::c_int as isize) as gpointer);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fp_print_deserialize(
    mut data: *const guchar,
    mut length: gsize,
    mut error: *mut *mut GError,
) -> *mut FpPrint {
    let mut current_block: u64;
    let mut result: FpPrint_autoptr = 0 as FpPrint_autoptr;
    let mut raw_value: GVariant_autoptr = 0 as GVariant_autoptr;
    let mut value: GVariant_autoptr = 0 as GVariant_autoptr;
    let mut print_data: GVariant_autoptr = 0 as GVariant_autoptr;
    let mut date: GDate_autoptr = 0 as GDate_autoptr;
    let mut aligned_data: *mut guchar = 0 as *mut guchar;
    let mut finger_int8: guint8 = 0;
    let mut finger: FpFinger = FP_FINGER_UNKNOWN;
    let mut username: *mut gchar = 0 as *mut gchar;
    let mut description: *mut gchar = 0 as *mut gchar;
    let mut julian_date: gint = 0;
    let mut type_0: FpiPrintType = FPI_PRINT_UNDEFINED;
    let mut driver: *const gchar = 0 as *const gchar;
    let mut device_id: *const gchar = 0 as *const gchar;
    let mut device_stored: gboolean = 0;
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
        g_assertion_message_expr(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-print.c\0" as *const u8 as *const libc::c_char,
            780 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fp_print_deserialize\0"))
                .as_ptr(),
            b"data\0" as *const u8 as *const libc::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if length > 3 as libc::c_int as libc::c_ulong {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint-print\0" as *const u8 as *const libc::c_char,
            b"../libfprint/fp-print.c\0" as *const u8 as *const libc::c_char,
            781 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"fp_print_deserialize\0"))
                .as_ptr(),
            b"length > 3\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(memcmp(
        data as *const libc::c_void,
        b"FP3\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        3 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int)
    {
        aligned_data = g_malloc(length.wrapping_sub(3 as libc::c_int as libc::c_ulong))
            as *mut guchar;
        memcpy(
            aligned_data as *mut libc::c_void,
            data.offset(3 as libc::c_int as isize) as *const libc::c_void,
            length.wrapping_sub(3 as libc::c_int as libc::c_ulong),
        );
        raw_value = g_variant_new_from_data(
            g_variant_type_checked_(
                b"(issbymsmsia{sv}v)\0" as *const u8 as *const libc::c_char,
            ),
            aligned_data as gconstpointer,
            length.wrapping_sub(3 as libc::c_int as libc::c_ulong),
            0 as libc::c_int,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            aligned_data as gpointer,
        );
        if !raw_value.is_null() {
            if 1234 as libc::c_int == 4321 as libc::c_int {
                value = g_variant_byteswap(raw_value);
            } else {
                value = g_variant_get_normal_form(raw_value);
            }
            g_variant_get(
                value,
                b"(i&s&sbymsmsi@a{sv}v)\0" as *const u8 as *const libc::c_char,
                &mut type_0 as *mut FpiPrintType,
                &mut driver as *mut *const gchar,
                &mut device_id as *mut *const gchar,
                &mut device_stored as *mut gboolean,
                &mut finger_int8 as *mut guint8,
                &mut username as *mut *mut gchar,
                &mut description as *mut *mut gchar,
                &mut julian_date as *mut gint,
                0 as *mut libc::c_void,
                &mut print_data as *mut GVariant_autoptr,
            );
            finger = finger_int8 as FpFinger;
            if type_0 as libc::c_uint == FPI_PRINT_NBIS as libc::c_int as libc::c_uint {
                let mut prints: GVariant_autoptr = g_variant_get_child_value(
                    print_data,
                    0 as libc::c_int as gsize,
                );
                let mut i: guint = 0;
                result = g_object_new(
                    fp_print_get_type(),
                    b"driver\0" as *const u8 as *const libc::c_char,
                    driver,
                    b"device-id\0" as *const u8 as *const libc::c_char,
                    device_id,
                    b"device-stored\0" as *const u8 as *const libc::c_char,
                    device_stored,
                    0 as *mut libc::c_void,
                ) as FpPrint_autoptr;
                g_object_ref_sink(result as gpointer);
                fpi_print_set_type(result, FPI_PRINT_NBIS);
                i = 0 as libc::c_int as guint;
                loop {
                    if !((i as libc::c_ulong) < g_variant_n_children(prints)) {
                        current_block = 1345366029464561491;
                        break;
                    }
                    let mut xyt: *mut xyt_struct = 0 as *mut xyt_struct;
                    let mut xcol: *const gint32 = 0 as *const gint32;
                    let mut ycol: *const gint32 = 0 as *const gint32;
                    let mut thetacol: *const gint32 = 0 as *const gint32;
                    let mut xlen: gsize = 0;
                    let mut ylen: gsize = 0;
                    let mut thetalen: gsize = 0;
                    let mut xyt_data: GVariant_autoptr = 0 as GVariant_autoptr;
                    let mut child: *mut GVariant = 0 as *mut GVariant;
                    xyt_data = g_variant_get_child_value(prints, i as gsize);
                    child = g_variant_get_child_value(
                        xyt_data,
                        0 as libc::c_int as gsize,
                    );
                    xcol = g_variant_get_fixed_array(
                        child,
                        &mut xlen,
                        ::core::mem::size_of::<gint32>() as libc::c_ulong,
                    ) as *const gint32;
                    g_variant_unref(child);
                    child = g_variant_get_child_value(
                        xyt_data,
                        1 as libc::c_int as gsize,
                    );
                    ycol = g_variant_get_fixed_array(
                        child,
                        &mut ylen,
                        ::core::mem::size_of::<gint32>() as libc::c_ulong,
                    ) as *const gint32;
                    g_variant_unref(child);
                    child = g_variant_get_child_value(
                        xyt_data,
                        2 as libc::c_int as gsize,
                    );
                    thetacol = g_variant_get_fixed_array(
                        child,
                        &mut thetalen,
                        ::core::mem::size_of::<gint32>() as libc::c_ulong,
                    ) as *const gint32;
                    g_variant_unref(child);
                    if xlen != ylen || xlen != thetalen {
                        current_block = 13591168841217423541;
                        break;
                    }
                    if xlen
                        > (::core::mem::size_of::<[libc::c_int; 200]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            )
                    {
                        current_block = 13591168841217423541;
                        break;
                    }
                    xyt = ({
                        let mut __n: gsize = 1 as libc::c_int as gsize;
                        let mut __s: gsize = ::core::mem::size_of::<xyt_struct>()
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
                    }) as *mut xyt_struct;
                    (*xyt).nrows = xlen as libc::c_int;
                    memcpy(
                        ((*xyt).xcol).as_mut_ptr() as *mut libc::c_void,
                        xcol as *const libc::c_void,
                        (::core::mem::size_of::<gint32>() as libc::c_ulong)
                            .wrapping_mul(xlen),
                    );
                    memcpy(
                        ((*xyt).ycol).as_mut_ptr() as *mut libc::c_void,
                        ycol as *const libc::c_void,
                        (::core::mem::size_of::<gint32>() as libc::c_ulong)
                            .wrapping_mul(xlen),
                    );
                    memcpy(
                        ((*xyt).thetacol).as_mut_ptr() as *mut libc::c_void,
                        thetacol as *const libc::c_void,
                        (::core::mem::size_of::<gint32>() as libc::c_ulong)
                            .wrapping_mul(xlen),
                    );
                    g_ptr_array_add(
                        (*result).prints,
                        if 0 as libc::c_int != 0 {
                            xyt as *mut libc::c_void
                        } else {
                            g_steal_pointer(&mut xyt as *mut *mut xyt_struct as gpointer)
                        },
                    );
                    i = i.wrapping_add(1);
                }
            } else if type_0 as libc::c_uint
                == FPI_PRINT_RAW as libc::c_int as libc::c_uint
            {
                let mut fp_data: GVariant_autoptr = g_variant_get_child_value(
                    print_data,
                    0 as libc::c_int as gsize,
                );
                result = g_object_new(
                    fp_print_get_type(),
                    b"fpi-type\0" as *const u8 as *const libc::c_char,
                    type_0 as libc::c_uint,
                    b"driver\0" as *const u8 as *const libc::c_char,
                    driver,
                    b"device-id\0" as *const u8 as *const libc::c_char,
                    device_id,
                    b"device-stored\0" as *const u8 as *const libc::c_char,
                    device_stored,
                    b"fpi-data\0" as *const u8 as *const libc::c_char,
                    fp_data,
                    0 as *mut libc::c_void,
                ) as FpPrint_autoptr;
                g_object_ref_sink(result as gpointer);
                current_block = 1345366029464561491;
            } else {
                g_log(
                    b"libfprint-print\0" as *const u8 as *const libc::c_char,
                    G_LOG_LEVEL_WARNING,
                    b"Invalid print type: 0x%X\0" as *const u8 as *const libc::c_char,
                    type_0 as libc::c_uint,
                );
                current_block = 13591168841217423541;
            }
            match current_block {
                13591168841217423541 => {}
                _ => {
                    date = g_date_new_julian(julian_date as guint32);
                    g_object_set(
                        result as gpointer,
                        b"finger\0" as *const u8 as *const libc::c_char,
                        finger as libc::c_uint,
                        b"username\0" as *const u8 as *const libc::c_char,
                        username,
                        b"description\0" as *const u8 as *const libc::c_char,
                        description,
                        b"enroll_date\0" as *const u8 as *const libc::c_char,
                        date,
                        0 as *mut libc::c_void,
                    );
                    return (if 0 as libc::c_int != 0 {
                        result as *mut libc::c_void
                    } else {
                        g_steal_pointer(&mut result as *mut FpPrint_autoptr as gpointer)
                    }) as *mut FpPrint;
                }
            }
        }
    }
    g_set_error(
        error,
        g_io_error_quark(),
        G_IO_ERROR_INVALID_DATA as libc::c_int,
        b"Data could not be parsed\0" as *const u8 as *const libc::c_char,
    );
    return 0 as *mut FpPrint;
}
