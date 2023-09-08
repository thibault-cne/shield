use std::fmt;

pub mod cesar;

pub type Result<T> = std::result::Result<T, Error>;

pub trait SymetricAlgorithm {
    fn encrypt(&self, plain_text: &[u8]) -> Result<Vec<u8>>;

    fn decrypt(&self, cypher_text: &[u8]) -> Result<Vec<u8>>;
}

#[derive(Debug)]
pub struct Error {}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("error")
    }
}
