// exports from gphoto2-camera.h

use libc::{c_int,c_uint,c_char,c_void};

use abilities::CameraAbilities;
use context::GPContext;
use file::{CameraFile, CameraFileType};
use filesys::{CameraFileInfo, CameraStorageInformation};
use list::CameraList;
use port::GPPortInfo;
use widget::CameraWidget;

#[repr(C)]
pub struct Camera {
    __private: c_void
}

#[repr(C)]
pub struct CameraText {
    pub text: [c_char; 32*1024],
}

#[repr(C)]
pub struct CameraFilePath {
    pub name: [c_char; 128],
    pub folder: [c_char; 1024],
}

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
#[repr(C)]
pub enum CameraCaptureType {
    GP_CAPTURE_IMAGE = 0,
    GP_CAPTURE_MOVIE = 1,
    GP_CAPTURE_SOUND = 2,
}

pub const GP_CAPTURE_IMAGE: CameraCaptureType = CameraCaptureType::GP_CAPTURE_IMAGE;
pub const GP_CAPTURE_MOVIE: CameraCaptureType = CameraCaptureType::GP_CAPTURE_MOVIE;
pub const GP_CAPTURE_SOUND: CameraCaptureType = CameraCaptureType::GP_CAPTURE_SOUND;

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
#[repr(C)]
pub enum CameraEventType {
    GP_EVENT_UNKNOWN          = 0,
    GP_EVENT_TIMEOUT          = 1,
    GP_EVENT_FILE_ADDED       = 2,
    GP_EVENT_FOLDER_ADDED     = 3,
    GP_EVENT_CAPTURE_COMPLETE = 4,
    GP_EVENT_FILE_CHANGED     = 5,
}

pub const GP_EVENT_UNKNOWN:          CameraEventType = CameraEventType::GP_EVENT_UNKNOWN;
pub const GP_EVENT_TIMEOUT:          CameraEventType = CameraEventType::GP_EVENT_TIMEOUT;
pub const GP_EVENT_FILE_ADDED:       CameraEventType = CameraEventType::GP_EVENT_FILE_ADDED;
pub const GP_EVENT_FOLDER_ADDED:     CameraEventType = CameraEventType::GP_EVENT_FOLDER_ADDED;
pub const GP_EVENT_CAPTURE_COMPLETE: CameraEventType = CameraEventType::GP_EVENT_CAPTURE_COMPLETE;
pub const GP_EVENT_FILE_CHANGED:     CameraEventType = CameraEventType::GP_EVENT_FILE_CHANGED;

pub type CameraTimeoutFunc      = extern "C" fn (camera: *mut Camera, context: *mut GPContext) -> c_int;
pub type CameraTimeoutStartFunc = extern "C" fn (camera: *mut Camera, timeout: c_uint, func: CameraTimeoutFunc, data: *mut c_void) -> c_uint;
pub type CameraTimeoutStopFunc  = extern "C" fn (camera: *mut Camera, id: c_uint, data: *mut c_void);

extern "C" {
    pub fn gp_camera_new(camera: *mut *mut Camera) -> c_int;

    // preparing initialization
    pub fn gp_camera_set_abilities(camera: *mut Camera, abilities: CameraAbilities) -> c_int;
    pub fn gp_camera_get_abilities(camera: *mut Camera, abilities: *mut CameraAbilities) -> c_int;
    pub fn gp_camera_set_port_info(camera: *mut Camera, info: GPPortInfo) -> c_int;
    pub fn gp_camera_get_port_info(camera: *mut Camera, info: *mut GPPortInfo) -> c_int;

    // camera speed
    pub fn gp_camera_get_port_speed(camera: *mut Camera) -> c_int;
    pub fn gp_camera_set_port_speed(camera: *mut Camera, speed: c_int) -> c_int;

    // initialization
    pub fn gp_camera_autodetect(list: *mut CameraList, context: *mut GPContext) -> c_int;
    pub fn gp_camera_init(camera: *mut Camera, context: *mut GPContext) -> c_int;
    pub fn gp_camera_exit(camera: *mut Camera, context: *mut GPContext) -> c_int;

    // operations on cameras
    pub fn gp_camera_ref(camera: *mut Camera) -> c_int;
    pub fn gp_camera_unref(camera: *mut Camera) -> c_int;
    pub fn gp_camera_free(camera: *mut Camera) -> c_int;
    pub fn gp_camera_list_config(camera: *mut Camera, list: *mut CameraList, context: *mut GPContext) -> c_int;
    pub fn gp_camera_get_config(camera: *mut Camera, window: *mut *mut CameraWidget, context: *mut GPContext) -> c_int;
    pub fn gp_camera_set_config(camera: *mut Camera, window: *mut CameraWidget, context: *mut GPContext) -> c_int;
    pub fn gp_camera_get_single_config(camera: *mut Camera, name: *const c_char, widget: *mut *mut CameraWidget, context: *mut GPContext) -> c_int;
    pub fn gp_camera_set_single_config(camera: *mut Camera, name: *const c_char, widget: *mut CameraWidget, context: *mut GPContext) -> c_int;
    pub fn gp_camera_get_summary(camera: *mut Camera, summary: *mut CameraText, context: *mut GPContext) -> c_int;
    pub fn gp_camera_get_manual(camera: *mut Camera, manual: *mut CameraText, context: *mut GPContext) -> c_int;
    pub fn gp_camera_get_about(camera: *mut Camera, about: *mut CameraText, context: *mut GPContext) -> c_int;
    pub fn gp_camera_capture(camera: *mut Camera, capture_type: CameraCaptureType, path: *mut CameraFilePath, context: *mut GPContext) -> c_int;
    pub fn gp_camera_trigger_capture(camera: *mut Camera, context: *mut GPContext) -> c_int;
    pub fn gp_camera_capture_preview(camera: *mut Camera, file: *mut CameraFile, context: *mut GPContext) -> c_int;
    pub fn gp_camera_wait_for_event(camera: *mut Camera, timeout: c_int, eventtype: *mut CameraEventType, eventdata: *mut *mut c_void, context: *mut GPContext) -> c_int;
    pub fn gp_camera_get_storageinfo(camera: *mut Camera, sifs: *mut *mut CameraStorageInformation, nrofsifs: *mut c_int, context: *mut GPContext) -> c_int;

    // operations on folders
    pub fn gp_camera_folder_list_files(camera: *mut Camera, folder: *const c_char, list: *mut CameraList, context: *mut GPContext) -> c_int;
    pub fn gp_camera_folder_list_folders(camera: *mut Camera, folder: *const c_char, list: *mut CameraList, context: *mut GPContext) -> c_int;
    pub fn gp_camera_folder_delete_all(camera: *mut Camera, folder: *const c_char, context: *mut GPContext) -> c_int;
    pub fn gp_camera_folder_put_file(camera: *mut Camera, folder: *const c_char, filename: *const c_char, file_type: CameraFileType, file: *mut CameraFile, context: *mut GPContext) -> c_int;
    pub fn gp_camera_folder_make_dir(camera: *mut Camera, folder: *const c_char, name: *const c_char, context: *mut GPContext) -> c_int;
    pub fn gp_camera_folder_remove_dir(camera: *mut Camera, folder: *const c_char, name: *const c_char, context: *mut GPContext) -> c_int;

    // operations on files
    pub fn gp_camera_file_get_info(camera: *mut Camera, folder: *const c_char, file: *const c_char, info: *mut CameraFileInfo, contest: *mut GPContext) -> c_int;
    pub fn gp_camera_file_set_info(camera: *mut Camera, folder: *const c_char, file: *const c_char, info: CameraFileInfo, context: *mut GPContext) -> c_int;
    pub fn gp_camera_file_get(camera: *mut Camera, folder: *const c_char, file: *const c_char, file_type: CameraFileType, camera_file: *mut CameraFile, context: *mut GPContext) -> c_int;
    pub fn gp_camera_file_read(camera: *mut Camera, folder: *const c_char, file: *const c_char, file_type: CameraFileType, offset: u64, buf: *mut c_char, size: *mut u64, context: *mut GPContext) -> c_int;
    pub fn gp_camera_file_delete(camera: *mut Camera, folder: *const c_char, file: *const c_char, context: *mut GPContext) -> c_int;

    // keep-alive messages
    pub fn gp_camera_set_timeout_funcs(camera: *mut Camera, start_func: CameraTimeoutStartFunc, stop_func: CameraTimeoutStopFunc, data: *mut c_void);
    pub fn gp_camera_start_timeout(camera: *mut Camera, timeout: c_uint, func: CameraTimeoutFunc) -> c_int;
    pub fn gp_camera_stop_timeout(camera: *mut Camera, id: c_uint);
}
