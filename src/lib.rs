mod aufgabe;

extern crate chrono;

use chrono::{NaiveDate, ParseError};
use std::{env, io};

use thiserror::Error;
use anyhow::{Context, Result};

pub struct Befehl {
    pub high_level: String,
    pub low_level: Option<String>,
}

impl Befehl {
    pub fn new(mut args: env::Args) -> Result<Befehl, &'static str> {
        //TODO Fehlermeldungen

        args.next();

        let high_level = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not find high level command"),
        };

        let low_level = match args.next() {
            Some(arg) => arg,
            None => {
                return Ok(Befehl {
                    high_level,
                    low_level: None,
                })
            }
        };

        Ok(Befehl {
            high_level,
            low_level: Some(low_level),
        })
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
            "generelle Ausgabe" => interp_kalender(),
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
