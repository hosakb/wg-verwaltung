pub struct Bewohner{

    pub id: i32,
    pub name: String,
    // Gebutstag: DATE
    pub admin: bool,
    

}

impl Bewohner{
    pub fn new(id: i32, name: String, admin: bool) -> Bewohner{
       Bewohner{
        id: id,
        name: name,
        // Gebutstag: DATE
        admin: admin,
       }  
    }
}