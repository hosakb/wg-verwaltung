use chrono::NaiveDate;

#[derive(Queryable)]
pub struct Bewohner {
    pub id: i32,
    pub name: String,
    pub nutzername: String,
    pub passwort: String, 
    pub admin: bool,
    pub geburtstag_id: i32,
}

impl Bewohner {
    pub fn new(
        id: i32,
        name: String,
        nutzername: String,
        passwort: String,
        admin: bool,
        geburtstag_id: i32,
    ) -> Bewohner {
        Bewohner {
            id,
            name,
            nutzername,
            passwort,
            admin,
            geburtstag_id,
        }
    }
}