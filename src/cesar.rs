use super::{Result, SymetricAlgorithm};

pub struct Algorithm {
    key: Key,
}

impl Algorithm {
    pub fn new(key: usize) -> Algorithm {
        Algorithm { key: Key::new(key) }
    }

    pub fn key(&self) -> &Key {
        &self.key
    }
}

pub struct Key(u8);

impl Key {
    pub fn new(key: usize) -> Key {
        Key((key % 26) as u8)
    }

    pub fn inner(&self) -> u8 {
        self.0
    }
}

impl SymetricAlgorithm for Algorithm {
    fn encrypt(&self, plain_text: &[u8]) -> Result<Vec<u8>> {
        Ok(plain_text
            .iter()
            .map(|&b| {
                if (65..=90).contains(&b) {
                    (((b - 65) + self.key().inner()) % 26) + 65
                } else if (97..=122).contains(&b) {
                    (((b - 97) + self.key().inner()) % 26) + 97
                } else {
                    b
                }
            })
            .collect())
    }

    fn decrypt(&self, cypher_text: &[u8]) -> Result<Vec<u8>> {
        Ok(cypher_text
            .iter()
            .map(|&b| {
                if (65..=90).contains(&b) {
                    // Add 26 to avoid substraction underflow because we work
                    // with u8
                    (((b - 65) + 26 - self.key().inner()) % 26) + 65
                } else if (97..=122).contains(&b) {
                    (((b - 97) + 26 - self.key().inner()) % 26) + 97
                } else {
                    b
                }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SymetricAlgorithm;

    #[test]
    fn encrypt() {
        let (plain, cypher) = (b"this is a plain text", b"wklv lv d sodlq whaw");
        let encoder = Algorithm::new(3);

        assert_eq!(encoder.encrypt(plain).unwrap(), cypher);
    }

    #[test]
    fn decrypt() {
        let (plain, cypher) = (b"this is a plain text", b"wklv lv d sodlq whaw");
        let alg = Algorithm::new(3);

        assert_eq!(alg.decrypt(cypher).unwrap(), plain);
    }
}
