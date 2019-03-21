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
extern crate regex;

#[cfg(windows)]
#[macro_use]
mod com;
mod repository;
mod self_update;
#[cfg(windows)]
mod shortcut;
mod ui;

use sciter::{RuntimeOptions, Window};
use ui::{Template, WindowEventHandler};

fn main() {
	sciter::set_options(RuntimeOptions::ScriptFeatures(
		sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8 | // Enables Sciter.machineName()
		sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8 | // Enables opening file dialog (view.selectFile())
		sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SOCKET_IO as u8, // Enables connecting to the inspector via Ctrl+Shift+I
	))
	.unwrap();
	sciter::set_options(RuntimeOptions::DebugMode(true)).unwrap();
	let template = Template::new(include_str!("shell.html"));
	let color_theme = Template::parse_toml(include_str!("../config/dark.color-theme.toml"));
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
