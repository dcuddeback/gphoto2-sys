// exports from gphoto2-abilities-list.h

use ::libc::{c_int,c_char};

use ::context::GPContext;
use ::list::CameraList;
use ::port::{GPPortInfoList,GPPortType};

#[repr(C)]
pub struct CameraAbilitiesList {
    pub count: c_int,
    pub abilities: *mut CameraAbilities,
}

#[repr(C)]
pub struct CameraAbilities {
    pub model: [c_char; 128],
    pub status: CameraDriverStatus,
    pub port: GPPortType,
    pub speed: [c_int; 64],
    pub operations: CameraOperation,
    pub file_operations: CameraFileOperation,
    pub folder_operations: CameraFolderOperation,
    pub usb_vendor: c_int,
    pub usb_product: c_int,
    pub usb_class: c_int,
    pub usb_subclass: c_int,
    pub usb_protocol: c_int,
    pub library: [c_char; 1024],
    pub id: [c_char; 1024],
    pub device_type: GphotoDeviceType,
    reserved2: c_int,
    reserved3: c_int,
    reserved4: c_int,
    reserved5: c_int,
    reserved6: c_int,
    reserved7: c_int,
    reserved8: c_int,
}

#[repr(C)]
pub enum CameraDriverStatus {
    GP_DRIVER_STATUS_PRODUCTION   = 0,
    GP_DRIVER_STATUS_TESTING      = 1,
    GP_DRIVER_STATUS_EXPERIMENTAL = 2,
    GP_DRIVER_STATUS_DEPRECATED   = 3,
}

pub const GP_DRIVER_STATUS_PRODUCTION:   CameraDriverStatus = CameraDriverStatus::GP_DRIVER_STATUS_PRODUCTION;
pub const GP_DRIVER_STATUS_TESTING:      CameraDriverStatus = CameraDriverStatus::GP_DRIVER_STATUS_TESTING;
pub const GP_DRIVER_STATUS_EXPERIMENTAL: CameraDriverStatus = CameraDriverStatus::GP_DRIVER_STATUS_EXPERIMENTAL;
pub const GP_DRIVER_STATUS_DEPRECATED:   CameraDriverStatus = CameraDriverStatus::GP_DRIVER_STATUS_DEPRECATED;


#[repr(C)]
pub enum GphotoDeviceType {
    GP_DEVICE_STILL_CAMERA = 0,
    GP_DEVICE_AUDIO_PLAYER = 1,
}

pub const GP_DEVICE_STILL_CAMERA: GphotoDeviceType = GphotoDeviceType::GP_DEVICE_STILL_CAMERA;
pub const GP_DEVICE_AUDIO_PLAYER: GphotoDeviceType = GphotoDeviceType::GP_DEVICE_AUDIO_PLAYER;

pub type CameraOperation = c_int;
pub const GP_OPERATION_NONE:            CameraOperation = 0;
pub const GP_OPERATION_CAPTURE_IMAGE:   CameraOperation = 1 << 0;
pub const GP_OPERATION_CAPTURE_VIDEO:   CameraOperation = 1 << 1;
pub const GP_OPERATION_CAPTURE_AUDIO:   CameraOperation = 1 << 2;
pub const GP_OPERATION_CAPTURE_PREVIEW: CameraOperation = 1 << 3;
pub const GP_OPERATION_CONFIG:          CameraOperation = 1 << 4;
pub const GP_OPERATION_TRIGGER_CAPTURE: CameraOperation = 1 << 5;

pub type CameraFileOperation = c_int;
pub const GP_FILE_OPERATION_NONE:    CameraFileOperation = 0;
pub const GP_FILE_OPERATION_DELETE:  CameraFileOperation = 1 << 1;
pub const GP_FILE_OPERATION_PREVIEW: CameraFileOperation = 1 << 3;
pub const GP_FILE_OPERATION_RAW:     CameraFileOperation = 1 << 4;
pub const GP_FILE_OPERATION_AUDIO:   CameraFileOperation = 1 << 5;
pub const GP_FILE_OPERATION_EXIF:    CameraFileOperation = 1 << 6;

pub type CameraFolderOperation = c_int;
pub const GP_FOLDER_OPERATION_NONE:       CameraFolderOperation = 0;
pub const GP_FOLDER_OPERATION_DELETE_ALL: CameraFolderOperation = 1 << 0;
pub const GP_FOLDER_OPERATION_PUT_FILE:   CameraFolderOperation = 1 << 1;
pub const GP_FOLDER_OPERATION_MAKE_DIR:   CameraFolderOperation = 1 << 2;
pub const GP_FOLDER_OPERATION_REMOVE_DIR: CameraFolderOperation = 1 << 3;

extern "C" {
    pub fn gp_abilities_list_new(list: *mut *mut CameraAbilitiesList) -> c_int;
    pub fn gp_abilities_list_free(list: *mut CameraAbilitiesList) -> c_int;
    pub fn gp_abilities_list_load(list: *mut CameraAbilitiesList, context: *mut GPContext) -> c_int;
    pub fn gp_abilities_list_load_dir(list: *mut CameraAbilitiesList, dir: *const c_char, context: *mut GPContext) -> c_int;
    pub fn gp_abilities_list_reset(list: *mut CameraAbilitiesList) -> c_int;
    pub fn gp_abilities_list_detect(list: *mut CameraAbilitiesList, info_list: *mut GPPortInfoList, l: *mut CameraList, context: *mut GPContext) -> c_int;
    pub fn gp_abilities_list_append(list: *mut CameraAbilitiesList, abilities: CameraAbilities) -> c_int;
    pub fn gp_abilities_list_count(list: *mut CameraAbilitiesList) -> c_int;
    pub fn gp_abilities_list_lookup_model(list: *mut CameraAbilitiesList, model: *const c_char) -> c_int;
    pub fn gp_abilities_list_get_abilities(list: *mut CameraAbilitiesList, index: c_int, abilities: *mut CameraAbilities) -> c_int;
    pub fn gp_message_codeset(_: *const c_char) -> *const c_char;
}
