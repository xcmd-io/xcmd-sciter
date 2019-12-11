use winapi::shared::guiddef::CLSID;
use winapi::shared::minwindef::{BOOL, DWORD};
use winapi::shared::wtypesbase::{LPCOLESTR, LPOLESTR};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::winnt::{HRESULT, LPCWSTR};
use winapi::RIDL;

RIDL! {#[uuid(0x0000_010c, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IPersist(IPersistVtbl): IUnknown(IUnknownVtbl) {
	fn get_class_id(
		p_class_id: *mut CLSID,
	) -> HRESULT,
}}

RIDL! {#[uuid(0x0000_010b, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IPersistFile(IPersistFileVtbl): IPersist(IPersistVtbl) {
	fn is_dirty(
	) -> HRESULT,

	fn load(
		psz_file_name: LPCOLESTR,
		dw_mode: DWORD,
	) -> HRESULT,

	fn save(
		wsz_link_file: LPCWSTR,
		f_remember: BOOL,
	) -> HRESULT,

	fn save_completed(
		psz_file_name: LPCOLESTR,
	) -> HRESULT,

	fn get_cur_file(
		ppsz_file_name: LPOLESTR,
	) -> HRESULT,
}}
