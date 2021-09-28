use std::env;
use std::error::Error;
use std::io;

pub struct Bewohner{

    pub id: i32,
    pub name: String,
    // Gebutstag: DATE
    pub admin: bool,
    pub username: String,
    pub passwort: String,
    

}

impl Bewohner{
    pub fn new(id: i32, name: String, admin: bool,username: String, passwort: String) -> Bewohner{
       Bewohner{
        id: id,
        name: name,
        // Gebutstag: DATE
        admin: admin,
        username: username,
        passwort: passwort,
       }  
    }
}

pub struct Befehl{
    pub high_level: String,
    pub low_level: Option<String>,
}

impl Befehl{
    pub fn new(mut args: env::Args) -> Result<Befehl, &'static str>{
        //TODO Fehlermeldungen

        args.next();

        let high_level = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not find high level command"),
        };

        let low_level = match args.next() {
            Some(arg) => arg,
            None => return Ok(Befehl {high_level,
            low_level: None}),
        };


        Ok(Befehl {
            high_level,
            low_level: Some(low_level),
        })
    }
}



pub fn present_interface(){
    let mut str = String::new(); 
    loop{
        println!("Willkommen zu WG-Verwaltung. Der professionellen WG-Verwaltungssoftware, welche es dir leicht macht, deine WG zu verwalten.");
        println!("Was möchtest du tun?");
        println!("\tkalender\n\tgeld\n\tbewohner\n\taufgaben\n\tbeenden");
        str.clear();
        io::stdin().read_line( &mut str).expect("You must put in a command");
        
        match str.trim() {
            "beenden" => break,
            "kalender" => interp_kalender(),
            "geld" => interp_geld(),
            _ => println!("Dieser Befehl existiert nicht. Bitte überlege noch einmal, welche Entscheidungen dich an diesen Punkt gebracht haben."),
        }
    }
}

fn interp_geld(){

    let mut str = String::new();
    loop{
        println!("Du hast gewählt, dass du etwas mit Geld tun möchtest. Was möchtest du tun?");
        println!("\tzurück\n\tzeige Nutzer an\n\tfüge generelle Ausgabe hinzu");

        str.clear();
        io::stdin().read_line( &mut str).expect("You must put in a command");
        
        match str.trim() {
            "zurück" => break,
            "generelle Ausgabe" => interp_kalender(),
            "geld" => interp_geld(),
            _ => println!("Dieser Befehl existiert nicht. Bitte überlege noch einmal, welche Entscheidungen dich an diesen Punkt gebracht haben."),
        }
    }
}

fn generelle_ausgabe(mut ghj:[i32]){

}



fn interp_kalender(){

}

fn interp_bewohner(low_level: String) {
    println!("Bewohner")
}
fn interp_aufgaben(low_level: String) {
    println!("Aufgaben")
}