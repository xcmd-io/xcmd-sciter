extern crate brotli;
extern crate sha2;
#[cfg(windows)]
extern crate windres;

use brotli::enc::{BrotliCompress, BrotliEncoderParams};
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::fs::File;
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
}
