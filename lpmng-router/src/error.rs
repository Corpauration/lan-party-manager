pub type Result<Ok> = core::result::Result<Ok, Error>;

#[allow(dead_code, clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Error {
    SerdeError(serde_json::Error),
    IoError(std::io::Error),
    CommandError(Option<i32>, String),
    NoResult,
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::SerdeError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}
