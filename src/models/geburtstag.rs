use chrono::NaiveDate;

use crate::db::schema::geburtstag;

#[derive(Insertable, Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "geburtstag"]
pub struct Geburtstag {
    id: i32,
    datum: NaiveDate,
}

impl Geburtstag {
    pub fn new(id: i32, datum: NaiveDate) -> Geburtstag {
        Geburtstag { id, datum }
    }

    /// Get a reference to the geburtstag's id.
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Get a reference to the geburtstag's datum.
    pub fn datum(&self) -> NaiveDate {
        self.datum
    }

    /// Set the geburtstag's id.
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    /// Set the geburtstag's datum.
    pub fn set_datum(&mut self, datum: NaiveDate) {
        self.datum = datum;
    }
}
