use clap::Parser;
use backuppit::{run, CliArgs};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  run(CliArgs::parse())?;
  Ok(())
}
