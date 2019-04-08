// mod config;
// mod engine;
// mod reader;
// mod threadpool;

extern crate serde;
extern crate serde_json;

use autoc::config;

use autoc::acio::{file, reader, writer};
use autoc::engine;

fn main() {
    let fns = file::get_filenames("");
    for fna in fns {
        println!("{}", fna);
    }
}

fn init() {
    let mut eng = engine::Engine::new("zlab.ac");
    for i in 1..10 {
        eng.set(format!("{}{}{}", i, i, i), format!("{}{}{}", i, i, i));
        let value = eng.get(format!("{}{}{}", i, i, i));
        println!("={}=", value);
        let fl = reader::read_len(String::from("zlab.ac")).unwrap();
        println!("file size:{}", fl);
        println!("");
    }
}
