use crate::reader;
pub fn load_config() {
    let text = reader::read_file("conf/config.toml");
    if text == String::from("") {
        panic!("config not found !")
    }
}
