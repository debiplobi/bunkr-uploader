use std::fs;

pub fn delete_all_dir(resources_path: &str) {
    match fs::read_dir(resources_path) {
        Ok(entries) => {
            for entry in entries {
                let path = entry.expect("failed to read directory entry").path();
                if path.is_dir() {
                    fs::remove_dir_all(path).expect("failed to remove chunk directory");
                }
            }
        }
        Err(e) => eprintln!("failed to read resources path, {}", e),
    };
}
