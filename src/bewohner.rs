use crate::db::Bewohner;
use std::io;

pub fn bewohnerveraltung_options(){
    let mut option = String::from("");

    println!("Willkomen in der Bewohnerverwaltung. Was möchtest du tun?");

    loop {
        option.clear();
        print_options();
        io::stdin().read_line(&mut option).expect("Input could not be read!");

        let o = option.as_str();
        match o{
            "1" => neuer_bewohner(),
            "2" => bewohner_löschen(),
            "3" => return,
            _ =>   eprint!("---Option nicht verfügbar---")
        }
    }
}

fn print_options(){
    println!("
    (1) Neuen Bewohner hinzufügen\n
    (2) Bewohner löschen\n
    (3) Zurück\n
    ");
}

fn neuer_bewohner(){
    print!("Neuen bewohner anlegen");
}

fn bewohner_löschen(){

}


