#![cfg_attr(feature = "cargo-clippy", allow(clippy::eval_order_dependence))]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
use ui::{WindowEventHandler, WindowSciterHandler};
use xcmd_core::errors;

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

fn initialize_sciter_library() -> Result<(), errors::AppError> {
	log::info!("Initializing sciter library.");
	let library = include_bytes!(concat!(lib_path!(), sciter_dll!(), dll_ext!(), ".br"));

	let mut temp = env::temp_dir();
	let checksum = include_bytes!(concat!(
		lib_path!(),
		sciter_dll!(),
		dll_ext!(),
		".br.sha256"
	));
	log::info!("Calculating checksum.");
	let mut checksum_string = String::new();
	for &byte in checksum {
		write!(&mut checksum_string, "{:x}", byte).or(Err("Failed to write checksum."))?;
	}
	temp.push(&format!(
		concat!(sciter_dll!(), "-{}", dll_ext!()),
		&checksum_string[..16]
	));

	if temp.exists() && compute_checksum(&temp)?.as_slice() != checksum {
		fs::remove_file(&temp)?;
	}

	if !temp.exists() {
		log::info!("Decompressing Sciter library.");
		let mut file = File::create(&temp)?;
		BrotliDecompress(&mut library.as_ref(), &mut file)?;
	}

	sciter::set_library(temp.to_str().ok_or("Failed to set Sciter library.")?)?;
	log::info!("Initialized sciter library.");

	Ok(())
}

use sha2::digest::generic_array::typenum::U32;
use sha2::digest::generic_array::GenericArray;

fn compute_checksum(path: &Path) -> Result<GenericArray<u8, U32>, errors::AppError> {
	let mut sha256 = Sha256::new();
	let mut input = File::open(&path)?;
	io::copy(&mut input, &mut sha256)?;
	Ok(sha256.result())
}

fn main() -> Result<(), errors::AppError> {
	std::env::set_var("RUST_LOG", "xcmd=info");
	env_logger::init();

	initialize_sciter_library()?;

	sciter::set_options(RuntimeOptions::ScriptFeatures(
		sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8 | // Enables Sciter.machineName()
		sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8 | // Enables opening file dialog (view.selectFile())
		sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SOCKET_IO as u8 | // Enables connecting to the inspector via Ctrl+Shift+I
		sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_EVAL as u8, // Enables eval
	))?;
	sciter::set_options(RuntimeOptions::DebugMode(true))?;
	let mut window = Window::new();
	window.event_handler(WindowEventHandler::new());
	window.sciter_handler(WindowSciterHandler::new());
	window.load_file("app://xcmd/shell.sciter.html");
	window.set_title(&format!("Cross Commander {}", env!("CARGO_PKG_VERSION")));
	window.run_app();

	Ok(())
}
