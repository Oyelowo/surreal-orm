use std::fs::create_dir_all;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::Path,
};


pub fn write_data_to_path(data: &String, path: impl AsRef<Path>) {
    let path_prefix = path.as_ref().parent().expect("Couldnt get parent path");
    create_dir_all(path_prefix).expect("Problem creaging directory for graphql");

    let f = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(path)
        .expect("unable to open file");

    let mut f = BufWriter::new(f);
    f.write_all(data.as_bytes()).expect("Unable to write data");
}
