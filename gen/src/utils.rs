use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn create_table(location: &str, headers: &[u8]) -> File {
    let mut fh = File::create(location).unwrap();
    fh.write(headers).unwrap();

    fh
}

pub fn write_entry(entry: String, file: &mut File) {
    file.write(entry.as_bytes()).unwrap();
}
