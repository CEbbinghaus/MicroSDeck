use log::warn;
use std::path::Path;

pub const PORT: u16 = 12412; // TODO replace with something unique

pub const PACKAGE_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const PACKAGE_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const PACKAGE_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

const TEMPDIR: &'static str = "/tmp/MicroSDeck";

pub fn get_data_dir() -> String {
	return match std::env::var("DECKY_PLUGIN_RUNTIME_DIR") {
		Ok(loc) => loc.to_string(),
		Err(_) => {
			warn!("Unable to find \"DECKY_PLUGIN_RUNTIME_DIR\" in env. Assuming Dev mode & using temporary directory");
			TEMPDIR.to_string() + "/data"
		}
	};
}
pub fn get_log_dir() -> String {
	return match std::env::var("DECKY_PLUGIN_LOG_DIR") {
		Ok(loc) => loc.to_string(),
		Err(_) => {
			warn!("Unable to find \"DECKY_PLUGIN_LOG_DIR\" in env. Assuming Dev mode & using temporary directory");
			TEMPDIR.to_string() + "/log"
		}
	};
}

pub fn get_file_path(file_name: &str, get_base_dir: &dyn Fn() -> String) -> Option<String> {
	let dir = get_base_dir();

	Path::new(dir.as_str())
		.join(file_name)
		.to_str()
		.map(|v| v.to_string())
}

pub fn get_file_path_and_create_directory(
	file_name: &str,
	get_base_dir: &dyn Fn() -> String,
) -> Option<String> {
	let dir = get_base_dir();

	if let Err(_) = std::fs::create_dir_all(Path::new(dir.as_str())) {
		return None;
	}

	get_file_path(file_name, get_base_dir)
}
