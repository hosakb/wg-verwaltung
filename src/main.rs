mod controller;
mod model;
mod view;

use anyhow::Result;

fn main() -> Result<()>{
    controller::run()?;
    Ok(())
}
