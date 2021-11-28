pub mod login {
    pub fn login_user() {
        println!("Willkommen zu Skittles. Der professionellen WG-Verwaltungssoftware, welche es dir leicht macht, deine WG zu verwalten.");
        println!("--Login--");
    }

    pub fn check_username() {
        println!("Bitte Username eingeben! Für abbruch Eingabe drücken!");
        print!("Username: ");
    }

    pub fn username_does_not_exists() {
        eprintln!("---Username existiert nicht!---");
    }

    pub fn ask_username() {
        println!("Nutzername:");
    }

    pub fn ask_password() {
        print!("Passwort: ");
    }

    pub fn ask_name() {
        println!("Wie heißt du?");
    }

    pub fn ask_birthday() {
        println!("Geburtstag");
    }

    pub fn login_successfull() {
        println!("Login erfolgreich!");
    }

    pub fn wrong_password() {
        println!("---Passwort ist falsch!---");
    }

    pub fn welcome() {
        println!("Wilkommen...");
    }

    pub fn day() {
        println!("Tag");
    }

    pub fn month() {
        println!("Monat");
    }

    pub fn year() {
        println!("Jahr");
    }
}

pub mod menu {
    use crate::models::bewohner::Bewohner;

    pub fn ask_options(bewohner: &Bewohner) {
        println!("Hallo {}, was möchtest du machen?", bewohner.name());
    }

    pub fn print_options(bewohner: &Bewohner) {
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

        if bewohner.admin() {
            println!("---Adminfunktionen---\n(7) Bewohnerverwaltung");
        }
    }

    pub fn option_unavailable() {
        eprint!("---Option nicht verfügbar---");
    }
}

#[cfg(tests)]
mod tests {

    use super::*;
}
