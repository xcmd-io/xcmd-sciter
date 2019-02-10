use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};

pub fn to_wchar(os_str: &OsStr) -> *const u16 {
	let v: Vec<u16> = os_str.encode_wide().chain(Some(0).into_iter()).collect();
	v.as_ptr()
}

pub fn from_wchar(vec: &[u16]) -> OsString {
	let slice = vec.split(|&v| v == 0).next().unwrap();
	OsString::from_wide(&slice)
}
