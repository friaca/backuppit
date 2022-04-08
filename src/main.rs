use clap::Parser;
use backuppit::{run, CliArgs};

fn main() {
  run(CliArgs::parse());
}
