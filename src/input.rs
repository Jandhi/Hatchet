use std::fs;

pub fn read_file(file_path : &str) -> String {

    match fs::read_to_string(file_path) {
        Ok(str) => str,
        Err(err) => panic!("{}", err),
    }
}