// exports from gphoto2-result.h and gphoto2-port-result.h

use libc::{c_int, c_char};

// from gphoto2-port-result.h
pub const GP_OK:                         c_int = 0;
pub const GP_ERROR:                      c_int = -1;
pub const GP_ERROR_BAD_PARAMETERS:       c_int = -2;
pub const GP_ERROR_NO_MEMORY:            c_int = -3;
pub const GP_ERROR_LIBRARY:              c_int = -4;
pub const GP_ERROR_UNKNOWN_PORT:         c_int = -5;
pub const GP_ERROR_NOT_SUPPORTED:        c_int = -6;
pub const GP_ERROR_IO:                   c_int = -7;
pub const GP_ERROR_FIXED_LIMIT_EXCEEDED: c_int = -8;
pub const GP_ERROR_TIMEOUT:              c_int = -10;
pub const GP_ERROR_IO_SUPPORTED_SERIAL:  c_int = -20;
pub const GP_ERROR_IO_SUPPORTED_USB:     c_int = -21;
pub const GP_ERROR_IO_INIT:              c_int = -31;
pub const GP_ERROR_IO_READ:              c_int = -34;
pub const GP_ERROR_IO_WRITE:             c_int = -35;
pub const GP_ERROR_IO_UPDATE:            c_int = -37;
pub const GP_ERROR_IO_SERIAL_SPEED:      c_int = -41;
pub const GP_ERROR_IO_USB_CLEAR_HALT:    c_int = -51;
pub const GP_ERROR_IO_USB_FIND:          c_int = -52;
pub const GP_ERROR_IO_USB_CLAIM:         c_int = -53;
pub const GP_ERROR_IO_LOCK:              c_int = -60;
pub const GP_ERROR_HAL:                  c_int = -70;

// from  gphoto2-result.h
pub const GP_ERROR_CORRUPTED_DATA:       c_int = -102;
pub const GP_ERROR_FILE_EXISTS:          c_int = -103;
pub const GP_ERROR_MODEL_NOT_FOUND:      c_int = -105;
pub const GP_ERROR_DIRECTORY_NOT_FOUND:  c_int = -107;
pub const GP_ERROR_FILE_NOT_FOUND:       c_int = -108;
pub const GP_ERROR_DIRECTORY_EXISTS:     c_int = -109;
pub const GP_ERROR_CAMERA_BUSY:          c_int = -110;
pub const GP_ERROR_PATH_NOT_ABSOLUTE:    c_int = -111;
pub const GP_ERROR_CANCEL:               c_int = -112;
pub const GP_ERROR_CAMERA_ERROR:         c_int = -113;
pub const GP_ERROR_OS_FAILURE:           c_int = -114;
pub const GP_ERROR_NO_SPACE:             c_int = -115;

extern "C" {
    pub fn gp_result_as_string(result: c_int) -> *const c_char;
}
