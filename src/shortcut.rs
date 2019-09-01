use com::{self, ComLibrary, ComPtr, IPersistFile, IShellLinkW, CLSID_SHELL_LINK, SLGP_SHORTPATH};
use sciter::types::_HWINDOW;
use std::path::{Path, PathBuf};
use std::ptr;
use winapi::shared::minwindef::{MAX_PATH, TRUE};
use winapi::um::coml2api::STGM_READ;

pub fn read_link(link: &Path, hwnd: *const _HWINDOW) -> PathBuf {
	let _library = ComLibrary::initialize().expect("COM library");

	let shell_link: ComPtr<IShellLinkW> =
		ComPtr::create_instance(&CLSID_SHELL_LINK).expect("IShellLinkW");
	let persist_file: ComPtr<IPersistFile> = shell_link.query_interface().expect("IPersistFile");

	invoke!(persist_file, load(com::to_wchar(link.as_ref()), STGM_READ)).expect("load link");
	invoke!(shell_link, resolve(hwnd, 0)).expect("link resolve");

	let mut path = vec![0u16; MAX_PATH];
	invoke!(
		shell_link,
		get_path(
			path.as_mut_ptr(),
			MAX_PATH as i32,
			ptr::null_mut(),
			SLGP_SHORTPATH
		)
	)
	.expect("get link path");

	let path_buf: PathBuf = From::from(com::from_wchar(&path));
	println!("link path: {:?}", &path_buf);
	path_buf
}

pub fn update_link(link: &Path, target: &Path) {
	let _library = ComLibrary::initialize().expect("COM library");

	let shell_link: ComPtr<IShellLinkW> =
		ComPtr::create_instance(&CLSID_SHELL_LINK).expect("IShellLinkW");

	println!("set path to {:?}", &target);
	invoke!(shell_link, set_path(com::to_wchar(target.as_ref()))).expect("set path");

	let persist_file: ComPtr<IPersistFile> = shell_link.query_interface().expect("IPersistFile");

	println!("save to {:?}", link);
	invoke!(persist_file, save(com::to_wchar(link.as_ref()), TRUE)).expect("save file");
}
