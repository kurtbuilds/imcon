use std::path::PathBuf;

pub fn create_path(path_template: &str, input_path: &PathBuf, page: usize, n_pages: usize) -> String {
    path_template
        .replace("{}", input_path.file_stem().unwrap().to_string_lossy().as_ref())
        .replace("{i}", format!("{:0places$}", page, places = n_pages).as_ref())
}