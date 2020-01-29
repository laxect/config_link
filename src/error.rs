use async_std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvFileError {
    #[error("Io error: `{0}`")]
    IoError(#[from] io::Error),
    #[error("Yaml error: `{0}`")]
    TomlError(#[from] toml::de::Error),
}

pub type Result<T> = std::result::Result<T, EnvFileError>;
