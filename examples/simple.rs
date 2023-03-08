use aes_config::ConfigType;
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
struct Config {
    port: u16,
    name: String,
}

/*
config.toml file:
port=10086
name="test"

encrypt_config.toml file:
qOOX56hXnadNC5uHU36IgB1i/2OKgfgXz4PmDYmy683qUewVqsg=
*/

fn main() {
    let salt = "abcdefghijklmnopqrstuvwxyz123456".to_string();
    let c = aes_config::ConfigInfo::new(
        "examples/encrypt_config.toml".to_string(),
        Some(salt),
        ConfigType::TOML,
    )
    .unwrap();
    println!("{:#?}", c.try_get_config::<Config>().unwrap());
}
