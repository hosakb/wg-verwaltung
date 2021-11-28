use crate::db::schema::bewohner;
use crate::models::geburtstag::Geburtstag;
use chrono::NaiveDate;

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Geburtstag)]
#[table_name = "bewohner"]
pub struct DbBewohner {
    id: i32,
    name: String,
    nutzername: String,
    passwort: String,
    admin: bool,
    geburtstag_id: i32,
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

    /// Get a reference to the bewohner's id.
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Get a reference to the bewohner's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the bewohner's nutzername.
    pub fn nutzername(&self) -> &str {
        self.nutzername.as_ref()
    }

    /// Get a reference to the bewohner's passwort.
    pub fn passwort(&self) -> &str {
        self.passwort.as_ref()
    }

    /// Get a reference to the bewohner's admin.
    pub fn admin(&self) -> bool {
        self.admin
    }

    /// Get a reference to the bewohner's geburtstag_id.
    pub fn geburtstag_id(&self) -> i32 {
        self.geburtstag_id
    }

    /// Set the bewohner's id.
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    /// Set the bewohner's name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Set the bewohner's nutzername.
    pub fn set_nutzername(&mut self, nutzername: String) {
        self.nutzername = nutzername;
    }

    /// Set the bewohner's passwort.
    pub fn set_passwort(&mut self, passwort: String) {
        self.passwort = passwort;
    }

    /// Set the bewohner's admin.
    pub fn set_admin(&mut self, admin: bool) {
        self.admin = admin;
    }

    /// Set the bewohner's geburtstag_id.
    pub fn set_geburtstag_id(&mut self, geburtstag_id: i32) {
        self.geburtstag_id = geburtstag_id;
    }
}

pub struct Bewohner {
    id: i32,
    name: String,
    nutzername: String,
    passwort: String,
    admin: bool,
    geburtstag: NaiveDate,
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

    /// Get a reference to the bewohner's id.
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Get a reference to the bewohner's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the bewohner's nutzername.
    pub fn nutzername(&self) -> &str {
        self.nutzername.as_ref()
    }

    /// Get a reference to the bewohner's passwort.
    pub fn passwort(&self) -> &str {
        self.passwort.as_ref()
    }

    /// Get a reference to the bewohner's admin.
    pub fn admin(&self) -> bool {
        self.admin
    }

    /// Get a reference to the bewohner's geburtstag.
    pub fn geburtstag(&self) -> NaiveDate {
        self.geburtstag
    }

    /// Set the bewohner's id.
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    /// Set the bewohner's name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Set the bewohner's nutzername.
    pub fn set_nutzername(&mut self, nutzername: String) {
        self.nutzername = nutzername;
    }

    /// Set the bewohner's passwort.
    pub fn set_passwort(&mut self, passwort: String) {
        self.passwort = passwort;
    }

    /// Set the bewohner's admin.
    pub fn set_admin(&mut self, admin: bool) {
        self.admin = admin;
    }

    /// Set the bewohner's geburtstag.
    pub fn set_geburtstag(&mut self, geburtstag: NaiveDate) {
        self.geburtstag = geburtstag;
    }
}
