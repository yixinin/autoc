use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//读取文本内容
pub fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let path = Path::new(&filename);
    // Open the path in read-only mode, returns `io::Result<File>`
    match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => Err(why),
        Ok(mut file) => {
            // Read the file contents into a string, returns `io::Result<usize>`
            let mut s: String = String::new();
            file.read_to_string(&mut s).unwrap();
            Ok(s)
        }
    }
}

pub fn read(filename: String, size: usize) -> Result<Vec<u8>, std::io::Error> {
    let path = Path::new(&filename);
    // Open the path in read-only mode, returns `io::Result<File>`
    match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => Err(why),
        Ok(mut file) => {
            // Read the file contents into a string, returns `io::Result<usize>`
            let mut buf: Vec<u8> = Vec::with_capacity(size);
            file.read_to_end(&mut buf).unwrap();
            Ok(buf)
        }
    }
}

pub fn write(filename: String, value: &[u8]) -> Result<bool, std::io::Error> {
    let path = Path::new(&filename);
    match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(error) => {
            if error.kind() == std::io::ErrorKind::NotFound {
                // 文件不存在 创建
                match File::create(path) {
                    Err(why) => Err(why), //创建文件失败
                    Ok(ref mut file) => {
                        file_write(file, value);
                        Ok(true)
                    }
                }
            } else {
                //其他错误 返回
                Err(error)
            }
        }
        Ok(ref mut file) => {
            file_write(file, value);
            Ok(true)
        }
    }
}

fn file_write(file: &mut File, value: &[u8]) {
    match file.write(value) {
        Err(_) => (),
        Ok(_) => {
            file.flush().ok();
        }
    }
}
