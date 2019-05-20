#![cfg_attr(feature = "cargo-clippy", allow(clippy::eval_order_dependence))]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate sciter;
extern crate reqwest;
extern crate semver;
extern crate separator;
extern crate url;
extern crate xcmd_core;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[cfg(windows)]
#[macro_use]
extern crate winapi;
extern crate brotli;
extern crate regex;
extern crate sha2;

#[cfg(windows)]
#[macro_use]
mod com;
mod data_source;
mod repository;
mod self_update;
#[cfg(windows)]
mod shortcut;
mod ui;

use brotli::BrotliDecompress;
use sciter::{RuntimeOptions, Window};
use sha2::{Digest, Sha256};
use std::env;
use std::fmt::Write;
use std::fs::{self, File};
use std::io;
use std::path::Path;
use ui::{Template, WindowEventHandler};

macro_rules! lib_path {
	() => {
		"../lib/"
	};
}

#[cfg(target_os = "windows")]
macro_rules! sciter_dll {
	() => {
		"sciter"
	};
}

#[cfg(target_os = "linux")]
macro_rules! sciter_dll {
	() => {
		"libsciter-gtk"
	};
}

#[cfg(target_os = "macos")]
macro_rules! sciter_dll {
	() => {
		"sciter-osx-64"
	};
}

#[cfg(target_os = "windows")]
macro_rules! dll_ext {
	() => {
		".dll"
	};
}

#[cfg(target_os = "linux")]
macro_rules! dll_ext {
	() => {
		".so"
	};
}

#[cfg(target_os = "macos")]
macro_rules! dll_ext {
	() => {
		".dylib"
	};
}

fn initialize_sciter_library() {
	let library = include_bytes!(concat!(lib_path!(), sciter_dll!(), dll_ext!(), ".br"));

	let mut temp = env::temp_dir();
	let checksum = include_bytes!(concat!(
		lib_path!(),
		sciter_dll!(),
		dll_ext!(),
		".br.sha256"
	));
	let mut checksum_string = String::new();
	for &byte in checksum {
		write!(&mut checksum_string, "{:x}", byte).unwrap();
	}
	temp.push(&format!(
		concat!(sciter_dll!(), "-{}", dll_ext!()),
		&checksum_string[..16]
	));

	if temp.exists() && compute_checksum(&temp).as_slice() != checksum {
		fs::remove_file(&temp).unwrap();
	}

	{
		let mut file = File::create(&temp).unwrap();
		BrotliDecompress(&mut library.as_ref(), &mut file).unwrap();
	}

	sciter::set_library(temp.to_str().unwrap()).unwrap();
}

use sha2::digest::generic_array::typenum::U32;
use sha2::digest::generic_array::GenericArray;

fn compute_checksum(path: &Path) -> GenericArray<u8, U32> {
	let mut sha256 = Sha256::new();
	let mut input = File::open(&path).unwrap();
	io::copy(&mut input, &mut sha256).unwrap();
	sha256.result()
}

fn main() {
	initialize_sciter_library();

	sciter::set_options(RuntimeOptions::ScriptFeatures(
		sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8 | // Enables Sciter.machineName()
		sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8 | // Enables opening file dialog (view.selectFile())
		sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SOCKET_IO as u8, // Enables connecting to the inspector via Ctrl+Shift+I
	))
	.unwrap();
	sciter::set_options(RuntimeOptions::DebugMode(true)).unwrap();
	let template = Template::new(include_str!("shell.html"));
	let color_theme = Template::parse_toml(include_str!("../config/light.color-theme.toml"));
	let rendered_template = template.render(&color_theme);
	let mut html_with_bom = vec![0xef, 0xbb, 0xbf];
	html_with_bom.extend_from_slice(rendered_template.as_bytes());
	let mut window = Window::new();
	let handler = WindowEventHandler::new();
	window.event_handler(handler);
	window.load_html(&html_with_bom, Some("app://shell.html"));
	window.set_title(&format!("Cross Commander {}", env!("CARGO_PKG_VERSION")));
	window.run_app();
}
