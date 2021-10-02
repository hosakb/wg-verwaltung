use std::hash;

use argon2::{self, Config};
use rand::Rng;

fn hash(pwd: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(pwd, &salt, &config).unwrap()
}

fn verify(hash: &str, pwd: &[u8]) -> bool {
    argon2::verify_encoded(hash, pwd).unwrap_or(false)
}

fn main() {
    let pw = b"4321";

    let hash1 = hash(pw);
    println!("{}", hash1);
    println!("Verify: {}", verify(hash1.as_str(), pw));

    let hash1 = hash(pw);
    println!("{}", hash1);
    println!("Verify: {}", verify(hash1.as_str(), pw));
    


}
