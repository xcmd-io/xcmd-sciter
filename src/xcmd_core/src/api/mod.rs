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
	pub selected: bool,
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
			selected: false,
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
