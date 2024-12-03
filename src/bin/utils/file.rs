use std::fs::File;
use std::io::Read;

pub fn read_content(path: &str) -> String {
    let mut input = File::open(path).unwrap();
    let buf = &mut String::new();
    input.read_to_string(buf).unwrap();
    buf.to_string()
}

