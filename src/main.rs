use crate::error::AppError;
extern crate serde_humanize_rs;
mod agent;
mod config;
mod error;
mod internal;
mod metric;
mod model;
mod plugins;

fn main() -> Result<(), AppError> {
    let c = config::Config::new()?;
    println!("{:#?}", c);
    Ok(())
}
