mod cli;
mod server;
mod templates;

use clap::Parser;
use cli::app;
use cli::cli_handlers;

fn main() {
    let args = cli::app::Cli::parse();

    match args.command {
        app::Commands::Up(app::UpCommand {}) => cli_handlers::spin_up_server(),
        app::Commands::Route(cli::app::RouteCommands::Create(app::CreateCommand { path })) => {
            cli_handlers::handle_create_command(path)
        }
        app::Commands::Route(app::RouteCommands::Delete(app::DeleteCommand { path })) => {
            cli_handlers::handle_delete_route(path)
        }
        app::Commands::Route(app::RouteCommands::Count(app::CountCommand {})) => {
            cli_handlers::handle_count_routes()
        }
    }
}
