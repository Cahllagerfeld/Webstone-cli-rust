mod command;
use clap::Parser;
use std::{
    fs::create_dir_all,
    fs::{self},
    io::Write,
};

use dialoguer::{theme::ColorfulTheme, MultiSelect};

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

    let multiselected = &["+page.svelte", "+page.ts", "+layout.svelte"];
    let defaults = &[true, false, false];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your types")
        .items(&multiselected[..])
        .defaults(&defaults[..])
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("No types selected. Aborting...");
        return;
    }

    for selection in selections {
        create_dir_all(format!("src/routes/{}", path)).expect("failed to create directory");

        let target_path = format!("src/routes/{}/{}", path, multiselected[selection]);
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(target_path)
            .expect("failed to create file");

        file.write_all(template.as_bytes())
            .expect("failed to write file");
    }
}
