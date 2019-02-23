use std::error;
use std::fmt;
use std::io;
use std::rc::Rc;

#[derive(Debug)]
pub enum Error {
	IoError(io::Error),
}

impl error::Error for Error {
	fn description(&self) -> &str {
		"Error occurred."
	}

	fn cause(&self) -> Option<&error::Error> {
		None
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::IoError(ref e) => e.fmt(f),
			// _ => {
			// 	use std::error::Error;
			// 	f.write_str(self.description())
			// }
		}
	}
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error::IoError(err)
	}
}

pub struct File {
	pub field_names: Rc<Vec<String>>,
	pub fields: Vec<Value>,
}

pub enum Icon {
	Local(String),
	Shell(String),
}

pub enum Value {
	String {
		string: String,
	},
	Path {
		path: String,
		name: String,
		icon: Icon,
	},
	Size {
		bytes: u64,
	},
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Value::String { string } => write!(f, "{}", string),
			Value::Path { name, .. } => write!(f, "{}", name),
			Value::Size { bytes } => write!(f, "{}", &bytes.to_string()),
		}
	}
}

impl File {
	pub fn new(field_names: &Rc<Vec<String>>, fields: Vec<Value>) -> Self {
		File {
			field_names: field_names.clone(),
			fields,
		}
	}

	pub fn get_field_index(&self, name: &str) -> Option<usize> {
		self.field_names.iter().position(|x| x == name)
	}
}

pub trait Cursor<TFile, TError> {
	fn next() -> Result<TFile, TError>;
}

pub trait System {
	fn get_root(&mut self, field_names: &Rc<Vec<String>>) -> Result<File, Error>;
	fn get_file(&mut self, path: &str, field_names: &Rc<Vec<String>>) -> Result<File, Error>;
	fn get_filename(&mut self, path: &str) -> String;
	fn list_files(
		&mut self,
		parent_directory: &File,
		field_names: &Rc<Vec<String>>,
	) -> Result<Vec<File>, Error>;
}

pub enum Key {
	Backspace = 8,
	Tab = 9,
	Enter = 13,
	Shift = 16,
	Ctrl = 17,
	Alt = 18,
	CapsLock = 20,
	Escape = 27,
	Space = 32,
	PageUp = 33,
	PageDown = 34,
	End = 35,
	Home = 36,
	LeftArrow = 37,
	UpArrow = 38,
	RightArrow = 39,
	DownArrow = 40,
	Insert = 45,
	Delete = 46,
	Key0 = 48,
	Key1 = 49,
	Key2 = 50,
	Key3 = 51,
	Key4 = 52,
	Key5 = 53,
	Key6 = 54,
	Key7 = 55,
	Key8 = 56,
	Key9 = 57,
	KeyA = 65,
	KeyB = 66,
	KeyC = 67,
	KeyD = 68,
	KeyE = 69,
	KeyF = 70,
	KeyG = 71,
	KeyH = 72,
	KeyI = 73,
	KeyJ = 74,
	KeyK = 75,
	KeyL = 76,
	KeyM = 77,
	KeyN = 78,
	KeyO = 79,
	KeyP = 80,
	KeyQ = 81,
	KeyR = 82,
	KeyS = 83,
	KeyT = 84,
	KeyU = 85,
	KeyV = 86,
	KeyW = 87,
	KeyX = 88,
	KeyY = 89,
	KeyZ = 90,
	ContextMenu = 93,
	Numpad0 = 96,
	Numpad1 = 97,
	Numpad2 = 98,
	Numpad3 = 99,
	Numpad4 = 100,
	Numpad5 = 101,
	Numpad6 = 102,
	Numpad7 = 103,
	Numpad8 = 104,
	Numpad9 = 105,
	NumpadMultiply = 106,
	NumpadAdd = 107,
	NumpadSeparator = 108,
	NumpadSubtract = 109,
	NumpadDecimal = 110,
	NumpadDivide = 111,
	F1 = 112,
	F2 = 113,
	F3 = 114,
	F4 = 115,
	F5 = 116,
	F6 = 117,
	F7 = 118,
	F8 = 119,
	F9 = 120,
	F10 = 121,
	F11 = 122,
	F12 = 123,
	F13 = 124,
	F14 = 125,
	F15 = 126,
	F16 = 127,
	F17 = 128,
	F18 = 129,
	F19 = 130,
	NumLock = 144,
	ScrollLock = 145,
	UsSemicolon = 186,
	UsEqual = 187,
	UsComma = 188,
	UsMinus = 189,
	UsDot = 190,
	UsSlash = 191,
	UsBacktick = 192,
	UsOpenSquareBracket = 219,
	UsBackslash = 220,
	UsCloseSquareBracket = 221,
	UsQuote = 222,
	Oem8 = 223,
	Oem102 = 226,
}

impl Key {
	pub fn modify_key<T: Into<i32>>(modifiers: i32, key: T) -> i32 {
		modifiers | key.into()
	}
}

impl Into<i32> for Key {
    fn into(self) -> i32 {
        self as i32
    }
}
