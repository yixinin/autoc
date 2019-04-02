pub mod acon;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

pub struct ValueInfo {
   pub length: u128,
   pub start: u128,
}
use acon::Acon;

pub struct Engine {
   pub filename: String,
   pub keymap: HashMap<String, ValueInfo>,
}

impl Engine {
   pub fn new(filename: &str) -> Engine {
      Engine {
         filename: String::from(filename),
         keymap: HashMap::new(),
      }
   }

   pub fn set<T>(&mut self, key: String, value: T)
   where
      T: Acon<T>,
   {
      self.keymap.insert(
         key,
         ValueInfo {
            length: 0,
            start: 0,
         },
      );
      let s = value.to_string();
      println!("{}", s)
   }

   pub fn get<T>(self, key: String) -> T
   where
      T: Acon<T>,
   {
      T::from_string(key)
   }
}
