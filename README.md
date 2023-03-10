# aes-config-rs

The Rust project is configured with an AES encryption function.

## usage

### cmd

You can pass all parameters through optional flags
```bash
# encrypt config file
./target/release/config_cli  -p config.toml -s "abcdefghijklmnopqrstuvwxyz123456" -n encrypt_config.toml  -f toml encrypt
# decrypt config file
./target/release/config_cli  -p encrypt_config.toml -s "abcdefghijklmnopqrstuvwxyz123456" -n encrypt_config.plain.toml  -f toml decrypt
```

You can also pass the key through the environment variable `AES_CONFIG_KEY`.
```bash
export AES_CONFIG_KEY=abcdefghijklmnopqrstuvwxyz123456 
# encrypt config file
./target/release/config_cli  -p config.toml -n encrypt_config.toml  -f toml encrypt
# decrypt config file
./target/release/config_cli  -p encrypt_config.toml -n encrypt_config.plain.toml  -f toml decrypt
```

### as crate

```rust
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

```
