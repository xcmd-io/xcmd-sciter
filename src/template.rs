use regex::{Captures, Regex};
use std::collections::HashMap;

pub struct Template {
	regex: Regex,
	template: String,
}

impl Template {
	pub fn new(template: &str) -> Self {
		Template {
			regex: Regex::new(r#"var\('([a-zA-Z0-9.]*)'\)"#).expect("regex"),
			template: template.to_owned(),
		}
	}

	pub fn parse_json(json: &str) -> HashMap<String, String> {
		serde_json::from_str::<HashMap<String, String>>(json).unwrap()
	}

	pub fn render(&self, map: &HashMap<String, String>) -> String {
		let regex = &self.regex;
		let default_value = "inherit".to_owned();
		regex
			.replace_all(&self.template, |captures: &Captures| {
				let value = captures.get(1).expect("capture 1").as_str();
				map.get(value).unwrap_or(&default_value)
			})
			.to_string()
	}
}
