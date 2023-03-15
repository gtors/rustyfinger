use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct minutiae_struct {
    pub col: [libc::c_int; 4],
}
#[no_mangle]
pub unsafe extern "C" fn sort_x_y(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut af: *mut minutiae_struct = 0 as *mut minutiae_struct;
    let mut bf: *mut minutiae_struct = 0 as *mut minutiae_struct;
    af = a as *mut minutiae_struct;
    bf = b as *mut minutiae_struct;
    if (*af).col[0 as libc::c_int as usize] < (*bf).col[0 as libc::c_int as usize] {
        return -(1 as libc::c_int);
    }
    if (*af).col[0 as libc::c_int as usize] > (*bf).col[0 as libc::c_int as usize] {
        return 1 as libc::c_int;
    }
    if (*af).col[1 as libc::c_int as usize] < (*bf).col[1 as libc::c_int as usize] {
        return -(1 as libc::c_int);
    }
    if (*af).col[1 as libc::c_int as usize] > (*bf).col[1 as libc::c_int as usize] {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
