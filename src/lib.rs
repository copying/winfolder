//! This crate provides access to Windows APIs for querying the location of standard
//! [standard Windows folders](https://msdn.microsoft.com/en-us/library/windows/desktop/bb776911.aspx)
//! on the current system.
//!
//! # Example
//!
//! ```
//! extern crate winfolder;
//!
//! use winfolder::Folder;
//!
//! # fn main() {
//! # let _ =
//! Folder::ProgramFilesX86.path()
//! # ;
//! # }
//! ```

#![cfg(windows)]

extern crate winapi;
extern crate shell32;
extern crate ole32;

#[macro_use]
extern crate guid;

pub mod id;
mod folder;

pub use folder::Folder;

use std::ptr::null_mut;
use std::mem;
use std::ffi::OsString;
use std::slice;
use std::os::windows::ffi::OsStringExt;
use std::path::{Path, PathBuf};

use winapi::winnt::PWSTR;
use winapi::minwindef::MAX_PATH;
use shell32::SHGetKnownFolderPath;
use ole32::CoTaskMemFree;

/// Construct an `OsString` from a pointer to a wide-character string.
/// This is marked as unsafe because it can only be safely used on a
/// string that is known to come from a trusted API.
unsafe fn os_string_from_trusted_api(mut p: PWSTR) -> OsString {
    let mut s: OsString = OsString::with_capacity(MAX_PATH + 1);
    while *p != 0 {
        s.push(&OsString::from_wide(slice::from_raw_parts(p, 1)));
        p = p.offset(1);
    }
    s
}

/// Returns the path for a Windows
/// [known folder](https://msdn.microsoft.com/en-us/library/windows/desktop/bb776911.aspx)
/// based on that folder's GUID. Some standard known folder GUIDs can be found in
/// the `winfolder::id` module.
///
/// If the GUID does not represent a standard folder, this function
/// produces `None`.
///
/// This function provides the functionality of the standard Windows
/// [SHGetKnownFolderPath](https://msdn.microsoft.com/en-us/library/windows/desktop/bb762188.aspx)
/// API.
pub fn known_path(guid: &guid::GUID) -> Option<PathBuf> {
    let string: OsString;
    unsafe {
        let mut path: PWSTR = null_mut();
        if SHGetKnownFolderPath(guid, 0, null_mut(), mem::transmute(&mut path)) != 0 {
            return None;
        }
        string = os_string_from_trusted_api(path);
        CoTaskMemFree(mem::transmute(path));
    }
    Some(Path::new(&string).to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::{known_path, Folder};
    use super::id;
    use std::path::Path;

    #[test]
    fn it_works() {
        assert_eq!(known_path(&id::PROGRAM_FILES_X86), Some(Path::new(r"C:\Program Files (x86)").to_path_buf()));
        assert_eq!(Folder::ProgramFilesX86.path(), Path::new(r"C:\Program Files (x86)").to_path_buf());
    }
}
