mod controller;
mod model;
mod view;

use anyhow::Result;

#[macro_use]
extern crate diesel;
extern crate dotenv;

fn main() -> Result<()>{
    controller::run()?;
    Ok(())
}
