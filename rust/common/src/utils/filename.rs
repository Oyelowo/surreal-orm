pub fn get_current_filename() -> &'static str {
    std::path::Path::new(file!())
        .file_stem()
        .map(|f| f.to_str().expect(" Problem converting to string"))
        .unwrap()
}
