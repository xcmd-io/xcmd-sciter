use semver::Version;
use std::fs::File;
use std::io;
use std::path::Path;
use url::Url;

#[derive(Deserialize)]
pub struct VersionFileData {
	pub filename: String,
	pub checksum: String,
}

#[derive(Deserialize)]
pub struct VersionData {
	pub version: Version,
	pub exe: String,
	pub files: Vec<VersionFileData>,
}

pub fn get_latest_version_data(url: &Url) -> VersionData {
	let mut response = reqwest::get(url.to_owned()).expect("download response");
	let json: VersionData = response.json().expect("JSON");
	json
}

pub fn download(target_dir: &Path, base: &Url, filename: &str, _checksum: &str) {
	let url = base.join(filename).expect("download url");
	let mut response = reqwest::get(url).expect("download response");
	assert!(response.status().is_success(), "download failed");
	let mut target_file = File::create(target_dir.join(filename)).expect("target file");
	io::copy(&mut response, &mut target_file).expect("download file");
}
