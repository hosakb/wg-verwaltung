mod aufgabe;
mod db;
mod login;

extern crate chrono;

use chrono::{NaiveDate, ParseError};
use core::panic;
use std::{clone, collections::HashMap, env, io};

use anyhow::{Context, Result};
use thiserror::Error;

use crate::db::Bewohner;

pub fn app() {
    let db = db::read_db();
    present_login(&db);
}

fn present_login(db: &Vec<Bewohner>) -> Option<Bewohner> {
    let mut username = String::from("");
    let mut password = String::from("");
    let mut bewohner: Option<&Bewohner>;

    println!("Willkommen zu Skittles. Der professionellen WG-Verwaltungssoftware, welche es dir leicht macht, deine WG zu verwalten.");
    println!("--Login--");
    loop {
        print!("Username: ");
        io::stdin().read_line(&mut username).unwrap();

        if username.is_empty() {
            return None;
        }

        bewohner = db.iter().find(|&b| b.username.eq(username.trim()));
        match bewohner {
            Some(b) => break,
            None => eprintln!("Username existiert nicht!"),
        }
    }

    match bewohner {
        Some(b) => loop {
            let c = b.clone();
            print!("Passwort: ");
            io::stdin().read_line(&mut password).unwrap();
            if b.passwort.eq(password.trim()) {
                return Some(b);
            } else if password.trim().is_empty() {
                return None;
            }
            println!("Passwort ist falsch!");
        },
        None => panic!("Bewohner is None"),
    }
}

fn interp_geld() {
    let mut str = String::new();
    loop {
        println!("Du hast gewählt, dass du etwas mit Geld tun möchtest. Was möchtest du tun?");
        println!("\tzurück\n\tzeige Nutzer an\n\tfüge generelle Ausgabe hinzu");

        str.clear();
        io::stdin()
            .read_line(&mut str)
            .expect("You must put in a command");

        match str.trim() {
            "zurück" => break,
         //   "generelle Ausgabe" => geld::generelle_ausgabe(),
            "geld" => interp_geld(),
            _ => println!("Dieser Befehl existiert nicht. Bitte überlege noch einmal, welche Entscheidungen dich an diesen Punkt gebracht haben."),
        }
    }
}

fn interp_kalender() {}

fn interp_bewohner() {
    println!("Bewohner")
}
fn interp_aufgaben() {
    loop {
        let mut str = format!("Wilkommen bei der AUfgabenverteilung\nDie folgenden Funbktionen stehen dir 
        heute zur verfügung:\n\n\t(1) Neue Aufgabe erstellen\n\t(2) Aufgabe beenden\n\t(3) Aufgabenübersicht");
        print!("{}", &str);
        io::stdin()
            .read_line(&mut str)
            .expect("Yout must enter a command!");

        // match str.trim() {
        //     "zurück" => break,
        //     "1" => aufgabe_erstellen(),
        //     "2" => aufgabe_beenden(),
        //     "3" => aufgabe_anzeigen(),
        //     _ => println!("Dieser Befehl existiert nicht. Bitte überlege noch einmal, welche Entscheidungen dich an diesen Punkt gebracht haben."),
        // }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn test_present_login() {
        let bewohner = Bewohner {
            id: 0,
            name: String::from("Ben"),
            bday: NaiveDate::parse_from_str("1998-02-10", "%Y-%m-%d").unwrap(),
            admin: true,
            username: String::from("hosakb"),
            passwort: String::from("1234"),
        };
        let bewohner_vec = vec![bewohner];
        let login = *present_login(&bewohner_vec).unwrap();

       assert_eq!(bewohner, login);

    }
}
