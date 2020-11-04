use std::collections::HashMap;
use std::path::Path;

use glob::glob;
use inflector::Inflector;

pub fn get_sources(root: &str) -> HashMap<String, String> {
    get_files(root, "n3")
}

pub fn get_externs(root: &str) -> HashMap<String, String> {
    get_files(root, "py")
}

fn get_files(root: &str, extension: &'static str) -> HashMap<String, String> {
    glob(&format!("{}/**/*.{}", root, extension))
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|p| (trim_path(&p), load_source(&p)))
        .collect()
}

pub fn trim_path(path: &Path) -> String {
    let filename = path.file_name().unwrap().to_str().unwrap();
    let filename = filename.split('.').next().unwrap();
    let filename = filename.replace("_", "-").to_pascal_case();
    filename.replace("-", "")
}

fn load_source(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap()
}
