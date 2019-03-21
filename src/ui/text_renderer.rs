use super::Renderer;
use sciter::Element;
use xcmd_core::api::{File, Value};

pub struct TextRenderer;

impl TextRenderer {
	pub fn new() -> Self {
		TextRenderer {}
	}
}

impl Renderer for TextRenderer {
	fn render(&self, _file: &File, value: &Value) -> Element {
		let text = &value.to_string();
		if !text.is_empty() {
			Element::with_text("td", text).unwrap()
		} else {
			Element::create("td").unwrap()
		}
	}
}
