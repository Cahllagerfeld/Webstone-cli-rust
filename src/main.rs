mod command;
use clap::Parser;

fn main() {
    let args = command::Cli::parse();
}
