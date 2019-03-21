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
use ui::WindowState;
use url::Url;

#[cfg(windows)]
pub fn win_append_extension(filename: &str, extension: &str) -> String {
	let mut link_name = filename.to_owned();
	link_name.push_str(extension);
	link_name
}

#[cfg(unix)]
pub fn win_append_extension(filename: &str, _extension: &str) -> String {
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
	let launcher_exe = launcher_dir.join(win_append_extension(pkg_name, ".lnk"));
	println!("launcher_exe={:?}", &launcher_exe);
	update_link(&launcher_exe, &current_exe);
	println!("link updated");
	if current_version == pkg_version && read_link(&launcher_exe, hwnd) == current_exe {
		let latest_release_url =
			Url::parse("https://api.github.com/repos/xcmd-io/xcmd/releases/latest")
				.expect("latest release url");
		println!("latest_release_url={:?}", &latest_release_url);
		let latest_release = repository::get_latest_release(&latest_release_url);
		if latest_release.tag_name > pkg_version {
			let latest_dir = launcher_dir.join(latest_release.tag_name.to_string());
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
			for asset in latest_release.assets {
				repository::download(&latest_dir, &asset);
			}
			let latest_exe = latest_dir.join(win_append_extension(pkg_name, ".exe"));
			update_link(&launcher_exe, &latest_exe);
			// start(&launcher_exe);
			// exit(state);
		}
	}
}
