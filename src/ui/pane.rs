use crate::data_source::DataSource;
use crate::ui::Column;
use sciter::dom::event::{BEHAVIOR_EVENTS, CLICK_REASON};
use sciter::Element;
use separator::Separatable;
use std::cell::RefCell;
use std::rc::Rc;
use xcmd_core::api::{Error, File, Icon, System, Value};

pub struct Pane {
	active: bool,
	pub system: Box<dyn System>,
	field_names: Vec<String>,
	pub data_source: Rc<RefCell<FilesDataSource>>,
	parent: String,
	pane: Element,
	tab: Element,
	input: Element,
	vtable: Element,
	files_height: i32,
	item_height: i32,
}

impl Pane {
	pub fn new(element: &mut Element, active: bool, mut system: Box<dyn System>) -> Pane {
		let mut field_names: Vec<String> = Vec::new();
		field_names.push("path".to_owned());
		field_names.push("extension".to_owned());
		field_names.push("size".to_owned());
		field_names.push("created_on".to_owned());
		field_names.push("attributes".to_owned());
		let root = &system.get_root(&Rc::new(field_names.clone())).unwrap();
		let parent_path = get_path(&root);
		println!("parent_path={}", parent_path);
		let mut pane = Pane {
			active,
			system,
			data_source: Rc::new(RefCell::new(FilesDataSource::new())),
			field_names,
			parent: parent_path,
			pane: Element::from(element.as_ptr()),
			tab: element.find_first("tab").unwrap().unwrap(),
			input: element.find_first("input").unwrap().unwrap(),
			vtable: element.find_first("vtable").unwrap().unwrap(),
			files_height: 640,
			item_height: 32,
		};
		if pane.active {
			element.set_attribute("class", "pane-active").unwrap();
			pane.tab.set_attribute("class", "tab-active").unwrap();
		}
		if let Ok(ref mut data_source) = RefCell::try_borrow_mut(&mut pane.data_source) {
			data_source.columns.push(Column::new("Name"));
			data_source.columns.push(Column::new("Ext"));
			data_source.columns.push(Column::new("Size"));
			data_source.columns.push(Column::new("Date"));
			data_source.columns.push(Column::new("Attributes"));
		}
		pane
	}

	pub fn update(&mut self, selected_path: Option<&str>) {
		let now = std::time::Instant::now();
		let filename = &self.system.get_filename(&self.parent);
		self.input.set_text(&self.parent).unwrap();
		self.tab.set_text(filename).unwrap();
		match self.list_files() {
			Ok(files) => {
				let mut active_index: Option<usize> = None;
				let mut path_index: Option<usize> = None;
				for (index, file) in files.iter().enumerate() {
					if active_index == None {
						path_index = path_index.or(file.get_field_index("path"));
						if Some(get_path_at(&file, path_index).as_ref()) == selected_path {
							active_index = Some(index);
						}
					}
				}
				if let Ok(ref mut data_source) = RefCell::try_borrow_mut(&mut self.data_source) {
					data_source.files = files;
				}
				self.set_active_item(active_index.unwrap_or(0) as u32);
			}
			Err(e) => println!("Error: {}", e),
		}
		println!("{:?}", now.elapsed());
		self.vtable
			.send_event(
				BEHAVIOR_EVENTS::CHANGE,
				Some(CLICK_REASON::SYNTHESIZED),
				None,
			)
			.unwrap();
	}

	pub fn get_active_path(&self) -> Option<String> {
		let data_source = self.data_source.borrow();
		if let Some(active_file) = data_source.files.get(data_source.active_index as usize) {
			Some(get_path(active_file))
		} else {
			None
		}
	}

	pub fn set_files_height(&mut self, files_height: i32, item_height: i32) {
		self.files_height = files_height;
		self.item_height = item_height;
	}

	pub fn move_up(&mut self) {
		if self.data_source.borrow().active_index != 0 {
			let active_index = self.data_source.borrow().active_index - 1;
			self.set_active_item(active_index);
		}
	}

	pub fn move_down(&mut self) {
		let active_index = std::cmp::min(
			self.data_source.borrow().files.len() as u32 - 1,
			self.data_source.borrow().active_index + 1,
		);
		self.set_active_item(active_index);
	}

	pub fn move_home(&mut self) {
		self.set_active_item(0);
	}

	pub fn move_end(&mut self) {
		if !self.data_source.borrow().files.is_empty() {
			let active_index = self.data_source.borrow().files.len() as u32 - 1;
			self.set_active_item(active_index);
		}
	}

	pub fn page_up(&mut self) {
		if !self.data_source.borrow().files.is_empty() {
			let page_size = (self.files_height / self.item_height) as u32;
			let active_index = self.data_source.borrow().active_index;
			let active_index = if active_index >= page_size {
				active_index - page_size
			} else {
				0
			};
			self.set_active_item(active_index);
		}
	}

	pub fn page_down(&mut self) {
		if !self.data_source.borrow().files.is_empty() {
			let page_size = (self.files_height / self.item_height) as u32;
			let active_index = self.data_source.borrow().active_index;
			let max_index = self.data_source.borrow().files.len() as u32 - 1;
			let active_index = if active_index + page_size <= max_index {
				active_index + page_size
			} else {
				max_index
			};
			self.set_active_item(active_index);
		}
	}

	pub fn toggle_select(&mut self) {
		if let Ok(ref mut data_source) = RefCell::try_borrow_mut(&mut self.data_source) {
			let active_index = data_source.active_index as usize;
			let file = data_source.files.get_mut(active_index);
			if let Some(mut file) = file {
				file.selected = !file.selected;
			};
		}
		self.vtable
			.send_event(
				BEHAVIOR_EVENTS::CHANGE,
				Some(CLICK_REASON::SYNTHESIZED),
				None,
			)
			.unwrap();
	}

	fn set_active_item(&mut self, active_index: u32) {
		if let Ok(ref mut data_source) = RefCell::try_borrow_mut(&mut self.data_source) {
			data_source.active_index = active_index;
		}
		self.vtable
			.call_method("onChange", &sciter::make_args!(active_index as i32))
			.unwrap();
	}

	pub fn activate(&mut self, active: bool) {
		self.active = active;
		if self.active {
			self.pane.set_attribute("class", "pane-active").unwrap();
		} else {
			self.pane.remove_attribute("class").unwrap();
		}
	}

	pub fn enter_item(&mut self) {
		let previous_parent = self.parent.clone();
		if let Some(new_parent) = self.get_active_path() {
			self.parent = new_parent;
			self.update(Some(&previous_parent));
		}
	}

	pub fn list_files(&mut self) -> Result<Vec<File>, Error> {
		let field_names = Rc::new((&self.field_names).clone());
		println!("parent={}", self.parent);
		let parent = self.system.get_file(&self.parent, &field_names)?;
		println!("list files");
		self.system.list_files(&parent, &field_names)
	}
}

pub struct FilesDataSource {
	active_index: u32,
	columns: Vec<Column>,
	files: Vec<File>,
}

impl FilesDataSource {
	pub fn new() -> Self {
		FilesDataSource {
			active_index: 0,
			columns: Vec::new(),
			files: Vec::new(),
		}
	}
}

impl DataSource for FilesDataSource {
	fn data_source_columns(&self) -> sciter::Value {
		let columns = &self.columns;
		let mut data = sciter::Value::array(columns.len());
		for (index, column) in columns.iter().enumerate() {
			data.set(index, column.get_name());
		}
		data
	}

	fn data_source_row_count(&self) -> i32 {
		self.files.len() as i32
	}

	fn data_source_rows_data(&self, row_index: i32, row_count: i32) -> sciter::Value {
		let mut rows = sciter::Value::array(row_count as usize);
		let columns = &self.columns;
		for index in 0..row_count {
			let virtual_row_index = row_index + index;
			if let Some(file) = &self.files.get(virtual_row_index as usize) {
				let mut row = sciter::Value::map();
				if virtual_row_index as u32 == self.active_index {
					row.set_item(sciter::Value::from("active"), sciter::Value::from(true));
				}
				if file.selected {
					row.set_item(sciter::Value::from("selected"), sciter::Value::from(true));
				}

				let mut cells = sciter::Value::array(columns.len());
				for (index, value) in (&file.fields).iter().enumerate() {
					// let column = &columns[index];
					let mut cell = sciter::Value::map();
					let text = if let Value::Size { bytes } = value {
						cell.set_item(
							sciter::Value::from("textAlign"),
							sciter::Value::from("right"),
						);
						bytes.separated_string()
					} else {
						value.to_string()
					};
					cell.set_item(sciter::Value::from("text"), sciter::Value::from(text));

					if let Value::Path { icon, .. } = value {
						match icon {
							Icon::Local(filename) => {
								cell.set_item(
									sciter::Value::from("fileIcon"),
									sciter::Value::from(filename.to_owned()),
								);
							}

							Icon::Shell(filename) => {
								cell.set_item(
									sciter::Value::from("shellIcon"),
									sciter::Value::from(filename.to_owned()),
								);
							}
						}
					}

					cells.set(index, cell);
				}
				row.set_item(sciter::Value::from("cells"), cells);
				rows.set(index as usize, row);
			}
		}
		rows
	}
}

pub fn get_path(file: &File) -> String {
	if let Some(path_index) = file.get_field_index("path") {
		if let Value::Path { path, .. } = &file.fields[path_index] {
			path.to_owned()
		} else {
			file.fields[path_index].to_string()
		}
	} else {
		"".to_owned()
	}
}

fn get_path_at(file: &File, path_index: Option<usize>) -> String {
	if let Some(path_index) = path_index {
		if let Value::Path { path, .. } = &file.fields[path_index] {
			path.to_owned()
		} else {
			file.fields[path_index].to_string()
		}
	} else {
		"".to_owned()
	}
}
