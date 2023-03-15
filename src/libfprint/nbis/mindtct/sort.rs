use ::libc;
extern "C" {
    fn g_malloc(n_bytes: gsize) -> gpointer;
}
pub type gsize = libc::c_ulong;
pub type gpointer = *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn sort_indices_int_inc(
    mut optr: *mut *mut libc::c_int,
    mut ranks: *mut libc::c_int,
    num: libc::c_int,
) -> libc::c_int {
    let mut order: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    order = g_malloc(
        (num as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < num {
        *order.offset(i as isize) = i;
        i += 1;
    }
    bubble_sort_int_inc_2(ranks, order, num);
    *optr = order;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bubble_sort_int_inc_2(
    mut ranks: *mut libc::c_int,
    mut items: *mut libc::c_int,
    len: libc::c_int,
) {
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut trank: libc::c_int = 0;
    let mut titem: libc::c_int = 0;
    n = len;
    while done == 0 {
        done = (0 as libc::c_int == 0) as libc::c_int;
        i = 1 as libc::c_int;
        p = 0 as libc::c_int;
        while i < n {
            if *ranks.offset(p as isize) > *ranks.offset(i as isize) {
                trank = *ranks.offset(i as isize);
                *ranks.offset(i as isize) = *ranks.offset(p as isize);
                *ranks.offset(p as isize) = trank;
                titem = *items.offset(i as isize);
                *items.offset(i as isize) = *items.offset(p as isize);
                *items.offset(p as isize) = titem;
                done = 0 as libc::c_int;
            }
            i += 1;
            p += 1;
        }
        n -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bubble_sort_double_inc_2(
    mut ranks: *mut libc::c_double,
    mut items: *mut libc::c_int,
    len: libc::c_int,
) {
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut titem: libc::c_int = 0;
    let mut trank: libc::c_double = 0.;
    n = len;
    while done == 0 {
        done = (0 as libc::c_int == 0) as libc::c_int;
        i = 1 as libc::c_int;
        p = 0 as libc::c_int;
        while i < n {
            if *ranks.offset(p as isize) > *ranks.offset(i as isize) {
                trank = *ranks.offset(i as isize);
                *ranks.offset(i as isize) = *ranks.offset(p as isize);
                *ranks.offset(p as isize) = trank;
                titem = *items.offset(i as isize);
                *items.offset(i as isize) = *items.offset(p as isize);
                *items.offset(p as isize) = titem;
                done = 0 as libc::c_int;
            }
            i += 1;
            p += 1;
        }
        n -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bubble_sort_double_dec_2(
    mut ranks: *mut libc::c_double,
    mut items: *mut libc::c_int,
    len: libc::c_int,
) {
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut titem: libc::c_int = 0;
    let mut trank: libc::c_double = 0.;
    n = len;
    while done == 0 {
        done = 1 as libc::c_int;
        i = 1 as libc::c_int;
        p = 0 as libc::c_int;
        while i < n {
            if *ranks.offset(p as isize) < *ranks.offset(i as isize) {
                trank = *ranks.offset(i as isize);
                *ranks.offset(i as isize) = *ranks.offset(p as isize);
                *ranks.offset(p as isize) = trank;
                titem = *items.offset(i as isize);
                *items.offset(i as isize) = *items.offset(p as isize);
                *items.offset(p as isize) = titem;
                done = 0 as libc::c_int;
            }
            i += 1;
            p += 1;
        }
        n -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bubble_sort_int_inc(
    mut ranks: *mut libc::c_int,
    len: libc::c_int,
) {
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut trank: libc::c_int = 0;
    n = len;
    while done == 0 {
        done = (0 as libc::c_int == 0) as libc::c_int;
        i = 1 as libc::c_int;
        p = 0 as libc::c_int;
        while i < n {
            if *ranks.offset(p as isize) > *ranks.offset(i as isize) {
                trank = *ranks.offset(i as isize);
                *ranks.offset(i as isize) = *ranks.offset(p as isize);
                *ranks.offset(p as isize) = trank;
                done = 0 as libc::c_int;
            }
            i += 1;
            p += 1;
        }
        n -= 1;
    }
}
