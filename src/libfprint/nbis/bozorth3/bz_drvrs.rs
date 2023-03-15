use ::libc;
extern "C" {
    fn bz_match(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn bz_match_score(
        _: libc::c_int,
        _: *mut xyt_struct,
        _: *mut xyt_struct,
    ) -> libc::c_int;
    fn bz_comp(
        _: libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut [libc::c_int; 6],
        _: *mut *mut libc::c_int,
    );
    static mut scols: [[libc::c_int; 6]; 20000];
    static mut fcols: [[libc::c_int; 6]; 20000];
    static mut scolpt: [*mut libc::c_int; 20000];
    static mut fcolpt: [*mut libc::c_int; 20000];
    fn bz_find(_: *mut libc::c_int, _: *mut *mut libc::c_int);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xyt_struct {
    pub nrows: libc::c_int,
    pub xcol: [libc::c_int; 200],
    pub ycol: [libc::c_int; 200],
    pub thetacol: [libc::c_int; 200],
}
#[no_mangle]
pub unsafe extern "C" fn bozorth_probe_init(
    mut pstruct: *mut xyt_struct,
) -> libc::c_int {
    let mut sim: libc::c_int = 0;
    let mut msim: libc::c_int = 0;
    bz_comp(
        (*pstruct).nrows,
        ((*pstruct).xcol).as_mut_ptr(),
        ((*pstruct).ycol).as_mut_ptr(),
        ((*pstruct).thetacol).as_mut_ptr(),
        &mut sim,
        scols.as_mut_ptr(),
        scolpt.as_mut_ptr(),
    );
    msim = sim;
    bz_find(&mut msim, scolpt.as_mut_ptr());
    if msim < 500 as libc::c_int {
        msim = if sim > 500 as libc::c_int { 500 as libc::c_int } else { sim };
    }
    return msim;
}
#[no_mangle]
pub unsafe extern "C" fn bozorth_gallery_init(
    mut gstruct: *mut xyt_struct,
) -> libc::c_int {
    let mut fim: libc::c_int = 0;
    let mut mfim: libc::c_int = 0;
    bz_comp(
        (*gstruct).nrows,
        ((*gstruct).xcol).as_mut_ptr(),
        ((*gstruct).ycol).as_mut_ptr(),
        ((*gstruct).thetacol).as_mut_ptr(),
        &mut fim,
        fcols.as_mut_ptr(),
        fcolpt.as_mut_ptr(),
    );
    mfim = fim;
    bz_find(&mut mfim, fcolpt.as_mut_ptr());
    if mfim < 500 as libc::c_int {
        mfim = if fim > 500 as libc::c_int { 500 as libc::c_int } else { fim };
    }
    return mfim;
}
#[no_mangle]
pub unsafe extern "C" fn bozorth_to_gallery(
    mut probe_len: libc::c_int,
    mut pstruct: *mut xyt_struct,
    mut gstruct: *mut xyt_struct,
) -> libc::c_int {
    let mut np: libc::c_int = 0;
    let mut gallery_len: libc::c_int = 0;
    gallery_len = bozorth_gallery_init(gstruct);
    np = bz_match(probe_len, gallery_len);
    return bz_match_score(np, pstruct, gstruct);
}
