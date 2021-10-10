use crate::db::Bewohner;
use std::io;

use crate::db;

pub fn bewohnerveraltung_options() {
    let mut option = String::from("");

    println!("Willkomen in der Bewohnerverwaltung. Was möchtest du tun?");

    loop {
        option.clear();
        print_options();
        io::stdin()
            .read_line(&mut option)
            .expect("Input could not be read!");

        let o = option.as_str();
        match o {
            "1" => neuer_bewohner(),
            "2" => bewohner_löschen(),
            "3" => return,
            _ => eprint!("---Option nicht verfügbar---"),
        }
    }
}

fn print_options() {
    println!(
        "
    (1) Neuen Bewohner hinzufügen\n
    (2) Bewohner löschen\n
    (3) Zurück\n
    "
    );
}

fn neuer_bewohner() {
    let mut name_buf = String::from("");
    let mut bday_buf = String::from("");
    let mut username_buf = String::from("");
    let mut username_clone;
    let mut password_buf = String::from("");
    let mut admin_buf = String::from("");
    let mut name;
    let mut bday;
    let mut username;
    let mut admin;

    println!("---Neuen bewohner anlegen---");

    loop {
        name = loop {
            name_buf.clear();
            print!("Name: ");
            io::Write::flush(&mut io::stdout()).expect("flush failed!");
            io::stdin()
                .read_line(&mut name_buf)
                .expect("Could not read password!");
            
            if !name_buf.is_empty() {
                break &name_buf;
            } else {
                eprint!("Name must not be empty");
            }
        }.to_string();;

        bday = loop {
            name_buf.clear();
            print!("Geburtstag eingeben(yyyy-mm-dd): ");
            io::Write::flush(&mut io::stdout()).expect("flush failed!");
            io::stdin()
                .read_line(&mut bday_buf)
                .expect("Could not read birthday!");
            
            if !bday_buf.is_empty() {
                break &bday_buf;
            } else {
                eprint!("Name must not be empty");
            }
        }.to_string();

        username = loop {
            username_buf.clear();
            print!("Nutzername: ");
            io::Write::flush(&mut io::stdout()).expect("flush failed!");
            io::stdin()
                .read_line(&mut username_buf)
                .expect("Could not read username!");

            username_clone = String::from(&username_buf);

            let username_exists = db::check_username_exists(&username_clone).is_some();

            if !username_buf.is_empty() && !username_exists {
                break &username_buf;
            } else if username_exists {
                eprint!("Username already exists!");
            } else {
                eprint!("Username must not be empty");
            }
        }.to_string();;

        admin = loop {
            admin_buf.clear();
            print!("Admin (y/n): ");
            io::Write::flush(&mut io::stdout()).expect("flush failed!");
            io::stdin()
                .read_line(&mut admin_buf)
                .expect("Could not read password!");

            match admin_buf.trim(){
                "y" => break true,
                "n" => break false,
                _ =>  eprint!("Admin choice must not be empty")
            }
        };
    

        println!("Möchtest du den Bewohner\nName: {}\nNutzername: {}\nAdmin: {}\nwirklich anlegen? (y/n)", name, username, admin);
        let mut confirm_buf = String::from("");
        let choice;
        loop {
            confirm_buf.clear();
            io::Write::flush(&mut io::stdout()).expect("flush failed!");
            io::stdin()
                .read_line(&mut confirm_buf)
                .expect("Could not read password!");

            let choice = match confirm_buf.trim(){
                "y" => {
                    choice = true;
                    break;
                },
                "n" => {
                    choice = false;
                    break;
                },
                _ =>  eprint!("Choice must not be empty!")
            };
        };

        if choice {
           Bewohner::new(name, bday, admin, username);
           break;
        }
    }
}

fn bewohner_löschen() {}
