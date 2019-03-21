#[cfg(windows)]
extern crate windres;

use std::env;
use std::fs;
use std::path::Path;
#[cfg(windows)]
use windres::Build;

fn copy_lib_to_output(out_dir: &str, file: &str) {
	let dest_path = Path::new(&out_dir).join("../../..").join(file);
	println!("copying {} to {}", file, out_dir);
	let _ = fs::copy(format!("lib/{}", file), dest_path);
}

fn main() {
	env::set_var("INCLUDE", "src/include");
	let out_dir = env::var("OUT_DIR").ok().expect("can't find out_dir");

	#[cfg(windows)]
	Build::new().compile("src/main.rc").unwrap();

	#[cfg(target_os = "windows")]
	copy_lib_to_output(&out_dir, "sciter.dll");

	#[cfg(target_os = "linux")]
	copy_lib_to_output(&out_dir, "libsciter-gtk.so");

	#[cfg(target_os = "macos")]
	copy_lib_to_output(&out_dir, "sciter-osx-64.dylib");
}
