mod aufgabe;
mod db;
mod login;

extern crate chrono;

use chrono::{NaiveDate, ParseError};
use login::login_user;
use core::panic;
use std::{clone, collections::HashMap, env, io};

use anyhow::{Context, Result};
use thiserror::Error;

use crate::db::Bewohner;

pub fn app() {
    let db = db::read_db();
    let bewohner;
    
    match login::login_user(&db) {
        Some(b) => bewohner = b,
        None => return,
    }

    if bewohner.admin {
        admin_functions();
    } else {
        standard_function();
    }
}

fn admin_functions(){
    
}

fn standard_function(){
    
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