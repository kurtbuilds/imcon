use std::path::PathBuf;

pub fn create_path(path_template: &str, input_path: &PathBuf, page: usize, n_pages: usize) -> String {
    let places = n_pages.to_string().len();
    path_template
        .replace("{}", input_path.file_stem().unwrap().to_string_lossy().as_ref())
        .replace("{i}", format!("{:0places$}", page, places = places).as_ref())
}