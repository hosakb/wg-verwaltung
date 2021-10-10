use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::path::Path;

use chrono::NaiveDate;

#[derive(Debug, Eq, PartialEq)]
pub struct Bewohner {
    pub id: i32,
    pub name: String,
    pub bday: NaiveDate,
    pub admin: bool,
    pub username: String,
    pub passwort: String,
}

impl Bewohner {
    pub fn new(name: String, bday: String, admin: bool, username: String) -> Bewohner {
        Bewohner {
            id: get_next_id(),
            name: name,
            bday: NaiveDate::parse_from_str( bday.as_str().trim(), "%Y-%m-%d").unwrap(),
            admin: admin,
            username: username,
            passwort: String::from("1234"),
        }
    }
}


#[derive(Debug, Deserialize)]
struct BewohnerJSON {
    id: i32,
    name: String,
    bday: String,
    admin: bool,
    username: String,
    passwort: String,
}

pub fn read_db() -> Vec<Bewohner>{
    let json_file_path = Path::new("db.json");
    let file = File::open(json_file_path).expect("file not found");
    let bewohner_json: Vec<BewohnerJSON> = serde_json::from_reader(file).expect("error while reading");
    let bewohner: Vec<Bewohner> = bewohner_json.into_iter().map(|b| {
       Bewohner{
        id: b.id,
        name: b.name,
        bday:  NaiveDate::parse_from_str( b.bday.as_str().trim(), "%Y-%m-%d").unwrap(),
        admin: b.admin,
        username: b.username,
        passwort: b.passwort,
       }
    }).collect();
    bewohner
}

pub fn write_db(){
    serde_json::
}

pub fn check_username_exists(username: &str) -> Option<Bewohner> {
    read_db().into_iter().find(|b| b.username.eq(username.trim()))
}

fn get_next_id() -> i32{
    match read_db().last() {
        Some(n) => return n.id + 1,
        None => return 1,
    }
}