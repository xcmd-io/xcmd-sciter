use crate::api::{Error, File, Icon, System, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Default)]
pub struct LocalSystem;

impl LocalSystem {
	fn get_local_file(
		&self,
		path: &Path,
		name: Option<String>,
		field_names: &Rc<Vec<String>>,
	) -> Result<File, Error> {
		let path = Path::new(path);
		let canonicalized_path = path
			.canonicalize()
			.unwrap_or_else(|_err| PathBuf::from(path));
		let full_path =
			trim_long_path_prefix(&canonicalized_path.to_string_lossy().into_owned()).to_owned();
		let (name, extension) = if let Some(name) = name {
			(name, "".to_owned())
		} else if path.is_dir() {
			let filename = path
				.file_name()
				.map(|x| x.to_string_lossy().into_owned())
				.unwrap_or_else(|| String::from(".."));
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
		let metadata = path.metadata();
		let size = if let Ok(metadata) = metadata {
			metadata.len()
		} else {
			0
		};
		Ok(File::new(
			field_names,
			vec![
				Value::Path {
					path: full_path.clone(),
					name: name,
					icon: Icon::Local(full_path),
				},
				Value::String { string: extension },
				Value::Size { bytes: size },
			],
		))
	}
}

impl System for LocalSystem {
	fn get_root(&mut self, field_names: &Rc<Vec<String>>) -> Result<File, Error> {
		self.get_file(".", field_names)
	}

	fn get_file(&mut self, path: &str, field_names: &Rc<Vec<String>>) -> Result<File, Error> {
		self.get_local_file(Path::new(path), None, field_names)
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
					files.push(self.get_local_file(
						&parent_path,
						Some("[..]".to_owned()),
						field_names,
					)?);
				}
				match fs::read_dir(&path) {
					Ok(read_dir) => {
						for child_path in read_dir {
							files.push(self.get_local_file(
								&child_path?.path(),
								None,
								field_names,
							)?);
						}
					}
					Err(e) => println!("Failed to read children: {}", e),
				}
			}
		}
		Ok(files)
	}
}

fn trim_long_path_prefix(path: &str) -> &str {
	if path.starts_with("\\\\?\\") {
		&path[4..]
	} else {
		path
	}
}
