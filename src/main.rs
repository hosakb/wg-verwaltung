use std::env;
use std::process;

use wg_verwaltung::Bewohner;
use wg_verwaltung::Befehl;


fn main() {


    let befehl = Befehl::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    wg_verwaltung::interp(befehl);

    

}
