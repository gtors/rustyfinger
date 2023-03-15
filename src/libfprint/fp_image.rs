use ::libc;
extern "C" {
    pub type _GData;
    pub type _GTimer;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GTask;
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn g_ptr_array_new_full(
        reserved_size: guint,
        element_free_func: GDestroyNotify,
    ) -> *mut GPtrArray;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter(location: *mut libc::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut libc::c_void, result: gsize);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_timer_new() -> *mut GTimer;
    fn g_timer_stop(timer: *mut GTimer);
    fn g_timer_elapsed(timer: *mut GTimer, microseconds: *mut gulong) -> gdouble;
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
    fn g_type_check_class_cast(
        g_class: *mut GTypeClass,
        is_a_type: GType,
    ) -> *mut GTypeClass;
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
    fn g_object_unref(object: gpointer);
    fn g_param_spec_uint(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: guint,
        maximum: guint,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    fn g_value_get_uint(value: *const GValue) -> guint;
    fn g_io_error_quark() -> GQuark;
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
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_new_error(
        task: *mut GTask,
        domain: GQuark,
        code: gint,
        format: *const libc::c_char,
        _: ...
    );
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn g_task_had_error(task: *mut GTask) -> gboolean;
    fn free_minutia(_: *mut MINUTIA);
    fn free_minutiae(_: *mut MINUTIAE);
    static mut g_lfsparms_V2: LFSPARMS;
    fn get_minutiae(
        _: *mut *mut MINUTIAE,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut *mut libc::c_uchar,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_double,
        _: *const LFSPARMS,
    ) -> libc::c_int;
}
pub type guint8 = libc::c_uchar;
pub type guint16 = libc::c_ushort;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
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
pub type GTimer = _GTimer;
pub type GTimer_autoptr = *mut GTimer;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fp_minutia {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub ex: libc::c_int,
    pub ey: libc::c_int,
    pub direction: libc::c_int,
    pub reliability: libc::c_double,
    pub type_0: libc::c_int,
    pub appearing: libc::c_int,
    pub feature_id: libc::c_int,
    pub nbrs: *mut libc::c_int,
    pub ridge_counts: *mut libc::c_int,
    pub num_nbrs: libc::c_int,
}
pub type FpMinutia = fp_minutia;
pub type FpImage = _FpImage;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpImageClass {
    pub parent_class: GObjectClass,
}
pub const N_PROPS: C2RustUnnamed_10 = 3;
pub const PROP_HEIGHT: C2RustUnnamed_10 = 2;
pub const PROP_WIDTH: C2RustUnnamed_10 = 1;
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
pub union C2RustUnnamed_3 {
    pub in_0: *mut libc::c_char,
    pub out: *mut gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fp_minutiae {
    pub alloc: libc::c_int,
    pub num: libc::c_int,
    pub list: *mut *mut fp_minutia,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DetectMinutiaeData {
    pub user_cb: GAsyncReadyCallback,
    pub minutiae: *mut fp_minutiae,
    pub width: gint,
    pub height: gint,
    pub ppmm: gdouble,
    pub flags: FpiImageFlags,
    pub image: *mut guchar,
    pub binarized: *mut guchar,
}
pub type LFSPARMS = g_lfsparms;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct g_lfsparms {
    pub pad_value: libc::c_int,
    pub join_line_radius: libc::c_int,
    pub blocksize: libc::c_int,
    pub windowsize: libc::c_int,
    pub windowoffset: libc::c_int,
    pub num_directions: libc::c_int,
    pub start_dir_angle: libc::c_double,
    pub rmv_valid_nbr_min: libc::c_int,
    pub dir_strength_min: libc::c_double,
    pub dir_distance_max: libc::c_int,
    pub smth_valid_nbr_min: libc::c_int,
    pub vort_valid_nbr_min: libc::c_int,
    pub highcurv_vorticity_min: libc::c_int,
    pub highcurv_curvature_min: libc::c_int,
    pub min_interpolate_nbrs: libc::c_int,
    pub percentile_min_max: libc::c_int,
    pub min_contrast_delta: libc::c_int,
    pub num_dft_waves: libc::c_int,
    pub powmax_min: libc::c_double,
    pub pownorm_min: libc::c_double,
    pub powmax_max: libc::c_double,
    pub fork_interval: libc::c_int,
    pub fork_pct_powmax: libc::c_double,
    pub fork_pct_pownorm: libc::c_double,
    pub dirbin_grid_w: libc::c_int,
    pub dirbin_grid_h: libc::c_int,
    pub isobin_grid_dim: libc::c_int,
    pub num_fill_holes: libc::c_int,
    pub max_minutia_delta: libc::c_int,
    pub max_high_curve_theta: libc::c_double,
    pub high_curve_half_contour: libc::c_int,
    pub min_loop_len: libc::c_int,
    pub min_loop_aspect_dist: libc::c_double,
    pub min_loop_aspect_ratio: libc::c_double,
    pub link_table_dim: libc::c_int,
    pub max_link_dist: libc::c_int,
    pub min_theta_dist: libc::c_int,
    pub maxtrans: libc::c_int,
    pub score_theta_norm: libc::c_double,
    pub score_dist_norm: libc::c_double,
    pub score_dist_weight: libc::c_double,
    pub score_numerator: libc::c_double,
    pub max_rmtest_dist: libc::c_int,
    pub max_hook_len: libc::c_int,
    pub max_half_loop: libc::c_int,
    pub trans_dir_pix: libc::c_int,
    pub small_loop_len: libc::c_int,
    pub side_half_contour: libc::c_int,
    pub inv_block_margin: libc::c_int,
    pub rm_valid_nbr_min: libc::c_int,
    pub max_overlap_dist: libc::c_int,
    pub max_overlap_join_dist: libc::c_int,
    pub malformation_steps_1: libc::c_int,
    pub malformation_steps_2: libc::c_int,
    pub min_malformation_ratio: libc::c_double,
    pub max_malformation_dist: libc::c_int,
    pub pores_trans_r: libc::c_int,
    pub pores_perp_steps: libc::c_int,
    pub pores_steps_fwd: libc::c_int,
    pub pores_steps_bwd: libc::c_int,
    pub pores_min_dist2: libc::c_double,
    pub pores_max_ratio: libc::c_double,
    pub remove_perimeter_pts: libc::c_int,
    pub min_pp_distance: libc::c_int,
    pub max_nbrs: libc::c_int,
    pub max_ridge_steps: libc::c_int,
}
pub type MINUTIAE = fp_minutiae;
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
pub type MINUTIA = fp_minutia;
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
pub type C2RustUnnamed_10 = libc::c_uint;
pub const PROP_0: C2RustUnnamed_10 = 0;
#[inline]
unsafe extern "C" fn g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = 0 as *mut libc::c_void;
    ref_0 = *ptr;
    *ptr = 0 as *mut libc::c_void;
    return ref_0;
}
#[inline]
unsafe extern "C" fn FP_IMAGE(mut ptr: gpointer) -> *mut FpImage {
    return g_type_check_instance_cast(ptr as *mut GTypeInstance, fp_image_get_type())
        as *mut libc::c_void as *mut FpImage;
}
static mut FpImage_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn fp_image_get_type() -> GType {
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
        let mut g_define_type_id: GType = fp_image_get_type_once();
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
unsafe extern "C" fn fp_image_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
        g_intern_static_string(b"FpImage\0" as *const u8 as *const libc::c_char),
        ::core::mem::size_of::<FpImageClass>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GClassInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(Some(fp_image_class_intern_init as unsafe extern "C" fn(gpointer) -> ())),
        ),
        ::core::mem::size_of::<FpImage>() as libc::c_ulong as guint,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            GInstanceInitFunc,
        >(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut FpImage) -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(Some(fp_image_init as unsafe extern "C" fn(*mut FpImage) -> ())),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut fp_image_parent_class: gpointer = 0 as *const libc::c_void
    as *mut libc::c_void;
unsafe extern "C" fn fp_image_class_intern_init(mut klass: gpointer) {
    fp_image_parent_class = g_type_class_peek_parent(klass);
    if FpImage_private_offset != 0 as libc::c_int {
        g_type_class_adjust_private_offset(klass, &mut FpImage_private_offset);
    }
    fp_image_class_init(klass as *mut FpImageClass);
}
static mut properties: [*mut GParamSpec; 3] = [0 as *const GParamSpec
    as *mut GParamSpec; 3];
#[no_mangle]
pub unsafe extern "C" fn fp_image_new(
    mut width: gint,
    mut height: gint,
) -> *mut FpImage {
    return g_object_new(
        fp_image_get_type(),
        b"width\0" as *const u8 as *const libc::c_char,
        width,
        b"height\0" as *const u8 as *const libc::c_char,
        height,
        0 as *mut libc::c_void,
    ) as *mut FpImage;
}
unsafe extern "C" fn fp_image_finalize(mut object: *mut GObject) {
    let mut self_0: *mut FpImage = object as *mut FpImage;
    let mut _pp: C2RustUnnamed_3 = C2RustUnnamed_3 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*self_0).data as *mut *mut guint8 as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    let mut _pp_0: C2RustUnnamed_2 = C2RustUnnamed_2 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_0: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp_0.in_0 = &mut (*self_0).binarized as *mut *mut guint8 as *mut libc::c_char;
    _p_0 = *_pp_0.out;
    if !_p_0.is_null() {
        *_pp_0.out = 0 as *mut libc::c_void;
        _destroy_0.expect("non-null function pointer")(_p_0);
    }
    let mut _pp_1: C2RustUnnamed_1 = C2RustUnnamed_1 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_1: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_1: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut GPtrArray) -> ()>,
        GDestroyNotify,
    >(Some(g_ptr_array_unref as unsafe extern "C" fn(*mut GPtrArray) -> ()));
    _pp_1.in_0 = &mut (*self_0).minutiae as *mut *mut GPtrArray as *mut libc::c_char;
    _p_1 = *_pp_1.out;
    if !_p_1.is_null() {
        *_pp_1.out = 0 as *mut libc::c_void;
        _destroy_1.expect("non-null function pointer")(_p_1);
    }
    ((*(g_type_check_class_cast(
        fp_image_parent_class as *mut GTypeClass,
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
    ) as *mut libc::c_void as *mut GObjectClass))
        .finalize)
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn fp_image_constructed(mut object: *mut GObject) {
    let mut self_0: *mut FpImage = object as *mut FpImage;
    (*self_0)
        .data = g_malloc0(((*self_0).width).wrapping_mul((*self_0).height) as gsize)
        as *mut guint8;
}
unsafe extern "C" fn fp_image_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut FpImage = FP_IMAGE(object as gpointer);
    match prop_id {
        1 => {
            g_value_set_uint(value, (*self_0).width);
        }
        2 => {
            g_value_set_uint(value, (*self_0).height);
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                b"libfprint-image\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const libc::c_char,
                b"../libfprint/fp-image.c\0" as *const u8 as *const libc::c_char,
                98 as libc::c_int,
                b"property\0" as *const u8 as *const libc::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn fp_image_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut FpImage = FP_IMAGE(object as gpointer);
    match prop_id {
        1 => {
            (*self_0).width = g_value_get_uint(value);
        }
        2 => {
            (*self_0).height = g_value_get_uint(value);
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                b"libfprint-image\0" as *const u8 as *const libc::c_char,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const libc::c_char,
                b"../libfprint/fp-image.c\0" as *const u8 as *const libc::c_char,
                121 as libc::c_int,
                b"property\0" as *const u8 as *const libc::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn fp_image_class_init(mut klass: *mut FpImageClass) {
    let mut object_class: *mut GObjectClass = g_type_check_class_cast(
        klass as *mut GTypeClass,
        ((20 as libc::c_int) << 2 as libc::c_int) as GType,
    ) as *mut libc::c_void as *mut GObjectClass;
    (*object_class)
        .finalize = Some(fp_image_finalize as unsafe extern "C" fn(*mut GObject) -> ());
    (*object_class)
        .constructed = Some(
        fp_image_constructed as unsafe extern "C" fn(*mut GObject) -> (),
    );
    (*object_class)
        .set_property = Some(
        fp_image_set_property
            as unsafe extern "C" fn(
                *mut GObject,
                guint,
                *const GValue,
                *mut GParamSpec,
            ) -> (),
    );
    (*object_class)
        .get_property = Some(
        fp_image_get_property
            as unsafe extern "C" fn(
                *mut GObject,
                guint,
                *mut GValue,
                *mut GParamSpec,
            ) -> (),
    );
    properties[PROP_WIDTH as libc::c_int
        as usize] = g_param_spec_uint(
        b"width\0" as *const u8 as *const libc::c_char,
        b"Width\0" as *const u8 as *const libc::c_char,
        b"The width of the image\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as guint,
        0xffff as libc::c_int as guint16 as guint,
        0 as libc::c_int as guint,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READWRITE as libc::c_int
            | G_PARAM_CONSTRUCT_ONLY as libc::c_int) as GParamFlags,
    );
    properties[PROP_HEIGHT as libc::c_int
        as usize] = g_param_spec_uint(
        b"height\0" as *const u8 as *const libc::c_char,
        b"Height\0" as *const u8 as *const libc::c_char,
        b"The height of the image\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as guint,
        0xffff as libc::c_int as guint16 as guint,
        0 as libc::c_int as guint,
        (G_PARAM_STATIC_NAME as libc::c_int | G_PARAM_STATIC_NICK as libc::c_int
            | G_PARAM_STATIC_BLURB as libc::c_int | G_PARAM_READWRITE as libc::c_int
            | G_PARAM_CONSTRUCT_ONLY as libc::c_int) as GParamFlags,
    );
    g_object_class_install_properties(
        object_class,
        N_PROPS as libc::c_int as guint,
        properties.as_mut_ptr(),
    );
}
unsafe extern "C" fn fp_image_init(mut self_0: *mut FpImage) {}
unsafe extern "C" fn fp_image_detect_minutiae_free(mut data: *mut DetectMinutiaeData) {
    let mut _pp: C2RustUnnamed_6 = C2RustUnnamed_6 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p: gpointer = 0 as *mut libc::c_void;
    let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp.in_0 = &mut (*data).image as *mut *mut guchar as *mut libc::c_char;
    _p = *_pp.out;
    if !_p.is_null() {
        *_pp.out = 0 as *mut libc::c_void;
        _destroy.expect("non-null function pointer")(_p);
    }
    let mut _pp_0: C2RustUnnamed_5 = C2RustUnnamed_5 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_0: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut MINUTIAE) -> ()>,
        GDestroyNotify,
    >(Some(free_minutiae as unsafe extern "C" fn(*mut MINUTIAE) -> ()));
    _pp_0.in_0 = &mut (*data).minutiae as *mut *mut fp_minutiae as *mut libc::c_char;
    _p_0 = *_pp_0.out;
    if !_p_0.is_null() {
        *_pp_0.out = 0 as *mut libc::c_void;
        _destroy_0.expect("non-null function pointer")(_p_0);
    }
    let mut _pp_1: C2RustUnnamed_4 = C2RustUnnamed_4 {
        in_0: 0 as *mut libc::c_char,
    };
    let mut _p_1: gpointer = 0 as *mut libc::c_void;
    let mut _destroy_1: GDestroyNotify = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(gpointer) -> ()>,
        GDestroyNotify,
    >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    _pp_1.in_0 = &mut (*data).binarized as *mut *mut guchar as *mut libc::c_char;
    _p_1 = *_pp_1.out;
    if !_p_1.is_null() {
        *_pp_1.out = 0 as *mut libc::c_void;
        _destroy_1.expect("non-null function pointer")(_p_1);
    }
    g_free(data as gpointer);
}
unsafe extern "C" fn fp_image_detect_minutiae_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = g_type_check_instance_cast(
        res as *mut GTypeInstance,
        g_task_get_type(),
    ) as *mut libc::c_void as *mut GTask;
    let mut image: *mut FpImage = 0 as *mut FpImage;
    let mut data: *mut DetectMinutiaeData = g_task_get_task_data(task)
        as *mut DetectMinutiaeData;
    if g_task_had_error(task) == 0 {
        let mut i: gint = 0;
        image = FP_IMAGE(source_object as gpointer);
        (*image).flags = (*data).flags;
        let mut _pp: C2RustUnnamed_9 = C2RustUnnamed_9 {
            in_0: 0 as *mut libc::c_char,
        };
        let mut _p: gpointer = 0 as *mut libc::c_void;
        let mut _destroy: GDestroyNotify = ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(gpointer) -> ()>,
            GDestroyNotify,
        >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
        _pp.in_0 = &mut (*image).data as *mut *mut guint8 as *mut libc::c_char;
        _p = *_pp.out;
        if !_p.is_null() {
            *_pp.out = 0 as *mut libc::c_void;
            _destroy.expect("non-null function pointer")(_p);
        }
        (*image)
            .data = (if 0 as libc::c_int != 0 {
            (*data).image as *mut libc::c_void
        } else {
            g_steal_pointer(&mut (*data).image as *mut *mut guchar as gpointer)
        }) as *mut guint8;
        let mut _pp_0: C2RustUnnamed_8 = C2RustUnnamed_8 {
            in_0: 0 as *mut libc::c_char,
        };
        let mut _p_0: gpointer = 0 as *mut libc::c_void;
        let mut _destroy_0: GDestroyNotify = ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(gpointer) -> ()>,
            GDestroyNotify,
        >(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
        _pp_0.in_0 = &mut (*image).binarized as *mut *mut guint8 as *mut libc::c_char;
        _p_0 = *_pp_0.out;
        if !_p_0.is_null() {
            *_pp_0.out = 0 as *mut libc::c_void;
            _destroy_0.expect("non-null function pointer")(_p_0);
        }
        (*image)
            .binarized = (if 0 as libc::c_int != 0 {
            (*data).binarized as *mut libc::c_void
        } else {
            g_steal_pointer(&mut (*data).binarized as *mut *mut guchar as gpointer)
        }) as *mut guint8;
        let mut _pp_1: C2RustUnnamed_7 = C2RustUnnamed_7 {
            in_0: 0 as *mut libc::c_char,
        };
        let mut _p_1: gpointer = 0 as *mut libc::c_void;
        let mut _destroy_1: GDestroyNotify = ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GPtrArray) -> ()>,
            GDestroyNotify,
        >(Some(g_ptr_array_unref as unsafe extern "C" fn(*mut GPtrArray) -> ()));
        _pp_1.in_0 = &mut (*image).minutiae as *mut *mut GPtrArray as *mut libc::c_char;
        _p_1 = *_pp_1.out;
        if !_p_1.is_null() {
            *_pp_1.out = 0 as *mut libc::c_void;
            _destroy_1.expect("non-null function pointer")(_p_1);
        }
        (*image)
            .minutiae = g_ptr_array_new_full(
            (*(*data).minutiae).num as guint,
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut MINUTIA) -> ()>,
                GDestroyNotify,
            >(Some(free_minutia as unsafe extern "C" fn(*mut MINUTIA) -> ())),
        );
        i = 0 as libc::c_int;
        while i < (*(*data).minutiae).num {
            g_ptr_array_add(
                (*image).minutiae,
                if 0 as libc::c_int != 0 {
                    *((*(*data).minutiae).list).offset(i as isize) as *mut libc::c_void
                } else {
                    g_steal_pointer(
                        &mut *((*(*data).minutiae).list).offset(i as isize)
                            as *mut *mut fp_minutia as gpointer,
                    )
                },
            );
            i += 1;
        }
        (*(*data).minutiae).num = 0 as libc::c_int;
    }
    if ((*data).user_cb).is_some() {
        ((*data).user_cb)
            .expect("non-null function pointer")(source_object, res, user_data);
    }
}
unsafe extern "C" fn vflip(mut data: *mut guint8, mut width: gint, mut height: gint) {
    let mut data_len: libc::c_int = width * height;
    let vla = width as usize;
    let mut rowbuf: Vec::<libc::c_uchar> = ::std::vec::from_elem(0, vla);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < height / 2 as libc::c_int {
        let mut offset: libc::c_int = i * width;
        let mut swap_offset: libc::c_int = data_len - width * (i + 1 as libc::c_int);
        memcpy(
            rowbuf.as_mut_ptr() as *mut libc::c_void,
            data.offset(offset as isize) as *const libc::c_void,
            width as libc::c_ulong,
        );
        memcpy(
            data.offset(offset as isize) as *mut libc::c_void,
            data.offset(swap_offset as isize) as *const libc::c_void,
            width as libc::c_ulong,
        );
        memcpy(
            data.offset(swap_offset as isize) as *mut libc::c_void,
            rowbuf.as_mut_ptr() as *const libc::c_void,
            width as libc::c_ulong,
        );
        i += 1;
    }
}
unsafe extern "C" fn hflip(mut data: *mut guint8, mut width: gint, mut height: gint) {
    let vla = width as usize;
    let mut rowbuf: Vec::<libc::c_uchar> = ::std::vec::from_elem(0, vla);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < height {
        let mut offset: libc::c_int = i * width;
        memcpy(
            rowbuf.as_mut_ptr() as *mut libc::c_void,
            data.offset(offset as isize) as *const libc::c_void,
            width as libc::c_ulong,
        );
        j = 0 as libc::c_int;
        while j < width {
            *data
                .offset(
                    (offset + j) as isize,
                ) = *rowbuf.as_mut_ptr().offset((width - j - 1 as libc::c_int) as isize);
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn invert_colors(
    mut data: *mut guint8,
    mut width: gint,
    mut height: gint,
) {
    let mut data_len: libc::c_int = width * height;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < data_len {
        *data
            .offset(
                i as isize,
            ) = (0xff as libc::c_int - *data.offset(i as isize) as libc::c_int)
            as guint8;
        i += 1;
    }
}
unsafe extern "C" fn fp_image_detect_minutiae_thread_func(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut timer: GTimer_autoptr = 0 as GTimer_autoptr;
    let mut data: *mut DetectMinutiaeData = task_data as *mut DetectMinutiaeData;
    let mut minutiae: *mut fp_minutiae = 0 as *mut fp_minutiae;
    let mut direction_map: *mut gint = 0 as *mut gint;
    let mut low_contrast_map: *mut gint = 0 as *mut gint;
    let mut low_flow_map: *mut gint = 0 as *mut gint;
    let mut high_curve_map: *mut gint = 0 as *mut gint;
    let mut quality_map: *mut gint = 0 as *mut gint;
    let mut bdata: *mut guchar = 0 as *mut guchar;
    let mut map_w: gint = 0;
    let mut map_h: gint = 0;
    let mut bw: gint = 0;
    let mut bh: gint = 0;
    let mut bd: gint = 0;
    let mut r: gint = 0;
    let mut lfsparms: *mut LFSPARMS = 0 as *mut LFSPARMS;
    if (*data).flags as libc::c_uint & FPI_IMAGE_H_FLIPPED as libc::c_int as libc::c_uint
        != 0
    {
        hflip((*data).image, (*data).width, (*data).height);
    }
    if (*data).flags as libc::c_uint & FPI_IMAGE_V_FLIPPED as libc::c_int as libc::c_uint
        != 0
    {
        vflip((*data).image, (*data).width, (*data).height);
    }
    if (*data).flags as libc::c_uint
        & FPI_IMAGE_COLORS_INVERTED as libc::c_int as libc::c_uint != 0
    {
        invert_colors((*data).image, (*data).width, (*data).height);
    }
    (*data)
        .flags = ::core::mem::transmute::<
        libc::c_uint,
        FpiImageFlags,
    >(
        (*data).flags as libc::c_uint
            & !(FPI_IMAGE_H_FLIPPED as libc::c_int | FPI_IMAGE_V_FLIPPED as libc::c_int
                | FPI_IMAGE_COLORS_INVERTED as libc::c_int) as libc::c_uint,
    );
    lfsparms = ({
        g_memdup2(
            &mut g_lfsparms_V2 as *mut LFSPARMS as gconstpointer,
            ::core::mem::size_of::<LFSPARMS>() as libc::c_ulong,
        )
    }) as *mut LFSPARMS;
    (*lfsparms)
        .remove_perimeter_pts = if (*data).flags as libc::c_uint
        & FPI_IMAGE_PARTIAL as libc::c_int as libc::c_uint != 0
    {
        (0 as libc::c_int == 0) as libc::c_int
    } else {
        0 as libc::c_int
    };
    timer = g_timer_new();
    r = get_minutiae(
        &mut minutiae,
        &mut quality_map,
        &mut direction_map,
        &mut low_contrast_map,
        &mut low_flow_map,
        &mut high_curve_map,
        &mut map_w,
        &mut map_h,
        &mut bdata,
        &mut bw,
        &mut bh,
        &mut bd,
        (*data).image,
        (*data).width,
        (*data).height,
        8 as libc::c_int,
        (*data).ppmm,
        lfsparms,
    );
    g_timer_stop(timer);
    g_log(
        b"libfprint-image\0" as *const u8 as *const libc::c_char,
        G_LOG_LEVEL_DEBUG,
        b"Minutiae scan completed in %f secs\0" as *const u8 as *const libc::c_char,
        g_timer_elapsed(timer, 0 as *mut gulong),
    );
    (*data)
        .binarized = (if 0 as libc::c_int != 0 {
        bdata as *mut libc::c_void
    } else {
        g_steal_pointer(&mut bdata as *mut *mut guchar as gpointer)
    }) as *mut guchar;
    (*data).minutiae = minutiae;
    if r != 0 {
        g_log(
            b"libfprint-image\0" as *const u8 as *const libc::c_char,
            G_LOG_LEVEL_CRITICAL,
            b"get minutiae failed, code %d\0" as *const u8 as *const libc::c_char,
            r,
        );
        g_task_return_new_error(
            task,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as libc::c_int,
            b"Minutiae scan failed with code %d\0" as *const u8 as *const libc::c_char,
            r,
        );
        g_object_unref(task as gpointer);
        return;
    }
    if ((*data).minutiae).is_null() || (*(*data).minutiae).num == 0 as libc::c_int {
        g_task_return_new_error(
            task,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as libc::c_int,
            b"No minutiae found\0" as *const u8 as *const libc::c_char,
        );
        g_object_unref(task as gpointer);
        return;
    }
    g_task_return_boolean(task, (0 as libc::c_int == 0) as libc::c_int);
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn fp_image_get_height(mut self_0: *mut FpImage) -> guint {
    return (*self_0).height;
}
#[no_mangle]
pub unsafe extern "C" fn fp_image_get_width(mut self_0: *mut FpImage) -> guint {
    return (*self_0).width;
}
#[no_mangle]
pub unsafe extern "C" fn fp_image_get_ppmm(mut self_0: *mut FpImage) -> gdouble {
    return (*self_0).ppmm;
}
#[no_mangle]
pub unsafe extern "C" fn fp_image_get_data(
    mut self_0: *mut FpImage,
    mut len: *mut gsize,
) -> *const guchar {
    if !len.is_null() {
        *len = ((*self_0).width).wrapping_mul((*self_0).height) as gsize;
    }
    return (*self_0).data;
}
#[no_mangle]
pub unsafe extern "C" fn fp_image_get_binarized(
    mut self_0: *mut FpImage,
    mut len: *mut gsize,
) -> *const guchar {
    if !len.is_null() && !((*self_0).binarized).is_null() {
        *len = ((*self_0).width).wrapping_mul((*self_0).height) as gsize;
    }
    return (*self_0).binarized;
}
#[no_mangle]
pub unsafe extern "C" fn fp_image_get_minutiae(
    mut self_0: *mut FpImage,
) -> *mut GPtrArray {
    return (*self_0).minutiae;
}
#[no_mangle]
pub unsafe extern "C" fn fp_image_detect_minutiae(
    mut self_0: *mut FpImage,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = 0 as *mut GTask;
    let mut data: *mut DetectMinutiaeData = ({
        let mut __n: gsize = 1 as libc::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<DetectMinutiaeData>()
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
    }) as *mut DetectMinutiaeData;
    task = g_task_new(
        self_0 as gpointer,
        cancellable,
        Some(
            fp_image_detect_minutiae_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        user_data,
    );
    (*data)
        .image = g_malloc(((*self_0).width).wrapping_mul((*self_0).height) as gsize)
        as *mut guchar;
    memcpy(
        (*data).image as *mut libc::c_void,
        (*self_0).data as *const libc::c_void,
        ((*self_0).width).wrapping_mul((*self_0).height) as libc::c_ulong,
    );
    (*data).flags = (*self_0).flags;
    (*data).width = (*self_0).width as gint;
    (*data).height = (*self_0).height as gint;
    (*data).ppmm = (*self_0).ppmm;
    (*data).user_cb = callback;
    g_task_set_task_data(
        task,
        data as gpointer,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut DetectMinutiaeData) -> ()>,
            GDestroyNotify,
        >(
            Some(
                fp_image_detect_minutiae_free
                    as unsafe extern "C" fn(*mut DetectMinutiaeData) -> (),
            ),
        ),
    );
    g_task_run_in_thread(
        task,
        Some(
            fp_image_detect_minutiae_thread_func
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
pub unsafe extern "C" fn fp_image_detect_minutiae_finish(
    mut self_0: *mut FpImage,
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
pub unsafe extern "C" fn fp_minutia_get_coords(
    mut min: *mut FpMinutia,
    mut x: *mut gint,
    mut y: *mut gint,
) {
    if !x.is_null() {
        *x = (*min).x;
    }
    if !y.is_null() {
        *y = (*min).y;
    }
}
