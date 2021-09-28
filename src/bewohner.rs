use chrono::{NaiveDate, ParseError};

pub struct Bewohner {
    id: i32,
    pub name: String,
    pub bday: NaiveDate,
    pub admin: bool,
    pub username: String,
    pub passwort: String,
}

impl Bewohner {
    pub fn new(id: i32, name: String, admin: bool, username: String, passwort: String) -> Bewohner {
        Bewohner {
            id: id,
            name: name,
            bday: bday,
            admin: admin,
            username: username,
            passwort: passwort,
        }
    }
}



