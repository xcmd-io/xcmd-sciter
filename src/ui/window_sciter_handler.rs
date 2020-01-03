use crate::ui::Template;
use sciter::host::{HostHandler, LOAD_RESULT, SCN_LOAD_DATA};

const APP_DATA: &[(&str, &[u8])] = &include!(concat!(env!("OUT_DIR"), "/$app_data.rs"));

pub struct WindowSciterHandler {}

impl WindowSciterHandler {
	pub fn new() -> Self {
		WindowSciterHandler {}
	}
}

impl HostHandler for WindowSciterHandler {
	fn on_data_load(&mut self, data: &mut SCN_LOAD_DATA) -> Option<LOAD_RESULT> {
		let requested_uri = sciter::w2s!(data.uri);
		log::info!("Loading: {:?}", &requested_uri);
		if requested_uri.starts_with("app://") {
			let requested_path = &requested_uri["app://".len()..];
			for (path, bytes) in APP_DATA {
				if &requested_path == path {
					if requested_path.ends_with(".html") || requested_path.ends_with(".css") {
						let template = Template::new(&String::from_utf8_lossy(bytes));
						let color_theme = Template::parse_toml(include_str!(
							"../../config/dark.color-theme.toml"
						));
						let rendered_template = template.render(&color_theme);
						let mut html_with_bom = vec![0xef, 0xbb, 0xbf];
						html_with_bom.extend_from_slice(rendered_template.as_bytes());
						self.data_ready(
							data.hwnd,
							&requested_uri,
							&html_with_bom,
							None, //Some(data.request_id),
						);
						return Some(LOAD_RESULT::LOAD_DEFAULT);
					} else {
						self.data_ready(
							data.hwnd,
							&requested_uri,
							bytes,
							None, //Some(data.request_id),
						);
						return Some(LOAD_RESULT::LOAD_DEFAULT);
					}
				}
			}
		}
		None
	}
}
