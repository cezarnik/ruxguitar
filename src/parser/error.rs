use std::io;

#[derive(Debug, thiserror::Error)]
pub enum RuxError {
    #[error("iced error: {0}")]
    IcedError(iced::Error),
    #[error("configuration error: {0}")]
    ConfigError(String),
    #[error("parsing error: {0}")]
    ParsingError(String),
    #[error("other error: {0}")]
    OtherError(String),
}

impl From<iced::Error> for RuxError {
    fn from(error: iced::Error) -> Self {
        Self::IcedError(error)
    }
}

impl From<io::Error> for RuxError {
    fn from(error: io::Error) -> Self {
        Self::OtherError(error.to_string())
    }
}