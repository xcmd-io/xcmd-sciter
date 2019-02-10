use repository;
use sciter::types::_HWINDOW;
use sciter::Element;
use semver::Version;
#[cfg(windows)]
use shortcut;
use std::fs;
use std::io::ErrorKind;
#[cfg(unix)]
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use url::Url;
use window_event_handler::WindowState;

#[cfg(windows)]
pub fn get_link_name(filename: &str) -> String {
	let mut link_name = filename.to_owned();
	link_name.push_str(".lnk");
	link_name
}

#[cfg(unix)]
pub fn get_link_name(filename: &str) -> String {
	filename.to_owned()
}

#[cfg(windows)]
fn read_link(link: &Path, hwnd: *const _HWINDOW) -> PathBuf {
	shortcut::read_link(link, hwnd)
}

#[cfg(unix)]
fn read_link(link: &Path, _hwnd: *const _HWINDOW) -> PathBuf {
	fs::read_link(link).expect("read link")
}

#[cfg(windows)]
fn update_link(link: &Path, target: &Path) {
	shortcut::update_link(link, target);
}

#[cfg(unix)]
fn update_link(link: &Path, target: &Path) {
	symlink(target, link).expect("symlink");
}

pub fn update_self(_state: &mut WindowState, root: &Element) {
	println!("self_update");
	let pkg_name = env!("CARGO_PKG_NAME");
	println!("pkg_name={}", &pkg_name);
	let pkg_version = Version::parse(env!("CARGO_PKG_VERSION")).expect("current version");
	println!("pkg_version={}", &pkg_version);
	let exe_path = std::env::current_exe().expect("current exe");
	println!("exe_path={:?}", &exe_path);
	let hwnd = root.get_hwnd(true);
	// let current_exe = read_link(&exe_path, hwnd);
	let current_exe = exe_path;
	println!("current_exe={:?}", &current_exe);
	let current_dir = current_exe.parent().expect("current dir");
	println!("current_dir={:?}", &current_dir);
	let current_dirname = current_dir.file_name().expect("current dir name");
	println!("current_dirname={:?}", &current_dirname);
	let current_version =
		Version::parse(&current_dirname.to_string_lossy()).expect("current dir version");
	println!("current_version={:?}", &current_version);
	let launcher_dir = current_dir.parent().expect("launcher dir");
	println!("launcher_dir={:?}", &launcher_dir);
	let launcher_exe = launcher_dir.join(get_link_name(pkg_name));
	println!("launcher_exe={:?}", &launcher_exe);
	update_link(&launcher_exe, &current_exe);
	println!("link updated");
	if current_version == pkg_version && read_link(&launcher_exe, hwnd) == current_exe {
		let base = Url::parse("http://localhost:3000/download/latest/").expect("base url");
		let latest_data_url = base.join("package.json").expect("package url");
		println!("latest_data_url={:?}", &latest_data_url);
		let latest_data = repository::get_latest_version_data(&latest_data_url);
		if latest_data.version > pkg_version {
			let latest_dir = launcher_dir.join(latest_data.version.to_string());
			println!("latest_dir={:?}", &latest_dir);
			fs::create_dir(&latest_dir)
				.or_else(|e| {
					if e.kind() == ErrorKind::AlreadyExists {
						Ok(())
					} else {
						Err(e)
					}
				})
				.expect("create latest dir");
			for file in latest_data.files {
				repository::download(&latest_dir, &base, &file.filename, &file.checksum);
			}
			let latest_exe = latest_dir.join(latest_data.exe);
			update_link(&launcher_exe, &latest_exe);
			// start(&launcher_exe);
			// exit(state);
		}
	}
}
