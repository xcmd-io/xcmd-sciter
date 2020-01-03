use crate::api::{Error, File, Icon, System, Value};
use ssh2::{FileStat, Session, Sftp};
use std::net::TcpStream;
use std::path::Path;
use std::rc::Rc;

pub struct SftpSystem {
	sftp: Sftp,
}

impl SftpSystem {
	pub fn new() -> SftpSystem {
		let mut session = Session::new().unwrap();
		let tcp = TcpStream::connect("hostname:port").unwrap();
		session.set_tcp_stream(tcp);
		session.handshake().unwrap();
		session.userauth_password("username", "password").unwrap();
		assert!(session.authenticated());

		SftpSystem {
			sftp: session.sftp().unwrap(),
		}
	}

	fn get_sftp_file(
		&self,
		path: &Path,
		name: Option<String>,
		stat: Option<FileStat>,
		field_names: &Rc<Vec<String>>,
	) -> Result<File, Error> {
		let path = Path::new(path);
		let full_path = path.to_string_lossy().into_owned();
		let filename = path
			.file_name()
			.map(|x| x.to_string_lossy().into_owned())
			.unwrap_or_else(|| String::from(".."));
		let stat = if let Some(stat) = stat {
			Ok(stat)
		} else {
			self.sftp.stat(path)
		};
		let is_dir = if let Ok(stat) = &stat {
			stat.is_dir()
		} else {
			false
		};
		let (name, extension) = if let Some(name) = name {
			(name, "".to_owned())
		} else if is_dir {
			(format!("[{}]", filename), "".to_owned())
		} else {
			(
				path.file_stem()
					.map(|x| x.to_string_lossy().into_owned())
					.unwrap_or_else(|| String::from("..")),
				path.extension()
					.map(|x| x.to_string_lossy().into_owned())
					.unwrap_or_else(|| String::from("")),
			)
		};
		let size = if let Ok(stat) = &stat {
			stat.size.unwrap()
		} else {
			0
		};
		Ok(File::new(
			field_names,
			vec![
				Value::Path {
					path: full_path,
					name: name,
					icon: self.get_icon(is_dir, &extension),
				},
				Value::String { string: extension },
				Value::Size { bytes: size },
			],
		))
	}

	fn get_icon(&self, is_dir: bool, ext: &str) -> Icon {
		let file = if is_dir {
			"C:\\.".to_owned()
		} else {
			format!("C:\\*.{}", ext)
		};
		Icon::Shell(file)
	}
}

impl System for SftpSystem {
	fn get_root(&mut self, field_names: &Rc<Vec<String>>) -> Result<File, Error> {
		self.get_file("/", field_names)
	}

	fn get_file(&mut self, path: &str, field_names: &Rc<Vec<String>>) -> Result<File, Error> {
		self.get_sftp_file(Path::new(path), None, None, field_names)
	}

	fn get_filename(&mut self, path: &str) -> String {
		Path::new(path)
			.file_name()
			.map(|x| x.to_string_lossy().into_owned())
			.unwrap_or_else(|| path.to_string())
	}

	fn list_files(
		&mut self,
		parent_directory: &File,
		field_names: &Rc<Vec<String>>,
	) -> Result<Vec<File>, Error> {
		let mut files: Vec<File> = Vec::new();
		if let Some(path_index) = parent_directory.get_field_index("path") {
			if let Value::Path { path, .. } = &parent_directory.fields[path_index] {
				let path = Path::new(path);
				if let Some(parent_path) = path.parent() {
					files.push(self.get_sftp_file(
						&parent_path,
						Some("[..]".to_owned()),
						None,
						field_names,
					)?);
				}
				log::info!("SFTP readdir: {:?}", path);
				match self.sftp.readdir(&path) {
					Ok(read_dir) => {
						for (child_path, stat) in read_dir {
							files.push(self.get_sftp_file(
								&child_path,
								None,
								Some(stat),
								field_names,
							)?);
						}
					}
					Err(e) => log::error!("Failed to read children: {}", e),
				}
			}
		}
		Ok(files)
	}
}
