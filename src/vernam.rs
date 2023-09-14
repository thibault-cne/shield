use super::SymetricAlgorithm;

pub struct Algorithm {
    key: Vec<u8>,
}

impl Algorithm {
    pub fn new(key: Vec<u8>) -> Algorithm {
        Algorithm {
            key: key
                .into_iter()
                .map(|k| {
                    if (65..=90).contains(&k) {
                        k - 65
                    } else if (97..122).contains(&k) {
                        k - 97
                    } else {
                        k
                    }
                })
                .collect(),
        }
    }
}

impl SymetricAlgorithm for Algorithm {
    fn encrypt(&self, plain_text: &[u8]) -> crate::error::Result<Vec<u8>> {
        Ok(plain_text
            .iter()
            .zip(self.key.iter())
            .map(|(&b, k)| {
                if (65..=90).contains(&b) {
                    (((b - 65) + k) % 26) + 65
                } else if (97..=122).contains(&b) {
                    (((b - 97) + k) % 26) + 97
                } else {
                    b
                }
            })
            .collect())
    }

    fn decrypt(&self, cypher_text: &[u8]) -> crate::error::Result<Vec<u8>> {
        Ok(cypher_text
            .iter()
            .zip(self.key.iter())
            .map(|(&b, &k)| {
                if (65..=90).contains(&b) {
                    (((b - 65) + 26 - k) % 26) + 65
                } else if (97..=122).contains(&b) {
                    (((b - 97) + 26 - k) % 26) + 97
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

    #[test]
    fn encrypt() {
        let tests = vec![(b"hello", b"wmckl", b"dqnvz")];

        tests.into_iter().for_each(|(plain, key, expect)| {
            let alg = Algorithm::new(key.to_vec());

            assert_eq!(&alg.encrypt(plain).unwrap(), expect);
        });
    }
}
