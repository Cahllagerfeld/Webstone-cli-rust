use std::string;

use clap::{Args, Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Command for handling route actions
    #[command(subcommand)]
    Route(RouteCommands),
}

#[derive(Subcommand, Debug)]
pub enum RouteCommands {
    Create(CreateCommand),
    Delete(DeleteCommand),
}

#[derive(Debug, Args)]
pub struct CreateCommand {
    pub path: String,
}

#[derive(Debug, Args)]
pub struct DeleteCommand {
    pub path: String,
}
