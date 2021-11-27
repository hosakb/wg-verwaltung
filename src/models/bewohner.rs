use crate::db::schema::bewohner;
use super::geburtstag::Geburtstag;
use chrono::NaiveDate;


#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Geburtstag)]
#[table_name = "bewohner"]
pub struct DbBewohner {
    pub id: i32,
    pub name: String,
    pub nutzername: String,
    pub passwort: String,
    pub admin: bool,
    pub geburtstag_id: i32,
}

impl DbBewohner {
    pub fn new(
        id: i32,
        name: String,
        nutzername: String,
        passwort: String,
        admin: bool,
        geburtstag_id: i32,
    ) -> DbBewohner {
        DbBewohner {
            id,
            name,
            nutzername,
            passwort,
            admin,
            geburtstag_id,
        }
    }
}

pub struct Bewohner {
    pub id: i32,
    pub name: String,
    pub nutzername: String,
    pub passwort: String,
    pub admin: bool,
    pub geburtstag: NaiveDate,
}

impl Bewohner {
    pub fn new(
        id: i32,
        name: String,
        nutzername: String,
        passwort: String,
        admin: bool,
        geburtstag: NaiveDate,
    ) -> Bewohner {
        Bewohner {
            id,
            name,
            nutzername,
            passwort,
            admin,
            geburtstag,
        }
    }
}
