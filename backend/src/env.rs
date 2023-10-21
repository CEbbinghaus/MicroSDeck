use std::path::Path;

const TEMPDIR: &'static str = "/tmp/MicroSDeck";

pub fn get_data_dir() -> String {
    return match std::env::var("DECKY_PLUGIN_RUNTIME_DIR") {
        Ok(loc) => loc.to_string(),
        // Err(_) => TEMPDIR.to_string() + "/data",
        Err(_) => "/home/deck/homebrew/data/DeckyPlugin/".to_string(),
    };
}
pub fn get_log_dir() -> String {
    return match std::env::var("DECKY_PLUGIN_LOG_DIR") {
        Ok(loc) => loc.to_string(),
        Err(_) => TEMPDIR.to_string() + "/log",
    };
}

pub fn get_file_path(file_name: &str, get_base_dir: &dyn Fn() -> String) -> Option<String> {
    let dir = get_base_dir();

    match Path::new(dir.as_str()).join(file_name).to_str() {
        Some(v) => Some(v.to_string()),
        None => None,
    }
}

pub fn get_file_path_and_create_directory(
    file_name: &str,
    get_base_dir: &dyn Fn() -> String,
) -> Option<String> {
    let dir = get_base_dir();

    if let Err(_) = std::fs::create_dir_all(std::path::Path::new(dir.as_str())) {
        return None;
    }

    get_file_path(file_name, get_base_dir)
}
