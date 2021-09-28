use crate::Bewohner;
use std::io;

pub struct GeldVerwaltung{
    pub bewohner:Vec<Bewohner>,
    pub ausgaben:Vec<Ausgabe>,
}

pub struct Ausgabe{
    pub bewohner_ID:u32,
    pub betrag:f32,
}

impl GeldVerwaltung{
    pub fn new() -> GeldVerwaltung{
        GeldVerwaltung{
            bewohner:Vec::<Bewohner>::new(),
            ausgaben:Vec::<Ausgabe>::new(),
        }
    }
}

impl Ausgabe{
    pub fn new(bewohner_ID:u32, betrag:f32) -> Ausgabe{
        Ausgabe{
            bewohner_ID:bewohner_ID,
            betrag:betrag,
        }
    }
}


pub fn generelle_ausgabe(){
    let mut str:String =String::new();

    let mut verwaltung:GeldVerwaltung = GeldVerwaltung::new();
    
    verwaltung.bewohner.push(Bewohner::new(1, String::from("Hans Günter"), false, String::from(""), String::from("")));
    verwaltung.bewohner.push(Bewohner::new(2, String::from("Gans Hünter"), false, String::from(""), String::from("")));

    println!("Welcher Bewohner fügt eine generelle Ausgabe hinzu?");
    for i in 0..verwaltung.bewohner.len(){
        println!("{}: {}", i, verwaltung.bewohner[i].name);
    }

    str.clear();
    io::stdin().read_line( &mut str).expect("You must put in a command");

}