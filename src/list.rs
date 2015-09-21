// exports from gphoto2-list.h

use ::libc::{c_void,c_int,c_char};

#[repr(C)]
pub struct CameraList {
    __private: c_void
}

extern "C" {
    pub fn gp_list_new(list: *mut *mut CameraList) -> c_int;
    pub fn gp_list_ref(list: *mut CameraList) -> c_int;
    pub fn gp_list_unref(list: *mut CameraList) -> c_int;
    pub fn gp_list_free(list: *mut CameraList) -> c_int;
    pub fn gp_list_count(list: *mut CameraList) -> c_int;
    pub fn gp_list_append(list: *mut CameraList, name: *const c_char, value: *const c_char) -> c_int;
    pub fn gp_list_reset(list: *mut CameraList) -> c_int;
    pub fn gp_list_sort(list: *mut CameraList) -> c_int;
    pub fn gp_list_find_by_name(list: *mut CameraList, index: *mut c_int, name: *const c_char) -> c_int;
    pub fn gp_list_get_name(list: *mut CameraList, index: c_int, name: *mut *const c_char) -> c_int;
    pub fn gp_list_get_value(list: *mut CameraList, index: c_int, value: *mut *const c_char) -> c_int;
    pub fn gp_list_set_name(list: *mut CameraList, index: c_int, name: *const c_char) -> c_int;
    pub fn gp_list_set_value(list: *mut CameraList, index: c_int, value: *const c_char) -> c_int;
    pub fn gp_list_populate(list: *mut CameraList, format: *const c_char, count: c_int) -> c_int;
}
