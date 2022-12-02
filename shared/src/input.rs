use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn get_input_from_file(input: &str) -> String {
    get_input_text_from_file(get_file_buffer(input))
}

pub fn get_input_text_from_file<R: Read>(mut input: R) -> String {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn get_file_buffer(path: &str) -> BufReader<File> {
    return BufReader::new(File::open(&path).expect(&format!("file not found at '{}'", path)));
}
