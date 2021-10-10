use chrono::{NaiveDate};
use std::io;
use thiserror::Error;
use anyhow::Result;
use crate::db::Bewohner;

#[derive(Error, Debug)]
pub enum AufgabenError {
    
    #[error("Failed to Parse the input text due to a parsing error")]
    ParseError {source: chrono::ParseError},

    #[error("Error while doing something stupid")]
    StupidError,

    #[error(transparent)]
    IOError {source: std::io::Error},
}

pub struct Aufgabe {
    beschreibung: String,
    name: String,
    datum: NaiveDate, //Date
    beendet: bool,
}

impl Aufgabe {
    // Handler
    fn new(name: String, beschreibung: String, datum: NaiveDate, beendet: bool) -> Aufgabe {
        Aufgabe {
            name: name,
            beschreibung: beschreibung,
            datum: datum,
            beendet: beendet,
        }
    }

    pub fn beenden(&mut self) {
        self.beendet = true;
    }
}

fn parse_ios_date(date: &String) -> Result<NaiveDate, chrono::ParseError>{
    let trim_date = date.as_str().trim();
    NaiveDate::parse_from_str(trim_date, "%Y-%m-%d")
}


fn aufgabe_erstellen() -> Result<Aufgabe, AufgabenError> {
    let mut name = String::new();
    let mut beschreibung = String::new();
    let mut tmp = String::new();

    print!("Neue Aufgabe erstellen\n");
    print!("Name der Aufgabe\n");
    io::stdin()
        .read_line(&mut name).map_err(|source| AufgabenError::IOError {source})?;

    print!("Bis wann muss die Aufgabe erledigt sein?\t(y-m-d)");
    io::stdin()
        .read_line(&mut tmp).map_err(|source| AufgabenError::IOError {source})?;
    
    let datum = parse_ios_date(&tmp).map_err(|source| AufgabenError::ParseError {source})?;
    

    print!("{}\t\t{}\nBeschribung", name, tmp);
    io::stdin()
        .read_line(&mut beschreibung).map_err(|source| AufgabenError::IOError {source})?;

    Ok(Aufgabe::new(
        String::from(name.trim()),
        String::from(beschreibung.trim()),
        datum,
        false,
    ))
}

pub fn putzplan_options(){

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

/* fn aufgabe_erstellen() -> Result<Aufgabe, anyhow::Error> {
    let mut name = String::new();
    let mut beschreibung = String::new();
    let mut tmp = String::new();

    print!("Neue Aufgabe erstellen\n");
    print!("Name der Aufgabe\n");
    io::stdin()
        .read_line(&mut name).context(format!("Name input failed!"))?;

    print!("Bis wann muss die Aufgabe erledigt sein?\n(y-m-d)");
    io::stdin()
        .read_line(&mut tmp).context(format!("Date input failed!"))?;
    
    let datum = parse_ios_date(&tmp).context(format!("Parsing datre and conversion to NaiveDate failed"))?;
    

    print!("{}\t\t{}\nBeschribung", name, tmp);
    io::stdin()
        .read_line(&mut beschreibung).context(format!("Name input failed!"))?;

    Ok(Aufgabe::new(
        String::from(name.trim()),
        String::from(beschreibung.trim()),
        datum,
        false,
    ))
} */

pub fn aufgaben_options(bewohner: &Bewohner){

}

/*
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

     #[test]
    fn test_aufgabe_new() {
        let aufgabe = Aufgabe {
            name: String::from("name"),
            beschreibung: String::from("beschreibung"),
            datum: NaiveDate::from_str("2022-02-02").unwrap(), //Date
            beendet: false,
        };

        let aufgabe_neu = aufgabe_erstellen().unwrap();

        assert_eq!(aufgabe_neu.name, aufgabe.name);
        assert_eq!(aufgabe_neu.beschreibung, aufgabe.beschreibung);
        assert_eq!(aufgabe_neu.datum.to_string(), aufgabe.datum.to_string());
    }

    #[test]
    fn test_aufgabe_beenden() {
        let mut aufgabe = Aufgabe {
            beschreibung: String::from("beschreibung"),
            name: String::from("name"),
            datum: NaiveDate::from_str("2022-02-02").unwrap(), //Date
            beendet: false,
        };

       aufgabe.beenden();
       assert!(aufgabe.beendet)
    }

} */
