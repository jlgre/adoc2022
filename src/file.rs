use std::env;
use std::fs;

pub fn read(day: &str) -> String {
    let path = env::current_dir()
        .unwrap()
        .join("data")
        .join(format!("{}.txt", day));

    let contents = fs::read_to_string(path)
        .expect("File not found");

    contents
}
