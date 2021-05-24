use std::path::PathBuf;
use std::env;

pub fn location_of(path: &str) -> PathBuf {
    let mut dir = match env::current_exe() {
        Err(e) => panic!("Error: {}", e),
        Ok(path) => path,
    };
    dir.pop();
    dir.push(path);
    dir
}
