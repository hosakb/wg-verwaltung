use std::env;
use std::error::Error;

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
    pub low_level: String,
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
            None => return Err("Did not find low level command"),
        };


        Ok(Befehl {
            high_level,
            low_level,
        })
    }
}

pub fn interp(befehl: Befehl){

}