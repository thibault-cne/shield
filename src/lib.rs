use error::Result;

pub mod cesar;
pub mod vernam;

pub mod error;

pub trait SymetricAlgorithm {
    fn encrypt(&self, plain_text: &[u8]) -> Result<Vec<u8>>;

    fn decrypt(&self, cypher_text: &[u8]) -> Result<Vec<u8>>;
}
