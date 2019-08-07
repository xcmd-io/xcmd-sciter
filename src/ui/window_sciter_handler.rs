use sciter::host::{HostHandler, LOAD_RESULT, SCN_LOAD_DATA};
use ui::Template;

pub struct WindowSciterHandler {}

impl WindowSciterHandler {
	pub fn new() -> Self {
		WindowSciterHandler {}
	}
}

impl HostHandler for WindowSciterHandler {
	fn on_data_load(&mut self, data: &mut SCN_LOAD_DATA) -> Option<LOAD_RESULT> {
		let requested_uri = sciter::w2s!(data.uri);
		eprintln!("Loading: {:?}", &requested_uri);
		if requested_uri.starts_with("app:") {
			if requested_uri == "app://xcmd/shell.html" {
				let template = Template::new(include_str!("../shell.html"));
				let color_theme =
					Template::parse_toml(include_str!("../../config/light.color-theme.toml"));
				let rendered_template = template.render(&color_theme);
				let mut html_with_bom = vec![0xef, 0xbb, 0xbf];
				html_with_bom.extend_from_slice(rendered_template.as_bytes());
				self.data_ready(
					data.hwnd,
					&requested_uri,
					&html_with_bom,
					Some(data.request_id),
				);
				return Some(LOAD_RESULT::LOAD_DELAYED);
			}
		}
		None
	}
}
