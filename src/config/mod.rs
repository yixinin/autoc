use crate::acio;
pub fn load_config() {
    match acio::read_file("conf/config.toml") {
        Err(why) => panic!("load config error {}", why),
        Ok(s) => {
            println!("{}", s);
            //TODO config
            println!("load success")
        }
    }
}
