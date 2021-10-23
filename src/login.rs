use argon2::{self, Config};
use rand::Rng;
use std::io;
use chrono::NaiveDate;

use crate::db;
use crate::db::bewohner::Bewohner;

fn hash(pwd: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(pwd, &salt, &config).unwrap()
}

fn verify(hash: &str, pwd: &[u8]) -> bool {
    argon2::verify_encoded(hash, pwd).unwrap_or(false)
}

pub fn login_user<'a>(bewohner_db: &'a Vec<Bewohner>) -> Result<Option<&'a Bewohner>, io::Error> {
    
    println!("Willkommen zu Skittles. Der professionellen WG-Verwaltungssoftware, welche es dir leicht macht, deine WG zu verwalten.");
    println!("--Login--");

    let bewohner = check_username(bewohner_db)?;

    match bewohner {
        Some(b) => return Ok(check_password(b)?),
        None => return Ok(None),
    }
}

fn check_username<'a>(bewohner_db: &'a Vec<Bewohner>) -> Result<Option<&'a Bewohner>, io::Error> {
    let mut username = String::from("");

    loop {
        username.clear();

        println!("Bitte Username eingeben! Für abbruch Eingabe drücken!");
        print!("Username: ");
        io::Write::flush(&mut io::stdout())?;
        io::stdin().read_line(&mut username)?;
       
        if username.trim().is_empty() {
            return Ok(None);
        }

        let bewohner = bewohner_db.into_iter().find(|b| b.username.eq(username.trim()));
       
        match bewohner {
            Some(b) => return Ok(Some(b)),
            None => eprintln!("---Username existiert nicht!---"),
        }
    }
}

fn check_password<'a>(bewohner: &'a Bewohner) -> Result<Option<&'a Bewohner>, io::Error> {
    let mut password = String::from("");

    loop {
        password.clear();

        print!("Passwort: ");
        io::Write::flush(&mut io::stdout())?;
        io::stdin().read_line(&mut password)?;
       
        if verify(bewohner.passwort.as_str(), password.trim().as_bytes()) {
            println!("Login erfolgreich!");
            return Ok(Some(bewohner));
        } else if password.trim().is_empty() {
            return Ok(None);
        }
        println!("---Passwort ist falsch!---");
    }
}

pub fn erster_bewohner() -> Result<Vec<Bewohner>, Box<dyn std::error::Error>>{ // Safer data input
    
    println!("Wilkommen...");

    println!("Wie heißt du?");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)?;

  
        
    println!("Nutzername");
    let mut nutzername = String::new();
    io::stdin()
        .read_line(&mut nutzername)?;
        
    println!("Passwort");    
    let mut passwort = String::new();
    io::stdin()
        .read_line(&mut passwort)?;
    let passwort = hash(passwort.as_bytes());
    
    
    
    let mut tag = String::new(); // Corner cases
    let mut monat = String::new();
    let mut jahr = String::new();

    println!("Geburtstag");

    println!("Tag");
    io::stdin()
        .read_line(&mut tag)?;
    
    let tag: u32 = tag.trim().parse().unwrap(); //unwrap

    println!("Monat");
    io::stdin()
        .read_line(&mut monat)?;

    let monat: u32 = monat.trim().parse().unwrap(); //unwrap


    println!("Jahr");
    io::stdin()
        .read_line(&mut jahr)?;

    let jahr: i32 = jahr.trim().parse().unwrap(); //unwrap


    let bd = NaiveDate::from_ymd(jahr, monat, tag);

    let bewohner = db::bewohner::create_bewohner(name.trim().to_string(), nutzername.trim().to_string(), passwort.trim().to_string(), true, bd)?;

    Ok(vec![bewohner])
    
}

#[cfg(test)]
mod test {

    use chrono::NaiveDate;

    use super::*;

    fn create_user() -> Bewohner{
        Bewohner {
            id: 0,
            name: String::from("Ben"),
            admin: true,
            username: String::from("hosakb"),
            passwort: String::from("$argon2i$v=19$m=4096,t=3,p=1$V43VlUIfE5+CmQk9smoYjnqCbdEVo4/fFnbzfhWE3E4$vr6PWVPVfN3CnFr6j9Nc5wgW0JeujtX2PWSpUMOvLbY"),
            birthday: NaiveDate::from_ymd(1998, 2, 10),
        }
    }

    fn create_user_vec() -> Vec<Bewohner>{
        vec![create_user()]
    }

    #[test]
    fn test_check_login(){
       
        let user = create_user_vec();
        let res = login_user(&user).unwrap();
        assert!(res.is_some());
    }



    #[test]
    fn test_check_username(){
       
        let user = create_user_vec();
        let res = check_username(&user).unwrap();
        assert!(res.is_some());
    }

    #[test]
    fn test_exit_username(){
       
        let user = create_user_vec();
        let res = check_username(&user).unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn test_check_pw(){
       
        let user = create_user();
        let res = check_password(&user).unwrap();
        assert!(res.is_some());
    }

    #[test]
    fn test_exit_pw(){
       
        let user = create_user();
        let res = check_password(&user).unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn erster_bewohner_erstellt() {
        let bewohner = erster_bewohner().unwrap();
        assert!(!bewohner.is_empty())
    }
}
