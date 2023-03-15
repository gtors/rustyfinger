use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn atanf(_: libc::c_float) -> libc::c_float;
    fn get_progname() -> *mut libc::c_char;
    fn get_probe_filename() -> *mut libc::c_char;
    fn get_gallery_filename() -> *mut libc::c_char;
    static mut colp: [[libc::c_int; 5]; 20000];
    static mut scolpt: [*mut libc::c_int; 20000];
    static mut fcolpt: [*mut libc::c_int; 20000];
    static mut sc: [libc::c_int; 20000];
    static mut yl: [[libc::c_int; 2000]; 2];
    static mut rq: [libc::c_int; 20000];
    static mut tq: [libc::c_int; 20000];
    static mut zz: [libc::c_int; 20000];
    static mut rx: [libc::c_int; 100];
    static mut mm: [libc::c_int; 100];
    static mut nn: [libc::c_int; 20];
    static mut qq: [libc::c_int; 4000];
    static mut rk: [libc::c_int; 20000];
    static mut cp: [libc::c_int; 20000];
    static mut rp: [libc::c_int; 20000];
    static mut rf: [[libc::c_int; 10]; 100];
    static mut cf: [[libc::c_int; 10]; 100];
    static mut bz_y: [libc::c_int; 20000];
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xyt_struct {
    pub nrows: libc::c_int,
    pub xcol: [libc::c_int; 200],
    pub ycol: [libc::c_int; 200],
    pub thetacol: [libc::c_int; 200],
}
#[no_mangle]
pub unsafe extern "C" fn bz_comp(
    mut npoints: libc::c_int,
    mut xcol: *mut libc::c_int,
    mut ycol: *mut libc::c_int,
    mut thetacol: *mut libc::c_int,
    mut ncomparisons: *mut libc::c_int,
    mut cols: *mut [libc::c_int; 6],
    mut colptrs: *mut *mut libc::c_int,
) {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut table_index: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut distance: libc::c_int = 0;
    let mut theta_kj: libc::c_int = 0;
    let mut beta_j: libc::c_int = 0;
    let mut beta_k: libc::c_int = 0;
    let mut c: *mut libc::c_int = 0 as *mut libc::c_int;
    c = &mut *(*cols.offset(0 as libc::c_int as isize))
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_int;
    table_index = 0 as libc::c_int;
    k = 0 as libc::c_int;
    's_34: while k < npoints - 1 as libc::c_int {
        j = k + 1 as libc::c_int;
        while j < npoints {
            if *thetacol.offset(j as isize) > 0 as libc::c_int {
                if *thetacol.offset(k as isize)
                    == *thetacol.offset(j as isize) - 180 as libc::c_int
                {
                    current_block = 13513818773234778473;
                } else {
                    current_block = 3512920355445576850;
                }
            } else if *thetacol.offset(k as isize)
                == *thetacol.offset(j as isize) + 180 as libc::c_int
            {
                current_block = 13513818773234778473;
            } else {
                current_block = 3512920355445576850;
            }
            match current_block {
                3512920355445576850 => {
                    dx = *xcol.offset(j as isize) - *xcol.offset(k as isize);
                    dy = *ycol.offset(j as isize) - *ycol.offset(k as isize);
                    distance = dx * dx + dy * dy;
                    if distance > 125 as libc::c_int * 125 as libc::c_int {
                        if dx > 125 as libc::c_int {
                            break;
                        }
                    } else {
                        if dx == 0 as libc::c_int {
                            theta_kj = 90 as libc::c_int;
                        } else {
                            let mut dz: libc::c_double = 0.;
                            dz = (180.0f32 / 3.14159265358979323846f64 as libc::c_float
                                * atanf(dy as libc::c_float / dx as libc::c_float))
                                as libc::c_double;
                            if dz < 0.0f32 as libc::c_double {
                                dz -= 0.5f32 as libc::c_double;
                            } else {
                                dz += 0.5f32 as libc::c_double;
                            }
                            theta_kj = dz as libc::c_int;
                        }
                        beta_k = theta_kj - *thetacol.offset(k as isize);
                        beta_k = if beta_k > 180 as libc::c_int {
                            beta_k - 360 as libc::c_int
                        } else if beta_k <= -(180 as libc::c_int) {
                            beta_k + 360 as libc::c_int
                        } else {
                            beta_k
                        };
                        beta_j = theta_kj - *thetacol.offset(j as isize)
                            + 180 as libc::c_int;
                        beta_j = if beta_j > 180 as libc::c_int {
                            beta_j - 360 as libc::c_int
                        } else if beta_j <= -(180 as libc::c_int) {
                            beta_j + 360 as libc::c_int
                        } else {
                            beta_j
                        };
                        if beta_k < beta_j {
                            let fresh0 = c;
                            c = c.offset(1);
                            *fresh0 = distance;
                            let fresh1 = c;
                            c = c.offset(1);
                            *fresh1 = beta_k;
                            let fresh2 = c;
                            c = c.offset(1);
                            *fresh2 = beta_j;
                            let fresh3 = c;
                            c = c.offset(1);
                            *fresh3 = k + 1 as libc::c_int;
                            let fresh4 = c;
                            c = c.offset(1);
                            *fresh4 = j + 1 as libc::c_int;
                            let fresh5 = c;
                            c = c.offset(1);
                            *fresh5 = theta_kj;
                        } else {
                            let fresh6 = c;
                            c = c.offset(1);
                            *fresh6 = distance;
                            let fresh7 = c;
                            c = c.offset(1);
                            *fresh7 = beta_j;
                            let fresh8 = c;
                            c = c.offset(1);
                            *fresh8 = beta_k;
                            let fresh9 = c;
                            c = c.offset(1);
                            *fresh9 = k + 1 as libc::c_int;
                            let fresh10 = c;
                            c = c.offset(1);
                            *fresh10 = j + 1 as libc::c_int;
                            let fresh11 = c;
                            c = c.offset(1);
                            *fresh11 = theta_kj + 400 as libc::c_int;
                        }
                        b = 0 as libc::c_int;
                        t = table_index + 1 as libc::c_int;
                        l = 1 as libc::c_int;
                        n = -(1 as libc::c_int);
                        while t - b > 1 as libc::c_int {
                            let mut midpoint: *mut libc::c_int = 0 as *mut libc::c_int;
                            l = (b + t) / 2 as libc::c_int;
                            midpoint = *colptrs.offset((l - 1 as libc::c_int) as isize);
                            i = 0 as libc::c_int;
                            while i < 3 as libc::c_int {
                                let mut dd: libc::c_int = 0;
                                let mut ff: libc::c_int = 0;
                                dd = (*cols.offset(table_index as isize))[i as usize];
                                ff = *midpoint.offset(i as isize);
                                n = if dd < ff {
                                    -(1 as libc::c_int)
                                } else if dd == ff {
                                    0 as libc::c_int
                                } else {
                                    1 as libc::c_int
                                };
                                if n < 0 as libc::c_int {
                                    t = l;
                                    break;
                                } else if n > 0 as libc::c_int {
                                    b = l;
                                    break;
                                } else {
                                    i += 1;
                                }
                            }
                            if n == 0 as libc::c_int {
                                n = 1 as libc::c_int;
                                b = l;
                            }
                        }
                        if n == 1 as libc::c_int {
                            l += 1;
                        }
                        i = table_index;
                        while i >= l {
                            let ref mut fresh12 = *colptrs.offset(i as isize);
                            *fresh12 = *colptrs.offset((i - 1 as libc::c_int) as isize);
                            i -= 1;
                        }
                        let ref mut fresh13 = *colptrs
                            .offset((l - 1 as libc::c_int) as isize);
                        *fresh13 = &mut *(*cols.offset(table_index as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut libc::c_int;
                        table_index += 1;
                        if table_index == 19999 as libc::c_int {
                            break 's_34;
                        }
                    }
                }
                _ => {}
            }
            j += 1;
        }
        k += 1;
    }
    *ncomparisons = table_index;
}
#[no_mangle]
pub unsafe extern "C" fn bz_find(
    mut xlim: *mut libc::c_int,
    mut colpt: *mut *mut libc::c_int,
) {
    let mut midpoint: libc::c_int = 0;
    let mut top: libc::c_int = 0;
    let mut bottom: libc::c_int = 0;
    let mut state: libc::c_int = 0;
    let mut distance: libc::c_int = 0;
    bottom = 0 as libc::c_int;
    top = *xlim + 1 as libc::c_int;
    midpoint = 1 as libc::c_int;
    state = -(1 as libc::c_int);
    while top - bottom > 1 as libc::c_int {
        midpoint = (bottom + top) / 2 as libc::c_int;
        distance = **colpt.offset((midpoint - 1 as libc::c_int) as isize);
        state = if (5625 as libc::c_int) < distance {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
        if state < 0 as libc::c_int {
            top = midpoint;
        } else {
            bottom = midpoint;
        }
    }
    if state > -(1 as libc::c_int) {
        midpoint += 1;
    }
    if midpoint < *xlim {
        *xlim = midpoint;
    }
}
unsafe extern "C" fn rtp_insert(
    mut rtp: *mut *mut libc::c_int,
    mut l: libc::c_int,
    mut idx: libc::c_int,
    mut ptr: *mut libc::c_int,
) {
    let mut shiftcount: libc::c_int = 0;
    let mut r1: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    let mut r2: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    r1 = &mut *rtp.offset(idx as isize) as *mut *mut libc::c_int;
    r2 = r1.offset(-(1 as libc::c_int as isize));
    shiftcount = idx - l + 1 as libc::c_int;
    loop {
        let fresh14 = shiftcount;
        shiftcount = shiftcount - 1;
        if !(fresh14 > 0 as libc::c_int) {
            break;
        }
        let fresh15 = r2;
        r2 = r2.offset(-1);
        let fresh16 = r1;
        r1 = r1.offset(-1);
        *fresh16 = *fresh15;
    }
    *r1 = ptr;
}
#[no_mangle]
pub unsafe extern "C" fn bz_match(
    mut probe_ptrlist_len: libc::c_int,
    mut gallery_ptrlist_len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut edge_pair_index: libc::c_int = 0;
    let mut dz: libc::c_float = 0.;
    let mut fi: libc::c_float = 0.;
    let mut ss: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ff: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut st: libc::c_int = 0;
    let mut p1: libc::c_int = 0;
    let mut p2: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut rotptr: *mut libc::c_int = 0 as *mut libc::c_int;
    static mut rot: [[libc::c_int; 5]; 20000] = [[0; 5]; 20000];
    static mut rtp: [*mut libc::c_int; 20000] = [0 as *const libc::c_int
        as *mut libc::c_int; 20000];
    st = 1 as libc::c_int;
    edge_pair_index = 0 as libc::c_int;
    rotptr = &mut *(*rot.as_mut_ptr().offset(0 as libc::c_int as isize))
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_int;
    k = 1 as libc::c_int;
    's_49: while k < probe_ptrlist_len {
        ss = scolpt[(k - 1 as libc::c_int) as usize];
        j = st;
        while j <= gallery_ptrlist_len {
            ff = fcolpt[(j - 1 as libc::c_int) as usize];
            dz = (*ff - *ss) as libc::c_float;
            fi = 2.0f32 * 0.05f32 * (*ff + *ss) as libc::c_float;
            if dz * dz > fi * fi {
                if !(dz < 0 as libc::c_int as libc::c_float) {
                    break;
                }
                st = j + 1 as libc::c_int;
            } else {
                i = 1 as libc::c_int;
                while i < 3 as libc::c_int {
                    let mut dz_squared: libc::c_float = 0.;
                    dz = (*ss.offset(i as isize) - *ff.offset(i as isize))
                        as libc::c_float;
                    dz_squared = dz * dz;
                    if dz_squared > 121 as libc::c_int as libc::c_float
                        && dz_squared < 121801 as libc::c_int as libc::c_float
                    {
                        break;
                    }
                    i += 1;
                }
                if !(i < 3 as libc::c_int) {
                    if *ss.offset(5 as libc::c_int as isize) >= 220 as libc::c_int {
                        p1 = *ss.offset(5 as libc::c_int as isize) - 580 as libc::c_int;
                        n = 1 as libc::c_int;
                    } else {
                        p1 = *ss.offset(5 as libc::c_int as isize);
                        n = 0 as libc::c_int;
                    }
                    if *ff.offset(5 as libc::c_int as isize) >= 220 as libc::c_int {
                        p2 = *ff.offset(5 as libc::c_int as isize) - 580 as libc::c_int;
                        b = 1 as libc::c_int;
                    } else {
                        p2 = *ff.offset(5 as libc::c_int as isize);
                        b = 0 as libc::c_int;
                    }
                    p1 -= p2;
                    p1 = if p1 > 180 as libc::c_int {
                        p1 - 360 as libc::c_int
                    } else if p1 <= -(180 as libc::c_int) {
                        p1 + 360 as libc::c_int
                    } else {
                        p1
                    };
                    if n != b {
                        let fresh17 = rotptr;
                        rotptr = rotptr.offset(1);
                        *fresh17 = p1;
                        let fresh18 = rotptr;
                        rotptr = rotptr.offset(1);
                        *fresh18 = *ss.offset(3 as libc::c_int as isize);
                        let fresh19 = rotptr;
                        rotptr = rotptr.offset(1);
                        *fresh19 = *ss.offset(4 as libc::c_int as isize);
                        let fresh20 = rotptr;
                        rotptr = rotptr.offset(1);
                        *fresh20 = *ff.offset(4 as libc::c_int as isize);
                        let fresh21 = rotptr;
                        rotptr = rotptr.offset(1);
                        *fresh21 = *ff.offset(3 as libc::c_int as isize);
                    } else {
                        let fresh22 = rotptr;
                        rotptr = rotptr.offset(1);
                        *fresh22 = p1;
                        let fresh23 = rotptr;
                        rotptr = rotptr.offset(1);
                        *fresh23 = *ss.offset(3 as libc::c_int as isize);
                        let fresh24 = rotptr;
                        rotptr = rotptr.offset(1);
                        *fresh24 = *ss.offset(4 as libc::c_int as isize);
                        let fresh25 = rotptr;
                        rotptr = rotptr.offset(1);
                        *fresh25 = *ff.offset(3 as libc::c_int as isize);
                        let fresh26 = rotptr;
                        rotptr = rotptr.offset(1);
                        *fresh26 = *ff.offset(4 as libc::c_int as isize);
                    }
                    n = -(1 as libc::c_int);
                    l = 1 as libc::c_int;
                    b = 0 as libc::c_int;
                    t = edge_pair_index + 1 as libc::c_int;
                    while t - b > 1 as libc::c_int {
                        l = (b + t) / 2 as libc::c_int;
                        i = 0 as libc::c_int;
                        while i < 3 as libc::c_int {
                            static mut ii_table: [libc::c_int; 3] = [
                                1 as libc::c_int,
                                3 as libc::c_int,
                                2 as libc::c_int,
                            ];
                            ii = ii_table[i as usize];
                            p1 = rot[edge_pair_index as usize][ii as usize];
                            p2 = *(rtp[(l - 1 as libc::c_int) as usize])
                                .offset(ii as isize);
                            n = if p1 < p2 {
                                -(1 as libc::c_int)
                            } else if p1 == p2 {
                                0 as libc::c_int
                            } else {
                                1 as libc::c_int
                            };
                            if n < 0 as libc::c_int {
                                t = l;
                                break;
                            } else if n > 0 as libc::c_int {
                                b = l;
                                break;
                            } else {
                                i += 1;
                            }
                        }
                        if n == 0 as libc::c_int {
                            n = 1 as libc::c_int;
                            b = l;
                        }
                    }
                    if n == 1 as libc::c_int {
                        l += 1;
                    }
                    rtp_insert(
                        rtp.as_mut_ptr(),
                        l,
                        edge_pair_index,
                        &mut *(*rot.as_mut_ptr().offset(edge_pair_index as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize),
                    );
                    edge_pair_index += 1;
                    if edge_pair_index == 19999 as libc::c_int {
                        break 's_49;
                    }
                }
            }
            j += 1;
        }
        k += 1;
    }
    let mut colp_ptr: *mut libc::c_int = &mut *(*colp
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < edge_pair_index {
        let mut int_copy_src: *mut libc::c_int = rtp[i as usize];
        let mut int_copy_count: libc::c_int = 5 as libc::c_int;
        loop {
            let fresh27 = int_copy_count;
            int_copy_count = int_copy_count - 1;
            if !(fresh27 > 0 as libc::c_int) {
                break;
            }
            let fresh28 = int_copy_src;
            int_copy_src = int_copy_src.offset(1);
            let fresh29 = colp_ptr;
            colp_ptr = colp_ptr.offset(1);
            *fresh29 = *fresh28;
        }
        i += 1;
    }
    return edge_pair_index;
}
static mut ct: [libc::c_int; 2000] = [0; 2000];
static mut gct: [libc::c_int; 2000] = [0; 2000];
static mut ctt: [libc::c_int; 2000] = [0; 2000];
static mut ctp: [[libc::c_int; 2500]; 2000] = [[0; 2500]; 2000];
static mut yy: [[[libc::c_int; 2000]; 2]; 1000] = [[[0; 2000]; 2]; 1000];
#[no_mangle]
pub unsafe extern "C" fn bz_match_score(
    mut np: libc::c_int,
    mut pstruct: *mut xyt_struct,
    mut gstruct: *mut xyt_struct,
) -> libc::c_int {
    let mut kx: libc::c_int = 0;
    let mut kq: libc::c_int = 0;
    let mut ftt: libc::c_int = 0;
    let mut tot: libc::c_int = 0;
    let mut qh: libc::c_int = 0;
    let mut tp: libc::c_int = 0;
    let mut ll: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut kz: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut p1: libc::c_int = 0;
    let mut p2: libc::c_int = 0;
    let mut dw: libc::c_int = 0;
    let mut ww: libc::c_int = 0;
    let mut match_score: libc::c_int = 0;
    let mut qq_overflow: libc::c_int = 0 as libc::c_int;
    let mut fi: libc::c_float = 0.;
    let mut rr: [libc::c_int; 100] = [0; 100];
    let mut avn: [libc::c_int; 5] = [0; 5];
    let mut avv: [[libc::c_int; 5]; 2000] = [[0; 5]; 2000];
    if (*pstruct).nrows < 10 as libc::c_int {
        (*gstruct).nrows < 10 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*gstruct).nrows < 10 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut int_set_dst: *mut libc::c_int = &mut yl as *mut [[libc::c_int; 2000]; 2]
        as *mut libc::c_int;
    let mut int_set_count: libc::c_int = 2 as libc::c_int * 2000 as libc::c_int;
    let mut int_set_value: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh30 = int_set_count;
        int_set_count = int_set_count - 1;
        if !(fresh30 > 0 as libc::c_int) {
            break;
        }
        let fresh31 = int_set_dst;
        int_set_dst = int_set_dst.offset(1);
        *fresh31 = int_set_value;
    }
    let mut int_set_dst_0: *mut libc::c_int = &mut sc as *mut [libc::c_int; 20000]
        as *mut libc::c_int;
    let mut int_set_count_0: libc::c_int = 20000 as libc::c_int;
    let mut int_set_value_0: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh32 = int_set_count_0;
        int_set_count_0 = int_set_count_0 - 1;
        if !(fresh32 > 0 as libc::c_int) {
            break;
        }
        let fresh33 = int_set_dst_0;
        int_set_dst_0 = int_set_dst_0.offset(1);
        *fresh33 = int_set_value_0;
    }
    let mut int_set_dst_1: *mut libc::c_int = &mut cp as *mut [libc::c_int; 20000]
        as *mut libc::c_int;
    let mut int_set_count_1: libc::c_int = 20000 as libc::c_int;
    let mut int_set_value_1: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh34 = int_set_count_1;
        int_set_count_1 = int_set_count_1 - 1;
        if !(fresh34 > 0 as libc::c_int) {
            break;
        }
        let fresh35 = int_set_dst_1;
        int_set_dst_1 = int_set_dst_1.offset(1);
        *fresh35 = int_set_value_1;
    }
    let mut int_set_dst_2: *mut libc::c_int = &mut rp as *mut [libc::c_int; 20000]
        as *mut libc::c_int;
    let mut int_set_count_2: libc::c_int = 20000 as libc::c_int;
    let mut int_set_value_2: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh36 = int_set_count_2;
        int_set_count_2 = int_set_count_2 - 1;
        if !(fresh36 > 0 as libc::c_int) {
            break;
        }
        let fresh37 = int_set_dst_2;
        int_set_dst_2 = int_set_dst_2.offset(1);
        *fresh37 = int_set_value_2;
    }
    let mut int_set_dst_3: *mut libc::c_int = &mut tq as *mut [libc::c_int; 20000]
        as *mut libc::c_int;
    let mut int_set_count_3: libc::c_int = 20000 as libc::c_int;
    let mut int_set_value_3: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh38 = int_set_count_3;
        int_set_count_3 = int_set_count_3 - 1;
        if !(fresh38 > 0 as libc::c_int) {
            break;
        }
        let fresh39 = int_set_dst_3;
        int_set_dst_3 = int_set_dst_3.offset(1);
        *fresh39 = int_set_value_3;
    }
    let mut int_set_dst_4: *mut libc::c_int = &mut rq as *mut [libc::c_int; 20000]
        as *mut libc::c_int;
    let mut int_set_count_4: libc::c_int = 20000 as libc::c_int;
    let mut int_set_value_4: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh40 = int_set_count_4;
        int_set_count_4 = int_set_count_4 - 1;
        if !(fresh40 > 0 as libc::c_int) {
            break;
        }
        let fresh41 = int_set_dst_4;
        int_set_dst_4 = int_set_dst_4.offset(1);
        *fresh41 = int_set_value_4;
    }
    let mut int_set_dst_5: *mut libc::c_int = &mut zz as *mut [libc::c_int; 20000]
        as *mut libc::c_int;
    let mut int_set_count_5: libc::c_int = 20000 as libc::c_int;
    let mut int_set_value_5: libc::c_int = 1000 as libc::c_int;
    loop {
        let fresh42 = int_set_count_5;
        int_set_count_5 = int_set_count_5 - 1;
        if !(fresh42 > 0 as libc::c_int) {
            break;
        }
        let fresh43 = int_set_dst_5;
        int_set_dst_5 = int_set_dst_5.offset(1);
        *fresh43 = int_set_value_5;
    }
    let mut int_set_dst_6: *mut libc::c_int = &mut avn as *mut [libc::c_int; 5]
        as *mut libc::c_int;
    let mut int_set_count_6: libc::c_int = 5 as libc::c_int;
    let mut int_set_value_6: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh44 = int_set_count_6;
        int_set_count_6 = int_set_count_6 - 1;
        if !(fresh44 > 0 as libc::c_int) {
            break;
        }
        let fresh45 = int_set_dst_6;
        int_set_dst_6 = int_set_dst_6.offset(1);
        *fresh45 = int_set_value_6;
    }
    tp = 0 as libc::c_int;
    p1 = 0 as libc::c_int;
    tot = 0 as libc::c_int;
    ftt = 0 as libc::c_int;
    match_score = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < np - 1 as libc::c_int {
        if !(sc[k as usize] != 0) {
            i = colp[k as usize][1 as libc::c_int as usize];
            t = colp[k as usize][3 as libc::c_int as usize];
            qq[0 as libc::c_int as usize] = i;
            rq[(t - 1 as libc::c_int) as usize] = i;
            tq[(i - 1 as libc::c_int) as usize] = t;
            ww = 0 as libc::c_int;
            dw = 0 as libc::c_int;
            loop {
                ftt += 1;
                tot = 0 as libc::c_int;
                qh = 1 as libc::c_int;
                kx = k;
                loop {
                    kz = colp[kx as usize][2 as libc::c_int as usize];
                    l = colp[kx as usize][4 as libc::c_int as usize];
                    kx += 1;
                    bz_sift(
                        &mut ww,
                        kz,
                        &mut qh,
                        l,
                        kx,
                        ftt,
                        &mut tot,
                        &mut qq_overflow,
                    );
                    if qq_overflow != 0 {
                        fprintf(
                            stderr,
                            b"%s: WARNING: bz_match_score(): qq[] overflow from bz_sift() #1 [p=%s; g=%s]\n\0"
                                as *const u8 as *const libc::c_char,
                            get_progname(),
                            get_probe_filename(),
                            get_gallery_filename(),
                        );
                        return 4000 as libc::c_int;
                    }
                    if !(colp[kx as usize][3 as libc::c_int as usize]
                        == colp[k as usize][3 as libc::c_int as usize]
                        && colp[kx as usize][1 as libc::c_int as usize]
                            == colp[k as usize][1 as libc::c_int as usize])
                    {
                        break;
                    }
                }
                kq = kx;
                j = 1 as libc::c_int;
                while j < qh {
                    i = kq;
                    while i < np {
                        z = 1 as libc::c_int;
                        while z < 3 as libc::c_int {
                            if z == 1 as libc::c_int {
                                if j + 1 as libc::c_int > 4000 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"%s: WARNING: bz_match_score(): qq[] overflow #1 in bozorth3(); j-1 is %d [p=%s; g=%s]\n\0"
                                            as *const u8 as *const libc::c_char,
                                        get_progname(),
                                        j - 1 as libc::c_int,
                                        get_probe_filename(),
                                        get_gallery_filename(),
                                    );
                                    return 4000 as libc::c_int;
                                }
                                p1 = qq[j as usize];
                            } else {
                                p1 = tq[(p1 - 1 as libc::c_int) as usize];
                            }
                            if colp[i as usize][(2 as libc::c_int * z) as usize] != p1 {
                                break;
                            }
                            z += 1;
                        }
                        if z == 3 as libc::c_int {
                            z = colp[i as usize][1 as libc::c_int as usize];
                            l = colp[i as usize][3 as libc::c_int as usize];
                            if z != colp[k as usize][1 as libc::c_int as usize]
                                && l != colp[k as usize][3 as libc::c_int as usize]
                            {
                                kx = i + 1 as libc::c_int;
                                bz_sift(
                                    &mut ww,
                                    z,
                                    &mut qh,
                                    l,
                                    kx,
                                    ftt,
                                    &mut tot,
                                    &mut qq_overflow,
                                );
                                if qq_overflow != 0 {
                                    fprintf(
                                        stderr,
                                        b"%s: WARNING: bz_match_score(): qq[] overflow from bz_sift() #2 [p=%s; g=%s]\n\0"
                                            as *const u8 as *const libc::c_char,
                                        get_progname(),
                                        get_probe_filename(),
                                        get_gallery_filename(),
                                    );
                                    return 4000 as libc::c_int;
                                }
                            }
                        }
                        i += 1;
                    }
                    t = np + 1 as libc::c_int;
                    b = kq;
                    while t - b > 1 as libc::c_int {
                        l = (b + t) / 2 as libc::c_int;
                        i = 1 as libc::c_int;
                        while i < 3 as libc::c_int {
                            if i == 1 as libc::c_int {
                                if j + 1 as libc::c_int > 4000 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"%s: WARNING: bz_match_score(): qq[] overflow #2 in bozorth3(); j-1 is %d [p=%s; g=%s]\n\0"
                                            as *const u8 as *const libc::c_char,
                                        get_progname(),
                                        j - 1 as libc::c_int,
                                        get_probe_filename(),
                                        get_gallery_filename(),
                                    );
                                    return 4000 as libc::c_int;
                                }
                                p1 = qq[j as usize];
                            } else {
                                p1 = tq[(p1 - 1 as libc::c_int) as usize];
                            }
                            p2 = colp[(l - 1 as libc::c_int)
                                as usize][(i * 2 as libc::c_int - 1 as libc::c_int)
                                as usize];
                            n = if p1 < p2 {
                                -(1 as libc::c_int)
                            } else if p1 == p2 {
                                0 as libc::c_int
                            } else {
                                1 as libc::c_int
                            };
                            if n < 0 as libc::c_int {
                                t = l;
                                break;
                            } else if n > 0 as libc::c_int {
                                b = l;
                                break;
                            } else {
                                i += 1;
                            }
                        }
                        if !(n == 0 as libc::c_int) {
                            continue;
                        }
                        while colp[(l - 2 as libc::c_int)
                            as usize][3 as libc::c_int as usize] == p2
                            && colp[(l - 2 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                                == colp[(l - 1 as libc::c_int)
                                    as usize][1 as libc::c_int as usize]
                        {
                            l -= 1;
                        }
                        kx = l - 1 as libc::c_int;
                        loop {
                            kz = colp[kx as usize][2 as libc::c_int as usize];
                            l = colp[kx as usize][4 as libc::c_int as usize];
                            kx += 1;
                            bz_sift(
                                &mut ww,
                                kz,
                                &mut qh,
                                l,
                                kx,
                                ftt,
                                &mut tot,
                                &mut qq_overflow,
                            );
                            if qq_overflow != 0 {
                                fprintf(
                                    stderr,
                                    b"%s: WARNING: bz_match_score(): qq[] overflow from bz_sift() #3 [p=%s; g=%s]\n\0"
                                        as *const u8 as *const libc::c_char,
                                    get_progname(),
                                    get_probe_filename(),
                                    get_gallery_filename(),
                                );
                                return 4000 as libc::c_int;
                            }
                            if !(colp[kx as usize][3 as libc::c_int as usize] == p2
                                && colp[kx as usize][1 as libc::c_int as usize]
                                    == colp[(kx - 1 as libc::c_int)
                                        as usize][1 as libc::c_int as usize])
                            {
                                break;
                            }
                        }
                        break;
                    }
                    j += 1;
                }
                if tot >= 3 as libc::c_int {
                    jj = 0 as libc::c_int;
                    kk = 0 as libc::c_int;
                    n = 0 as libc::c_int;
                    l = 0 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < tot {
                        let mut colp_value: libc::c_int = colp[(bz_y[i as usize]
                            - 1 as libc::c_int) as usize][0 as libc::c_int as usize];
                        if colp_value < 0 as libc::c_int {
                            kk += colp_value;
                            n += 1;
                        } else {
                            jj += colp_value;
                            l += 1;
                        }
                        i += 1;
                    }
                    if n == 0 as libc::c_int {
                        n = 1 as libc::c_int;
                    } else if l == 0 as libc::c_int {
                        l = 1 as libc::c_int;
                    }
                    fi = jj as libc::c_float / l as libc::c_float
                        - kk as libc::c_float / n as libc::c_float;
                    if fi > 180.0f32 {
                        fi = (jj + kk + n * 360 as libc::c_int) as libc::c_float
                            / tot as libc::c_float;
                        if fi > 180.0f32 {
                            fi -= 360.0f32;
                        }
                    } else {
                        fi = (jj + kk) as libc::c_float / tot as libc::c_float;
                    }
                    jj = if fi < 0.0f32 {
                        (fi - 0.5f32) as libc::c_int
                    } else {
                        (fi + 0.5f32) as libc::c_int
                    };
                    if jj <= -(180 as libc::c_int) {
                        jj += 360 as libc::c_int;
                    }
                    kk = 0 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < tot {
                        let mut diff: libc::c_int = colp[(bz_y[i as usize]
                            - 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                            - jj;
                        j = diff * diff;
                        if j > 121 as libc::c_int && j < 121801 as libc::c_int {
                            kk += 1;
                        } else {
                            bz_y[(i - kk) as usize] = bz_y[i as usize];
                        }
                        i += 1;
                    }
                    tot -= kk;
                }
                if tot < 3 as libc::c_int {
                    i = tot - 1 as libc::c_int;
                    while i >= 0 as libc::c_int {
                        let mut idx: libc::c_int = bz_y[i as usize] - 1 as libc::c_int;
                        if rk[idx as usize] == 0 as libc::c_int {
                            sc[idx as usize] = -(1 as libc::c_int);
                        } else {
                            sc[idx as usize] = rk[idx as usize];
                        }
                        i -= 1;
                    }
                    ftt -= 1;
                } else {
                    let mut pa: libc::c_int = 0 as libc::c_int;
                    let mut pb: libc::c_int = 0 as libc::c_int;
                    let mut pc: libc::c_int = 0 as libc::c_int;
                    let mut pd: libc::c_int = 0 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < tot {
                        let mut idx_0: libc::c_int = bz_y[i as usize] - 1 as libc::c_int;
                        ii = 1 as libc::c_int;
                        while ii < 4 as libc::c_int {
                            kk = (ii * ii - ii + 2 as libc::c_int) / 2 as libc::c_int
                                - 1 as libc::c_int;
                            jj = colp[idx_0 as usize][kk as usize];
                            match ii {
                                1 => {
                                    if colp[idx_0 as usize][0 as libc::c_int as usize]
                                        < 0 as libc::c_int
                                    {
                                        pd += colp[idx_0 as usize][0 as libc::c_int as usize];
                                        pb += 1;
                                    } else {
                                        pa += colp[idx_0 as usize][0 as libc::c_int as usize];
                                        pc += 1;
                                    }
                                }
                                2 => {
                                    avn[(ii - 1 as libc::c_int) as usize]
                                        += (*pstruct).xcol[(jj - 1 as libc::c_int) as usize];
                                    avn[ii as usize]
                                        += (*pstruct).ycol[(jj - 1 as libc::c_int) as usize];
                                }
                                _ => {
                                    avn[ii as usize]
                                        += (*gstruct).xcol[(jj - 1 as libc::c_int) as usize];
                                    avn[(ii + 1 as libc::c_int) as usize]
                                        += (*gstruct).ycol[(jj - 1 as libc::c_int) as usize];
                                }
                            }
                            ii += 1;
                        }
                        ii = 0 as libc::c_int;
                        while ii < 2 as libc::c_int {
                            n = -(1 as libc::c_int);
                            l = 1 as libc::c_int;
                            jj = 1 as libc::c_int;
                            while jj < 3 as libc::c_int {
                                p1 = colp[idx_0
                                    as usize][(2 as libc::c_int * ii + jj) as usize];
                                b = 0 as libc::c_int;
                                t = yl[ii as usize][tp as usize] + 1 as libc::c_int;
                                while t - b > 1 as libc::c_int {
                                    l = (b + t) / 2 as libc::c_int;
                                    p2 = yy[(l - 1 as libc::c_int)
                                        as usize][ii as usize][tp as usize];
                                    n = if p1 < p2 {
                                        -(1 as libc::c_int)
                                    } else if p1 == p2 {
                                        0 as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                    };
                                    if n < 0 as libc::c_int {
                                        t = l;
                                    } else {
                                        if !(n > 0 as libc::c_int) {
                                            break;
                                        }
                                        b = l;
                                    }
                                }
                                if n != 0 as libc::c_int {
                                    if n == 1 as libc::c_int {
                                        l += 1;
                                    }
                                    kk = yl[ii as usize][tp as usize];
                                    while kk >= l {
                                        yy[kk
                                            as usize][ii
                                            as usize][tp
                                            as usize] = yy[(kk - 1 as libc::c_int)
                                            as usize][ii as usize][tp as usize];
                                        kk -= 1;
                                    }
                                    yl[ii as usize][tp as usize] += 1;
                                    yy[(l - 1 as libc::c_int)
                                        as usize][ii as usize][tp as usize] = p1;
                                }
                                jj += 1;
                            }
                            ii += 1;
                        }
                        i += 1;
                    }
                    if pb == 0 as libc::c_int {
                        pb = 1 as libc::c_int;
                    } else if pc == 0 as libc::c_int {
                        pc = 1 as libc::c_int;
                    }
                    fi = pa as libc::c_float / pc as libc::c_float
                        - pd as libc::c_float / pb as libc::c_float;
                    if fi > 180.0f32 {
                        fi = (pa + pd + pb * 360 as libc::c_int) as libc::c_float
                            / tot as libc::c_float;
                        if fi > 180.0f32 {
                            fi -= 360.0f32;
                        }
                    } else {
                        fi = (pa + pd) as libc::c_float / tot as libc::c_float;
                    }
                    pa = if fi < 0.0f32 {
                        (fi - 0.5f32) as libc::c_int
                    } else {
                        (fi + 0.5f32) as libc::c_int
                    };
                    if pa <= -(180 as libc::c_int) {
                        pa += 360 as libc::c_int;
                    }
                    avv[tp as usize][0 as libc::c_int as usize] = pa;
                    ii = 1 as libc::c_int;
                    while ii < 5 as libc::c_int {
                        avv[tp as usize][ii as usize] = avn[ii as usize] / tot;
                        avn[ii as usize] = 0 as libc::c_int;
                        ii += 1;
                    }
                    ct[tp as usize] = tot;
                    gct[tp as usize] = tot;
                    if tot > match_score {
                        match_score = tot;
                    }
                    ctt[tp as usize] = 0 as libc::c_int;
                    ctp[tp as usize][0 as libc::c_int as usize] = tp;
                    ii = 0 as libc::c_int;
                    while ii < tp {
                        let mut found: libc::c_int = 0;
                        let mut diff_0: libc::c_int = 0;
                        let mut avv_tp_ptr: *mut libc::c_int = &mut *(*avv
                            .as_mut_ptr()
                            .offset(tp as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut libc::c_int;
                        let mut avv_ii_ptr: *mut libc::c_int = &mut *(*avv
                            .as_mut_ptr()
                            .offset(ii as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut libc::c_int;
                        let fresh46 = avv_tp_ptr;
                        avv_tp_ptr = avv_tp_ptr.offset(1);
                        let fresh47 = avv_ii_ptr;
                        avv_ii_ptr = avv_ii_ptr.offset(1);
                        diff_0 = *fresh46 - *fresh47;
                        j = diff_0 * diff_0;
                        if !(j > 121 as libc::c_int && j < 121801 as libc::c_int) {
                            let fresh48 = avv_tp_ptr;
                            avv_tp_ptr = avv_tp_ptr.offset(1);
                            let fresh49 = avv_ii_ptr;
                            avv_ii_ptr = avv_ii_ptr.offset(1);
                            ll = *fresh48 - *fresh49;
                            let fresh50 = avv_tp_ptr;
                            avv_tp_ptr = avv_tp_ptr.offset(1);
                            let fresh51 = avv_ii_ptr;
                            avv_ii_ptr = avv_ii_ptr.offset(1);
                            jj = *fresh50 - *fresh51;
                            let fresh52 = avv_tp_ptr;
                            avv_tp_ptr = avv_tp_ptr.offset(1);
                            let fresh53 = avv_ii_ptr;
                            avv_ii_ptr = avv_ii_ptr.offset(1);
                            kk = *fresh52 - *fresh53;
                            let fresh54 = avv_tp_ptr;
                            avv_tp_ptr = avv_tp_ptr.offset(1);
                            let fresh55 = avv_ii_ptr;
                            avv_ii_ptr = avv_ii_ptr.offset(1);
                            j = *fresh54 - *fresh55;
                            let mut tt: libc::c_float = 0.;
                            let mut ai: libc::c_float = 0.;
                            let mut dz: libc::c_float = 0.;
                            tt = (ll * ll + jj * jj) as libc::c_float;
                            ai = (j * j + kk * kk) as libc::c_float;
                            fi = 2.0f32 * 0.05f32 * (tt + ai);
                            dz = tt - ai;
                            if !(dz * dz > fi * fi) {
                                if ll != 0 {
                                    fi = 180.0f32 / 3.14159265358979323846f64 as libc::c_float
                                        * atanf(jj as libc::c_float / ll as libc::c_float);
                                    if fi < 0.0f32 {
                                        if ll < 0 as libc::c_int {
                                            fi += 180.5f32;
                                        } else {
                                            fi -= 0.5f32;
                                        }
                                    } else if ll < 0 as libc::c_int {
                                        fi -= 180.5f32;
                                    } else {
                                        fi += 0.5f32;
                                    }
                                    jj = fi as libc::c_int;
                                    if jj <= -(180 as libc::c_int) {
                                        jj += 360 as libc::c_int;
                                    }
                                } else if jj > 0 as libc::c_int {
                                    jj = 90 as libc::c_int;
                                } else {
                                    jj = -(90 as libc::c_int);
                                }
                                if kk != 0 {
                                    fi = 180.0f32 / 3.14159265358979323846f64 as libc::c_float
                                        * atanf(j as libc::c_float / kk as libc::c_float);
                                    if fi < 0.0f32 {
                                        if kk < 0 as libc::c_int {
                                            fi += 180.5f32;
                                        } else {
                                            fi -= 0.5f32;
                                        }
                                    } else if kk < 0 as libc::c_int {
                                        fi -= 180.5f32;
                                    } else {
                                        fi += 0.5f32;
                                    }
                                    j = fi as libc::c_int;
                                    if j <= -(180 as libc::c_int) {
                                        j += 360 as libc::c_int;
                                    }
                                } else if j > 0 as libc::c_int {
                                    j = 90 as libc::c_int;
                                } else {
                                    j = -(90 as libc::c_int);
                                }
                                pa = 0 as libc::c_int;
                                pb = 0 as libc::c_int;
                                pc = 0 as libc::c_int;
                                pd = 0 as libc::c_int;
                                if avv[tp as usize][0 as libc::c_int as usize]
                                    < 0 as libc::c_int
                                {
                                    pd += avv[tp as usize][0 as libc::c_int as usize];
                                    pb += 1;
                                } else {
                                    pa += avv[tp as usize][0 as libc::c_int as usize];
                                    pc += 1;
                                }
                                if avv[ii as usize][0 as libc::c_int as usize]
                                    < 0 as libc::c_int
                                {
                                    pd += avv[ii as usize][0 as libc::c_int as usize];
                                    pb += 1;
                                } else {
                                    pa += avv[ii as usize][0 as libc::c_int as usize];
                                    pc += 1;
                                }
                                if pb == 0 as libc::c_int {
                                    pb = 1 as libc::c_int;
                                } else if pc == 0 as libc::c_int {
                                    pc = 1 as libc::c_int;
                                }
                                fi = pa as libc::c_float / pc as libc::c_float
                                    - pd as libc::c_float / pb as libc::c_float;
                                if fi > 180.0f32 {
                                    fi = (pa + pd + pb * 360 as libc::c_int) as libc::c_float
                                        / 2.0f32;
                                    if fi > 180.0f32 {
                                        fi -= 360.0f32;
                                    }
                                } else {
                                    fi = (pa + pd) as libc::c_float / 2.0f32;
                                }
                                pb = if fi < 0.0f32 {
                                    (fi - 0.5f32) as libc::c_int
                                } else {
                                    (fi + 0.5f32) as libc::c_int
                                };
                                if pb <= -(180 as libc::c_int) {
                                    pb += 360 as libc::c_int;
                                }
                                pa = jj - j;
                                pa = if pa > 180 as libc::c_int {
                                    pa - 360 as libc::c_int
                                } else if pa <= -(180 as libc::c_int) {
                                    pa + 360 as libc::c_int
                                } else {
                                    pa
                                };
                                kk = (pb - pa) * (pb - pa);
                                if !(kk > 121 as libc::c_int && kk < 121801 as libc::c_int)
                                {
                                    found = 0 as libc::c_int;
                                    kk = 0 as libc::c_int;
                                    while kk < 2 as libc::c_int {
                                        jj = 0 as libc::c_int;
                                        ll = 0 as libc::c_int;
                                        loop {
                                            while yy[jj as usize][kk as usize][ii as usize]
                                                < yy[ll as usize][kk as usize][tp as usize]
                                                && jj < yl[kk as usize][ii as usize]
                                            {
                                                jj += 1;
                                            }
                                            while yy[jj as usize][kk as usize][ii as usize]
                                                > yy[ll as usize][kk as usize][tp as usize]
                                                && ll < yl[kk as usize][tp as usize]
                                            {
                                                ll += 1;
                                            }
                                            if yy[jj as usize][kk as usize][ii as usize]
                                                == yy[ll as usize][kk as usize][tp as usize]
                                                && jj < yl[kk as usize][ii as usize]
                                                && ll < yl[kk as usize][tp as usize]
                                            {
                                                found = 1 as libc::c_int;
                                                break;
                                            } else if !(jj < yl[kk as usize][ii as usize]
                                                && ll < yl[kk as usize][tp as usize])
                                            {
                                                break;
                                            }
                                        }
                                        if found != 0 {
                                            break;
                                        }
                                        kk += 1;
                                    }
                                    if found == 0 {
                                        gct[ii as usize] += ct[tp as usize];
                                        if gct[ii as usize] > match_score {
                                            match_score = gct[ii as usize];
                                        }
                                        ctt[ii as usize] += 1;
                                        ctp[ii as usize][ctt[ii as usize] as usize] = tp;
                                    }
                                }
                            }
                        }
                        ii += 1;
                    }
                    tp += 1;
                }
                if qh > 4000 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%s: WARNING: bz_match_score(): qq[] overflow #3 in bozorth3(); qh-1 is %d [p=%s; g=%s]\n\0"
                            as *const u8 as *const libc::c_char,
                        get_progname(),
                        qh - 1 as libc::c_int,
                        get_probe_filename(),
                        get_gallery_filename(),
                    );
                    return 4000 as libc::c_int;
                }
                i = qh - 1 as libc::c_int;
                while i > 0 as libc::c_int {
                    n = qq[i as usize] - 1 as libc::c_int;
                    if tq[n as usize] - 1 as libc::c_int >= 0 as libc::c_int {
                        rq[(tq[n as usize] - 1 as libc::c_int)
                            as usize] = 0 as libc::c_int;
                        tq[n as usize] = 0 as libc::c_int;
                        zz[n as usize] = 1000 as libc::c_int;
                    }
                    i -= 1;
                }
                i = dw - 1 as libc::c_int;
                while i >= 0 as libc::c_int {
                    n = rr[i as usize] - 1 as libc::c_int;
                    if tq[n as usize] != 0 {
                        rq[(tq[n as usize] - 1 as libc::c_int)
                            as usize] = 0 as libc::c_int;
                        tq[n as usize] = 0 as libc::c_int;
                    }
                    i -= 1;
                }
                i = 0 as libc::c_int;
                j = ww - 1 as libc::c_int;
                while i >= 0 as libc::c_int && j >= 0 as libc::c_int {
                    if nn[j as usize] < mm[j as usize] {
                        nn[j as usize] += 1;
                        i = ww - 1 as libc::c_int;
                        while i >= 0 as libc::c_int {
                            let mut rt: libc::c_int = rx[i as usize];
                            if rt < 0 as libc::c_int {
                                rt = -rt;
                                rt -= 1;
                                z = rf[i
                                    as usize][(nn[i as usize] - 1 as libc::c_int) as usize]
                                    - 1 as libc::c_int;
                                if tq[z as usize] != rt + 1 as libc::c_int
                                    && tq[z as usize] != 0
                                    || rq[rt as usize] != z + 1 as libc::c_int
                                        && rq[rt as usize] != 0
                                {
                                    break;
                                }
                                tq[z as usize] = rt + 1 as libc::c_int;
                                rq[rt as usize] = z + 1 as libc::c_int;
                                rr[i as usize] = z + 1 as libc::c_int;
                            } else {
                                rt -= 1;
                                z = cf[i
                                    as usize][(nn[i as usize] - 1 as libc::c_int) as usize]
                                    - 1 as libc::c_int;
                                if tq[rt as usize] != z + 1 as libc::c_int
                                    && tq[rt as usize] != 0
                                    || rq[z as usize] != rt + 1 as libc::c_int
                                        && rq[z as usize] != 0
                                {
                                    break;
                                }
                                tq[rt as usize] = z + 1 as libc::c_int;
                                rq[z as usize] = rt + 1 as libc::c_int;
                                rr[i as usize] = rt + 1 as libc::c_int;
                            }
                            i -= 1;
                        }
                        if i >= 0 as libc::c_int {
                            z = i + 1 as libc::c_int;
                            while z < ww {
                                n = rr[z as usize] - 1 as libc::c_int;
                                if tq[n as usize] - 1 as libc::c_int >= 0 as libc::c_int {
                                    rq[(tq[n as usize] - 1 as libc::c_int)
                                        as usize] = 0 as libc::c_int;
                                    tq[n as usize] = 0 as libc::c_int;
                                }
                                z += 1;
                            }
                            j = ww - 1 as libc::c_int;
                        }
                    } else {
                        nn[j as usize] = 1 as libc::c_int;
                        j -= 1;
                    }
                }
                if tp > 1999 as libc::c_int {
                    break;
                }
                dw = ww;
                if !(j >= 0 as libc::c_int) {
                    break;
                }
            }
            if tp > 1999 as libc::c_int {
                break;
            }
            n = qq[0 as libc::c_int as usize] - 1 as libc::c_int;
            if tq[n as usize] - 1 as libc::c_int >= 0 as libc::c_int {
                rq[(tq[n as usize] - 1 as libc::c_int) as usize] = 0 as libc::c_int;
                tq[n as usize] = 0 as libc::c_int;
            }
            i = ww - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                n = rx[i as usize];
                if n < 0 as libc::c_int {
                    n = -n;
                    rp[(n - 1 as libc::c_int) as usize] = 0 as libc::c_int;
                } else {
                    cp[(n - 1 as libc::c_int) as usize] = 0 as libc::c_int;
                }
                i -= 1;
            }
        }
        k += 1;
    }
    if match_score < 8 as libc::c_int {
        return match_score;
    }
    match_score = bz_final_loop(tp);
    return match_score;
}
#[no_mangle]
pub unsafe extern "C" fn bz_sift(
    mut ww: *mut libc::c_int,
    mut kz: libc::c_int,
    mut qh: *mut libc::c_int,
    mut l: libc::c_int,
    mut kx: libc::c_int,
    mut ftt: libc::c_int,
    mut tot: *mut libc::c_int,
    mut qq_overflow: *mut libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    n = tq[(kz - 1 as libc::c_int) as usize];
    t = rq[(l - 1 as libc::c_int) as usize];
    if n == 0 as libc::c_int && t == 0 as libc::c_int {
        if sc[(kx - 1 as libc::c_int) as usize] != ftt {
            let fresh56 = *tot;
            *tot = *tot + 1;
            bz_y[fresh56 as usize] = kx;
            rk[(kx - 1 as libc::c_int) as usize] = sc[(kx - 1 as libc::c_int) as usize];
            sc[(kx - 1 as libc::c_int) as usize] = ftt;
        }
        if *qh >= 4000 as libc::c_int {
            fprintf(
                stderr,
                b"%s: ERROR: bz_sift(): qq[] overflow #1; the index [*qh] is %d [p=%s; g=%s]\n\0"
                    as *const u8 as *const libc::c_char,
                get_progname(),
                *qh,
                get_probe_filename(),
                get_gallery_filename(),
            );
            *qq_overflow = 1 as libc::c_int;
            return;
        }
        qq[*qh as usize] = kz;
        let fresh57 = *qh;
        *qh = *qh + 1;
        zz[(kz - 1 as libc::c_int) as usize] = fresh57;
        tq[(kz - 1 as libc::c_int) as usize] = l;
        rq[(l - 1 as libc::c_int) as usize] = kz;
        return;
    }
    if n == l {
        if sc[(kx - 1 as libc::c_int) as usize] != ftt {
            if zz[(kx - 1 as libc::c_int) as usize] == 1000 as libc::c_int {
                if *qh >= 4000 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%s: ERROR: bz_sift(): qq[] overflow #2; the index [*qh] is %d [p=%s; g=%s]\n\0"
                            as *const u8 as *const libc::c_char,
                        get_progname(),
                        *qh,
                        get_probe_filename(),
                        get_gallery_filename(),
                    );
                    *qq_overflow = 1 as libc::c_int;
                    return;
                }
                qq[*qh as usize] = kz;
                let fresh58 = *qh;
                *qh = *qh + 1;
                zz[(kz - 1 as libc::c_int) as usize] = fresh58;
            }
            let fresh59 = *tot;
            *tot = *tot + 1;
            bz_y[fresh59 as usize] = kx;
            rk[(kx - 1 as libc::c_int) as usize] = sc[(kx - 1 as libc::c_int) as usize];
            sc[(kx - 1 as libc::c_int) as usize] = ftt;
        }
        return;
    }
    if *ww >= 10 as libc::c_int {
        return;
    }
    let mut b: libc::c_int = 0;
    let mut b_index: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut notfound: libc::c_int = 0;
    let mut lim: libc::c_int = 0;
    let mut lptr: *mut libc::c_int = 0 as *mut libc::c_int;
    if n != 0 {
        b = cp[(kz - 1 as libc::c_int) as usize];
        if b == 0 as libc::c_int {
            *ww += 1;
            b = *ww;
            b_index = b - 1 as libc::c_int;
            cp[(kz - 1 as libc::c_int) as usize] = b;
            cf[b_index as usize][0 as libc::c_int as usize] = n;
            mm[b_index as usize] = 1 as libc::c_int;
            nn[b_index as usize] = 1 as libc::c_int;
            rx[b_index as usize] = kz;
        } else {
            b_index = b - 1 as libc::c_int;
        }
        lim = mm[b_index as usize];
        lptr = &mut *(*cf.as_mut_ptr().offset(b_index as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut libc::c_int;
        notfound = 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < lim {
            let fresh61 = lptr;
            lptr = lptr.offset(1);
            if *fresh61 == l {
                notfound = 0 as libc::c_int;
                break;
            } else {
                i += 1;
            }
        }
        if notfound != 0 {
            cf[b_index as usize][i as usize] = l;
            mm[b_index as usize] += 1;
        }
    }
    if t != 0 {
        b = rp[(l - 1 as libc::c_int) as usize];
        if b == 0 as libc::c_int {
            *ww += 1;
            b = *ww;
            b_index = b - 1 as libc::c_int;
            rp[(l - 1 as libc::c_int) as usize] = b;
            rf[b_index as usize][0 as libc::c_int as usize] = t;
            mm[b_index as usize] = 1 as libc::c_int;
            nn[b_index as usize] = 1 as libc::c_int;
            rx[b_index as usize] = -l;
        } else {
            b_index = b - 1 as libc::c_int;
        }
        lim = mm[b_index as usize];
        lptr = &mut *(*rf.as_mut_ptr().offset(b_index as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut libc::c_int;
        notfound = 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < lim {
            let fresh63 = lptr;
            lptr = lptr.offset(1);
            if *fresh63 == kz {
                notfound = 0 as libc::c_int;
                break;
            } else {
                i += 1;
            }
        }
        if notfound != 0 {
            rf[b_index as usize][i as usize] = kz;
            mm[b_index as usize] += 1;
        }
    }
}
unsafe extern "C" fn bz_final_loop(mut tp: libc::c_int) -> libc::c_int {
    let mut ii: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut lim: libc::c_int = 0;
    let mut match_score: libc::c_int = 0;
    static mut sct: [[libc::c_int; 1000]; 2500] = [[0; 1000]; 2500];
    match_score = 0 as libc::c_int;
    ii = 0 as libc::c_int;
    while ii < tp {
        if !(match_score >= gct[ii as usize]) {
            lim = ctt[ii as usize] + 1 as libc::c_int;
            i = 0 as libc::c_int;
            while i < lim {
                sct[i
                    as usize][0 as libc::c_int as usize] = ctp[ii as usize][i as usize];
                i += 1;
            }
            t = 0 as libc::c_int;
            bz_y[0 as libc::c_int as usize] = lim;
            cp[0 as libc::c_int as usize] = 1 as libc::c_int;
            b = 0 as libc::c_int;
            n = 1 as libc::c_int;
            loop {
                if bz_y[t as usize] - cp[t as usize] > 1 as libc::c_int {
                    k = sct[cp[t as usize] as usize][t as usize];
                    j = ctt[k as usize] + 1 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < j {
                        rp[i as usize] = ctp[k as usize][i as usize];
                        i += 1;
                    }
                    k = 0 as libc::c_int;
                    kk = cp[t as usize];
                    jj = 0 as libc::c_int;
                    loop {
                        while rp[jj as usize] < sct[kk as usize][t as usize] && jj < j {
                            jj += 1;
                        }
                        while rp[jj as usize] > sct[kk as usize][t as usize]
                            && kk < bz_y[t as usize]
                        {
                            kk += 1;
                        }
                        while rp[jj as usize] == sct[kk as usize][t as usize]
                            && kk < bz_y[t as usize] && jj < j
                        {
                            sct[k
                                as usize][(t + 1 as libc::c_int)
                                as usize] = sct[kk as usize][t as usize];
                            k += 1;
                            kk += 1;
                            jj += 1;
                        }
                        if !(kk < bz_y[t as usize] && jj < j) {
                            break;
                        }
                    }
                    t += 1;
                    cp[t as usize] = 1 as libc::c_int;
                    bz_y[t as usize] = k;
                    b = t;
                    n = 1 as libc::c_int;
                } else {
                    let mut tot: libc::c_int = 0 as libc::c_int;
                    lim = bz_y[t as usize];
                    i = n - 1 as libc::c_int;
                    while i < lim {
                        tot += ct[sct[i as usize][t as usize] as usize];
                        i += 1;
                    }
                    i = 0 as libc::c_int;
                    while i < b {
                        tot += ct[sct[0 as libc::c_int as usize][i as usize] as usize];
                        i += 1;
                    }
                    if tot > match_score {
                        match_score = tot;
                        i = 0 as libc::c_int;
                        while i < b {
                            rk[i as usize] = sct[0 as libc::c_int as usize][i as usize];
                            i += 1;
                        }
                        let mut rk_index: libc::c_int = b;
                        lim = bz_y[t as usize];
                        i = n - 1 as libc::c_int;
                        while i < lim {
                            let fresh64 = i;
                            i = i + 1;
                            let fresh65 = rk_index;
                            rk_index = rk_index + 1;
                            rk[fresh65 as usize] = sct[fresh64 as usize][t as usize];
                        }
                    }
                    b = t;
                    t -= 1;
                    if t >= 0 as libc::c_int {
                        cp[t as usize] += 1;
                        n = bz_y[t as usize];
                    }
                }
                if !(t >= 0 as libc::c_int) {
                    break;
                }
            }
        }
        ii += 1;
    }
    return match_score;
}
