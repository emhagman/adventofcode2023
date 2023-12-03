use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn read_entire_file(filename: &str) -> String {
    let file = File::open(filename).expect("failed to read file");
    let mut buf = BufReader::new(file);
    let mut output = String::from("");
    buf.read_to_string(&mut output).expect("failed to read into string");
    output
}
