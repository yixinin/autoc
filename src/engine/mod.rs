pub mod acon;

use crate::acio::{reader, writer};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

use acon::Acon;
#[derive(Debug, Copy, Clone)]
pub struct Engine {
   pub filename: String,
   pub current: usize,
   pub keymap: HashMap<String, usize>,
}

impl Engine {
   pub fn new(filename: &str) -> Engine {
      Engine {
         filename: String::from(filename),
         keymap: HashMap::new(),
         current: match reader::read_len(String::from(filename)) {
            Err(_) => 0,
            Ok(size) => size,
         },
      }
   }

   //存储 key:文件名称 value 文件类容
   pub fn set(&mut self, key: String, value: String) {
      let data = value.as_bytes();
      let length = data.len();

      let mut buf: Vec<u8> = Vec::with_capacity(length + 4);

      let lbuf = unsafe { std::mem::transmute::<u32, [u8; 4]>(length as u32) };
      for b in &lbuf {
         buf.push(*b);
      }

      for b in data {
         buf.push(*b);
      }

      println!("{:?}", &buf);
      match writer::write_append(self.filename.clone(), &buf) {
         Err(why) => println!("write file error {}", why),
         Ok(_) => {
            self.keymap.insert(key, self.current);
            self.current += length + 4;
         }
      };
   }

   pub fn get(self, key: String) -> String {
      match self.keymap.get(&key) {
         None => String::from(""),
         Some(off) => match reader::read_at(self.filename, *off as u64) {
            Err(_) => String::from(""),
            Ok(value) => String::from_utf8_lossy(&value).as_ref().to_string(),
         },
      }
   }
}
impl Copy for Engine {}
impl Clone for Engine {
   fn clone(&self) -> Engine {
      *self
   }
}
