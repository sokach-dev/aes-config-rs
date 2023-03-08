use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::engine::general_purpose;
use base64::Engine;
use serde::de::DeserializeOwned;
use std::{fmt::Debug, str::FromStr, string::FromUtf8Error};

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("get path error")]
    PathError(#[from] std::io::Error),

    #[error("parse toml error")]
    TomlError(#[from] toml::de::Error),

    #[error("encrypt or decrypt error")]
    AesError(aes_gcm::aead::Error),

    #[error("string from utf8 error")]
    StringFromUtf8Error(#[from] FromUtf8Error),

    #[error("unknown config type: {0}")]
    UnknownConfigType(String),

    #[error("salt need 32 bity")]
    SaltLenError,

    #[error("base64 decode error")]
    Base64Error(#[from] base64::DecodeError),
}

impl From<aes_gcm::aead::Error> for ConfigError {
    fn from(e: aes_gcm::aead::Error) -> Self {
        Self::AesError(e)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ConfigType {
    TOML,
    JSON,
    INI,
}

#[derive(Debug)]
pub struct ConfigInfo {
    path: String,
    salt: Option<String>,
    file_type: ConfigType,
}

const FIXED_NONCE: [u8; 12] = [
    0x12, 0x34, 0x56, 0x70, 0x9a, 0xba, 0x99, 0xf9, 0x12, 0x34, 0x56, 0x78,
];

impl FromStr for ConfigType {
    type Err = ConfigError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "toml" => Ok(ConfigType::TOML),
            "json" => Ok(ConfigType::JSON),
            "ini" => Ok(ConfigType::INI),
            unknown => Err(ConfigError::UnknownConfigType(unknown.to_string())),
        }
    }
}

impl ConfigInfo {
    pub fn new(
        path: String,
        salt: Option<String>,
        file_type: ConfigType,
    ) -> Result<Self, ConfigError> {
        if let Some(key) = salt.clone() {
            if key.len() != 32 {
                return Err(ConfigError::SaltLenError);
            }
        }

        Ok(Self {
            path,
            salt,
            file_type,
        })
    }

    pub fn try_get_config<T: DeserializeOwned>(&self) -> Result<T, ConfigError> {
        let config_string = self.try_decrypt_config()?;

        match self.file_type {
            ConfigType::TOML => {
                let t: T = toml::from_str(&config_string)?;
                Ok(t)
            }
            ConfigType::JSON => {
                todo!()
            }
            ConfigType::INI => {
                todo!()
            }
        }
    }

    pub fn try_encrypt_config(&self) -> Result<String, ConfigError> {
        let config_string = std::fs::read_to_string(&self.path)?;
        if let Some(salt) = &self.salt {
            let salt = salt.as_bytes().try_into().unwrap();
            return encrypt_config(config_string.as_bytes(), salt);
        } else {
            return Err(ConfigError::SaltLenError);
        }
    }

    pub fn try_decrypt_config(&self) -> Result<String, ConfigError> {
        let config_string = std::fs::read_to_string(&self.path)?;
        if let Some(salt) = &self.salt {
            let salt = salt.as_bytes().try_into().unwrap();
            let plain = decrypt_config(config_string, salt)?;
            let config_string = String::from_utf8(plain)?;
            return Ok(config_string);
        } else {
            return Ok(config_string);
        }
    }
}

fn encrypt_config(config: &[u8], key: &[u8; 32]) -> Result<String, ConfigError> {
    let key = GenericArray::from_slice(key);
    let nonce = Nonce::from_slice(&FIXED_NONCE);
    let cipher = Aes256Gcm::new(key).encrypt(nonce, config)?;
    Ok(general_purpose::STANDARD.encode(cipher))
}

fn decrypt_config(cipher: String, key: &[u8; 32]) -> Result<Vec<u8>, ConfigError> {
    let cipher = general_purpose::STANDARD.decode(cipher)?;
    let key = GenericArray::from_slice(key);
    let nonce = Nonce::from_slice(&FIXED_NONCE);
    let plain = Aes256Gcm::new(key).decrypt(nonce, cipher.as_slice())?;
    Ok(plain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_config_should_work() {
        let config = "hello world";
        let key = [0u8; 32];
        let cipher = encrypt_config(config.as_bytes(), &key).unwrap();
        let plain = decrypt_config(cipher, &key).unwrap();
        assert_eq!(config.as_bytes(), plain.as_slice());
        assert_eq!(config, String::from_utf8(plain).unwrap());
    }
}
