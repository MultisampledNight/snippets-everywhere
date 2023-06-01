use thiserror::Error;

mod ols;
mod ui;
mod ultisnips;

#[derive(Debug, Error)]
pub enum RunError {}

pub fn run() -> Result<(), RunError> {
    let args = ui::cmdline();
    dbg!(args);

    Ok(())
}
