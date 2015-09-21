// exports from gphoto2-version.h

use libc::c_char;

#[repr(C)]
pub enum GPVersionVerbosity {
    GP_VERSION_SHORT   = 0,
    GP_VERSION_VERBOSE = 1,
}

extern "C" {
    pub fn gp_library_version(verbose: GPVersionVerbosity) -> *const *const c_char;
}
