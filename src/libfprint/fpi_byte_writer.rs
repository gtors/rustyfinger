use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_try_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_return_if_fail_warning(
        log_domain: *const libc::c_char,
        pretty_function: *const libc::c_char,
        expression: *const libc::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
}
pub type gint8 = libc::c_schar;
pub type guint8 = libc::c_uchar;
pub type gint16 = libc::c_short;
pub type guint16 = libc::c_ushort;
pub type gint32 = libc::c_int;
pub type guint32 = libc::c_uint;
pub type gint64 = libc::c_long;
pub type guint64 = libc::c_ulong;
pub type gsize = libc::c_ulong;
pub type gchar = libc::c_char;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type guint = libc::c_uint;
pub type gfloat = libc::c_float;
pub type gdouble = libc::c_double;
pub type gpointer = *mut libc::c_void;
pub type gconstpointer = *const libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub i: guint32,
    pub f: gfloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub i: guint32,
    pub f: gfloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: guint64,
    pub d: gdouble,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub i: guint64,
    pub d: gdouble,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiByteReader {
    pub data: *const guint8,
    pub size: guint,
    pub byte: guint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpiByteWriter {
    pub parent: FpiByteReader,
    pub alloc_size: guint,
    pub fixed: gboolean,
    pub owned: gboolean,
}
#[inline]
unsafe extern "C" fn FP_WRITE_FLOAT_LE(mut data: *mut guint8, mut num: gfloat) {
    let mut u: C2RustUnnamed = C2RustUnnamed { i: 0 };
    u.f = num;
    let mut __put_data: gpointer = data as gpointer;
    let mut __put_val: guint32 = u.i;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            2 as libc::c_int as isize,
        ) = (__put_val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            3 as libc::c_int as isize,
        ) = (__put_val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
}
#[inline]
unsafe extern "C" fn FP_WRITE_FLOAT_BE(mut data: *mut guint8, mut num: gfloat) {
    let mut u: C2RustUnnamed_0 = C2RustUnnamed_0 { i: 0 };
    u.f = num;
    let mut __put_data: gpointer = data as gpointer;
    let mut __put_val: guint32 = u.i;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            2 as libc::c_int as isize,
        ) = (__put_val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            3 as libc::c_int as isize,
        ) = (__put_val >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
}
#[inline]
unsafe extern "C" fn FP_WRITE_DOUBLE_LE(mut data: *mut guint8, mut num: gdouble) {
    let mut u: C2RustUnnamed_1 = C2RustUnnamed_1 { i: 0 };
    u.d = num;
    let mut __put_data: gpointer = data as gpointer;
    let mut __put_val: guint64 = u.i;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            2 as libc::c_int as isize,
        ) = (__put_val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            3 as libc::c_int as isize,
        ) = (__put_val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            4 as libc::c_int as isize,
        ) = (__put_val >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            5 as libc::c_int as isize,
        ) = (__put_val >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            6 as libc::c_int as isize,
        ) = (__put_val >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            7 as libc::c_int as isize,
        ) = (__put_val >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
}
#[inline]
unsafe extern "C" fn FP_WRITE_DOUBLE_BE(mut data: *mut guint8, mut num: gdouble) {
    let mut u: C2RustUnnamed_2 = C2RustUnnamed_2 { i: 0 };
    u.d = num;
    let mut __put_data: gpointer = data as gpointer;
    let mut __put_val: guint64 = u.i;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            2 as libc::c_int as isize,
        ) = (__put_val >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            3 as libc::c_int as isize,
        ) = (__put_val >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            4 as libc::c_int as isize,
        ) = (__put_val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            5 as libc::c_int as isize,
        ) = (__put_val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            6 as libc::c_int as isize,
        ) = (__put_val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            7 as libc::c_int as isize,
        ) = (__put_val >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_fill_unchecked(
    mut writer: *mut FpiByteWriter,
    mut value: guint8,
    mut size: guint,
) {
    memset(
        &*((*writer).parent.data).offset((*writer).parent.byte as isize) as *const guint8
            as *mut guint8 as *mut libc::c_void,
        value as libc::c_int,
        size as libc::c_ulong,
    );
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint).wrapping_add(size) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_fill_inline(
    mut writer: *mut FpiByteWriter,
    mut value: guint8,
    mut size: guint,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
            >(b"fpi_byte_writer_fill_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(writer, size) == 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_fill_unchecked(writer, value, size);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_data_unchecked(
    mut writer: *mut FpiByteWriter,
    mut data: *const guint8,
    mut size: guint,
) {
    memcpy(
        &*((*writer).parent.data).offset((*writer).parent.byte as isize) as *const guint8
            as *mut guint8 as *mut libc::c_void,
        data as *const libc::c_void,
        size as libc::c_ulong,
    );
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint).wrapping_add(size) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_data_inline(
    mut writer: *mut FpiByteWriter,
    mut data: *const guint8,
    mut size: guint,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"fpi_byte_writer_put_data_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(writer, size) == 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_data_unchecked(writer, data, size);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_ensure_free_space_inline(
    mut writer: *mut FpiByteWriter,
    mut size: guint,
) -> gboolean {
    let mut data: gpointer = 0 as *mut libc::c_void;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if size <= ((*writer).alloc_size).wrapping_sub((*writer).parent.byte) {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*writer).fixed != 0 || (*writer).owned == 0 {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (*writer).parent.byte
            > (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_sub(size)
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    (*writer)
        .alloc_size = fpi_byte_writer_next_pow2(
        ((*writer).parent.byte).wrapping_add(size),
    );
    data = g_try_realloc(
        (*writer).parent.data as *mut guint8 as gpointer,
        (*writer).alloc_size as gsize,
    );
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if data.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    (*writer).parent.data = data as *mut guint8;
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_next_pow2(mut n: guint) -> guint {
    let mut ret: guint = 16 as libc::c_int as guint;
    while ret < n && ret > 0 as libc::c_int as libc::c_uint {
        ret <<= 1 as libc::c_int;
    }
    return if ret != 0 { ret } else { n };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_float64_le_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: gdouble,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    FP_WRITE_DOUBLE_LE(write_data, val);
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((64 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint8_inline(
    mut writer: *mut FpiByteWriter,
    mut val: guint8,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"fpi_byte_writer_put_uint8_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (8 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_uint8_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint8_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: guint8,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    *write_data
        .offset(
            0 as libc::c_int as isize,
        ) = (val as libc::c_int >> 0 as libc::c_int & 0xff as libc::c_int) as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((8 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_float64_le_inline(
    mut writer: *mut FpiByteWriter,
    mut val: gdouble,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"fpi_byte_writer_put_float64_le_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (64 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_float64_le_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int8_inline(
    mut writer: *mut FpiByteWriter,
    mut val: gint8,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"fpi_byte_writer_put_int8_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (8 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_int8_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int8_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: gint8,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    *write_data
        .offset(
            0 as libc::c_int as isize,
        ) = (val as guint8 as libc::c_int >> 0 as libc::c_int & 0xff as libc::c_int)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((8 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_float64_be_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: gdouble,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    FP_WRITE_DOUBLE_BE(write_data, val);
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((64 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint16_be_inline(
    mut writer: *mut FpiByteWriter,
    mut val: guint16,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fpi_byte_writer_put_uint16_be_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (16 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_uint16_be_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint16_be_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: guint16,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    let mut __put_data: gpointer = write_data as gpointer;
    let mut __put_val: guint16 = val;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val as libc::c_int >> 0 as libc::c_int & 0xff as libc::c_int)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((16 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_float64_be_inline(
    mut writer: *mut FpiByteWriter,
    mut val: gdouble,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"fpi_byte_writer_put_float64_be_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (64 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_float64_be_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint16_le_inline(
    mut writer: *mut FpiByteWriter,
    mut val: guint16,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fpi_byte_writer_put_uint16_le_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (16 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_uint16_le_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint16_le_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: guint16,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    let mut __put_data: gpointer = write_data as gpointer;
    let mut __put_val: guint16 = val;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val as libc::c_int >> 0 as libc::c_int & 0xff as libc::c_int)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((16 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_float32_le_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: gfloat,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    FP_WRITE_FLOAT_LE(write_data, val);
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((32 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int16_be_inline(
    mut writer: *mut FpiByteWriter,
    mut val: gint16,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"fpi_byte_writer_put_int16_be_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (16 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_int16_be_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int16_be_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: gint16,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    let mut __put_data: gpointer = write_data as gpointer;
    let mut __put_val: guint16 = val as guint16;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val as libc::c_int >> 0 as libc::c_int & 0xff as libc::c_int)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((16 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_float32_le_inline(
    mut writer: *mut FpiByteWriter,
    mut val: gfloat,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"fpi_byte_writer_put_float32_le_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (32 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_float32_le_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int16_le_inline(
    mut writer: *mut FpiByteWriter,
    mut val: gint16,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"fpi_byte_writer_put_int16_le_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (16 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_int16_le_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int16_le_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: gint16,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    let mut __put_data: gpointer = write_data as gpointer;
    let mut __put_val: guint16 = val as guint16;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val as libc::c_int >> 0 as libc::c_int & 0xff as libc::c_int)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((16 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_float32_be_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: gfloat,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    FP_WRITE_FLOAT_BE(write_data, val);
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((32 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint24_be_inline(
    mut writer: *mut FpiByteWriter,
    mut val: guint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fpi_byte_writer_put_uint24_be_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (24 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_uint24_be_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint24_be_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: guint32,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    let mut __put_data: gpointer = write_data as gpointer;
    let mut __put_val: guint32 = val;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            2 as libc::c_int as isize,
        ) = (__put_val >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((24 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_float32_be_inline(
    mut writer: *mut FpiByteWriter,
    mut val: gfloat,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"fpi_byte_writer_put_float32_be_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (32 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_float32_be_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint24_le_inline(
    mut writer: *mut FpiByteWriter,
    mut val: guint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fpi_byte_writer_put_uint24_le_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (24 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_uint24_le_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint24_le_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: guint32,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    let mut __put_data: gpointer = write_data as gpointer;
    let mut __put_val: guint32 = val;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            2 as libc::c_int as isize,
        ) = (__put_val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((24 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int64_le_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: gint64,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    let mut __put_data: gpointer = write_data as gpointer;
    let mut __put_val: guint64 = val as guint64;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            2 as libc::c_int as isize,
        ) = (__put_val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            3 as libc::c_int as isize,
        ) = (__put_val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            4 as libc::c_int as isize,
        ) = (__put_val >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            5 as libc::c_int as isize,
        ) = (__put_val >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            6 as libc::c_int as isize,
        ) = (__put_val >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            7 as libc::c_int as isize,
        ) = (__put_val >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((64 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int24_be_inline(
    mut writer: *mut FpiByteWriter,
    mut val: gint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"fpi_byte_writer_put_int24_be_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (24 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_int24_be_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int24_be_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: gint32,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    let mut __put_data: gpointer = write_data as gpointer;
    let mut __put_val: guint32 = val as guint32;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            2 as libc::c_int as isize,
        ) = (__put_val >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((24 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int64_le_inline(
    mut writer: *mut FpiByteWriter,
    mut val: gint64,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"fpi_byte_writer_put_int64_le_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (64 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_int64_le_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int24_le_inline(
    mut writer: *mut FpiByteWriter,
    mut val: gint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"fpi_byte_writer_put_int24_le_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (24 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_int24_le_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int24_le_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: gint32,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    let mut __put_data: gpointer = write_data as gpointer;
    let mut __put_val: guint32 = val as guint32;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            2 as libc::c_int as isize,
        ) = (__put_val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((24 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int64_be_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: gint64,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    let mut __put_data: gpointer = write_data as gpointer;
    let mut __put_val: guint64 = val as guint64;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            2 as libc::c_int as isize,
        ) = (__put_val >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            3 as libc::c_int as isize,
        ) = (__put_val >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            4 as libc::c_int as isize,
        ) = (__put_val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            5 as libc::c_int as isize,
        ) = (__put_val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            6 as libc::c_int as isize,
        ) = (__put_val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            7 as libc::c_int as isize,
        ) = (__put_val >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((64 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint32_be_inline(
    mut writer: *mut FpiByteWriter,
    mut val: guint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fpi_byte_writer_put_uint32_be_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (32 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_uint32_be_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint32_be_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: guint32,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    let mut __put_data: gpointer = write_data as gpointer;
    let mut __put_val: guint32 = val;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            2 as libc::c_int as isize,
        ) = (__put_val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            3 as libc::c_int as isize,
        ) = (__put_val >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((32 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int64_be_inline(
    mut writer: *mut FpiByteWriter,
    mut val: gint64,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"fpi_byte_writer_put_int64_be_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (64 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_int64_be_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint32_le_inline(
    mut writer: *mut FpiByteWriter,
    mut val: guint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fpi_byte_writer_put_uint32_le_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (32 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_uint32_le_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint32_le_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: guint32,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    let mut __put_data: gpointer = write_data as gpointer;
    let mut __put_val: guint32 = val;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            2 as libc::c_int as isize,
        ) = (__put_val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            3 as libc::c_int as isize,
        ) = (__put_val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((32 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint64_le_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: guint64,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    let mut __put_data: gpointer = write_data as gpointer;
    let mut __put_val: guint64 = val;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            2 as libc::c_int as isize,
        ) = (__put_val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            3 as libc::c_int as isize,
        ) = (__put_val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            4 as libc::c_int as isize,
        ) = (__put_val >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            5 as libc::c_int as isize,
        ) = (__put_val >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            6 as libc::c_int as isize,
        ) = (__put_val >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            7 as libc::c_int as isize,
        ) = (__put_val >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((64 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int32_be_inline(
    mut writer: *mut FpiByteWriter,
    mut val: gint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"fpi_byte_writer_put_int32_be_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (32 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_int32_be_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int32_be_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: gint32,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    let mut __put_data: gpointer = write_data as gpointer;
    let mut __put_val: guint32 = val as guint32;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            2 as libc::c_int as isize,
        ) = (__put_val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            3 as libc::c_int as isize,
        ) = (__put_val >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((32 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint64_le_inline(
    mut writer: *mut FpiByteWriter,
    mut val: guint64,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fpi_byte_writer_put_uint64_le_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (64 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_uint64_le_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int32_le_inline(
    mut writer: *mut FpiByteWriter,
    mut val: gint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"fpi_byte_writer_put_int32_le_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (32 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_int32_le_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_int32_le_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: gint32,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    let mut __put_data: gpointer = write_data as gpointer;
    let mut __put_val: guint32 = val as guint32;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            2 as libc::c_int as isize,
        ) = (__put_val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            3 as libc::c_int as isize,
        ) = (__put_val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((32 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint64_be_unchecked(
    mut writer: *mut FpiByteWriter,
    mut val: guint64,
) {
    let mut write_data: *mut guint8 = 0 as *mut guint8;
    write_data = ((*writer).parent.data as *mut guint8)
        .offset((*writer).parent.byte as isize);
    let mut __put_data: gpointer = write_data as gpointer;
    let mut __put_val: guint64 = val;
    *(__put_data as *mut guint8)
        .offset(
            0 as libc::c_int as isize,
        ) = (__put_val >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            1 as libc::c_int as isize,
        ) = (__put_val >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            2 as libc::c_int as isize,
        ) = (__put_val >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            3 as libc::c_int as isize,
        ) = (__put_val >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            4 as libc::c_int as isize,
        ) = (__put_val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            5 as libc::c_int as isize,
        ) = (__put_val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            6 as libc::c_int as isize,
        ) = (__put_val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    *(__put_data as *mut guint8)
        .offset(
            7 as libc::c_int as isize,
        ) = (__put_val >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as guint8;
    (*writer)
        .parent
        .byte = ((*writer).parent.byte as libc::c_uint)
        .wrapping_add((64 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    (*writer)
        .parent
        .size = if (*writer).parent.size > (*writer).parent.byte {
        (*writer).parent.size
    } else {
        (*writer).parent.byte
    };
}
#[inline]
unsafe extern "C" fn fpi_byte_writer_put_uint64_be_inline(
    mut writer: *mut FpiByteWriter,
    mut val: guint64,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"fpi_byte_writer_put_uint64_be_inline\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            (64 as libc::c_int / 8 as libc::c_int) as guint,
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_uint64_be_unchecked(writer, val);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_new() -> *mut FpiByteWriter {
    let mut ret: *mut FpiByteWriter = ({
        let mut __s: gsize = ::core::mem::size_of::<FpiByteWriter>() as libc::c_ulong;
        let mut __p: gpointer = 0 as *mut libc::c_void;
        __p = g_slice_alloc(__s);
        memset(__p, 0 as libc::c_int, __s);
        __p
    }) as *mut FpiByteWriter;
    (*ret).owned = (0 as libc::c_int == 0) as libc::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_new_with_size(
    mut size: guint,
    mut fixed: gboolean,
) -> *mut FpiByteWriter {
    let mut ret: *mut FpiByteWriter = fpi_byte_writer_new();
    (*ret).alloc_size = size;
    (*ret).parent.data = g_malloc((*ret).alloc_size as gsize) as *const guint8;
    (*ret).fixed = fixed;
    (*ret).owned = (0 as libc::c_int == 0) as libc::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_new_with_data(
    mut data: *mut guint8,
    mut size: guint,
    mut initialized: gboolean,
) -> *mut FpiByteWriter {
    let mut ret: *mut FpiByteWriter = fpi_byte_writer_new();
    (*ret).parent.data = data;
    (*ret)
        .parent
        .size = if initialized != 0 { size } else { 0 as libc::c_int as libc::c_uint };
    (*ret).alloc_size = size;
    (*ret).fixed = (0 as libc::c_int == 0) as libc::c_int;
    (*ret).owned = 0 as libc::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_init(mut writer: *mut FpiByteWriter) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
            >(b"fpi_byte_writer_init\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    memset(
        writer as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<FpiByteWriter>() as libc::c_ulong,
    );
    (*writer).owned = (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_init_with_size(
    mut writer: *mut FpiByteWriter,
    mut size: guint,
    mut fixed: gboolean,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"fpi_byte_writer_init_with_size\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    fpi_byte_writer_init(writer);
    (*writer).parent.data = g_malloc(size as gsize) as *const guint8;
    (*writer).alloc_size = size;
    (*writer).fixed = fixed;
    (*writer).owned = (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_init_with_data(
    mut writer: *mut FpiByteWriter,
    mut data: *mut guint8,
    mut size: guint,
    mut initialized: gboolean,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"fpi_byte_writer_init_with_data\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    fpi_byte_writer_init(writer);
    (*writer).parent.data = data;
    (*writer)
        .parent
        .size = if initialized != 0 { size } else { 0 as libc::c_int as libc::c_uint };
    (*writer).alloc_size = size;
    (*writer).fixed = (0 as libc::c_int == 0) as libc::c_int;
    (*writer).owned = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_reset(mut writer: *mut FpiByteWriter) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"fpi_byte_writer_reset\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*writer).owned != 0 {
        g_free((*writer).parent.data as *mut guint8 as gpointer);
    }
    memset(
        writer as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<FpiByteWriter>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_reset_and_get_data(
    mut writer: *mut FpiByteWriter,
) -> *mut guint8 {
    let mut data: *mut guint8 = 0 as *mut guint8;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"fpi_byte_writer_reset_and_get_data\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut guint8;
    }
    data = (*writer).parent.data as *mut guint8;
    if (*writer).owned == 0 {
        data = ({ g_memdup2(data as gconstpointer, (*writer).parent.size as gsize) })
            as *mut guint8;
    }
    (*writer).parent.data = 0 as *const guint8;
    fpi_byte_writer_reset(writer);
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_free(mut writer: *mut FpiByteWriter) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
            >(b"fpi_byte_writer_free\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    fpi_byte_writer_reset(writer);
    g_slice_free1(
        ::core::mem::size_of::<FpiByteWriter>() as libc::c_ulong,
        writer as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_free_and_get_data(
    mut writer: *mut FpiByteWriter,
) -> *mut guint8 {
    let mut data: *mut guint8 = 0 as *mut guint8;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"fpi_byte_writer_free_and_get_data\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut guint8;
    }
    data = fpi_byte_writer_reset_and_get_data(writer);
    g_slice_free1(
        ::core::mem::size_of::<FpiByteWriter>() as libc::c_ulong,
        writer as gpointer,
    );
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_get_remaining(
    mut writer: *const FpiByteWriter,
) -> guint {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"fpi_byte_writer_get_remaining\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int) as guint;
    }
    if (*writer).fixed == 0 {
        return -(1 as libc::c_int) as guint
    } else {
        return ((*writer).alloc_size).wrapping_sub((*writer).parent.byte)
    };
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_ensure_free_space(
    mut writer: *mut FpiByteWriter,
    mut size: guint,
) -> gboolean {
    return fpi_byte_writer_ensure_free_space_inline(writer, size);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_uint8(
    mut writer: *mut FpiByteWriter,
    mut val: guint8,
) -> gboolean {
    return fpi_byte_writer_put_uint8_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_int8(
    mut writer: *mut FpiByteWriter,
    mut val: gint8,
) -> gboolean {
    return fpi_byte_writer_put_int8_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_uint16_le(
    mut writer: *mut FpiByteWriter,
    mut val: guint16,
) -> gboolean {
    return fpi_byte_writer_put_uint16_le_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_uint16_be(
    mut writer: *mut FpiByteWriter,
    mut val: guint16,
) -> gboolean {
    return fpi_byte_writer_put_uint16_be_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_int16_le(
    mut writer: *mut FpiByteWriter,
    mut val: gint16,
) -> gboolean {
    return fpi_byte_writer_put_int16_le_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_int16_be(
    mut writer: *mut FpiByteWriter,
    mut val: gint16,
) -> gboolean {
    return fpi_byte_writer_put_int16_be_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_uint24_le(
    mut writer: *mut FpiByteWriter,
    mut val: guint32,
) -> gboolean {
    return fpi_byte_writer_put_uint24_le_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_uint24_be(
    mut writer: *mut FpiByteWriter,
    mut val: guint32,
) -> gboolean {
    return fpi_byte_writer_put_uint24_be_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_int24_le(
    mut writer: *mut FpiByteWriter,
    mut val: gint32,
) -> gboolean {
    return fpi_byte_writer_put_int24_le_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_int24_be(
    mut writer: *mut FpiByteWriter,
    mut val: gint32,
) -> gboolean {
    return fpi_byte_writer_put_int24_be_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_uint32_le(
    mut writer: *mut FpiByteWriter,
    mut val: guint32,
) -> gboolean {
    return fpi_byte_writer_put_uint32_le_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_uint32_be(
    mut writer: *mut FpiByteWriter,
    mut val: guint32,
) -> gboolean {
    return fpi_byte_writer_put_uint32_be_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_int32_le(
    mut writer: *mut FpiByteWriter,
    mut val: gint32,
) -> gboolean {
    return fpi_byte_writer_put_int32_le_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_int32_be(
    mut writer: *mut FpiByteWriter,
    mut val: gint32,
) -> gboolean {
    return fpi_byte_writer_put_int32_be_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_uint64_le(
    mut writer: *mut FpiByteWriter,
    mut val: guint64,
) -> gboolean {
    return fpi_byte_writer_put_uint64_le_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_uint64_be(
    mut writer: *mut FpiByteWriter,
    mut val: guint64,
) -> gboolean {
    return fpi_byte_writer_put_uint64_be_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_int64_le(
    mut writer: *mut FpiByteWriter,
    mut val: gint64,
) -> gboolean {
    return fpi_byte_writer_put_int64_le_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_int64_be(
    mut writer: *mut FpiByteWriter,
    mut val: gint64,
) -> gboolean {
    return fpi_byte_writer_put_int64_be_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_float32_be(
    mut writer: *mut FpiByteWriter,
    mut val: gfloat,
) -> gboolean {
    return fpi_byte_writer_put_float32_be_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_float32_le(
    mut writer: *mut FpiByteWriter,
    mut val: gfloat,
) -> gboolean {
    return fpi_byte_writer_put_float32_le_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_float64_be(
    mut writer: *mut FpiByteWriter,
    mut val: gdouble,
) -> gboolean {
    return fpi_byte_writer_put_float64_be_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_float64_le(
    mut writer: *mut FpiByteWriter,
    mut val: gdouble,
) -> gboolean {
    return fpi_byte_writer_put_float64_le_inline(writer, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_data(
    mut writer: *mut FpiByteWriter,
    mut data: *const guint8,
    mut size: guint,
) -> gboolean {
    return fpi_byte_writer_put_data_inline(writer, data, size);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_fill(
    mut writer: *mut FpiByteWriter,
    mut value: guint8,
    mut size: guint,
) -> gboolean {
    return fpi_byte_writer_fill_inline(writer, value, size);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_string_utf8(
    mut writer: *mut FpiByteWriter,
    mut data: *const gchar,
) -> gboolean {
    let mut size: guint = 0 as libc::c_int as guint;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"fpi_byte_writer_put_string_utf8\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    while *data.offset(size as isize) as libc::c_int != 0 as libc::c_int {
        if ({
            let mut _g_boolean_var_: libc::c_int = 0;
            if size
                == (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint)
            {
                _g_boolean_var_ = 1 as libc::c_int;
            } else {
                _g_boolean_var_ = 0 as libc::c_int;
            }
            _g_boolean_var_
        }) as libc::c_long != 0
        {
            return 0 as libc::c_int;
        }
        size = size.wrapping_add(1);
    }
    size = size.wrapping_add(1);
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            size.wrapping_mul((8 as libc::c_int / 8 as libc::c_int) as libc::c_uint),
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_data_inline(
        writer,
        data as *const guint8,
        size.wrapping_mul((8 as libc::c_int / 8 as libc::c_int) as libc::c_uint),
    );
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_string_utf16(
    mut writer: *mut FpiByteWriter,
    mut data: *const guint16,
) -> gboolean {
    let mut size: guint = 0 as libc::c_int as guint;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"fpi_byte_writer_put_string_utf16\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    while *data.offset(size as isize) as libc::c_int != 0 as libc::c_int {
        if ({
            let mut _g_boolean_var_: libc::c_int = 0;
            if size
                == (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint)
            {
                _g_boolean_var_ = 1 as libc::c_int;
            } else {
                _g_boolean_var_ = 0 as libc::c_int;
            }
            _g_boolean_var_
        }) as libc::c_long != 0
        {
            return 0 as libc::c_int;
        }
        size = size.wrapping_add(1);
    }
    size = size.wrapping_add(1);
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            size.wrapping_mul((16 as libc::c_int / 8 as libc::c_int) as libc::c_uint),
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_data_inline(
        writer,
        data as *const guint8,
        size.wrapping_mul((16 as libc::c_int / 8 as libc::c_int) as libc::c_uint),
    );
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_writer_put_string_utf32(
    mut writer: *mut FpiByteWriter,
    mut data: *const guint32,
) -> gboolean {
    let mut size: guint = 0 as libc::c_int as guint;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !writer.is_null() {
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
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"fpi_byte_writer_put_string_utf32\0"))
                .as_ptr(),
            b"writer != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    while *data.offset(size as isize) != 0 as libc::c_int as libc::c_uint {
        if ({
            let mut _g_boolean_var_: libc::c_int = 0;
            if size
                == (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint)
            {
                _g_boolean_var_ = 1 as libc::c_int;
            } else {
                _g_boolean_var_ = 0 as libc::c_int;
            }
            _g_boolean_var_
        }) as libc::c_long != 0
        {
            return 0 as libc::c_int;
        }
        size = size.wrapping_add(1);
    }
    size = size.wrapping_add(1);
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_writer_ensure_free_space_inline(
            writer,
            size.wrapping_mul((32 as libc::c_int / 8 as libc::c_int) as libc::c_uint),
        ) == 0
        {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    fpi_byte_writer_put_data_inline(
        writer,
        data as *const guint8,
        size.wrapping_mul((32 as libc::c_int / 8 as libc::c_int) as libc::c_uint),
    );
    return (0 as libc::c_int == 0) as libc::c_int;
}
