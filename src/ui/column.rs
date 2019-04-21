use sciter::Element;
use xcmd_core::api::{File, Value};

pub struct Column {
	name: String,
}

impl Column {
	pub fn new(name: &str) -> Self {
		Column {
			name: name.to_owned(),
		}
	}

	pub fn get_name(&self) -> &str {
		&self.name
	}
}
