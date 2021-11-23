use chrono::NaiveDate;
use postgres::{types::ToSql, Client, Error, NoTls, Row};

fn connect_db() -> Result<Client, Error> {
    let db = Client::connect("postgresql://postgres:1234@localhost/postgres", NoTls)?;
    Ok(db)
}

fn close_db_connection(db: Client) -> Result<(), postgres::Error> {
    db.close()
}

type Sql = dyn ToSql + Sync;

pub fn query(query_str: &str, params: &[&Sql]) -> Result<Vec<Row>, Error> {
    let mut db = connect_db()?;
    let bewohner = db.query(query_str, params)?;
    close_db_connection(db)?;

    Ok(bewohner)
}

pub fn query_single(query_str: &str, params: &[&Sql]) -> Result<Option<Row>, Error> {
    let mut db = connect_db()?;
    let bewohner = db.query_opt(query_str, params)?;
    close_db_connection(db)?;
    Ok(bewohner)
}

pub mod bewohner {

    use core::panic;

    use super::*;

    pub struct Bewohner {
        pub id: i32,
        pub name: String,
        pub admin: bool,
        pub username: String,
        pub passwort: String,
        pub birthday: NaiveDate,
    }

    impl Bewohner {
        pub fn new(
            id: i32,
            name: String,
            username: String,
            passwort: String,
            admin: bool,
            birthday: NaiveDate,
        ) -> Bewohner {
            Bewohner {
                id,
                name,
                admin,
                username,
                passwort,
                birthday,
            }
        }
    }

    pub fn read_bewohner() -> Result<Option<Vec<Bewohner>>, Error> {
        let q = "SELECT bewohner.id, bewohner.name, bewohner.nutzername, bewohner.passwort, bewohner.admin, bd.datum FROM bewohner INNER JOIN bd ON bewohner.bdid = bd.id";

        let bewohner: Vec<Bewohner> = query(q, &[])?
            .iter()
            .map(|row| {
                Bewohner::new(
                    row.get(0),
                    row.get(1),
                    row.get(2),
                    row.get(3),
                    row.get(4),
                    row.get(5),
                )
            })
            .collect();

        if bewohner.is_empty() {
            return Ok(None);
        }

        Ok(Some(bewohner))
    }

    pub fn get_bewohner_by_id(id: i32) -> Result<Option<Bewohner>, Error> {
        let q = "SELECT bewohner.id, bewohner.name, bewohner.nutzername, bewohner.passwort, bewohner.admin, bd.datum  FROM bewohner INNER JOIN bd ON bewohner.bdid = bd.id WHERE bewohner.id = $1";

        let row = query_single(q, &[&id])?;

        if row.is_none() {
            return Ok(None);
        }

        let row = row.unwrap();

        let bewohner = Bewohner::new(
            row.get(0),
            row.get(1),
            row.get(2),
            row.get(3),
            row.get(4),
            row.get(5),
        );

        Ok(Some(bewohner))
    }

    pub fn create_bewohner(
        name: String,
        username: String,
        passwort: String,
        admin: bool,
        birthday: NaiveDate,
    ) -> Result<Bewohner, Error> {
        let bdid: i32;

        let q_bd = "SELECT id FROM bd WHERE datum = $1;";
        let row = query_single(q_bd, &[&birthday])?;

        if row.is_none() {
            let q = "INSERT INTO bd(datum) VALUES ($1);";
            let query = query_single(q, &[])?;

            if query.is_none() {
                panic!("Unexpected None Value");
            }

            bdid = query.unwrap().get(0);
        } else {
            bdid = row.unwrap().get(0);
        }

        let q = "INSERT INTO bewohner(name, nutzername, passwort, admin, bdid) VALUES ($1, $2, $3, $4, $5) RETURNING bewohner.id;";
        let bewohner_id = query_single(q, &[&name, &username, &passwort, &admin, &bdid])?;

        if bewohner_id.is_none() {
            panic!("Unexpected None Value");
        }

        let bewohner = get_bewohner_by_id(bewohner_id.unwrap().get(0))?.unwrap(); // Besseres handeling

        Ok(bewohner)
    }

    pub fn username_exists(username: String) -> Result<bool, Error> {
        let q = "SELECT * FROM bewohner WHERE nutzername = $1";
        if query_single(q, &[&username])?.is_none() {
            return Ok(false);
        }

        Ok(true)
    }

    pub fn user_update_bewohner() {}

    pub fn admin_update_bewohner() {}
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_read_bewohner() {
        let bewohner = bewohner::read_bewohner().unwrap();

        assert!(bewohner.is_some());
    }
}
