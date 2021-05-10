use chrono::prelude::Utc;
use hmac::{Hmac, NewMac};
use jwt::{Error, SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

lazy_static! {
    static ref SIGNER: Signer = Signer::new();
}

struct Signer {
    key: Hmac<Sha256>,
}

impl Signer {
    fn new() -> Signer {
        let content = std::fs::read("key").unwrap();
        Signer {
            key: Hmac::new_varkey(&content).unwrap(),
        }
    }

    fn sign(&self, id: &str) -> String {
        let mut claims = BTreeMap::new();
        claims.insert("token", id);
        let time = Utc::now().to_string();
        claims.insert("time", &time);
        println!("Sign with id = {}, time: {}", id, &time);
        claims.sign_with_key(&self.key).unwrap()
    }

    fn verify(&self, token: &str) -> Result<(String, String), Error> {
        let claims: BTreeMap<String, String> = token.verify_with_key(&self.key)?;
        Ok((claims["token"].clone(), claims["time"].clone()))
    }
}

pub fn sign(id: &str) -> String {
    SIGNER.sign(id)
}

pub fn verify(token: &str) -> Result<(String, String), Error> {
    SIGNER.verify(token)
}
