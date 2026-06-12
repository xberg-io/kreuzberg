use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;

use libheif_sys as lh;

use crate::{FileTypeResult, HeifError, HeifErrorCode, HeifErrorSubCode, Result};

#[inline]
#[allow(unsafe_code)]
pub(crate) fn cstr_to_str<'a>(c_str: *const c_char) -> Option<&'a str> {
    if c_str.is_null() {
        None
    } else {
        // SAFETY: c_str is non-null and points to a valid NUL-terminated C string.
        let res = unsafe { CStr::from_ptr(c_str).to_str() };
        res.ok()
    }
}

pub(crate) fn str_to_cstring(s: &str, name: &str) -> Result<CString> {
    CString::new(s).map_err(|e| HeifError {
        code: HeifErrorCode::UsageError,
        sub_code: HeifErrorSubCode::InvalidParameterValue,
        message: format!("Invalid value of '{}': {}", name, e),
    })
}

pub(crate) fn path_to_cstring(path: &Path) -> CString {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        CString::new(path.as_os_str().as_bytes()).unwrap_or_default()
    }

    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        let mut buf = Vec::new();
        buf.extend(
            path.as_os_str()
                .encode_wide()
                .chain(Some(0))
                .flat_map(|b| b.to_ne_bytes()),
        );
        CString::new(buf).unwrap_or_default()
    }
}

#[cfg(feature = "v1_18")]
pub(crate) fn get_non_null_ptr<T>(ptr: *mut T) -> Result<std::ptr::NonNull<T>> {
    std::ptr::NonNull::new(ptr).ok_or_else(|| HeifError {
        code: HeifErrorCode::MemoryAllocationError,
        sub_code: HeifErrorSubCode::Unspecified,
        message: "".to_string(),
    })
}

/// Check a file type by it first bytes.
/// Input data should be at least 12 bytes.
#[allow(unsafe_code)]
pub fn check_file_type(data: &[u8]) -> FileTypeResult {
    let res = unsafe { lh::heif_check_filetype(data.as_ptr(), data.len() as _) };
    FileTypeResult::n(res).unwrap_or(FileTypeResult::No)
}
