use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//直接写入
pub fn write(filename: String, value: &[u8]) -> Result<bool, std::io::Error> {
    let path = Path::new(&filename);
    match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
    {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => Err(why),
        Ok(ref mut file) => {
            file_write(file, value, 0)?;
            Ok(true)
        }
    }
}
//在指定位置写入
pub fn write_at(filename: String, value: &[u8], offset: u64) -> Result<bool, std::io::Error> {
    let path = Path::new(&filename);
    match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
    {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => Err(why),
        Ok(ref mut file) => {
            file_write(file, value, offset)?;
            Ok(true)
        }
    }
}

//追加写入
pub fn write_append(filename: String, value: &[u8]) -> Result<usize, std::io::Error> {
    let path = Path::new(&filename);
    match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
    {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => Err(why),
        Ok(ref mut file) => file_append(file, value),
    }
}

fn file_write(file: &mut File, value: &[u8], offset: u64) -> Result<bool, std::io::Error> {
    if offset != 0 {
        file.seek(std::io::SeekFrom::Start(offset)).unwrap();
    }

    match file.write(value) {
        Err(why) => {
            println!("write file error {}", why);
            Err(why)
        }
        Ok(_) => {
            file.flush().ok();
            Ok(true)
        }
    }
}

fn file_append(file: &mut File, value: &[u8]) -> Result<usize, std::io::Error> {
    match file.write(value) {
        Err(why) => {
            println!("write file error {}", why);
            Err(why)
        }
        Ok(off) => {
            file.flush().ok();
            Ok(off)
        }
    }
}
