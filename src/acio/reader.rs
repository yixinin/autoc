use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::util::fill_vec;
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
            file.read_to_string(&mut s)?;
            Ok(s)
        }
    }
}

pub fn read_acm(filename: String) -> Result<(usize, Vec<u8>), std::io::Error> {
    let path = Path::new(&filename);
    // Open the path in read-only mode, returns `io::Result<File>`
    match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => Err(why),
        Ok(mut file) => {
            //读取当前游标
            let mut hbuf = [0u8; 8];
            match file.read_exact(&mut hbuf) {
                Err(e) => Err(e),
                Ok(_) => {
                    //读取key长度
                    let mut hbuf = [0u8; 8];
                    match file.read_exact(&mut hbuf) {
                        Err(e) => Err(e),
                        Ok(_) => {
                            let size = unsafe { std::mem::transmute::<[u8; 8], usize>(hbuf) };
                            let mut buf: Vec<u8> = Vec::with_capacity(size);
                            match file.read_to_end(&mut buf) {
                                Err(why) => Err(why),
                                Ok(_) => Ok((size, buf)),
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read(filename: &String) -> Result<Vec<u8>, std::io::Error> {
    let path = Path::new(&filename);
    // Open the path in read-only mode, returns `io::Result<File>`
    match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => Err(why),
        Ok(mut file) => {
            //读取数据头
            let mut hbuf = [0u8; 8];
            match file.read_exact(&mut hbuf) {
                Err(e) => Err(e),
                Ok(_) => {
                    let size = unsafe { std::mem::transmute::<[u8; 8], usize>(hbuf) };
                    let mut buf: Vec<u8> = Vec::with_capacity(size);
                    match file.read_to_end(&mut buf) {
                        Err(why) => Err(why),
                        Ok(_) => Ok(buf),
                    }
                }
            }
        }
    }
}

pub fn read_at(filename: &String, offset: u64) -> Result<Vec<u8>, std::io::Error> {
    let path = Path::new(&filename);
    // Open the path in read-only mode, returns `io::Result<File>`
    match std::fs::OpenOptions::new().read(true).open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => Err(why),
        Ok(mut file) => {
            let mut hbuf = [0u8; 4];
            match file.read_exact(&mut hbuf) {
                Err(e) => Err(e),
                Ok(_) => {
                    if offset != 0 {
                        let cursor = file.seek(std::io::SeekFrom::Current(offset as i64))?;
                        println!("curosr:{}", cursor);
                    }
                    let size = unsafe { std::mem::transmute::<[u8; 4], u32>(hbuf) };
                    println!("buf len {}", size);
                    let mut buf: Vec<u8> = Vec::with_capacity(size as usize);

                    fill_vec(&mut buf, size as usize);
                    let cursor = file.seek(std::io::SeekFrom::Current(0))?;
                    println!("curosr:{}", cursor);
                    match file.read_exact(&mut buf) {
                        Err(why) => Err(why),
                        Ok(_) => Ok(buf),
                    }
                }
            }
        }
    }
}

pub fn read_len(filename: String) -> Result<usize, std::io::Error> {
    let path = Path::new(&filename);
    match std::fs::metadata(path) {
        Err(e) => Err(e),
        Ok(metadata) => Ok(metadata.len() as usize),
    }
}
