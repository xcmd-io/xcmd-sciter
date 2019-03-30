use sciter::Element;

pub struct Palette {
	active: bool,
	pub active_index: u32,
	palette: Element,
	input: Element,
	tbody: Element,
}

impl Palette {
	pub fn new(element: &mut Element) -> Palette {
		let mut palette = Palette {
			active: false,
			active_index: 0,
			palette: Element::from(element.as_ptr()),
			input: element.find_first("input").unwrap().unwrap(),
			tbody: element.find_first("tbody").unwrap().unwrap(),
		};
		let commands = vec![
			"Edit File",
			"Enter Item",
			"Exit",
			"Hide Palette",
			"Move Down",
			"Move Home",
			"Move Page Down",
			"Move Page Up",
			"Move Up",
			"Move to Line Start",
			"Move to Line End",
			"Select Up",
			"Select Down",
			"Show Palette",
			"Switch Pane",
			"Toggle Selection",
			"Update Application",
			"View File",
		];
		for (index, command) in commands.iter().enumerate() {
			palette.append_row(command, index as u32);
		}
		palette
	}

	fn append_row(&mut self, text: &str, index: u32) {
		let mut row = Element::create("tr").unwrap();
		if index == self.active_index {
			row.set_attribute("active", "true").unwrap();
		}
		self.tbody.append(&row).unwrap();

		let cell = Element::with_text("td", text).unwrap();
		row.append(&cell).unwrap();
	}

	pub fn get_item(&self, index: u32) -> Option<Element> {
		self.tbody.child(index as usize)
	}

	pub fn activate(&mut self, active: bool) {
		self.active = active;
		if self.active {
			self.palette.set_attribute("active", "true").unwrap();
		} else {
			self.palette.remove_attribute("active").unwrap();
		}
	}

	pub fn move_up(&mut self) {
		if self.active_index != 0 {
			let active_index = self.active_index - 1;
			self.set_active_item(active_index);
		}
	}

	pub fn move_down(&mut self) {
		let active_index = self.active_index + 1;
		self.set_active_item(active_index);
	}

	pub fn move_home(&mut self) {
		if self.active_index != 0 {
			self.set_active_item(0);
		}
	}

	pub fn move_end(&mut self) {
		// if !self.files.is_empty() {
		// 	let active_index = self.files.len() as u32 - 1;
		// 	if self.active_index != active_index {
		// 		self.set_active_item(active_index);
		// 	}
		// }
	}

	pub fn page_up(&mut self) {
		if self.active_index != 0 {
			if let Some(active_item) = self.get_item(self.active_index) {
				let height = active_item
					.call_method("box", &[sciter::Value::symbol("height")])
					.expect("row height")
					.to_int()
					.expect("int");
				let tbody = active_item.parent().expect("tbody");
				let tbody_height = tbody
					.call_method("box", &[sciter::Value::symbol("height")])
					.expect("tbody height")
					.to_int()
					.expect("int");
				let items_per_page = (tbody_height / height) as u32;
				let active_index = if self.active_index < items_per_page {
					0
				} else {
					self.active_index - items_per_page
				};
				self.set_active_item(active_index);
			}
		}
	}

	pub fn page_down(&mut self) {
		if let Some(active_item) = self.get_item(self.active_index) {
			let height = active_item
				.call_method("box", &[sciter::Value::symbol("height")])
				.expect("row height")
				.to_int()
				.expect("int");
			let tbody = active_item.parent().expect("tbody");
			let tbody_height = tbody
				.call_method("box", &[sciter::Value::symbol("height")])
				.expect("tbody height")
				.to_int()
				.expect("int");
			let items_per_page = (tbody_height / height) as u32;
			let items_count = tbody.children_count() as u32;
			let active_index = if self.active_index + items_per_page > items_count {
				items_count - 1
			} else {
				self.active_index + items_per_page
			};
			self.set_active_item(active_index);
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
}
