use wg_verwaltung::Bewohner;

fn main() {

    let admin = Bewohner::new(0, String::from("Tobias"), true);
    print!("{}", admin.name);

}
