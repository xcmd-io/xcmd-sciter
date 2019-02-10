use super::renderer::Renderer;
use sciter::Element;
use separator::Separatable;
use xcmd_core::api::{File, Value};

pub struct SizeRenderer;

impl SizeRenderer {
	pub fn new() -> Self {
		SizeRenderer {}
	}
}

impl Renderer for SizeRenderer {
	fn render(&self, _file: &File, value: &Value) -> Element {
		let text = if let Value::Size { bytes } = value {
			bytes.separated_string()
		} else {
			"".to_owned()
		};
		if !text.is_empty() {
			let mut cell = Element::with_text("td", &text).unwrap();
			cell.set_attribute("class", "align-right").unwrap();
			cell
		} else {
			Element::create("td").unwrap()
		}
	}
}
