use sciter::Element;
use xcmd_core::api::{File, Value};

pub trait Renderer {
	fn render(&self, file: &File, value: &Value) -> Element;
}
