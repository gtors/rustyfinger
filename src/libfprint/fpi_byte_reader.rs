use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
#[inline]
unsafe extern "C" fn _fpi_slow_read64_be(mut data: *const guint8) -> guint64 {
    return (*data.offset(0 as libc::c_int as isize) as guint64) << 56 as libc::c_int
        | (*data.offset(1 as libc::c_int as isize) as guint64) << 48 as libc::c_int
        | (*data.offset(2 as libc::c_int as isize) as guint64) << 40 as libc::c_int
        | (*data.offset(3 as libc::c_int as isize) as guint64) << 32 as libc::c_int
        | (*data.offset(4 as libc::c_int as isize) as guint64) << 24 as libc::c_int
        | (*data.offset(5 as libc::c_int as isize) as guint64) << 16 as libc::c_int
        | (*data.offset(6 as libc::c_int as isize) as guint64) << 8 as libc::c_int
        | (*data.offset(7 as libc::c_int as isize) as guint64) << 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _fpi_slow_read64_le(mut data: *const guint8) -> guint64 {
    return (*data.offset(7 as libc::c_int as isize) as guint64) << 56 as libc::c_int
        | (*data.offset(6 as libc::c_int as isize) as guint64) << 48 as libc::c_int
        | (*data.offset(5 as libc::c_int as isize) as guint64) << 40 as libc::c_int
        | (*data.offset(4 as libc::c_int as isize) as guint64) << 32 as libc::c_int
        | (*data.offset(3 as libc::c_int as isize) as guint64) << 24 as libc::c_int
        | (*data.offset(2 as libc::c_int as isize) as guint64) << 16 as libc::c_int
        | (*data.offset(1 as libc::c_int as isize) as guint64) << 8 as libc::c_int
        | (*data.offset(0 as libc::c_int as isize) as guint64) << 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _fpi_slow_read32_be(mut data: *const guint8) -> guint32 {
    return (*data.offset(0 as libc::c_int as isize) as guint32) << 24 as libc::c_int
        | (*data.offset(1 as libc::c_int as isize) as guint32) << 16 as libc::c_int
        | (*data.offset(2 as libc::c_int as isize) as guint32) << 8 as libc::c_int
        | (*data.offset(3 as libc::c_int as isize) as guint32) << 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _fpi_slow_read32_le(mut data: *const guint8) -> guint32 {
    return (*data.offset(3 as libc::c_int as isize) as guint32) << 24 as libc::c_int
        | (*data.offset(2 as libc::c_int as isize) as guint32) << 16 as libc::c_int
        | (*data.offset(1 as libc::c_int as isize) as guint32) << 8 as libc::c_int
        | (*data.offset(0 as libc::c_int as isize) as guint32) << 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _fpi_slow_read24_be(mut data: *const guint8) -> guint32 {
    return (*data.offset(0 as libc::c_int as isize) as guint32) << 16 as libc::c_int
        | (*data.offset(1 as libc::c_int as isize) as guint32) << 8 as libc::c_int
        | (*data.offset(2 as libc::c_int as isize) as guint32) << 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _fpi_slow_read24_le(mut data: *const guint8) -> guint32 {
    return (*data.offset(2 as libc::c_int as isize) as guint32) << 16 as libc::c_int
        | (*data.offset(1 as libc::c_int as isize) as guint32) << 8 as libc::c_int
        | (*data.offset(0 as libc::c_int as isize) as guint32) << 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _fpi_slow_read16_be(mut data: *const guint8) -> guint16 {
    return ((*data.offset(0 as libc::c_int as isize) as guint16 as libc::c_int)
        << 8 as libc::c_int
        | (*data.offset(1 as libc::c_int as isize) as guint16 as libc::c_int)
            << 0 as libc::c_int) as guint16;
}
#[inline]
unsafe extern "C" fn _fpi_slow_read16_le(mut data: *const guint8) -> guint16 {
    return ((*data.offset(1 as libc::c_int as isize) as guint16 as libc::c_int)
        << 8 as libc::c_int
        | (*data.offset(0 as libc::c_int as isize) as guint16 as libc::c_int)
            << 0 as libc::c_int) as guint16;
}
#[inline]
unsafe extern "C" fn FP_READ_FLOAT_LE(mut data: *const guint8) -> gfloat {
    let mut u: C2RustUnnamed = C2RustUnnamed { i: 0 };
    u.i = _fpi_slow_read32_le(data);
    return u.f;
}
#[inline]
unsafe extern "C" fn FP_READ_FLOAT_BE(mut data: *const guint8) -> gfloat {
    let mut u: C2RustUnnamed_0 = C2RustUnnamed_0 { i: 0 };
    u.i = _fpi_slow_read32_be(data);
    return u.f;
}
#[inline]
unsafe extern "C" fn FP_READ_DOUBLE_LE(mut data: *const guint8) -> gdouble {
    let mut u: C2RustUnnamed_1 = C2RustUnnamed_1 { i: 0 };
    u.i = _fpi_slow_read64_le(data);
    return u.d;
}
#[inline]
unsafe extern "C" fn FP_READ_DOUBLE_BE(mut data: *const guint8) -> gdouble {
    let mut u: C2RustUnnamed_2 = C2RustUnnamed_2 { i: 0 };
    u.i = _fpi_slow_read64_be(data);
    return u.d;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_sub_reader_inline(
    mut reader: *mut FpiByteReader,
    mut sub_reader: *mut FpiByteReader,
    mut size: guint,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"fpi_byte_reader_peek_sub_reader_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !sub_reader.is_null() {
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
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"fpi_byte_reader_peek_sub_reader_inline\0"))
                .as_ptr(),
            b"sub_reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader) < size {
        return 0 as libc::c_int;
    }
    (*sub_reader).data = ((*reader).data).offset((*reader).byte as isize);
    (*sub_reader).byte = 0 as libc::c_int as guint;
    (*sub_reader).size = size;
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_remaining_unchecked(
    mut reader: *const FpiByteReader,
) -> guint {
    return ((*reader).size).wrapping_sub((*reader).byte);
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_sub_reader_inline(
    mut reader: *mut FpiByteReader,
    mut sub_reader: *mut FpiByteReader,
    mut size: guint,
) -> gboolean {
    if fpi_byte_reader_peek_sub_reader_inline(reader, sub_reader, size) == 0 {
        return 0 as libc::c_int;
    }
    fpi_byte_reader_skip_unchecked(reader, size);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_skip_unchecked(
    mut reader: *mut FpiByteReader,
    mut nbytes: guint,
) {
    (*reader)
        .byte = ((*reader).byte as libc::c_uint).wrapping_add(nbytes) as guint as guint;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_pos_inline(
    mut reader: *const FpiByteReader,
) -> guint {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_pos_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int as guint;
    }
    return fpi_byte_reader_get_pos_unchecked(reader);
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_pos_unchecked(
    mut reader: *const FpiByteReader,
) -> guint {
    return (*reader).byte;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_remaining_inline(
    mut reader: *const FpiByteReader,
) -> guint {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_remaining_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int as guint;
    }
    return fpi_byte_reader_get_remaining_unchecked(reader);
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_size_inline(
    mut reader: *const FpiByteReader,
) -> guint {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_size_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int as guint;
    }
    return fpi_byte_reader_get_size_unchecked(reader);
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_size_unchecked(
    mut reader: *const FpiByteReader,
) -> guint {
    return (*reader).size;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_skip_inline(
    mut reader: *mut FpiByteReader,
    mut nbytes: guint,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_skip_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if fpi_byte_reader_get_remaining_unchecked(reader) < nbytes {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    (*reader)
        .byte = ((*reader).byte as libc::c_uint).wrapping_add(nbytes) as guint as guint;
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint8_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint8,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_uint8_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_uint8_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (8 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_uint8_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint8_unchecked(
    mut reader: *mut FpiByteReader,
) -> guint8 {
    let mut val: guint8 = fpi_byte_reader_peek_uint8_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((8 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint8_unchecked(
    mut reader: *const FpiByteReader,
) -> guint8 {
    let mut val: guint8 = ((*((*reader).data)
        .offset((*reader).byte as isize)
        .offset(0 as libc::c_int as isize) as libc::c_int) << 0 as libc::c_int)
        as guint8;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int8_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint8,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_int8_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_int8_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (8 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_int8_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int8_unchecked(
    mut reader: *mut FpiByteReader,
) -> gint8 {
    let mut val: gint8 = fpi_byte_reader_peek_int8_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((8 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int8_unchecked(
    mut reader: *const FpiByteReader,
) -> gint8 {
    let mut val: gint8 = ((*((*reader).data)
        .offset((*reader).byte as isize)
        .offset(0 as libc::c_int as isize) as libc::c_int) << 0 as libc::c_int) as gint8;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint16_le_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint16,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_uint16_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_uint16_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (16 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_uint16_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint16_le_unchecked(
    mut reader: *mut FpiByteReader,
) -> guint16 {
    let mut val: guint16 = fpi_byte_reader_peek_uint16_le_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((16 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint16_le_unchecked(
    mut reader: *const FpiByteReader,
) -> guint16 {
    let mut val: guint16 = _fpi_slow_read16_le(
        ((*reader).data).offset((*reader).byte as isize),
    );
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int16_le_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint16,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_int16_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_int16_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (16 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_int16_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int16_le_unchecked(
    mut reader: *mut FpiByteReader,
) -> gint16 {
    let mut val: gint16 = fpi_byte_reader_peek_int16_le_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((16 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int16_le_unchecked(
    mut reader: *const FpiByteReader,
) -> gint16 {
    let mut val: gint16 = _fpi_slow_read16_le(
        ((*reader).data).offset((*reader).byte as isize),
    ) as gint16;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint16_be_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint16,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_uint16_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_uint16_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (16 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_uint16_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint16_be_unchecked(
    mut reader: *mut FpiByteReader,
) -> guint16 {
    let mut val: guint16 = fpi_byte_reader_peek_uint16_be_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((16 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint16_be_unchecked(
    mut reader: *const FpiByteReader,
) -> guint16 {
    let mut val: guint16 = _fpi_slow_read16_be(
        ((*reader).data).offset((*reader).byte as isize),
    );
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int16_be_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint16,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_int16_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_int16_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (16 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_int16_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int16_be_unchecked(
    mut reader: *mut FpiByteReader,
) -> gint16 {
    let mut val: gint16 = fpi_byte_reader_peek_int16_be_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((16 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int16_be_unchecked(
    mut reader: *const FpiByteReader,
) -> gint16 {
    let mut val: gint16 = _fpi_slow_read16_be(
        ((*reader).data).offset((*reader).byte as isize),
    ) as gint16;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint24_le_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_uint24_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_uint24_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (24 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_uint24_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint24_le_unchecked(
    mut reader: *mut FpiByteReader,
) -> guint32 {
    let mut val: guint32 = fpi_byte_reader_peek_uint24_le_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((24 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint24_le_unchecked(
    mut reader: *const FpiByteReader,
) -> guint32 {
    let mut val: guint32 = _fpi_slow_read24_le(
        ((*reader).data).offset((*reader).byte as isize),
    );
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int24_le_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_int24_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_int24_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (24 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_int24_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int24_le_unchecked(
    mut reader: *mut FpiByteReader,
) -> gint32 {
    let mut val: gint32 = fpi_byte_reader_peek_int24_le_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((24 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int24_le_unchecked(
    mut reader: *const FpiByteReader,
) -> gint32 {
    let mut val: gint32 = _fpi_slow_read24_le(
        ((*reader).data).offset((*reader).byte as isize),
    ) as gint32;
    if val & 0x800000 as libc::c_int != 0 {
        val = (val as libc::c_uint | 0xff000000 as libc::c_uint) as gint32;
    }
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint24_be_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_uint24_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_uint24_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (24 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_uint24_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint24_be_unchecked(
    mut reader: *mut FpiByteReader,
) -> guint32 {
    let mut val: guint32 = fpi_byte_reader_peek_uint24_be_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((24 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint24_be_unchecked(
    mut reader: *const FpiByteReader,
) -> guint32 {
    let mut val: guint32 = _fpi_slow_read24_be(
        ((*reader).data).offset((*reader).byte as isize),
    );
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int24_be_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_int24_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_int24_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (24 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_int24_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int24_be_unchecked(
    mut reader: *mut FpiByteReader,
) -> gint32 {
    let mut val: gint32 = fpi_byte_reader_peek_int24_be_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((24 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int24_be_unchecked(
    mut reader: *const FpiByteReader,
) -> gint32 {
    let mut val: gint32 = _fpi_slow_read24_be(
        ((*reader).data).offset((*reader).byte as isize),
    ) as gint32;
    if val & 0x800000 as libc::c_int != 0 {
        val = (val as libc::c_uint | 0xff000000 as libc::c_uint) as gint32;
    }
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint32_le_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_uint32_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_uint32_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (32 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_uint32_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint32_le_unchecked(
    mut reader: *mut FpiByteReader,
) -> guint32 {
    let mut val: guint32 = fpi_byte_reader_peek_uint32_le_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((32 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint32_le_unchecked(
    mut reader: *const FpiByteReader,
) -> guint32 {
    let mut val: guint32 = _fpi_slow_read32_le(
        ((*reader).data).offset((*reader).byte as isize),
    );
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int32_le_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_int32_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_int32_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (32 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_int32_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int32_le_unchecked(
    mut reader: *mut FpiByteReader,
) -> gint32 {
    let mut val: gint32 = fpi_byte_reader_peek_int32_le_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((32 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int32_le_unchecked(
    mut reader: *const FpiByteReader,
) -> gint32 {
    let mut val: gint32 = _fpi_slow_read32_le(
        ((*reader).data).offset((*reader).byte as isize),
    ) as gint32;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint32_be_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_uint32_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_uint32_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (32 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_uint32_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint32_be_unchecked(
    mut reader: *mut FpiByteReader,
) -> guint32 {
    let mut val: guint32 = fpi_byte_reader_peek_uint32_be_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((32 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint32_be_unchecked(
    mut reader: *const FpiByteReader,
) -> guint32 {
    let mut val: guint32 = _fpi_slow_read32_be(
        ((*reader).data).offset((*reader).byte as isize),
    );
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int32_be_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_int32_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_int32_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (32 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_int32_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int32_be_unchecked(
    mut reader: *mut FpiByteReader,
) -> gint32 {
    let mut val: gint32 = fpi_byte_reader_peek_int32_be_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((32 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int32_be_unchecked(
    mut reader: *const FpiByteReader,
) -> gint32 {
    let mut val: gint32 = _fpi_slow_read32_be(
        ((*reader).data).offset((*reader).byte as isize),
    ) as gint32;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint64_le_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint64,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_uint64_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_uint64_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (64 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_uint64_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint64_le_unchecked(
    mut reader: *mut FpiByteReader,
) -> guint64 {
    let mut val: guint64 = fpi_byte_reader_peek_uint64_le_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((64 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint64_le_unchecked(
    mut reader: *const FpiByteReader,
) -> guint64 {
    let mut val: guint64 = _fpi_slow_read64_le(
        ((*reader).data).offset((*reader).byte as isize),
    );
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int64_le_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint64,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_int64_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_int64_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (64 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_int64_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int64_le_unchecked(
    mut reader: *mut FpiByteReader,
) -> gint64 {
    let mut val: gint64 = fpi_byte_reader_peek_int64_le_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((64 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int64_le_unchecked(
    mut reader: *const FpiByteReader,
) -> gint64 {
    let mut val: gint64 = _fpi_slow_read64_le(
        ((*reader).data).offset((*reader).byte as isize),
    ) as gint64;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint64_be_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint64,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_uint64_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_uint64_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (64 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_uint64_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_uint64_be_unchecked(
    mut reader: *mut FpiByteReader,
) -> guint64 {
    let mut val: guint64 = fpi_byte_reader_peek_uint64_be_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((64 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint64_be_unchecked(
    mut reader: *const FpiByteReader,
) -> guint64 {
    let mut val: guint64 = _fpi_slow_read64_be(
        ((*reader).data).offset((*reader).byte as isize),
    );
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int64_be_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint64,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_int64_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_int64_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (64 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_int64_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_int64_be_unchecked(
    mut reader: *mut FpiByteReader,
) -> gint64 {
    let mut val: gint64 = fpi_byte_reader_peek_int64_be_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((64 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int64_be_unchecked(
    mut reader: *const FpiByteReader,
) -> gint64 {
    let mut val: gint64 = _fpi_slow_read64_be(
        ((*reader).data).offset((*reader).byte as isize),
    ) as gint64;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint8_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut guint8,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_uint8_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_uint8_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (8 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_uint8_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int8_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut gint8,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_int8_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_int8_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (8 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_int8_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint16_le_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut guint16,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_uint16_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_uint16_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (16 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_uint16_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int16_le_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut gint16,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_int16_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_int16_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (16 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_int16_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint16_be_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut guint16,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_uint16_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_uint16_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (16 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_uint16_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int16_be_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut gint16,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_int16_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_int16_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (16 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_int16_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint24_le_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_uint24_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_uint24_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (24 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_uint24_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int24_le_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut gint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_int24_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_int24_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (24 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_int24_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint24_be_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_uint24_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_uint24_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (24 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_uint24_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int24_be_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut gint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_int24_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_int24_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (24 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_int24_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint32_le_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_uint32_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_uint32_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (32 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_uint32_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int32_le_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut gint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_int32_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_int32_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (32 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_int32_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint32_be_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_uint32_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_uint32_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (32 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_uint32_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int32_be_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut gint32,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_int32_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_int32_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (32 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_int32_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint64_le_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut guint64,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_uint64_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_uint64_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (64 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_uint64_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int64_le_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut gint64,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_int64_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_int64_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (64 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_int64_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_uint64_be_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut guint64,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_uint64_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_uint64_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (64 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_uint64_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_int64_be_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut gint64,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_int64_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_int64_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (64 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_int64_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_float32_le_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut gfloat,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_float32_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_float32_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (32 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_float32_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_float32_le_unchecked(
    mut reader: *mut FpiByteReader,
) -> gfloat {
    let mut val: gfloat = fpi_byte_reader_peek_float32_le_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((32 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_float32_le_unchecked(
    mut reader: *const FpiByteReader,
) -> gfloat {
    let mut val: gfloat = FP_READ_FLOAT_LE(
        ((*reader).data).offset((*reader).byte as isize),
    );
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_float32_be_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut gfloat,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_float32_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_float32_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (32 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_float32_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_float32_be_unchecked(
    mut reader: *mut FpiByteReader,
) -> gfloat {
    let mut val: gfloat = fpi_byte_reader_peek_float32_be_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((32 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_float32_be_unchecked(
    mut reader: *const FpiByteReader,
) -> gfloat {
    let mut val: gfloat = FP_READ_FLOAT_BE(
        ((*reader).data).offset((*reader).byte as isize),
    );
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_float64_le_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut gdouble,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_float64_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_float64_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (64 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_float64_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_float64_le_unchecked(
    mut reader: *mut FpiByteReader,
) -> gdouble {
    let mut val: gdouble = fpi_byte_reader_peek_float64_le_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((64 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_float64_le_unchecked(
    mut reader: *const FpiByteReader,
) -> gdouble {
    let mut val: gdouble = FP_READ_DOUBLE_LE(
        ((*reader).data).offset((*reader).byte as isize),
    );
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_float64_be_inline(
    mut reader: *mut FpiByteReader,
    mut val: *mut gdouble,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_float64_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_float64_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (64 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_get_float64_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_float64_be_unchecked(
    mut reader: *mut FpiByteReader,
) -> gdouble {
    let mut val: gdouble = fpi_byte_reader_peek_float64_be_unchecked(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint)
        .wrapping_add((64 as libc::c_int / 8 as libc::c_int) as libc::c_uint) as guint
        as guint;
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_float64_be_unchecked(
    mut reader: *const FpiByteReader,
) -> gdouble {
    let mut val: gdouble = FP_READ_DOUBLE_BE(
        ((*reader).data).offset((*reader).byte as isize),
    );
    return val;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_float32_le_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut gfloat,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"fpi_byte_reader_peek_float32_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"fpi_byte_reader_peek_float32_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (32 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_float32_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_float32_be_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut gfloat,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"fpi_byte_reader_peek_float32_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"fpi_byte_reader_peek_float32_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (32 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_float32_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_float64_le_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut gdouble,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"fpi_byte_reader_peek_float64_le_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"fpi_byte_reader_peek_float64_le_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (64 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_float64_le_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_float64_be_inline(
    mut reader: *const FpiByteReader,
    mut val: *mut gdouble,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"fpi_byte_reader_peek_float64_be_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"fpi_byte_reader_peek_float64_be_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_get_remaining_unchecked(reader)
        < (64 as libc::c_int / 8 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *val = fpi_byte_reader_peek_float64_be_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_dup_data_inline(
    mut reader: *mut FpiByteReader,
    mut size: guint,
    mut val: *mut *mut guint8,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_dup_data_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_dup_data_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if size > (*reader).size
            || fpi_byte_reader_get_remaining_unchecked(reader) < size
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
    *val = fpi_byte_reader_dup_data_unchecked(reader, size);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_dup_data_unchecked(
    mut reader: *mut FpiByteReader,
    mut size: guint,
) -> *mut guint8 {
    let mut data: gconstpointer = fpi_byte_reader_get_data_unchecked(reader, size)
        as gconstpointer;
    return ({ g_memdup2(data, size as gsize) }) as *mut guint8;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_data_unchecked(
    mut reader: *mut FpiByteReader,
    mut size: guint,
) -> *const guint8 {
    let mut data: *const guint8 = 0 as *const guint8;
    data = fpi_byte_reader_peek_data_unchecked(reader);
    fpi_byte_reader_skip_unchecked(reader, size);
    return data;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_data_unchecked(
    mut reader: *const FpiByteReader,
) -> *const guint8 {
    return ((*reader).data).offset((*reader).byte as isize);
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_get_data_inline(
    mut reader: *mut FpiByteReader,
    mut size: guint,
    mut val: *mut *const guint8,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_data_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_get_data_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if size > (*reader).size
            || fpi_byte_reader_get_remaining_unchecked(reader) < size
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
    *val = fpi_byte_reader_get_data_unchecked(reader, size);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fpi_byte_reader_peek_data_inline(
    mut reader: *const FpiByteReader,
    mut size: guint,
    mut val: *mut *const guint8,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_data_inline\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !val.is_null() {
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
            >(b"fpi_byte_reader_peek_data_inline\0"))
                .as_ptr(),
            b"val != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if size > (*reader).size
            || fpi_byte_reader_get_remaining_unchecked(reader) < size
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
    *val = fpi_byte_reader_peek_data_unchecked(reader);
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_new(
    mut data: *const guint8,
    mut size: guint,
) -> *mut FpiByteReader {
    let mut ret: *mut FpiByteReader = ({
        let mut __s: gsize = ::core::mem::size_of::<FpiByteReader>() as libc::c_ulong;
        let mut __p: gpointer = 0 as *mut libc::c_void;
        __p = g_slice_alloc(__s);
        memset(__p, 0 as libc::c_int, __s);
        __p
    }) as *mut FpiByteReader;
    (*ret).data = data;
    (*ret).size = size;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_free(mut reader: *mut FpiByteReader) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_free\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    g_slice_free1(
        ::core::mem::size_of::<FpiByteReader>() as libc::c_ulong,
        reader as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_init(
    mut reader: *mut FpiByteReader,
    mut data: *const guint8,
    mut size: guint,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_init\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*reader).data = data;
    (*reader).size = size;
    (*reader).byte = 0 as libc::c_int as guint;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_sub_reader(
    mut reader: *mut FpiByteReader,
    mut sub_reader: *mut FpiByteReader,
    mut size: guint,
) -> gboolean {
    return fpi_byte_reader_peek_sub_reader_inline(reader, sub_reader, size);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_sub_reader(
    mut reader: *mut FpiByteReader,
    mut sub_reader: *mut FpiByteReader,
    mut size: guint,
) -> gboolean {
    return fpi_byte_reader_get_sub_reader_inline(reader, sub_reader, size);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_set_pos(
    mut reader: *mut FpiByteReader,
    mut pos: guint,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_set_pos\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if pos > (*reader).size {
        return 0 as libc::c_int;
    }
    (*reader).byte = pos;
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_pos(
    mut reader: *const FpiByteReader,
) -> guint {
    return fpi_byte_reader_get_pos_inline(reader);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_remaining(
    mut reader: *const FpiByteReader,
) -> guint {
    return fpi_byte_reader_get_remaining_inline(reader);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_size(
    mut reader: *const FpiByteReader,
) -> guint {
    return fpi_byte_reader_get_size_inline(reader);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_skip(
    mut reader: *mut FpiByteReader,
    mut nbytes: guint,
) -> gboolean {
    return fpi_byte_reader_skip_inline(reader, nbytes);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_uint8(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint8,
) -> gboolean {
    return fpi_byte_reader_get_uint8_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_uint8(
    mut reader: *const FpiByteReader,
    mut val: *mut guint8,
) -> gboolean {
    return fpi_byte_reader_peek_uint8_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_int8(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint8,
) -> gboolean {
    return fpi_byte_reader_get_int8_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_int8(
    mut reader: *const FpiByteReader,
    mut val: *mut gint8,
) -> gboolean {
    return fpi_byte_reader_peek_int8_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_uint16_le(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint16,
) -> gboolean {
    return fpi_byte_reader_get_uint16_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_uint16_le(
    mut reader: *const FpiByteReader,
    mut val: *mut guint16,
) -> gboolean {
    return fpi_byte_reader_peek_uint16_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_uint16_be(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint16,
) -> gboolean {
    return fpi_byte_reader_get_uint16_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_uint16_be(
    mut reader: *const FpiByteReader,
    mut val: *mut guint16,
) -> gboolean {
    return fpi_byte_reader_peek_uint16_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_int16_le(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint16,
) -> gboolean {
    return fpi_byte_reader_get_int16_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_int16_le(
    mut reader: *const FpiByteReader,
    mut val: *mut gint16,
) -> gboolean {
    return fpi_byte_reader_peek_int16_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_int16_be(
    mut reader: *const FpiByteReader,
    mut val: *mut gint16,
) -> gboolean {
    return fpi_byte_reader_peek_int16_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_int16_be(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint16,
) -> gboolean {
    return fpi_byte_reader_get_int16_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_uint24_le(
    mut reader: *const FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    return fpi_byte_reader_peek_uint24_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_uint24_le(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    return fpi_byte_reader_get_uint24_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_uint24_be(
    mut reader: *const FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    return fpi_byte_reader_peek_uint24_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_uint24_be(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    return fpi_byte_reader_get_uint24_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_int24_le(
    mut reader: *const FpiByteReader,
    mut val: *mut gint32,
) -> gboolean {
    return fpi_byte_reader_peek_int24_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_int24_le(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint32,
) -> gboolean {
    return fpi_byte_reader_get_int24_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_int24_be(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint32,
) -> gboolean {
    return fpi_byte_reader_get_int24_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_int24_be(
    mut reader: *const FpiByteReader,
    mut val: *mut gint32,
) -> gboolean {
    return fpi_byte_reader_peek_int24_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_uint32_le(
    mut reader: *const FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    return fpi_byte_reader_peek_uint32_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_uint32_le(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    return fpi_byte_reader_get_uint32_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_uint32_be(
    mut reader: *const FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    return fpi_byte_reader_peek_uint32_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_uint32_be(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint32,
) -> gboolean {
    return fpi_byte_reader_get_uint32_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_int32_le(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint32,
) -> gboolean {
    return fpi_byte_reader_get_int32_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_int32_le(
    mut reader: *const FpiByteReader,
    mut val: *mut gint32,
) -> gboolean {
    return fpi_byte_reader_peek_int32_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_int32_be(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint32,
) -> gboolean {
    return fpi_byte_reader_get_int32_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_int32_be(
    mut reader: *const FpiByteReader,
    mut val: *mut gint32,
) -> gboolean {
    return fpi_byte_reader_peek_int32_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_uint64_le(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint64,
) -> gboolean {
    return fpi_byte_reader_get_uint64_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_uint64_le(
    mut reader: *const FpiByteReader,
    mut val: *mut guint64,
) -> gboolean {
    return fpi_byte_reader_peek_uint64_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_uint64_be(
    mut reader: *const FpiByteReader,
    mut val: *mut guint64,
) -> gboolean {
    return fpi_byte_reader_peek_uint64_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_uint64_be(
    mut reader: *mut FpiByteReader,
    mut val: *mut guint64,
) -> gboolean {
    return fpi_byte_reader_get_uint64_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_int64_le(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint64,
) -> gboolean {
    return fpi_byte_reader_get_int64_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_int64_le(
    mut reader: *const FpiByteReader,
    mut val: *mut gint64,
) -> gboolean {
    return fpi_byte_reader_peek_int64_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_int64_be(
    mut reader: *mut FpiByteReader,
    mut val: *mut gint64,
) -> gboolean {
    return fpi_byte_reader_get_int64_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_int64_be(
    mut reader: *const FpiByteReader,
    mut val: *mut gint64,
) -> gboolean {
    return fpi_byte_reader_peek_int64_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_float32_le(
    mut reader: *const FpiByteReader,
    mut val: *mut gfloat,
) -> gboolean {
    return fpi_byte_reader_peek_float32_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_float32_le(
    mut reader: *mut FpiByteReader,
    mut val: *mut gfloat,
) -> gboolean {
    return fpi_byte_reader_get_float32_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_float32_be(
    mut reader: *const FpiByteReader,
    mut val: *mut gfloat,
) -> gboolean {
    return fpi_byte_reader_peek_float32_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_float32_be(
    mut reader: *mut FpiByteReader,
    mut val: *mut gfloat,
) -> gboolean {
    return fpi_byte_reader_get_float32_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_float64_le(
    mut reader: *const FpiByteReader,
    mut val: *mut gdouble,
) -> gboolean {
    return fpi_byte_reader_peek_float64_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_float64_le(
    mut reader: *mut FpiByteReader,
    mut val: *mut gdouble,
) -> gboolean {
    return fpi_byte_reader_get_float64_le_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_float64_be(
    mut reader: *mut FpiByteReader,
    mut val: *mut gdouble,
) -> gboolean {
    return fpi_byte_reader_get_float64_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_float64_be(
    mut reader: *const FpiByteReader,
    mut val: *mut gdouble,
) -> gboolean {
    return fpi_byte_reader_peek_float64_be_inline(reader, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_data(
    mut reader: *mut FpiByteReader,
    mut size: guint,
    mut val: *mut *const guint8,
) -> gboolean {
    return fpi_byte_reader_get_data_inline(reader, size, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_data(
    mut reader: *const FpiByteReader,
    mut size: guint,
    mut val: *mut *const guint8,
) -> gboolean {
    return fpi_byte_reader_peek_data_inline(reader, size, val);
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_dup_data(
    mut reader: *mut FpiByteReader,
    mut size: guint,
    mut val: *mut *mut guint8,
) -> gboolean {
    return fpi_byte_reader_dup_data_inline(reader, size, val);
}
#[inline]
unsafe extern "C" fn _scan_for_start_code(
    mut data: *const guint8,
    mut size: guint,
) -> gint {
    let mut pdata: *mut guint8 = data as *mut guint8;
    let mut pend: *mut guint8 = data
        .offset(size as isize)
        .offset(-(4 as libc::c_int as isize)) as *mut guint8;
    while pdata <= pend {
        if *pdata.offset(2 as libc::c_int as isize) as libc::c_int > 1 as libc::c_int {
            pdata = pdata.offset(3 as libc::c_int as isize);
        } else if *pdata.offset(1 as libc::c_int as isize) != 0 {
            pdata = pdata.offset(2 as libc::c_int as isize);
        } else if *pdata.offset(0 as libc::c_int as isize) as libc::c_int != 0
            || *pdata.offset(2 as libc::c_int as isize) as libc::c_int
                != 1 as libc::c_int
        {
            pdata = pdata.offset(1);
        } else {
            return pdata.offset_from(data) as libc::c_long as gint
        }
    }
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn _masked_scan_uint32_peek(
    mut reader: *const FpiByteReader,
    mut mask: guint32,
    mut pattern: guint32,
    mut offset: guint,
    mut size: guint,
    mut value: *mut guint32,
) -> guint {
    let mut data: *const guint8 = 0 as *const guint8;
    let mut state: guint32 = 0;
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if size > 0 as libc::c_int as libc::c_uint {
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
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"_masked_scan_uint32_peek\0"))
                .as_ptr(),
            b"size > 0\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int) as guint;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if (offset as guint64).wrapping_add(size as libc::c_ulong)
            <= ((*reader).size).wrapping_sub((*reader).byte) as libc::c_ulong
        {
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
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"_masked_scan_uint32_peek\0"))
                .as_ptr(),
            b"(guint64) offset + size <= reader->size - reader->byte\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int) as guint;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if size < 4 as libc::c_int as libc::c_uint {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {
        return -(1 as libc::c_int) as guint;
    }
    data = ((*reader).data).offset((*reader).byte as isize).offset(offset as isize);
    if pattern == 0x100 as libc::c_int as libc::c_uint
        && mask == 0xffffff00 as libc::c_uint
    {
        let mut ret: gint = _scan_for_start_code(data, size);
        if ret == -(1 as libc::c_int) {
            return ret as guint;
        }
        if !value.is_null() {
            *value = ((1 as libc::c_int) << 8 as libc::c_int
                | *data.offset((ret + 3 as libc::c_int) as isize) as libc::c_int)
                as guint32;
        }
        return (ret as libc::c_uint).wrapping_add(offset);
    }
    state = !pattern;
    i = 0 as libc::c_int as guint;
    while i < size {
        state = state << 8 as libc::c_int | *data.offset(i as isize) as libc::c_uint;
        if ({
            let mut _g_boolean_var_: libc::c_int = 0;
            if state & mask == pattern {
                _g_boolean_var_ = 1 as libc::c_int;
            } else {
                _g_boolean_var_ = 0 as libc::c_int;
            }
            _g_boolean_var_
        }) as libc::c_long != 0
        {
            if ({
                let mut _g_boolean_var_: libc::c_int = 0;
                if i >= 3 as libc::c_int as libc::c_uint {
                    _g_boolean_var_ = 1 as libc::c_int;
                } else {
                    _g_boolean_var_ = 0 as libc::c_int;
                }
                _g_boolean_var_
            }) as libc::c_long != 0
            {
                if !value.is_null() {
                    *value = state;
                }
                return offset
                    .wrapping_add(i)
                    .wrapping_sub(3 as libc::c_int as libc::c_uint);
            }
        }
        i = i.wrapping_add(1);
    }
    return -(1 as libc::c_int) as guint;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_masked_scan_uint32(
    mut reader: *const FpiByteReader,
    mut mask: guint32,
    mut pattern: guint32,
    mut offset: guint,
    mut size: guint,
) -> guint {
    return _masked_scan_uint32_peek(
        reader,
        mask,
        pattern,
        offset,
        size,
        0 as *mut guint32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_masked_scan_uint32_peek(
    mut reader: *const FpiByteReader,
    mut mask: guint32,
    mut pattern: guint32,
    mut offset: guint,
    mut size: guint,
    mut value: *mut guint32,
) -> guint {
    return _masked_scan_uint32_peek(reader, mask, pattern, offset, size, value);
}
unsafe extern "C" fn fpi_byte_reader_scan_string_utf8(
    mut reader: *const FpiByteReader,
) -> guint {
    let mut len: guint = 0;
    let mut off: guint = 0;
    let mut max_len: guint = 0;
    max_len = (((*reader).size).wrapping_sub((*reader).byte) as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<guint8>() as libc::c_ulong) as guint;
    if max_len < 1 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as guint;
    }
    len = 0 as libc::c_int as guint;
    off = (*reader).byte;
    while (*(&*((*reader).data).offset(off as isize) as *const guint8)
        .offset(0 as libc::c_int as isize) as libc::c_int) << 0 as libc::c_int
        != 0 as libc::c_int
    {
        len = len.wrapping_add(1);
        off = (off as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<guint8>() as libc::c_ulong) as guint
            as guint;
        if len == max_len {
            return 0 as libc::c_int as guint;
        }
    }
    return (len.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<guint8>() as libc::c_ulong) as guint;
}
unsafe extern "C" fn fpi_byte_reader_scan_string_utf16(
    mut reader: *const FpiByteReader,
) -> guint {
    let mut len: guint = 0;
    let mut off: guint = 0;
    let mut max_len: guint = 0;
    max_len = (((*reader).size).wrapping_sub((*reader).byte) as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<guint16>() as libc::c_ulong) as guint;
    if max_len < 1 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as guint;
    }
    len = 0 as libc::c_int as guint;
    off = (*reader).byte;
    while _fpi_slow_read16_le(&*((*reader).data).offset(off as isize) as *const guint8)
        as libc::c_int != 0 as libc::c_int
    {
        len = len.wrapping_add(1);
        off = (off as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<guint16>() as libc::c_ulong) as guint
            as guint;
        if len == max_len {
            return 0 as libc::c_int as guint;
        }
    }
    return (len.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<guint16>() as libc::c_ulong) as guint;
}
unsafe extern "C" fn fpi_byte_reader_scan_string_utf32(
    mut reader: *const FpiByteReader,
) -> guint {
    let mut len: guint = 0;
    let mut off: guint = 0;
    let mut max_len: guint = 0;
    max_len = (((*reader).size).wrapping_sub((*reader).byte) as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<guint32>() as libc::c_ulong) as guint;
    if max_len < 1 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as guint;
    }
    len = 0 as libc::c_int as guint;
    off = (*reader).byte;
    while _fpi_slow_read32_le(&*((*reader).data).offset(off as isize) as *const guint8)
        != 0 as libc::c_int as libc::c_uint
    {
        len = len.wrapping_add(1);
        off = (off as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<guint32>() as libc::c_ulong) as guint
            as guint;
        if len == max_len {
            return 0 as libc::c_int as guint;
        }
    }
    return (len.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<guint32>() as libc::c_ulong) as guint;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_skip_string_utf8(
    mut reader: *mut FpiByteReader,
) -> gboolean {
    let mut size: guint = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_skip_string_utf8\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    size = fpi_byte_reader_scan_string_utf8(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint).wrapping_add(size) as guint as guint;
    return (size > 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_skip_string_utf16(
    mut reader: *mut FpiByteReader,
) -> gboolean {
    let mut size: guint = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_skip_string_utf16\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    size = fpi_byte_reader_scan_string_utf16(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint).wrapping_add(size) as guint as guint;
    return (size > 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_skip_string_utf32(
    mut reader: *mut FpiByteReader,
) -> gboolean {
    let mut size: guint = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_skip_string_utf32\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    size = fpi_byte_reader_scan_string_utf32(reader);
    (*reader)
        .byte = ((*reader).byte as libc::c_uint).wrapping_add(size) as guint as guint;
    return (size > 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_peek_string_utf8(
    mut reader: *const FpiByteReader,
    mut str: *mut *const gchar,
) -> gboolean {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_peek_string_utf8\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !str.is_null() {
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
            >(b"fpi_byte_reader_peek_string_utf8\0"))
                .as_ptr(),
            b"str != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fpi_byte_reader_scan_string_utf8(reader) > 0 as libc::c_int as libc::c_uint {
        *str = ((*reader).data).offset((*reader).byte as isize) as *const gchar;
    } else {
        *str = 0 as *const gchar;
    }
    return (*str != 0 as *mut libc::c_void as *const gchar) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_get_string_utf8(
    mut reader: *mut FpiByteReader,
    mut str: *mut *const gchar,
) -> gboolean {
    let mut size: guint = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_get_string_utf8\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !str.is_null() {
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
            >(b"fpi_byte_reader_get_string_utf8\0"))
                .as_ptr(),
            b"str != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    size = fpi_byte_reader_scan_string_utf8(reader);
    if size == 0 as libc::c_int as libc::c_uint {
        *str = 0 as *const gchar;
        return 0 as libc::c_int;
    }
    *str = ((*reader).data).offset((*reader).byte as isize) as *const gchar;
    (*reader)
        .byte = ((*reader).byte as libc::c_uint).wrapping_add(size) as guint as guint;
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_dup_string_utf8(
    mut reader: *mut FpiByteReader,
    mut str: *mut *mut gchar,
) -> gboolean {
    let mut size: guint = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_dup_string_utf8\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !str.is_null() {
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
            >(b"fpi_byte_reader_dup_string_utf8\0"))
                .as_ptr(),
            b"str != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    size = fpi_byte_reader_scan_string_utf8(reader);
    if size == 0 as libc::c_int as libc::c_uint {
        *str = 0 as *mut gchar;
        return 0 as libc::c_int;
    }
    *str = ({
        g_memdup2(
            ((*reader).data).offset((*reader).byte as isize) as gconstpointer,
            size as gsize,
        )
    }) as *mut gchar;
    (*reader)
        .byte = ((*reader).byte as libc::c_uint).wrapping_add(size) as guint as guint;
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_dup_string_utf16(
    mut reader: *mut FpiByteReader,
    mut str: *mut *mut guint16,
) -> gboolean {
    let mut size: guint = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_dup_string_utf16\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !str.is_null() {
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
            >(b"fpi_byte_reader_dup_string_utf16\0"))
                .as_ptr(),
            b"str != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    size = fpi_byte_reader_scan_string_utf16(reader);
    if size == 0 as libc::c_int as libc::c_uint {
        *str = 0 as *mut guint16;
        return 0 as libc::c_int;
    }
    *str = ({
        g_memdup2(
            ((*reader).data).offset((*reader).byte as isize) as gconstpointer,
            size as gsize,
        )
    }) as *mut guint16;
    (*reader)
        .byte = ((*reader).byte as libc::c_uint).wrapping_add(size) as guint as guint;
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpi_byte_reader_dup_string_utf32(
    mut reader: *mut FpiByteReader,
    mut str: *mut *mut guint32,
) -> gboolean {
    let mut size: guint = 0;
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !reader.is_null() {
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
            >(b"fpi_byte_reader_dup_string_utf32\0"))
                .as_ptr(),
            b"reader != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !str.is_null() {
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
            >(b"fpi_byte_reader_dup_string_utf32\0"))
                .as_ptr(),
            b"str != NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    size = fpi_byte_reader_scan_string_utf32(reader);
    if size == 0 as libc::c_int as libc::c_uint {
        *str = 0 as *mut guint32;
        return 0 as libc::c_int;
    }
    *str = ({
        g_memdup2(
            ((*reader).data).offset((*reader).byte as isize) as gconstpointer,
            size as gsize,
        )
    }) as *mut guint32;
    (*reader)
        .byte = ((*reader).byte as libc::c_uint).wrapping_add(size) as guint as guint;
    return (0 as libc::c_int == 0) as libc::c_int;
}
