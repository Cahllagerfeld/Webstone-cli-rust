use clap::{Args, Parser, Subcommand};

/// Program for creating and deleting Sveltekit routes
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
    /// Create a new route
    Create(CreateCommand),
    /// Delete an existing route
    Delete(DeleteCommand),
    /// Count the number of routes
    Count(CountCommand),
}

#[derive(Debug, Args)]
pub struct CreateCommand {
    pub path: String,
}

#[derive(Debug, Args)]
pub struct DeleteCommand {
    pub path: String,
}

#[derive(Debug, Args)]
pub struct CountCommand {}
