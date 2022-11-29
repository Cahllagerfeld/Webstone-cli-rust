mod command;
use clap::Parser;
use std::{
    fs::create_dir_all,
    fs::{self},
    io::Write,
};

fn main() {
    let args = command::Cli::parse();

    match args.command {
        command::Commands::Route(command::RouteCommands::Create(command::CreateCommand {
            path,
        })) => handle_create_command(path),
        command::Commands::Route(command::RouteCommands::Delete(command::DeleteCommand {
            path,
        })) => {
            println!("Delete route: {}", path);
        }
    }
}

fn handle_create_command(path: String) -> () {
    let template = include_str!("../templates/+page.svelte.template");
    let target_path = format!("src/routes/{}/+page.svelte", path);

    create_dir_all(format!("src/routes/{}", path)).expect("failed to create directory");

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(target_path)
        .expect("failed to create file");

    file.write_all(template.as_bytes())
        .expect("failed to write file");
}
