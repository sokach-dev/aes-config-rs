use aes_config::{ConfigInfo, ConfigType};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[clap(version = "1.0", author = "sokach")]
struct Cli {
    #[clap(short, long)]
    path: String,
    #[clap(short, long)]
    salt: Option<String>,
    #[clap(short, long)]
    file_type: ConfigType,
    #[clap(short, long)]
    new_file_path: String,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Encrypt,
    Decrypt,
}

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    port: u16,
    name: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Encrypt => {
            let config_info = ConfigInfo::new(cli.path, cli.salt, cli.file_type).unwrap();
            let cipher = config_info.try_encrypt_config().unwrap();
            std::fs::write(cli.new_file_path, cipher).unwrap();
            return;
        }
        Command::Decrypt => {
            let config_info = ConfigInfo::new(cli.path, cli.salt, cli.file_type).unwrap();
            let plain = config_info.try_get_config::<Config>().unwrap();
            let pretty = toml::to_string_pretty(&plain).unwrap();
            std::fs::write(cli.new_file_path, pretty).unwrap();
            return;
        }
    }
}
