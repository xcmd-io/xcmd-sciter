use super::Renderer;
use sciter::Element;
use xcmd_core::api::{File, Icon, Value};

pub struct NameRenderer {}

impl NameRenderer {
	pub fn new() -> Self {
		NameRenderer {}
	}
}

impl Renderer for NameRenderer {
	fn render(&self, _file: &File, value: &Value) -> Element {
		let text = &value.to_string();
		let mut cell = if !text.is_empty() {
			Element::with_text("td", text).unwrap()
		} else {
			Element::create("td").unwrap()
		};
		if let Value::Path { icon, .. } = value {
			match icon {
				Icon::Local(filename) => {
					cell.set_attribute("filename", filename).unwrap();
					cell.set_attribute("behavior", "file-icon").unwrap();
				}

				Icon::Shell(filename) => {
					cell.set_attribute("filename", filename).unwrap();
					cell.set_attribute("behavior", "shell-icon").unwrap();
				}
			}
		}
		cell
	}
}
