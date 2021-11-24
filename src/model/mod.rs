pub mod models;
pub mod schema;

use std::env; 

use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use anyhow::{Result, Context};
use models::Bewohner;

use schema::bewohner::dsl::*;


pub fn connect_db() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

    use super::*;

pub fn read_bewohner() -> Result<Vec<Bewohner>> {
    let connection = connect_db();
    let result = bewohner.load::<Bewohner>(&connection)?;

    Ok(result)
}

    // pub fn create_bewohner(
    //     name: String,
    //     username: String,
    //     passwort: String,
    //     admin: bool,
    //     birthday: NaiveDate,
    // ) -> Result<Bewohner, Error> {
    //     let bdid: i32;

    //     let q_bd = "SELECT id FROM bd WHERE datum = $1;";
    //     let row = query_single(q_bd, &[&birthday])?;

    //     if row.is_none() {
    //         let q = "INSERT INTO bd(datum) VALUES ($1);";
    //         let query = query_single(q, &[])?;

    //         if query.is_none() {
    //             panic!("Unexpected None Value");
    //         }

    //         bdid = query.unwrap().get(0);
    //     } else {
    //         bdid = row.unwrap().get(0);
    //     }

    //     let q = "INSERT INTO bewohner(name, nutzername, passwort, admin, bdid) VALUES ($1, $2, $3, $4, $5) RETURNING bewohner.id;";
    //     let bewohner_id = query_single(q, &[&name, &username, &passwort, &admin, &bdid])?;

    //     if bewohner_id.is_none() {
    //         panic!("Unexpected None Value");
    //     }

    //     let bewohner = get_bewohner_by_id(bewohner_id.unwrap().get(0))?.unwrap(); // Besseres handeling

    //     Ok(bewohner)
    // }

    // pub fn username_exists(username: String) -> Result<bool, Error> {
    //     let q = "SELECT * FROM bewohner WHERE nutzername = $1";
    //     if query_single(q, &[&username])?.is_none() {
    //         return Ok(false);
    //     }

    //     Ok(true)
    // }

    pub fn user_update_bewohner() {}

    pub fn admin_update_bewohner() {}

#[cfg(test)]
mod test {

    use super::*;

    // #[test]
    // fn test_read_bewohner() {
    //     let bewohner = entity::read_bewohner().unwrap();

    //     assert!(bewohner.is_some());
    // }
}
