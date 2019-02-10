use std::ops::Deref;
use std::ptr::{self, NonNull};
use winapi::shared::guiddef::REFCLSID;
use winapi::um::combaseapi::{CoCreateInstance, CLSCTX_ALL};
use winapi::um::unknwnbase::IUnknown;
use winapi::Interface;

#[repr(transparent)]
pub struct ComPtr<T>(NonNull<T>)
where
	T: Interface;

impl<T> ComPtr<T>
where
	T: Interface,
{
	pub unsafe fn from_raw(ptr: *mut T) -> ComPtr<T> {
		ComPtr(NonNull::new(ptr).expect("ptr should not be null"))
	}

	pub fn create_instance(rclsid: REFCLSID) -> Result<ComPtr<T>, i32> {
		let mut obj = ptr::null_mut();
		let hr = unsafe {
			CoCreateInstance(
				rclsid,
				ptr::null_mut(),
				CLSCTX_ALL,
				&T::uuidof(),
				&mut obj as *mut *mut T as *mut _,
			)
		};
		if hr < 0 {
			return Err(hr);
		}
		Ok(unsafe { ComPtr::from_raw(obj) })
	}

	fn as_unknown(&self) -> &IUnknown {
		unsafe { &*(self.as_raw() as *mut IUnknown) }
	}

	pub fn query_interface<U>(&self) -> Result<ComPtr<U>, i32>
	where
		U: Interface,
	{
		let mut obj = ptr::null_mut();
		let err = unsafe { self.as_unknown().QueryInterface(&U::uuidof(), &mut obj) };
		if err < 0 {
			return Err(err);
		}
		Ok(unsafe { ComPtr::from_raw(obj as *mut U) })
	}

	pub fn as_raw(&self) -> *mut T {
		self.0.as_ptr()
	}
}

impl<T> Deref for ComPtr<T>
where
	T: Interface,
{
	type Target = T;
	fn deref(&self) -> &T {
		unsafe { &*self.as_raw() }
	}
}

impl<T> Clone for ComPtr<T>
where
	T: Interface,
{
	fn clone(&self) -> Self {
		unsafe {
			self.as_unknown().AddRef();
			ComPtr::from_raw(self.as_raw())
		}
	}
}

impl<T> Drop for ComPtr<T>
where
	T: Interface,
{
	fn drop(&mut self) {
		unsafe {
			self.as_unknown().Release();
		}
	}
}

impl<T> PartialEq<ComPtr<T>> for ComPtr<T>
where
	T: Interface,
{
	fn eq(&self, other: &ComPtr<T>) -> bool {
		self.0 == other.0
	}
}
