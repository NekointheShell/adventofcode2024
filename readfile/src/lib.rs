use std::path::Path;
use std::fs::read_to_string;


#[allow(dead_code)]
pub fn readfile(filepath: &String) -> String {
    let path = Path::new(filepath);
    if !path.exists() { panic!("File doesn't exist!"); }

    let content = read_to_string(path).unwrap();
    return content;
}
