mod com_library;
mod com_ptr;
mod persist_file;
mod shell_link;
mod wchar;

pub use self::com_library::ComLibrary;
pub use self::com_ptr::ComPtr;
pub use self::persist_file::IPersistFile;
pub use self::shell_link::{CLSID_ShellLink, IShellLinkW, SLGP_SHORTPATH};
pub use self::wchar::{from_wchar, to_wchar};

#[macro_export]
macro_rules! invoke (
	($e: expr, $f: ident($($params:tt)*)) => {
		{
			let hr = unsafe { $e.$f($($params)*) };
			if hr < 0 { Err(hr) } else { Ok(()) }
		}
	};
);
