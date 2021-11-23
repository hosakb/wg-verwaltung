mod geld;
mod login;
mod aufgabe;
mod einkauf;
mod kalender;
mod putzplan;

use crate::model::bewohner::{self, Bewohner};
use crate::view;
use std::io;
use anyhow::{Result, Context};

pub fn run() -> Result<()>{
    let model = bewohner::read_bewohner().context("Something happened while reading the Database!")?;

    let model = match model {
        Some(v) => v,
        None => login::erster_bewohner().context("Something happened while creating the first user!")?,
    };

    let bewohner_option = login::login_user(&model).context("Something happened during login!")?;


    match bewohner_option {
        Some(b) => options(b)?,
        None => return Ok(()),
    }

    Ok(())
}

fn options(bewohner: &Bewohner) -> Result<()>{
    let mut option = String::new();
    let admin = bewohner.admin;

    view::menu::ask_options(bewohner);

    loop {
        option.clear();
        view::menu::print_options(bewohner);
        io::stdin()
            .read_line(&mut option)
            .context("Input could not be read!")?;

        let o = option.as_str();
        match o {
            // "1" => kalender::kalender_options(bewohner),
            // "2" => einkauf::einkaufsliste_options(bewohner),
            // "3" => putzplan::putzplan_options(bewohner),
            "4" => geld::finanzen_options(bewohner),
            // "5" => aufgabe::aufgaben_options(bewohner),
            "6" => todo!(), 
            _ => {
                if admin {
                    match o {
                        // "7" => bewohner::bewohnerveraltung_options(),
                        _ => view::menu::option_unavailable(),
                    }
                } else {
                    view::menu::option_unavailable();
                }
            }
        }
    }

    Ok(())
}
