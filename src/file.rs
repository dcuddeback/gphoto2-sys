use ::libc::{c_void,c_int,c_char,c_uchar,c_ulong,size_t,time_t};

// TODO: should be *const c_char
pub const GP_MIME_WAV:       &'static [u8] = b"audio/wav\0";
pub const GP_MIME_RAW:       &'static [u8] = b"image/x-raw\0";
pub const GP_MIME_PNG:       &'static [u8] = b"image/png\0";
pub const GP_MIME_PGM:       &'static [u8] = b"image/x-portable-graymap\0";
pub const GP_MIME_PPM:       &'static [u8] = b"image/x-portable-pixmap\0";
pub const GP_MIME_PNM:       &'static [u8] = b"image/x-portable-anymap\0";
pub const GP_MIME_JPEG:      &'static [u8] = b"image/jpeg\0";
pub const GP_MIME_TIFF:      &'static [u8] = b"image/tiff\0";
pub const GP_MIME_BMP:       &'static [u8] = b"image/bmp\0";
pub const GP_MIME_QUICKTIME: &'static [u8] = b"video/quicktime\0";
pub const GP_MIME_AVI:       &'static [u8] = b"video/x-msvideo\0";
pub const GP_MIME_CRW:       &'static [u8] = b"image/x-canon-raw\0";
pub const GP_MIME_CR2:       &'static [u8] = b"image/x-canon-cr2\0";
pub const GP_MIME_UNKNOWN:   &'static [u8] = b"application/octet-stream\0";
pub const GP_MIME_EXIF:      &'static [u8] = b"application/x-exif\0";
pub const GP_MIME_MP3:       &'static [u8] = b"audio/mpeg\0";
pub const GP_MIME_OGG:       &'static [u8] = b"application/ogg\0";
pub const GP_MIME_WMA:       &'static [u8] = b"audio/x-wma\0";
pub const GP_MIME_ASF:       &'static [u8] = b"audio/x-asf\0";
pub const GP_MIME_MPEG:      &'static [u8] = b"video/mpeg\0";
pub const GP_MIME_AVCHD:     &'static [u8] = b"video/mp2t\0";
pub const GP_MIME_RW2:       &'static [u8] = b"image/x-panasonic-raw2\0";
pub const GP_MIME_ARW:       &'static [u8] = b"image/x-sony-arw\0";

#[repr(C)]
pub struct CameraFileHandler {
    size: extern "C" fn (prv: *mut c_void, size: *mut u64) -> c_int,
    read: extern "C" fn (prv: *mut c_void, data: *mut c_uchar, len: *mut u64) -> c_int,
    write: extern "C" fn (prv: *mut c_void, data: *mut c_uchar, len: *mut u64) -> c_int,
}

#[repr(C)]
pub struct CameraFile {
    __private: c_void
}

#[repr(C)]
pub enum CameraFileType {
    GP_FILE_TYPE_PREVIEW  = 0,
    GP_FILE_TYPE_NORMAL   = 1,
    GP_FILE_TYPE_RAW      = 2,
    GP_FILE_TYPE_AUDIO    = 3,
    GP_FILE_TYPE_EXIF     = 4,
    GP_FILE_TYPE_METADATA = 5,
}

pub const GP_FILE_TYPE_PREVIEW:  CameraFileType = CameraFileType::GP_FILE_TYPE_PREVIEW;
pub const GP_FILE_TYPE_NORMAL:   CameraFileType = CameraFileType::GP_FILE_TYPE_NORMAL;
pub const GP_FILE_TYPE_RAW:      CameraFileType = CameraFileType::GP_FILE_TYPE_RAW;
pub const GP_FILE_TYPE_AUDIO:    CameraFileType = CameraFileType::GP_FILE_TYPE_AUDIO;
pub const GP_FILE_TYPE_EXIF:     CameraFileType = CameraFileType::GP_FILE_TYPE_EXIF;
pub const GP_FILE_TYPE_METADATA: CameraFileType = CameraFileType::GP_FILE_TYPE_METADATA;

#[repr(C)]
pub enum CameraFileAccessType {
    GP_FILE_ACCESSTYPE_MEMORY  = 0,
    GP_FILE_ACCESSTYPE_FD      = 1,
    GP_FILE_ACCESSTYPE_HANDLER = 2,
}

pub const GP_FILE_ACCESSTYPE_MEMORY:  CameraFileAccessType = CameraFileAccessType::GP_FILE_ACCESSTYPE_MEMORY;
pub const GP_FILE_ACCESSTYPE_FD:      CameraFileAccessType = CameraFileAccessType::GP_FILE_ACCESSTYPE_FD;
pub const GP_FILE_ACCESSTYPE_HANDLER: CameraFileAccessType = CameraFileAccessType::GP_FILE_ACCESSTYPE_HANDLER;

extern "C" {
    pub fn gp_file_new(file: *mut *mut CameraFile) -> c_int;
    pub fn gp_file_new_from_fd(file: *mut *mut CameraFile, fd: c_int) -> c_int;
    pub fn gp_file_new_from_handler(file: *mut *mut CameraFile, handler: *mut CameraFileHandler, prv: *mut c_void) -> c_int;
    pub fn gp_file_ref(file: *mut CameraFile) -> c_int;
    pub fn gp_file_unref(file: *mut CameraFile) -> c_int;
    pub fn gp_file_free(file: *mut CameraFile) -> c_int;
    pub fn gp_file_set_name(file: *mut CameraFile, name: *const c_char) -> c_int;
    pub fn gp_file_get_name(file: *mut CameraFile, name: *mut *const c_char) -> c_int;
    pub fn gp_file_set_mime_type(file: *mut CameraFile, mime_type: *const c_char) -> c_int;
    pub fn gp_file_get_mime_type(file: *mut CameraFile, mime_type: *mut *const c_char) -> c_int;
    pub fn gp_file_set_mtime(file: *mut CameraFile, mtime: time_t) -> c_int;
    pub fn gp_file_get_mtime(file: *mut CameraFile, mtime: *mut time_t) -> c_int;
    pub fn gp_file_detect_mime_type(file: *mut CameraFile) -> c_int;
    pub fn gp_file_adjust_name_for_mime_type(file: *mut CameraFile) -> c_int;
    pub fn gp_file_get_name_by_type(file: *mut CameraFile, basename: *const c_char, file_type: CameraFileType, newname: *mut *mut c_char) -> c_int;
    pub fn gp_file_set_data_and_size(file: *mut CameraFile, data: *mut c_char, size: c_ulong) -> c_int;
    pub fn gp_file_get_data_and_size(file: *mut CameraFile, data: *mut *const c_char, size: *mut c_ulong) -> c_int;
    pub fn gp_file_open(file: *mut CameraFile, filename: *const c_char) -> c_int;
    pub fn gp_file_save(file: *mut CameraFile, filename: *const c_char) -> c_int;
    pub fn gp_file_clean(file: *mut CameraFile) -> c_int;
    pub fn gp_file_copy(destination: *mut CameraFile, source: *mut CameraFile) -> c_int;
    pub fn gp_file_append(file: *mut CameraFile, data: *const c_char, size: c_ulong) -> c_int;
    pub fn gp_file_slurp(file: *mut CameraFile, data: *mut c_char, size: size_t, readlen: *mut size_t) -> c_int;
}
