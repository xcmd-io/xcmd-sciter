use reqwest::Url;
use semver::Version;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Deserialize)]
pub struct Asset {
	pub name: String,
	#[serde(with = "url_serde")]
	pub url: Url,
}

#[derive(Deserialize)]
pub struct Release {
	pub tag_name: Version,
	pub assets: Vec<Asset>,
}

pub fn get_latest_release(url: &Url) -> Release {
	let mut response = reqwest::get(url.to_owned()).expect("download response");
	let json: Release = response.json().expect("JSON");
	json
}

pub fn download(target_dir: &Path, asset: &Asset) {
	let client = reqwest::Client::new();
	let mut response = client
		.get(&asset.url.to_string())
		.header("Accept", "application/octet-stream")
		.send()
		.expect("download asset");
	assert!(response.status().is_success(), "download failed");
	let mut target_file = File::create(target_dir.join(&asset.name)).expect("target file");
	io::copy(&mut response, &mut target_file).expect("download file");
}
