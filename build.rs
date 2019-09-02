extern crate brotli;
extern crate sha2;
#[cfg(windows)]
extern crate windres;

use brotli::enc::{BrotliCompress, BrotliEncoderParams};
use sha2::{Digest, Sha256};
use std::env;
use std::error::Error;
use std::fs::{self, DirEntry, File};
use std::io::{self, Write};
use std::path::Path;
#[cfg(windows)]
use windres::Build;

fn prepare_sciter_lib(file: &str) {
	let lib_path_string = format!("lib/{}", file);
	let compressed_path_string = format!("lib/{}.br", file);
	let checksum_path_string = format!("lib/{}.br.sha256", file);
	let lib_path = Path::new(&lib_path_string);
	let compressed_path = Path::new(&compressed_path_string);
	let checksum_path = Path::new(&checksum_path_string);
	if !compressed_path.exists() {
		{
			println!("compressing");
			let mut input = File::open(lib_path).unwrap();
			let mut output = File::create(compressed_path).unwrap();
			let encoder_params = BrotliEncoderParams::default();
			BrotliCompress(&mut input, &mut output, &encoder_params).unwrap();
			println!("compressed");
		}
		{
			let mut sha256 = Sha256::new();
			let mut input = File::open(lib_path).unwrap();
			io::copy(&mut input, &mut sha256).unwrap();
			let mut output = File::create(checksum_path).unwrap();
			let hash = sha256.result();
			output.write_all(&hash).unwrap();
			println!("hash is: {}", format!("{:x}", &hash));
		}
	}
}

fn visit_dirs(
	dir: &Path,
	cb: &dyn Fn(&DirEntry) -> Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
			let entry = entry?;
			let path = entry.path();
			if path.is_dir() {
				visit_dirs(&path, cb)?;
			} else {
				cb(&entry)?;
			}
		}
	}
	Ok(())
}

fn prepare_sciter_app_files() -> Result<(), Box<dyn Error>> {
	let app_data_dir = "src/app/";
	let app_data_path = Path::new(app_data_dir);
	let out_dir = env::var("OUT_DIR")?;
	let output_path = Path::new(&out_dir).join("$app_data.rs");
	let mut output = File::create(&output_path)?;

	writeln!(&mut output, r#"["#,)?;

	visit_dirs(app_data_path, &|dir_entry: &DirEntry| {
		if dir_entry.file_type()?.is_file() {
			let dir_entry_path = dir_entry.path();
			writeln!(
				&output,
				r#"("{name}", include_bytes!("{path}")),"#,
				name = dir_entry_path.to_string_lossy()[app_data_dir.len()..].replace("\\", "/"),
				path = dir_entry_path
					.canonicalize()?
					.to_string_lossy()
					.replace("\\", "\\\\")
			)?;
		}
		Ok(())
	})?;

	writeln!(&mut output, r#"];"#,)?;

	Ok(())
}

fn main() {
	env::set_var("INCLUDE", "src/include");

	#[cfg(windows)]
	Build::new().compile("src/main.rc").unwrap();

	#[cfg(target_os = "windows")]
	prepare_sciter_lib("sciter.dll");

	#[cfg(target_os = "linux")]
	prepare_sciter_lib("libsciter-gtk.so");

	#[cfg(target_os = "macos")]
	prepare_sciter_lib("sciter-osx-64.dylib");

	prepare_sciter_app_files().unwrap();
}
