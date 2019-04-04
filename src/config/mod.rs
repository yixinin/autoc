use crate::acio::reader;
pub fn load_config() {
    match reader::read_file("conf/config.toml") {
        Err(why) => panic!("load config error {}", why),
        Ok(s) => {
            println!("{}", s);
            //TODO config
            println!("load success")
        }
    }
}
