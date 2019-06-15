// exports from gphoto2-setting.h

use libc::{c_int, c_char};

extern "C" {
    pub fn gp_setting_set(id: *const c_char, key: *const c_char, value: *const c_char) -> c_int;
    pub fn gp_setting_get(id: *const c_char, key: *const c_char, value: *const c_char) -> c_int;
}
