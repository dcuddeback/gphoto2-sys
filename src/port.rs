// exports from gphoto2/gphoto2-port.h

use libc::{c_void, c_char, c_int};

#[repr(C)]
pub struct GPPortInfoList {
    __private: c_void
}

#[repr(C)]
pub struct _GPPortInfo {
    __private: c_void
}

pub type GPPortInfo = *mut _GPPortInfo;

pub type GPPortType = c_int;
pub const GP_PORT_NONE:            GPPortType = 0;
pub const GP_PORT_SERIAL:          GPPortType = 1 << 0;
pub const GP_PORT_USB:             GPPortType = 1 << 2;
pub const GP_PORT_DISK:            GPPortType = 1 << 3;
pub const GP_PORT_PTPIP:           GPPortType = 1 << 4;
pub const GP_PORT_USB_DISK_DIRECT: GPPortType = 1 << 5;
pub const GP_PORT_USB_SCSI:        GPPortType = 1 << 6;

extern "C" {
    pub fn gp_port_info_new(info: *mut GPPortInfo) -> c_int;
    pub fn gp_port_info_get_name(info: GPPortInfo, name: *mut *const c_char) -> c_int;
    pub fn gp_port_info_set_name(info: GPPortInfo, name: *const c_char) -> c_int;
    pub fn gp_port_info_get_path(info: GPPortInfo, path: *mut *const c_char) -> c_int;
    pub fn gp_port_info_set_path(info: GPPortInfo, path: *const c_char) -> c_int;
    pub fn gp_port_info_get_type(info: GPPortInfo, port_type: *mut GPPortType) -> c_int;
    pub fn gp_port_info_set_type(info: GPPortInfo, port_type: GPPortType) -> c_int;
    pub fn gp_port_info_get_library_filename(info: GPPortInfo, lib: *mut *const c_char) -> c_int;
    pub fn gp_port_info_set_library_filename(info: GPPortInfo, liv: *const c_char) -> c_int;
    pub fn gp_port_info_list_new(list: *mut *const GPPortInfoList) -> c_int;
    pub fn gp_port_info_list_free(list: *const GPPortInfoList) -> c_int;
    pub fn gp_port_info_list_append(list: *const GPPortInfoList, info: GPPortInfo) -> c_int;
    pub fn gp_port_info_list_load(list: *mut GPPortInfoList) -> c_int;
    pub fn gp_port_info_list_count(list: *const GPPortInfoList) -> c_int;
    pub fn gp_port_info_list_lookup_path(list: *const GPPortInfoList, path: *const c_char) -> c_int;
    pub fn gp_port_info_list_lookup_name(list: *const GPPortInfoList, name: *const c_char) -> c_int;
    pub fn gp_port_info_list_get_info(list: *const GPPortInfoList, n: c_int, info: *mut GPPortInfo) -> c_int;
    pub fn gp_port_message_codeset(_: *const c_char) -> *const c_char;
}
