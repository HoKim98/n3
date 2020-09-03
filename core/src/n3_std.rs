use std::collections::HashMap;
use std::path::Path;

use include_dir::{include_dir, Dir};
use inflector::Inflector;

// note: This macro will include a directory relative to the project root.
const STD_DIR: Dir = include_dir!("./std");

pub fn get_sources() -> HashMap<String, String> {
    STD_DIR
        .find("**/*.n3")
        .unwrap()
        .map(|e| e.path())
        .map(|p| (trim_path(p), load_source(p)))
        .collect()
}

fn trim_path(path: &Path) -> String {
    let filename = path.file_name().unwrap().to_str().unwrap();
    let filename = filename.split(".").next().unwrap();
    let filename = filename.replace("_", "-").to_pascal_case();
    filename.replace("-", "")
}

fn load_source(path: &Path) -> String {
    STD_DIR
        .get_file(path)
        .unwrap()
        .contents_utf8()
        .unwrap()
        .to_string()
}
