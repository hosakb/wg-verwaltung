mod aufgabe;
mod db;
mod login;
mod kalender;
mod einkauf;
mod geld;
mod bewohner_verwaltung;
mod putzplan;

extern crate chrono;

use chrono::{NaiveDate, ParseError};
use login::login_user;
use core::panic;
use std::{clone, collections::HashMap, env, io, option};

use anyhow::{Context, Result};
use thiserror::Error;

use crate::db::Bewohner;

pub fn run() {
    let db = db::read_db();
    let bewohner;
    
    match login::login_user(&db) {
        Some(b) => bewohner = b,
        None => return,
    }

    options(bewohner);
}

fn options(bewohner: Bewohner){
    let mut option = String::from("");
    let admin = bewohner.admin;

    println!("Hallo {}, was möchtest du machen?", bewohner.name);

    loop {
        option.clear();
        print_options(&bewohner);
        io::stdin().read_line(&mut option).expect("Input could not be read!");

        let o = option.as_str();
        match o{
            "1" => kalender::kalender_options(&bewohner),
            "2" => einkauf::einkaufsliste_options(&bewohner),
            "3" => putzplan::putzplan_options(&bewohner),
            "4" => geld::finanzen_options(&bewohner),
            "5" => aufgabe::aufgaben_options(&bewohner),
            "6" => return,
            _ => {
                if admin {
                    match o {
                        "7" => bewohner_verwaltung::bewohnerveraltung_options(),
                        _ => eprint!("---Option nicht verfügbar---"),
                    }
                } else {
                    eprint!("---Option nicht verfügbar---")
                }
            }
        }
    }
}

fn print_options(bewohner: &Bewohner){
    println!("
    (1) Kalender\n
    (2) Einkaufsliste\n
    (3) Putzplan\n
    (4) Finanzen\n
    (5) Aufgabenverteilung\n
    (6) Ausloggen\n
    ");

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