use ::libc;
extern "C" {
    fn g_array_new(
        zero_terminated: gboolean,
        clear_: gboolean,
        element_size: guint,
    ) -> *mut GArray;
    fn g_array_append_vals(
        array: *mut GArray,
        data: gconstpointer,
        len: guint,
    ) -> *mut GArray;
    fn fpi_device_upektc_img_get_type() -> GType;
    fn fpi_device_vfs5011_get_type() -> GType;
    fn fpi_device_vfs7552_get_type() -> GType;
    fn fpi_device_aes3500_get_type() -> GType;
    fn fpi_device_aes4000_get_type() -> GType;
    fn fpi_device_aes1610_get_type() -> GType;
    fn fpi_device_aes1660_get_type() -> GType;
    fn fpi_device_aes2660_get_type() -> GType;
    fn fpi_device_aes2501_get_type() -> GType;
    fn fpi_device_aes2550_get_type() -> GType;
    fn fpi_device_vfs101_get_type() -> GType;
    fn fpi_device_vfs301_get_type() -> GType;
    fn fpi_device_vfs0050_get_type() -> GType;
    fn fpi_device_etes603_get_type() -> GType;
    fn fpi_device_egis0570_get_type() -> GType;
    fn fpi_device_vcom5s_get_type() -> GType;
    fn fpi_device_synaptics_get_type() -> GType;
    fn fpi_device_elan_get_type() -> GType;
    fn fpi_device_elanmoc_get_type() -> GType;
    fn fpi_device_uru4000_get_type() -> GType;
    fn fpi_device_upektc_get_type() -> GType;
    fn fpi_device_upeksonly_get_type() -> GType;
    fn fpi_device_upekts_get_type() -> GType;
    fn fpi_device_goodixmoc_get_type() -> GType;
    fn fpi_device_nb1010_get_type() -> GType;
    fn fpi_device_fpcmoc_get_type() -> GType;
    fn fpi_device_elanspi_get_type() -> GType;
}
pub type gsize = libc::c_ulong;
pub type gchar = libc::c_char;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type guint = libc::c_uint;
pub type gconstpointer = *const libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
pub type GType = gsize;
#[no_mangle]
pub unsafe extern "C" fn fpi_get_driver_types() -> *mut GArray {
    let mut drivers: *mut GArray = g_array_new(
        (0 as libc::c_int == 0) as libc::c_int,
        0 as libc::c_int,
        ::core::mem::size_of::<GType>() as libc::c_ulong as guint,
    );
    let mut t: GType = 0;
    t = fpi_device_upektc_img_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_vfs5011_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_vfs7552_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_aes3500_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_aes4000_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_aes1610_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_aes1660_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_aes2660_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_aes2501_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_aes2550_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_vfs101_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_vfs301_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_vfs0050_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_etes603_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_egis0570_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_vcom5s_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_synaptics_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_elan_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_elanmoc_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_uru4000_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_upektc_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_upeksonly_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_upekts_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_goodixmoc_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_nb1010_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_fpcmoc_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    t = fpi_device_elanspi_get_type();
    g_array_append_vals(
        drivers,
        &mut t as *mut GType as gconstpointer,
        1 as libc::c_int as guint,
    );
    return drivers;
}
