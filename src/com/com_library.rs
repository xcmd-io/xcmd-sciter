use std::ptr;
use winapi::um::combaseapi::CoUninitialize;
use winapi::um::objbase::CoInitialize;

pub struct ComLibrary;

impl ComLibrary {
	pub fn initialize() -> Result<ComLibrary, i32> {
		let hr = unsafe { CoInitialize(ptr::null_mut()) };
		if hr < 0 {
			unsafe { CoUninitialize() };
			return Err(hr);
		}
		Ok(ComLibrary)
	}
}

impl Drop for ComLibrary {
	fn drop(&mut self) {
		unsafe {
			CoUninitialize();
		}
	}
}
