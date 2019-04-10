use autoc::threadpool::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::util;
//监听
/*
    header+app name + opt  + parameters
    header= 1byte应用名字 + 1byte操作长度 + 4byte 参数长度


*/
fn handle_connection(mut stream: TcpStream) {
    let mut headbuf = [0u8;6];
    stream.read(&mut headbuf).unwrap();


    

    let applen = headbuf[0] as usize; // app名称长度
    let oplen = headbuf[1] as usize; // 操作长度

    let datalen ==  unsafe { std::mem::transmute::<[u8; 4], u32>(headbuf[2..]) }; //数据长度
    
    let mut appbuf:Vec<u8> = Vec::with_capacity(datalen);
     let mut opbuf:Vec<u8> = Vec::with_capacity(datalen);

    let mut databuf:Vec<u8> = Vec::with_capacity(datalen);
    util.fill_vec(&mut appbuf,applen);
    util.fill_vec(&mut opbuf,oplen);
    util.fill_vec(&mut databuf,datalen); 
    
    stream.read_exact(&mut appbuf).unwrap(); 
    stream.read_exact(&mut opbuf).unwrap();
    stream.read_exact(&mut databuf).unwrap();
 
    let app=String::from_utf8_lossy(&appbuf[..]).to_string();
    let op = String::from_utf8_lossy(&opbuf[..]).to_string();
    let data = String::from_utf8_lossy(&databuf[..]).to_string();
    let response = format!("{}", contents); 

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    while stream.
}


fn listen(ip:&str,port :u32){
    let listener = TcpListener::bind(ip+port.as_str()).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) { 
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

//心跳 💓
fn keep_alive(){

}