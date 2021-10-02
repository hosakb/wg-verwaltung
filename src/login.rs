use argon2::{self, Config};
use rand::Rng;
use std::io;

use crate::db::Bewohner;

fn hash(pwd: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(pwd, &salt, &config).unwrap()
}

fn verify(hash: &str, pwd: &[u8]) -> bool {
    argon2::verify_encoded(hash, pwd).unwrap_or(false)
}

pub fn login_user<'a>(db: &'a Vec<Bewohner>) -> Option<&'a Bewohner> {
    println!("Willkommen zu Skittles. Der professionellen WG-Verwaltungssoftware, welche es dir leicht macht, deine WG zu verwalten.");
    println!("--Login--");

    let bewohner = check_username(db);
    match bewohner {
        Some(b) => return check_password(b),
        None => return None,
    }
}

fn check_username<'a>(db: &'a Vec<Bewohner>) -> Option<&'a Bewohner> {
    let mut username = String::from("");

    loop {
        print!("Username: ");
        io::stdin().read_line(&mut username).unwrap();
        if username.trim().is_empty() {
            return None;
        }

        let bewohner = db.into_iter().find(|b| b.username.eq(username.trim()));
        match bewohner {
            Some(b) => return Some(b),
            None => eprintln!("Username existiert nicht!"),
        }
    }
}

fn check_password<'a>(bewohner: &'a Bewohner) -> Option<&'a Bewohner> {
    let mut password = String::from("");

    loop {
        print!("Passwort: ");
        io::stdin().read_line(&mut password).unwrap();
       
        if verify(bewohner.passwort.as_str(), password.trim().as_bytes()) {
            return Some(bewohner);
        } else if password.trim().is_empty() {
            return None;
        }
        println!("Passwort ist falsch!");
    }
}

#[cfg(test)]
mod test {

    use chrono::NaiveDate;

    use super::*;

    fn create_user() -> Bewohner{
        Bewohner {
            id: 0,
            name: String::from("Ben"),
            bday: NaiveDate::parse_from_str("1998-02-10", "%Y-%m-%d").unwrap(),
            admin: true,
            username: String::from("hosakb"),
            passwort: String::from("$argon2i$v=19$m=4096,t=3,p=1$V43VlUIfE5+CmQk9smoYjnqCbdEVo4/fFnbzfhWE3E4$vr6PWVPVfN3CnFr6j9Nc5wgW0JeujtX2PWSpUMOvLbY"),
        }
    }

    fn create_user_list() -> Vec<Bewohner>{
        
        let bewohner = create_user();
        vec![bewohner]
    }

    #[test]
    fn test_login_user() {
        
        let v = create_user_list();
        assert!(login_user(&v).is_some());
    }

    #[test]
    fn test_login_user_empty() {
        
        let v = create_user_list();
        assert!(login_user(&v).is_none());
    }

    #[test]
    fn test_check_username() {
        
        let v = create_user_list();
        assert!(check_username(&v).is_some());
    }

    #[test]
    fn test_check_username_empty() {
        
        let v = create_user_list();
        assert!(check_username(&v).is_none());
    }

    #[test]
    fn test_check_password() {
        
        let v = create_user();
        assert!(check_password(&v).is_some());
    }

    #[test]
    fn test_check_password_empty() {
        
        let v = create_user();
        assert!(check_password(&v).is_none());
    }
}
