use super::Renderer;
use sciter::Element;
use xcmd_core::api::{File, Value};

pub struct Column {
	name: String,
	renderer: Box<Renderer>,
}

impl Column {
	pub fn new(name: &str, renderer: Box<Renderer>) -> Self {
		Column {
			name: name.to_owned(),
			renderer,
		}
	}

	pub fn get_name(&self) -> &str {
		&self.name
	}

	pub fn render_value(&self, file: &File, value: &Value) -> Element {
		self.renderer.render(file, value)
	}
}
