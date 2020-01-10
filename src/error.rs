use std::io;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AppError {
    IOError(String),
    ConfigError(String),
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::IOError(e.to_string())
    }
}

impl From<toml::de::Error> for AppError {
    fn from(e: toml::de::Error) -> Self {
        AppError::ConfigError(e.to_string())
    }
}
