
use reader;

pub struct UserConfig{
    UserName: String
    Password: String
}

pub fn load_config(){
    let text = reader::read_file("conf/config.toml")
    if text == String::empty(){
        panic!("config not found !")
    }
    
}