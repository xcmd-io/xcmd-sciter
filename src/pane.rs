use super::column::Column;
use super::name_renderer::NameRenderer;
use super::size_renderer::SizeRenderer;
use super::text_renderer::TextRenderer;
use sciter::Element;
use std::rc::Rc;
use xcmd_core::api::{Error, File, System, Value};

pub struct Pane {
	active: bool,
	active_index: u32,
	system: Box<System>,
	columns: Vec<Column>,
	field_names: Vec<String>,
	parent: String,
	pane: Element,
	tab: Element,
	input: Element,
	thead: Element,
	tbody: Element,
}

impl Pane {
	pub fn new(element: &mut Element, active: bool, mut system: Box<System>) -> Pane {
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
			active_index: 0,
			system,
			columns: Vec::new(),
			field_names,
			parent: parent_path,
			pane: Element::from(element.as_ptr()),
			tab: element.find_first("tab").unwrap().unwrap(),
			input: element.find_first("input").unwrap().unwrap(),
			thead: element.find_first("thead").unwrap().unwrap(),
			tbody: element.find_first("tbody").unwrap().unwrap(),
		};
		if pane.active {
			element.set_attribute("class", "pane-active").unwrap();
			pane.tab.set_attribute("class", "tab-active").unwrap();
		}
		pane.columns
			.push(Column::new("Name", Box::new(NameRenderer::new())));
		pane.columns
			.push(Column::new("Ext", Box::new(TextRenderer::new())));
		pane.columns
			.push(Column::new("Size", Box::new(SizeRenderer::new())));
		pane.columns
			.push(Column::new("Date", Box::new(TextRenderer::new())));
		pane.columns
			.push(Column::new("Attributes", Box::new(TextRenderer::new())));
		pane.append_title_row();
		pane
	}

	pub fn update(&mut self, selected_path: Option<&str>) {
		let filename = &self.system.get_filename(&self.parent);
		self.input.set_text(&self.parent).unwrap();
		self.tab.set_text(filename).unwrap();
		self.tbody.clear().unwrap();
		match self.list_files() {
			Ok(files) => {
				let mut path_index: Option<usize> = None;
				for (index, file) in files.iter().enumerate() {
					self.append_row(file, index as u32);
					path_index = if let Some(value) = path_index {
						Some(value)
					} else {
						file.get_field_index("path")
					};
					if Some(get_path_at(&file, path_index).as_ref()) == selected_path {
						self.set_active_item(index as u32);
					}
				}
			}
			Err(e) => println!("Error: {}", e),
		}
	}

	fn append_title_row(&mut self) {
		let mut row = Element::create("tr").unwrap();
		self.thead.append(&row).unwrap();
		for column in &self.columns {
			let cell = Element::with_text("th", column.get_name()).unwrap();
			row.append(&cell).unwrap();
		}
	}

	fn append_row(&mut self, file: &File, index: u32) {
		let mut row = Element::create("tr").unwrap();
		row.set_attribute("path", get_path(&file).as_ref()).unwrap();
		if index == self.active_index {
			row.set_attribute("active", "true").unwrap();
		}
		self.tbody.append(&row).unwrap();

		for (index, value) in (&file.fields).iter().enumerate() {
			let column = &self.columns[index];
			let cell = column.render_value(file, &value);
			row.append(&cell).unwrap();
		}
	}

	fn get_item(&self, index: u32) -> Option<Element> {
		self.tbody.child(index as usize)
	}

	pub fn move_up(&mut self) {
		if self.active_index != 0 {
			let active_index = &self.active_index - 1;
			self.set_active_item(active_index);
		}
	}

	pub fn move_down(&mut self) {
		let active_index = &self.active_index + 1;
		self.set_active_item(active_index);
	}

	pub fn toggle_select(&mut self) {
		if let Some(mut active_item) = self.get_item(self.active_index) {
			if let Some(_selected) = &active_item.get_attribute("selected") {
				active_item
					.remove_attribute("selected")
					.expect("remove selected");
			} else {
				active_item
					.set_attribute("selected", "true")
					.expect("set selected");
			}
		}
	}

	fn set_active_item(&mut self, active_index: u32) {
		if let Some(mut new_item) = self.get_item(active_index) {
			if let Some(mut old_item) = self.get_item(self.active_index) {
				old_item.remove_attribute("active").expect("remove active");
			}

			self.active_index = active_index;
			new_item
				.set_attribute("active", "true")
				.expect("set active");
			let scapi = sciter::SciterAPI();
			(scapi.SciterScrollToView)(new_item.as_ptr(), 0);
		}
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
		if let Some(active_item) = self.get_item(self.active_index) {
			self.set_active_item(0);
			let previous_parent = self.parent.clone();
			self.parent = active_item
				.get_attribute("path")
				.unwrap_or_else(|| "".to_owned());
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

fn get_path(file: &File) -> String {
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
