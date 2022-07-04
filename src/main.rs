mod app;
mod crossterm;
mod ui;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let res = crossterm::run();
    if let Err(err) = res {
        panic!("Error message in crossterm::run, {}", err);
    }
    Ok(())
}
