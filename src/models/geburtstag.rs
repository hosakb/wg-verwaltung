use crate::db::schema::geburtstag;

use chrono::NaiveDate;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "geburtstag"]
pub struct Geburtstag {
    pub id: i32,
    pub datum: NaiveDate,
}

impl Geburtstag {
    pub fn new(id: i32, datum: NaiveDate) -> Geburtstag {
        Geburtstag { id, datum }
    }
}
