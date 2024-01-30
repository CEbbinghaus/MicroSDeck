use lazy_static::*;
use std::path::PathBuf;

pub const PACKAGE_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const PACKAGE_VERSION: &'static str = include_str!("../version");
pub const PACKAGE_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

const TEMPDIR: &'static str = "/tmp/MicroSDeck";

lazy_static! {
	pub static ref DATA_DIR: PathBuf = PathBuf::from(
		match std::env::var("DECKY_PLUGIN_RUNTIME_DIR") {
			Ok(loc) => loc.to_string(),
			Err(_) => {
				println!("Unable to find \"DECKY_PLUGIN_RUNTIME_DIR\" in env. Assuming Dev mode & using temporary directory");
				TEMPDIR.to_string() + "/data"
			}
		}
	);
	pub static ref LOG_DIR: PathBuf = PathBuf::from(match std::env::var("DECKY_PLUGIN_LOG_DIR") {
		Ok(loc) => loc.to_string(),
		Err(_) => {
			println!("Unable to find \"DECKY_PLUGIN_LOG_DIR\" in env. Assuming Dev mode & using temporary directory");
			TEMPDIR.to_string() + "/log"
		}
	});
}

pub fn get_file_path_and_create_directory(
	file_name: &PathBuf,
	base_dir: &PathBuf,
) -> Option<PathBuf> {

	if let Err(_) = std::fs::create_dir_all(base_dir) {
		return None;
	}

	Some(base_dir.join(file_name))
}
