use ::libc;
extern "C" {
    fn g_free(mem: gpointer);
}
pub type gpointer = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dir2rad {
    pub ndirs: libc::c_int,
    pub cos: *mut libc::c_double,
    pub sin: *mut libc::c_double,
}
pub type DIR2RAD = dir2rad;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dftwave {
    pub cos: *mut libc::c_double,
    pub sin: *mut libc::c_double,
}
pub type DFTWAVE = dftwave;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dftwaves {
    pub nwaves: libc::c_int,
    pub wavelen: libc::c_int,
    pub waves: *mut *mut DFTWAVE,
}
pub type DFTWAVES = dftwaves;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rotgrids {
    pub pad: libc::c_int,
    pub relative2: libc::c_int,
    pub start_angle: libc::c_double,
    pub ngrids: libc::c_int,
    pub grid_w: libc::c_int,
    pub grid_h: libc::c_int,
    pub grids: *mut *mut libc::c_int,
}
pub type ROTGRIDS = rotgrids;
#[no_mangle]
pub unsafe extern "C" fn free_dir2rad(mut dir2rad: *mut DIR2RAD) {
    g_free((*dir2rad).cos as gpointer);
    g_free((*dir2rad).sin as gpointer);
    g_free(dir2rad as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn free_dftwaves(mut dftwaves: *mut DFTWAVES) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*dftwaves).nwaves {
        g_free((**((*dftwaves).waves).offset(i as isize)).cos as gpointer);
        g_free((**((*dftwaves).waves).offset(i as isize)).sin as gpointer);
        g_free(*((*dftwaves).waves).offset(i as isize) as gpointer);
        i += 1;
    }
    g_free((*dftwaves).waves as gpointer);
    g_free(dftwaves as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn free_rotgrids(mut rotgrids: *mut ROTGRIDS) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*rotgrids).ngrids {
        g_free(*((*rotgrids).grids).offset(i as isize) as gpointer);
        i += 1;
    }
    g_free((*rotgrids).grids as gpointer);
    g_free(rotgrids as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn free_dir_powers(
    mut powers: *mut *mut libc::c_double,
    nwaves: libc::c_int,
) {
    let mut w: libc::c_int = 0;
    w = 0 as libc::c_int;
    while w < nwaves {
        g_free(*powers.offset(w as isize) as gpointer);
        w += 1;
    }
    g_free(powers as gpointer);
}
