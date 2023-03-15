use ::libc;
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn g_free(mem: gpointer);
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_sort(list: *mut GList, compare_func: GCompareFunc) -> *mut GList;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_insert(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_print(format: *const gchar, _: ...);
    fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    fn g_strcmp0(str1: *const libc::c_char, str2: *const libc::c_char) -> libc::c_int;
    fn g_type_class_ref(type_0: GType) -> gpointer;
    fn fpi_get_driver_types() -> *mut GArray;
}
pub type gint32 = libc::c_int;
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
pub type GCompareFunc = Option::<
    unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
>;
pub type GEqualFunc = Option::<
    unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
>;
pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option::<unsafe extern "C" fn(gconstpointer) -> guint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GArray_autoptr = *mut GArray;
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
pub struct _GUsbDevice {
    pub parent_instance: GObject,
}
pub type GUsbDevice = _GUsbDevice;
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
pub type FpDeviceClass_autoptr = *mut FpDeviceClass;
#[no_mangle]
pub static mut printed: *mut GHashTable = 0 as *const GHashTable as *mut GHashTable;
unsafe extern "C" fn insert_drivers(
    mut usb_list: *mut *mut GList,
    mut spi_list: *mut *mut GList,
) {
    let mut drivers: GArray_autoptr = fpi_get_driver_types();
    let mut i: gint = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*drivers).len {
        let mut driver: GType = *((*drivers).data as *mut libc::c_void as *mut GType)
            .offset(i as isize);
        let mut cls: FpDeviceClass_autoptr = g_type_class_ref(driver)
            as FpDeviceClass_autoptr;
        let mut entry: *const FpIdEntry = 0 as *const FpIdEntry;
        match (*cls).type_0 as libc::c_uint {
            2 => {
                entry = (*cls).id_table;
                while (*entry).c2rust_unnamed.c2rust_unnamed.vid != 0 {
                    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
                    key = g_strdup_printf(
                        b"%04x:%04x\0" as *const u8 as *const libc::c_char,
                        (*entry).c2rust_unnamed.c2rust_unnamed.vid,
                        (*entry).c2rust_unnamed.c2rust_unnamed.pid,
                    );
                    if !(g_hash_table_lookup(printed, key as gconstpointer)).is_null() {
                        g_free(key as gpointer);
                    } else {
                        g_hash_table_insert(
                            printed,
                            key as gpointer,
                            1 as libc::c_int as glong as gpointer,
                        );
                        *usb_list = g_list_prepend(
                            *usb_list,
                            g_strdup_printf(
                                b"%s | %s\n\0" as *const u8 as *const libc::c_char,
                                key,
                                (*cls).full_name,
                            ) as gpointer,
                        );
                    }
                    entry = entry.offset(1);
                }
            }
            1 => {
                entry = (*cls).id_table;
                while (*entry).c2rust_unnamed.c2rust_unnamed_0.udev_types as u64 != 0 {
                    let mut key_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    if !((*entry).c2rust_unnamed.c2rust_unnamed_0.udev_types
                        as libc::c_uint
                        & FPI_DEVICE_UDEV_SUBTYPE_SPIDEV as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint)
                    {
                        key_0 = g_strdup_printf(
                            b"SPI:%s:%04x:%04x\0" as *const u8 as *const libc::c_char,
                            (*entry).c2rust_unnamed.c2rust_unnamed_0.spi_acpi_id,
                            (*entry).c2rust_unnamed.c2rust_unnamed_0.hid_id.vid,
                            (*entry).c2rust_unnamed.c2rust_unnamed_0.hid_id.pid,
                        );
                        if !(g_hash_table_lookup(printed, key_0 as gconstpointer))
                            .is_null()
                        {
                            g_free(key_0 as gpointer);
                        } else {
                            g_hash_table_insert(
                                printed,
                                key_0 as gpointer,
                                1 as libc::c_int as glong as gpointer,
                            );
                            if (*entry).c2rust_unnamed.c2rust_unnamed_0.udev_types
                                as libc::c_uint
                                & FPI_DEVICE_UDEV_SUBTYPE_HIDRAW as libc::c_int
                                    as libc::c_uint != 0
                            {
                                *spi_list = g_list_prepend(
                                    *spi_list,
                                    g_strdup_printf(
                                        b"%s | %04x:%04x | %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        (*entry).c2rust_unnamed.c2rust_unnamed_0.spi_acpi_id,
                                        (*entry).c2rust_unnamed.c2rust_unnamed_0.hid_id.vid,
                                        (*entry).c2rust_unnamed.c2rust_unnamed_0.hid_id.pid,
                                        (*cls).full_name,
                                    ) as gpointer,
                                );
                            } else {
                                *spi_list = g_list_prepend(
                                    *spi_list,
                                    g_strdup_printf(
                                        b"%s | - | %s\n\0" as *const u8 as *const libc::c_char,
                                        (*entry).c2rust_unnamed.c2rust_unnamed_0.spi_acpi_id,
                                        (*cls).full_name,
                                    ) as gpointer,
                                );
                            }
                        }
                    }
                    entry = entry.offset(1);
                }
            }
            0 | _ => {}
        }
        i += 1;
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut usb_list: *mut GList = 0 as *mut GList;
    let mut spi_list: *mut GList = 0 as *mut GList;
    let mut l: *mut GList = 0 as *mut GList;
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    printed = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(
            g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
        ),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        None,
    );
    g_print(
        b"%% lifprint \xE2\x80\x94 Supported Devices\n\0" as *const u8
            as *const libc::c_char,
    );
    g_print(b"%% Bastien Nocera, Daniel Drake\n\0" as *const u8 as *const libc::c_char);
    g_print(b"%% 2018\n\0" as *const u8 as *const libc::c_char);
    g_print(b"\n\0" as *const u8 as *const libc::c_char);
    g_print(b"# Supported Devices\n\0" as *const u8 as *const libc::c_char);
    g_print(b"\n\0" as *const u8 as *const libc::c_char);
    g_print(
        b"This is a list of supported devices in libfprint's development version. Those drivers might not all be available in the stable, released version. If in doubt, contact your distribution or systems integrator for details.\n\0"
            as *const u8 as *const libc::c_char,
    );
    g_print(b"\n\0" as *const u8 as *const libc::c_char);
    insert_drivers(&mut usb_list, &mut spi_list);
    g_print(b"## USB devices\n\0" as *const u8 as *const libc::c_char);
    g_print(b"\n\0" as *const u8 as *const libc::c_char);
    g_print(b"USB ID | Driver\n\0" as *const u8 as *const libc::c_char);
    g_print(b"------------ | ------------\n\0" as *const u8 as *const libc::c_char);
    usb_list = g_list_sort(
        usb_list,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
            GCompareFunc,
        >(
            Some(
                g_strcmp0
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        ),
    );
    l = usb_list;
    while !l.is_null() {
        g_print(
            b"%s\0" as *const u8 as *const libc::c_char,
            (*l).data as *mut libc::c_char,
        );
        l = (*l).next;
    }
    g_print(b"\n\0" as *const u8 as *const libc::c_char);
    g_list_free_full(usb_list, Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    g_print(b"## SPI devices\n\0" as *const u8 as *const libc::c_char);
    g_print(b"\n\0" as *const u8 as *const libc::c_char);
    g_print(
        b"The ACPI ID represents the SPI interface. Some sensors are also connected to a HID device (Human Input Device) for side-channel requests such as resets.\n\0"
            as *const u8 as *const libc::c_char,
    );
    g_print(b"\n\0" as *const u8 as *const libc::c_char);
    g_print(b"ACPI ID | HID ID | Driver\n\0" as *const u8 as *const libc::c_char);
    g_print(
        b"------------ | ------------ | ------------\n\0" as *const u8
            as *const libc::c_char,
    );
    spi_list = g_list_sort(
        spi_list,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
            GCompareFunc,
        >(
            Some(
                g_strcmp0
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        ),
    );
    l = spi_list;
    while !l.is_null() {
        g_print(
            b"%s\0" as *const u8 as *const libc::c_char,
            (*l).data as *mut libc::c_char,
        );
        l = (*l).next;
    }
    g_print(b"\n\0" as *const u8 as *const libc::c_char);
    g_list_free_full(usb_list, Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    g_hash_table_destroy(printed);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
