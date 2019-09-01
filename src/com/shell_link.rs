use sciter::types::_HWINDOW;
use winapi::shared::minwindef::{DWORD, WORD};
use winapi::um::minwinbase::WIN32_FIND_DATAW;
use winapi::um::shtypes::PIDLIST_ABSOLUTE;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::winnt::{HRESULT, LPCWSTR, LPWSTR};

pub const SLGP_SHORTPATH: DWORD = 0x1;

DEFINE_GUID! {CLSID_SHELL_LINK, 0x0002_1401, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}

RIDL! {#[uuid(0x0002_14f9, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IShellLinkW(IShellLinkWVtbl): IUnknown(IUnknownVtbl) {
	fn get_path(
		psz_file: LPWSTR,
		cch: i32,
		pfd: *mut WIN32_FIND_DATAW,
		f_flags: DWORD,
	) -> HRESULT,

	fn get_id_list(
		ppidl: *mut PIDLIST_ABSOLUTE,
	) -> HRESULT,

	fn set_id_list(
		pidl: PIDLIST_ABSOLUTE,
	) -> HRESULT,

	fn get_description(
		psz_name: LPWSTR,
		cch: i32,
	) -> HRESULT,

	fn set_description(
		psz_name: LPCWSTR,
	) -> HRESULT,

	fn get_working_directory(
		psz_dir: LPWSTR,
		cch: i32,
	) -> HRESULT,

	fn set_working_directory(
		psz_dir: LPCWSTR,
	) -> HRESULT,

	fn get_arguments(
		psz_args: LPWSTR,
		cch: i32,
	) -> HRESULT,

	fn set_arguments(
		psz_args: LPCWSTR,
	) -> HRESULT,

	fn get_hotkey(
		pw_hotkey: *mut WORD,
		cch: i32,
	) -> HRESULT,

	fn set_hotkey(
		w_hotkey: WORD,
	) -> HRESULT,

	fn get_show_cmd(
		pi_show_cmd: *mut i32,
		cch: i32,
	) -> HRESULT,

	fn set_show_cmd(
		i_show_cmd: i32,
	) -> HRESULT,

	fn get_icon_location(
		psz_icon_path: LPWSTR,
		cch: i32,
		pi_icon: *mut i32,
	) -> HRESULT,

	fn set_icon_location(
		psz_icon_path: LPCWSTR,
		i_icon: i32,
	) -> HRESULT,

	fn set_relative_path(
		psz_path_rel: LPCWSTR,
		dw_reserved: DWORD,
	) -> HRESULT,

	fn resolve(
		hwnd: *const _HWINDOW,
		f_flags: DWORD,
	) -> HRESULT,

	fn set_path(
		psz_file: LPCWSTR,
	) -> HRESULT,
}}
