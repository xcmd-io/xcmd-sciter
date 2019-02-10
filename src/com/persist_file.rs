use winapi::shared::guiddef::CLSID;
use winapi::shared::minwindef::{BOOL, DWORD};
use winapi::shared::wtypesbase::{LPCOLESTR, LPOLESTR};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::winnt::{HRESULT, LPCWSTR};

RIDL! {#[uuid(0x0000_010c, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IPersist(IPersistVtbl): IUnknown(IUnknownVtbl) {
	fn GetClassID(
		pClassID: *mut CLSID,
	) -> HRESULT,
}}

RIDL! {#[uuid(0x0000_010b, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IPersistFile(IPersistFileVtbl): IPersist(IPersistVtbl) {
	fn IsDirty(
	) -> HRESULT,

	fn Load(
		pszFileName: LPCOLESTR,
		dwMode: DWORD,
	) -> HRESULT,

	fn Save(
		wszLinkFile: LPCWSTR,
		fRemember: BOOL,
	) -> HRESULT,

	fn SaveCompleted(
		pszFileName: LPCOLESTR,
	) -> HRESULT,

	fn GetCurFile(
		ppszFileName: LPOLESTR,
	) -> HRESULT,
}}
