use super::{Palette, Pane};
use crate::data_source::DataSource;
use crate::self_update::update_self;
use sciter::dom::event::{EventReason, BEHAVIOR_EVENTS, EVENT_GROUPS, PHASE_MASK};
use sciter::dom::{ELEMENT_STATE_BITS, HELEMENT};
use sciter::{Element, EventHandler, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::process::Command;
use std::rc::Rc;
use xcmd_core::api::System;
use xcmd_core::local::LocalSystem;
use xcmd_core::sftp::SftpSystem;

type Callback = Box<dyn (Fn(&mut WindowState, &Element) -> ()) + 'static>;

pub fn mk_callback<F>(f: F) -> Callback
where
	F: (Fn(&mut WindowState, &Element) -> ()) + 'static,
{
	Box::new(f) as Callback
}

#[derive(Deserialize)]
struct KeyBinding {
	key: String,
	command: String,
	when: String,
}

pub struct WindowState {
	active_pane: u8,
	left_pane: Option<Pane>,
	right_pane: Option<Pane>,
	palette: Option<Palette>,
	data_sources: HashMap<String, Rc<RefCell<dyn DataSource>>>,
}

impl WindowState {
	fn get_pane(&mut self, index: u8) -> &mut Option<Pane> {
		if index == 0 {
			&mut self.left_pane
		} else {
			&mut self.right_pane
		}
	}

	fn get_active_pane(&mut self) -> &mut Option<Pane> {
		let active_pane = self.active_pane;
		self.get_pane(active_pane)
	}

	fn set_active_pane(&mut self, active_pane: u8) {
		let old_active_pane = self.active_pane;
		if let Some(ref mut old_pane) = &mut self.get_pane(old_active_pane) {
			old_pane.activate(false);
		}

		self.active_pane = active_pane;
		if let Some(ref mut new_pane) = &mut self.get_pane(active_pane) {
			new_pane.activate(true);
		}
	}
}

pub struct WindowEventHandler {
	root: Option<Element>,
	commands: HashMap<String, Callback>,
	key_map: HashMap<i32, i32>,
	key_names: HashMap<String, i32>,
	key_handlers: HashMap<i32, String>,
	state: WindowState,
}

const ALT: i32 = 0x1000_0000;
const CTRL: i32 = 0x0100_0000;
const SHIFT: i32 = 0x0010_0000;

impl WindowEventHandler {
	pub fn new() -> Self {
		WindowEventHandler {
			root: None,
			commands: HashMap::new(),
			key_handlers: HashMap::new(),
			key_map: HashMap::new(),   // code -> index
			key_names: HashMap::new(), // name -> index
			state: WindowState {
				active_pane: 0,
				left_pane: None,
				right_pane: None,
				palette: None,
				data_sources: HashMap::new(),
			},
		}
	}

	fn get_key_map() -> HashMap<String, String> {
		#[cfg(target_os = "windows")]
		return toml::from_str::<HashMap<String, String>>(include_str!(
			"../../config/windows.key-map.toml"
		))
		.unwrap();

		#[cfg(target_os = "linux")]
		return toml::from_str::<HashMap<String, String>>(include_str!(
			"../../config/linux.key-map.toml"
		))
		.unwrap();

		#[cfg(target_os = "macos")]
		return toml::from_str::<HashMap<String, String>>(include_str!(
			"../../config/macos.key-map.toml"
		))
		.unwrap();
	}

	fn initialize_key_map(&mut self) {
		let key_map = Self::get_key_map();
		let mut key_names_len = self.key_names.len() as i32;
		for (key_code, key_name) in &key_map {
			let key_code = key_code.parse::<i32>().unwrap();
			let key_index = if let Some(key_index) = self.key_names.get(key_name) {
				*key_index
			} else {
				key_names_len
			};
			if key_names_len == key_index {
				self.key_names.insert(key_name.to_owned(), key_index);
				key_names_len = key_names_len + 1;
			}
			self.key_map.insert(key_code, key_index);
		}
	}

	pub fn register_command(&mut self, command_name: &str, command_handler: Callback) {
		self.commands
			.insert(command_name.to_owned(), command_handler);
	}

	fn on_document_ready(&mut self, root: HELEMENT) {
		let root = Element::from(root);

		let left_pane = self.create_pane(&mut find_first(&root, "#left-pane"), 0);
		let left_data_source = Rc::clone(&left_pane.data_source);
		self.state.data_sources.insert(
			"left-pane".to_owned(),
			left_data_source as Rc<RefCell<dyn DataSource>>,
		);

		let right_pane = self.create_pane(&mut find_first(&root, "#right-pane"), 0);
		let right_data_source = Rc::clone(&right_pane.data_source);
		self.state.data_sources.insert(
			"right-pane".to_owned(),
			right_data_source as Rc<RefCell<dyn DataSource>>,
		);

		self.state.left_pane = Some(left_pane);
		self.state.right_pane = Some(right_pane);
		self.state.palette = Some(Palette::new(&mut find_first(&root, "#palette")));
		self.root = Some(root);

		self.register_command(
			"pane.switchPane",
			mk_callback(|state: &mut WindowState, _root: &Element| switch_pane(state)),
		);
		self.register_command(
			"pane.moveUp",
			mk_callback(|state: &mut WindowState, _root: &Element| move_up(state)),
		);
		self.register_command(
			"pane.moveDown",
			mk_callback(|state: &mut WindowState, _root: &Element| move_down(state)),
		);
		self.register_command(
			"pane.moveHome",
			mk_callback(|state: &mut WindowState, _root: &Element| move_home(state)),
		);
		self.register_command(
			"pane.moveEnd",
			mk_callback(|state: &mut WindowState, _root: &Element| move_end(state)),
		);
		self.register_command(
			"pane.pageUp",
			mk_callback(|state: &mut WindowState, _root: &Element| page_up(state)),
		);
		self.register_command(
			"pane.pageDown",
			mk_callback(|state: &mut WindowState, _root: &Element| page_down(state)),
		);
		self.register_command(
			"pane.selectUp",
			mk_callback(|state: &mut WindowState, _root: &Element| select_up(state)),
		);
		self.register_command(
			"pane.selectDown",
			mk_callback(|state: &mut WindowState, _root: &Element| select_down(state)),
		);
		self.register_command(
			"pane.toggleSelect",
			mk_callback(|state: &mut WindowState, _root: &Element| toggle_select(state)),
		);
		self.register_command(
			"pane.enterItem",
			mk_callback(|state: &mut WindowState, _root: &Element| enter_item(state)),
		);
		self.register_command(
			"pane.exit",
			mk_callback(|state: &mut WindowState, root: &Element| exit(state, root)),
		);
		self.register_command(
			"pane.updateSelf",
			mk_callback(|state: &mut WindowState, root: &Element| update_self(state, root)),
		);
		self.register_command(
			"pane.viewFile",
			mk_callback(|state: &mut WindowState, _root: &Element| view_file(state)),
		);
		self.register_command(
			"pane.editFile",
			mk_callback(|state: &mut WindowState, _root: &Element| edit_file(state)),
		);
		self.register_command(
			"palette.show",
			mk_callback(|state: &mut WindowState, _root: &Element| show_palette(state)),
		);
		self.register_command(
			"palette.hide",
			mk_callback(|state: &mut WindowState, _root: &Element| hide_palette(state)),
		);

		self.initialize_key_map();

		let json = include_str!("../../config/keybindings.json");
		let key_bindings = serde_json::from_str::<Vec<KeyBinding>>(json).unwrap();
		for key_binding in &key_bindings {
			if let Some(modified_key_index) = self.parse_key(&key_binding.key) {
				self.key_handlers
					.insert(modified_key_index, key_binding.command.to_owned());
			}
		}

		if let Some(ref mut pane) = &mut self.state.left_pane {
			pane.update(None);
		}
		if let Some(ref mut pane) = &mut self.state.right_pane {
			pane.activate(false);
			pane.update(None);
		}
	}

	fn parse_key(&mut self, key: &str) -> Option<i32> {
		let mut key = key;
		let mut modifier = 0;
		loop {
			let mut found_modifier = false;
			if key.starts_with("alt+") {
				key = &key["alt+".len()..];
				modifier |= ALT;
				found_modifier = true;
			}
			if key.starts_with("shift+") {
				key = &key["shift+".len()..];
				modifier |= SHIFT;
				found_modifier = true;
			}
			if key.starts_with("ctrl+") {
				key = &key["ctrl+".len()..];
				modifier |= CTRL;
				found_modifier = true;
			}
			if !found_modifier {
				break;
			}
		}
		if let Some(key_index) = self.key_names.get(key) {
			Some(modifier | *key_index)
		} else {
			None
		}
	}

	fn log(&mut self, message: String) {
		println!("log: {}", message);
	}

	fn on_key(
		&mut self,
		event_type: i32,
		key_code: i32,
		alt_key: bool,
		ctrl_key: bool,
		shift_key: bool,
	) -> bool {
		println!(
			"on_key: type={}, keyCode={}, alt={}, ctrl={}, shift={}",
			event_type, key_code, alt_key, ctrl_key, shift_key
		);
		if event_type == BEHAVIOR_EVENTS::BUTTON_CLICK as i32 {
			if let Some(key_index) = self.key_map.get(&key_code) {
				let key = if alt_key { ALT } else { 0 }
					| if ctrl_key { CTRL } else { 0 }
					| if shift_key { SHIFT } else { 0 }
					| key_index;
				let key_command = if let Some(key_handler) = self.key_handlers.get(&key) {
					self.commands.get(key_handler)
				} else {
					None
				};
				let mut state = &mut self.state;
				if let (Some(command), Some(root)) = (key_command, &self.root) {
					command(&mut state, &root);
				}
				true
			} else {
				false
			}
		} else {
			false
		}
	}

	fn on_resize_files(&mut self, files_height: i32, item_height: i32) {
		if let Some(ref mut pane) = &mut self.state.left_pane {
			pane.set_files_height(files_height, item_height);
		}
		if let Some(ref mut pane) = &mut self.state.right_pane {
			pane.set_files_height(files_height, item_height);
		}
	}

	fn data_source_columns(&mut self, name: String) -> Value {
		if let Some(data_source) = self.state.data_sources.get(&name) {
			data_source.borrow().data_source_columns()
		} else {
			Value::array(0)
		}
	}

	fn data_source_row_count(&mut self, name: String) -> i32 {
		if let Some(data_source) = self.state.data_sources.get(&name) {
			data_source.borrow().data_source_row_count()
		} else {
			0
		}
	}

	fn data_source_rows_data(&mut self, name: String, row_index: i32, row_count: i32) -> Value {
		if let Some(data_source) = self.state.data_sources.get(&name) {
			data_source
				.borrow()
				.data_source_rows_data(row_index, row_count)
		} else {
			Value::array(row_count as usize)
		}
	}

	fn create_pane(&self, element: &mut Element, index: u8) -> Pane {
		let system: Box<dyn System> = if index < 2 {
			Box::new(LocalSystem::default())
		} else {
			Box::new(SftpSystem::new())
		};
		Pane::new(element, index == self.state.active_pane, system)
	}
}

impl EventHandler for WindowEventHandler {
	fn attached(&mut self, root: HELEMENT) {
		let mut root = Element::from(root);
		let _ = root.set_state(ELEMENT_STATE_BITS::STATE_FOCUSABLE, None, true);
	}

	fn get_subscription(&mut self) -> Option<EVENT_GROUPS> {
		Some(EVENT_GROUPS::HANDLE_ALL)
	}

	#[allow(clippy::eval_order_dependence)]
	dispatch_script_call! {
		fn log(String);
		fn on_key(i32, i32, bool, bool, bool);
		fn data_source_columns(String);
		fn data_source_row_count(String);
		fn data_source_rows_data(String, i32, i32);
		fn on_resize_files(i32, i32);
	}

	fn on_event(
		&mut self,
		root: HELEMENT,
		_source: HELEMENT,
		_target: HELEMENT,
		code: BEHAVIOR_EVENTS,
		phase: PHASE_MASK,
		_reason: EventReason,
	) -> bool {
		if phase != PHASE_MASK::BUBBLING {
			return false;
		}

		// println!("code={:?}, reason={:?}", code, reason);

		if code == BEHAVIOR_EVENTS::DOCUMENT_READY {
			self.on_document_ready(root);
		}

		false
	}
}

fn find_first(element: &Element, selector: &str) -> Element {
	element.find_first(selector).unwrap().unwrap()
}

fn switch_pane(state: &mut WindowState) {
	let active_pane = 1 - state.active_pane;
	state.set_active_pane(active_pane);
}

fn move_up(state: &mut WindowState) {
	if let Some(pane) = state.get_active_pane() {
		pane.move_up();
	}
}

fn move_down(state: &mut WindowState) {
	if let Some(pane) = state.get_active_pane() {
		pane.move_down();
	}
}

fn page_up(state: &mut WindowState) {
	if let Some(pane) = state.get_active_pane() {
		pane.page_up();
	}
}

fn page_down(state: &mut WindowState) {
	if let Some(pane) = state.get_active_pane() {
		pane.page_down();
	}
}

fn move_home(state: &mut WindowState) {
	if let Some(pane) = state.get_active_pane() {
		pane.move_home();
	}
}

fn move_end(state: &mut WindowState) {
	if let Some(pane) = state.get_active_pane() {
		pane.move_end();
	}
}

fn toggle_select(state: &mut WindowState) {
	if let Some(pane) = state.get_active_pane() {
		pane.toggle_select();
	}
}

fn enter_item(state: &mut WindowState) {
	if let Some(pane) = state.get_active_pane() {
		pane.enter_item();
	}
}

fn select_up(state: &mut WindowState) {
	if let Some(pane) = state.get_active_pane() {
		pane.toggle_select();
		pane.move_up();
	}
}

fn select_down(state: &mut WindowState) {
	if let Some(pane) = state.get_active_pane() {
		pane.toggle_select();
		pane.move_down();
	}
}

fn view_file(state: &mut WindowState) {
	if let Some(pane) = state.get_active_pane() {
		if let Some(path) = pane.get_active_path() {
			Command::new("lister").arg(path).output().expect("lister");
		}
	}
}

fn edit_file(state: &mut WindowState) {
	if let Some(pane) = state.get_active_pane() {
		if let Some(path) = pane.get_active_path() {
			Command::new("notepad").arg(path).output().expect("notepad");
		}
	}
}

fn exit(_state: &mut WindowState, root: &Element) {
	root.eval_script("view.close()").unwrap();
}

fn show_palette(state: &mut WindowState) {
	if let Some(palette) = &mut state.palette {
		palette.activate(true);
	}
}

fn hide_palette(state: &mut WindowState) {
	if let Some(palette) = &mut state.palette {
		palette.activate(false);
	}
}

// fn resolve_link(path: &Path) -> PathBuf {
// 	match std::fs::read_link(path) {
// 		Ok(path_buf) => path_buf,
// 		Err(_) => path.to_owned()
// 	}
// }
