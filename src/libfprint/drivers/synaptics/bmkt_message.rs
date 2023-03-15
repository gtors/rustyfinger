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
}
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_init_resp {
    pub finger_presence: uint8_t,
}
pub type bmkt_init_resp_t = bmkt_init_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_enroll_resp {
    pub progress: libc::c_int,
    pub finger_id: uint8_t,
    pub user_id: [uint8_t; 100],
}
pub type bmkt_enroll_resp_t = bmkt_enroll_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_auth_resp {
    pub match_result: libc::c_double,
    pub finger_id: uint8_t,
    pub user_id: [uint8_t; 100],
}
pub type bmkt_verify_resp_t = bmkt_auth_resp;
pub type bmkt_identify_resp_t = bmkt_auth_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_fps_mode_resp {
    pub mode: uint8_t,
    pub level2_mode: uint8_t,
    pub cmd_id: uint8_t,
    pub finger_presence: uint8_t,
}
pub type bmkt_fps_mode_resp_t = bmkt_fps_mode_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_get_version_resp {
    pub part: [uint8_t; 10],
    pub year: uint8_t,
    pub week: uint8_t,
    pub patch: uint8_t,
    pub supplier_id: [uint8_t; 2],
}
pub type bmkt_get_version_resp_t = bmkt_get_version_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_get_db_capacity_resp {
    pub total: uint8_t,
    pub empty: uint8_t,
    pub bad_slots: uint8_t,
    pub corrupt_templates: uint8_t,
}
pub type bmkt_get_db_capacity_resp_t = bmkt_get_db_capacity_resp;
pub type bmkt_sec_level = libc::c_uint;
pub const BMKT_SECURITY_LEVEL_HIGH: bmkt_sec_level = 96;
pub const BMKT_SECURITY_LEVEL_MEDIUM: bmkt_sec_level = 64;
pub const BMKT_SECURITY_LEVEL_LOW: bmkt_sec_level = 16;
pub type bmkt_sec_level_t = bmkt_sec_level;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_set_sec_level_resp {
    pub sec_level: bmkt_sec_level_t,
}
pub type bmkt_set_sec_level_resp_t = bmkt_set_sec_level_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_del_all_users_resp {
    pub progress: libc::c_int,
}
pub type bmkt_del_all_users_resp_t = bmkt_del_all_users_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_del_user_resp {
    pub progress: libc::c_int,
}
pub type bmkt_del_user_resp_t = bmkt_del_user_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_enroll_template {
    pub user_id_len: uint8_t,
    pub template_status: uint8_t,
    pub finger_id: uint8_t,
    pub user_id: [uint8_t; 101],
}
pub type bmkt_enroll_template_t = bmkt_enroll_template;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_enroll_templates_resp {
    pub total_query_messages: uint8_t,
    pub query_sequence: uint8_t,
    pub templates: [bmkt_enroll_template_t; 15],
}
pub type bmkt_enroll_templates_resp_t = bmkt_enroll_templates_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_enrolled_fingers {
    pub finger_id: uint8_t,
    pub template_status: uint8_t,
}
pub type bmkt_enrolled_fingers_t = bmkt_enrolled_fingers;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_enrolled_fingers_resp {
    pub fingers: [bmkt_enrolled_fingers_t; 10],
}
pub type bmkt_enrolled_fingers_resp_t = bmkt_enrolled_fingers_resp;
#[derive(Copy, Clone)]
#[repr(C)]
pub union bmkt_response_data_t {
    pub init_resp: bmkt_init_resp_t,
    pub enroll_resp: bmkt_enroll_resp_t,
    pub verify_resp: bmkt_verify_resp_t,
    pub id_resp: bmkt_identify_resp_t,
    pub fps_mode_resp: bmkt_fps_mode_resp_t,
    pub get_version_resp: bmkt_get_version_resp_t,
    pub db_cap_resp: bmkt_get_db_capacity_resp_t,
    pub sec_level_resp: bmkt_set_sec_level_resp_t,
    pub del_all_users_resp: bmkt_del_all_users_resp_t,
    pub enroll_templates_resp: bmkt_enroll_templates_resp_t,
    pub del_user_resp: bmkt_del_user_resp_t,
    pub del_all_user_resp: bmkt_del_all_users_resp_t,
    pub enrolled_fingers_resp: bmkt_enrolled_fingers_resp_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_response {
    pub response_id: libc::c_int,
    pub result: libc::c_int,
    pub complete: libc::c_int,
    pub response: bmkt_response_data_t,
}
pub type bmkt_response_t = bmkt_response;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmkt_msg_resp {
    pub msg_id: uint8_t,
    pub seq_num: uint8_t,
    pub payload_len: uint8_t,
    pub payload: *mut uint8_t,
    pub result: libc::c_int,
}
pub type bmkt_msg_resp_t = bmkt_msg_resp;
unsafe extern "C" fn extract8(
    mut buf: *const uint8_t,
    mut offset: *mut libc::c_int,
) -> uint8_t {
    let mut ret: uint8_t = 0 as libc::c_int as uint8_t;
    let mut off: libc::c_int = 0 as libc::c_int;
    if !offset.is_null() {
        off = *offset;
    }
    ret = *buf.offset(off as isize);
    if !offset.is_null() {
        *offset += 1 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn parse_error_response(
    mut msg_resp: *mut bmkt_msg_resp_t,
    mut resp: *mut bmkt_response_t,
) -> libc::c_int {
    if (*msg_resp).payload_len as libc::c_int != 2 as libc::c_int {
        return 112 as libc::c_int;
    }
    (*resp)
        .result = (*((*msg_resp).payload).offset(0 as libc::c_int as isize)
        as libc::c_int) << 8 as libc::c_int
        | *((*msg_resp).payload).offset(1 as libc::c_int as isize) as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_init_ok(
    mut msg_resp: *mut bmkt_msg_resp_t,
    mut resp: *mut bmkt_response_t,
) -> libc::c_int {
    let mut init_resp: *mut bmkt_init_resp_t = &mut (*resp).response.init_resp;
    if (*msg_resp).payload_len as libc::c_int != 1 as libc::c_int {
        return 112 as libc::c_int;
    }
    (*init_resp).finger_presence = extract8((*msg_resp).payload, 0 as *mut libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_fps_mode_report(
    mut msg_resp: *mut bmkt_msg_resp_t,
    mut resp: *mut bmkt_response_t,
) -> libc::c_int {
    let mut offset: libc::c_int = 0 as libc::c_int;
    let mut fps_mode_resp: *mut bmkt_fps_mode_resp_t = &mut (*resp)
        .response
        .fps_mode_resp;
    if (*msg_resp).payload_len as libc::c_ulong
        != ::core::mem::size_of::<bmkt_fps_mode_resp_t>() as libc::c_ulong
    {
        return 112 as libc::c_int;
    }
    (*fps_mode_resp).mode = extract8((*msg_resp).payload, &mut offset);
    (*fps_mode_resp).level2_mode = extract8((*msg_resp).payload, &mut offset);
    (*fps_mode_resp).cmd_id = extract8((*msg_resp).payload, &mut offset);
    (*fps_mode_resp).finger_presence = extract8((*msg_resp).payload, &mut offset);
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_enroll_report(
    mut msg_resp: *mut bmkt_msg_resp_t,
    mut resp: *mut bmkt_response_t,
) -> libc::c_int {
    let mut enroll_resp: *mut bmkt_enroll_resp_t = &mut (*resp).response.enroll_resp;
    if (*msg_resp).payload_len as libc::c_int != 1 as libc::c_int {
        return 112 as libc::c_int;
    }
    (*enroll_resp)
        .progress = extract8((*msg_resp).payload, 0 as *mut libc::c_int) as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_enroll_ok(
    mut msg_resp: *mut bmkt_msg_resp_t,
    mut resp: *mut bmkt_response_t,
) -> libc::c_int {
    let mut enroll_resp: *mut bmkt_enroll_resp_t = &mut (*resp).response.enroll_resp;
    if ((*msg_resp).payload_len as libc::c_int) < 1 as libc::c_int
        || (*msg_resp).payload_len as libc::c_int > 100 as libc::c_int + 1 as libc::c_int
    {
        return 112 as libc::c_int;
    }
    (*enroll_resp).finger_id = *((*msg_resp).payload).offset(0 as libc::c_int as isize);
    memcpy(
        ((*enroll_resp).user_id).as_mut_ptr() as *mut libc::c_void,
        &mut *((*msg_resp).payload).offset(1 as libc::c_int as isize) as *mut uint8_t
            as *const libc::c_void,
        ((*msg_resp).payload_len as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_auth_ok(
    mut msg_resp: *mut bmkt_msg_resp_t,
    mut resp: *mut bmkt_response_t,
) -> libc::c_int {
    let mut id_resp: *mut bmkt_identify_resp_t = &mut (*resp).response.id_resp;
    if ((*msg_resp).payload_len as libc::c_int) < 3 as libc::c_int
        || (*msg_resp).payload_len as libc::c_int > 100 as libc::c_int + 3 as libc::c_int
    {
        return 112 as libc::c_int;
    }
    (*id_resp)
        .match_result = *((*msg_resp).payload).offset(0 as libc::c_int as isize)
        as libc::c_double
        + 0.01f64
            * *((*msg_resp).payload).offset(1 as libc::c_int as isize) as libc::c_double;
    (*id_resp).finger_id = *((*msg_resp).payload).offset(2 as libc::c_int as isize);
    memcpy(
        ((*id_resp).user_id).as_mut_ptr() as *mut libc::c_void,
        &mut *((*msg_resp).payload).offset(3 as libc::c_int as isize) as *mut uint8_t
            as *const libc::c_void,
        ((*msg_resp).payload_len as libc::c_int - 3 as libc::c_int) as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_security_level_report(
    mut msg_resp: *mut bmkt_msg_resp_t,
    mut resp: *mut bmkt_response_t,
) -> libc::c_int {
    let mut sec_level_resp: *mut bmkt_set_sec_level_resp_t = &mut (*resp)
        .response
        .sec_level_resp;
    if (*msg_resp).payload_len as libc::c_int != 1 as libc::c_int {
        return 112 as libc::c_int;
    }
    (*sec_level_resp)
        .sec_level = extract8((*msg_resp).payload, 0 as *mut libc::c_int)
        as bmkt_sec_level_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_del_all_users_progress_report(
    mut msg_resp: *mut bmkt_msg_resp_t,
    mut resp: *mut bmkt_response_t,
) -> libc::c_int {
    let mut del_all_users_resp: *mut bmkt_del_all_users_resp_t = &mut (*resp)
        .response
        .del_all_users_resp;
    if (*msg_resp).payload_len as libc::c_int != 1 as libc::c_int {
        return 112 as libc::c_int;
    }
    (*del_all_users_resp)
        .progress = extract8((*msg_resp).payload, 0 as *mut libc::c_int) as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_db_cap_report(
    mut msg_resp: *mut bmkt_msg_resp_t,
    mut resp: *mut bmkt_response_t,
) -> libc::c_int {
    let mut db_cap_resp: *mut bmkt_get_db_capacity_resp_t = &mut (*resp)
        .response
        .db_cap_resp;
    let mut offset: libc::c_int = 0 as libc::c_int;
    if ((*msg_resp).payload_len as libc::c_int) < 2 as libc::c_int
        || (*msg_resp).payload_len as libc::c_int > 4 as libc::c_int
    {
        return 112 as libc::c_int;
    }
    (*db_cap_resp).total = extract8((*msg_resp).payload, &mut offset);
    (*db_cap_resp).empty = extract8((*msg_resp).payload, &mut offset);
    if (*msg_resp).payload_len as libc::c_int == 4 as libc::c_int {
        (*db_cap_resp).bad_slots = extract8((*msg_resp).payload, &mut offset);
        (*db_cap_resp).corrupt_templates = extract8((*msg_resp).payload, &mut offset);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_get_enrolled_fingers_report(
    mut msg_resp: *mut bmkt_msg_resp_t,
    mut resp: *mut bmkt_response_t,
) -> libc::c_int {
    let mut offset: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    if ((*msg_resp).payload_len as libc::c_int) < 2 as libc::c_int {
        return 112 as libc::c_int;
    }
    let mut num_fingers: libc::c_int = (*msg_resp).payload_len as libc::c_int
        / 2 as libc::c_int;
    let mut get_enrolled_fingers_resp: *mut bmkt_enrolled_fingers_resp_t = &mut (*resp)
        .response
        .enrolled_fingers_resp;
    i = 0 as libc::c_int;
    while i < num_fingers {
        (*get_enrolled_fingers_resp)
            .fingers[i as usize]
            .finger_id = extract8((*msg_resp).payload, &mut offset);
        (*get_enrolled_fingers_resp)
            .fingers[i as usize]
            .template_status = extract8((*msg_resp).payload, &mut offset);
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_get_enrolled_users_report(
    mut msg_resp: *mut bmkt_msg_resp_t,
    mut resp: *mut bmkt_response_t,
) -> libc::c_int {
    let mut offset: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    if ((*msg_resp).payload_len as libc::c_int) < 2 as libc::c_int {
        return 112 as libc::c_int;
    }
    let mut get_enroll_templates_resp: *mut bmkt_enroll_templates_resp_t = &mut (*resp)
        .response
        .enroll_templates_resp;
    (*get_enroll_templates_resp)
        .total_query_messages = extract8((*msg_resp).payload, &mut offset);
    (*get_enroll_templates_resp)
        .query_sequence = extract8((*msg_resp).payload, &mut offset);
    let mut n: libc::c_int = 0 as libc::c_int;
    n = 0 as libc::c_int;
    while n < 15 as libc::c_int {
        if offset >= (*msg_resp).payload_len as libc::c_int {
            break;
        }
        (*get_enroll_templates_resp)
            .templates[n as usize]
            .user_id_len = (extract8((*msg_resp).payload, &mut offset) as libc::c_int
            - 2 as libc::c_int) as uint8_t;
        if (*get_enroll_templates_resp).templates[n as usize].user_id_len as libc::c_int
            > 100 as libc::c_int
        {
            return 112 as libc::c_int;
        }
        (*get_enroll_templates_resp)
            .templates[n as usize]
            .template_status = extract8((*msg_resp).payload, &mut offset);
        (*get_enroll_templates_resp)
            .templates[n as usize]
            .finger_id = extract8((*msg_resp).payload, &mut offset);
        i = 0 as libc::c_int;
        while i
            < (*get_enroll_templates_resp).templates[n as usize].user_id_len
                as libc::c_int
        {
            (*get_enroll_templates_resp)
                .templates[n as usize]
                .user_id[i as usize] = extract8((*msg_resp).payload, &mut offset);
            i += 1;
        }
        (*get_enroll_templates_resp)
            .templates[n as usize]
            .user_id[i as usize] = '\0' as i32 as uint8_t;
        n += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_get_version_report(
    mut msg_resp: *mut bmkt_msg_resp_t,
    mut resp: *mut bmkt_response_t,
) -> libc::c_int {
    let mut get_version_resp: *mut bmkt_get_version_resp_t = &mut (*resp)
        .response
        .get_version_resp;
    let mut offset: libc::c_int = 0 as libc::c_int;
    if (*msg_resp).payload_len as libc::c_int != 15 as libc::c_int {
        return 112 as libc::c_int;
    }
    memcpy(
        ((*get_version_resp).part).as_mut_ptr() as *mut libc::c_void,
        (*msg_resp).payload as *const libc::c_void,
        10 as libc::c_int as libc::c_ulong,
    );
    offset += 10 as libc::c_int;
    (*get_version_resp).year = extract8((*msg_resp).payload, &mut offset);
    (*get_version_resp).week = extract8((*msg_resp).payload, &mut offset);
    (*get_version_resp).patch = extract8((*msg_resp).payload, &mut offset);
    memcpy(
        ((*get_version_resp).supplier_id).as_mut_ptr() as *mut libc::c_void,
        ((*msg_resp).payload).offset(offset as isize) as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bmkt_compose_message(
    mut cmd: *mut uint8_t,
    mut cmd_len: *mut libc::c_int,
    mut msg_id: uint8_t,
    mut seq_num: uint8_t,
    mut payload_size: uint8_t,
    mut payload: *const uint8_t,
) -> libc::c_int {
    let mut message_len: libc::c_int = 4 as libc::c_int + payload_size as libc::c_int;
    if *cmd_len < message_len {
        return 104 as libc::c_int;
    }
    *cmd.offset(0 as libc::c_int as isize) = 0xfe as libc::c_int as uint8_t;
    *cmd.offset(1 as libc::c_int as isize) = seq_num;
    *cmd.offset(2 as libc::c_int as isize) = msg_id;
    *cmd.offset(3 as libc::c_int as isize) = payload_size;
    memcpy(
        &mut *cmd.offset(4 as libc::c_int as isize) as *mut uint8_t as *mut libc::c_void,
        payload as *const libc::c_void,
        payload_size as libc::c_ulong,
    );
    *cmd_len = message_len;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bmkt_parse_message_header(
    mut resp_buf: *mut uint8_t,
    mut resp_len: libc::c_int,
    mut msg_resp: *mut bmkt_msg_resp_t,
) -> libc::c_int {
    if *resp_buf.offset(0 as libc::c_int as isize) as libc::c_int != 0xfe as libc::c_int
    {
        return 110 as libc::c_int;
    }
    (*msg_resp).seq_num = *resp_buf.offset(1 as libc::c_int as isize);
    (*msg_resp).msg_id = *resp_buf.offset(2 as libc::c_int as isize);
    (*msg_resp).payload_len = *resp_buf.offset(3 as libc::c_int as isize);
    if (*msg_resp).payload_len as libc::c_int > 0 as libc::c_int {
        (*msg_resp)
            .payload = &mut *resp_buf.offset(4 as libc::c_int as isize) as *mut uint8_t;
    } else {
        (*msg_resp).payload = 0 as *mut uint8_t;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bmkt_parse_message_payload(
    mut msg_resp: *mut bmkt_msg_resp_t,
    mut resp: *mut bmkt_response_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    memset(
        resp as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<bmkt_response_t>() as libc::c_ulong,
    );
    (*resp).response_id = (*msg_resp).msg_id as libc::c_int;
    match (*msg_resp).msg_id as libc::c_int {
        2 | 9 | 18 | 34 | 50 | 53 | 67 | 88 | 99 | 103 | 121 | 130 | 133 | 147 | 163
        | 179 | 195 | 198 | 211 | 229 => {
            ret = parse_error_response(msg_resp, resp);
            (*resp).complete = 1 as libc::c_int;
        }
        19 => {
            ret = parse_init_ok(msg_resp, resp);
            (*resp).complete = 1 as libc::c_int;
        }
        66 | 134 | 131 => {
            (*resp).result = 0 as libc::c_int;
            (*resp).complete = 1 as libc::c_int;
        }
        35 => {
            ret = parse_fps_mode_report(msg_resp, resp);
            (*resp).complete = 1 as libc::c_int;
        }
        54 | 51 => {
            ret = parse_security_level_report(msg_resp, resp);
            (*resp).complete = 1 as libc::c_int;
        }
        135 => {
            ret = parse_del_all_users_progress_report(msg_resp, resp);
        }
        96 => {
            (*resp).result = 0 as libc::c_int;
        }
        84 => {
            (*resp).result = 0 as libc::c_int;
        }
        85 => {
            ret = parse_enroll_report(msg_resp, resp);
        }
        89 => {
            (*resp).complete = 1 as libc::c_int;
            ret = parse_enroll_ok(msg_resp, resp);
        }
        100 | 104 => {
            ret = parse_auth_ok(msg_resp, resp);
            (*resp).complete = 1 as libc::c_int;
        }
        119 => {
            ret = parse_get_enrolled_fingers_report(msg_resp, resp);
            (*resp).complete = 1 as libc::c_int;
        }
        120 => {
            (*resp).complete = 1 as libc::c_int;
            ret = parse_db_cap_report(msg_resp, resp);
        }
        117 => {
            ret = parse_get_enrolled_users_report(msg_resp, resp);
        }
        118 => {
            (*resp).complete = 1 as libc::c_int;
        }
        178 => {
            ret = parse_get_version_report(msg_resp, resp);
            (*resp).complete = 1 as libc::c_int;
        }
        162 => {
            (*resp).complete = 1 as libc::c_int;
        }
        _ => {}
    }
    return ret;
}
