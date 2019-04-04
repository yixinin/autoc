// mod config;
// mod engine;
// mod reader;
// mod threadpool;

extern crate serde;
extern crate serde_json;

use autoc::config;
use autoc::threadpool::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use autoc::acio::{reader, writer};
use autoc::engine;

fn main() {
    // config::load_config();
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // let pool = ThreadPool::new(4);

    // for stream in listener.incoming().take(2) {
    //     let stream = stream.unwrap();

    //     pool.execute(|| {
    //         handle_connection(stream);
    //     });
    // }

    // println!("Shutting down.");

    let mut buf = String::from("hello").as_bytes();

    let mut eng = engine::Engine::new("zlab.ac");
    eng.set(String::from("name"), String::from("yixin"));
    let value = eng.get(String::from("name"));
    println!("={}=", value);
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let _url = "ac://yixin:0000@127.0.0.1:95328/zlab";

    let ac = b"ac://";

    let mut contents: String;
    if !buffer.starts_with(ac) {
        contents = String::from("unkown connection")
    } else {
        //检查用户名密码
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        //TODO 检查文件是否存在
        contents = String::from("unkown connection")
    }

    let response = format!("{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
