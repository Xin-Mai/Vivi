use chrono::prelude::Utc;
use hmac::{Hmac, NewMac};
use jwt::{SignWithKey, VerifyWithKey, Error};
use sha2::Sha256;
use std::collections::BTreeMap;

pub struct Signer {
    pub key: Hmac<Sha256>,
}

impl Signer {
    pub fn new() -> Signer {
        let content = std::fs::read("key").unwrap();
        Signer {
            key: Hmac::new_varkey(&content).unwrap(),
        }
    }

    pub fn sign(&self, name: &str) -> String {
        let mut claims = BTreeMap::new();
        claims.insert("token", name);
        let time = Utc::now().to_string();
        claims.insert("time", &time);
        println!("Sign with time: {}", &time);
        claims.sign_with_key(&self.key).unwrap()
    }

    pub fn verify(&self, token: &str) -> Result<(String, String),Error> {
        let claims: BTreeMap<String,String> = token.verify_with_key(&self.key)?;
        Ok((claims["token"].clone(), claims["time"].clone()))
    }
}
