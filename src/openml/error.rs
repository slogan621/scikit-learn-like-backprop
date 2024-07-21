use std::fmt;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum DatasetError {
    IO(std::io::Error),
    Utf8(FromUtf8Error),
    Serde(serde_json::Error),
    Arff(arff::Error),
    MissingID,
}

impl From<serde_json::Error> for DatasetError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(e)
    }
}

impl From<arff::Error> for DatasetError {
    fn from(e: arff::Error) -> Self {
        Self::Arff(e)
    }
}

impl From<std::io::Error> for DatasetError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<FromUtf8Error> for DatasetError {
    fn from(e: FromUtf8Error) -> Self {
        Self::Utf8(e)
    }
}

impl fmt::Display for DatasetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatasetError::Serde(e) => write!(f, "{}", e),
            DatasetError::IO(e) => write!(f, "{}", e),
            DatasetError::Utf8(e) => write!(f, "{}", e),
            DatasetError::Arff(e) => write!(f, "{}", e),
            DatasetError::MissingID => write!(f, "No OpenML ID provided"),
        }

    }
}