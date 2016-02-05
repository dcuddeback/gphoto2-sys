// exports from gphoto2-filesys.h

use ::libc::{c_int,c_char,time_t};

#[repr(C)]
pub struct CameraStorageInformation {
    pub fields: CameraStorageInfoFields,
    pub basedir: [c_char; 256],
    pub label: [c_char; 256],
    pub description: [c_char; 256],
    pub storage_type: CameraStorageType,
    pub fstype: CameraStorageFilesystemType,
    pub access: CameraStorageAccessType,
    pub capacitykbytes: u64,
    pub freekbytes: u64,
    pub freeimages: u64,
}

pub type CameraStorageInfoFields = c_int;
pub const GP_STORAGEINFO_BASE:            CameraStorageInfoFields = 1 << 0;
pub const GP_STORAGEINFO_LABEL:           CameraStorageInfoFields = 1 << 1;
pub const GP_STORAGEINFO_DESCRIPTION:     CameraStorageInfoFields = 1 << 2;
pub const GP_STORAGEINFO_ACCESS:          CameraStorageInfoFields = 1 << 3;
pub const GP_STORAGEINFO_STORAGETYPE:     CameraStorageInfoFields = 1 << 4;
pub const GP_STORAGEINFO_FILESYSTEMTYPE:  CameraStorageInfoFields = 1 << 5;
pub const GP_STORAGEINFO_MAXCAPACITY:     CameraStorageInfoFields = 1 << 6;
pub const GP_STORAGEINFO_FREESPACEKBYTES: CameraStorageInfoFields = 1 << 7;
pub const GP_STORAGEINFO_FREESPACEIMAGES: CameraStorageInfoFields = 1 << 8;

#[repr(C)]
pub enum CameraStorageType {
    GP_STORAGEINFO_ST_UNKNOWN       = 0,
    GP_STORAGEINFO_ST_FIXED_ROM     = 1,
    GP_STORAGEINFO_ST_REMOVABLE_ROM = 2,
    GP_STORAGEINFO_ST_FIXED_RAM     = 3,
    GP_STORAGEINFO_ST_REMOVABLE_RAM = 4,
}

pub const GP_STORAGEINFO_ST_UNKNOWN:       CameraStorageType = CameraStorageType::GP_STORAGEINFO_ST_UNKNOWN;
pub const GP_STORAGEINFO_ST_FIXED_ROM:     CameraStorageType = CameraStorageType::GP_STORAGEINFO_ST_FIXED_ROM;
pub const GP_STORAGEINFO_ST_REMOVABLE_ROM: CameraStorageType = CameraStorageType::GP_STORAGEINFO_ST_REMOVABLE_ROM;
pub const GP_STORAGEINFO_ST_FIXED_RAM:     CameraStorageType = CameraStorageType::GP_STORAGEINFO_ST_FIXED_RAM;
pub const GP_STORAGEINFO_ST_REMOVABLE_RAM: CameraStorageType = CameraStorageType::GP_STORAGEINFO_ST_REMOVABLE_RAM;

#[repr(C)]
pub enum CameraStorageFilesystemType {
    GP_STORAGEINFO_FST_UNDEFINED           = 0,
    GP_STORAGEINFO_FST_GENERICFLAT         = 1,
    GP_STORAGEINFO_FST_GENERICHIERARCHICAL = 2,
    GP_STORAGEINFO_FST_DCF                 = 3,
}

pub const GP_STORAGEINFO_FST_UNDEFINED:           CameraStorageFilesystemType = CameraStorageFilesystemType::GP_STORAGEINFO_FST_UNDEFINED;
pub const GP_STORAGEINFO_FST_GENERICFLAT:         CameraStorageFilesystemType = CameraStorageFilesystemType::GP_STORAGEINFO_FST_GENERICFLAT;
pub const GP_STORAGEINFO_FST_GENERICHIERARCHICAL: CameraStorageFilesystemType = CameraStorageFilesystemType::GP_STORAGEINFO_FST_GENERICHIERARCHICAL;
pub const GP_STORAGEINFO_FST_DCF:                 CameraStorageFilesystemType = CameraStorageFilesystemType::GP_STORAGEINFO_FST_DCF;

#[repr(C)]
pub enum CameraStorageAccessType {
    GP_STORAGEINFO_AC_READWRITE            = 0,
    GP_STORAGEINFO_AC_READONLY             = 1,
    GP_STORAGEINFO_AC_READONLY_WITH_DELETE = 2,
}

pub const GP_STORAGEINFO_AC_READWRITE:            CameraStorageAccessType = CameraStorageAccessType::GP_STORAGEINFO_AC_READWRITE;
pub const GP_STORAGEINFO_AC_READONLY:             CameraStorageAccessType = CameraStorageAccessType::GP_STORAGEINFO_AC_READONLY;
pub const GP_STORAGEINFO_AC_READONLY_WITH_DELETE: CameraStorageAccessType = CameraStorageAccessType::GP_STORAGEINFO_AC_READONLY_WITH_DELETE;

#[repr(C)]
pub struct CameraFileInfo {
    pub preview: CameraFileInfoPreview,
    pub file: CameraFileInfoFile,
    pub audio: CameraFileInfoAudio,
}

#[repr(C)]
pub struct CameraFileInfoPreview {
    pub fields: CameraFileInfoFields,
    pub status: CameraFileStatus,
    pub size: u64,
    pub mime_type: [c_char; 64],
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
pub struct CameraFileInfoFile {
    pub fields: CameraFileInfoFields,
    pub status: CameraFileStatus,
    pub size: u64,
    pub mime_type: [c_char; 64],
    pub width: u32,
    pub height: u32,
    pub permissions: CameraFilePermissions,
    pub mtime: time_t,
}

#[repr(C)]
pub struct CameraFileInfoAudio {
    pub fields: CameraFileInfoFields,
    pub status: CameraFileStatus,
    pub size: u64,
    pub mime_type: [c_char; 64],
}

pub type CameraFileInfoFields = c_int;
pub const GP_FILE_INFO_NONE:        CameraFileInfoFields = 0;
pub const GP_FILE_INFO_TYPE:        CameraFileInfoFields = 1 << 0;
pub const GP_FILE_INFO_SIZE:        CameraFileInfoFields = 1 << 2;
pub const GP_FILE_INFO_WIDTH:       CameraFileInfoFields = 1 << 3;
pub const GP_FILE_INFO_HEIGHT:      CameraFileInfoFields = 1 << 4;
pub const GP_FILE_INFO_PERMISSIONS: CameraFileInfoFields = 1 << 5;
pub const GP_FILE_INFO_STATUS:      CameraFileInfoFields = 1 << 6;
pub const GP_FILE_INFO_MTIME:       CameraFileInfoFields = 1 << 7;
pub const GP_FILE_INFO_ALL:         CameraFileInfoFields = 0xFF;

pub type CameraFilePermissions = c_int;
pub const GP_FILE_PERM_NONE:   CameraFilePermissions = 0;
pub const GP_FILE_PERM_READ:   CameraFilePermissions = 1 << 0;
pub const GP_FILE_PERM_DELETE: CameraFilePermissions = 1 << 1;
pub const GP_FILE_PERM_ALL:    CameraFilePermissions = 0xFF;

#[repr(C)]
pub enum CameraFileStatus {
    GP_FILE_STATUS_NOT_DOWNLOADED = 0,
    GP_FILE_STATUS_DOWNLOADED     = 1,
}
