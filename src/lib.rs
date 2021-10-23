mod aufgabe;
mod bewohner;
mod db;
mod einkauf;
mod geld;
mod kalender;
mod login;
mod putzplan;

extern crate chrono;

use core::panic;

use std::io;

use crate::db::bewohner::Bewohner;


pub fn run(){
    
    let db = db::bewohner::read_bewohner().unwrap_or_else(|err| panic!("Something happened while reading the Datatabse:\n{}", err));

    let db = match db {
        Some(v) => v,
        None => login::erster_bewohner().unwrap_or_else(|err| panic!("Something happened while creating the first user:\n{}", err)),
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
