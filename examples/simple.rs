use aes_config::ConfigType;
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
struct Config {
    port: u16,
    name: String,
}

fn main() {
    let c = aes_config::ConfigInfo::new("config.toml".to_string(), None, ConfigType::TOML).unwrap();
    println!("{:#?}", c.try_get_config::<Config>().unwrap());
}
