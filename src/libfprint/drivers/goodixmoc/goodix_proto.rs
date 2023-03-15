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
    fn g_log(
        log_domain: *const gchar,
        log_level: GLogLevelFlags,
        format: *const gchar,
        _: ...
    );
    fn g_assertion_message_expr(
        domain: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        func: *const libc::c_char,
        expr: *const libc::c_char,
    ) -> !;
}
pub type guint16 = libc::c_ushort;
pub type guint32 = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type gchar = libc::c_char;
pub type glong = libc::c_long;
pub type GLogLevelFlags = libc::c_int;
pub const G_LOG_LEVEL_MASK: GLogLevelFlags = -4;
pub const G_LOG_LEVEL_DEBUG: GLogLevelFlags = 128;
pub const G_LOG_LEVEL_INFO: GLogLevelFlags = 64;
pub const G_LOG_LEVEL_MESSAGE: GLogLevelFlags = 32;
pub const G_LOG_LEVEL_WARNING: GLogLevelFlags = 16;
pub const G_LOG_LEVEL_CRITICAL: GLogLevelFlags = 8;
pub const G_LOG_LEVEL_ERROR: GLogLevelFlags = 4;
pub const G_LOG_FLAG_FATAL: GLogLevelFlags = 2;
pub const G_LOG_FLAG_RECURSION: GLogLevelFlags = 1;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_version_info {
    pub format: [uint8_t; 2],
    pub fwtype: [uint8_t; 8],
    pub fwversion: [uint8_t; 8],
    pub customer: [uint8_t; 8],
    pub mcu: [uint8_t; 8],
    pub sensor: [uint8_t; 8],
    pub algversion: [uint8_t; 8],
    pub interface: [uint8_t; 8],
    pub protocol: [uint8_t; 8],
    pub flashVersion: [uint8_t; 8],
    pub reserved: [uint8_t; 38],
}
pub type gxfp_version_info_t = _gxfp_version_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_parse_msg {
    pub ack_cmd: uint8_t,
    pub has_no_config: bool,
}
pub type gxfp_parse_msg_t = _gxfp_parse_msg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_enroll_init {
    pub tid: [uint8_t; 32],
}
pub type gxfp_enroll_init_t = _gxfp_enroll_init;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct _template_format {
    pub _0x43_byte: uint8_t,
    pub type_0: uint8_t,
    pub finger_index: uint8_t,
    pub pad0: uint8_t,
    pub accountid: [uint8_t; 32],
    pub tid: [uint8_t; 32],
    pub payload: C2RustUnnamed,
    pub reserve: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct C2RustUnnamed {
    pub size: uint8_t,
    pub data: [uint8_t; 56],
}
pub type template_format_t = _template_format;
pub type ptemplate_format_t = *mut _template_format;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_verify {
    pub match_0: bool,
    pub rejectdetail: uint32_t,
    pub template: template_format_t,
}
pub type gxfp_verify_t = _gxfp_verify;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_capturedata {
    pub img_quality: uint8_t,
    pub img_coverage: uint8_t,
}
pub type gxfp_capturedata_t = _gxfp_capturedata;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_check_duplicate {
    pub duplicate: bool,
    pub template: template_format_t,
}
pub type gxfp_check_duplicate_t = _gxfp_check_duplicate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_enroll_update {
    pub rollback: bool,
    pub img_overlay: uint8_t,
    pub img_preoverlay: uint8_t,
}
pub type gxfp_enroll_update_t = _gxfp_enroll_update;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_enum_fingerlist {
    pub finger_num: uint8_t,
    pub finger_list: [template_format_t; 20],
}
pub type gxfp_enum_fingerlist_t = _gxfp_enum_fingerlist;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_enroll_commit {
    pub result: uint8_t,
}
pub type gxfp_enroll_commit_t = _gxfp_enroll_commit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fp_finger_status {
    pub status: uint8_t,
}
pub type fp_finger_status_t = _fp_finger_status;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fp_finger_config {
    pub status: uint8_t,
    pub max_stored_prints: uint8_t,
}
pub type fp_finger_config_t = _fp_finger_config;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fp_pwr_btn_shield {
    pub resp_cmd1: uint8_t,
}
pub type fp_pwr_btn_shield_t = _fp_pwr_btn_shield;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fp_cmd_response {
    pub result: uint8_t,
    pub c2rust_unnamed: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub parse_msg: gxfp_parse_msg_t,
    pub verify: gxfp_verify_t,
    pub enroll_init: gxfp_enroll_init_t,
    pub capture_data_resp: gxfp_capturedata_t,
    pub check_duplicate_resp: gxfp_check_duplicate_t,
    pub enroll_commit: gxfp_enroll_commit_t,
    pub enroll_update: gxfp_enroll_update_t,
    pub finger_list_resp: gxfp_enum_fingerlist_t,
    pub version_info: gxfp_version_info_t,
    pub finger_status: fp_finger_status_t,
    pub finger_config: fp_finger_config_t,
    pub power_button_shield_resp: fp_pwr_btn_shield_t,
}
pub type pgxfp_cmd_response_t = *mut _fp_cmd_response;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _pack_header {
    pub cmd0: uint8_t,
    pub cmd1: uint8_t,
    pub packagenum: uint8_t,
    pub reserved: uint8_t,
    pub len: uint16_t,
    pub crc8: uint8_t,
    pub rev_crc8: uint8_t,
}
pub type pack_header = _pack_header;
pub type ppack_header = *mut _pack_header;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gxfp_sensor_cfg {
    pub config: [uint8_t; 26],
    pub reserved: [uint8_t; 98],
    pub crc_value: [uint8_t; 4],
}
pub type pgxfp_sensor_cfg_t = *mut _gxfp_sensor_cfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gf_crc32_context {
    pub crc: uint32_t,
}
#[no_mangle]
pub unsafe extern "C" fn gx_proto_crc8_calc(
    mut lubp_date: *mut uint8_t,
    mut lui_len: uint32_t,
) -> uint8_t {
    let mut data: *const uint8_t = lubp_date;
    let mut crc: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = lui_len as libc::c_int;
    while j != 0 {
        crc ^= ((*data as libc::c_int) << 8 as libc::c_int) as libc::c_uint;
        i = 8 as libc::c_int;
        while i != 0 {
            if crc & 0x8000 as libc::c_int as libc::c_uint != 0 {
                crc ^= ((0x1070 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
            }
            crc <<= 1 as libc::c_int;
            i -= 1;
        }
        j -= 1;
        data = data.offset(1);
    }
    crc >>= 8 as libc::c_int;
    crc = !crc;
    return crc as uint8_t;
}
static mut s_crc_table: [uint32_t; 256] = [
    0 as libc::c_int as uint32_t,
    0x4c11db7 as libc::c_int as uint32_t,
    0x9823b6e as libc::c_int as uint32_t,
    0xd4326d9 as libc::c_int as uint32_t,
    0x130476dc as libc::c_int as uint32_t,
    0x17c56b6b as libc::c_int as uint32_t,
    0x1a864db2 as libc::c_int as uint32_t,
    0x1e475005 as libc::c_int as uint32_t,
    0x2608edb8 as libc::c_int as uint32_t,
    0x22c9f00f as libc::c_int as uint32_t,
    0x2f8ad6d6 as libc::c_int as uint32_t,
    0x2b4bcb61 as libc::c_int as uint32_t,
    0x350c9b64 as libc::c_int as uint32_t,
    0x31cd86d3 as libc::c_int as uint32_t,
    0x3c8ea00a as libc::c_int as uint32_t,
    0x384fbdbd as libc::c_int as uint32_t,
    0x4c11db70 as libc::c_int as uint32_t,
    0x48d0c6c7 as libc::c_int as uint32_t,
    0x4593e01e as libc::c_int as uint32_t,
    0x4152fda9 as libc::c_int as uint32_t,
    0x5f15adac as libc::c_int as uint32_t,
    0x5bd4b01b as libc::c_int as uint32_t,
    0x569796c2 as libc::c_int as uint32_t,
    0x52568b75 as libc::c_int as uint32_t,
    0x6a1936c8 as libc::c_int as uint32_t,
    0x6ed82b7f as libc::c_int as uint32_t,
    0x639b0da6 as libc::c_int as uint32_t,
    0x675a1011 as libc::c_int as uint32_t,
    0x791d4014 as libc::c_int as uint32_t,
    0x7ddc5da3 as libc::c_int as uint32_t,
    0x709f7b7a as libc::c_int as uint32_t,
    0x745e66cd as libc::c_int as uint32_t,
    0x9823b6e0 as libc::c_uint,
    0x9ce2ab57 as libc::c_uint,
    0x91a18d8e as libc::c_uint,
    0x95609039 as libc::c_uint,
    0x8b27c03c as libc::c_uint,
    0x8fe6dd8b as libc::c_uint,
    0x82a5fb52 as libc::c_uint,
    0x8664e6e5 as libc::c_uint,
    0xbe2b5b58 as libc::c_uint,
    0xbaea46ef as libc::c_uint,
    0xb7a96036 as libc::c_uint,
    0xb3687d81 as libc::c_uint,
    0xad2f2d84 as libc::c_uint,
    0xa9ee3033 as libc::c_uint,
    0xa4ad16ea as libc::c_uint,
    0xa06c0b5d as libc::c_uint,
    0xd4326d90 as libc::c_uint,
    0xd0f37027 as libc::c_uint,
    0xddb056fe as libc::c_uint,
    0xd9714b49 as libc::c_uint,
    0xc7361b4c as libc::c_uint,
    0xc3f706fb as libc::c_uint,
    0xceb42022 as libc::c_uint,
    0xca753d95 as libc::c_uint,
    0xf23a8028 as libc::c_uint,
    0xf6fb9d9f as libc::c_uint,
    0xfbb8bb46 as libc::c_uint,
    0xff79a6f1 as libc::c_uint,
    0xe13ef6f4 as libc::c_uint,
    0xe5ffeb43 as libc::c_uint,
    0xe8bccd9a as libc::c_uint,
    0xec7dd02d as libc::c_uint,
    0x34867077 as libc::c_int as uint32_t,
    0x30476dc0 as libc::c_int as uint32_t,
    0x3d044b19 as libc::c_int as uint32_t,
    0x39c556ae as libc::c_int as uint32_t,
    0x278206ab as libc::c_int as uint32_t,
    0x23431b1c as libc::c_int as uint32_t,
    0x2e003dc5 as libc::c_int as uint32_t,
    0x2ac12072 as libc::c_int as uint32_t,
    0x128e9dcf as libc::c_int as uint32_t,
    0x164f8078 as libc::c_int as uint32_t,
    0x1b0ca6a1 as libc::c_int as uint32_t,
    0x1fcdbb16 as libc::c_int as uint32_t,
    0x18aeb13 as libc::c_int as uint32_t,
    0x54bf6a4 as libc::c_int as uint32_t,
    0x808d07d as libc::c_int as uint32_t,
    0xcc9cdca as libc::c_int as uint32_t,
    0x7897ab07 as libc::c_int as uint32_t,
    0x7c56b6b0 as libc::c_int as uint32_t,
    0x71159069 as libc::c_int as uint32_t,
    0x75d48dde as libc::c_int as uint32_t,
    0x6b93dddb as libc::c_int as uint32_t,
    0x6f52c06c as libc::c_int as uint32_t,
    0x6211e6b5 as libc::c_int as uint32_t,
    0x66d0fb02 as libc::c_int as uint32_t,
    0x5e9f46bf as libc::c_int as uint32_t,
    0x5a5e5b08 as libc::c_int as uint32_t,
    0x571d7dd1 as libc::c_int as uint32_t,
    0x53dc6066 as libc::c_int as uint32_t,
    0x4d9b3063 as libc::c_int as uint32_t,
    0x495a2dd4 as libc::c_int as uint32_t,
    0x44190b0d as libc::c_int as uint32_t,
    0x40d816ba as libc::c_int as uint32_t,
    0xaca5c697 as libc::c_uint,
    0xa864db20 as libc::c_uint,
    0xa527fdf9 as libc::c_uint,
    0xa1e6e04e as libc::c_uint,
    0xbfa1b04b as libc::c_uint,
    0xbb60adfc as libc::c_uint,
    0xb6238b25 as libc::c_uint,
    0xb2e29692 as libc::c_uint,
    0x8aad2b2f as libc::c_uint,
    0x8e6c3698 as libc::c_uint,
    0x832f1041 as libc::c_uint,
    0x87ee0df6 as libc::c_uint,
    0x99a95df3 as libc::c_uint,
    0x9d684044 as libc::c_uint,
    0x902b669d as libc::c_uint,
    0x94ea7b2a as libc::c_uint,
    0xe0b41de7 as libc::c_uint,
    0xe4750050 as libc::c_uint,
    0xe9362689 as libc::c_uint,
    0xedf73b3e as libc::c_uint,
    0xf3b06b3b as libc::c_uint,
    0xf771768c as libc::c_uint,
    0xfa325055 as libc::c_uint,
    0xfef34de2 as libc::c_uint,
    0xc6bcf05f as libc::c_uint,
    0xc27dede8 as libc::c_uint,
    0xcf3ecb31 as libc::c_uint,
    0xcbffd686 as libc::c_uint,
    0xd5b88683 as libc::c_uint,
    0xd1799b34 as libc::c_uint,
    0xdc3abded as libc::c_uint,
    0xd8fba05a as libc::c_uint,
    0x690ce0ee as libc::c_int as uint32_t,
    0x6dcdfd59 as libc::c_int as uint32_t,
    0x608edb80 as libc::c_int as uint32_t,
    0x644fc637 as libc::c_int as uint32_t,
    0x7a089632 as libc::c_int as uint32_t,
    0x7ec98b85 as libc::c_int as uint32_t,
    0x738aad5c as libc::c_int as uint32_t,
    0x774bb0eb as libc::c_int as uint32_t,
    0x4f040d56 as libc::c_int as uint32_t,
    0x4bc510e1 as libc::c_int as uint32_t,
    0x46863638 as libc::c_int as uint32_t,
    0x42472b8f as libc::c_int as uint32_t,
    0x5c007b8a as libc::c_int as uint32_t,
    0x58c1663d as libc::c_int as uint32_t,
    0x558240e4 as libc::c_int as uint32_t,
    0x51435d53 as libc::c_int as uint32_t,
    0x251d3b9e as libc::c_int as uint32_t,
    0x21dc2629 as libc::c_int as uint32_t,
    0x2c9f00f0 as libc::c_int as uint32_t,
    0x285e1d47 as libc::c_int as uint32_t,
    0x36194d42 as libc::c_int as uint32_t,
    0x32d850f5 as libc::c_int as uint32_t,
    0x3f9b762c as libc::c_int as uint32_t,
    0x3b5a6b9b as libc::c_int as uint32_t,
    0x315d626 as libc::c_int as uint32_t,
    0x7d4cb91 as libc::c_int as uint32_t,
    0xa97ed48 as libc::c_int as uint32_t,
    0xe56f0ff as libc::c_int as uint32_t,
    0x1011a0fa as libc::c_int as uint32_t,
    0x14d0bd4d as libc::c_int as uint32_t,
    0x19939b94 as libc::c_int as uint32_t,
    0x1d528623 as libc::c_int as uint32_t,
    0xf12f560e as libc::c_uint,
    0xf5ee4bb9 as libc::c_uint,
    0xf8ad6d60 as libc::c_uint,
    0xfc6c70d7 as libc::c_uint,
    0xe22b20d2 as libc::c_uint,
    0xe6ea3d65 as libc::c_uint,
    0xeba91bbc as libc::c_uint,
    0xef68060b as libc::c_uint,
    0xd727bbb6 as libc::c_uint,
    0xd3e6a601 as libc::c_uint,
    0xdea580d8 as libc::c_uint,
    0xda649d6f as libc::c_uint,
    0xc423cd6a as libc::c_uint,
    0xc0e2d0dd as libc::c_uint,
    0xcda1f604 as libc::c_uint,
    0xc960ebb3 as libc::c_uint,
    0xbd3e8d7e as libc::c_uint,
    0xb9ff90c9 as libc::c_uint,
    0xb4bcb610 as libc::c_uint,
    0xb07daba7 as libc::c_uint,
    0xae3afba2 as libc::c_uint,
    0xaafbe615 as libc::c_uint,
    0xa7b8c0cc as libc::c_uint,
    0xa379dd7b as libc::c_uint,
    0x9b3660c6 as libc::c_uint,
    0x9ff77d71 as libc::c_uint,
    0x92b45ba8 as libc::c_uint,
    0x9675461f as libc::c_uint,
    0x8832161a as libc::c_uint,
    0x8cf30bad as libc::c_uint,
    0x81b02d74 as libc::c_uint,
    0x857130c3 as libc::c_uint,
    0x5d8a9099 as libc::c_int as uint32_t,
    0x594b8d2e as libc::c_int as uint32_t,
    0x5408abf7 as libc::c_int as uint32_t,
    0x50c9b640 as libc::c_int as uint32_t,
    0x4e8ee645 as libc::c_int as uint32_t,
    0x4a4ffbf2 as libc::c_int as uint32_t,
    0x470cdd2b as libc::c_int as uint32_t,
    0x43cdc09c as libc::c_int as uint32_t,
    0x7b827d21 as libc::c_int as uint32_t,
    0x7f436096 as libc::c_int as uint32_t,
    0x7200464f as libc::c_int as uint32_t,
    0x76c15bf8 as libc::c_int as uint32_t,
    0x68860bfd as libc::c_int as uint32_t,
    0x6c47164a as libc::c_int as uint32_t,
    0x61043093 as libc::c_int as uint32_t,
    0x65c52d24 as libc::c_int as uint32_t,
    0x119b4be9 as libc::c_int as uint32_t,
    0x155a565e as libc::c_int as uint32_t,
    0x18197087 as libc::c_int as uint32_t,
    0x1cd86d30 as libc::c_int as uint32_t,
    0x29f3d35 as libc::c_int as uint32_t,
    0x65e2082 as libc::c_int as uint32_t,
    0xb1d065b as libc::c_int as uint32_t,
    0xfdc1bec as libc::c_int as uint32_t,
    0x3793a651 as libc::c_int as uint32_t,
    0x3352bbe6 as libc::c_int as uint32_t,
    0x3e119d3f as libc::c_int as uint32_t,
    0x3ad08088 as libc::c_int as uint32_t,
    0x2497d08d as libc::c_int as uint32_t,
    0x2056cd3a as libc::c_int as uint32_t,
    0x2d15ebe3 as libc::c_int as uint32_t,
    0x29d4f654 as libc::c_int as uint32_t,
    0xc5a92679 as libc::c_uint,
    0xc1683bce as libc::c_uint,
    0xcc2b1d17 as libc::c_uint,
    0xc8ea00a0 as libc::c_uint,
    0xd6ad50a5 as libc::c_uint,
    0xd26c4d12 as libc::c_uint,
    0xdf2f6bcb as libc::c_uint,
    0xdbee767c as libc::c_uint,
    0xe3a1cbc1 as libc::c_uint,
    0xe760d676 as libc::c_uint,
    0xea23f0af as libc::c_uint,
    0xeee2ed18 as libc::c_uint,
    0xf0a5bd1d as libc::c_uint,
    0xf464a0aa as libc::c_uint,
    0xf9278673 as libc::c_uint,
    0xfde69bc4 as libc::c_uint,
    0x89b8fd09 as libc::c_uint,
    0x8d79e0be as libc::c_uint,
    0x803ac667 as libc::c_uint,
    0x84fbdbd0 as libc::c_uint,
    0x9abc8bd5 as libc::c_uint,
    0x9e7d9662 as libc::c_uint,
    0x933eb0bb as libc::c_uint,
    0x97ffad0c as libc::c_uint,
    0xafb010b1 as libc::c_uint,
    0xab710d06 as libc::c_uint,
    0xa6322bdf as libc::c_uint,
    0xa2f33668 as libc::c_uint,
    0xbcb4666d as libc::c_uint,
    0xb8757bda as libc::c_uint,
    0xb5365d03 as libc::c_uint,
    0xb1f740b4 as libc::c_uint,
];
unsafe extern "C" fn reflect(mut data: uint32_t, mut n_bits: uint8_t) -> uint32_t {
    let mut reflection: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut bit: uint8_t = 0;
    bit = 0 as libc::c_int as uint8_t;
    while (bit as libc::c_int) < n_bits as libc::c_int {
        if data & 0x1 as libc::c_int as libc::c_uint != 0 {
            reflection
                |= ((1 as libc::c_int)
                    << n_bits as libc::c_int - 1 as libc::c_int - bit as libc::c_int)
                    as libc::c_ulong;
        }
        data = data >> 1 as libc::c_int;
        bit = bit.wrapping_add(1);
    }
    return reflection as uint32_t;
}
unsafe extern "C" fn crc32_init(mut ctx: *mut gf_crc32_context) {
    (*ctx).crc = 0xffffffff as libc::c_uint;
}
unsafe extern "C" fn crc32_update(
    mut ctx: *mut gf_crc32_context,
    mut message: *const uint8_t,
    mut n_bytes: uint32_t,
) {
    let mut data: uint8_t = 0;
    let mut byte: uint32_t = 0;
    byte = 0 as libc::c_int as uint32_t;
    while byte < n_bytes {
        data = (reflect(
            *message.offset(byte as isize) as uint32_t,
            8 as libc::c_int as uint8_t,
        ) as uint8_t as libc::c_uint
            ^ (*ctx).crc
                >> (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong)) as uint8_t;
        (*ctx).crc = s_crc_table[data as usize] ^ (*ctx).crc << 8 as libc::c_int;
        byte = byte.wrapping_add(1);
    }
}
unsafe extern "C" fn crc32_final(mut ctx: *mut gf_crc32_context, mut md: *mut uint8_t) {
    let mut crc: uint32_t = 0 as libc::c_int as uint32_t;
    (*ctx)
        .crc = reflect(
        (*ctx).crc,
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong) as uint8_t,
    ) ^ 0xffffffff as libc::c_uint;
    crc = (*ctx).crc;
    memcpy(
        md as *mut libc::c_void,
        &mut crc as *mut uint32_t as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gx_proto_crc32_calc(
    mut pchMsg: *mut uint8_t,
    mut wDataLen: uint32_t,
    mut pchMsgDst: *mut uint8_t,
) -> uint8_t {
    let mut context: gf_crc32_context = {
        let mut init = gf_crc32_context {
            crc: 0 as libc::c_int as uint32_t,
        };
        init
    };
    if pchMsg.is_null() {
        return 0 as libc::c_int as uint8_t;
    }
    crc32_init(&mut context);
    crc32_update(&mut context, pchMsg, wDataLen);
    crc32_final(&mut context, pchMsgDst);
    return 1 as libc::c_int as uint8_t;
}
static mut dump_seq: uint8_t = 0 as libc::c_int as uint8_t;
unsafe extern "C" fn init_pack_header(
    mut pheader: ppack_header,
    mut len: uint16_t,
    mut cmd: uint16_t,
    mut packagenum: uint8_t,
) {
    if ({
        let mut _g_boolean_var_: libc::c_int = 0;
        if !pheader.is_null() {
            _g_boolean_var_ = 1 as libc::c_int;
        } else {
            _g_boolean_var_ = 0 as libc::c_int;
        }
        _g_boolean_var_
    }) as libc::c_long != 0
    {} else {
        g_assertion_message_expr(
            b"libfprint\0" as *const u8 as *const libc::c_char,
            b"../libfprint/drivers/goodixmoc/goodix_proto.c\0" as *const u8
                as *const libc::c_char,
            183 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"init_pack_header\0"))
                .as_ptr(),
            b"pheader\0" as *const u8 as *const libc::c_char,
        );
    }
    memset(
        pheader as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<_pack_header>() as libc::c_ulong,
    );
    (*pheader)
        .cmd0 = (cmd as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as uint8_t;
    (*pheader).cmd1 = cmd as uint8_t;
    (*pheader).packagenum = packagenum;
    let fresh0 = dump_seq;
    dump_seq = dump_seq.wrapping_add(1);
    (*pheader).reserved = fresh0;
    (*pheader).len = (len as libc::c_int + 4 as libc::c_int) as guint16;
    (*pheader)
        .crc8 = gx_proto_crc8_calc(
        pheader as *mut uint8_t,
        6 as libc::c_int as uint32_t,
    );
    (*pheader).rev_crc8 = !((*pheader).crc8 as libc::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn gx_proto_build_package(
    mut ppackage: *mut uint8_t,
    mut package_len: *mut uint32_t,
    mut cmd: uint16_t,
    mut payload: *const uint8_t,
    mut payload_size: uint32_t,
) -> libc::c_int {
    let mut header: pack_header = pack_header {
        cmd0: 0,
        cmd1: 0,
        packagenum: 0,
        reserved: 0,
        len: 0,
        crc8: 0,
        rev_crc8: 0,
    };
    if ppackage.is_null() || package_len.is_null() {
        return -(1 as libc::c_int);
    }
    if *package_len
        < payload_size
            .wrapping_add(8 as libc::c_int as libc::c_uint)
            .wrapping_add(4 as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    init_pack_header(
        &mut header,
        payload_size as uint16_t,
        cmd,
        0 as libc::c_int as uint8_t,
    );
    memcpy(
        ppackage as *mut libc::c_void,
        &mut header as *mut pack_header as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        ppackage.offset(8 as libc::c_int as isize) as *mut libc::c_void,
        payload as *const libc::c_void,
        payload_size as libc::c_ulong,
    );
    gx_proto_crc32_calc(
        ppackage,
        (8 as libc::c_int as libc::c_uint).wrapping_add(payload_size),
        ppackage.offset(8 as libc::c_int as isize).offset(payload_size as isize),
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gx_proto_parse_header(
    mut buffer: *mut uint8_t,
    mut buffer_len: uint32_t,
    mut pheader: *mut pack_header,
) -> libc::c_int {
    if buffer.is_null() || pheader.is_null() {
        return -(1 as libc::c_int);
    }
    if buffer_len < (8 as libc::c_int + 4 as libc::c_int) as libc::c_uint {
        return -(1 as libc::c_int);
    }
    memcpy(
        pheader as *mut libc::c_void,
        buffer as *const libc::c_void,
        ::core::mem::size_of::<pack_header>() as libc::c_ulong,
    );
    (*pheader).len = (*pheader).len;
    if buffer_len < ((*pheader).len as libc::c_int + 8 as libc::c_int) as libc::c_uint {
        return -(1 as libc::c_int);
    }
    (*pheader).len = ((*pheader).len as libc::c_int - 4 as libc::c_int) as uint16_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn gx_proto_parse_fingerid(
    mut fid_buffer: *mut uint8_t,
    mut fid_buffer_size: uint16_t,
    mut template: ptemplate_format_t,
) -> libc::c_int {
    let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut Offset: uint16_t = 0 as libc::c_int as uint16_t;
    if template.is_null() || fid_buffer.is_null() {
        return -(1 as libc::c_int);
    }
    if (fid_buffer_size as libc::c_ulong)
        < (68 as libc::c_ulong as glong as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
    {
        return -(1 as libc::c_int);
    }
    buffer = fid_buffer;
    Offset = 0 as libc::c_int as uint16_t;
    let fresh1 = Offset;
    Offset = Offset.wrapping_add(1);
    if *buffer.offset(fresh1 as isize) as libc::c_int != 67 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let fresh2 = Offset;
    Offset = Offset.wrapping_add(1);
    (*template).type_0 = *buffer.offset(fresh2 as isize);
    let fresh3 = Offset;
    Offset = Offset.wrapping_add(1);
    (*template).finger_index = *buffer.offset(fresh3 as isize);
    Offset = Offset.wrapping_add(1);
    memcpy(
        ((*template).accountid).as_mut_ptr() as *mut libc::c_void,
        &mut *buffer.offset(Offset as isize) as *mut uint8_t as *const libc::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong,
    );
    Offset = (Offset as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong)
        as uint16_t as uint16_t;
    memcpy(
        ((*template).tid).as_mut_ptr() as *mut libc::c_void,
        &mut *buffer.offset(Offset as isize) as *mut uint8_t as *const libc::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong,
    );
    Offset = (Offset as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong)
        as uint16_t as uint16_t;
    let fresh4 = Offset;
    Offset = Offset.wrapping_add(1);
    (*template).payload.size = *buffer.offset(fresh4 as isize);
    if (*template).payload.size as libc::c_ulong
        > ::core::mem::size_of::<[uint8_t; 56]>() as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    if (*template).payload.size as libc::c_int + Offset as libc::c_int
        > fid_buffer_size as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    memset(
        ((*template).payload.data).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (*template).payload.size as libc::c_ulong,
    );
    memcpy(
        ((*template).payload.data).as_mut_ptr() as *mut libc::c_void,
        &mut *buffer.offset(Offset as isize) as *mut uint8_t as *const libc::c_void,
        (*template).payload.size as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gx_proto_parse_body(
    mut cmd: uint16_t,
    mut buffer: *mut uint8_t,
    mut buffer_len: uint16_t,
    mut presp: pgxfp_cmd_response_t,
) -> libc::c_int {
    let mut offset: uint16_t = 0 as libc::c_int as uint16_t;
    let mut fingerlist: *mut uint8_t = 0 as *mut uint8_t;
    if buffer.is_null() || presp.is_null() {
        return -(1 as libc::c_int);
    }
    if (buffer_len as libc::c_int) < 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*presp).result = *buffer.offset(0 as libc::c_int as isize);
    match (cmd as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t
        as libc::c_int
    {
        170 => {
            if (buffer_len as libc::c_ulong)
                < (::core::mem::size_of::<gxfp_parse_msg_t>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                return -(1 as libc::c_int);
            }
            (*presp)
                .c2rust_unnamed
                .parse_msg
                .ack_cmd = *buffer.offset(1 as libc::c_int as isize);
        }
        192 => {
            (*presp)
                .c2rust_unnamed
                .finger_config
                .status = *buffer.offset(0 as libc::c_int as isize);
            if buffer_len as libc::c_int >= 3 as libc::c_int {
                (*presp)
                    .c2rust_unnamed
                    .finger_config
                    .max_stored_prints = *buffer.offset(2 as libc::c_int as isize);
            } else {
                (*presp)
                    .c2rust_unnamed
                    .finger_config
                    .max_stored_prints = 20 as libc::c_int as uint8_t;
            }
        }
        224 => {
            (*presp).c2rust_unnamed.power_button_shield_resp.resp_cmd1 = cmd as uint8_t;
            if buffer_len as libc::c_int >= 2 as libc::c_int {
                let mut support_pwr_shield: uint8_t = *buffer
                    .offset(1 as libc::c_int as isize);
                if support_pwr_shield as libc::c_int == 0xff as libc::c_int {
                    g_log(
                        b"libfprint\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"Power button shield feature not supported!\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
        }
        208 => {
            if (buffer_len as libc::c_ulong)
                < (::core::mem::size_of::<gxfp_version_info_t>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                return -(1 as libc::c_int);
            }
            memcpy(
                &mut (*presp).c2rust_unnamed.version_info as *mut gxfp_version_info_t
                    as *mut libc::c_void,
                buffer.offset(1 as libc::c_int as isize) as *const libc::c_void,
                ::core::mem::size_of::<gxfp_version_info_t>() as libc::c_ulong,
            );
        }
        162 => {
            if cmd as uint8_t as libc::c_int == 0 as libc::c_int {
                if (buffer_len as libc::c_ulong)
                    < (::core::mem::size_of::<gxfp_capturedata_t>() as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    return -(1 as libc::c_int);
                }
                (*presp)
                    .c2rust_unnamed
                    .capture_data_resp
                    .img_quality = *buffer.offset(1 as libc::c_int as isize);
                (*presp)
                    .c2rust_unnamed
                    .capture_data_resp
                    .img_coverage = *buffer.offset(2 as libc::c_int as isize);
            }
        }
        161 => {
            if (buffer_len as libc::c_ulong)
                < (::core::mem::size_of::<gxfp_enroll_init_t>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                return -(1 as libc::c_int);
            }
            if (*presp).result as libc::c_int == 0 as libc::c_int {
                memcpy(
                    &mut (*presp).c2rust_unnamed.enroll_init.tid as *mut [uint8_t; 32]
                        as *mut libc::c_void,
                    &mut *buffer.offset(1 as libc::c_int as isize) as *mut uint8_t
                        as *const libc::c_void,
                    32 as libc::c_int as libc::c_ulong,
                );
            }
        }
        160 => {
            if (buffer_len as libc::c_ulong)
                < ::core::mem::size_of::<gxfp_enroll_update_t>() as libc::c_ulong
            {
                return -(1 as libc::c_int);
            }
            (*presp)
                .c2rust_unnamed
                .enroll_update
                .rollback = if (*buffer.offset(0 as libc::c_int as isize) as libc::c_int)
                < 0x80 as libc::c_int
            {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            } != 0;
            (*presp)
                .c2rust_unnamed
                .enroll_update
                .img_overlay = *buffer.offset(1 as libc::c_int as isize);
            (*presp)
                .c2rust_unnamed
                .enroll_update
                .img_preoverlay = *buffer.offset(2 as libc::c_int as isize);
        }
        163 => {
            (*presp)
                .c2rust_unnamed
                .check_duplicate_resp
                .duplicate = if (*presp).result as libc::c_int == 0 as libc::c_int {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            } != 0;
            if (*presp).c2rust_unnamed.check_duplicate_resp.duplicate {
                if (buffer_len as libc::c_int) < 3 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                let mut tid_size: uint16_t = *(buffer.offset(1 as libc::c_int as isize)
                    as *mut uint16_t);
                offset = (offset as libc::c_int + 3 as libc::c_int) as uint16_t;
                if (buffer_len as libc::c_int)
                    < tid_size as libc::c_int + offset as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
                if gx_proto_parse_fingerid(
                    buffer.offset(offset as libc::c_int as isize),
                    tid_size,
                    &mut (*presp).c2rust_unnamed.check_duplicate_resp.template,
                ) != 0 as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
            }
        }
        166 => {
            if !((*presp).result as libc::c_int != 0 as libc::c_int) {
                if (buffer_len as libc::c_int) < 2 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                (*presp)
                    .c2rust_unnamed
                    .finger_list_resp
                    .finger_num = *buffer.offset(1 as libc::c_int as isize);
                fingerlist = buffer.offset(2 as libc::c_int as isize);
                let mut num: uint8_t = 0 as libc::c_int as uint8_t;
                while (num as libc::c_int)
                    < (*presp).c2rust_unnamed.finger_list_resp.finger_num as libc::c_int
                {
                    let mut fingerid_length: uint16_t = 0;
                    if (buffer_len as libc::c_int)
                        < offset as libc::c_int + 2 as libc::c_int
                    {
                        return -(1 as libc::c_int);
                    }
                    fingerid_length = *(fingerlist.offset(offset as libc::c_int as isize)
                        as *mut uint16_t);
                    offset = (offset as libc::c_int + 2 as libc::c_int) as uint16_t;
                    if (buffer_len as libc::c_int)
                        < fingerid_length as libc::c_int + offset as libc::c_int
                    {
                        return -(1 as libc::c_int);
                    }
                    if gx_proto_parse_fingerid(
                        fingerlist.offset(offset as libc::c_int as isize),
                        fingerid_length,
                        &mut *((*presp).c2rust_unnamed.finger_list_resp.finger_list)
                            .as_mut_ptr()
                            .offset(num as isize),
                    ) != 0 as libc::c_int
                    {
                        g_log(
                            b"libfprint\0" as *const u8 as *const libc::c_char,
                            G_LOG_LEVEL_WARNING,
                            b"Failed to parse finger list\0" as *const u8
                                as *const libc::c_char,
                        );
                        return -(1 as libc::c_int);
                    }
                    offset = (offset as libc::c_int + fingerid_length as libc::c_int)
                        as uint16_t;
                    num = num.wrapping_add(1);
                }
            }
        }
        165 => {
            let mut score: uint32_t = 0 as libc::c_int as uint32_t;
            let mut study: uint8_t = 0 as libc::c_int as uint8_t;
            let mut fingerid_size: uint16_t = 0 as libc::c_int as uint16_t;
            (*presp)
                .c2rust_unnamed
                .verify
                .match_0 = if *buffer.offset(0 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } != 0;
            if (*presp).c2rust_unnamed.verify.match_0 {
                if (buffer_len as libc::c_int) < 10 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                offset = (offset as libc::c_int + 1 as libc::c_int) as uint16_t;
                (*presp)
                    .c2rust_unnamed
                    .verify
                    .rejectdetail = *(buffer.offset(offset as libc::c_int as isize)
                    as *mut uint16_t) as uint32_t;
                offset = (offset as libc::c_int + 2 as libc::c_int) as uint16_t;
                score = *(buffer.offset(offset as libc::c_int as isize)
                    as *mut uint32_t);
                offset = (offset as libc::c_int + 4 as libc::c_int) as uint16_t;
                study = *buffer.offset(offset as isize);
                offset = (offset as libc::c_int + 1 as libc::c_int) as uint16_t;
                fingerid_size = *(buffer.offset(offset as libc::c_int as isize)
                    as *mut uint16_t);
                offset = (offset as libc::c_int + 2 as libc::c_int) as uint16_t;
                if (buffer_len as libc::c_int)
                    < fingerid_size as libc::c_int + offset as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
                if gx_proto_parse_fingerid(
                    buffer.offset(offset as libc::c_int as isize),
                    fingerid_size,
                    &mut (*presp).c2rust_unnamed.verify.template,
                ) != 0 as libc::c_int
                {
                    (*presp).result = 0x80 as libc::c_int as uint8_t;
                } else {
                    g_log(
                        b"libfprint\0" as *const u8 as *const libc::c_char,
                        G_LOG_LEVEL_DEBUG,
                        b"match, score: %d, study: %d\0" as *const u8
                            as *const libc::c_char,
                        score,
                        study as libc::c_int,
                    );
                }
            }
        }
        176 => {
            (*presp)
                .c2rust_unnamed
                .finger_status
                .status = *buffer.offset(0 as libc::c_int as isize);
        }
        164 | 167 | _ => {}
    }
    return 0 as libc::c_int;
}
static mut sensor_config: [uint8_t; 26] = [
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0x50 as libc::c_int as uint8_t,
    0xf as libc::c_int as uint8_t,
    0x41 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0xa as libc::c_int as uint8_t,
    0x18 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x23 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn gx_proto_init_sensor_config(
    mut pconfig: pgxfp_sensor_cfg_t,
) -> libc::c_int {
    let mut crc32_calc: uint32_t = 0 as libc::c_int as uint32_t;
    if pconfig.is_null() {
        return -(1 as libc::c_int);
    }
    memset(
        pconfig as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<_gxfp_sensor_cfg>() as libc::c_ulong,
    );
    memcpy(
        &mut (*pconfig).config as *mut [uint8_t; 26] as *mut libc::c_void,
        sensor_config.as_mut_ptr() as *const libc::c_void,
        (::core::mem::size_of::<[uint8_t; 26]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
    );
    (*pconfig).reserved[0 as libc::c_int as usize] = 1 as libc::c_int as uint8_t;
    gx_proto_crc32_calc(
        pconfig as *mut uint8_t,
        (::core::mem::size_of::<_gxfp_sensor_cfg>() as libc::c_ulong)
            .wrapping_sub(4 as libc::c_int as libc::c_ulong) as uint32_t,
        &mut crc32_calc as *mut uint32_t as *mut uint8_t,
    );
    memcpy(
        ((*pconfig).crc_value).as_mut_ptr() as *mut libc::c_void,
        &mut crc32_calc as *mut uint32_t as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
