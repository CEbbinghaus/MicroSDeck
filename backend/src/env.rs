use std::path::Path;

pub fn get_dir() -> String {
    return match std::env::var("DECKY_PLUGIN_RUNTIME_DIR") {
        Err(_) => 
            if cfg!(debug_assertions) {
                Path::new("/tmp").join("MicroSDeck").to_str().unwrap().to_owned()
            } else {
                panic!("Unable to proceed")
            }
        
        Ok(loc) => loc.to_string(),
    }
}

pub fn get_file_path(file_name: &str) -> String {
    let dir = get_dir();

    Path::new(dir.as_str()).join(file_name).to_str().unwrap().to_owned()
}

pub fn get_file_path_and_create_directory(file_name: &str) -> String {
    let dir = get_dir();

    std::fs::create_dir_all(std::path::Path::new(dir.as_str())).unwrap();

    Path::new(dir.as_str()).join(file_name).to_str().unwrap().to_owned()
}