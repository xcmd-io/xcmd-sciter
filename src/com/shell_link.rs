use sciter::types::_HWINDOW;
use winapi::shared::minwindef::{DWORD, WORD};
use winapi::um::minwinbase::WIN32_FIND_DATAW;
use winapi::um::shtypes::PIDLIST_ABSOLUTE;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::winnt::{HRESULT, LPCWSTR, LPWSTR};

pub const SLGP_SHORTPATH: DWORD = 0x1;

DEFINE_GUID! {CLSID_ShellLink, 0x0002_1401, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}

RIDL! {#[uuid(0x0002_14f9, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IShellLinkW(IShellLinkWVtbl): IUnknown(IUnknownVtbl) {
	fn GetPath(
		pszFile: LPWSTR,
		cch: i32,
		pfd: *mut WIN32_FIND_DATAW,
		fFlags: DWORD,
	) -> HRESULT,

	fn GetIDList(
		ppidl: *mut PIDLIST_ABSOLUTE,
	) -> HRESULT,

	fn SetIDList(
		pidl: PIDLIST_ABSOLUTE,
	) -> HRESULT,

	fn GetDescription(
		pszName: LPWSTR,
		cch: i32,
	) -> HRESULT,

	fn SetDescription(
		pszName: LPCWSTR,
	) -> HRESULT,

	fn GetWorkingDirectory(
		pszDir: LPWSTR,
		cch: i32,
	) -> HRESULT,

	fn SetWorkingDirectory(
		pszDir: LPCWSTR,
	) -> HRESULT,

	fn GetArguments(
		pszArgs: LPWSTR,
		cch: i32,
	) -> HRESULT,

	fn SetArguments(
		pszArgs: LPCWSTR,
	) -> HRESULT,

	fn GetHotkey(
		pwHotkey: *mut WORD,
		cch: i32,
	) -> HRESULT,

	fn SetHotkey(
		wHotkey: WORD,
	) -> HRESULT,

	fn GetShowCmd(
		piShowCmd: *mut i32,
		cch: i32,
	) -> HRESULT,

	fn SetShowCmd(
		iShowCmd: i32,
	) -> HRESULT,

	fn GetIconLocation(
		pszIconPath: LPWSTR,
		cch: i32,
		piIcon: *mut i32,
	) -> HRESULT,

	fn SetIconLocation(
		pszIconPath: LPCWSTR,
		iIcon: i32,
	) -> HRESULT,

	fn SetRelativePath(
		pszPathRel: LPCWSTR,
		dwReserved: DWORD,
	) -> HRESULT,

	fn Resolve(
		hwnd: *const _HWINDOW,
		fFlags: DWORD,
	) -> HRESULT,

	fn SetPath(
		pszFile: LPCWSTR,
	) -> HRESULT,
}}
