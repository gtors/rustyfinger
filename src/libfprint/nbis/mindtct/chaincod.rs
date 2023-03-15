use ::libc;
extern "C" {
    fn g_malloc(n_bytes: gsize) -> gpointer;
    static mut g_chaincodes_nbr8: [libc::c_int; 0];
}
pub type gsize = libc::c_ulong;
pub type gpointer = *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn chain_code_loop(
    mut ochain: *mut *mut libc::c_int,
    mut onchain: *mut libc::c_int,
    mut contour_x: *const libc::c_int,
    mut contour_y: *const libc::c_int,
    ncontour: libc::c_int,
) -> libc::c_int {
    let mut chain: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    if ncontour <= 3 as libc::c_int {
        *onchain = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    chain = g_malloc(
        (ncontour as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while i < ncontour - 1 as libc::c_int {
        dx = *contour_x.offset(j as isize) - *contour_x.offset(i as isize);
        dy = *contour_y.offset(j as isize) - *contour_y.offset(i as isize);
        *chain
            .offset(
                i as isize,
            ) = *g_chaincodes_nbr8
            .as_mut_ptr()
            .offset(((dy + 1 as libc::c_int) * 3 as libc::c_int) as isize)
            .offset(dx as isize)
            .offset(1 as libc::c_int as isize);
        i += 1;
        j += 1;
    }
    dx = *contour_x.offset(0 as libc::c_int as isize) - *contour_x.offset(i as isize);
    dy = *contour_y.offset(0 as libc::c_int as isize) - *contour_y.offset(i as isize);
    *chain
        .offset(
            i as isize,
        ) = *g_chaincodes_nbr8
        .as_mut_ptr()
        .offset(((dy + 1 as libc::c_int) * 3 as libc::c_int) as isize)
        .offset(dx as isize)
        .offset(1 as libc::c_int as isize);
    *ochain = chain;
    *onchain = ncontour;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_chain_clockwise(
    mut chain: *const libc::c_int,
    nchain: libc::c_int,
    default_ret: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    sum = 0 as libc::c_int;
    i = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while i < nchain - 1 as libc::c_int {
        d = *chain.offset(j as isize) - *chain.offset(i as isize);
        if d >= 4 as libc::c_int {
            d -= 8 as libc::c_int;
        } else if d <= -(4 as libc::c_int) {
            d += 8 as libc::c_int;
        }
        sum += d;
        i += 1;
        j += 1;
    }
    d = *chain.offset(0 as libc::c_int as isize) - *chain.offset(i as isize);
    if d >= 4 as libc::c_int {
        d -= 8 as libc::c_int;
    } else if d <= -(4 as libc::c_int) {
        d += 8 as libc::c_int;
    }
    sum += d;
    if sum == 0 as libc::c_int {
        return default_ret
    } else if sum > 0 as libc::c_int {
        return 0 as libc::c_int
    } else {
        return (0 as libc::c_int == 0) as libc::c_int
    };
}
