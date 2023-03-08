use aes_config::{ConfigInfo, ConfigType};
use clap::{Parser, Subcommand};

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
            let plain = config_info.try_decrypt_config().unwrap();
            std::fs::write(cli.new_file_path, plain).unwrap();
            return;
        }
    }
}
