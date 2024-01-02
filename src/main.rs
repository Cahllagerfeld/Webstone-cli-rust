mod cli;
mod templates;

use clap::Parser;
use cli::app;
use cli::handlers;

fn main() {
    let args = cli::app::Cli::parse();

    match args.command {
        app::Commands::Route(cli::app::RouteCommands::Create(app::CreateCommand { path })) => {
            handlers::handle_create_command(path)
        }
        app::Commands::Route(app::RouteCommands::Delete(app::DeleteCommand { path })) => {
            println!("Delete route: {}", path);
        }
    }
}
