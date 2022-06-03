mod app;
mod board;
mod cell;
mod crossterm;
mod game;
mod ui;

use std::error::Error;
use std::time::Duration;

use crate::crossterm::run;

fn main() -> Result<(), Box<dyn Error>> {
    run(Duration::from_millis(32))?;
    Ok(())
}
