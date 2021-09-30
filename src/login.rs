use argon2::{self, Config};
use rand::Rng;

pub struct Login {
    username: String,
    password: String,
}

impl Login {
    pub fn new(username: String, password: String) -> Login{
        Login {
            username: username,
            password: password,
        }
    }
}

pub fn hash(pwd: &[u8]) -> String{
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(pwd, &salt, &config).unwrap()
}

pub fn verify(hash: &str, pwd: &[u8]) -> bool {
    argon2::verify_encoded(hash, pwd).unwrap_or(false)
}

pub fn login_user(){
    
}

pub fn check_username(){
    
}