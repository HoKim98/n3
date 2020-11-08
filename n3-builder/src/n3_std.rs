use std::collections::HashMap;
use std::path::Path;

use glob::glob;
use inflector::Inflector;

pub fn get_sources(root: &Path) -> HashMap<String, String> {
    get_files(root, "n3")
}

pub fn get_externs(root: &Path) -> HashMap<String, String> {
    get_files(root, "py")
}

fn get_files(root: &Path, extension: &'static str) -> HashMap<String, String> {
    let result: HashMap<_, _> = glob(&format!("{}/std/**/*.{}", root.display(), extension))
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|p| (trim_path(&p), load_source(&p)))
        .collect();

    if result.is_empty() {
        panic!("variable 'N3_SOURCE_ROOT' is incorrect")
    }
    result
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
