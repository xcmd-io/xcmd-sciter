use super::pane::Pane;
use super::self_update::update_self;
use sciter::dom::event::{EventReason, BEHAVIOR_EVENTS, EVENT_GROUPS, PHASE_MASK};
use sciter::dom::{ELEMENT_STATE_BITS, HELEMENT};
use sciter::{Element, EventHandler};
use std::collections::HashMap;
use std::process::Command;
use xcmd_core::api::System;
use xcmd_core::local::LocalSystem;
use xcmd_core::sftp::SftpSystem;

type Callback = Box<(Fn(&mut WindowState, &Element) -> ()) + 'static>;

fn mk_callback<F>(f: F) -> Callback
where
	F: (Fn(&mut WindowState, &Element) -> ()) + 'static,
{
	Box::new(f) as Callback
}

pub struct WindowState {
	active_pane: u8,
	left_pane: Option<Pane>,
	right_pane: Option<Pane>,
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
			state: WindowState {
				active_pane: 0,
				left_pane: None,
				right_pane: None,
			},
		}
	}

	fn on_document_ready(&mut self, root: HELEMENT) {
		let root = Element::from(root);
		self.state.left_pane = Some(self.create_pane(&mut find_first(&root, "#left-pane"), 0));
		self.state.right_pane = Some(self.create_pane(&mut find_first(&root, "#right-pane"), 1));
		self.root = Some(root);

		self.commands.insert(
			"switch_pane".to_owned(),
			mk_callback(|state: &mut WindowState, _root: &Element| switch_pane(state)),
		);
		self.commands.insert(
			"move_up".to_owned(),
			mk_callback(|state: &mut WindowState, _root: &Element| move_up(state)),
		);
		self.commands.insert(
			"move_down".to_owned(),
			mk_callback(|state: &mut WindowState, _root: &Element| move_down(state)),
		);
		self.commands.insert(
			"page_up".to_owned(),
			mk_callback(|state: &mut WindowState, _root: &Element| page_up(state)),
		);
		self.commands.insert(
			"page_down".to_owned(),
			mk_callback(|state: &mut WindowState, _root: &Element| page_down(state)),
		);
		self.commands.insert(
			"select_up".to_owned(),
			mk_callback(|state: &mut WindowState, _root: &Element| select_up(state)),
		);
		self.commands.insert(
			"select_down".to_owned(),
			mk_callback(|state: &mut WindowState, _root: &Element| select_down(state)),
		);
		self.commands.insert(
			"toggle_select".to_owned(),
			mk_callback(|state: &mut WindowState, _root: &Element| toggle_select(state)),
		);
		self.commands.insert(
			"enter_item".to_owned(),
			mk_callback(|state: &mut WindowState, _root: &Element| enter_item(state)),
		);
		self.commands.insert(
			"exit".to_owned(),
			mk_callback(|state: &mut WindowState, _root: &Element| exit(state)),
		);
		self.commands.insert(
			"update_self".to_owned(),
			mk_callback(|state: &mut WindowState, root: &Element| update_self(state, root)),
		);
		self.commands.insert(
			"view_file".to_owned(),
			mk_callback(|state: &mut WindowState, _root: &Element| view_file(state)),
		);
		self.commands.insert(
			"edit_file".to_owned(),
			mk_callback(|state: &mut WindowState, _root: &Element| edit_file(state)),
		);

		self.key_handlers.insert(9, "switch_pane".to_owned()); // tab
		self.key_handlers.insert(38, "move_up".to_owned()); // up
		self.key_handlers.insert(39, "update_self".to_owned()); // up
		self.key_handlers.insert(40, "move_down".to_owned()); // down
		self.key_handlers.insert(SHIFT | 38, "select_up".to_owned()); // shift+up
		self.key_handlers
			.insert(SHIFT | 40, "select_down".to_owned()); // shift+down
		self.key_handlers.insert(32, "toggle_select".to_owned()); // space
		self.key_handlers.insert(13, "enter_item".to_owned()); // enter
		self.key_handlers.insert(ALT | 115, "exit".to_owned()); // alt+f4
		self.key_handlers
			.insert(CTRL | 85, "update_self".to_owned()); // ctrl+u
		self.key_handlers.insert(33, "page_up".to_owned()); // pgup
		self.key_handlers.insert(34, "page_down".to_owned()); // pgdn
		self.key_handlers.insert(114, "view_file".to_owned()); // f3
		self.key_handlers.insert(115, "edit_file".to_owned()); // f4
	}

	fn on_key(
		&mut self,
		event_type: i32,
		key_code: i32,
		alt_key: bool,
		ctrl_key: bool,
		shift_key: bool,
	) -> bool {
		if event_type == BEHAVIOR_EVENTS::BUTTON_CLICK as i32 {
			let key = if alt_key { ALT } else { 0 }
				| if ctrl_key { CTRL } else { 0 }
				| if shift_key { SHIFT } else { 0 }
				| key_code;
			// println!("on_key: type={}, keyCode={}, alt={}, ctrl={}, shift={}", event_type, key_code, alt_key, ctrl_key, shift_key);
			let mut key_command = None;
			if let Some(key_handler) = self.key_handlers.get(&key) {
				if let Some(command) = self.commands.get(key_handler) {
					key_command = Some(command);
				}
			};
			let mut state = &mut self.state;
			if let Some(command) = key_command {
				if let Some(root) = &self.root {
					command(&mut state, &root);
				}
			}
			true
		} else {
			false
		}
	}

	fn create_pane(&self, element: &mut Element, index: u8) -> Pane {
		let system: Box<System> = if index < 2 {
			Box::new(LocalSystem::default())
		} else {
			Box::new(SftpSystem::new())
		};
		let mut pane = Pane::new(element, index == self.state.active_pane, system);
		pane.update(None);
		pane
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
		fn on_key(i32, i32, bool, bool, bool);
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
		if let Some(file) = pane.files.get(pane.active_index as usize) {
			Command::new("lister")
				.arg(super::pane::get_path(file))
				.output()
				.expect("lister");
		}
	}
}

fn edit_file(state: &mut WindowState) {
	if let Some(pane) = state.get_active_pane() {
		if let Some(file) = pane.files.get(pane.active_index as usize) {
			Command::new("notepad")
				.arg(super::pane::get_path(file))
				.output()
				.expect("lister");
		}
	}
}

fn exit(_state: &mut WindowState) {
	// TODO: close application
}

// fn resolve_link(path: &Path) -> PathBuf {
// 	match std::fs::read_link(path) {
// 		Ok(path_buf) => path_buf,
// 		Err(_) => path.to_owned()
// 	}
// }
