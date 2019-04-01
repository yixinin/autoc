use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//读取文本内容
pub fn read_file(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            //return s
            panic!("couldn't read {}: {}", display, why.description());
        }
        Ok(_) => println!("found"),
    }
    s
}
