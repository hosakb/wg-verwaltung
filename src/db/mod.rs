pub mod schema;

use std::env;

use anyhow::Result;
use chrono::NaiveDate;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

use crate::models::{bewohner::*, geburtstag::Geburtstag};

use schema::bewohner::dsl as b_dsl;
use schema::geburtstag::dsl as g_dsl;

pub fn connect_db() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {} failed", database_url))
}

pub fn read_bewohner() -> Result<Vec<Bewohner>> {
    let conn = connect_db();

    let db_b = b_dsl::bewohner.load::<DbBewohner>(&conn)?;

    let b_list: Vec<Bewohner> = db_b
        .iter()
        .map(|b| {
            let bd = g_dsl::geburtstag
                .filter(g_dsl::id.eq(b.geburtstag_id()))
                .first::<Geburtstag>(&conn)
                .unwrap_or_else(|_| {
                    panic!(
                        "Geburtstag konnte nicht für Bewohner {} gefunden werden!",
                        b.nutzername()
                    )
                });

            Bewohner::new(
                b.id(),
                String::from(b.name()),
                String::from(b.nutzername()),
                String::from(b.passwort()),
                b.admin(),
                bd.datum(),
            )
        })
        .collect();

    Ok(b_list)
}

fn insert_into_bewohner_returning(name: &str, nutzername:  &str, passwort:  &str, admin: &bool, g: &Geburtstag, conn: &PgConnection) -> Result<DbBewohner, anyhow::Error> {
    let b: DbBewohner = diesel::insert_into(b_dsl::bewohner)
        .values((
            b_dsl::name.eq(name),
            b_dsl::nutzername.eq(&nutzername),
            b_dsl::passwort.eq(&passwort),
            b_dsl::admin.eq(admin),
            b_dsl::geburtstag_id.eq(g.id()),
        ))
        .get_result(conn)?;
    Ok(b)
}

pub fn create_bewohner(
    name: String,
    nutzername: String,
    passwort: String,
    admin: bool,
    birthday: NaiveDate,
) -> Result<Bewohner> {
    let conn = connect_db();

    let g = match check_existing_bd(birthday, &conn){
        Ok(g) => g,
        Err(_) => diesel::insert_into(g_dsl::geburtstag)
        .values(g_dsl::datum.eq(&birthday))
        .get_result(&conn)?
    };

    let b = insert_into_bewohner_returning(&name, &nutzername, &passwort, &admin, &g, &conn)?;

    Ok(Bewohner::new(
        b.id(),
        String::from(b.name()),
        String::from(b.nutzername()),
        String::from(b.passwort()),
        b.admin(),
        g.datum(),
    ))
}

fn check_existing_bd(birthday: NaiveDate, conn: &PgConnection) -> Result<Geburtstag> {
    let g = g_dsl::geburtstag
    .filter(g_dsl::datum.eq(birthday))
    .first::<Geburtstag>(conn)?;
    Ok(g)
}

pub fn username_exists(nutzername: String) -> Result<bool> {
    let conn = connect_db();

    let b = b_dsl::bewohner
        .filter(b_dsl::nutzername.eq(nutzername))
        .get_results::<DbBewohner>(&conn)?;

    if b.is_empty() {
        return Ok(false);
    }

    Ok(true)
}

#[cfg(test)]
mod test {

    use super::*;

    fn setup() {
        let conn = connect_db();

        let d = NaiveDate::from_ymd(1998, 2, 10);
        let g: Geburtstag = diesel::insert_into(g_dsl::geburtstag)
            .values(g_dsl::datum.eq(d))
            .get_result(&conn)
            .unwrap();
        let _: DbBewohner = diesel::insert_into(b_dsl::bewohner)
            .values((
                b_dsl::name.eq("Ben"),
                b_dsl::nutzername.eq("hosakb"),
                b_dsl::passwort.eq("1234"),
                b_dsl::admin.eq(true),
                b_dsl::geburtstag_id.eq(g.id()),
            ))
            .get_result(&conn)
            .unwrap();
    }

    #[test]
    fn test_read_bewohner() {
        setup();
        assert!(!read_bewohner().unwrap().is_empty());
    }

    #[test]
    fn test_create_bewohner() {
        let d = NaiveDate::from_ymd(1998, 2, 10);
        let b = create_bewohner(String::from("Ben"), String::from("hosakb"), String::from("1234"), true, d);
        assert!(b.is_ok());
        assert!(!read_bewohner().unwrap().is_empty());
    }
}
