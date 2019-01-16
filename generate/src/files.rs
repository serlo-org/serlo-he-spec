use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct GenerationError {
    /// why file generation has failed.
    pub msg: String,
}

impl GenerationError {
    pub fn new(message: String) -> Self {
        Self { msg: message }
    }
}

impl fmt::Display for GenerationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for GenerationError {
    fn description(&self) -> &str {
        &self.msg
    }
}

/// A generated file with its generated contents.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedFile {
    pub path: PathBuf,
    pub content: String,
}
