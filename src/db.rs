use chrono::NaiveDate;
use postgres::{Client, Error, NoTls, types::ToSql, Row};

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

fn connect_db() -> Result<Client, Error> {
    let mut db = Client::connect("postgresql://postgres:1234@localhost/postgres", NoTls)?;
    Ok(db)
}

fn close_db_connection(db: Client) -> Result<(), postgres::Error>{
    db.close()
}

type Sql = dyn ToSql + Sync;

pub fn query(query: &str, params: &[&Sql]) -> Result<Vec<Row>, Error> {
    let mut db = connect_db()?;
    let bewohner = db.query(query, params)?;
    close_db_connection(db)?;

    Ok(bewohner)
}

pub fn query_single(query: &str, params: &[&Sql]) -> Result<Row, Error> {
    let mut db = connect_db()?;
    let bewohner = db.query_one(query, params)?;
    close_db_connection(db)?;

    Ok(bewohner)
}

pub fn read_bewohner() -> Result<Option<Vec<Bewohner>>, Error> {

    let q = "SELECT bewohner.id, bewohner.name, bewohner.nutzername, bewohner.passwort, bewohner.admin, bd.datum  FROM bewohner INNER JOIN bd ON bewohner.bdid = bd.id";

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
        }).collect();
    
    if bewohner.is_empty(){
        return Ok(None);
    }

    Ok(Some(bewohner))
}

pub fn create_bewohner(name: String, username: String, passwort: String, admin: bool, birthday: NaiveDate) -> Result<(), Error>{
    
    let bdid: i32;

    let q_bd = "SELECT bdid FROM bd WHERE id = $1";
    let row = query_single(q_bd, &[&birthday])?;

    if row.is_empty(){
        let q = "INSERT INTO bd(datum) VALUES ($1);";
        bdid = query_single(q, &[])?.get(1);
       
    } else {
        bdid = row.get(1);
    }

    let q = "INSERT INTO bewohner(name, nutzername, passwort, admin, bdid) VALUES ($1, $2, $3, $4, $5);";
    query(q, &[&name ,&username, &passwort, &admin, &bdid])?;
    
    
    Ok(())
    
}

pub fn user_update_bewohner(){

}

pub fn admin_update_bewohner(){
    
}



#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_read_bewohner() {
        let bewohner = read_bewohner().unwrap();

        assert!(bewohner.is_some());
    }
}
