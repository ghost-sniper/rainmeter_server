extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};

fn encrypt(plain_text: String) -> String {
    return hash(plain_text, DEFAULT_COST).unwrap();
}

fn decrypt(cipher_text: String, password: String) -> bool {
    return verify(password, cipher_text.as_str()).unwrap();
}