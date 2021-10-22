mod aufgabe;
mod bewohner;
mod db;
mod einkauf;
mod geld;
mod kalender;
mod login;
mod putzplan;

extern crate chrono;

use chrono::{NaiveDate, ParseError};
use core::panic;
use login::login_user;
use std::{clone, collections::HashMap, env, io, option};

use anyhow::{Context, Result};
use thiserror::Error;

use crate::db::Bewohner;

pub fn run(){
    
    let db = db::read_bewohner().unwrap_or_else(|err| panic!("Something happened while reading the Datatabse:\n{}", err));

    let db = match db {
        Some(v) => v,
        None => todo!(),                 //TODO: implement or panic
    };

    let bewohner_option = login::login_user(&db).unwrap_or_else(|err| panic!("Something happened during login:\n{}", err));

    let bewohner = match bewohner_option{
        Some(b) => b,
        None => return,
    };
    options(bewohner);
}

fn options(bewohner: &Bewohner) {
    let mut option = String::from("");
    let admin = bewohner.admin;

    println!("Hallo {}, was möchtest du machen?", bewohner.name);

    loop {
        option.clear();
        print_options(bewohner);
        io::stdin()
            .read_line(&mut option)
            .expect("Input could not be read!");

        let o = option.as_str();
        match o {
            "1" => kalender::kalender_options(bewohner),
            "2" => einkauf::einkaufsliste_options(bewohner),
            "3" => putzplan::putzplan_options(bewohner),
            "4" => geld::finanzen_options(bewohner),
            "5" => aufgabe::aufgaben_options(bewohner),
            "6" => return,
            _ => {
                if admin {
                    match o {
                        "7" => bewohner::bewohnerveraltung_options(),
                        _ => eprint!("---Option nicht verfügbar---"),
                    }
                } else {
                    eprint!("---Option nicht verfügbar---")
                }
            }
        }
    }
}

fn print_options(bewohner: &Bewohner) {
    println!(
        "
    (1) Kalender\n
    (2) Einkaufsliste\n
    (3) Putzplan\n
    (4) Finanzen\n
    (5) Aufgabenverteilung\n
    (6) Ausloggen\n
    "
    );

    if bewohner.admin {
        println!("---Adminfunktionen---\n(7) Bewohnerverwaltung");
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
